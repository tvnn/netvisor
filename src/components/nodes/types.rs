use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: String,
    pub name: String,
    pub domain: Option<String>,
    pub ip: Option<String>,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NetworkNode {
    pub fn new(name: String, domain: Option<String>, ip: Option<String>, port: Option<i64>, path: Option<String>, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            domain,
            ip,
            port,
            path,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateNodeRequest {
    pub name: String,
    pub description: Option<String>,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub ip: Option<String>
}