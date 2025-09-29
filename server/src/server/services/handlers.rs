use crate::server::{
    config::AppState,
    services::types::base::Service,
    shared::types::api::{ApiError, ApiResponse, ApiResult},
};
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use uuid::Uuid;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_service))
        .route("/", get(get_all_services))
        .route("/", put(update_service))
        .route("/:id", delete(delete_service))
}

async fn create_service(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Service>,
) -> ApiResult<Json<ApiResponse<Service>>> {
    let service = &state.services.service_service;
    let created_subnet = service.create_service(request).await?;

    Ok(Json(ApiResponse::success(created_subnet)))
}

async fn get_all_services(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Service>>>> {
    let service_service = &state.services.service_service;

    let subnets = service_service.get_all_services().await?;

    Ok(Json(ApiResponse::success(subnets)))
}

async fn update_service(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Service>,
) -> ApiResult<Json<ApiResponse<Service>>> {
    let service_service = &state.services.service_service;

    let updated_service = service_service.update_service(request).await?;

    Ok(Json(ApiResponse::success(updated_service)))
}

async fn delete_service(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service_service = &state.services.service_service;

    // Check if host exists
    if service_service.get_service(&id).await?.is_none() {
        return Err(ApiError::not_found(&format!("Service '{}' not found", &id)));
    }

    service_service.delete_service(&id, true).await?;

    Ok(Json(ApiResponse::success(())))
}
