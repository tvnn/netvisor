
use std::collections::HashMap;
use std::net::{IpAddr};
use std::time::Duration;
use anyhow::{Result};
use anyhow::anyhow;
use anyhow::Error;
use async_trait::async_trait;
use cidr::IpCidr;
use mac_address::MacAddress;
use pnet::ipnetwork::IpNetwork;
use tokio::net::{TcpStream};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;
use crate::server::interfaces::types::base::{Interface, InterfaceBase};
use crate::server::subnets::types::base::{Subnet};
use crate::daemon::utils::udp::{send_udp_probe, test_dhcp_service, test_dns_service, test_ntp_service, test_snmp_service};
use crate::server::services::types::base::Service;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::ports::{Port, TransportProtocol};
use crate::server::utils::base::NetworkUtils;

const SCAN_TIMEOUT: Duration = Duration::from_millis(800);

/// Cross-platform system utilities trait
#[async_trait]
pub trait DaemonUtils: NetworkUtils {
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
    
    async fn get_hostname_for_ip(&self, ip: IpAddr) -> Result<Option<String>, Error> {
        match timeout(SCAN_TIMEOUT, async {
            tokio::task::spawn_blocking(move || dns_lookup::lookup_addr(&ip)).await?
        }).await {
            Ok(Ok(hostname)) => Ok(Some(hostname)),
            _ => Ok(None),
        }
    }
    
    async fn scan_interfaces(&self, daemon_id: Uuid) -> Result<(Vec<Interface>, Vec<Subnet>)> {
        let interfaces = self.get_own_interfaces();
        
        // First pass: collect all interface data and potential subnets
        let mut potential_subnets: Vec<(String, IpNetwork)> = Vec::new();
        let mut interface_data: Vec<(String, IpAddr, Option<MacAddress>)> = Vec::new();
        
        for interface in interfaces.into_iter().filter(|i| !i.is_loopback()) {
            let name = interface.name.clone();
            let mac_address = match interface.mac {
                Some(mac) if !mac.octets().iter().all(|o| *o==0) => Some(MacAddress::new(mac.octets())),
                _ => None
            };
            
            for ip in interface.ips.iter() {
                interface_data.push((name.clone(), ip.ip(), mac_address));
                potential_subnets.push((name.clone(), *ip));
            }
        }
        
        // Second pass: create unique subnets from valid networks
        let mut subnet_map: HashMap<IpCidr, Subnet> = HashMap::new();
        
        for (interface_name, ip_network) in potential_subnets {
            if let Some(subnet) = Subnet::from_discovery(&interface_name, &ip_network, daemon_id) {
                subnet_map.entry(subnet.base.cidr).or_insert(subnet);
            }
        }
        
        // Third pass: assign all interfaces to appropriate subnets
        let mut interfaces_list = Vec::new();
        
        for (interface_name, ip_addr, mac_address) in interface_data {
            // Find which subnet this IP belongs to
            if let Some(subnet) = subnet_map.values().find(|s| s.base.cidr.contains(&ip_addr)) {
                interfaces_list.push(Interface::new(InterfaceBase{
                    name: Some(interface_name),
                    subnet_id: subnet.id,
                    ip_address: ip_addr,
                    mac_address
                }));
            }
        }

        let subnets: Vec<Subnet> = subnet_map.into_values().collect();
        
        Ok((interfaces_list, subnets))
    }

    async fn scan_ports_and_endpoints(&self, ip: IpAddr, cancel: CancellationToken) -> Result<(Vec<Port>, Vec<EndpointResponse>), Error> {
        if cancel.is_cancelled() {
            return Err(anyhow!("Operation cancelled"));
        }

        let mut open_ports = Vec::new();
        let mut endpoint_responses = Vec::new();

        // Scan TCP ports sequentially (not concurrently)
        let tcp_ports = self.scan_tcp_ports(ip, cancel.clone()).await?;
        open_ports.extend(tcp_ports);

        if cancel.is_cancelled() {
            return Err(anyhow!("Operation cancelled"));
        }

        // Scan UDP ports sequentially  
        let udp_ports = self.scan_udp_ports(ip, cancel.clone()).await?;
        open_ports.extend(udp_ports);

        if cancel.is_cancelled() {
            return Err(anyhow!("Operation cancelled"));
        }

        // Scan endpoints sequentially
        let endpoints = self.scan_endpoints(ip, cancel.clone()).await?;
        endpoint_responses.extend(endpoints);

        tracing::info!("📊 Scan results for {}: found {} open ports, {} endpoint responses", 
                    ip, open_ports.len(), endpoint_responses.len());
        Ok((open_ports, endpoint_responses))
    }

