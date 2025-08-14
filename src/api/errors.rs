// src/api/mod.rs - Enhanced API error handling for tests

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use super::responses::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self { status, message }
    }

    pub fn internal_error(message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
    }

    pub fn validation_error(message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message.to_string())
    }

    pub fn not_found(resource: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, format!("{} not found", resource))
    }

    pub fn node_not_found(node_id: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, format!("Node '{}' not found", node_id))
    }

    pub fn group_not_found(group_id: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, format!("Group '{}' not found", group_id))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let response = ApiResponse::<()>::error(self.message);
        (self.status, Json(response)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("Internal error: {}", err);
        Self::internal_error(&err.to_string())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", err);
        match err {
            sqlx::Error::RowNotFound => Self::not_found("Resource"),
            _ => Self::internal_error("Database operation failed"),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        tracing::error!("JSON serialization error: {}", err);
        Self::validation_error("Invalid JSON data")
    }
}