use crate::types::*;
use crate::network_checks::{create_http_client};
use serde_json::{json, Value};
use std::net::{ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use std::net::IpAddr;
use tokio::time::timeout;

// VPN connectivity test - checks if VPN tunnel is active and routing correctly
pub async fn vpn_connectivity_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port = config.port.unwrap_or(51820); // Default WireGuard port
    let timeout_ms = config.timeout.unwrap_or(5000);
    
    println!("Testing VPN connectivity to {}:{}", target, port);
    
    // First, test if we can reach the VPN endpoint (UDP for WireGuard)
    let socket_addr = format!("{}:{}", target, port)
        .to_socket_addrs()
        .map_err(|e| format!("Failed to resolve VPN endpoint {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for VPN endpoint {}", target))?;
    
    let start = Instant::now();
    
    // Test TCP connectivity to the WireGuard port (not ideal but better than nothing)
    let connect_future = TcpStream::connect(&socket_addr);
    let connect_result = timeout(Duration::from_millis(timeout_ms), connect_future).await;
    
    let duration = start.elapsed();
    
    match connect_result {
        Ok(Ok(_)) => {
            // TCP connection succeeded - this might indicate the port is open
            // but WireGuard uses UDP, so this is not definitive
            Ok(json!({
                "vpn_endpoint": format!("{}:{}", target, port),
                "tcp_reachable": true,
                "response_time_ms": duration.as_millis(),
                "note": "TCP connection successful, but WireGuard uses UDP"
            }))
        },
        Ok(Err(e)) => {
            // Connection failed - could be firewall, network issue, or server down
            Err(format!("VPN endpoint unreachable: {}", e))
        },
        Err(_) => {
            // Timeout
            Err(format!("VPN endpoint timeout after {}ms", timeout_ms))
        }
    }
}

// VPN tunnel validation - checks if we're actually routing through the VPN
pub async fn vpn_tunnel_check(config: &CheckConfig) -> Result<Value, String> {
    let expected_subnet = config.target.as_ref().ok_or("Expected VPN subnet required (e.g., 10.100.0.0/24)")?;
    let timeout_ms = config.timeout;
    
    println!("Testing if device is on VPN subnet: {}", expected_subnet);
    
    let client = create_http_client(timeout_ms)?;
    
    // Use multiple IP detection services to check current public IP
    let ip_services = vec![
        "https://api.ipify.org?format=json",
        "https://httpbin.org/ip",
        "https://icanhazip.com",
    ];
    
    let mut detected_ip = None;
    
    for service_url in &ip_services {
        match client.get(*service_url).send().await {
            Ok(response) => {
                if let Ok(text) = response.text().await {
                    // Try to extract IP from different response formats
                    if let Ok(json_resp) = serde_json::from_str::<Value>(&text) {
                        if let Some(ip) = json_resp["ip"].as_str().or_else(|| json_resp["origin"].as_str()) {
                            detected_ip = Some(ip.to_string());
                            break;
                        }
                    } else {
                        // Plain text response (like icanhazip.com)
                        let ip = text.trim();
                        if ip.parse::<IpAddr>().is_ok() {
                            detected_ip = Some(ip.to_string());
                            break;
                        }
                    }
                }
            },
            Err(_) => continue,
        }
    }
    
    let detected_ip = detected_ip.ok_or("Could not detect public IP address")?;
    
    // Parse the expected subnet
    let subnet_parts: Vec<&str> = expected_subnet.split('/').collect();
    if subnet_parts.len() != 2 {
        return Err("Invalid subnet format. Expected format: 10.100.0.0/24".to_string());
    }
    
    let network_addr = subnet_parts[0].parse::<IpAddr>()
        .map_err(|_| "Invalid network address in subnet")?;
    let prefix_len = subnet_parts[1].parse::<u8>()
        .map_err(|_| "Invalid prefix length in subnet")?;
    
    // Check if detected IP is in the expected VPN subnet
    let detected_ip_addr = detected_ip.parse::<IpAddr>()
        .map_err(|_| "Invalid detected IP address")?;
    
    let is_in_vpn_subnet = match (network_addr, detected_ip_addr) {
        (IpAddr::V4(net), IpAddr::V4(ip)) => {
            let net_u32 = u32::from(net);
            let ip_u32 = u32::from(ip);
            let mask = !((1u32 << (32 - prefix_len)) - 1);
            (net_u32 & mask) == (ip_u32 & mask)
        },
        _ => false, // IPv6 or mixed - simplified for now
    };
    
    if is_in_vpn_subnet {
        Ok(json!({
            "detected_public_ip": detected_ip,
            "expected_subnet": expected_subnet,
            "is_vpn_active": true,
            "tunnel_status": "Active - routing through VPN"
        }))
    } else {
        Err(format!("Not routing through VPN. Public IP {} is not in expected subnet {}", detected_ip, expected_subnet))
    }
}