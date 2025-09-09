use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::server::{config::AppState, host_groups::{types::{HostGroup}}, shared::types::api::{ApiError, ApiResponse, ApiResult}};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_host_group))
        .route("/", get(get_all_host_groups))
        .route("/:id", put(update_host_group))
        .route("/:id", delete(delete_host_group))
}

async fn create_host_group(
    State(state): State<Arc<AppState>>,
    Json(request): Json<HostGroup>,
) -> ApiResult<Json<ApiResponse<HostGroup>>> {
    let service = &state.services.host_group_service;
    
    let group = HostGroup::new(request.base);
    
    let created_group = service.create_group(group).await?;
    
    Ok(Json(ApiResponse::success( created_group)))
}

async fn get_all_host_groups(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<HostGroup>>>> {
    let service = &state.services.host_group_service;
    
    let groups = service.get_all_groups().await?;
    
    Ok(Json(ApiResponse::success(groups)))
}

async fn update_host_group(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<HostGroup>,
) -> ApiResult<Json<ApiResponse<HostGroup>>> {
    let service = &state.services.host_group_service;
    
    let mut group = service.get_group(&id).await?
        .ok_or_else(|| ApiError::not_found(&format!("Host group '{}' not found", &id)))?;

    group.base = request.base;    
    let updated_group = service.update_group(group).await?;
    
    Ok(Json(ApiResponse::success(updated_group)))
}

async fn delete_host_group(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = &state.services.host_group_service;
    
    service.delete_group(&id).await?;
    Ok(Json(ApiResponse::success(())))
}