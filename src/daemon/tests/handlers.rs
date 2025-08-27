use axum::{
    extract::State,
    response::Json,
    routing::{post},
    Router,
};
use std::sync::Arc;
use crate::{daemon::{shared::storage::ConfigStore, tests::service::DaemonTestService}, server::shared::types::api::ApiError};
use crate::{
    server::{
        daemons::types::api::{
            DaemonTestRequest, DaemonTestResponse,
        },
        shared::types::api::{ApiResponse, ApiResult},
    },
};

pub fn create_router() -> Router<Arc<ConfigStore>> {
    Router::new()
        .route("/execute", post(handle_test_execution))
}

/// Handle test execution request from server (session-based async)
async fn handle_test_execution(
    State(config): State<Arc<ConfigStore>>,
    Json(request): Json<DaemonTestRequest>,
) -> ApiResult<Json<ApiResponse<DaemonTestResponse>>> {
    let session_id = request.session_id.clone();
    tracing::info!("Received test execution request for session {}", session_id);
    
    let test_service = DaemonTestService::new(config.clone());

    tokio::spawn(async move {
        if let Err(e) = test_service.run_test_execution(request).await {
            return Err(Json(ApiError::internal_error(&format!("Test execution failed: {}", e))));
        } else {
            Ok(())
        }
    });

    // Return immediate acknowledgment
    Ok(Json(ApiResponse::success(DaemonTestResponse{session_id})))
}