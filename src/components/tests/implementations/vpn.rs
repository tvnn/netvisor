use std::time::Duration;
use anyhow::{Error, Result};
use tokio::net::TcpStream;
use tokio::time::timeout;
use cidr::IpCidr;
use std::process::Command;
use crate::components::{
    nodes::types::base::{Node},
    tests::types::configs::*,
    tests::types::execution::*
};

/// Execute VPN connectivity test - test basic connection to VPN server
pub async fn execute_vpn_connectivity_test(
    config: &VpnConnectivityConfig,
    timer: &Timer,
    node: &Node,
) -> Result<TestResult> {
    let timeout_duration = Duration::from_millis(config.timeout_ms.unwrap_or(30000) as u64);
    
    // Extract VPN server target from node configuration
    let vpn_target = &node.base.target.to_string();

    // Test basic connectivity to VPN server port
    let connection_result = timeout(timeout_duration, TcpStream::connect(&vpn_target)).await;

    let (success, message, details) = match connection_result {
        Ok(Ok(_stream)) => {
            (
                true,
                format!("VPN server {} is reachable", vpn_target),
                serde_json::json!({
                    "vpn_server": vpn_target,
                    "connection_time_ms": timer.elapsed_ms(),
                    "test_type": "basic_connectivity"
                })
            )
        },
        Ok(Err(e)) => {
            (
                false,
                format!("Failed to reach VPN server {}: {}", vpn_target, e),
                serde_json::json!({
                    "vpn_server": vpn_target,
                    "error": e.to_string(),
                    "test_type": "basic_connectivity"
                })
            )
        },
        Err(_) => {
            (
                false,
                format!("VPN server {} connection timed out after {}ms", vpn_target, timeout_duration.as_millis()),
                serde_json::json!({
                    "vpn_server": vpn_target,
                    "timeout_ms": timeout_duration.as_millis(),
                    "test_type": "basic_connectivity"
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

/// Execute VPN tunnel test - test VPN tunnel functionality and subnet access
pub async fn execute_vpn_tunnel_test(
    config: &VpnTunnelConfig,
    timer: &Timer,
    node: &Node,
) -> Result<TestResult> {
    let _timeout_duration = Duration::from_millis(config.timeout_ms.unwrap_or(30000) as u64);
    let expected_subnet = &config.expected_subnet;
    
    // Get VPN server information
    let vpn_server = &node.base.target.to_string();

    // Check if we can detect VPN tunnel interface
    let tunnel_check_result = check_vpn_tunnel_interface().await;

    let (success, message, details) = match tunnel_check_result {
        Ok(Some(tunnel_info)) => {
            // We found a VPN tunnel, check if it matches expected subnet
            if subnet_matches_expected(&tunnel_info.subnet, expected_subnet) {
                (
                    true,
                    format!("VPN tunnel active with expected subnet {}", expected_subnet),
                    serde_json::json!({
                        "vpn_server": vpn_server,
                        "tunnel_interface": tunnel_info.interface_name,
                        "tunnel_subnet": tunnel_info.subnet,
                        "expected_subnet": expected_subnet,
                        "test_type": "tunnel_validation"
                    })
                )
            } else {
                (
                    false,
                    format!("VPN tunnel active but subnet {} doesn't match expected {}", tunnel_info.subnet, expected_subnet),
                    serde_json::json!({
                        "vpn_server": vpn_server,
                        "tunnel_interface": tunnel_info.interface_name,
                        "tunnel_subnet": tunnel_info.subnet,
                        "expected_subnet": expected_subnet,
                        "test_type": "tunnel_validation"
                    })
                )
            }
        },
        Ok(None) => {
            (
                false,
                format!("No VPN tunnel detected for server {}", vpn_server),
                serde_json::json!({
                    "vpn_server": vpn_server,
                    "expected_subnet": expected_subnet,
                    "error": "No active VPN tunnel found",
                    "test_type": "tunnel_validation"
                })
            )
        },
        Err(e) => {
            (
                false,
                format!("Failed to check VPN tunnel status: {}", e),
                serde_json::json!({
                    "vpn_server": vpn_server,
                    "expected_subnet": expected_subnet,
                    "error": e.to_string(),
                    "test_type": "tunnel_validation"
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

// Helper struct for tunnel information
#[derive(Debug)]
struct TunnelInfo {
    interface_name: String,
    subnet: String,
}

/// Check for VPN tunnel interfaces (platform-specific implementation)
async fn check_vpn_tunnel_interface() -> Result<Option<TunnelInfo>> {
    // This is a simplified implementation
    // In practice, you'd check for VPN interfaces like tun0, wg0, etc.
    
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("ip")
            .args(&["route", "show"])
            .output()
            .map_err(|e| Error::msg(format!("Failed to execute ip command: {}", e)))?;

        let route_output = String::from_utf8_lossy(&output.stdout);
        
        // Look for VPN-like routes (this is a simplified heuristic)
        for line in route_output.lines() {
            if line.contains("tun") || line.contains("wg") || line.contains("ppp") {
                // Extract interface and subnet information
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let subnet = parts[0];
                    let interface = parts.last().unwrap_or("unknown");
                    
                    return Ok(Some(TunnelInfo {
                        interface_name: interface.to_string(),
                        subnet: subnet.to_string(),
                    }));
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows implementation would use netsh or PowerShell
        let output = Command::new("netsh")
            .args(&["interface", "show", "interface"])
            .output()
            .map_err(|e| Error::msg(format!("Failed to execute netsh command: {}", e)))?;

        let interface_output = String::from_utf8_lossy(&output.stdout);
        
        // Look for VPN interfaces
        for line in interface_output.lines() {
            if line.to_lowercase().contains("vpn") || line.to_lowercase().contains("tunnel") {
                return Ok(Some(TunnelInfo {
                    interface_name: "VPN Connection".to_string(),
                    subnet: "10.0.0.0/24".to_string(), // Placeholder
                }));
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS implementation would use route or netstat
        let output = Command::new("route")
            .args(&["-n", "get", "default"])
            .output()
            .map_err(|e| Error::msg(format!("Failed to execute route command: {}", e)))?;

        let route_output = String::from_utf8_lossy(&output.stdout);
        
        // Check for VPN interfaces
        if route_output.contains("utun") || route_output.contains("ppp") {
            return Ok(Some(TunnelInfo {
                interface_name: "utun0".to_string(),
                subnet: "10.0.0.0/24".to_string(), // Placeholder
            }));
        }
    }

    // No VPN tunnel detected
    Ok(None)
}

/// Check if detected subnet matches expected subnet
fn subnet_matches_expected(detected: &str, expected: &IpCidr) -> bool {
    // Parse detected subnet and compare with expected
    if let Ok(detected_cidr) = detected.parse::<IpCidr>() {
        // For now, simple equality check
        // In practice, you might want more sophisticated matching
        detected_cidr == *expected
    } else {
        false
    }
}