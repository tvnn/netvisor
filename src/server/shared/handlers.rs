use axum::{routing::get, Json, Router};
use strum::IntoEnumIterator;
use std::{sync::Arc};
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::shared::constants::{Entity};
use crate::server::shared::types::metadata::{MetadataProvider, MetadataRegistry};
use crate::server::topology::types::base::EdgeType;
use crate::server::{
        config::AppState, 
        daemons::handlers as daemon_handlers, 
        topology::handlers as topology_handlers, 
        discovery::handlers as discovery_handlers, 
        groups::handlers as group_handlers, 
        services::handlers as service_handlers,
        hosts::{handlers as host_handlers}, 
        subnets::{handlers as subnet_handlers, types::base::SubnetType},
        shared::types::{api::ApiResponse}, 
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api/hosts", host_handlers::create_router())
        .route("/api/metadata", get(get_metadata_registry))
        .nest("/api/groups", group_handlers::create_router())
        .nest("/api/daemons", daemon_handlers::create_router())
        .nest("/api/discovery", discovery_handlers::create_router())
        .nest("/api/subnets", subnet_handlers::create_router())
        .nest("/api/topology", topology_handlers::create_router())
        .nest("/api/services", service_handlers::create_router())
        .route("/api/health", get(get_health))
}

async fn get_metadata_registry() -> Json<ApiResponse<MetadataRegistry>> {
    let registry = MetadataRegistry {
        service_definitions: ServiceDefinitionRegistry::all_service_definitions().iter().map(|t| t.to_metadata()).collect(),
        subnet_types: SubnetType::iter().map(|t| t.to_metadata()).collect(),
        edge_types: EdgeType::iter().map(|t| t.to_metadata()).collect(),
        entities: Entity::iter().map(|e| e.to_metadata()).collect()
    };
    
    Json(ApiResponse::success(registry))
}

async fn get_health() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("API Running".to_string()))
}