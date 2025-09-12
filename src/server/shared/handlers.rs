use axum::{routing::get, Json, Router};
use strum::IntoEnumIterator;
use std::{sync::Arc};
use crate::server::hosts::types::targets::HostTarget;
use crate::server::services::types::types::ServiceType;
use crate::server::topology::types::base::EdgeType;
use crate::server::{
        config::AppState, daemons::handlers as daemon_handlers, topology::handlers as topology_handlers, discovery::handlers as discovery_handlers, host_groups::handlers as group_handlers, hosts::{handlers as host_handlers}, shared::types::{api::ApiResponse, metadata::{TypeMetadataProvider, TypeRegistry}}, subnets::{handlers as subnet_handlers, types::base::SubnetType}
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api/hosts", host_handlers::create_router())
        .route("/api/registry", get(get_type_registry))
        .nest("/api/host_groups", group_handlers::create_router())
        .nest("/api/daemons", daemon_handlers::create_router())
        .nest("/api/discovery", discovery_handlers::create_router())
        .nest("/api/subnets", subnet_handlers::create_router())
        .nest("/api/topology", topology_handlers::create_router())
}

async fn get_type_registry() -> Json<ApiResponse<TypeRegistry>> {
    let registry = TypeRegistry {
        services: ServiceType::iter().map(|t| t.to_metadata()).collect(),
        host_targets: HostTarget::iter().map(|t| t.to_metadata()).collect(),
        subnet_types: SubnetType::iter().map(|t| t.to_metadata()).collect(),
        graph_edge_types: EdgeType::iter().map(|t| t.to_metadata()).collect(),
    };
    
    Json(ApiResponse::success(registry))
}