use crate::{
    daemon::{discovery::handlers as discovery_handlers, runtime::types::DaemonAppState},
    server::shared::types::api::{ApiResponse, ApiResult},
};
use axum::{Json, Router, routing::get};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<DaemonAppState>> {
    Router::new()
        .nest("/api/discovery", discovery_handlers::create_router())
        .route("/api/health", get(get_health))
}

async fn get_health() -> ApiResult<Json<ApiResponse<String>>> {
    tracing::info!("Received healthcheck request");

    Ok(Json(ApiResponse::success(
        "Netvisor Daemon Running".to_string(),
    )))
}
