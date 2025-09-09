use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::shared::types::api::deserialize_empty_string_as_none;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostGroupBase {
    pub name: String,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub description: Option<String>,
    pub hosts: Vec<Uuid>,  // Ordered diagnostic sequence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostGroup {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: HostGroupBase,
}

impl HostGroup {
    pub fn new(base: HostGroupBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            base,
        }
    }

    pub fn from_name(name: String) -> Self {
        let base = HostGroupBase {
            name,
            description: None,
            hosts: Vec::new(),
        };

        Self::new(base)
    }
}