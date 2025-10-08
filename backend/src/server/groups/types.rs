use crate::server::{
    shared::types::api::deserialize_empty_string_as_none,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::server::services::types::bindings::ServiceBinding;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupType {
    NetworkPath,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct GroupBase {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    #[validate(length(min = 0, max = 500))]
    pub description: Option<String>,
    pub service_bindings: Vec<ServiceBinding>,
    pub group_type: GroupType,
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
