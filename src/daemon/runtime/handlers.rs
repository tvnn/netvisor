use std::sync::Arc;

use axum::{
    response::Json,
    routing::{get},
    Router,
};
use crate::{
    daemon::shared::storage::ConfigStore, server::shared::types::api::ApiResponse
};

pub fn create_router() -> Router<Arc<ConfigStore>> {
    Router::new()
        .route("/health", get(health_check))
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("Daemon is healthy"))
}