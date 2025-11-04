use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::{
    models::{auth::*, AppState, user::UserResponse},
    services::auth_service,
    utils::errors::AppError,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let (token, user) = auth_service::login(&state.db, &state.config, payload).await?;

    Ok(Json(LoginResponse {
        token,
        user: user.into(),
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = auth_service::register(&state.db, payload).await?;
    Ok(Json(user.into()))
}

pub async fn logout() -> impl IntoResponse {
    // For JWT, logout is typically handled client-side
    // We can implement token blacklisting if needed
    StatusCode::OK
}

pub async fn me(
    State(state): State<AppState>,
    // Extract user from JWT (requires middleware)
) -> Result<Json<UserResponse>, AppError> {
    // This will be implemented with auth middleware
    todo!("Implement with auth middleware")
}
