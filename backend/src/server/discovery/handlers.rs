use crate::server::{
    config::AppState,
    daemons::types::api::{DaemonDiscoveryRequest, DiscoveryType, DiscoveryUpdatePayload},
    discovery::types::api::InitiateDiscoveryRequest,
    shared::types::api::{ApiError, ApiResponse, ApiResult},
};
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use uuid::Uuid;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/initiate", post(initiate_discovery))
        .route("/daemon-initiate", post(daemon_initiate_discovery))
        .route("/:session_id/status", get(get_discovery_status))
        .route("/:session_id/cancel", post(cancel_discovery))
        .route("/active", get(get_active_sessions))
        .route("/update", post(receive_discovery_update))
}

/// Receive discovery progress update from daemon
async fn receive_discovery_update(
    State(state): State<Arc<AppState>>,
    Json(update): Json<DiscoveryUpdatePayload>,
) -> ApiResult<Json<ApiResponse<()>>> {
    state.discovery_manager.update_session(update).await?;

    Ok(Json(ApiResponse::success(())))
}


/// Endpoint for daemon to initiate discovery
async fn daemon_initiate_discovery(
    State(state): State<Arc<AppState>>,
    Json(request): Json<InitiateDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<Uuid>>> {
    tracing::info!("daemon_initiate_discovery handler called");
    
    let daemon_service = &state.services.daemon_service;

    // Get the specified daemon
    let daemon = match daemon_service.get_daemon(&request.daemon_id).await? {
        Some(daemon) => daemon,
        None => {
            return Err(ApiError::not_found(&format!(
                "Daemon '{}' not found",
                &request.daemon_id
            )))
        }
    };

    // Check if daemon is already running discovery
    if let Some(existing_session_id) = state
        .discovery_manager
        .is_daemon_discovering(&daemon.id)
        .await
    {
        return Err(ApiError::conflict(&format!(
            "Daemon '{}' is already running discovery session '{}'",
            daemon.id, existing_session_id
        )));
    }

    let session_id = Uuid::new_v4();

    // Create discovery session
    state
        .discovery_manager
        .create_session(session_id, daemon.id)
        .await
        .map_err(|e| {
            ApiError::internal_error(&format!("Failed to create discovery session: {}", e))
        })?;

    Ok(Json(ApiResponse::success(session_id)))

}

/// Endpoint for users to initiate discovery on a specific daemon
async fn initiate_discovery(
    State(state): State<Arc<AppState>>,
    Json(request): Json<InitiateDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<DiscoveryUpdatePayload>>> {
    let daemon_service = &state.services.daemon_service;

    // Get the specified daemon
    let daemon = match daemon_service.get_daemon(&request.daemon_id).await? {
        Some(daemon) => daemon,
        None => {
            return Err(ApiError::not_found(&format!(
                "Daemon '{}' not found",
                &request.daemon_id
            )))
        }
    };

    // Check if daemon is already running discovery
    if let Some(existing_session_id) = state
        .discovery_manager
        .is_daemon_discovering(&daemon.id)
        .await
    {
        return Err(ApiError::conflict(&format!(
            "Daemon '{}' is already running discovery session '{}'",
            daemon.id, existing_session_id
        )));
    }

    let session_id = Uuid::new_v4();

    // Send discovery request to daemon
    daemon_service
        .send_discovery_request(
            &daemon,
            DaemonDiscoveryRequest {
                discovery_type: DiscoveryType::Network,
                session_id,
            },
        )
        .await?;

    // Create discovery session
    let update = state
        .discovery_manager
        .create_session(session_id, daemon.id)
        .await
        .map_err(|e| {
            ApiError::internal_error(&format!("Failed to create discovery session: {}", e))
        })?;

    Ok(Json(ApiResponse::success(update)))
}

// Get all active discovery sessions
async fn get_active_sessions(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<DiscoveryUpdatePayload>>>> {
    let sessions = state.discovery_manager.get_active_sessions().await;
    Ok(Json(ApiResponse::success(sessions)))
}

/// Get discovery status for polling
async fn get_discovery_status(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<DiscoveryUpdatePayload>>> {
    let status = state
        .discovery_manager
        .get_session(&session_id)
        .await
        .ok_or_else(|| {
            ApiError::not_found(&format!("Discovery session '{}' not found", session_id))
        })?;

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
        None => {
            return Err(ApiError::not_found(&format!(
                "Session '{}' not found",
                session_id
            )))
        }
    };

    let daemon_service = &state.services.daemon_service;

    if let Some(daemon) = daemon_service.get_daemon(&daemon_id).await? {
        if let Err(e) = daemon_service
            .send_discovery_cancellation(&daemon, session_id)
            .await
        {
            tracing::warn!(
                "Failed to send discovery cancellation to daemon {} for session {}: {}",
                daemon_id,
                session_id,
                e
            );
            // Don't return error - local cancellation succeeded
        } else {
            tracing::info!(
                "Cancellation request sent to daemon {} for session {}",
                daemon_id,
                session_id
            );
        }
    } else {
        tracing::warn!(
            "Daemon {} not found when trying to cancel discovery session {}",
            daemon_id,
            session_id
        );
    }

    tracing::info!("Discovery session was {} cancelled", session_id);
    Ok(Json(ApiResponse::success(())))
}
