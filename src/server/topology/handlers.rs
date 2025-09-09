use axum::{
    extract::{State},
    response::Json,
    routing::{get},
    Router,
};
use std::{sync::Arc};
use crate::server::{
        config::AppState, shared::types::api::{ApiResponse, ApiResult}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_topology))

}

async fn get_topology(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<String>>> {

    let service = &state.services.topology_service;
    let graph = service.generate_topology_graph().await?;
    
    Ok(Json(ApiResponse::success(graph)))
}