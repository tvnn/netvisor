use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    server::shared::types::api::{ApiResponse, ApiResult},
    server::diagnostics::{
        types::*,
    },
    server::config::AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/execute/:group_id", post(execute_group_diagnostic))
        .route("/:execution_id", delete(delete_diagnostic_execution))
        .route("/", get(get_diagnostic_executions))
}

/// Execute diagnostics on a node group
async fn execute_group_diagnostic(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<Uuid>,
    Json(request): Json<ExecuteDiagnosticRequest>,
) -> ApiResult<Json<ApiResponse<DiagnosticExecution>>> {
    let service = &state.services.diagnostic_service;

    let execution = service.execute_group_diagnostic(&group_id, request.diagnostic.trigger_reason).await?;

    Ok(Json(ApiResponse::success(execution)))
}

/// Delete a diagnostic execution
async fn delete_diagnostic_execution(
    State(state): State<Arc<AppState>>,
    Path(execution_id): Path<String>,
) -> ApiResult<Json<ApiResponse<String>>> {
    let service = &state.services.diagnostic_service;

    service.delete_execution(&execution_id).await?;

    Ok(Json(ApiResponse::success(format!(
        "Diagnostic execution {} deleted successfully",
        execution_id
    ))))
}

/// Get diagnostic executions with optional filters
async fn get_diagnostic_executions(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<DiagnosticExecution>>>> {
    let service = &state.services.diagnostic_service;

    let executions= service.get_all_executions().await?;

    Ok(Json(ApiResponse::success(executions)))
}