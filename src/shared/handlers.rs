use axum::{routing::get, Router, Json};
use std::sync::Arc;
use crate::{
    components::{
        nodes::handlers as node_handlers,
        node_groups::handlers as group_handlers,
        // diagnostics::handlers as diagnostic_handlers, // TODO: Implement
    },
    api::{ApiResponse, SystemStatusResponse},
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/status", get(system_status))
        .nest("/api/nodes", node_handlers::create_router())
        .nest("/api/groups", group_handlers::create_router()) // TODO: Implement
        // .nest("/api/diagnostics", diagnostic_handlers::create_router()) // TODO: Implement
        // .nest("/api/monitoring", monitoring_handlers::create_router()) // TODO: Implement
        // .nest("/api/discovery", discovery_handlers::create_router()) // TODO: Implement
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("NetFrog server is running".to_string()))
}

async fn system_status(
    // State(_state): State<Arc<AppState>>, // TODO: Use state for real metrics
) -> Json<ApiResponse<SystemStatusResponse>> {
    let response = SystemStatusResponse {
        server_status: "running".to_string(),
        database_status: "connected".to_string(),
        monitoring_active: false, // TODO: Implement monitoring status
        total_nodes: 0, // TODO: Get from storage
        total_groups: 0, // TODO: Get from storage
        uptime_seconds: 0, // TODO: Track uptime
    };
    
    Json(ApiResponse::success(response))
}