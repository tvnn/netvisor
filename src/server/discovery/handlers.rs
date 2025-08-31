use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::{server::daemons::types::api::DaemonDiscoveryUpdate};
use crate::{server::{
    config::AppState, daemons::{
        service::DaemonService, 
        types::api::DaemonDiscoveryRequest
    }, discovery::types::api::{InitiateDiscoveryRequest}, shared::types::api::{ApiError, ApiResponse, ApiResult}
}};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/initiate", post(initiate_discovery))
        .route("/:session_id/status", get(get_discovery_status))
        .route("/:session_id/cancel", post(cancel_discovery))
        .route("/active", get(get_active_sessions))
}

/// Initiate discovery on a specific daemon
async fn initiate_discovery(
    State(state): State<Arc<AppState>>,
    Json(request): Json<InitiateDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryUpdate>>> {
    let daemon_service = DaemonService::new(state.daemon_storage.clone(), state.node_storage.clone());
    
    // Get the specified daemon
    let daemon = match daemon_service.get_daemon(&request.daemon_id).await? {
        Some(daemon) => daemon,
        None => return Err(ApiError::not_found(&format!("Daemon '{}' not found", &request.daemon_id)))
    };

    // Check if daemon is already running discovery
    if let Some(existing_session_id) = state.discovery_manager.is_daemon_discovering(&daemon.id).await {
        return Err(ApiError::conflict(&format!(
            "Daemon '{}' is already running discovery session '{}'", 
            daemon.id, 
            existing_session_id
        )));
    }

    let session_id = Uuid::new_v4();

    // Send discovery request to daemon
    daemon_service.send_discovery_request(&daemon, DaemonDiscoveryRequest { session_id }).await?;    

    // Create discovery session
    let update  = state.discovery_manager.create_session(session_id, daemon.id).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to create discovery session: {}", e)))?;
        
    Ok(Json(ApiResponse::success( update)))
}

// Get all active discovery sessions
async fn get_active_sessions(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<DaemonDiscoveryUpdate>>>> {
    let sessions = state.discovery_manager.get_active_sessions().await;
    Ok(Json(ApiResponse::success(sessions)))
}

/// Get discovery status for polling
async fn get_discovery_status(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryUpdate>>> {
    let status = state.discovery_manager.get_session(&session_id).await
        .ok_or_else(|| ApiError::not_found(&format!("Discovery session '{}' not found", session_id)))?;

    Ok(Json(ApiResponse::success(status)))
}

/// Cancel an active discovery session
async fn cancel_discovery(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    // Cancel the session and get daemon ID
    let daemon_id = match state.discovery_manager.get_session(&session_id).await {
        Some(session) => session.daemon_id,
        None => return Err(ApiError::not_found(&format!("Session '{}' not found", session_id)))
    };
    
    let daemon_service = DaemonService::new(state.daemon_storage.clone(), state.node_storage.clone());
    
    if let Some(daemon) = daemon_service.get_daemon(&daemon_id).await? {
        if let Err(e) = daemon_service.send_discovery_cancellation(&daemon, session_id).await {
            tracing::warn!("Failed to send discovery cancellation to daemon {} for session {}: {}", daemon_id, session_id, e);
            // Don't return error - local cancellation succeeded
        } else {
            tracing::info!("Cancellation request sent to daemon {} for session {}", daemon_id, session_id);
        }
    } else {
        tracing::warn!("Daemon {} not found when trying to cancel discovery session {}", daemon_id, session_id);
    }

    tracing::info!("Discovery session {} cancelled", session_id);
    Ok(Json(ApiResponse::success(())))
}