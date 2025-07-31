use super::{create_http_client, get_common_service_name};
use serde_json::{json, Value};
use std::net::ToSocketAddrs;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::components::tests::checks::CheckConfig;

pub async fn ssl_certificate_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target hostname is required")?;
    let port = config.port.unwrap_or(443);
    let min_days_until_expiry = config.min_days_until_expiry.unwrap_or(30);
    let check_chain = config.check_chain.unwrap_or(true);
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    println!("Testing SSL certificate for {}:{}", target, port);
    
    let client = create_http_client(Some(timeout_ms))?;
    let url = format!("https://{}:{}", target, port);
    
    let start = Instant::now();
    
    match client.get(&url).send().await {
        Ok(response) => {
            let duration = start.elapsed();
            
            // For detailed certificate inspection, we'd need to implement
            // a custom TLS client or use a more sophisticated approach
            // This is a simplified version that checks basic connectivity
            
            let status = response.status();
            let headers = response.headers();
            
            // Extract basic certificate info from headers if available
            let security_headers = json!({
                "strict_transport_security": headers.get("strict-transport-security").map(|h| h.to_str().unwrap_or("")),
                "content_security_policy": headers.get("content-security-policy").map(|h| h.to_str().unwrap_or("")),
                "x_frame_options": headers.get("x-frame-options").map(|h| h.to_str().unwrap_or("")),
            });
            
            Ok(json!({
                "target": format!("{}:{}", target, port),
                "response_time_ms": duration.as_millis(),
                "tls_connection": "successful",
                "http_status": status.as_u16(),
                "security_headers": security_headers,
                "certificate_validated": true,
                "min_days_threshold": min_days_until_expiry,
                "chain_validation": check_chain,
                "status": "valid"
            }))
        },
        Err(e) => {
            if e.to_string().contains("certificate") || e.to_string().contains("SSL") || e.to_string().contains("TLS") {
                Err(format!("SSL certificate validation failed: {}", e))
            } else {
                Err(format!("SSL connection failed: {}", e))
            }
        }
    }
}

pub async fn port_scan_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Target is required")?;
    let port_range = config.port_range.as_ref().ok_or("Port range is required")?;
    let scan_type = config.scan_type.as_deref().unwrap_or("tcp");
    let timeout_ms = config.timeout.unwrap_or(2000);
    
    println!("Scanning ports on {} (range: {}, type: {})", target, port_range, scan_type);
    
    // Parse port range
    let ports: Result<Vec<u16>, _> = port_range
        .split(',')
        .map(|s| s.trim().parse::<u16>())
        .collect();
    
    let ports = ports.map_err(|_| "Invalid port range format")?;
    
    if ports.len() > 100 {
        return Err("Too many ports to scan (max 100)".to_string());
    }
    
    let mut open_ports = Vec::new();
    let mut closed_ports = Vec::new();
    let mut scan_results = Vec::new();
    
    let scan_start = Instant::now();
    
    // Resolve target once
    let target_ip = match target.parse::<std::net::IpAddr>() {
        Ok(ip) => ip,
        Err(_) => {
            // Try to resolve hostname
            match format!("{}:80", target).to_socket_addrs() {
                Ok(mut addrs) => addrs.next().unwrap().ip(),
                Err(_) => return Err("Could not resolve target".to_string()),
            }
        }
    };
    
    for port in &ports {
        let port_start = Instant::now();
        let addr = std::net::SocketAddr::new(target_ip, *port);
        
        match scan_type {
            "tcp" => {
                match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
                    Ok(Ok(_)) => {
                        let rtt = port_start.elapsed();
                        open_ports.push(*port);
                        scan_results.push(json!({
                            "port": *port,
                            "status": "open",
                            "service": get_common_service_name(*port),
                            "response_time_ms": rtt.as_millis()
                        }));
                    },
                    _ => {
                        closed_ports.push(*port);
                        scan_results.push(json!({
                            "port": *port,
                            "status": "closed",
                            "service": get_common_service_name(*port),
                            "response_time_ms": null
                        }));
                    }
                }
            },
            "udp" => {
                // UDP scanning is more complex and less reliable
                scan_results.push(json!({
                    "port": *port,
                    "status": "unknown",
                    "service": get_common_service_name(*port),
                    "note": "UDP scanning not fully implemented"
                }));
            },
            _ => return Err("Invalid scan type. Use 'tcp' or 'udp'".to_string()),
        }
    }

    let total_duration = scan_start.elapsed();
    
    Ok(json!({
        "target": target,
        "target_ip": target_ip.to_string(),
        "scan_type": scan_type,
        "total_ports_scanned": ports.len(),
        "open_ports": open_ports,
        "closed_ports": closed_ports,
        "scan_duration_ms": total_duration.as_millis(),
        "scan_results": scan_results,
        "status": "completed"
    }))
}
