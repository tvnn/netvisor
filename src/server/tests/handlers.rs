use axum::{extract::State, routing::post, Json, Router};
use std::{sync::Arc, collections::HashMap};
use strum::IntoEnumIterator;
use crate::{
    server::{
        nodes::service::NodeService, shared::types::api::{ApiError, ApiResponse, ApiResult}, tests::types::{api::{SchemaRequest, SchemaResponse}, base::{Test, TestDiscriminants}, configs::*}
    }, 
    server::config::AppState
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/schemas", post(get_test_schemas))  // Single endpoint handles all cases
}

impl TestDiscriminants {
    fn create_default_test(&self) -> Test {
        match self {
            TestDiscriminants::Connectivity => Test::Connectivity(ConnectivityConfig::default()),
            TestDiscriminants::ServiceHealth => Test::ServiceHealth(ServiceHealthConfig::default()),
            TestDiscriminants::DnsResolution => Test::DnsResolution(DnsResolutionConfig::default()),
            TestDiscriminants::DnsLookup => Test::DnsLookup(DnsLookupConfig::default()),
            TestDiscriminants::VpnSubnetAccess => Test::VpnSubnetAccess(VpnSubnetAccessConfig::default()),
            TestDiscriminants::DnsOverHttps => Test::DnsOverHttps(DnsOverHttpsConfig::default()),
            TestDiscriminants::ReverseDns => Test::ReverseDns(ReverseDnsConfig::default()),
        }
    }
}

pub async fn get_test_schemas(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SchemaRequest>,
) -> ApiResult<Json<ApiResponse<SchemaResponse>>> {
    
    let node_service = NodeService::new(state.node_storage.clone(), state.node_group_storage.clone());
    let available_nodes = node_service.get_all_nodes().await
        .map_err(|e| ApiError::internal_error(&format!("Failed to load nodes: {}", e)))?;
    
    let mut schemas = HashMap::new();
    
    // Determine which test types to generate schemas for
    let test_types = match request.test_types {
        Some(types) => types,  // Specific types requested
        None => TestDiscriminants::iter().collect(),  // All types
    };
    
    for test_discriminant in test_types {
        let test_instance = test_discriminant.create_default_test();
        let schema = test_instance.generate_schema(&request.node_context, &available_nodes);
        schemas.insert(test_discriminant, schema);
    }
    
    Ok(Json(ApiResponse::success(SchemaResponse { schemas })))
}