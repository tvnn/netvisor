use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonBase {
    pub host_id: Uuid
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
    pub fn new(id: Uuid, base: DaemonBase) -> Self {
        let now = Utc::now();
        Self {
            id,
            base,
            last_seen: now,
            registered_at: now,
        }
    }
}