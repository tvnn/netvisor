use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::shared::types::api::deserialize_empty_string_as_none;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupBase {
    pub name: String,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub description: Option<String>,
    pub service_bindings: Vec<ServiceBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBinding {
    pub service_id: Uuid,
    pub interface_id: Uuid
}

impl PartialEq for ServiceBinding {
    fn eq(&self, other: &Self) -> bool {
        self.interface_id == other.interface_id && self.service_id == other.service_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: GroupBase,
}

impl Group {
    pub fn new(base: GroupBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            base,
        }
    }

    pub fn from_name(name: String) -> Self {
        let base = GroupBase {
            name,
            description: None,
            service_bindings: Vec::new(),
        };

        Self::new(base)
    }
}