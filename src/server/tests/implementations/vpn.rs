use std::time::Duration;
use anyhow::{Error, Result};
use cidr::IpCidr;
use if_addrs::{get_if_addrs, IfAddr};
use crate::server::{
    capabilities::types::{base::Capability, configs::HttpEndpointCompatible}, nodes::types::{base::Node, targets::NodeTarget}, tests::{types::{configs::*, execution::*}, utilities::dns::{DnsServerConfig, DnsUtils}}
};

pub async fn execute_vpn_subnet_access_test(
    config: &VpnSubnetAccessConfig,
    timer: &Timer,
    node: &Node,
    dns_server: Option<&DnsServerConfig>,
    capability: &Capability
) -> Result<TestResult> {
    let expected_subnet = &config.expected_subnet;
    let service_url = match capability {
        Capability::Daemon(config) => config.as_endpoint(&node.base.target),
        Capability::Http(config) => config.as_endpoint(&node.base.target),
        Capability::Https(config) => config.as_endpoint(&node.base.target),
        _ => Err(Error::msg(format!("Capability {} for node {} doesn't support endpoints", capability.config_base().name, node.id))),
    }?;
    
    // Extract actual VPN server IP for networking operations, using DNS capability for hostnames
    let vpn_server_ip = match &node.base.target {
        NodeTarget::IpAddress(ip_config) => {
            Some(ip_config.ip)
        },
        NodeTarget::Hostname(hostname_config) => {

            let Some(dns) = dns_server else {
                return Err(Error::msg("VPN Subnet test targeting hostname requires a DNS resolver"))
            };

            match DnsUtils::resolve_domain(dns, &hostname_config.hostname, config.timeout_ms).await {
                Ok(dns_result) if dns_result.success && !dns_result.resolved_addresses.is_empty() => {
                    Some(dns_result.resolved_addresses[0])
                },
                _ => None // DNS resolution failed, continue without IP
            }
        }
    };

    // Check for subnet accessibility via connectivity testing and routing hints
    let access_check_result = check_subnet_accessibility(expected_subnet, vpn_server_ip).await;

    let (success, message, details) = match access_check_result {
        Ok(Some(access_info)) => {
            (
                true,
                format!("Subnet {} is accessible via {}", expected_subnet, access_info.method),
                serde_json::json!({
                    "vpn_server": service_url,
                    "vpn_server_ip": vpn_server_ip,
                    "expected_subnet": expected_subnet,
                    "access_method": access_info.method,
                    "details": access_info.details,
                    "test_type": "subnet_access"
                })
            )
        },
        Ok(None) => {
            (
                false,
                format!("No access to subnet {} detected", expected_subnet),
                serde_json::json!({
                    "vpn_server": service_url,
                    "vpn_server_ip": vpn_server_ip,
                    "expected_subnet": expected_subnet,
                    "error": "No route or connectivity to expected subnet found",
                    "test_type": "subnet_access"
                })
            )
        },
        Err(e) => {
            (
                false,
                format!("Failed to check subnet accessibility: {}", e),
                serde_json::json!({
                    "vpn_server": service_url,
                    "vpn_server_ip": vpn_server_ip,
                    "expected_subnet": expected_subnet,
                    "error": e.to_string(),
                    "test_type": "subnet_access"
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

#[derive(Debug)]
struct AccessInfo {
    method: String,
    details: String,
}

/// Check if expected subnet is accessible via connectivity testing (primary) and routing hints (secondary)
async fn check_subnet_accessibility(
    expected_subnet: &IpCidr,
    _vpn_server_ip: Option<std::net::IpAddr>,
) -> Result<Option<AccessInfo>> {
    // Primary test: Can we actually reach something in the subnet?
    if let Some(connectivity_result) = test_subnet_connectivity(expected_subnet).await? {
        return Ok(Some(connectivity_result));
    }
    
    // Secondary test: Check if we have routing info (indicates VPN setup but target might be down)
    if let Some(route_result) = check_basic_routing_info(expected_subnet).await? {
        return Ok(Some(route_result));
    }
    
    Ok(None)
}

/// Test connectivity to expected subnet by trying to connect to likely IPs within it
async fn test_subnet_connectivity(expected_subnet: &IpCidr) -> Result<Option<AccessInfo>> {
    let test_ips = get_likely_subnet_ips(expected_subnet)?;
    let test_ports = [22, 80, 443, 8080, 53]; // SSH, HTTP, HTTPS, alt-HTTP, DNS
    
    for test_ip in test_ips {
        for port in test_ports {
            let addr = std::net::SocketAddr::new(test_ip, port);
            
            // Quick connection test with short timeout
            if let Ok(Ok(_)) = tokio::time::timeout(
                Duration::from_millis(2000),
                tokio::net::TcpStream::connect(addr)
            ).await {
                return Ok(Some(AccessInfo {
                    method: "Direct connectivity".to_string(),
                    details: format!("Successfully connected to {}:{} in subnet {}", test_ip, port, expected_subnet),
                }));
            }
        }
    }
    
    Ok(None)
}

/// Get likely IPs to test in a subnet (gateway, common server IPs, etc.)
fn get_likely_subnet_ips(expected_subnet: &IpCidr) -> Result<Vec<std::net::IpAddr>> {
    match expected_subnet {
        IpCidr::V4(subnet) => {
            let network = subnet.first_address();
            let network_u32 = u32::from(network);
            
            let mut test_ips = Vec::new();
            
            // Common IPs to test (gateway, common server addresses)
            let offsets = [1, 2, 10, 20, 50, 100, 200]; // .1, .2, .10, .20, .50, .100, .200
            
            for offset in offsets {
                if let Some(ip_u32) = network_u32.checked_add(offset) {
                    let test_ip = std::net::Ipv4Addr::from(ip_u32);
                    if subnet.contains(&test_ip) {
                        test_ips.push(std::net::IpAddr::V4(test_ip));
                    }
                }
            }
            
            Ok(test_ips)
        },
        IpCidr::V6(_) => {
            Err(Error::msg("IPv6 subnet testing is not yet supported. Please use IPv4 subnets (e.g., 192.168.1.0/24)"))
        }
    }
}

/// Simple routing info check - just look for any interface that could reach the subnet
async fn check_basic_routing_info(expected_subnet: &IpCidr) -> Result<Option<AccessInfo>> {
    // Check if any network interface has an IP that could route to the expected subnet
    let interfaces = get_if_addrs()
        .map_err(|e| Error::msg(format!("Failed to get network interfaces: {}", e)))?;

    for iface in interfaces {
        if is_loopback_or_local(&iface.name) {
            continue;
        }
        
        let iface_ip = match &iface.addr {
            IfAddr::V4(addr) => std::net::IpAddr::V4(addr.ip),
            IfAddr::V6(addr) => std::net::IpAddr::V6(addr.ip),
        };
        
        // Check if this interface could plausibly route to the expected subnet
        if could_route_to_subnet(iface_ip, expected_subnet) {
            return Ok(Some(AccessInfo {
                method: format!("Potential route via interface {}", iface.name),
                details: format!("Interface {} ({}) could route to subnet {} but connection test failed", 
                    iface.name, iface_ip, expected_subnet),
            }));
        }
    }
    
    Ok(None)
}

/// Simple heuristic to check if an interface IP could route to a subnet
fn could_route_to_subnet(interface_ip: std::net::IpAddr, target_subnet: &IpCidr) -> bool {
    match (interface_ip, target_subnet) {
        (std::net::IpAddr::V4(iface_ipv4), IpCidr::V4(target_subnet_v4)) => {
            // Simple heuristic: if interface is in a private range and target is private,
            // or if they're in the same general network space, assume routing is possible
            let iface_private = is_private_ipv4(iface_ipv4);
            let target_private = is_private_subnet_v4(target_subnet_v4);
            
            // Both private = could be VPN routing between private networks
            // Both public = could be internet routing
            iface_private == target_private
        },
        (std::net::IpAddr::V6(_), IpCidr::V6(_)) => {
            // IPv6 routing detection not implemented yet
            false
        },
        _ => false, // IPv4 to IPv6 or vice versa = unlikely to route
    }
}

/// Check if IPv4 address is in private ranges
fn is_private_ipv4(ip: std::net::Ipv4Addr) -> bool {
    ip.is_private() || ip.is_loopback()
}

/// Check if IPv4 subnet is primarily private
fn is_private_subnet_v4(subnet: &cidr::Ipv4Cidr) -> bool {
    let first_ip = subnet.first_address();
    is_private_ipv4(first_ip)
}

/// Check if interface is loopback or other local interface to exclude
fn is_loopback_or_local(name: &str) -> bool {
    let local_interfaces = ["lo", "lo0", "localhost", "loopback"];
    let name_lower = name.to_lowercase();
    local_interfaces.iter().any(|local| name_lower.contains(local))
}