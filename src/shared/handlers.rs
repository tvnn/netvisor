use axum::{
    extract::{State},
    response::{Json},
    routing::{get, post, put},
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

// use crate::components::discovery::handlers::*;
use crate::components::nodes::handlers::*;
use crate::components::tests::handlers::*;
use crate::components::diagnostics::handlers::*;
use crate::AppState;

// Request/Response types
#[derive(Deserialize)]
pub struct QueryParams {
    pub limit: Option<u32>,
    pub test_id: Option<String>,
}

// API Response wrapper
#[derive(Serialize)]
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

// Health check
pub async fn health() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
    Json(ApiResponse::success(status))
}

// Configuration handlers
pub async fn get_config(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<crate::config::ServerConfig>> {
    Json(ApiResponse::success(state.config.clone()))
}

// Create the router
pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/health", get(health))
        
        // Node routes
        .route("/api/nodes", get(get_nodes).post(create_node))
        .route("/api/nodes/:id", put(update_node).delete(delete_node))
        
        // Test routes  
        .route("/api/tests", get(get_tests).post(create_test))
        .route("/api/tests/:id", put(update_test).delete(delete_test))
        
        // Diagnostic routes
        .route("/api/diagnostics/run/:test_id", post(run_diagnostics))
        .route("/api/diagnostics/results", get(get_diagnostic_results))
        
        // Check execution
        .route("/api/checks/:check_type", post(execute_check))
        
        // Configuration
        .route("/api/config", get(get_config))

        // Discovery routes
        // .route("/api/discovery/start", post(start_discovery))
        // .route("/api/discovery/stop", post(stop_discovery))
        // .route("/api/discovery/progress", get(get_discovery_progress))
        // .route("/api/discovery/devices", get(get_discovered_devices))
        // .route("/api/discovery/devices/:device_id/accept", post(accept_discovered_device))
        // .route("/api/discovery/devices/:device_id/reject", post(reject_discovered_device))
}