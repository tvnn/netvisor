use std::{net::{SocketAddr}, time::Duration};
use anyhow::Error;
use reqwest::Client;
use tokio::{net::TcpStream, time::timeout};
use crate::components::{
    nodes::{service::NodeService, types::{
        base::Node, 
        targets::NodeTarget
    }}, 
    tests::types::{configs::*, execution::*}
};

/// Execute connectivity test - tests TCP connection to node's target
pub async fn execute_connectivity_test(
    config: &ConnectivityConfig,
    timer: &Timer,
    node: &Node,
    node_service: &NodeService
) -> Result<TestResult, Error> {
    let timeout_duration = Duration::from_millis(config.timeout_ms.unwrap_or(30000) as u64);
    
    let (connection_result, dns_result) = match &node.base.target {

        NodeTarget::IpAddress(ip_config) => {
            // Direct IP connection - no DNS needed
            let port = ip_config.port.unwrap_or(80);
            let addr = SocketAddr::new(ip_config.ip, port);
            (timeout(timeout_duration, TcpStream::connect(addr)).await, None)
        },
        NodeTarget::Hostname(_) | NodeTarget::Service(_) => {
            // Get DNS resolver for cases that need it
            let dns_resolver_node = match config.dns_resolver_node {
                Some(node_id) => {
                    match node_service.get_node(&node_id).await? {
                        Some(node) => node,
                        None => return Err(Error::msg(format!("DNS resolver node {} not found", node_id))),
                    }
                },
                None => return Err(Error::msg("DNS resolver node required for hostname/service targets")),
            };

            let dns_capability = match dns_resolver_node.as_dns_capability() {
                Some(capability) => capability,
                None => {
                    return Ok(TestResult {
                        success: false,
                        message: format!("Node {} is not a valid DNS resolver. Select a node with DNS capability.", dns_resolver_node.base.name),
                        duration_ms: timer.elapsed_ms(),
                        executed_at: timer.datetime(),
                        details: None,
                        criticality: None,
                    });
                }
            };

            // Handle the specific DNS-requiring case
            match &node.base.target {
                NodeTarget::Hostname(hostname_config) => {
                    let dns_result = dns_capability.resolve_domain(&hostname_config.hostname, config.timeout_ms).await?;
                    let port = hostname_config.port.unwrap_or(80);
                    let addr = SocketAddr::new(dns_result.resolved_addresses[0], port);
                    (timeout(timeout_duration, TcpStream::connect(addr)).await, Some(dns_result))
                },
                NodeTarget::Service(service_config) => {
                    let dns_result = dns_capability.resolve_domain(&service_config.hostname, config.timeout_ms).await?;
                    let port = service_config.port.unwrap_or(service_config.protocol.default_port());
                    let addr = SocketAddr::new(dns_result.resolved_addresses[0], port);
                    (timeout(timeout_duration, TcpStream::connect(addr)).await, Some(dns_result))
                },
                _ => unreachable!(),
            }
        }
    };

    let target = node.base.target.to_string();

    let (success, message, details) = match connection_result {
        Ok(Ok(_stream)) => {
            (
                true,
                format!("Successfully connected to {}", target),
                serde_json::json!({
                    "target": target,
                    "connection_time_ms": timer.elapsed_ms(),
                    "dns_server": dns_result,
                    "target_type": node.base.target.variant_name()
                })
            )
        },
        Ok(Err(e)) => {
            (
                false,
                format!("Connection to {} failed: {}", target, e),
                serde_json::json!({
                    "target": target,
                    "error": e.to_string(),
                    "dns_server": dns_result,
                    "target_type": node.base.target.variant_name()
                })
            )
        },
        Err(_) => {
            (
                false,
                format!("Connection to {} timed out after {}ms", target, timeout_duration.as_millis()),
                serde_json::json!({
                    "target": target,
                    "timeout_ms": timeout_duration.as_millis(),
                    "dns_server": dns_result,
                    "target_type": node.base.target.variant_name()
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

/// Execute service health test - HTTP request to node's service endpoint
pub async fn execute_service_health_test(
    config: &ServiceHealthConfig,
    timer: &Timer,
    node: &Node,
    _node_service: &NodeService
) -> Result<TestResult, Error> {
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