use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json},
};
use std::sync::Arc;
use crate::AppState;
use crate::shared::handlers::ApiResponse;
use crate::shared::storage::{StorageError};

use crate::components::tests::types::{Test, CreateTestRequest, ExecuteCheckRequest};
use crate::components::tests::checks;
use crate::components::diagnostics::types::CheckResult;

// Test handlers
pub async fn get_tests(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Test>>>, StatusCode> {
    match state.test_storage.get_tests().await {
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

    match state.test_storage.save_test(&test).await {
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
    let mut test = match state.test_storage.get_test(&id).await {
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

    match state.test_storage.update_test(&id, &test).await {
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
    match state.test_storage.delete_test(&id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(StorageError::NotFound) => Ok(Json(ApiResponse::error("Test not found".to_string()))),
        Err(e) => {
            tracing::error!("Failed to delete test: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to delete test: {}", e))))
        }
    }
}

// Individual check execution
pub async fn execute_check(
    Path(check_type): Path<String>,
    Json(request): Json<ExecuteCheckRequest>,
) -> Result<Json<ApiResponse<CheckResult>>, StatusCode> {
    let result = checks::execute_check(&check_type, &request.config).await;
    Ok(Json(ApiResponse::success(result)))
}
