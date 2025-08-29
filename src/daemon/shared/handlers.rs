use std::{sync::Arc};
use axum::Router;
use crate::daemon::{
    discovery::handlers as discovery_handlers, runtime::types::base::DaemonState, tests::handlers as test_handlers
};

pub fn create_router() -> Router<Arc<DaemonState>> {
    Router::new()
        .nest("/api/discovery", discovery_handlers::create_router())
        .nest("/api/tests", test_handlers::create_router())
}