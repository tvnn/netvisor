use std::{net::IpAddr, time::Duration};
use anyhow::{anyhow, Error, Result};
use cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr};
use local_ip_address::local_ip;
use tokio::{net::TcpStream, time::timeout};
use get_if_addrs::get_if_addrs;

const PORT_SCAN_TIMEOUT: Duration = Duration::from_millis(2000);

/// Perform concurrent port scan on a specific host
pub async fn port_scan(ip: IpAddr, ports: &Vec<u16>) -> Result<Vec<u16>> {
    tracing::debug!("Port scanning {} on {} ports", ip, ports.len());
    let mut open_ports = Vec::new();
    
    // Use futures to scan ports concurrently
    let mut handles = Vec::new();
    for &port in ports {
        let handle = tokio::spawn(async move {
            match timeout(PORT_SCAN_TIMEOUT, TcpStream::connect((ip, port))).await {
                Ok(Ok(_)) => Some(port),
                _ => None,
            }
        });
        handles.push(handle);
    }

    // Wait for all scans to complete
    for handle in handles {
        if let Ok(Some(port)) = handle.await {
            open_ports.push(port);
        }
    }

    tracing::debug!("Found {} open ports on {}: {:?}", open_ports.len(), ip, open_ports);
    Ok(open_ports)
}

/// Get the local IP address using local-ip-address crate
pub fn get_local_ip_address() -> Result<IpAddr> {
    local_ip().map_err(|e| anyhow!("Failed to get local IP address: {}", e))
}

/// Get daemon subnet using network interface detection
pub fn get_daemon_subnet() -> Result<IpCidr, Error> {
    let local_ip = get_local_ip_address()?;
    let interfaces = get_if_addrs()
        .map_err(|e| anyhow!("Failed to get network interfaces: {}", e))?;
    
    for interface in interfaces {
        if !interface.is_loopback() {
            let interface_ip = interface.ip();
            if interface_ip == local_ip {
                match interface.addr {
                    get_if_addrs::IfAddr::V4(v4_addr) => {
                        let netmask = v4_addr.netmask;
                        let prefix_len = netmask.octets().iter()
                            .map(|&octet| octet.count_ones())
                            .sum::<u32>() as u8;

                        let network = std::net::Ipv4Addr::from(
                            u32::from(v4_addr.ip) & u32::from(netmask)
                        );

                        return Ok(IpCidr::V4(Ipv4Cidr::new(network, prefix_len)?));
                    }
                    get_if_addrs::IfAddr::V6(v6_addr) => {
                        let netmask = v6_addr.netmask;
                        let prefix_len = netmask.octets().iter()
                            .map(|&octet| octet.count_ones())
                            .sum::<u32>() as u8;

                        let ip_bytes = v6_addr.ip.octets();
                        let mask_bytes = netmask.octets();
                        let mut network_bytes = [0u8; 16];
                        for i in 0..16 {
                            network_bytes[i] = ip_bytes[i] & mask_bytes[i];
                        }
                        let network = std::net::Ipv6Addr::from(network_bytes);

                        return Ok(IpCidr::V6(Ipv6Cidr::new(network, prefix_len)?));
                    }
                }
            }
        }
    }

    Err(Error::msg(format!("Could not find subnet for local IP: {}", local_ip)))
}

/// Use existing reverse DNS lookup from utilities
// pub async fn reverse_dns_lookup(ip: IpAddr) -> Result<String, anyhow::Error> {
//     // Create a basic DNS server config using system DNS
//     let dns_server = crate::server::tests::utilities::dns::DnsServerConfig {
//         ip: "8.8.8.8".parse().unwrap(),
//         port: 53,
//         name: "Google DNS".to_string(),
//     };
    
//     let result = DnsUtils::reverse_lookup_ip(&dns_server, ip, Some(5000)).await?;
    
//     if result.success && !result.resolved_domains.is_empty() {
//         Ok(result.resolved_domains[0].clone())
//     } else {
//         Err(anyhow!("Reverse DNS lookup failed or returned no results"))
//     }
// }

pub async fn reverse_dns_lookup(ip: IpAddr) -> Result<String, anyhow::Error> {
    use trust_dns_resolver::TokioAsyncResolver;
    
    // Use system resolver - automatically picks up local DNS settings
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    
    match resolver.reverse_lookup(ip).await {
        Ok(lookup) => {
            if let Some(name) = lookup.iter().next() {
                let hostname = name.to_string();
                // Remove trailing dot if present
                let hostname = hostname.trim_end_matches('.').to_string();
                Ok(hostname)
            } else {
                Err(anyhow!("No PTR records found"))
            }
        },
        Err(e) => Err(anyhow!("Reverse DNS lookup failed: {}", e))
    }
}

/// Get MAC address from ARP table
pub async fn arp_lookup(ip: IpAddr) -> Result<String, anyhow::Error> {
    use tokio::process::Command;
    
    // Try to get MAC address from ARP table
    let output = Command::new("arp")
        .args(&["-n", &ip.to_string()])
        .output()
        .await?;
        
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        // Parse ARP output for MAC address
        for line in output_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[0] == ip.to_string() {
                // Look for MAC address pattern (xx:xx:xx:xx:xx:xx)
                for part in parts {
                    if part.contains(':') && part.len() == 17 {
                        return Ok(part.to_string());
                    }
                }
            }
        }
    }
    
    Err(anyhow!("ARP lookup failed"))
}