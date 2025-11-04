use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    user_email: String,
}

#[derive(Template)]
#[template(path = "servers.html")]
struct ServersTemplate {}

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {}

#[derive(Template)]
#[template(path = "monitoring.html")]
struct MonitoringTemplate {}

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    Html(template.render().unwrap())
}

pub async fn login_page() -> impl IntoResponse {
    let template = LoginTemplate {};
    Html(template.render().unwrap())
}

pub async fn register_page() -> impl IntoResponse {
    let template = RegisterTemplate {};
    Html(template.render().unwrap())
}

pub async fn dashboard() -> impl IntoResponse {
    // TODO: Get user from JWT
    let template = DashboardTemplate {
        user_email: "admin@example.com".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn servers_page() -> impl IntoResponse {
    let template = ServersTemplate {};
    Html(template.render().unwrap())
}

pub async fn users_page() -> impl IntoResponse {
    let template = UsersTemplate {};
    Html(template.render().unwrap())
}

pub async fn monitoring_page() -> impl IntoResponse {
    let template = MonitoringTemplate {};
    Html(template.render().unwrap())
}

pub async fn settings_page() -> impl IntoResponse {
    let template = SettingsTemplate {};
    Html(template.render().unwrap())
}
