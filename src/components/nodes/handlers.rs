use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use crate::{
    api::{
        ApiResult, ApiResponse, ApiError,
        CreateNodeRequest, UpdateNodeRequest, AssignTestRequest, SetMonitoringRequest,
        NodeResponse, NodeListResponse, NodeCompatibilityResponse
    },
    core::{Node, TestType},
    components::nodes::service::NodeService,
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_node))
        .route("/", get(get_all_nodes))
        .route("/:id", get(get_node))
        .route("/:id", put(update_node))
        .route("/:id", delete(delete_node))
        .route("/:id/assign-test", post(assign_test))
        .route("/:id/remove-test/:test_type", delete(remove_test))
        .route("/:id/monitoring", put(set_monitoring))
        .route("/:id/compatibility", get(get_node_compatibility))
}

async fn create_node(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateNodeRequest>,
) -> ApiResult<Json<ApiResponse<NodeResponse>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    let mut node = Node::new(request.name);
    node.domain = request.domain;
    node.ip = request.ip;
    node.port = request.port;
    node.path = request.path;
    node.description = request.description;
    
    if let Some(node_type) = request.node_type {
        node.node_type = Some(node_type);
    }
    
    if let Some(capabilities) = request.capabilities {
        node.capabilities = capabilities;
    }
    
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
        node.name = name;
    }
    if let Some(domain) = request.domain {
        node.domain = Some(domain);
    }
    if let Some(ip) = request.ip {
        node.ip = Some(ip);
    }
    if let Some(port) = request.port {
        node.port = Some(port);
    }
    if let Some(path) = request.path {
        node.path = Some(path);
    }
    if let Some(description) = request.description {
        node.description = Some(description);
    }
    if let Some(node_type) = request.node_type {
        node.node_type = Some(node_type);
    }
    if let Some(capabilities) = request.capabilities {
        node.capabilities = capabilities;
    }
    if let Some(monitoring_enabled) = request.monitoring_enabled {
        node.monitoring_enabled = monitoring_enabled;
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

async fn assign_test(
    State(state): State<Arc<AppState>>,
    Path(_node_id): Path<String>,
    Json(request): Json<AssignTestRequest>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    service.assign_test_to_node(
        &request.node_id,
        request.test_type,
        request.test_config,
        request.criticality,
        request.monitor_interval_minutes,
    ).await?;
    
    Ok(Json(ApiResponse::success(())))
}

async fn remove_test(
    State(state): State<Arc<AppState>>,
    Path((node_id, test_type_str)): Path<(String, String)>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    // Parse test type from string
    let test_type: TestType = serde_json::from_str(&format!("\"{}\"", test_type_str))
        .map_err(|_| ApiError::validation_error("Invalid test type"))?;
    
    service.remove_test_from_node(&node_id, &test_type).await?;
    
    Ok(Json(ApiResponse::success(())))
}

async fn set_monitoring(
    State(state): State<Arc<AppState>>,
    Path(_node_id): Path<String>,
    Json(request): Json<SetMonitoringRequest>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeService::new(state.node_storage.clone());
    
    service.set_monitoring_enabled(&request.node_id, request.enabled).await?;
    
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