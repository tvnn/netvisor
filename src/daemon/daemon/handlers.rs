use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::{
    daemon::daemon::service::DaemonClientService,
    server::{
        daemons::types::api::{
            DaemonDiscoveryRequest, DaemonDiscoveryResponse,
            DaemonTestRequest, DaemonTestResponse,
        },
        shared::types::api::{ApiResponse, ApiResult},
    },
};

pub fn create_daemon_router() -> Router<Arc<DaemonClientService>> {
    Router::new()
        .route("/health", get(health_check))
        .route("/discover", post(handle_discovery_request))
        .route("/execute_test", post(handle_test_execution))
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("Daemon is healthy"))
}

/// Handle discovery request from server (session-based async)
async fn handle_discovery_request(
    State(service): State<Arc<DaemonClientService>>,
    Json(request): Json<DaemonDiscoveryRequest>,
) -> ApiResult<Json<ApiResponse<DaemonDiscoveryResponse>>> {
    let session_id = request.session_id.clone();
    tracing::info!("Received discovery request for session {}", session_id);
    
    // Execute discovery in background task
    let service_clone = service.clone();
    tokio::spawn(async move {
        if let Err(e) = run_discovery_session(service_clone, request).await {
            tracing::error!("Discovery session failed: {}", e);
        }
    });

    // Return immediate acknowledgment
    let response = DaemonDiscoveryResponse {
        success: true,
        session_id: session_id,
        message: "Discovery started".to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Handle test execution request from server (session-based async)
async fn handle_test_execution(
    State(service): State<Arc<DaemonClientService>>,
    Json(request): Json<DaemonTestRequest>,
) -> ApiResult<Json<ApiResponse<DaemonTestResponse>>> {
    let session_id = request.session_id.clone();
    tracing::info!("Received test execution request for session {}", session_id);
    
    // Execute test in background task
    let service_clone = service.clone();
    tokio::spawn(async move {
        if let Err(e) = run_test_execution(service_clone, request).await {
            tracing::error!("Test execution failed: {}", e);
        }
    });

    // Return immediate acknowledgment
    let response = DaemonTestResponse {
        success: true,
        session_id: session_id,
        message: "Test execution started".to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Background task for discovery session with progress reporting
async fn run_discovery_session(
    service: Arc<DaemonClientService>, 
    request: DaemonDiscoveryRequest
) -> anyhow::Result<()> {
    tracing::info!("Starting discovery session {} on subnets: {:?}", 
                  request.session_id, request.target_subnets);
    
    // Get server target from config for reporting back
    // let server_target = service.config_store.get_server_endpoint().await?
    //     .ok_or_else(|| anyhow::anyhow!("No server endpoint configured"))?;
    
    // TODO: Implement actual discovery logic
    // This would:
    // 1. Set up discovery configuration from request parameters
    // 2. Scan target subnets for devices
    // 3. For each discovered device, call service.report_discovered_node()
    // 4. Periodically call service.report_discovery_progress() with updates
    
    // Placeholder: simulate discovery work with progress reporting
    // let total_ips = request.target_subnets.iter()
    //     .map(|subnet| subnet.iter().count())
    //     .sum::<usize>();
    
    // for (completed, _subnet) in request.target_subnets.iter().enumerate() {
    //     // Report progress
    //     let progress = crate::server::daemons::types::api::DaemonDiscoveryProgress {
    //         session_id: request.session_id,
    //         phase: "Scanning subnet".to_string(),
    //         completed,
    //         total: request.target_subnets.len(),
    //         discovered_count: completed, // Placeholder
    //     };
        
    //     if let Err(e) = service.report_discovery_progress(&server_target, progress).await {
    //         tracing::warn!("Failed to report discovery progress: {}", e);
    //     }
        
    //     // Simulate work
    //     tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    // }
    
    tracing::info!("Discovery session {} completed", request.session_id);
    Ok(())
}

/// Background task for test execution session
async fn run_test_execution(
    service: Arc<DaemonClientService>, 
    request: DaemonTestRequest
) -> anyhow::Result<()> {
    tracing::info!("Executing tests for session {} on node {}", 
                  request.session_id, request.node.base.name);
    
    // Get server target from config for reporting back
    // let server_target = service.config_store.get_server_endpoint().await?
    //     .ok_or_else(|| anyhow::anyhow!("No server endpoint configured"))?;
    
    // TODO: Implement actual test execution
    // This would:
    // 1. Extract test configurations from request.node.assigned_tests
    // 2. Execute each test using existing test execution framework
    // 3. For each completed test, call service.report_test_result()
    
    // Placeholder: simulate test execution
    // for (i, assigned_test) in request.node.base.assigned_tests.iter().enumerate() {
    //     tracing::info!("Executing test {}/{}: {:?}", 
    //                   i + 1, 
    //                   request.node.base.assigned_tests.len(), 
    //                   assigned_test.test);
        
    //     // Simulate test execution time
    //     tokio::time::sleep(std::time::Duration::from_millis(800)).await;
        
    //     // Create placeholder result
    //     let test_result = crate::server::tests::types::execution::TestResult {
    //         success: true,
    //         message: Some(format!("Test completed successfully")),
    //         details: None,
    //         criticality: Some(assigned_test.criticality.clone()),
    //         duration_ms: 800,
    //         timestamp: chrono::Utc::now(),
    //     };
        
    //     // Report result back to server
    //     if let Err(e) = service.report_test_result(&server_target, request.session_id, test_result).await {
    //         tracing::error!("Failed to report test result: {}", e);
    //     }
    // }
    
    tracing::info!("Test execution session {} completed", request.session_id);
    Ok(())
}