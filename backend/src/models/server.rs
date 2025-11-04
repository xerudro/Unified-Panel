use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    Online,
    Offline,
    Maintenance,
    Provisioning,
    Error,
}

impl ServerStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ServerStatus::Online => "online",
            ServerStatus::Offline => "offline",
            ServerStatus::Maintenance => "maintenance",
            ServerStatus::Provisioning => "provisioning",
            ServerStatus::Error => "error",
        }
    }
}

impl std::str::FromStr for ServerStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "online" => Ok(ServerStatus::Online),
            "offline" => Ok(ServerStatus::Offline),
            "maintenance" => Ok(ServerStatus::Maintenance),
            "provisioning" => Ok(ServerStatus::Provisioning),
            "error" => Ok(ServerStatus::Error),
            _ => Err(format!("Invalid server status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Server {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub hostname: String,
    pub ip_address: String,
    pub status: String,
    pub server_type: String,
    pub location: Option<String>,
    pub cpu_cores: Option<i32>,
    pub ram_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub os: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Server {
    pub fn get_status(&self) -> ServerStatus {
        self.status.parse().unwrap_or(ServerStatus::Offline)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateServer {
    pub name: String,
    pub hostname: String,
    pub ip_address: String,
    pub server_type: String,
    pub location: Option<String>,
    pub cpu_cores: Option<i32>,
    pub ram_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub os: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServer {
    pub name: Option<String>,
    pub hostname: Option<String>,
    pub ip_address: Option<String>,
    pub status: Option<ServerStatus>,
    pub location: Option<String>,
    pub cpu_cores: Option<i32>,
    pub ram_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub os: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ServerMetrics {
    pub id: Uuid,
    pub server_id: Uuid,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_in: i64,
    pub network_out: i64,
    pub timestamp: DateTime<Utc>,
}
