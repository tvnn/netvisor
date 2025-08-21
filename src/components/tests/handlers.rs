use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use crate::{
    api::{ApiError, ApiResponse, ApiResult}, 
    components::{
        nodes::{service::NodeService}, 
        tests::types::{base::Test, configs::*},
    }, shared::{schema::{NodeContext, TestConfigSchema}}, AppState
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/schema", post(get_test_config_schema))
}

#[derive(Deserialize)]
pub struct SchemaRequest {
    pub test_type: String,
    pub node_context: NodeContext,
}

pub async fn get_test_config_schema(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SchemaRequest>,
) -> ApiResult<Json<ApiResponse<TestConfigSchema>>> {
    
    // Get available nodes for node selectors
    let node_service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    
    let available_nodes = node_service.get_all_nodes().await
        .map_err(|e| ApiError::internal_error(&format!("Failed to load nodes: {}", e)))?;
    
    // Parse test type and create default instance
    let test_instance = match request.test_type.as_str() {
        "Connectivity" => Test::Connectivity(ConnectivityConfig::default()),
        "ServiceHealth" => Test::ServiceHealth(ServiceHealthConfig::default()),
        "DnsResolution" => Test::DnsResolution(DnsResolutionConfig::default()),
        "DnsLookup" => Test::DnsLookup(DnsLookupConfig::default()),
        "VpnSubnetAccess" => Test::VpnSubnetAccess(VpnSubnetAccessConfig::default()),
        "DnsOverHttps" => Test::DnsOverHttps(DnsOverHttpsConfig::default()),
        "ReverseDns" => Test::ReverseDns(ReverseDnsConfig::default()),
        _ => return Err(ApiError::bad_request(&format!("Invalid test type: {}", request.test_type))),
    };
    
    // Generate schema
    let schema = test_instance.generate_schema(&request.node_context, &available_nodes);
    
    Ok(Json(ApiResponse::success(schema)))
}