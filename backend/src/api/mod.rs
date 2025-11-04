pub mod auth;
pub mod servers;
pub mod users;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::models::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        // Auth routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me))

        // Server routes
        .route("/servers", get(servers::list_servers).post(servers::create_server))
        .route("/servers/:id", get(servers::get_server).put(servers::update_server).delete(servers::delete_server))
        .route("/servers/:id/metrics", get(servers::get_server_metrics))

        // User routes
        .route("/users", get(users::list_users).post(users::create_user))
        .route("/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))
}
