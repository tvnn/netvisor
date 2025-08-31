use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::server::{config::AppState, node_groups::{service::NodeGroupService, types::{NodeGroup}}, shared::types::api::{ApiError, ApiResponse, ApiResult}};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_node_group))
        .route("/", get(get_all_node_groups))
        .route("/:id", put(update_node_group))
        .route("/:id", delete(delete_node_group))
}

async fn create_node_group(
    State(state): State<Arc<AppState>>,
    Json(request): Json<NodeGroup>,
) -> ApiResult<Json<ApiResponse<NodeGroup>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    let group = NodeGroup::new(request.base);
    
    let created_group = service.create_group(group).await?;
    
    Ok(Json(ApiResponse::success( created_group)))
}

async fn get_all_node_groups(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<NodeGroup>>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    let groups = service.get_all_groups().await?;
    
    Ok(Json(ApiResponse::success(groups)))
}

async fn update_node_group(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<NodeGroup>,
) -> ApiResult<Json<ApiResponse<NodeGroup>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    let mut group = service.get_group(&id).await?
        .ok_or_else(|| ApiError::not_found(&format!("Node group '{}' not found", &id)))?;

    group.base = request.base;    
    let updated_group = service.update_group(group).await?;
    
    Ok(Json(ApiResponse::success(updated_group)))
}

async fn delete_node_group(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    service.delete_group(&id).await?;
    Ok(Json(ApiResponse::success(())))
}