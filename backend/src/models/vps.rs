use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VpsStatus {
    Running,
    Starting,
    Stopping,
    Stopped,
    Deleting,
    Error,
}

impl VpsStatus {
    pub fn as_str(&self) -> &str {
        match self {
            VpsStatus::Running => "running",
            VpsStatus::Starting => "starting",
            VpsStatus::Stopping => "stopping",
            VpsStatus::Stopped => "stopped",
            VpsStatus::Deleting => "deleting",
            VpsStatus::Error => "error",
        }
    }
}

impl std::str::FromStr for VpsStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "running" => Ok(VpsStatus::Running),
            "starting" => Ok(VpsStatus::Starting),
            "stopping" => Ok(VpsStatus::Stopping),
            "stopped" => Ok(VpsStatus::Stopped),
            "deleting" => Ok(VpsStatus::Deleting),
            "error" => Ok(VpsStatus::Error),
            _ => Err(format!("Invalid VPS status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vps {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub hetzner_id: Option<i64>,
    pub status: String,
    pub server_type: String,
    pub location: String,
    pub image: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub cpu_cores: i32,
    pub ram_gb: i32,
    pub disk_gb: i32,
    pub monthly_cost: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Vps {
    pub fn get_status(&self) -> VpsStatus {
        self.status.parse().unwrap_or(VpsStatus::Error)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateVps {
    pub name: String,
    pub server_type: String,
    pub location: String,
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub user_data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVps {
    pub name: Option<String>,
    pub status: Option<VpsStatus>,
}

// Hetzner API Response Models
#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerServer {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub public_net: HetznerPublicNet,
    pub server_type: HetznerServerType,
    pub datacenter: HetznerDatacenter,
    pub image: HetznerImage,
    pub created: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerPublicNet {
    pub ipv4: Option<HetznerIpv4>,
    pub ipv6: Option<HetznerIpv6>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerIpv4 {
    pub ip: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerIpv6 {
    pub ip: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerServerType {
    pub name: String,
    pub cores: i32,
    pub memory: f64,
    pub disk: i32,
    pub prices: Vec<HetznerPrice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerPrice {
    pub location: String,
    pub price_monthly: HetznerPriceDetails,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerPriceDetails {
    pub gross: String,
    pub net: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerDatacenter {
    pub name: String,
    pub location: HetznerLocation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerLocation {
    pub name: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HetznerImage {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct HetznerServerResponse {
    pub server: HetznerServer,
}

#[derive(Debug, Deserialize)]
pub struct HetznerServersResponse {
    pub servers: Vec<HetznerServer>,
}

#[derive(Debug, Serialize)]
pub struct HetznerCreateServerRequest {
    pub name: String,
    pub server_type: String,
    pub location: String,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    pub start_after_create: bool,
}
