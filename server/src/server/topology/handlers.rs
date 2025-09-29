use crate::server::{
    config::AppState,
    shared::types::api::{ApiResponse, ApiResult},
};
use axum::{extract::State, response::Json, routing::get, Router};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_topology))
}

async fn get_topology(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<serde_json::Value>>> {
    let service = &state.services.topology_service;
    let graph = service.build_graph().await?;

    let json = serde_json::to_value(&graph)?;

    Ok(Json(ApiResponse::success(json)))
}
