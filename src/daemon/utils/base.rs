use std::net::{IpAddr};
use std::time::Duration;
use anyhow::anyhow;
use anyhow::Error;
use async_trait::async_trait;
use local_ip_address::local_ip;
use mac_address::MacAddress;
use tokio::net::TcpStream;
use tokio::time::timeout;

const PORT_SCAN_TIMEOUT: Duration = Duration::from_millis(2000);

/// Cross-platform system utilities trait
#[async_trait]
pub trait SystemUtils {
    /// Get MAC address for an IP from ARP table
    async fn get_mac_address_for_ip(&self, ip: IpAddr) -> Result<Option<MacAddress>, Error>;
    
    /// Get list of TCP ports that are being listened on locally
    async fn scan_own_tcp_ports(&self) -> Result<Vec<u16>, Error>;
    
    /// Get list of UDP ports that are being listened on locally  
    async fn scan_own_udp_ports(&self) -> Result<Vec<u16>, Error>;

    fn get_own_hostname(&self) -> Option<String> {
        hostname::get()
            .ok()
            .map(|os_str| os_str.to_string_lossy().into_owned())
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

    async fn scan_tcp_ports(&self, ip: IpAddr) -> Result<Vec<u16>, Error> {
        use crate::server::capabilities::types::base::Capability;
        let ports: Vec<u16> = Capability::discovery_ports();
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
                open_ports.push(port);
            }
        }

        tracing::debug!("Found {} open ports on {}: {:?}", open_ports.len(), ip, open_ports);
        Ok(open_ports)
    }
}

#[cfg(target_os = "linux")]
pub type PlatformSystemUtils = LinuxSystemUtils;

#[cfg(target_os = "macos")] 
use crate::daemon::utils::macos::MacOsSystemUtils;
pub type PlatformSystemUtils = MacOsSystemUtils;

#[cfg(target_family = "windows")]
pub type PlatformSystemUtils = WindowsSystemUtils;

pub fn create_system_utils() -> PlatformSystemUtils {
    PlatformSystemUtils::new()
}
