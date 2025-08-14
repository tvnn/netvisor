// NetFrog Library - Core type system exports
pub mod api;
pub mod components;
pub mod shared;
pub mod config;

// Re-export core types for external use
pub use core::*;

// Re-export AppState from main for handlers
pub struct AppState {
    pub config: config::ServerConfig,
    pub node_storage: std::sync::Arc<dyn crate::components::nodes::storage::NodeStorage>,
    pub node_group_storage: std::sync::Arc<dyn crate::components::node_groups::storage::NodeGroupStorage>,
    pub diagnostic_storage: std::sync::Arc<dyn crate::shared::storage::DiagnosticStorage>,
}