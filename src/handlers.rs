use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::{Json},
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{AppState, types::*, storage::{StorageError}, network_checks};

// API Response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// Request/Response types
#[derive(Deserialize)]
pub struct CreateNodeRequest {
    name: String,
    description: Option<String>,
    port: Option<i64>,
    path: Option<String>,
    domain: Option<String>,
    ip: Option<String>
}

#[derive(Deserialize)]
pub struct CreateTestRequest {
    name: String,
    description: Option<String>,
    layers: serde_json::Value, // Will be parsed as Vec<Layer>
}

#[derive(Deserialize)]
pub struct ExecuteCheckRequest {
    config: CheckConfig,
}

#[derive(Deserialize)]
pub struct QueryParams {
    limit: Option<u32>,
    test_id: Option<String>,
}

// Health check
pub async fn health() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
    Json(ApiResponse::success(status))
}

// Node handlers
pub async fn get_nodes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<NetworkNode>>>, StatusCode> {
    match state.storage.get_nodes().await {
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

    match state.storage.save_node(&node).await {
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
    let mut node = match state.storage.get_node(&id).await {
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

    match state.storage.update_node(&id, &node).await {
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
    match state.storage.delete_node(&id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(StorageError::NotFound) => Ok(Json(ApiResponse::error("Node not found".to_string()))),
        Err(e) => {
            tracing::error!("Failed to delete node: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to delete node: {}", e))))
        }
    }
}

// Test handlers
pub async fn get_tests(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Test>>>, StatusCode> {
    match state.storage.get_tests().await {
        Ok(tests) => Ok(Json(ApiResponse::success(tests))),
        Err(e) => {
            tracing::error!("Failed to get tests: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to get tests: {}", e))))
        }
    }
}

pub async fn create_test(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTestRequest>,
) -> Result<Json<ApiResponse<Test>>, StatusCode> {
    // Parse layers from JSON
    let layers = match serde_json::from_value(request.layers) {
        Ok(layers) => layers,
        Err(e) => return Ok(Json(ApiResponse::error(format!("Invalid layers format: {}", e)))),
    };

    let test = Test::new(
        request.name,
        request.description,
        layers,
    );

    match state.storage.save_test(&test).await {
        Ok(_) => Ok(Json(ApiResponse::success(test))),
        Err(e) => {
            tracing::error!("Failed to create test: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to create test: {}", e))))
        }
    }
}

pub async fn update_test(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<CreateTestRequest>,
) -> Result<Json<ApiResponse<Test>>, StatusCode> {
    // Get existing test to preserve timestamps
    let mut test = match state.storage.get_test(&id).await {
        Ok(test) => test,
        Err(StorageError::NotFound) => return Ok(Json(ApiResponse::error("Test not found".to_string()))),
        Err(e) => {
            tracing::error!("Failed to get test: {}", e);
            return Ok(Json(ApiResponse::error(format!("Failed to get test: {}", e))));
        }
    };

    let layers = match serde_json::from_value(request.layers) {
        Ok(layers) => layers,
        Err(e) => return Ok(Json(ApiResponse::error(format!("Invalid layers format: {}", e)))),
    };

    // Update fields
    test.name = request.name;
    test.layers = layers;
    test.description = request.description;
    test.updated_at = chrono::Utc::now();

    match state.storage.update_test(&id, &test).await {
        Ok(_) => Ok(Json(ApiResponse::success(test))),
        Err(e) => {
            tracing::error!("Failed to update test: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to update test: {}", e))))
        }
    }
}

pub async fn delete_test(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.storage.delete_test(&id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(StorageError::NotFound) => Ok(Json(ApiResponse::error("Test not found".to_string()))),
        Err(e) => {
            tracing::error!("Failed to delete test: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to delete test: {}", e))))
        }
    }
}

// Diagnostic handlers
pub async fn run_diagnostics(
    State(state): State<Arc<AppState>>,
    Path(test_id): Path<String>,
) -> Result<Json<ApiResponse<DiagnosticResults>>, StatusCode> {
    // Get the test
    let test = match state.storage.get_test(&test_id).await {
        Ok(test) => test,
        Err(StorageError::NotFound) => return Ok(Json(ApiResponse::error("Test not found".to_string()))),
        Err(e) => {
            tracing::error!("Failed to get test: {}", e);
            return Ok(Json(ApiResponse::error(format!("Failed to get test: {}", e))));
        }
    };

    // Execute the test layers
    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let mut layer_results = Vec::new();

    for layer in &test.layers {
        let layer_start = std::time::Instant::now();
        let mut check_results = Vec::new();
        let mut layer_success = true;

        for check in &layer.checks {
            let result = network_checks::execute_check(&check.r#type, &check.config).await;
            if !result.success {
                layer_success = false;
            }
            check_results.push(result);
        }

        let layer_duration = layer_start.elapsed().as_millis() as u64;
        let layer_end_time = start_time + layer_duration;

        layer_results.push(crate::types::LayerResult {
            name: layer.name.clone(),
            description: layer.description.clone(),
            checks: check_results,
            success: layer_success,
            start_time,
            end_time: layer_end_time,
            duration: layer_duration,
        });
    }

    let diagnostic_result = DiagnosticResults::new(
        test.id.clone(),
        test.name.clone(),
        layer_results,
    );

    // Save the result
    if let Err(e) = state.storage.save_diagnostic_result(&diagnostic_result).await {
        tracing::error!("Failed to save diagnostic result: {}", e);
        // Continue anyway, return the result even if we couldn't save it
    }

    Ok(Json(ApiResponse::success(diagnostic_result)))
}

// Individual check execution
pub async fn execute_check(
    Path(check_type): Path<String>,
    Json(request): Json<ExecuteCheckRequest>,
) -> Result<Json<ApiResponse<crate::types::CheckResult>>, StatusCode> {
    let result = network_checks::execute_check(&check_type, &request.config).await;
    Ok(Json(ApiResponse::success(result)))
}

// Get diagnostic results
pub async fn get_diagnostic_results(
    State(state): State<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<ApiResponse<Vec<DiagnosticResults>>>, StatusCode> {
    match state.storage.get_diagnostic_results(
        params.test_id.as_deref(),
        params.limit,
    ).await {
        Ok(results) => Ok(Json(ApiResponse::success(results))),
        Err(e) => {
            tracing::error!("Failed to get diagnostic results: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to get diagnostic results: {}", e))))
        }
    }
}

// Configuration handlers
pub async fn get_config(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<crate::config::ServerConfig>> {
    Json(ApiResponse::success(state.config.clone()))
}

// Create the router
pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/health", get(health))
        
        // Node routes
        .route("/api/nodes", get(get_nodes).post(create_node))
        .route("/api/nodes/:id", put(update_node).delete(delete_node))
        
        // Test routes  
        .route("/api/tests", get(get_tests).post(create_test))
        .route("/api/tests/:id", put(update_test).delete(delete_test))
        
        // Diagnostic routes
        .route("/api/diagnostics/run/:test_id", post(run_diagnostics))
        .route("/api/diagnostics/results", get(get_diagnostic_results))
        
        // Check execution
        .route("/api/checks/:check_type", post(execute_check))
        
        // Configuration
        .route("/api/config", get(get_config))
}