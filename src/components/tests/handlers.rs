use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::{
    api::{ApiResult, ApiResponse, ApiError},
    core::{TestResult, Node},
    components::{
        tests::execution::{execute_test, execute_node_tests, execute_adhoc_test},
        nodes::service::NodeService,
    },
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/execute-adhoc", post(execute_adhoc_test_handler))
        .route("/execute-node/:node_id", post(execute_node_tests_handler))
        .route("/results/:node_id", get(get_node_test_results))
}

#[derive(serde::Deserialize)]
pub struct ExecuteAdhocTestRequest {
    pub node_id: String,
    pub test_type: crate::core::TestType,
    pub test_config: crate::core::TestConfiguration,
}

#[derive(serde::Serialize)]
pub struct TestExecutionResponse {
    pub result: TestResult,
}

#[derive(serde::Serialize)]
pub struct NodeTestExecutionResponse {
    pub node_id: String,
    pub results: Vec<TestResult>,
    pub executed_at: String,
}

/// Execute a single ad-hoc test
async fn execute_adhoc_test_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ExecuteAdhocTestRequest>,
) -> ApiResult<Json<ApiResponse<TestExecutionResponse>>> {
    let node_service = NodeService::new(state.node_storage.clone());
    
    // Get the target node
    let node = node_service.get_node(&request.node_id).await?
        .ok_or_else(|| ApiError::node_not_found(&request.node_id))?;
    
    // Execute the test
    let result = execute_adhoc_test(
        request.test_type,
        request.test_config,
        &node,
    ).await.map_err(|e| ApiError::test_execution_error(&e.to_string()))?;
    
    Ok(Json(ApiResponse::success(TestExecutionResponse { result })))
}

/// Execute all assigned tests on a node
async fn execute_node_tests_handler(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<String>,
) -> ApiResult<Json<ApiResponse<NodeTestExecutionResponse>>> {
    let node_service = NodeService::new(state.node_storage.clone());
    
    // Get the target node
    let node = node_service.get_node(&node_id).await?
        .ok_or_else(|| ApiError::node_not_found(&node_id))?;
    
    // Execute all assigned tests
    let results = execute_node_tests(&node).await
        .map_err(|e| ApiError::test_execution_error(&e.to_string()))?;
    
    // Update node status based on results
    let mut updated_node = node.clone();
    updated_node.compute_status_from_tests(&results);
    updated_node.last_seen = Some(chrono::Utc::now());
    node_service.update_node(updated_node).await?;
    
    Ok(Json(ApiResponse::success(NodeTestExecutionResponse {
        node_id: node_id.clone(),
        results,
        executed_at: chrono::Utc::now().to_rfc3339(),
    })))
}

/// Get recent test results for a node (placeholder - you might want to store results)
async fn get_node_test_results(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<String>,
) -> ApiResult<Json<ApiResponse<Vec<TestResult>>>> {
    let node_service = NodeService::new(state.node_storage.clone());
    
    // Verify node exists
    let _node = node_service.get_node(&node_id).await?
        .ok_or_else(|| ApiError::node_not_found(&node_id))?;
    
    // For now, return empty results - you might want to implement result storage later
    Ok(Json(ApiResponse::success(Vec::new())))
}