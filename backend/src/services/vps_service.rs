use crate::{
    database::DbPool,
    models::vps::*,
    utils::errors::AppError,
};
use chrono::Utc;
use reqwest;
use uuid::Uuid;

const HETZNER_API_URL: &str = "https://api.hetzner.cloud/v1";

#[derive(Clone)]
pub struct HetznerClient {
    api_token: String,
    client: reqwest::Client,
}

impl HetznerClient {
    pub fn new(api_token: String) -> Self {
        Self {
            api_token,
            client: reqwest::Client::new(),
        }
    }

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<impl serde::Serialize>,
    ) -> Result<T, AppError> {
        let url = format!("{}{}", HETZNER_API_URL, endpoint);

        let mut req = self.client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.api_token));

        if let Some(body) = body {
            req = req.json(&body);
        }

        let response = req.send().await
            .map_err(|e| AppError::InternalError(format!("Hetzner API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::InternalError(
                format!("Hetzner API error {}: {}", status, error_text)
            ));
        }

        response.json().await
            .map_err(|e| AppError::InternalError(format!("Failed to parse Hetzner response: {}", e)))
    }

    pub async fn create_server(&self, request: HetznerCreateServerRequest) -> Result<HetznerServer, AppError> {
        let response: HetznerServerResponse = self.request(
            reqwest::Method::POST,
            "/servers",
            Some(request),
        ).await?;

        Ok(response.server)
    }

    pub async fn get_server(&self, server_id: i64) -> Result<HetznerServer, AppError> {
        let response: HetznerServerResponse = self.request(
            reqwest::Method::GET,
            &format!("/servers/{}", server_id),
            None::<()>,
        ).await?;

        Ok(response.server)
    }

    pub async fn list_servers(&self) -> Result<Vec<HetznerServer>, AppError> {
        let response: HetznerServersResponse = self.request(
            reqwest::Method::GET,
            "/servers",
            None::<()>,
        ).await?;

        Ok(response.servers)
    }

    pub async fn delete_server(&self, server_id: i64) -> Result<(), AppError> {
        self.request::<serde_json::Value>(
            reqwest::Method::DELETE,
            &format!("/servers/{}", server_id),
            None::<()>,
        ).await?;

        Ok(())
    }

    pub async fn power_on(&self, server_id: i64) -> Result<(), AppError> {
        self.request::<serde_json::Value>(
            reqwest::Method::POST,
            &format!("/servers/{}/actions/poweron", server_id),
            None::<()>,
        ).await?;

        Ok(())
    }

    pub async fn power_off(&self, server_id: i64) -> Result<(), AppError> {
        self.request::<serde_json::Value>(
            reqwest::Method::POST,
            &format!("/servers/{}/actions/poweroff", server_id),
            None::<()>,
        ).await?;

        Ok(())
    }

    pub async fn reboot(&self, server_id: i64) -> Result<(), AppError> {
        self.request::<serde_json::Value>(
            reqwest::Method::POST,
            &format!("/servers/{}/actions/reboot", server_id),
            None::<()>,
        ).await?;

        Ok(())
    }
}

// Database operations
pub async fn list_vps(db: &DbPool, user_id: Option<Uuid>) -> Result<Vec<Vps>, AppError> {
    let vps = if let Some(uid) = user_id {
        sqlx::query_as::<_, Vps>(
            "SELECT * FROM vps WHERE user_id = $1 ORDER BY created_at DESC"
        )
        .bind(uid)
        .fetch_all(db)
        .await?
    } else {
        sqlx::query_as::<_, Vps>(
            "SELECT * FROM vps ORDER BY created_at DESC"
        )
        .fetch_all(db)
        .await?
    };

    Ok(vps)
}

