use std::sync::Arc;

use axum::{Router};
use crate::daemon::{
    discovery::handlers as discovery_handlers, runtime::handlers as runtime_handlers, shared::storage::ConfigStore, tests::handlers as test_handlers
};

pub fn create_router() -> Router<Arc<ConfigStore>> {
    Router::new()
        .nest("/api/discovery", discovery_handlers::create_router())
        .nest("/api/runtime", runtime_handlers::create_router())
        .nest("/api/tests", test_handlers::create_router())
}