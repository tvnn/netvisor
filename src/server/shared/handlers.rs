use axum::{routing::get, Json, Router};
use strum::IntoEnumIterator;
use std::{sync::Arc};
use crate::server::{
        capabilities::{handlers as capability_handlers, types::base::Capability}, config::AppState, daemons::handlers as daemon_handlers, diagnostics::{handlers as diagnostic_handlers, types::DiagnosticStatus}, discovery::handlers as discovery_handlers, node_groups::handlers as group_handlers, nodes::{handlers as node_handlers, types::{criticality::TestCriticality, status::NodeStatus, targets::NodeTarget, types::NodeType}}, shared::types::{api::ApiResponse, metadata::{TypeMetadataProvider, TypeRegistry}}, tests::types::base::Test
    };

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api/nodes", node_handlers::create_router())
        .route("/api/registry", get(get_type_registry))
        .nest("/api/capabilities", capability_handlers::create_router())
        .nest("/api/groups", group_handlers::create_router())
        .nest("/api/diagnostics", diagnostic_handlers::create_router())
        .nest("/api/daemons", daemon_handlers::create_router())
        .nest("/api/discovery", discovery_handlers::create_router())
}

async fn get_type_registry() -> Json<ApiResponse<TypeRegistry>> {
    let registry = TypeRegistry {
        test_types: Test::iter().map(|t| t.to_metadata()).collect(),
        node_types: NodeType::iter().map(|t| t.to_metadata()).collect(),
        capabilities: Capability::iter().map(|t| t.to_metadata()).collect(),
        criticality_levels: TestCriticality::iter().map(|t| t.to_metadata()).collect(),
        node_statuses: NodeStatus::iter().map(|t| t.to_metadata()).collect(),
        node_targets: NodeTarget::iter().map(|t| t.to_metadata()).collect(),
        diagnostic_statuses: DiagnosticStatus::iter().map(|t| t.to_metadata()).collect(),
    };
    
    Json(ApiResponse::success(registry))
}