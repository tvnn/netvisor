use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
        config::AppState, hosts::types::{api::HostUpdateRequest, base::Host}, shared::types::api::{ApiError, ApiResponse, ApiResult}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_host))
        .route("/", get(get_all_hosts))
        .route("/:id", put(update_host))
        .route("/:id", delete(delete_host))
        .route("/:destination_host/consolidate/:other_host", put(consolidate_hosts))

}

async fn create_host(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Host>,
) -> ApiResult<Json<ApiResponse<Host>>> {
    let service = &state.services.host_service;
    let created_host = service.create_host(request.base).await?;
    
    Ok(Json(ApiResponse::success(created_host)))
}

async fn get_all_hosts(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Host>>>> {
    let service = &state.services.host_service;
    let hosts = service.get_all_hosts().await?;
    
    Ok(Json(ApiResponse::success(hosts)))
}

async fn update_host(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<HostUpdateRequest>,
) -> ApiResult<Json<ApiResponse<Host>>> {

    let service = &state.services.host_service;

    let updated_host = service.update_host(
        &id, 
        request, 
        ).await?;
    
    Ok(Json(ApiResponse::success(updated_host)))
}

async fn consolidate_hosts(
    State(state): State<Arc<AppState>>,
    Path((destination_host_id, other_host_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<ApiResponse<Host>>> {

    let host_service = &state.services.host_service;
    
    let destination_host = host_service.get_host(&destination_host_id).await?.ok_or_else(|| ApiError::not_found("Could not find host"))?;
    let other_host = host_service.get_host(&other_host_id).await?.ok_or_else(|| ApiError::not_found("Could not find host to convert"))?;

    let updated_host = host_service.consolidate_hosts(destination_host, other_host).await?;
    
    Ok(Json(ApiResponse::success(updated_host)))
}

async fn delete_host(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = &state.services.host_service;
    
    // Check if host exists
    if service.get_host(&id).await?.is_none() {
        return Err(ApiError::not_found(&format!("Host '{}' not found", &id)));
    }
    
    service.delete_host(&id).await?;
    
    Ok(Json(ApiResponse::success(())))
}