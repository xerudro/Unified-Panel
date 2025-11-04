use crate::config::Config;
use crate::database::DbPool;
use crate::services::vps_service::HetznerClient;
use std::sync::Arc;

pub mod user;
pub mod server;
pub mod auth;
pub mod vps;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub config: Arc<Config>,
    pub hetzner_client: HetznerClient,
}

impl AppState {
    pub fn new(db: DbPool, config: Config) -> Self {
        let hetzner_client = HetznerClient::new(config.hetzner_api_token.clone());
        Self {
            db,
            config: Arc::new(config),
            hetzner_client,
    pub hetzner_client: Arc<HetznerClient>,
}

impl AppState {
    pub fn new(db: DbPool, config: Config, hetzner_client: HetznerClient) -> Self {
        Self {
            db,
            config: Arc::new(config),
            hetzner_client: Arc::new(hetzner_client),
        }
    }
}
