use std::sync::Arc;
use axum::{
    response::Json, routing::post, Router
};
use crate::daemon::runtime::types::DaemonAppState;
use crate::{
    server::{
        shared::types::api::{ApiResponse, ApiResult},
    },
};

pub fn create_router() -> Router<Arc<DaemonAppState>> {
    Router::new()
        .route("/health", post(handle_healthcheck_request))
}

async fn handle_healthcheck_request(
) -> ApiResult<Json<ApiResponse<String>>> {
    tracing::info!("Received healthcheck request");

    Ok(Json(ApiResponse::success("Netvisor Daemon Running".to_string())))
}