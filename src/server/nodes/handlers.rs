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
            service::NodeService, types::{
                base::{Node}
            }
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
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let node = Node::new(request.base);
            
    let created_node = service.create_node(node).await?;
    
    Ok(Json(ApiResponse::success(created_node)))
}

async fn get_all_nodes(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Node>>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let nodes = service.get_all_nodes().await?;
    
    Ok(Json(ApiResponse::success(nodes)))
}

async fn update_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<Node>,
) -> ApiResult<Json<ApiResponse<Node>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let mut node = service.get_node(&id).await?
        .ok_or_else(|| ApiError::not_found(&format!("Node '{}' not found", &id)))?;

    node.base = request.base;    
    let updated_node = service.update_node(node).await?;
    
    Ok(Json(ApiResponse::success(updated_node)))
}

async fn delete_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    // Check if node exists
    if service.get_node(&id).await?.is_none() {
        return Err(ApiError::not_found(&format!("Node '{}' not found", &id)));
    }
    
    service.delete_node(&id).await?;
    
    Ok(Json(ApiResponse::success(())))
}