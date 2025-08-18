use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use crate::{
    api::{ApiError, ApiResponse, ApiResult},
    components::diagnostics::{
        service::DiagnosticService,
        types::*,
    },
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/execute-group/:group_id", post(execute_group_diagnostic))
        .route("/:execution_id", get(get_diagnostic_execution))
        .route("/:execution_id", delete(delete_diagnostic_execution))
        .route("/", get(get_diagnostic_executions))
        .route("/group/:group_id", get(get_group_diagnostic_executions))
        .route("/group/:group_id/latest", get(get_latest_group_diagnostic))
        .route("/group/:group_id/status", get(get_group_diagnostic_status))
        .route("/statistics", get(get_diagnostic_statistics))
}

/// Execute diagnostics on a node group
async fn execute_group_diagnostic(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<String>,
    Json(request): Json<ExecuteDiagnosticRequest>,
) -> ApiResult<Json<ApiResponse<DiagnosticExecutionResponse>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    let execution = service.execute_group_diagnostic(&group_id, request.diagnostic.trigger_reason).await?;
    let summary = execution.get_summary();

    Ok(Json(ApiResponse::success(DiagnosticExecutionResponse {
        execution,
        summary,
    })))
}

/// Get a specific diagnostic execution
async fn get_diagnostic_execution(
    State(state): State<Arc<AppState>>,
    Path(execution_id): Path<String>,
) -> ApiResult<Json<ApiResponse<DiagnosticExecutionResponse>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    match service.get_execution(&execution_id).await? {
        Some(execution) => {
            let summary = execution.get_summary();
            Ok(Json(ApiResponse::success(DiagnosticExecutionResponse {
                execution,
                summary,
            })))
        }
        None => Err(ApiError::not_found(&format!("Diagnostic execution not found: {}", execution_id))),
    }
}

/// Delete a diagnostic execution
async fn delete_diagnostic_execution(
    State(state): State<Arc<AppState>>,
    Path(execution_id): Path<String>,
) -> ApiResult<Json<ApiResponse<String>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    service.delete_execution(&execution_id).await?;

    Ok(Json(ApiResponse::success(format!(
        "Diagnostic execution {} deleted successfully",
        execution_id
    ))))
}

/// Get diagnostic executions with optional filters
async fn get_diagnostic_executions(
    State(state): State<Arc<AppState>>,
    Query(query): Query<DiagnosticListQuery>,
) -> ApiResult<Json<ApiResponse<DiagnosticListResponse>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    let executions = if query.group_id.is_some() || query.status.is_some() || query.limit.is_some() || query.offset.is_some() {
        service.get_executions_with_filters(query).await?
    } else {
        service.get_all_executions().await?
    };

    let total = executions.len();

    Ok(Json(ApiResponse::success(DiagnosticListResponse {
        executions,
        total,
    })))
}

/// Get diagnostic executions for a specific group
async fn get_group_diagnostic_executions(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<String>,
) -> ApiResult<Json<ApiResponse<DiagnosticListResponse>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    let executions = service.get_group_executions(&group_id).await?;
    let total = executions.len();

    Ok(Json(ApiResponse::success(DiagnosticListResponse {
        executions,
        total,
    })))
}

/// Get the latest diagnostic execution for a group
async fn get_latest_group_diagnostic(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<String>,
) -> ApiResult<Json<ApiResponse<DiagnosticExecutionResponse>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    let executions = service.get_group_executions(&group_id).await?;
    
    match executions.first() {
        Some(execution) => {
            let summary = execution.get_summary();
            Ok(Json(ApiResponse::success(DiagnosticExecutionResponse {
                execution: execution.clone(),
                summary,
            })))
        }
        None => Err(ApiError::not_found(&format!("No diagnostic executions found for group {}", group_id))),
    }
}

/// Get the current diagnostic status for a group
async fn get_group_diagnostic_status(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<String>,
) -> ApiResult<Json<ApiResponse<GroupDiagnosticStatusResponse>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    let latest_status = service.get_latest_group_status(&group_id).await?;
    let executions = service.get_group_executions(&group_id).await?;
    let total_executions = executions.len();

    let latest_execution_id = executions.first().map(|e| e.id.clone());
    let last_execution_time = executions.first().map(|e| e.started_at);

    Ok(Json(ApiResponse::success(GroupDiagnosticStatusResponse {
        group_id,
        latest_status,
        latest_execution_id,
        last_execution_time,
        total_executions,
    })))
}

/// Get diagnostic statistics
async fn get_diagnostic_statistics(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<DiagnosticStatisticsResponse>>> {
    let service = DiagnosticService::new(
        state.diagnostic_storage.clone(),
        state.node_storage.clone(),
        state.node_group_storage.clone(),
    );

    let statistics = service.get_statistics().await?;

    Ok(Json(ApiResponse::success(DiagnosticStatisticsResponse {
        statistics,
    })))
}