use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::components::tests::types::CheckConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub r#type: String,
    pub config: CheckConfig,
    pub success: bool, 
    pub message: String,
    pub error: Option<String>,
    pub details: Option<serde_json::Value>,
    pub duration: u64,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerResult {
    pub name: String,
    pub description: String,
    pub checks: Vec<CheckResult>,
    pub success: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticResults {
    pub id: String,
    pub test_id: String,
    pub test_name: String,
    pub timestamp: DateTime<Utc>,
    pub layers: Vec<LayerResult>,
    pub success: bool,
    pub total_duration: u64,
}

impl DiagnosticResults {
    pub fn new(test_id: String, test_name: String, layers: Vec<LayerResult>) -> Self {
        let success = layers.iter().all(|layer| layer.success);
        let total_duration = layers.iter().map(|layer| layer.duration).sum();
        
        Self {
            id: Uuid::new_v4().to_string(),
            test_id,
            test_name,
            timestamp: Utc::now(),
            layers,
            success,
            total_duration,
        }
    }
}