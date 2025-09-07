use std::io::ErrorKind;
use std::net::{IpAddr};
use std::sync::Arc;
use std::time::Duration;
use anyhow::anyhow;
use anyhow::Error;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use async_trait::async_trait;
use local_ip_address::local_ip;
use mac_address::MacAddress;
use pnet::datalink::NetworkInterface;
use tokio::net::{TcpStream};
use tokio::sync::Semaphore;
use tokio::time::timeout;
use crate::daemon::utils::udp::{send_udp_probe, test_dns_service, test_ntp_service, test_snmp_service};
use crate::server::services::types::base::Service;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::ports::Port;

const SCAN_TIMEOUT: Duration = Duration::from_millis(2000);
const MAX_CONCURRENT_CONNECTIONS: usize = 5;

pub enum ScanHandle {
    Tcp(u16, tokio::task::JoinHandle<Result<Option<u16>, Error>>),
    Udp(u16, tokio::task::JoinHandle<Result<Option<u16>, Error>>),
    Endpoint(Endpoint, tokio::task::JoinHandle<Result<Option<EndpointResponse>, Error>>),
}

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

    async fn ping_host(&self, ip: IpAddr) -> Result<bool, Error> {
        let config = Config::default();
        let client = Client::new(&config)?;
        
        let mut pinger = client.pinger(ip, PingIdentifier(rand::random())).await;
        
        // Send a few pings to increase reliability
        for sequence in 0..3 {
            let payload = [0; 56]; // Standard ping payload size
            
            match timeout(Duration::from_secs(2), pinger.ping(PingSequence(sequence), &payload)).await {
                Ok(Ok((_packet, _duration))) => {
                    tracing::debug!("Ping successful to {}", ip);
                    return Ok(true);
                },
                Ok(Err(e)) => {
                    tracing::debug!("Ping failed to {}: {}", ip, e);
                },
                Err(_) => {
                    tracing::debug!("Ping timeout to {}", ip);
                }
            }
            
            // Small delay between pings
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        Ok(false)
    }

    async fn scan_ports_and_endpoints(&self, ip: IpAddr) -> Result<(Vec<Port>, Vec<EndpointResponse>), Error> {
        let (tcp_handles, udp_handles, endpoint_handles) = tokio::join!(
            self.create_tcp_scan_handles(ip),
            self.create_udp_scan_handles(ip),
            self.create_endpoint_scan_handles(ip)
        );

        let tcp_handles = tcp_handles?;
        let udp_handles = udp_handles?;
        let endpoint_handles = endpoint_handles?;

        // Combine all handles
        let mut all_handles = Vec::new();
        all_handles.extend(tcp_handles);
        all_handles.extend(udp_handles);
        all_handles.extend(endpoint_handles);

        let mut open_ports = Vec::new();
        let mut endpoint_responses = Vec::new();

        // Process all handles with unified error handling
        for handle in all_handles {
            match handle {
                ScanHandle::Tcp(port_num, join_handle) => {
                    match join_handle.await {
                        Ok(Ok(Some(port))) => open_ports.push(Port::new_tcp(port)),
                        Ok(Ok(None)) => (),
                        Ok(Err(e)) => self.handle_scan_error(e, &format!("{}:{}", ip, port_num), "TCP")?,
                        Err(e) => tracing::debug!("JoinError for {}:{} (TCP): {}", ip, port_num, e),
                    }
                },
                ScanHandle::Udp(port_num, join_handle) => {
                    match join_handle.await {
                        Ok(Ok(Some(port))) => open_ports.push(Port::new_udp(port)),
                        Ok(Ok(None)) => (),
                        Ok(Err(e)) => self.handle_scan_error(e, &format!("{}:{}", ip, port_num), "UDP")?,
                        Err(e) => tracing::debug!("JoinError for {}:{} (UDP): {}", ip, port_num, e),
                    }
                },
                ScanHandle::Endpoint(endpoint, join_handle) => {
                    match join_handle.await {
                        Ok(Ok(Some(response))) => endpoint_responses.push(response),
                        Ok(Ok(None)) => (),
                        Ok(Err(e)) => self.handle_scan_error(e, &endpoint.to_string(), "Endpoint")?,
                        Err(e) => tracing::debug!("JoinError for endpoint {}: {}", endpoint, e),
                    }
                }
            }
        }

        tracing::info!("📊 Scan results for {}: found {} open ports, {} endpoint responses", 
                    ip, open_ports.len(), endpoint_responses.len());
        Ok((open_ports, endpoint_responses))
    }

    fn handle_scan_error(&self, e: Error, target: &str, scan_type: &str) -> Result<(), Error> {
        if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
            if io_error.kind() == ErrorKind::ResourceBusy || 
            io_error.raw_os_error() == Some(24) { // EMFILE on Unix
                tracing::error!("💥 File descriptor exhaustion on {} ({})", target, scan_type);
                return Err(e);
            }
        }
        
        tracing::debug!("Scan failed for {} ({}) - {}", target, scan_type, e);
        Ok(())
    }

    async fn create_endpoint_scan_handles(&self, ip: IpAddr) -> Result<Vec<ScanHandle>, Error> {

        let endpoints: Vec<Endpoint> = Service::discovery_endpoints().iter().map(|e| e.new_with_ip(ip)).collect();
        
        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_CONNECTIONS));
        let mut handles = Vec::new();
        
        for endpoint in endpoints {
            let semaphore = semaphore.clone();
            let endpoint_clone = endpoint.clone();
            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await;
                let client = reqwest::Client::new();
                let url = endpoint_clone.to_string();
                
                match timeout(SCAN_TIMEOUT, client.get(&url).send()).await {
                    Ok(Ok(response)) => {
                        match response.text().await {
                            Ok(text) => Ok(Some(EndpointResponse {
                                endpoint: endpoint_clone,
                                response: text,
                            })),
                            Err(_) => Ok(None),
                        }
                    },
                    _ => Ok(None),
                }
            });
            handles.push(ScanHandle::Endpoint(endpoint, handle));
        }
        
        Ok(handles)
    }

    async fn create_tcp_scan_handles(&self, ip: IpAddr) -> Result<Vec<ScanHandle>, Error> {
        let discovery_ports = Service::discovery_ports();
        let ports: Vec<u16> = discovery_ports.iter().filter_map(|p| p.tcp.then(|| p.number)).collect();
        
        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_CONNECTIONS));
        let mut handles = Vec::new();
        
        for port in ports {
            let semaphore = semaphore.clone();
            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await;
                match timeout(SCAN_TIMEOUT, TcpStream::connect((ip, port))).await {
                    Ok(Ok(_)) => Ok(Some(port)),
                    _ => Ok(None),
                }
            });
            handles.push(ScanHandle::Tcp(port, handle));
        }
        
        Ok(handles)
    }

    async fn create_udp_scan_handles(&self, ip: IpAddr) -> Result<Vec<ScanHandle>, Error> {
        let discovery_ports = Service::discovery_ports();
        let ports: Vec<u16> = discovery_ports.iter().filter_map(|p| p.udp.then(|| p.number)).collect();
        
        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_CONNECTIONS));
        let mut handles = Vec::new();
        
        for port in ports {
            let semaphore = semaphore.clone();
            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await;
                
                match port {
                    53 => test_dns_service(ip).await,
                    123 => test_ntp_service(ip).await,
                    161 => test_snmp_service(ip).await,
                    _ => send_udp_probe(ip, port).await,
                }
            });
            handles.push(ScanHandle::Udp(port, handle));
        }
        
        Ok(handles)
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
