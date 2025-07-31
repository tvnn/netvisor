use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json},
};
use std::sync::Arc;
use crate::AppState;
use crate::shared::handlers::ApiResponse;
use super::types::{NetworkNode, CreateNodeRequest};

use crate::shared::storage::StorageError;

// Node handlers
pub async fn get_nodes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<NetworkNode>>>, StatusCode> {
    match state.node_storage.get_nodes().await {
        Ok(nodes) => Ok(Json(ApiResponse::success(nodes))),
        Err(e) => {
            tracing::error!("Failed to get nodes: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to get nodes: {}", e))))
        }
    }
}

pub async fn create_node(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateNodeRequest>,
) -> Result<Json<ApiResponse<NetworkNode>>, StatusCode> {
    let node = NetworkNode::new(
        request.name,
        request.domain,
        request.ip,
        request.port,
        request.path,
        request.description
    );

    match state.node_storage.save_node(&node).await {
        Ok(_) => Ok(Json(ApiResponse::success(node))),
        Err(e) => {
            tracing::error!("Failed to create node: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to create node: {}", e))))
        }
    }
}

pub async fn update_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<CreateNodeRequest>,
) -> Result<Json<ApiResponse<NetworkNode>>, StatusCode> {
    // Get existing node to preserve timestamps
    let mut node = match state.node_storage.get_node(&id).await {
        Ok(node) => node,
        Err(StorageError::NotFound) => return Ok(Json(ApiResponse::error("Node not found".to_string()))),
        Err(e) => {
            tracing::error!("Failed to get node: {}", e);
            return Ok(Json(ApiResponse::error(format!("Failed to get node: {}", e))));
        }
    };

    // Update fields
    node.name = request.name;
    node.domain = request.domain;
    node.ip = request.ip;
    node.port = request.port;
    node.path = request.path;
    node.description = request.description;
    node.updated_at = chrono::Utc::now();

    match state.node_storage.update_node(&id, &node).await {
        Ok(_) => Ok(Json(ApiResponse::success(node))),
        Err(e) => {
            tracing::error!("Failed to update node: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to update node: {}", e))))
        }
    }
}

pub async fn delete_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    println!("DELETE request for node ID: '{}'", id);
    match state.node_storage.delete_node(&id).await {
        Ok(_) => {
            Ok(Json(ApiResponse::success(())))
        }
        Err(StorageError::NotFound) => {
            Ok(Json(ApiResponse::error("Node not found".to_string())))
        }
        Err(e) => {
            tracing::error!("Failed to delete node: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to delete node: {}", e))))
        }
    }
}