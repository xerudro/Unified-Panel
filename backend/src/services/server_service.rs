use crate::{
    database::DbPool,
    models::server::*,
    utils::errors::AppError,
};
use chrono::Utc;
use uuid::Uuid;

pub async fn list_servers(db: &DbPool) -> Result<Vec<Server>, AppError> {
    let servers = sqlx::query_as::<_, Server>(
        "SELECT * FROM servers ORDER BY created_at DESC"
    )
    .fetch_all(db)
    .await?;

    Ok(servers)
}

pub async fn get_server(db: &DbPool, id: Uuid) -> Result<Server, AppError> {
    let server = sqlx::query_as::<_, Server>(
        "SELECT * FROM servers WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound("Server not found".to_string()))?;

    Ok(server)
}

pub async fn create_server(
    db: &DbPool,
    user_id: Uuid,
    payload: CreateServer,
) -> Result<Server, AppError> {
    let server = sqlx::query_as::<_, Server>(
        "INSERT INTO servers (
            id, user_id, name, hostname, ip_address, status, server_type,
            location, cpu_cores, ram_gb, disk_gb, os, created_at, updated_at
         )
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&payload.name)
    .bind(&payload.hostname)
    .bind(&payload.ip_address)
    .bind(ServerStatus::Provisioning.as_str())
    .bind(&payload.server_type)
    .bind(&payload.location)
    .bind(&payload.cpu_cores)
    .bind(&payload.ram_gb)
    .bind(&payload.disk_gb)
    .bind(&payload.os)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(db)
    .await?;

    Ok(server)
}

pub async fn update_server(
    db: &DbPool,
    id: Uuid,
    payload: UpdateServer,
) -> Result<Server, AppError> {
    // Get existing server
    let mut server = get_server(db, id).await?;

    // Update fields
    if let Some(name) = payload.name {
        server.name = name;
    }
    if let Some(hostname) = payload.hostname {
        server.hostname = hostname;
    }
    if let Some(ip_address) = payload.ip_address {
        server.ip_address = ip_address;
    }
    if let Some(status) = payload.status {
        server.status = status.as_str().to_string();
    }
    if let Some(location) = payload.location {
        server.location = Some(location);
    }
    if let Some(cpu_cores) = payload.cpu_cores {
        server.cpu_cores = Some(cpu_cores);
    }
    if let Some(ram_gb) = payload.ram_gb {
        server.ram_gb = Some(ram_gb);
    }
    if let Some(disk_gb) = payload.disk_gb {
        server.disk_gb = Some(disk_gb);
    }
    if let Some(os) = payload.os {
        server.os = Some(os);
    }

    // Save to database
    let server = sqlx::query_as::<_, Server>(
        "UPDATE servers
         SET name = $1, hostname = $2, ip_address = $3, status = $4, location = $5,
             cpu_cores = $6, ram_gb = $7, disk_gb = $8, os = $9, updated_at = $10
         WHERE id = $11
         RETURNING *"
    )
    .bind(&server.name)
    .bind(&server.hostname)
    .bind(&server.ip_address)
    .bind(&server.status)
    .bind(&server.location)
    .bind(&server.cpu_cores)
    .bind(&server.ram_gb)
    .bind(&server.disk_gb)
    .bind(&server.os)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(server)
}

pub async fn delete_server(db: &DbPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM servers WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Server not found".to_string()));
    }

    Ok(())
}

pub async fn get_server_metrics(
    db: &DbPool,
    server_id: Uuid,
) -> Result<Vec<ServerMetrics>, AppError> {
    let metrics = sqlx::query_as::<_, ServerMetrics>(
        "SELECT * FROM server_metrics
         WHERE server_id = $1
         ORDER BY timestamp DESC
         LIMIT 100"
    )
    .bind(server_id)
    .fetch_all(db)
    .await?;

    Ok(metrics)
}
