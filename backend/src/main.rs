mod api;
mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "unified_panel=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;

    // Initialize database
    let db_pool = database::create_pool(&config.database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await?;

    tracing::info!("Database migrations completed successfully");

    // Initialize Hetzner client
    let hetzner_token = std::env::var("HETZNER_API_TOKEN")
        .expect("HETZNER_API_TOKEN must be set");
    let hetzner_client = services::vps_service::HetznerClient::new(hetzner_token);

    // Create application state
    let app_state = models::AppState::new(db_pool, config.clone(), hetzner_client);

    // Build routes
    let app = Router::new()
        // Public routes
        .route("/", get(handlers::pages::index))
        .route("/login", get(handlers::pages::login_page))
        .route("/register", get(handlers::pages::register_page))
        .route("/api/auth/login", post(api::auth::login))
        .route("/api/auth/register", post(api::auth::register))
        // API routes
        .nest("/api", api::router())

        // Dashboard routes (protected)
        .route("/dashboard", get(handlers::pages::dashboard))
        .route("/servers", get(handlers::pages::servers_page))
        .route("/vps", get(handlers::pages::vps_page))
        .route("/users", get(handlers::pages::users_page))
        .route("/monitoring", get(handlers::pages::monitoring_page))
        .route("/settings", get(handlers::pages::settings_page))

        // Static files
        .nest_service("/static", ServeDir::new("static"))

        // Middleware
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())

        // State
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("🚀 Unified Hosting Panel starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
