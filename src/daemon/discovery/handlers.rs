use std::sync::Arc;
use axum::{
    extract::State, response::Json, routing::post, Router
};
use crate::daemon::runtime::types::base::DaemonState;
use crate::server::daemons::types::api::{DaemonDiscoveryCancellationRequest, DaemonDiscoveryCancellationResponse};
use crate::{
    server::{
        daemons::types::api::{
            DaemonDiscoveryRequest, DaemonDiscoveryResponse,
        },
        shared::types::api::{ApiResponse, ApiResult, ApiError},
    },
};

pub fn create_router() -> Router<Arc<DaemonState>> {
    Router::new()
        .route("/initiate", post(handle_discovery_request))
        .route("/cancel", post(handle_cancel_request))
}

async fn handle_discovery_request(
    State(state): State<Arc<DaemonState>>,
    Json(request): Json<DaemonDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryResponse>>> {
    let session_id = request.session_id.clone();
    tracing::info!("Received discovery request for session {}", session_id);

    let discovery_service = state.discovery_service.clone();
    let manager = discovery_service.discovery_manager.clone();

    if manager.is_discovery_running().await {
        return Err(ApiError::conflict(&"Discovery session already running"));
    } else {
        
        let cancel_token = manager.start_new_session().await;

        let inner_manager = manager.clone();
        let handle = tokio::spawn(async move {
            match discovery_service.run_discovery_session(request, cancel_token).await {
                Ok(()) => {
                    tracing::info!("Discovery completed successfully");
                },
                Err(e) => {
                    tracing::error!("Discovery failed: {}", e);
                }
            }
            inner_manager.clear_completed_task().await;
        });
        manager.set_current_task(handle).await;

        // Return immediate acknowledgment
        Ok(Json(ApiResponse::success(DaemonDiscoveryResponse{session_id})))
    }
}

async fn handle_cancel_request(
    State(state): State<Arc<DaemonState>>,
    Json(request): Json<DaemonDiscoveryCancellationRequest>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryCancellationResponse>>> {
    let session_id = request.session_id.clone();
    tracing::info!("Received discovery cancellation request for session {}", session_id);

    let discovery_service = &state.discovery_service;
    let manager = discovery_service.discovery_manager.clone();

    if manager.is_discovery_running().await {
        if manager.cancel_current_discovery().await {
            manager.clear_completed_task().await;
            Ok(Json(ApiResponse::success(DaemonDiscoveryCancellationResponse{session_id})))
        } else {
            Err(ApiError::internal_error("Failed to cancel discovery session"))
        }
    } else {
        Err(ApiError::conflict("Discovery session not currently running"))
    }
}