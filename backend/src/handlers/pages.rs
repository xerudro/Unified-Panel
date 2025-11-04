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
    user_role: String,
    total_servers: i32,
    total_websites: i32,
    total_users: i32,
}

#[derive(Template)]
#[template(path = "servers.html")]
struct ServersTemplate {
    page_title: String,
}

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    page_title: String,
}

#[derive(Template)]
#[template(path = "monitoring.html")]
struct MonitoringTemplate {
    page_title: String,
}

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    page_title: String,
}

#[derive(Template)]
#[template(path = "vps.html")]
struct VpsTemplate {
    page_title: String,
}

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
    // TODO: Get user from JWT and fetch real stats from database
    let template = DashboardTemplate {
        user_email: "admin@example.com".to_string(),
        user_role: "admin".to_string(), // Can be: admin, reseller, user
        total_servers: 12,
        total_websites: 45,
        total_users: 156,
    };
    Html(template.render().unwrap())
}

pub async fn servers_page() -> impl IntoResponse {
    let template = ServersTemplate {
        page_title: "Servers".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn users_page() -> impl IntoResponse {
    let template = UsersTemplate {
        page_title: "Users".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn monitoring_page() -> impl IntoResponse {
    let template = MonitoringTemplate {
        page_title: "Monitoring".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn settings_page() -> impl IntoResponse {
    let template = SettingsTemplate {
        page_title: "Settings".to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn vps_page() -> impl IntoResponse {
    let template = VpsTemplate {
        page_title: "VPS Management".to_string(),
    };
    Html(template.render().unwrap())
}
