use crate::daemon::discovery::service::base::Discovery;
use crate::daemon::discovery::service::network::NetworkScanDiscovery;
use crate::daemon::runtime::types::DaemonAppState;
use crate::server::daemons::types::api::{
    DaemonDiscoveryCancellationRequest, DaemonDiscoveryCancellationResponse,
};
use crate::server::{
    daemons::types::api::{DaemonDiscoveryRequest, DaemonDiscoveryResponse},
    shared::types::api::{ApiError, ApiResponse, ApiResult},
};
use axum::{Router, extract::State, response::Json, routing::post};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<DaemonAppState>> {
    Router::new()
        .route("/initiate", post(handle_discovery_request))
        .route("/cancel", post(handle_cancel_request))
}

async fn handle_discovery_request(
    State(state): State<Arc<DaemonAppState>>,
    Json(request): Json<DaemonDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryResponse>>> {
    let session_id = request.session_id;
    tracing::info!("Received discovery request for session {}", session_id);

    let discovery = Arc::new(Discovery::new(
        state.services.discovery_service.clone(),
        state.services.discovery_manager.clone(),
        NetworkScanDiscovery::default(),
    ));

    discovery.discover_on_network(request).await?;

    Ok(Json(ApiResponse::success(DaemonDiscoveryResponse {
        session_id,
    })))
}

async fn handle_cancel_request(
    State(state): State<Arc<DaemonAppState>>,
    Json(request): Json<DaemonDiscoveryCancellationRequest>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryCancellationResponse>>> {
    let session_id = request.session_id;
    tracing::info!(
        "Received discovery cancellation request for session {}",
        session_id
    );

    let manager = state.services.discovery_manager.clone();

    if manager.is_discovery_running().await {
        if manager.cancel_current_session().await {
            manager.clear_completed_task().await;
            Ok(Json(ApiResponse::success(
                DaemonDiscoveryCancellationResponse { session_id },
            )))
        } else {
            Err(ApiError::internal_error(
                "Failed to cancel discovery session",
            ))
        }
    } else {
        Err(ApiError::conflict(
            "Discovery session not currently running",
        ))
    }
}
