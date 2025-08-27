use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;
use std::{sync::Arc};
use crate::{
    server::{
        nodes::{
            service::NodeService, types::{
                api::{CreateNodeRequest, NodeListResponse, NodeResponse, UpdateNodeRequest}, base::Node
            }
        },
        shared::types::api::{ApiError, ApiResponse, ApiResult},
    },
    server::config::AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_node))
        .route("/", get(get_all_nodes))
        .route("/:id", get(get_node))
        .route("/:id", put(update_node))
        .route("/:id", delete(delete_node))

}

async fn create_node(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateNodeRequest>,
) -> ApiResult<Json<ApiResponse<NodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let node = Node::new(request.node);
            
    let created_node = service.create_node(node).await?;
    
    Ok(Json(ApiResponse::success(NodeResponse {
        node: created_node,
    })))
}

async fn get_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<NodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    match service.get_node(&id).await? {
        Some(node) => Ok(Json(ApiResponse::success(NodeResponse { node }))),
        None => Err(ApiError::not_found(&format!("Node '{}' not found", &id))),
    }
}

async fn get_all_nodes(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<NodeListResponse>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let nodes = service.get_all_nodes().await?;
    let total = nodes.len();
    
    Ok(Json(ApiResponse::success(NodeListResponse { nodes, total })))
}

async fn update_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateNodeRequest>,
) -> ApiResult<Json<ApiResponse<NodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let mut node = service.get_node(&id).await?
        .ok_or_else(|| ApiError::not_found(&format!("Node '{}' not found", &id)))?;
    
    // Update fields if provided
    if let Some(name) = request.name {
        node.base.name = name;
    }
    if let Some(target) = request.target {
        node.base.target = target;
    }
    if let Some(description) = request.description {
        node.base.description = description;
    }
    if let Some(node_type) = request.node_type {
        node.base.node_type = node_type;
    }
    if let Some(capabilities) = request.capabilities {
        node.base.capabilities = capabilities;
    }
    if let Some(monitoring_interval) = request.monitoring_interval {
        node.base.monitoring_interval = monitoring_interval;
    }
    if let Some(assigned_tests) = request.assigned_tests {
        node.base.assigned_tests = assigned_tests;
    }
    
    let updated_node = service.update_node(node).await?;
    
    Ok(Json(ApiResponse::success(NodeResponse {
        node: updated_node,
    })))
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