pub async fn get_vps(db: &DbPool, id: Uuid) -> Result<Vps, AppError> {
    let vps = sqlx::query_as::<_, Vps>(
        "SELECT * FROM vps WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound("VPS not found".to_string()))?;

    Ok(vps)
}

pub async fn create_vps(
    db: &DbPool,
    hetzner_client: &HetznerClient,
    user_id: Uuid,
    payload: CreateVps,
) -> Result<Vps, AppError> {
    // Create server on Hetzner
    let hetzner_request = HetznerCreateServerRequest {
        name: payload.name.clone(),
        server_type: payload.server_type.clone(),
        location: payload.location.clone(),
        image: payload.image.clone(),
        ssh_keys: payload.ssh_keys,
        user_data: payload.user_data,
        start_after_create: true,
    };

    let hetzner_server = hetzner_client.create_server(hetzner_request).await?;

    // Extract pricing
    let monthly_cost = hetzner_server.server_type.prices
        .first()
        .and_then(|p| p.price_monthly.gross.parse::<f64>().ok());

    // Save to database - if this fails, rollback by deleting the Hetzner server
    let vps = sqlx::query_as::<_, Vps>(
        "INSERT INTO vps (
            id, user_id, name, hetzner_id, status, server_type, location, image,
            ipv4, ipv6, cpu_cores, ram_gb, disk_gb, monthly_cost, created_at, updated_at
         )
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&hetzner_server.name)
    .bind(hetzner_server.id)
    .bind(&hetzner_server.status)
    .bind(&hetzner_server.server_type.name)
    .bind(&hetzner_server.datacenter.location.name)
    .bind(&hetzner_server.image.name)
    .bind(hetzner_server.public_net.ipv4.as_ref().map(|ip| ip.ip.clone()))
    .bind(hetzner_server.public_net.ipv6.as_ref().map(|ip| ip.ip.clone()))
    .bind(hetzner_server.server_type.cores)
    .bind((hetzner_server.server_type.memory / 1024.0) as i32)
    .bind(hetzner_server.server_type.disk)
    .bind(monthly_cost)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(db)
    .await
    .map_err(|e| {
        // Database insert failed - attempt to clean up the Hetzner server
        let hetzner_id = hetzner_server.id;
        let client = hetzner_client.clone();
        tokio::spawn(async move {
            if let Err(cleanup_err) = client.delete_server(hetzner_id).await {
                tracing::error!("Failed to cleanup Hetzner server {} after database error: {}", hetzner_id, cleanup_err);
            }
        });
        e
    })?;

    Ok(vps)
}

pub async fn update_vps(
    db: &DbPool,
    id: Uuid,
    payload: UpdateVps,
) -> Result<Vps, AppError> {
    let mut vps = get_vps(db, id).await?;

    if let Some(name) = payload.name {
        vps.name = name;
    }
    if let Some(status) = payload.status {
        vps.status = status.as_str().to_string();
    }

    let vps = sqlx::query_as::<_, Vps>(
        "UPDATE vps
         SET name = $1, status = $2, updated_at = $3
         WHERE id = $4
         RETURNING *"
    )
    .bind(&vps.name)
    .bind(&vps.status)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(vps)
}

pub async fn delete_vps(
    db: &DbPool,
    hetzner_client: &HetznerClient,
    id: Uuid,
) -> Result<(), AppError> {
    let vps = get_vps(db, id).await?;

    // Delete from Hetzner if hetzner_id exists
    if let Some(hetzner_id) = vps.hetzner_id {
        hetzner_client.delete_server(hetzner_id).await?;
    }

    // Delete from database
    let result = sqlx::query("DELETE FROM vps WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("VPS not found".to_string()));
    }

    Ok(())
}

pub async fn sync_vps_status(
    db: &DbPool,
    hetzner_client: &HetznerClient,
    id: Uuid,
) -> Result<Vps, AppError> {
    let vps = get_vps(db, id).await?;

    if let Some(hetzner_id) = vps.hetzner_id {
        let hetzner_server = hetzner_client.get_server(hetzner_id).await?;

        let vps = sqlx::query_as::<_, Vps>(
            "UPDATE vps
             SET status = $1, ipv4 = $2, ipv6 = $3, updated_at = $4
             WHERE id = $5
             RETURNING *"
        )
        .bind(&hetzner_server.status)
        .bind(hetzner_server.public_net.ipv4.as_ref().map(|ip| ip.ip.clone()))
        .bind(hetzner_server.public_net.ipv6.as_ref().map(|ip| ip.ip.clone()))
        .bind(Utc::now())
        .bind(id)
        .fetch_one(db)
        .await?;

        Ok(vps)
    } else {
        Ok(vps)
    }
}
