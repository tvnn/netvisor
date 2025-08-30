use std::{net::{SocketAddr}, time::Duration};
use anyhow::Error;
use reqwest::Client;
use tokio::{net::TcpStream, time::timeout};
use crate::server::{
    capabilities::types::{base::Capability, configs::{HttpEndpointCompatible}}, nodes::types::{
        base::Node, 
        targets::NodeTarget
    }, tests::{types::{configs::*, execution::*}, utilities::dns::{DnsServerConfig, DnsUtils}}
};

/// Execute connectivity test - tests TCP connection to node's target
pub async fn execute_connectivity_test(
    config: &ConnectivityConfig,
    timer: &Timer,
    node: &Node,
    dns_server: Option<&DnsServerConfig>,
    capability: &Capability
) -> Result<TestResult, Error> {
    let timeout_duration = Duration::from_millis(config.timeout_ms.unwrap_or(30000) as u64);

    let port = match capability.config_base().port {
        Some(p) => p,
        None => return Err(Error::msg("Selected capability does not have a port"))
    };
    
    let (connection_result, dns_result) = match &node.base.target {

        NodeTarget::IpAddress(ip_config) => {
            // Direct IP connection - no DNS needed
            let addr = SocketAddr::new(ip_config.ip, port);
            (timeout(timeout_duration, TcpStream::connect(addr)).await, None)
        },
        NodeTarget::Hostname(hostname_config) => {

            let Some(dns) = dns_server else {
                return Err(Error::msg("Connectivity test targeting service requires a DNS resolver in test config"))
            };

            let dns_result = DnsUtils::resolve_domain(dns, &hostname_config.hostname, config.timeout_ms).await?;
            let addr = SocketAddr::new(dns_result.resolved_addresses[0], port);
            (timeout(timeout_duration, TcpStream::connect(addr)).await, Some(dns_result))
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

/// Execute service health test - HTTP/S request to node's service endpoint
pub async fn execute_service_health_test(
    config: &ServiceHealthConfig,
    timer: &Timer,
    node: &Node,
    capability: &Capability
) -> Result<TestResult, Error> {
    let timeout_duration = Duration::from_millis(config.timeout_ms.unwrap_or(30000) as u64);
    
    let service_url = match capability {
        Capability::Daemon(config) => config.as_endpoint(&node.base.target),
        Capability::Http(config) => config.as_endpoint(&node.base.target),
        Capability::Https(config) => config.as_endpoint(&node.base.target),
        _ => Err(Error::msg(format!("Capability {} for node {} doesn't support endpoints", capability.config_base().name, node.id))),
    }?;
    
    // Create HTTP client with timeout
    let client = Client::builder()
        .timeout(timeout_duration)
        .build()
        .map_err(|e| Error::msg(format!("Failed to create HTTP client: {}", e)))?;

    // Perform HTTP request
    let request_result = timeout(timeout_duration, client.get(service_url.clone()).send()).await;

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