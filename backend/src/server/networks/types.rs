use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NetworkBase {
    #[validate(length(min = 0, max = 100))]
    pub name: String,
    pub user_id: Uuid,
    pub is_default: bool,
}

impl NetworkBase {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            name: "My Network".to_string(),
            is_default: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: NetworkBase,
}

impl Network {
    pub fn new(base: NetworkBase) -> Self {
        let now = chrono::Utc::now();
        Network {
            base,
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.base.name, self.id)
    }
}
