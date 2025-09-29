use std::{sync::Arc};
use axum::Router;
use crate::daemon::{
    discovery::handlers as discovery_handlers, runtime::types::DaemonAppState
};

pub fn create_router() -> Router<Arc<DaemonAppState>> {
    Router::new()
        .nest("/api/discovery", discovery_handlers::create_router())
}