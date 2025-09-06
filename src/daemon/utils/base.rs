use std::net::{IpAddr};
use std::time::Duration;
use anyhow::anyhow;
use anyhow::Error;
use async_trait::async_trait;
use local_ip_address::local_ip;
use mac_address::MacAddress;
use pnet::datalink::NetworkInterface;
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::timeout;

const PORT_SCAN_TIMEOUT: Duration = Duration::from_millis(2000);

/// Cross-platform system utilities trait
#[async_trait]
pub trait SystemUtils {
    /// Get MAC address for an IP from ARP table
    async fn get_mac_address_for_ip(&self, ip: IpAddr) -> Result<Option<MacAddress>, Error>;
    
    fn get_own_mac_address(&self) -> Result<Option<MacAddress>, Error> {
        mac_address::get_mac_address().map_err(|e| anyhow!("Failed to get own MAC address: {}", e))
    }

    fn get_own_hostname(&self) -> Option<String> {
        hostname::get()
            .ok()
            .map(|os_str| os_str.to_string_lossy().into_owned())
    }

    fn get_own_interfaces(&self) -> Vec<NetworkInterface> {
        pnet::datalink::interfaces()
    }

    fn get_own_ip_address(&self) -> Result<IpAddr, Error> {
        local_ip().map_err(|e| anyhow!("Failed to get local IP address: {}", e))
    }
    
    async fn get_hostname_for_ip(&self, ip: IpAddr) -> Result<Option<String>, Error> {
        match dns_lookup::lookup_addr(&ip) {
            Ok(hostname) => Ok(Some(hostname)),
            Err(_) => Ok(None),
        }
    }

    async fn scan_own_tcp_ports(&self) -> Result<Vec<Port>, Error>;

    async fn scan_own_udp_ports(&self) -> Result<Vec<Port>, Error>;

    async fn scan_ports(&self, ip: IpAddr) -> Result<Vec<Port>, Error> {
        let (tcp_result, udp_result) = tokio::join!(
            self.scan_tcp_ports(ip),
            self.scan_udp_ports(ip)
        );

        if tcp_result.is_err() && udp_result.is_err() {
            return Err(anyhow::anyhow!(
                "Both port scans failed - TCP: {}, UDP: {}", 
                tcp_result.unwrap_err(), 
                udp_result.unwrap_err()
            ));
        }

        let mut all_ports = Vec::new();
        
        // Handle TCP results
        match tcp_result {
            Ok(tcp_ports) => all_ports.extend(tcp_ports),
            Err(e) => tracing::warn!("TCP port scan failed for {}: {}", ip, e)
        }
        
        // Handle UDP results  
        match udp_result {
            Ok(udp_ports) => all_ports.extend(udp_ports),
            Err(e) => tracing::warn!("UDP port scan failed for {}: {}", ip, e)
        }
        
        Ok(all_ports)
    }

    async fn scan_tcp_ports(&self, ip: IpAddr) -> Result<Vec<Port>, Error> {
        let discovery_ports = Service::discovery_ports(ip);
        let ports: Vec<u16> = discovery_ports.iter().filter_map(|p| p.tcp.then(|| p.number)).collect();
        tracing::debug!("Port scanning {} on {} ports", ip, ports.len());
        let mut open_ports = Vec::new();
        
        // Use futures to scan ports concurrently
        let mut handles = Vec::new();
        for port in ports {
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
                open_ports.push(Port::new_tcp(port));
            }
        }

        tracing::debug!("Found {} open ports on {}: {:?}", open_ports.len(), ip, open_ports);
        Ok(open_ports)
    }

    async fn scan_udp_ports(&self, ip: IpAddr) -> Result<Vec<Port>, Error> {
        let discovery_ports = Service::discovery_ports(ip);
        let ports: Vec<u16> = discovery_ports.iter().filter_map(|p| p.udp.then(|| p.number)).collect();
        let mut open_ports = Vec::new();
        
        // Use futures to scan ports concurrently
        let mut handles = Vec::new();
        for port in ports {
            let handle = tokio::spawn(async move {
                match timeout(PORT_SCAN_TIMEOUT, UdpSocket::bind("0.0.0.0:0")).await {
                    Ok(Ok(socket)) => {
                        match timeout(PORT_SCAN_TIMEOUT, socket.connect((ip, port))).await {
                            Ok(Ok(_)) => {
                                // Try to send a small probe packet
                                match timeout(PORT_SCAN_TIMEOUT, socket.send(&[0u8; 1])).await {
                                    Ok(Ok(_)) => Some(port),
                                    _ => None,
                                }
                            },
                            _ => None,
                        }
                    },
                    _ => None,
                }
            });
            handles.push(handle);
        }

        // Wait for all scans to complete
        for handle in handles {
            if let Ok(Some(port)) = handle.await {
                open_ports.push(Port::new_udp(port));
            }
        }

        tracing::debug!("Found {} open UDP ports on {}: {:?}", open_ports.len(), ip, open_ports);
        Ok(open_ports)
    }

    async fn scan_endpoints(&self, endpoints: Vec<Endpoint>) -> Result<Vec<EndpointResponse>, Error> {
        tracing::debug!("Scanning {} endpoints", endpoints.len());
        let mut responses = Vec::new();
        
        // Use futures to scan endpoints concurrently
        let mut handles = Vec::new();
        for endpoint in endpoints {
            let handle = tokio::spawn(async move {
                let client = reqwest::Client::new();
                let url = endpoint.to_string();
                
                match timeout(PORT_SCAN_TIMEOUT, client.get(&url).send()).await {
                    Ok(Ok(response)) => {
                        match response.text().await {
                            Ok(text) => Some(EndpointResponse {
                                endpoint,
                                response: text,
                            }),
                            Err(_) => None,
                        }
                    },
                    _ => None,
                }
            });
            handles.push(handle);
        }

        // Wait for all requests to complete
        for handle in handles {
            if let Ok(Some(endpoint_response)) = handle.await {
                responses.push(endpoint_response);
            }
        }

        tracing::debug!("Received {} endpoint responses", responses.len());
        Ok(responses)
    }
}

#[cfg(target_os = "linux")]
pub type PlatformSystemUtils = LinuxSystemUtils;

#[cfg(target_os = "macos")] 
use crate::daemon::utils::macos::MacOsSystemUtils;
use crate::server::services::types::base::Service;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::ports::Port;
pub type PlatformSystemUtils = MacOsSystemUtils;

#[cfg(target_family = "windows")]
pub type PlatformSystemUtils = WindowsSystemUtils;

pub fn create_system_utils() -> PlatformSystemUtils {
    PlatformSystemUtils::new()
}
