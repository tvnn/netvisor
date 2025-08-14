use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use std::{collections::HashMap, sync::Arc};
use crate::{
    api::{
        ApiError, ApiResponse, ApiResult
    },
    components::{
        nodes::{
            service::NodeService,
            types::{CapabilityRecommendations, CreateNodeRequest, Node, NodeCapability, NodeCompatibilityResponse, NodeListResponse, NodeResponse, NodeType, UpdateNodeRequest}
        },
        tests::types::TestType,
    },
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_node))
        .route("/", get(get_all_nodes))
        .route("/:id", get(get_node))
        .route("/:id", put(update_node))
        .route("/:id", delete(delete_node))
        .route("/:id/compatibility", get(get_node_compatibility))
        .route("/capability-recommendations", get(get_capability_recommendations_handler))
}

async fn create_node(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateNodeRequest>,
) -> ApiResult<Json<ApiResponse<NodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    let mut node = Node::from_name(request.node.name);
    node.base.domain = request.node.domain;
    node.base.ip = request.node.ip;
    node.base.port = request.node.port;
    node.base.path = request.node.path;
    node.base.description = request.node.description;
    node.base.capabilities = request.node.capabilities;
    node.base.node_type = request.node.node_type;
    node.base.assigned_tests = request.node.assigned_tests;
            
    let created_node = service.create_node(node).await?;
    
    Ok(Json(ApiResponse::success(NodeResponse {
        node: created_node,
    })))
}

async fn get_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<ApiResponse<NodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    match service.get_node(&id).await? {
        Some(node) => Ok(Json(ApiResponse::success(NodeResponse { node }))),
        None => Err(ApiError::node_not_found(&id)),
    }
}

async fn get_all_nodes(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<NodeListResponse>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    let nodes = service.get_all_nodes().await?;
    let total = nodes.len();
    
    Ok(Json(ApiResponse::success(NodeListResponse { nodes, total })))
}

async fn update_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateNodeRequest>,
) -> ApiResult<Json<ApiResponse<NodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    let mut node = service.get_node(&id).await?
        .ok_or_else(|| ApiError::node_not_found(&id))?;
    
    // Update fields if provided
    if let Some(name) = request.name {
        node.base.name = name;
    }
    if let Some(domain) = request.domain {
        node.base.domain = domain;
    }
    if let Some(ip) = request.ip {
        node.base.ip = ip;
    }
    if let Some(port) = request.port {
        node.base.port = port;
    }
    if let Some(path) = request.path {
        node.base.path = path;
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
    if let Some(monitoring_enabled) = request.monitoring_enabled {
        node.base.monitoring_enabled = monitoring_enabled;
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
    Path(id): Path<String>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    // Check if node exists
    if service.get_node(&id).await?.is_none() {
        return Err(ApiError::node_not_found(&id));
    }
    
    service.delete_node(&id).await?;
    
    Ok(Json(ApiResponse::success(())))
}

async fn get_node_compatibility(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<String>,
) -> ApiResult<Json<ApiResponse<NodeCompatibilityResponse>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    let node = service.get_node(&node_id).await?
        .ok_or_else(|| ApiError::node_not_found(&node_id))?;
    
    let compatible_test_types = service.get_compatible_test_types(&node);
    
    let all_test_types = vec![
        TestType::Connectivity,
        TestType::DirectIp,
        TestType::Ping,
        TestType::WellknownIp,
        TestType::DnsResolution,
        TestType::DnsOverHttps,
        TestType::VpnConnectivity,
        TestType::VpnTunnel,
        TestType::ServiceHealth,
        TestType::DaemonCommand,
        TestType::SshScript,
    ];
    
    let incompatible_test_types: Vec<TestType> = all_test_types
        .into_iter()
        .filter(|t| !compatible_test_types.contains(t))
        .collect();
    
    // TODO: Implement missing_capabilities analysis
    let missing_capabilities = Vec::new();
    
    Ok(Json(ApiResponse::success(NodeCompatibilityResponse {
        node_id: node_id.clone(),
        compatible_test_types,
        incompatible_test_types,
        missing_capabilities,
    })))
}

async fn get_capability_recommendations_handler(
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Json<ApiResponse<CapabilityRecommendations>>> {
    let node_type_str = params.get("node_type")
        .ok_or_else(|| ApiError::validation_error("node_type parameter required"))?;
    
    let node_type: NodeType = serde_json::from_str(&format!("\"{}\"", node_type_str))
        .map_err(|_| ApiError::validation_error("Invalid node type"))?;
    
    let recommendations = CapabilityRecommendations {
        all_capabilities: NodeCapability::all(),
        current_capabilities: vec![], // Not relevant for suggestions
        suggested_capabilities: node_type.typical_capabilities(),
    };
    
    Ok(Json(ApiResponse::success(recommendations)))
}