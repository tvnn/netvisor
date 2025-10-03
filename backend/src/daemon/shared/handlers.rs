use crate::daemon::{discovery::handlers as discovery_handlers, runtime::types::DaemonAppState};
use axum::Router;
use std::sync::Arc;

pub fn create_router() -> Router<Arc<DaemonAppState>> {
    Router::new().nest("/api/discovery", discovery_handlers::create_router())
}
