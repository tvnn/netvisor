use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::{Json},
};
use std::sync::Arc;
use crate::AppState;
use crate::shared::handlers::ApiResponse;
use crate::shared::storage::StorageError;
use crate::shared::handlers::QueryParams;

use crate::components::diagnostics::types::{LayerResult, DiagnosticResults};
use crate::components::tests::checks;

// Diagnostic handlers
pub async fn run_diagnostics(
    State(state): State<Arc<AppState>>,
    Path(test_id): Path<String>,
) -> Result<Json<ApiResponse<DiagnosticResults>>, StatusCode> {
    // Get the test
    let test = match state.test_storage.get_test(&test_id).await {
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
            let result = checks::execute_check(&check.r#type, &check.config).await;
            if !result.success {
                layer_success = false;
            }
            check_results.push(result);
        }

        let layer_duration = layer_start.elapsed().as_millis() as u64;
        let layer_end_time = start_time + layer_duration;

        layer_results.push(LayerResult {
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
    if let Err(e) = state.diagnostic_storage.save_diagnostic_result(&diagnostic_result).await {
        tracing::error!("Failed to save diagnostic result: {}", e);
        // Continue anyway, return the result even if we couldn't save it
    }

    Ok(Json(ApiResponse::success(diagnostic_result)))
}

// Get diagnostic results
pub async fn get_diagnostic_results(
    State(state): State<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<ApiResponse<Vec<DiagnosticResults>>>, StatusCode> {
    match state.diagnostic_storage.get_diagnostic_results(
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