    async fn scan_tcp_ports(&self, ip: IpAddr, cancel: CancellationToken) -> Result<Vec<Port>, Error> {
        let discovery_ports = Service::all_discovery_ports();
        let ports: Vec<u16> = discovery_ports.iter().filter_map(|p| (p.protocol == TransportProtocol::Tcp).then(|| p.number)).collect();
        
        let mut open_ports = Vec::new();
        
        for port in ports {
            if cancel.is_cancelled() {
                break;
            }
            
            match timeout(SCAN_TIMEOUT, TcpStream::connect((ip, port))).await {
                Ok(Ok(_)) => {
                    open_ports.push(Port::new_tcp(port));
                    tracing::debug!("Found open TCP port {}:{}", ip, port);
                },
                _ => {} // Port closed or timeout
            }
        }
        
        Ok(open_ports)
    }

    async fn scan_udp_ports(&self, ip: IpAddr, cancel: CancellationToken) -> Result<Vec<Port>, Error> {
        let discovery_ports = Service::all_discovery_ports();
        let ports: Vec<u16> = discovery_ports.iter().filter_map(|p| (p.protocol == TransportProtocol::Udp).then(|| p.number)).collect();
        
        let mut open_ports = Vec::new();
        
        for port in ports {
            if cancel.is_cancelled() {
                break;
            }
            
            let result = match port {
                53 => test_dns_service(ip).await,
                123 => test_ntp_service(ip).await,
                161 => test_snmp_service(ip).await,
                67 => test_dhcp_service(ip).await,
                _ => send_udp_probe(ip, port).await,
            };
            
            if let Ok(Some(detected_port)) = result {
                open_ports.push(Port::new_udp(detected_port));
                tracing::debug!("Found open UDP port {}:{}", ip, detected_port);
            }
        }
        
        Ok(open_ports)
    }

    async fn scan_endpoints(&self, ip: IpAddr, cancel: CancellationToken) -> Result<Vec<EndpointResponse>, Error> {
        let endpoints: Vec<Endpoint> = Service::all_discovery_endpoints().iter().map(|e| e.use_ip(ip)).collect();
        let mut responses = Vec::new();

        let client = reqwest::Client::builder()
            .timeout(SCAN_TIMEOUT)  // Total request timeout
            .connect_timeout(Duration::from_millis(SCAN_TIMEOUT.as_millis() as u64 / 2))  // Half for connection
            .build()?;
        
        for endpoint in endpoints {
            if cancel.is_cancelled() {
                break;
            }
            
            let url = endpoint.to_string();
            
            // No need for additional timeout() wrapper since client has timeout configured
            match client.get(&url).send().await {
                Ok(response) => {
                    if let Ok(text) = response.text().await {
                        responses.push(EndpointResponse {
                            endpoint,
                            response: text,
                        });
                    }
                },
                Err(_) => {} // Endpoint not responding or timed out
            }
        }
        
        Ok(responses)
    }
}

#[cfg(target_os = "linux")]
use crate::daemon::utils::linux::LinuxDaemonUtils;
#[cfg(target_os = "linux")]
pub type PlatformDaemonUtils = LinuxDaemonUtils;

#[cfg(target_os = "macos")] 
use crate::daemon::utils::macos::MacOsDaemonUtils;
#[cfg(target_os = "macos")]
pub type PlatformDaemonUtils = MacOsDaemonUtils;

#[cfg(target_family = "windows")]
use crate::daemon::utils::windows::WindowsDaemonUtils;
#[cfg(target_family = "windows")]
pub type PlatformDaemonUtils = WindowsDaemonUtils;

pub fn create_system_utils() -> PlatformDaemonUtils {
    PlatformDaemonUtils::new()
}
