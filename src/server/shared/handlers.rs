use axum::{routing::get, Json, Router};
use strum::IntoEnumIterator;
use std::{sync::Arc};
use crate::server::{
        config::AppState, daemons::handlers as daemon_handlers, discovery::handlers as discovery_handlers, node_groups::handlers as group_handlers, nodes::{handlers as node_handlers, types::{targets::NodeTarget}}, services::types::base::{ServiceDiscriminants}, shared::types::{api::ApiResponse, metadata::{TypeMetadataProvider, TypeRegistry}}, subnets::handlers as subnet_handlers
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api/nodes", node_handlers::create_router())
        .route("/api/registry", get(get_type_registry))
        .nest("/api/groups", group_handlers::create_router())
        .nest("/api/daemons", daemon_handlers::create_router())
        .nest("/api/discovery", discovery_handlers::create_router())
        .nest("/api/subnets", subnet_handlers::create_router())
}

async fn get_type_registry() -> Json<ApiResponse<TypeRegistry>> {
    let registry = TypeRegistry {
        services: ServiceDiscriminants::iter().map(|t| t.to_metadata()).collect(),
        node_targets: NodeTarget::iter().map(|t| t.to_metadata()).collect(),
    };
    
    Json(ApiResponse::success(registry))
}