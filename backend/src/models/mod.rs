use crate::config::Config;
use crate::database::DbPool;
use std::sync::Arc;

pub mod user;
pub mod server;
pub mod auth;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(db: DbPool, config: Config) -> Self {
        Self {
            db,
            config: Arc::new(config),
        }
    }
}
