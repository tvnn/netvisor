use axum::{
    extract::{Path, State},
    response::Json,
    routing::{post, put},
    Router,
};
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    config::AppState, daemons::{service::DaemonService, types::{api::{DaemonRegistrationRequest, DaemonRegistrationResponse}, base::{Daemon, DaemonBase}}}, 
    shared::types::api::{ApiError, ApiResponse, ApiResult}
};
use super::types::base::DaemonStatus;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_daemon))
        .route("/:id/heartbeat", put(receive_heartbeat))
}

async fn register_daemon(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DaemonRegistrationRequest>,
) -> ApiResult<Json<ApiResponse<DaemonRegistrationResponse>>> {
    let service = DaemonService::new(state.daemon_storage.clone());
    
    let daemon = Daemon::new(DaemonBase {
        ip: request.ip, port: request.port, hostname: request.hostname, status: DaemonStatus::Active
    });
            
    let registered_daemon = service.register_daemon(daemon).await?;
    
    Ok(Json(ApiResponse::success(DaemonRegistrationResponse {
        daemon: registered_daemon,
    })))
}

async fn receive_heartbeat(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = DaemonService::new(state.daemon_storage.clone());

    let daemon = service.get_daemon(&id).await?
        .ok_or_else(|| ApiError::not_found(&format!("Daemon '{}' not found", &id)))?;

    service.receive_heartbeat(daemon).await?;
    Ok(Json(ApiResponse::success(())))
}
