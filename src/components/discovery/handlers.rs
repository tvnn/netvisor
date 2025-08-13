use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json},
};
use std::sync::Arc;
use crate::AppState;
use crate::shared::handlers::ApiResponse;

use crate::components::discovery::types::{DiscoveryConfig, StartDiscoveryRequest, DiscoveryProgress, DiscoveredDevice};
use crate::components::discovery::types::DiscoveryDepth::{Basic, Deep, Standard};
use crate::components::nodes::types::NetworkNode;

pub async fn start_discovery(
    State(state): State<Arc<AppState>>,
    Json(request): Json<StartDiscoveryRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let config = DiscoveryConfig {
        target_subnets: request.target_subnets,
        discovery_depth: match request.discovery_depth.as_str() {
            "basic" => Basic,
            "deep" => Deep,
            _ => Standard,
        },
        include_services: request.include_services,
        snmp_communities: request.snmp_communities,
        max_concurrent: request.max_concurrent,
        timeout_ms: request.timeout,
        port_scan_enabled: true,
        common_ports: vec![22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, 5900, 8080, 8443],
    };

    let discovery = state.discovery.clone();
    tokio::spawn(async move {
        let discovery_guard = discovery.lock().await;
        if let Err(e) = discovery_guard.start_discovery(config).await {
            tracing::error!("Discovery failed: {}", e);
        }
    });

    Ok(Json(ApiResponse::success(())))
}

pub async fn stop_discovery(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let discovery = state.discovery.lock().await;
    discovery.stop_discovery().await;
    Ok(Json(ApiResponse::success(())))
}

pub async fn get_discovery_progress(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<DiscoveryProgress>>, StatusCode> {
    let discovery = state.discovery.lock().await;
    let progress = discovery.get_progress().await;
    Ok(Json(ApiResponse::success(progress)))
}

pub async fn get_discovered_devices(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<DiscoveredDevice>>>, StatusCode> {
    let discovery = state.discovery.lock().await;
    let devices = discovery.get_discovered_devices().await;
    Ok(Json(ApiResponse::success(devices)))
}

pub async fn accept_discovered_device(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<String>,
) -> Result<Json<ApiResponse<NetworkNode>>, StatusCode> {
    let discovery = state.discovery.lock().await;
    
    match discovery.accept_device(&device_id).await {
        Ok(node) => {
            // Save the new node to storage
            match state.node_storage.save_node(&node).await {
                Ok(_) => Ok(Json(ApiResponse::success(node))),
                Err(e) => {
                    tracing::error!("Failed to save accepted device as node: {}", e);
                    Ok(Json(ApiResponse::error(format!("Failed to save node: {}", e))))
                }
            }
        },
        Err(e) => Ok(Json(ApiResponse::error(e)))
    }
}

pub async fn reject_discovered_device(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let discovery = state.discovery.lock().await;
    
    match discovery.reject_device(&device_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => Ok(Json(ApiResponse::error(e)))
    }
}