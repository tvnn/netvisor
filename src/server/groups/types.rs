use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::{hosts::types::targets::ServiceBinding, shared::types::api::deserialize_empty_string_as_none};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupType {
    NetworkPath
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupBase {
    pub name: String,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub description: Option<String>,
    pub service_bindings: Vec<ServiceBinding>,
    pub group_type: GroupType
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
}