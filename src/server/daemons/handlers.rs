use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::{server::{
    config::AppState, 
    daemons::{
        types::{
            api::{
                DaemonDiscoveryUpdate, DaemonRegistrationRequest, DaemonRegistrationResponse, DaemonResponse
            }, 
            base::{Daemon, DaemonBase}
        }
    }, 
    shared::types::api::{ApiError, ApiResponse, ApiResult}
}};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_daemon))
        .route("/:id/heartbeat", put(receive_heartbeat))
        .route("/", get(get_all_daemons))
        .route("/:id", get(get_daemon))
        // Routes for receiving reports from daemons
        .route("/discovery_update", post(receive_discovery_update))
}

/// Register a new daemon
async fn register_daemon(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DaemonRegistrationRequest>,
) -> ApiResult<Json<ApiResponse<DaemonRegistrationResponse>>> {

    let service = &state.services.daemon_service;

    let daemon = Daemon::new(request.daemon_id, DaemonBase {
        node_id: request.node.id
    });
            
    let registered_daemon = service.register_daemon(daemon).await
        .map_err(|e| {
            ApiError::internal_error(&format!("Failed to register daemon: {}", e))
        })?;
    
    Ok(Json(ApiResponse::success(DaemonRegistrationResponse {
        daemon: registered_daemon,
    })))
}

/// Receive heartbeat from daemon
async fn receive_heartbeat(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = &state.services.daemon_service;

    let daemon = service.get_daemon(&id).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to get daemon: {}", e)))?
        .ok_or_else(|| ApiError::not_found(&format!("Daemon '{}' not found", &id)))?;

    service.receive_heartbeat(daemon).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to update heartbeat: {}", e)))?;
    
    Ok(Json(ApiResponse::success(())))
}

/// Get all registered daemons
async fn get_all_daemons(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<Daemon>>>> {
    let service = &state.services.daemon_service;
    
    let daemons = service.get_all_daemons().await
        .map_err(|e| ApiError::internal_error(&format!("Failed to get daemons: {}", e)))?;
    
    Ok(Json(ApiResponse::success( daemons )))
}

/// Get specific daemon by ID
async fn get_daemon(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<DaemonResponse>>> {
    let service = &state.services.daemon_service;
    
    let daemon = service.get_daemon(&id).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to get daemon: {}", e)))?
        .ok_or_else(|| ApiError::not_found(&format!("Daemon '{}' not found", &id)))?;
    
    Ok(Json(ApiResponse::success(DaemonResponse{ daemon })))
}

/// Receive discovery progress update from daemon
async fn receive_discovery_update(
    State(state): State<Arc<AppState>>,
    Json(update): Json<DaemonDiscoveryUpdate>,
) -> ApiResult<Json<ApiResponse<()>>> {
    
    state.discovery_manager.update_session(update).await?;
    
    Ok(Json(ApiResponse::success(())))
}