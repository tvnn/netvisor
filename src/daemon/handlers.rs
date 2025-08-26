use axum::{routing::{get, post}, Json, Router};
use strum::IntoEnumIterator;
use std::sync::Arc;

use crate::{daemon::types::base::DaemonState, server::shared::types::api::ApiResponse};

fn create_router() -> Router<Arc<DaemonState>> {
    Router::new()
        .route("/health", get(health_check))
        .route("/discover", post(handle_discovery_request))
        .route("/execute_test", post(handle_test_execution))
}

// Daemon API handlers
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("Daemon is healthy"))
}

async fn handle_discovery_request(
    State(state): State<Arc<DaemonState>>,
    Json(request): Json<DaemonDiscoveryRequest>,
) -> Result<Json<ApiResponse<DaemonDiscoveryResponse>>, StatusCode> {
    tracing::info!("Received discovery request for session {}", request.session_id);
    
    let state_clone = state.clone();
    tokio::spawn(async move {
        if let Err(e) = run_discovery_session(state_clone, request).await {
            tracing::error!("Discovery session failed: {}", e);
        }
    });

    Ok(Json(ApiResponse::success(DaemonDiscoveryResponse {
        success: true,
        session_id: request.session_id,
        message: "Discovery started".to_string(),
    })))
}

async fn handle_test_execution(
    State(state): State<Arc<DaemonState>>,
    Json(request): Json<DaemonTestRequest>,
) -> Result<Json<ApiResponse<DaemonTestResponse>>, StatusCode> {
    tracing::info!("Received test execution request for session {}", request.session_id);
    
    let state_clone = state.clone();
    tokio::spawn(async move {
        if let Err(e) = run_test_execution(state_clone, request).await {
            tracing::error!("Test execution failed: {}", e);
        }
    });

    Ok(Json(ApiResponse::success(DaemonTestResponse {
        success: true,
        session_id: request.session_id,
        message: "Test execution started".to_string(),
    })))
}

async fn run_discovery_session(state: Arc<DaemonState>, request: DaemonDiscoveryRequest) -> anyhow::Result<()> {
    tracing::info!("Starting discovery session {} on subnets: {:?}", request.session_id, request.target_subnets);
    
    let discovery_config = DiscoveryConfig {
        target_subnets: request.target_subnets,
        discovery_depth: match request.discovery_depth.as_str() {
            "basic" => DiscoveryDepth::Basic,
            "deep" => DiscoveryDepth::Deep,
            _ => DiscoveryDepth::Standard,
        },
        include_services: request.include_services,
        snmp_communities: request.snmp_communities,
        max_concurrent: request.max_concurrent,
        timeout_ms: request.timeout_ms,
        port_scan_enabled: true,
        common_ports: vec![22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, 5900, 8080, 8443],
    };

    let discovery = NetworkDiscovery::new();
    if let Err(e) = discovery.start_discovery(discovery_config).await {
        tracing::error!("Discovery failed: {}", e);
        return Err(e.into());
    }

    let mut last_reported_count = 0;
    let progress_interval = Duration::from_secs(1);
    let mut interval_timer = interval(progress_interval);
    
    loop {
        interval_timer.tick().await;
        
        let progress = discovery.get_progress().await;
        
        // Send progress update to server
        let progress_update = DaemonDiscoveryProgress {
            session_id: request.session_id,
            phase: progress.current_phase.clone(),
            completed: progress.completed,
            total: progress.total_targets,
            discovered_count: progress.discovered_devices,
        };
        
        if let Err(e) = send_progress_update(&state, &progress_update).await {
            tracing::warn!("Failed to send progress update: {}", e);
        }
        
        // Report newly discovered devices
        let discovered_devices = discovery.get_discovered_devices().await;
        if discovered_devices.len() > last_reported_count {
            for device in discovered_devices.iter().skip(last_reported_count) {
                if let Err(e) = report_discovered_device(&state, request.session_id, device).await {
                    tracing::warn!("Failed to report discovered device: {}", e);
                }
            }
            last_reported_count = discovered_devices.len();
        }
        
        if !progress.is_running {
            tracing::info!("Discovery session {} completed", request.session_id);
            break;
        }
    }
    
    Ok(())
}

async fn run_test_execution(state: Arc<DaemonState>, request: DaemonTestRequest) -> anyhow::Result<()> {
    tracing::info!("Executing test for session {}", request.session_id);
    
    let timer = Timer::now();
    let result = request.daemon_test.execute(&timer, &request.node).await?;
    
    // Report result back to server
    if let Err(e) = state.report_test_result(request.session_id, result).await {
        tracing::error!("Failed to report test result: {}", e);
    }
    
    Ok(())
}

async fn send_progress_update(state: &DaemonState, progress: &DaemonDiscoveryProgress) -> anyhow::Result<()> {
    let response = state
        .client
        .post(format!("{}/api/discovery/progress", state.server_url))
        .json(progress)
        .send()
        .await?;
        
    if !response.status().is_success() {
        anyhow::bail!("Failed to send progress update: HTTP {}", response.status());
    }
    
    Ok(())
}

async fn report_discovered_device(
    state: &DaemonState, 
    session_id: Uuid, 
    device: &netfrog_server::components::discovery::types::DiscoveredDevice
) -> anyhow::Result<()> {
    let mut capabilities = Vec::new();
    
    for service in &device.services {
        match service.as_str() {
            s if s.contains("ssh") || s.contains("22/tcp") => {
                // Add SSH capability when implemented
            }
            s if s.contains("http") || s.contains("80/tcp") => {
                // Add HTTP capability when implemented  
            }
            s if s.contains("https") || s.contains("443/tcp") => {
                // Add HTTPS capability when implemented
            }
            s if s.contains("dns") || s.contains("53/tcp") || s.contains("53/udp") => {
                // Add DNS capability when implemented
            }
            _ => {}
        }
    }
    
    let node = Node::new(
        device.hostname.clone().unwrap_or_else(|| format!("device-{}", device.ip)),
        Some(NodeTarget::IpAddress(netfrog_server::components::nodes::types::targets::IpAddressTarget {
            ip: device.ip.clone(),
            port: None,
        })),
        capabilities,
    );
    
    let node_report = DaemonNodeReport {
        session_id,
        node,
    };
    
    let response = state
        .client
        .post(format!("{}/api/discovery/node", state.server_url))
        .json(&node_report)
        .send()
        .await?;
        
    if !response.status().is_success() {
        anyhow::bail!("Failed to report discovered node: HTTP {}", response.status());
    }
    
    Ok(())
}