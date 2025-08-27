use axum::{
    extract::{State},
    response::Json,
    routing::{get, post},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::server::{
    config::AppState, daemons::{
        service::DaemonService, 
        types::{api::{DaemonDiscoveryRequest, InitiateDiscoveryRequest, InitiateDiscoveryResponse}}
    }, shared::types::api::{ApiError, ApiResponse, ApiResult}
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/initiate", post(initiate_discovery))
        .route("/:session_id/stream", get(stream_discovery_results))
        .route("/:session_id/cancel", post(cancel_discovery))
}

async fn initiate_discovery(
    State(state): State<Arc<AppState>>,
    Json(request): Json<InitiateDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<InitiateDiscoveryResponse>>> {
    let daemon_service = DaemonService::new(state.daemon_storage.clone());
    
    let daemon = match daemon_service.get_daemon(&request.daemon_id).await? {
        Some(daemon) => daemon,
        None => return Err(ApiError::not_found(&format!("Daemon '{}' not found", &request.daemon_id)))
    };

    let session_id = Uuid::new_v4();

    daemon_service.send_discovery_request(&daemon, DaemonDiscoveryRequest{session_id}).await?;    
    
    Ok(Json(ApiResponse::success(InitiateDiscoveryResponse {
        session_id
    })))
}

async fn stream_discovery_results(
    State(state): State<Arc<AppState>>,
    Json(request): Json<InitiateDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<InitiateDiscoveryResponse>>> {
    let daemon_service = DaemonService::new(state.daemon_storage.clone());
    
    let daemon = match daemon_service.get_daemon(&request.daemon_id).await? {
        Some(daemon) => daemon,
        None => return Err(ApiError::not_found(&format!("Daemon '{}' not found", &request.daemon_id)))
    };

    let session_id = Uuid::new_v4();

    daemon_service.send_discovery_request(&daemon, DaemonDiscoveryRequest {session_id});
    
    Ok(Json(ApiResponse::success(InitiateDiscoveryResponse {
        session_id
    })))
}

async fn cancel_discovery(
    State(state): State<Arc<AppState>>,
    Json(request): Json<InitiateDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<InitiateDiscoveryResponse>>> {
    let daemon_service = DaemonService::new(state.daemon_storage.clone());
    
    let daemon = match daemon_service.get_daemon(&request.daemon_id).await? {
        Some(daemon) => daemon,
        None => return Err(ApiError::not_found(&format!("Daemon '{}' not found", &request.daemon_id)))
    };

    let session_id = Uuid::new_v4();

    daemon_service.send_discovery_request(&daemon, DaemonDiscoveryRequest {session_id});
    
    Ok(Json(ApiResponse::success(InitiateDiscoveryResponse {
        session_id
    })))
}