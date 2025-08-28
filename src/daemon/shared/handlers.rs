use std::{sync::Arc, time::Duration};

use axum::{body::Body, http::{Request, Response}, Router};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use crate::daemon::{
    discovery::handlers as discovery_handlers, shared::storage::ConfigStore, tests::handlers as test_handlers
};

pub fn create_router() -> Router<Arc<ConfigStore>> {
    Router::new()
        .nest("/api/discovery", discovery_handlers::create_router())
        .nest("/api/tests", test_handlers::create_router())
        .layer(TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            tracing::info_span!(
                "request",
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        })
        .on_request(|_request: &Request<Body>, _span: &Span| {
            tracing::debug!("started processing request");
        })
        .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
            tracing::debug!(
                latency = ?latency.as_millis(),
                status = %response.status(),
                "finished processing request"
            );
        })
        .on_failure(|failure_classification: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
            tracing::error!(
                latency = ?latency.as_millis(),
                failure = ?failure_classification,
                "request failed"
            );
        })
    )
}