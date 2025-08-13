use crate::components::discovery::types::*;
use super::{snmp,scanner};
use std::net::Ipv4Addr;
use std::time::Instant;
use tokio::sync::RwLock;
use trust_dns_resolver::{TokioAsyncResolver};
use trust_dns_resolver::config::{ResolverOpts,ResolverConfig};
use std::net::IpAddr;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use std::collections::{HashMap,HashSet};
use crate::components::nodes::types::NetworkNode;

impl NetworkDiscovery {
    pub fn new() -> Result<Self> {
        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        );

        Ok(Self {
            discovered_devices: Arc::new(RwLock::new(HashMap::new())),
            rejected_devices: Arc::new(RwLock::new(HashSet::new())),
            progress: Arc::new(RwLock::new(DiscoveryProgress {
                total_targets: 0,
                completed: 0,
                discovered_devices: 0,
                current_phase: "Not started".to_string(),
                is_running: false,
                start_time: chrono::Utc::now(),
                estimated_completion: None,
            })),
            stop_signal: Arc::new(RwLock::new(false)),
            resolver,
        })
    }

    pub async fn start_discovery(&self, config: DiscoveryConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Reset state
        {
            let mut progress = self.progress.write().await;
            progress.is_running = true;
            progress.start_time = chrono::Utc::now();
            progress.current_phase = "Initializing".to_string();
            progress.completed = 0;
            progress.discovered_devices = 0;
        }

        *self.stop_signal.write().await = false;
        self.discovered_devices.write().await.clear();

        // Generate target IP ranges
        let target_ips = self.generate_target_ips(&config.target_subnets).await?;
        
        {
            let mut progress = self.progress.write().await;
            progress.total_targets = target_ips.len();
        }

        println!("Starting network discovery for {} targets", target_ips.len());

        // Phase 1: Ping sweep
        self.update_phase("Ping sweep").await;
        let alive_hosts = scanner::ping_sweep(&target_ips, &config, self.stop_signal.clone()).await;

        if *self.stop_signal.read().await {
            return Ok(());
        }

        // Filter out rejected devices
        let rejected_ips = self.rejected_devices.read().await;
        let alive_hosts: Vec<String> = alive_hosts
            .into_iter()
            .filter(|ip| !rejected_ips.contains(ip))
            .collect();

        // Phase 2: Port scanning (if enabled)
        let mut hosts_with_ports = HashMap::new();
        if config.port_scan_enabled && matches!(config.discovery_depth, DiscoveryDepth::Standard | DiscoveryDepth::Deep) {
            self.update_phase("Port scanning").await;
            hosts_with_ports = scanner::port_scan(&alive_hosts, &config, self.stop_signal.clone()).await;
        } else {
            // For Basic discovery, just add hosts without port info
            for host in &alive_hosts {
                hosts_with_ports.insert(host.clone(), vec![]);
            }
        }

        if *self.stop_signal.read().await {
            return Ok(());
        }

        // Phase 3: Service detection
        if config.include_services {
            self.update_phase("Service detection").await;
            self.detect_services(&hosts_with_ports, &config).await;
        }

        if *self.stop_signal.read().await {
            return Ok(());
        }

        // Phase 4: SNMP queries (for deeper discovery)
        if matches!(config.discovery_depth, DiscoveryDepth::Deep) {
            self.update_phase("SNMP discovery").await;
            let snmp_info = snmp::snmp_discovery(&alive_hosts, &config, self.stop_signal.clone()).await;
            self.apply_snmp_info(snmp_info).await;
        }

        // Mark discovery as complete
        {
            let mut progress = self.progress.write().await;
            progress.is_running = false;
            progress.current_phase = "Completed".to_string();
            progress.completed = progress.total_targets;
        }

        println!("Network discovery completed. Found {} devices", 
                    self.discovered_devices.read().await.len());

        Ok(())
    }

    pub async fn stop_discovery(&self) {
        *self.stop_signal.write().await = true;
        let mut progress = self.progress.write().await;
        progress.is_running = false;
        progress.current_phase = "Stopped".to_string();
    }

    pub async fn get_progress(&self) -> DiscoveryProgress {
        let progress = self.progress.read().await;
        let discovered_count = self.discovered_devices.read().await.len();
        
        DiscoveryProgress {
            discovered_devices: discovered_count,
            ..progress.clone()
        }
    }

    pub async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice> {
        self.discovered_devices.read().await.values().cloned().collect()
    }

    pub async fn accept_device(&self, device_id: &str) -> Result<NetworkNode, String> {
        let mut devices = self.discovered_devices.write().await;
        
        if let Some(device) = devices.get_mut(device_id) {
            device.status = DiscoveryStatus::Accepted;
            
            // Convert to NetworkNode
            let node = NetworkNode::new(
                device.hostname.clone().unwrap_or_else(|| format!("Device-{}", &device.ip[..8])),
                None, // domain
                Some(device.ip.clone()),
                self.get_primary_port(&device.open_ports), // Use primary service port
                None, // path
                Some(format!("{:?} device discovered via network scan", device.device_type))
            );
            
            Ok(node)
        } else {
            Err("Device not found".to_string())
        }
    }

    pub async fn reject_device(&self, device_id: &str) -> Result<(), String> {
        let mut devices = self.discovered_devices.write().await;
        
        if let Some(device) = devices.get_mut(device_id) {
            device.status = DiscoveryStatus::Rejected;
            
            // Add to rejected devices list to prevent future discovery
            let mut rejected_ips = self.rejected_devices.write().await;
            rejected_ips.insert(device.ip.clone());
            
            Ok(())
        } else {
            Err("Device not found".to_string())
        }
    }

    async fn generate_target_ips(&self, subnets: &[String]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut target_ips = Vec::new();

        for subnet in subnets {
            if subnet == "auto" {
                // Auto-detect local subnets
                target_ips.extend(self.detect_local_subnets().await?);
            } else {
                // Parse CIDR notation
                target_ips.extend(self.parse_cidr(subnet)?);
            }
        }

        Ok(target_ips)
    }

    async fn detect_local_subnets(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Simple implementation - in practice, this would use system APIs to get network interfaces
        // For now, return common private network ranges
        let common_subnets = vec![
            "192.168.1.0/24",
            "192.168.0.0/24", 
            "10.0.0.0/24",
            "172.16.0.0/24"
        ];

        let mut all_ips = Vec::new();
        for subnet in common_subnets {
            if let Ok(ips) = self.parse_cidr(subnet) {
                all_ips.extend(ips);
            }
        }

        Ok(all_ips)
    }

    fn parse_cidr(&self, cidr: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid CIDR format".into());
        }

        let base_ip: Ipv4Addr = parts[0].parse()?;
        let prefix_len: u8 = parts[1].parse()?;

        if prefix_len > 30 {
            return Err("Subnet too small".into());
        }

        let host_bits = 32 - prefix_len;
        let num_hosts = (1u32 << host_bits) - 2; // Exclude network and broadcast
        let base_ip_u32 = u32::from(base_ip);

        let mut ips = Vec::new();
        for i in 1..=num_hosts.min(254) { // Limit to reasonable range
            let ip_u32 = base_ip_u32 + i;
            let ip = Ipv4Addr::from(ip_u32);
            ips.push(ip.to_string());
        }

        Ok(ips)
    }

    async fn detect_services(&self, hosts_with_ports: &HashMap<String, Vec<u16>>, config: &DiscoveryConfig) {
        for (ip, ports) in hosts_with_ports {
            if *self.stop_signal.read().await {
                break;
            }

            let start_time = Instant::now();
            
            // Reverse DNS lookup
            let hostname = self.reverse_dns_lookup(ip).await;
            
            // Classify device type based on open ports
            let device_type = scanner::classify_device_type(ports);
            
            // Detect services on open ports
            let services = scanner::detect_services_on_ports(ip, ports, config.timeout_ms).await;

            let device = DiscoveredDevice {
                id: Uuid::new_v4().to_string(),
                ip: ip.clone(),
                hostname,
                mac_address: None, // Would need ARP table access
                device_type,
                open_ports: ports.clone(),
                services,
                vendor: None,
                os_fingerprint: None,
                response_time_ms: start_time.elapsed().as_millis() as u64,
                last_seen: chrono::Utc::now(),
                status: DiscoveryStatus::Pending,
            };

            self.discovered_devices.write().await.insert(device.id.clone(), device);
        }
    }

    async fn apply_snmp_info(&self, snmp_info: HashMap<String, HashMap<String, String>>) {
        let mut devices = self.discovered_devices.write().await;
        
        for (ip, info) in snmp_info {
            if let Some(device) = devices.values_mut().find(|d| d.ip == ip) {
                if let Some(hostname) = info.get("hostname") {
                    device.hostname = Some(hostname.clone());
                }
                if let Some(vendor) = info.get("vendor") {
                    device.vendor = Some(vendor.clone());
                }
                if let Some(description) = info.get("description") {
                    device.os_fingerprint = Some(description.clone());
                }
            }
        }
    }

    async fn reverse_dns_lookup(&self, ip: &str) -> Option<String> {
        if let Ok(ip_addr) = ip.parse::<IpAddr>() {
            if let Ok(response) = self.resolver.reverse_lookup(ip_addr).await {
                return response.iter().next().map(|name| name.to_string());
            }
        }
        None
    }

    async fn update_phase(&self, phase: &str) {
        let mut progress = self.progress.write().await;
        progress.current_phase = phase.to_string();
        println!("Discovery phase: {}", phase);
    }

    fn get_primary_port(&self, ports: &[u16]) -> Option<i64> {
        // Return the most likely primary service port
        let priority_ports = [443, 80, 22, 23, 3389, 5900, 161];
        
        for &priority_port in &priority_ports {
            if ports.contains(&priority_port) {
                return Some(priority_port as i64);
            }
        }
        
        // If no priority port found, return the first port
        ports.first().map(|&p| p as i64)
    }
}
