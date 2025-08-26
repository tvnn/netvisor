use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DaemonStatus {
    Active,
    Inactive,
    Error,
}

/// Daemon record stored in server database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonBase {
    pub ip: IpAddr,
    pub port: u16,
    pub hostname: Option<String>,
    pub status: DaemonStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Daemon {
    pub id: Uuid,
    pub last_seen: DateTime<Utc>,
    pub registered_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: DaemonBase,
}

impl Daemon {
    pub fn new(base: DaemonBase) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            base,
            last_seen: now,
            registered_at: now,
        }
    }

    pub fn endpoint_url(&self) -> String {
        format!("http://{}:{}", self.base.ip, self.base.port)
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
        self.base.status = DaemonStatus::Active;
    }
}