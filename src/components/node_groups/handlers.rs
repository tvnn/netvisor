use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use crate::{
    api::{ApiResult, ApiResponse, ApiError,},
    components::node_groups::{
        types::{NodeGroup, CreateNodeGroupRequest, NodeGroupResponse, NodeGroupListResponse, UpdateNodeGroupRequest},
        service::NodeGroupService
    },
    components::nodes::types::NodeListResponse,
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_node_group))
        .route("/", get(get_all_node_groups))
        .route("/:id", get(get_node_group))
        .route("/:id", put(update_node_group))
        .route("/:id", delete(delete_node_group))
        .route("/:id/nodes", get(get_group_nodes))
        .route("/:id/add-node/:node_id", post(add_node_to_group))
        .route("/:id/remove-node/:node_id", delete(remove_node_from_group))
}

async fn create_node_group(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateNodeGroupRequest>,
) -> ApiResult<Json<ApiResponse<NodeGroupResponse>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    let mut group = NodeGroup::from_name(request.group.name);
    group.base.description = request.group.description;
    group.base.node_sequence = request.group.node_sequence;
    group.base.auto_diagnostic_enabled = request.group.auto_diagnostic_enabled;
    
    let created_group = service.create_group(group).await?;
    
    Ok(Json(ApiResponse::success(NodeGroupResponse {
        group: created_group,
    })))
}

async fn get_node_group(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<ApiResponse<NodeGroupResponse>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    match service.get_group(&id).await? {
        Some(group) => Ok(Json(ApiResponse::success(NodeGroupResponse { group }))),
        None => Err(ApiError::group_not_found(&id)),
    }
}

async fn get_all_node_groups(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<NodeGroupListResponse>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    let groups = service.get_all_groups().await?;
    let total = groups.len();
    
    Ok(Json(ApiResponse::success(NodeGroupListResponse { groups, total })))
}

async fn update_node_group(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateNodeGroupRequest>,
) -> ApiResult<Json<ApiResponse<NodeGroupResponse>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    let mut group = service.get_group(&id).await?
        .ok_or_else(|| ApiError::group_not_found(&id))?;
    
    if let Some(name) = request.name {
        group.base.name = name;
    }
    if let Some(description) = request.description {
        group.base.description = Some(description);
    }
    if let Some(node_sequence) = request.node_sequence {
        group.base.node_sequence = node_sequence;
    }
    if let Some(auto_diagnostic_enabled) = request.auto_diagnostic_enabled {
        group.base.auto_diagnostic_enabled = auto_diagnostic_enabled;
    }
    
    let updated_group = service.update_group(group).await?;
    
    Ok(Json(ApiResponse::success(NodeGroupResponse {
        group: updated_group,
    })))
}

async fn delete_node_group(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    service.delete_group(&id).await?;
    Ok(Json(ApiResponse::success(())))
}

async fn get_group_nodes(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<String>,
) -> ApiResult<Json<ApiResponse<NodeListResponse>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    let nodes = service.get_group_nodes(&group_id).await?;
    let total = nodes.len();
    
    Ok(Json(ApiResponse::success(NodeListResponse { nodes, total })))
}

async fn add_node_to_group(
    State(state): State<Arc<AppState>>,
    Path((group_id, node_id)): Path<(String, String)>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    service.add_node_to_group(&group_id, &node_id, None).await?;
    Ok(Json(ApiResponse::success(())))
}

async fn remove_node_from_group(
    State(state): State<Arc<AppState>>,
    Path((group_id, node_id)): Path<(String, String)>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = NodeGroupService::new(
        state.node_group_storage.clone(),
        state.node_storage.clone(),
    );
    
    service.remove_node_from_group(&group_id, &node_id).await?;
    Ok(Json(ApiResponse::success(())))
}