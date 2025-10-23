use crate::server::{
    config::AppState,
    services::types::base::Service,
    shared::types::api::{ApiError, ApiResponse, ApiResult},
};
use axum::{
    Router,
    extract::{Query, State},
    response::Json,
    routing::get,
};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_all_services))
}

async fn get_all_services(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Json<ApiResponse<Vec<Service>>>> {
    let network_id = params
        .get("network_id")
        .and_then(|id| Uuid::parse_str(id).ok())
        .ok_or_else(|| ApiError::bad_request("network_id query parameter required"))?;

    let service_service = &state.services.service_service;

    let subnets = service_service.get_all_services(&network_id).await?;

    Ok(Json(ApiResponse::success(subnets)))
}
