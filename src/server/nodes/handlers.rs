use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Serialize};
use strum::IntoDiscriminant;
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};
use crate::server::{
        capabilities::types::base::CapabilityDiscriminants, config::AppState, nodes::{
            service::NodeService, types::base::Node
        }, shared::types::api::{ApiError, ApiResponse, ApiResult}, tests::types::base::TestDiscriminants
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

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
struct UpdateNodeResponse {
    node: Node,
    capability_test_changes: HashMap<CapabilityDiscriminants, UpdateNodeCapabilityTestChange>
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
struct UpdateNodeCapabilityTestChange {
    newly_compatible: Vec<TestDiscriminants>, 
    incompatible: Vec<TestDiscriminants>
}

async fn update_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<Node>,
) -> ApiResult<Json<ApiResponse<UpdateNodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let mut node = service.get_node(&id).await?
        .ok_or_else(|| ApiError::not_found(&format!("Node '{}' not found", &id)))?;

    let node_context = node.clone().as_context();

    let mut capability_test_changes: HashMap<CapabilityDiscriminants, UpdateNodeCapabilityTestChange> = HashMap::new();

    for capability in node.base.capabilities.iter_mut() {
        let (newly_compatible, incompatible) = capability.validate_node_capability_test_compatibility(&node_context);

        capability.config_base_mut().remove_tests(incompatible.clone());
        capability.config_base_mut().add_tests(newly_compatible.clone());

        capability_test_changes.insert(capability.discriminant(), UpdateNodeCapabilityTestChange {
            newly_compatible: newly_compatible.iter().map(|ct| ct.test.discriminant()).collect(),
            incompatible
        });
    }

    node.base = request.base;    
    let updated_node = service.update_node(node).await?;
    
    Ok(Json(ApiResponse::success(UpdateNodeResponse {
        node: updated_node,
        capability_test_changes
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