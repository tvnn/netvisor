use std::time::Duration;
use anyhow::{Error, Result};
use reqwest::Client;
use tokio::time::timeout;
use crate::components::{
    nodes::types::base::{Node},
    tests::types::configs::*,
    tests::types::execution::*
};

/// Execute service health test - HTTP request to node's service endpoint
pub async fn execute_service_health_test(
    config: &ServiceHealthConfig,
    timer: &Timer,
    node: &Node,
) -> Result<TestResult> {
    let timeout_duration = Duration::from_millis(config.timeout_ms.unwrap_or(30000) as u64);
    
    // Extract service URL from node configuration
    let service_url = &node.base.target.to_string();
    
    // Create HTTP client with timeout
    let client = Client::builder()
        .timeout(timeout_duration)
        .build()
        .map_err(|e| Error::msg(format!("Failed to create HTTP client: {}", e)))?;

    // Perform HTTP request
    let request_result = timeout(timeout_duration, client.get(service_url).send()).await;

    let (success, message, details) = match request_result {
        Ok(Ok(response)) => {
            let status_code = response.status().as_u16();
            let expected_status = config.expected_status_code;
            
            if status_code == expected_status {
                (
                    true,
                    format!("Service health check passed: {} returned {}", service_url, status_code),
                    serde_json::json!({
                        "url": service_url,
                        "status_code": status_code,
                        "expected_status": expected_status,
                        "response_time_ms": timer.elapsed_ms(),
                        "headers": response.headers().len()
                    })
                )
            } else {
                (
                    false,
                    format!("Unexpected status code: expected {}, got {}", expected_status, status_code),
                    serde_json::json!({
                        "url": service_url,
                        "status_code": status_code,
                        "expected_status": expected_status,
                        "response_time_ms": timer.elapsed_ms()
                    })
                )
            }
        },
        Ok(Err(e)) => {
            (
                false,
                format!("Service health check failed: {}", e),
                serde_json::json!({
                    "url": service_url,
                    "error": e.to_string(),
                    "request_timeout_ms": timeout_duration.as_millis()
                })
            )
        },
        Err(_) => {
            (
                false,
                format!("Service request to {} timed out after {}ms", service_url, timeout_duration.as_millis()),
                serde_json::json!({
                    "url": service_url,
                    "timeout_ms": timeout_duration.as_millis(),
                    "error": "Request timeout"
                })
            )
        }
    };

    Ok(TestResult {
        success,
        message,
        duration_ms: timer.elapsed_ms(),
        executed_at: timer.datetime(),
        details: Some(details),
        criticality: None,
    })
}