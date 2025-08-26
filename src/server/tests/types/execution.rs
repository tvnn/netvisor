use std::time::Instant;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::server::nodes::types::criticality::TestCriticality;

pub struct Timer {
    instant: Instant,
    datetime: DateTime<Utc>,
}

impl Timer {
    pub fn now() -> Self {
        Self {
            instant: Instant::now(),
            datetime: Utc::now(),
        }
    }
    
    pub fn elapsed_ms(&self) -> u64 {
        self.instant.elapsed().as_millis() as u64
    }
    
    pub fn datetime(&self) -> DateTime<Utc> {
        self.datetime
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub success: bool,
    pub message: String,
    pub duration_ms: u64,
    pub executed_at: DateTime<Utc>,
    pub details: Option<serde_json::Value>, // Test-specific result data
    pub criticality: Option<TestCriticality>
}

impl TestResult {
    pub fn error_result(error: anyhow::Error, criticality: Option<TestCriticality>, timer: Timer) -> Self {
        Self {
            criticality: criticality,
            success: false,
            message: "Error executing test".to_string(),
            details: Some(serde_json::json!({
                "error": error.to_string(),
            })),
            duration_ms: timer.elapsed_ms(),
            executed_at: timer.datetime(),
        }
    }
}