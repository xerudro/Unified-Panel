use crate::{
    models::{server::*, AppState},
    services::server_service,
    utils::errors::AppError,
};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn list_servers(
    State(state): State<AppState>,
    // user_id should come from JWT middleware
) -> Result<Json<Vec<Server>>, AppError> {
    // For now, list all servers (should be filtered by user role)
    let servers = server_service::list_servers(&state.db).await?;
    Ok(Json(servers))
}

pub async fn get_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Server>, AppError> {
    let server = server_service::get_server(&state.db, id).await?;
    Ok(Json(server))
}

pub async fn create_server(
    State(state): State<AppState>,
    Json(payload): Json<CreateServer>,
) -> Result<Json<Server>, AppError> {
    // user_id should come from JWT middleware
    let user_id = Uuid::new_v4(); // Placeholder
    let server = server_service::create_server(&state.db, user_id, payload).await?;
    Ok(Json(server))
}

pub async fn update_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateServer>,
) -> Result<Json<Server>, AppError> {
    let server = server_service::update_server(&state.db, id, payload).await?;
    Ok(Json(server))
}

pub async fn delete_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    server_service::delete_server(&state.db, id).await?;
    Ok(Json(()))
}

pub async fn get_server_metrics(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<ServerMetrics>>, AppError> {
    let metrics = server_service::get_server_metrics(&state.db, id).await?;
    Ok(Json(metrics))
}
