use crate::server::{
    config::AppState,
    services::types::base::Service,
    shared::types::api::{ApiResponse, ApiResult},
};
use axum::{
    extract::{State},
    response::Json,
    routing::{get},
    Router,
};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_services))
}

async fn get_all_services(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Service>>>> {
    let service_service = &state.services.service_service;

    let subnets = service_service.get_all_services().await?;

    Ok(Json(ApiResponse::success(subnets)))
}