use serde_json::{json, Value};
use std::net::{ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::net::Ipv4Addr;
use crate::components::tests::checks::CheckConfig;

// Local network gateway test - tests basic local network connectivity
pub async fn local_gateway_check(config: &CheckConfig) -> Result<Value, String> {
    let timeout_ms = config.timeout.unwrap_or(3000);
    
    println!("Testing local network gateway connectivity");
    
    // Get default gateway (this is platform-specific, simplified approach)
    let gateway_ips = vec![
        "192.168.1.1",   // Common home router
        "192.168.0.1",   // Common home router
        "10.0.0.1",      // Common home router
        "172.16.0.1",    // Less common but possible
    ];
    
    let mut successful_gateways = Vec::new();
    let mut failed_gateways = Vec::new();
    
    for gateway_ip in &gateway_ips {
        let socket_addr = format!("{}:80", gateway_ip); // Try HTTP port
        
        match socket_addr.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    let start = Instant::now();
                    let connect_future = TcpStream::connect(&addr);
                    
                    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
                        Ok(Ok(_)) => {
                            let duration = start.elapsed();
                            successful_gateways.push(json!({
                                "gateway_ip": gateway_ip,
                                "port": 80,
                                "response_time_ms": duration.as_millis(),
                                "status": "reachable"
                            }));
                        },
                        Ok(Err(e)) => {
                            failed_gateways.push(json!({
                                "gateway_ip": gateway_ip,
                                "port": 80,
                                "error": e.to_string(),
                                "status": "unreachable"
                            }));
                        },
                        Err(_) => {
                            failed_gateways.push(json!({
                                "gateway_ip": gateway_ip,
                                "port": 80,
                                "error": "timeout",
                                "status": "timeout"
                            }));
                        }
                    }
                }
            },
            Err(_) => continue,
        }
    }
    
    if !successful_gateways.is_empty() {
        Ok(json!({
            "local_network_status": "reachable",
            "reachable_gateways": successful_gateways,
            "failed_gateways": failed_gateways,
            "total_tested": gateway_ips.len()
        }))
    } else {
        Err("No local network gateways reachable - local network may be down".to_string())
    }
}

pub async fn dhcp_discovery_check(config: &CheckConfig) -> Result<Value, String> {
    let timeout_ms = config.timeout.unwrap_or(5000);
    let interface = config.interface.as_deref().unwrap_or("auto");
    
    println!("Discovering DHCP servers on network interface: {}", interface);
    
    // This is a simplified DHCP discovery - a full implementation would
    // require sending actual DHCP DISCOVER packets
    
    // Check common DHCP server ports on likely gateway IPs
    let potential_dhcp_servers = vec![
        "192.168.1.1:67",
        "192.168.0.1:67", 
        "10.0.0.1:67",
        "172.16.0.1:67",
        "192.168.1.254:67",
    ];
    
    let mut discovered_servers = Vec::new();
    let mut failed_checks = Vec::new();
    
    for server_addr in &potential_dhcp_servers {
        let start = Instant::now();
        
        match server_addr.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    // Try UDP connection to DHCP port
                    match tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                        Ok(socket) => {
                            match timeout(Duration::from_millis(timeout_ms), socket.connect(addr)).await {
                                Ok(Ok(_)) => {
                                    let duration = start.elapsed();
                                    discovered_servers.push(json!({
                                        "server_ip": server_addr,
                                        "response_time_ms": duration.as_millis(),
                                        "port": 67,
                                        "status": "likely_dhcp_server"
                                    }));
                                },
                                _ => {
                                    failed_checks.push(json!({
                                        "server_ip": server_addr,
                                        "error": "connection_failed",
                                        "status": "unreachable"
                                    }));
                                }
                            }
                        },
                        Err(e) => {
                            failed_checks.push(json!({
                                "server_ip": server_addr,
                                "error": format!("UDP socket error: {}", e),
                                "status": "error"
                            }));
                        }
                    }
                }
            },
            Err(_) => continue,
        }
    }
    
    Ok(json!({
        "interface": interface,
        "discovered_dhcp_servers": discovered_servers,
        "failed_checks": failed_checks,
        "total_checked": potential_dhcp_servers.len(),
        "discovery_method": "port_probe"
    }))
}

pub async fn subnet_scan_check(config: &CheckConfig) -> Result<Value, String> {
    let subnet = config.subnet.as_ref().ok_or("Subnet is required (e.g., 192.168.1.0/24)")?;
    let port = config.port.unwrap_or(80);
    let concurrent_scans = config.concurrent_scans.unwrap_or(50).min(100); // Limit concurrency
    let timeout_ms = config.timeout.unwrap_or(3000);
    
    println!("Scanning subnet {} on port {}", subnet, port);
    
    // Parse subnet CIDR notation
    let parts: Vec<&str> = subnet.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid subnet format. Use CIDR notation (e.g., 192.168.1.0/24)".to_string());
    }
    
    let base_ip = parts[0].parse::<Ipv4Addr>()
        .map_err(|_| "Invalid IP address in subnet")?;
    let prefix_len: u8 = parts[1].parse()
        .map_err(|_| "Invalid subnet prefix length")?;
    
    if prefix_len > 30 {
        return Err("Subnet too small (prefix length > 30)".to_string());
    }
    
    // Generate IP range
    let host_bits = 32 - prefix_len;
    let num_hosts = (1u32 << host_bits) - 2; // Exclude network and broadcast
    let base_ip_u32 = u32::from(base_ip);
    
    let mut scan_futures = Vec::new();
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrent_scans as usize));
    
    for i in 1..=num_hosts.min(254) { // Limit to reasonable range
        let ip_u32 = base_ip_u32 + i;
        let ip = Ipv4Addr::from(ip_u32);
        let addr = format!("{}:{}", ip, port);
        let sem = semaphore.clone();
        
        let future = async move {
            let _permit = sem.acquire().await.unwrap();
            let start = Instant::now();
            
            match addr.to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(socket_addr) = addrs.next() {
                        match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(socket_addr)).await {
                            Ok(Ok(_)) => {
                                let duration = start.elapsed();
                                Some(json!({
                                    "ip": ip.to_string(),
                                    "port": port,
                                    "response_time_ms": duration.as_millis(),
                                    "status": "reachable"
                                }))
                            },
                            _ => None,
                        }
                    } else {
                        None
                    }
                },
                Err(_) => None,
            }
        };
        
        scan_futures.push(future);
    }
    
    let start_time = Instant::now();
    let results: Vec<_> = futures::future::join_all(scan_futures).await;
    let scan_duration = start_time.elapsed();
    
    let active_hosts: Vec<_> = results.into_iter().filter_map(|r| r).collect();
    
    Ok(json!({
        "subnet": subnet,
        "port": port,
        "scan_duration_ms": scan_duration.as_millis(),
        "active_hosts": active_hosts,
        "active_count": active_hosts.len(),
        "total_scanned": num_hosts.min(254),
        "concurrent_scans": concurrent_scans
    }))
}