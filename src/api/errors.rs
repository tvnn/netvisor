use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
        }
    }

    pub fn with_details(code: &str, message: &str, details: serde_json::Value) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: Some(details),
        }
    }

    // Common error constructors
    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::new(
            "RESOURCE_NOT_FOUND",
            &format!("{} with id '{}' not found", resource, id),
        )
    }

    pub fn validation_error(message: &str) -> Self {
        Self::new("VALIDATION_ERROR", message)
    }

    pub fn internal_error(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }

    pub fn test_incompatible(test_type: &str, node_name: &str, reason: &str) -> Self {
        Self::with_details(
            "TEST_INCOMPATIBLE",
            &format!("Test '{}' is not compatible with node '{}'", test_type, node_name),
            serde_json::json!({ "reason": reason }),
        )
    }

    pub fn node_not_found(node_id: &str) -> Self {
        Self::not_found("Node", node_id)
    }

    pub fn group_not_found(group_id: &str) -> Self {
        Self::not_found("Node Group", group_id)
    }

    pub fn diagnostic_not_found(diagnostic_id: &str) -> Self {
        Self::not_found("Diagnostic Execution", diagnostic_id)
    }

    pub fn invalid_configuration(message: &str) -> Self {
        Self::new("INVALID_CONFIGURATION", message)
    }

    pub fn monitoring_error(message: &str) -> Self {
        Self::new("MONITORING_ERROR", message)
    }

    pub fn test_execution_error(message: &str) -> Self {
        Self::new("TEST_EXECUTION_ERROR", message)
    }

    pub fn database_error(message: &str) -> Self {
        Self::new("DATABASE_ERROR", message)
    }

    pub fn unauthorized() -> Self {
        Self::new("UNAUTHORIZED", "Authentication required")
    }

    pub fn forbidden(message: &str) -> Self {
        Self::new("FORBIDDEN", message)
    }

    pub fn rate_limited() -> Self {
        Self::new("RATE_LIMITED", "Too many requests")
    }

    pub fn service_unavailable(service: &str) -> Self {
        Self::new(
            "SERVICE_UNAVAILABLE",
            &format!("{} service is temporarily unavailable", service),
        )
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.code.as_str() {
            "RESOURCE_NOT_FOUND" => StatusCode::NOT_FOUND,
            "VALIDATION_ERROR" => StatusCode::BAD_REQUEST,
            "INVALID_CONFIGURATION" => StatusCode::BAD_REQUEST,
            "TEST_INCOMPATIBLE" => StatusCode::BAD_REQUEST,
            "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
            "FORBIDDEN" => StatusCode::FORBIDDEN,
            "RATE_LIMITED" => StatusCode::TOO_MANY_REQUESTS,
            "SERVICE_UNAVAILABLE" => StatusCode::SERVICE_UNAVAILABLE,
            "MONITORING_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            "TEST_EXECUTION_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            "DATABASE_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        // Check if the error chain contains specific error types
        let error_msg = err.to_string();
        
        if error_msg.contains("not found") {
            Self::not_found("Resource", "unknown")
        } else if error_msg.contains("validation") || error_msg.contains("invalid") {
            Self::validation_error(&error_msg)
        } else if error_msg.contains("database") || error_msg.contains("sql") {
            Self::database_error(&error_msg)
        } else {
            Self::internal_error(&error_msg)
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::new("RESOURCE_NOT_FOUND", "Resource not found"),
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    Self::validation_error("Resource already exists")
                } else if db_err.is_foreign_key_violation() {
                    Self::validation_error("Referenced resource does not exist")
                } else if db_err.is_check_violation() {
                    Self::validation_error("Data constraint violation")
                } else {
                    Self::database_error(&format!("Database error: {}", db_err))
                }
            },
            sqlx::Error::PoolTimedOut => {
                Self::service_unavailable("Database")
            },
            sqlx::Error::Io(io_err) => {
                Self::internal_error(&format!("I/O error: {}", io_err))
            },
            sqlx::Error::Tls(tls_err) => {
                Self::internal_error(&format!("TLS error: {}", tls_err))
            },
            sqlx::Error::Protocol(protocol_err) => {
                Self::database_error(&format!("Database protocol error: {}", protocol_err))
            },
            sqlx::Error::TypeNotFound { type_name } => {
                Self::internal_error(&format!("Database type not found: {}", type_name))
            },
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                Self::internal_error(&format!("Column index {} out of bounds (len: {})", index, len))
            },
            sqlx::Error::ColumnNotFound(column_name) => {
                Self::internal_error(&format!("Column not found: {}", column_name))
            },
            sqlx::Error::ColumnDecode { index, source } => {
                Self::internal_error(&format!("Failed to decode column {}: {}", index, source))
            },
            sqlx::Error::Decode(decode_err) => {
                Self::internal_error(&format!("Failed to decode data: {}", decode_err))
            },
            sqlx::Error::AnyDriverError(any_err) => {
                Self::database_error(&format!("Driver error: {}", any_err))
            },
            _ => Self::database_error(&format!("Database error: {}", err)),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        Self::validation_error(&format!("JSON serialization error: {}", err))
    }
}

impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> Self {
        Self::validation_error(&format!("Invalid UUID: {}", err))
    }
}

impl From<chrono::ParseError> for ApiError {
    fn from(err: chrono::ParseError) -> Self {
        Self::validation_error(&format!("Invalid date/time format: {}", err))
    }
}

impl From<std::net::AddrParseError> for ApiError {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::validation_error(&format!("Invalid IP address: {}", err))
    }
}

impl From<std::num::ParseIntError> for ApiError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::validation_error(&format!("Invalid number format: {}", err))
    }
}

impl From<tokio::time::error::Elapsed> for ApiError {
    fn from(_err: tokio::time::error::Elapsed) -> Self {
        Self::new("TIMEOUT", "Operation timed out")
    }
}

// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            ApiError::not_found("Node", "123").into_response().status(),
            StatusCode::NOT_FOUND
        );
        
        assert_eq!(
            ApiError::validation_error("Invalid input").into_response().status(),
            StatusCode::BAD_REQUEST
        );
        
        assert_eq!(
            ApiError::unauthorized().into_response().status(),
            StatusCode::UNAUTHORIZED
        );
        
        assert_eq!(
            ApiError::internal_error("Something went wrong").into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_details() {
        let error = ApiError::with_details(
            "TEST_ERROR",
            "Test message",
            serde_json::json!({"key": "value"})
        );
        
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test message");
        assert!(error.details.is_some());
    }

    #[test]
    fn test_convenience_constructors() {
        let node_error = ApiError::node_not_found("123");
        assert_eq!(node_error.code, "RESOURCE_NOT_FOUND");
        assert!(node_error.message.contains("Node"));
        assert!(node_error.message.contains("123"));

        let group_error = ApiError::group_not_found("456");
        assert_eq!(group_error.code, "RESOURCE_NOT_FOUND");
        assert!(group_error.message.contains("Node Group"));
        assert!(group_error.message.contains("456"));

        let test_error = ApiError::test_incompatible("VpnConnectivity", "printer", "Wrong node type");
        assert_eq!(test_error.code, "TEST_INCOMPATIBLE");
        assert!(test_error.message.contains("VpnConnectivity"));
        assert!(test_error.message.contains("printer"));
        assert!(test_error.details.is_some());
    }
}