use std::sync::Arc;

use axum::{
    extract::State, response::Json, routing::post, Router
};
use crate::daemon::{discovery::service::DaemonDiscoveryService, shared::storage::ConfigStore};
use crate::{
    server::{
        daemons::types::api::{
            DaemonDiscoveryRequest, DaemonDiscoveryResponse,
        },
        shared::types::api::{ApiResponse, ApiResult, ApiError},
    },
};

pub fn create_router() -> Router<Arc<ConfigStore>> {
    Router::new()
        .route("/initiate", post(handle_discovery_request))
        // .route("/cancel", post(handle_discovery_request))
}

async fn handle_discovery_request(
    State(config): State<Arc<ConfigStore>>,
    Json(request): Json<DaemonDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryResponse>>> {
    let session_id = request.session_id.clone();
    tracing::info!("Received discovery request for session {}", session_id);

    let discovery_service = DaemonDiscoveryService::new(config.clone());
    
    tokio::spawn(async move {
        if let Err(e) = discovery_service.run_discovery_session(request).await {
            return Err(Json(ApiError::internal_error(&format!("Discovery session failed: {}", e))));
        } else {
            Ok(())
        }
    });

    // Return immediate acknowledgment
    Ok(Json(ApiResponse::success(DaemonDiscoveryResponse{session_id})))
}