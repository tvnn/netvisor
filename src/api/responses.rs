use serde::{Deserialize, Serialize};

// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// System status responses
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatusResponse {
    pub server_status: String,
    pub database_status: String,
    pub monitoring_active: bool,
    pub total_nodes: usize,
    pub total_groups: usize,
    pub uptime_seconds: u64,
}