use crate::{
    models::{vps::*, AppState},
    services::vps_service,
    utils::errors::AppError,
};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn list_vps(
    State(state): State<AppState>,
    // TODO: Extract user from JWT middleware
) -> Result<Json<Vec<Vps>>, AppError> {
    let vps = vps_service::list_vps(&state.db, None).await?;
    Ok(Json(vps))
}

pub async fn get_vps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vps>, AppError> {
    let vps = vps_service::get_vps(&state.db, id).await?;
    Ok(Json(vps))
}

pub async fn create_vps(
    State(state): State<AppState>,
    Json(payload): Json<CreateVps>,
) -> Result<Json<Vps>, AppError> {
    // TODO: Get user_id from JWT
    let user_id = Uuid::new_v4(); // Placeholder

    let vps = vps_service::create_vps(&state.db, &state.hetzner_client, user_id, payload).await?;
    Ok(Json(vps))
}

pub async fn update_vps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateVps>,
) -> Result<Json<Vps>, AppError> {
    let vps = vps_service::update_vps(&state.db, id, payload).await?;
    Ok(Json(vps))
}

pub async fn delete_vps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    vps_service::delete_vps(&state.db, &state.hetzner_client, id).await?;
    Ok(Json(()))
}

pub async fn power_on_vps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vps>, AppError> {
    let vps = vps_service::get_vps(&state.db, id).await?;

    if let Some(hetzner_id) = vps.hetzner_id {
        state.hetzner_client.power_on(hetzner_id).await?;

        // Wait a bit and sync status
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let vps = vps_service::sync_vps_status(&state.db, &state.hetzner_client, id).await?;
        Ok(Json(vps))
    } else {
        Err(AppError::BadRequest(
            "VPS not linked to Hetzner server".to_string(),
        ))
    }
}

pub async fn power_off_vps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vps>, AppError> {
    let vps = vps_service::get_vps(&state.db, id).await?;

    if let Some(hetzner_id) = vps.hetzner_id {
        state.hetzner_client.power_off(hetzner_id).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let vps = vps_service::sync_vps_status(&state.db, &state.hetzner_client, id).await?;
        Ok(Json(vps))
    } else {
        Err(AppError::BadRequest(
            "VPS not linked to Hetzner server".to_string(),
        ))
    }
}

pub async fn reboot_vps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vps>, AppError> {
    let vps = vps_service::get_vps(&state.db, id).await?;

    if let Some(hetzner_id) = vps.hetzner_id {
        state.hetzner_client.reboot(hetzner_id).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let vps = vps_service::sync_vps_status(&state.db, &state.hetzner_client, id).await?;
        Ok(Json(vps))
    } else {
        Err(AppError::BadRequest(
            "VPS not linked to Hetzner server".to_string(),
        ))
    }
}

pub async fn sync_vps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vps>, AppError> {
    let vps = vps_service::sync_vps_status(&state.db, &state.hetzner_client, id).await?;
    Ok(Json(vps))
}
