pub mod auth;
pub mod dashboard;
pub mod servers;
pub mod users;
pub mod vps;

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

        // VPS routes
        .route("/vps", get(vps::list_vps).post(vps::create_vps))
        .route("/vps/:id", get(vps::get_vps).put(vps::update_vps).delete(vps::delete_vps))
        .route("/vps/:id/power-on", post(vps::power_on_vps))
        .route("/vps/:id/power-off", post(vps::power_off_vps))
        .route("/vps/:id/reboot", post(vps::reboot_vps))
        .route("/vps/:id/sync", post(vps::sync_vps))

        // User routes
        .route("/users", get(users::list_users).post(users::create_user))
        .route("/users/:id", get(users::get_user).put(users::update_user).delete(users::delete_user))

        // Dashboard routes
        .route("/dashboard/stats", get(dashboard::get_stats))
        .route("/dashboard/activity", get(dashboard::get_activity))
        .route("/dashboard/health", get(dashboard::get_health))
        .route("/dashboard/servers", get(dashboard::get_servers))
        .route("/dashboard/hosting/activity", get(dashboard::get_hosting_activity))
        .route("/dashboard/hosting/storage", get(dashboard::get_hosting_storage))
}
