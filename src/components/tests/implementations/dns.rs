use anyhow::{Error, Result};
use std::net::IpAddr;
use crate::components::{
    nodes::{service::NodeService, types::{base::Node, targets::NodeTarget}},
    tests::types::{configs::*, execution::*}
};

/// Execute DNS resolution test - test DNS server's ability to resolve domains
pub async fn execute_dns_resolution_test(
    config: &DnsResolutionConfig,
    timer: &Timer,
    node: &Node,
    _node_service: &NodeService
) -> Result<TestResult> {
    // This test uses the node itself as the DNS server to test
    let dns_capability = match node.as_dns_capability() {
        Some(capability) => capability,
        None => {
            return Ok(TestResult {
                success: false,
                message: format!("Node {} does not have DNS capability. Add DNS Service capability to this node.", node.base.name),
                duration_ms: timer.elapsed_ms(),
                executed_at: timer.datetime(),
                details: Some(serde_json::json!({
                    "error": "Node lacks DNS service capability",
                    "node_name": node.base.name,
                    "required_capability": "DnsService"
                })),
                criticality: None,
            });
        }
    };

    // Use the DNS capability to resolve the domain
    let dns_result = dns_capability.resolve_domain(&config.domain, config.timeout_ms).await?;

    let (success, message, details) = if dns_result.success {
        let expected_ip = config.expected_ip;
        
        if dns_result.resolved_addresses.contains(&expected_ip) {
            (
                true,
                format!("DNS resolution successful: {} resolved to expected IP {}", config.domain, expected_ip),
                serde_json::json!({
                    "domain": config.domain,
                    "resolved_ips": dns_result.resolved_addresses,
                    "expected_ip": expected_ip,
                    "dns_server": dns_result.dns_server,
                    "resolution_time_ms": dns_result.duration_ms
                })
            )
        } else {
            (
                false,
                format!("DNS resolved {} to {:?} but expected {}", config.domain, dns_result.resolved_addresses, expected_ip),
                serde_json::json!({
                    "domain": config.domain,
                    "resolved_ips": dns_result.resolved_addresses,
                    "expected_ip": expected_ip,
                    "dns_server": dns_result.dns_server,
                    "resolution_time_ms": dns_result.duration_ms
                })
            )
        }
    } else {
        let error_msg = dns_result.error_message.clone().unwrap_or_else(|| "Unknown error".to_string());
        (
            false,
            format!("DNS resolution failed for {}: {}", config.domain, error_msg),
            serde_json::json!({
                "domain": config.domain,
                "error": dns_result.error_message,
                "dns_server": dns_result.dns_server,
                "resolution_time_ms": dns_result.duration_ms
            })
        )
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

/// Execute DNS lookup test - validate that this node's domain resolves correctly using a DNS resolver
pub async fn execute_dns_lookup_test(
    config: &DnsLookupConfig,
    timer: &Timer,
    node: &Node,
    node_service: &NodeService
) -> Result<TestResult, Error> {
    
    let dns_resolver_node = match node_service.get_node(&config.dns_resolver_node).await? {
        Some(node) => node,
        None => return Err(Error::msg(format!("DNS resolver node {} not found", &config.dns_resolver_node))),
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

    // Get the domain to lookup from node's hostname target
    let domain_to_lookup = match &node.base.target {
        NodeTarget::Hostname(hostname_config) => hostname_config.hostname.clone(),
        NodeTarget::Service(service_config) => service_config.hostname.clone(),
        NodeTarget::IpAddress(_) => {
            return Ok(TestResult {
                success: false,
                message: "DNS lookup test requires node with hostname or service target".to_string(),
                duration_ms: timer.elapsed_ms(),
                executed_at: timer.datetime(),
                details: Some(serde_json::json!({
                    "error": "Node must have hostname or service target for DNS lookup validation",
                    "target_type": node.base.target.variant_name()
                })),
                criticality: None,
            });
        }
    };

    // Use DNS capability to resolve the domain
    let dns_result = dns_capability.resolve_domain(&domain_to_lookup, config.timeout_ms).await?;

    let (success, message, details) = if dns_result.success {
        let expected_ip = config.expected_ip;
        
        if dns_result.resolved_addresses.contains(&expected_ip) {
            (
                true,
                format!("DNS lookup validation passed: {} resolves to expected IP {}", domain_to_lookup, expected_ip),
                serde_json::json!({
                    "domain": domain_to_lookup,
                    "resolved_ips": dns_result.resolved_addresses,
                    "expected_ip": expected_ip,
                    "dns_server": dns_result.dns_server,
                    "lookup_time_ms": dns_result.duration_ms
                })
            )
        } else {
            (
                false,
                format!("DNS lookup failed: {} resolves to {:?} but expected {}", domain_to_lookup, dns_result.resolved_addresses, expected_ip),
                serde_json::json!({
                    "domain": domain_to_lookup,
                    "resolved_ips": dns_result.resolved_addresses,
                    "expected_ip": expected_ip,
                    "dns_server": dns_result.dns_server,
                    "lookup_time_ms": dns_result.duration_ms
                })
            )
        }
    } else {
        let error_msg = dns_result.error_message.clone().unwrap_or_else(|| "Unknown error".to_string());
        (
            false,
            format!("DNS lookup failed for {}: {}", domain_to_lookup, error_msg),
            serde_json::json!({
                "domain": domain_to_lookup,
                "error": dns_result.error_message,
                "dns_server": dns_result.dns_server,
                "lookup_time_ms": dns_result.duration_ms
            })
        )
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

/// Execute reverse DNS test - test reverse DNS lookup capability using a DNS resolver
pub async fn execute_reverse_dns_lookup_test(
    config: &ReverseDnsConfig,
    timer: &Timer,
    node: &Node,
    node_service: &NodeService
) -> Result<TestResult> {
    
    let dns_resolver_node = match node_service.get_node(&config.dns_resolver_node).await? {
        Some(node) => node,
        None => return Err(Error::msg(format!("DNS resolver node {} not found", &config.dns_resolver_node))),
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

    // Get the IP to reverse resolve from the node's IP target
    let ip_to_resolve: IpAddr = match &node.base.target {
        NodeTarget::IpAddress(ip_config) => ip_config.ip,
        _ => {
            return Ok(TestResult {
                success: false,
                message: "Reverse DNS test requires node with IP address target".to_string(),
                duration_ms: timer.elapsed_ms(),
                executed_at: timer.datetime(),
                details: Some(serde_json::json!({
                    "error": "Node must have IP address for reverse DNS lookup",
                    "target_type": node.base.target.variant_name()
                })),
                criticality: None,
            });
        }
    };

    // Use DNS capability for reverse lookup
    let dns_result = dns_capability.reverse_lookup_ip(ip_to_resolve, config.timeout_ms).await?;

    let (success, message, details) = if dns_result.success && !dns_result.resolved_domains.is_empty() {
        let resolved_domain = &dns_result.resolved_domains[0];
        let expected_domain = &config.expected_domain;
        
        if resolved_domain.contains(expected_domain) {
            (
                true,
                format!("Reverse DNS successful: {} → {}", ip_to_resolve, resolved_domain),
                serde_json::json!({
                    "ip_address": ip_to_resolve,
                    "resolved_domains": dns_result.resolved_domains,
                    "expected_domain": expected_domain,
                    "dns_server": dns_result.dns_server,
                    "lookup_time_ms": dns_result.duration_ms
                })
            )
        } else {
            (
                false,
                format!("Reverse DNS resolved {} to {} but expected {}", ip_to_resolve, resolved_domain, expected_domain),
                serde_json::json!({
                    "ip_address": ip_to_resolve,
                    "resolved_domains": dns_result.resolved_domains,
                    "expected_domain": expected_domain,
                    "dns_server": dns_result.dns_server,
                    "lookup_time_ms": dns_result.duration_ms
                })
            )
        }
    } else {
        let error_msg = if dns_result.resolved_domains.is_empty() {
            "No reverse DNS records found".to_string()
        } else {
            dns_result.error_message.unwrap_or_else(|| "Unknown error".to_string())
        };
        
        (
            false,
            format!("Reverse DNS lookup failed for {}: {}", ip_to_resolve, error_msg),
            serde_json::json!({
                "ip_address": ip_to_resolve,
                "error": error_msg,
                "dns_server": dns_result.dns_server,
                "lookup_time_ms": dns_result.duration_ms
            })
        )
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

/// Execute DNS over HTTPS test - test DoH capability (placeholder implementation)
pub async fn execute_dns_over_https_test(
    config: &DnsOverHttpsConfig,
    timer: &Timer,
    node: &Node,
    _node_service: &NodeService
) -> Result<TestResult> {
    // This is a more complex test that would require implementing DoH protocol
    // For now, return a placeholder indicating it needs implementation
    
    Ok(TestResult {
        success: false,
        message: format!("DNS-over-HTTPS test not yet implemented for domain: {}", config.domain),
        duration_ms: timer.elapsed_ms(),
        executed_at: timer.datetime(),
        details: Some(serde_json::json!({
            "domain": config.domain,
            "expected_ip": config.expected_ip,
            "node_target": node.base.target.to_string(),
            "implementation_status": "TODO: Implement DoH protocol support"
        })),
        criticality: None,
    })
}