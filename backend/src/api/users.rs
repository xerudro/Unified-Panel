use crate::{
    models::{user::*, AppState},
    services::user_service,
    utils::errors::AppError,
};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = user_service::list_users(&state.db).await?;
    let response: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
    Ok(Json(response))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_service::get_user(&state.db, id).await?;
    Ok(Json(user.into()))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_service::create_user(&state.db, payload).await?;
    Ok(Json(user.into()))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_service::update_user(&state.db, id, payload).await?;
    Ok(Json(user.into()))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    user_service::delete_user(&state.db, id).await?;
    Ok(Json(()))
}
