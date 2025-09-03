use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
        config::AppState, nodes::{
            types::{api::{NodeUpdateRequest, UpdateNodeResponse}, base::{Node}}
        }, shared::types::api::{ApiError, ApiResponse, ApiResult}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_node))
        .route("/", get(get_all_nodes))
        .route("/:id", put(update_node))
        .route("/:id", delete(delete_node))

}

async fn create_node(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Node>,
) -> ApiResult<Json<ApiResponse<Node>>> {
    let service = &state.services.node_service;
    let created_node = service.create_node(request.base).await?;
    
    Ok(Json(ApiResponse::success(created_node)))
}

async fn get_all_nodes(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Node>>>> {
    let service = &state.services.node_service;
    let nodes = service.get_all_nodes().await?;
    
    Ok(Json(ApiResponse::success(nodes)))
}

async fn update_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<NodeUpdateRequest>,
) -> ApiResult<Json<ApiResponse<UpdateNodeResponse>>> {

    let service = &state.services.node_service;

    let (updated_node, capability_test_changes, subnet_relationship_changes) = service.update_node(
        &id, 
        request, 
        ).await?;
    
    Ok(Json(ApiResponse::success(UpdateNodeResponse {
        node: updated_node,
        capability_test_changes,
        subnet_changes: subnet_relationship_changes
    })))
}

async fn delete_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = &state.services.node_service;
    
    // Check if node exists
    if service.get_node(&id).await?.is_none() {
        return Err(ApiError::not_found(&format!("Node '{}' not found", &id)));
    }
    
    service.delete_node(&id).await?;
    
    Ok(Json(ApiResponse::success(())))
}