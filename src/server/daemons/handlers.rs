use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put},
    Router,
};
use uuid::Uuid;
use std::sync::Arc;
use crate::server::{
    config::AppState, 
    daemons::{
        service::DaemonService, 
        types::{
            api::{
                DaemonRegistrationRequest, DaemonRegistrationResponse,
                DaemonDiscoveryProgress, DaemonNodeReport, DaemonTestResult
            }, 
            base::{Daemon, DaemonBase, DaemonStatus}
        }
    }, 
    shared::types::api::{ApiError, ApiResponse, ApiResult}
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_daemon))
        .route("/:id/heartbeat", put(receive_heartbeat))
        .route("/", get(get_all_daemons))
        .route("/:id", get(get_daemon))
        .route("/:id/health", get(check_daemon_health))
        // Routes for receiving reports from daemons
        .route("/discovery_progress", post(receive_discovery_progress))
        .route("/discovered_node", post(receive_discovered_node))
        .route("/test_result", post(receive_test_result))
}

/// Register a new daemon
async fn register_daemon(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DaemonRegistrationRequest>,
) -> ApiResult<Json<ApiResponse<DaemonRegistrationResponse>>> {
    let service = DaemonService::new(state.daemon_storage.clone());
    
    let daemon = Daemon::new(DaemonBase {
        ip: request.ip, 
        port: request.port, 
        name: request.name,
        hostname: request.hostname, 
        status: DaemonStatus::Active
    });
            
    let registered_daemon = service.register_daemon(daemon).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to register daemon: {}", e)))?;
    
    Ok(Json(ApiResponse::success(DaemonRegistrationResponse {
        daemon: registered_daemon,
    })))
}

/// Receive heartbeat from daemon
async fn receive_heartbeat(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = DaemonService::new(state.daemon_storage.clone());

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
    let service = DaemonService::new(state.daemon_storage.clone());
    
    let daemons = service.get_all_daemons().await
        .map_err(|e| ApiError::internal_error(&format!("Failed to get daemons: {}", e)))?;
    
    Ok(Json(ApiResponse::success(daemons)))
}

/// Get specific daemon by ID
async fn get_daemon(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Daemon>>> {
    let service = DaemonService::new(state.daemon_storage.clone());
    
    let daemon = service.get_daemon(&id).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to get daemon: {}", e)))?
        .ok_or_else(|| ApiError::not_found(&format!("Daemon '{}' not found", &id)))?;
    
    Ok(Json(ApiResponse::success(daemon)))
}

/// Check daemon health status
async fn check_daemon_health(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<bool>>> {
    let service = DaemonService::new(state.daemon_storage.clone());
    
    let daemon = service.get_daemon(&id).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to get daemon: {}", e)))?
        .ok_or_else(|| ApiError::not_found(&format!("Daemon '{}' not found", &id)))?;
    
    let is_healthy = service.check_daemon_health(&daemon).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to check daemon health: {}", e)))?;
    
    Ok(Json(ApiResponse::success(is_healthy)))
}

/// Receive discovery progress update from daemon
async fn receive_discovery_progress(
    State(state): State<Arc<AppState>>,
    Json(progress): Json<DaemonDiscoveryProgress>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = DaemonService::new(state.daemon_storage.clone());
    
    service.process_discovery_progress(progress).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to process discovery progress: {}", e)))?;
    
    Ok(Json(ApiResponse::success(())))
}

/// Receive discovered node report from daemon
async fn receive_discovered_node(
    State(state): State<Arc<AppState>>,
    Json(node_report): Json<DaemonNodeReport>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = DaemonService::new(state.daemon_storage.clone());
    
    service.process_discovered_node(node_report).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to process discovered node: {}", e)))?;
    
    Ok(Json(ApiResponse::success(())))
}

/// Receive test result from daemon
async fn receive_test_result(
    State(state): State<Arc<AppState>>,
    Json(test_result): Json<DaemonTestResult>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = DaemonService::new(state.daemon_storage.clone());
    
    service.process_test_result(test_result).await
        .map_err(|e| ApiError::internal_error(&format!("Failed to process test result: {}", e)))?;
    
    Ok(Json(ApiResponse::success(())))
}