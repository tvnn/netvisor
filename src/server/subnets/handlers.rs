use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
        config::AppState, shared::types::api::{ApiError, ApiResponse, ApiResult}, subnets::{types::base::Subnet}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_subnet))
        .route("/", get(get_all_subnets))
        .route("/:id", put(update_subnet))
        .route("/:id", delete(delete_subnet))

}

async fn create_subnet(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Subnet>,
) -> ApiResult<Json<ApiResponse<Subnet>>> {

    let service = &state.services.subnet_service;
    let created_subnet = service.create_subnet(request).await?;
    
    Ok(Json(ApiResponse::success(created_subnet)))
}

async fn get_all_subnets(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Subnet>>>> {
    let service = &state.services.subnet_service;
    
    let subnets = service.get_all_subnets().await?;
    
    Ok(Json(ApiResponse::success(subnets)))
}

async fn update_subnet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<Subnet>,
) -> ApiResult<Json<ApiResponse<Subnet>>> {
    let service = &state.services.subnet_service;
    
    let mut subnet = service.get_subnet(&id).await?
        .ok_or_else(|| ApiError::not_found(&format!("Subnet '{}' not found", &id)))?;

    subnet.base = request.base;

    let updated_subnet = service.update_subnet(subnet).await?;
    
    Ok(Json(ApiResponse::success(updated_subnet)))
}

async fn delete_subnet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = &state.services.subnet_service;
    
    // Check if node exists
    if service.get_subnet(&id).await?.is_none() {
        return Err(ApiError::not_found(&format!("Subnet '{}' not found", &id)));
    }
    
    service.delete_subnet(&id).await?;
    
    Ok(Json(ApiResponse::success(())))
}