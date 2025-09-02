use std::{net::IpAddr, sync::Arc};
use anyhow::{Error, Result};
use cidr::{IpCidr};
use chrono::{DateTime, Utc};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;
use futures::{future::{try_join_all}, stream::{self, StreamExt}};
use std::result::Result::Ok;
use crate::{
    daemon::{discovery::{manager::DaemonDiscoverySessionManager, types::base::DiscoveryPhase}, shared::storage::ConfigStore, utils::base::{create_system_utils, PlatformSystemUtils, SystemUtils}},
    server::{
        capabilities::types::base::Capability, daemons::types::api::{DaemonDiscoveryRequest, DaemonDiscoveryUpdate}, nodes::types::{
            base::{DiscoveryStatus, Node, NodeBase}, status::NodeStatus, targets::{IpAddressTargetConfig, NodeTarget}, types::NodeType
        }, subnets::types::base::{NodeSubnetMembership, Subnet}
    },
};

const CONCURRENT_SCANS: usize = 100;

pub struct DaemonDiscoveryService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
    pub discovery_manager: Arc<DaemonDiscoverySessionManager>,
    pub utils: PlatformSystemUtils
}

impl DaemonDiscoveryService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        let discovery_manager = Arc::new(DaemonDiscoverySessionManager::new());
        
        Self {
            config_store,
            client: reqwest::Client::new(),
            discovery_manager,
            utils: create_system_utils()
        }
    }

    /// Main discovery session - discovers self and scans local subnet
    pub async fn run_discovery_session(&self, request: DaemonDiscoveryRequest, cancel: CancellationToken, subnets: Vec<Subnet>) -> Result<(), Error> {
        let daemon_id = self.config_store.get_id().await?.expect("By the time discovery is running, ID will be assigned");
        let session_id = request.session_id;
        let discovered_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let started_at = Utc::now();
        tracing::info!("Starting discovery session {}", session_id);

        let total_subnets = subnets.len();

        self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
            session_id,
            phase: DiscoveryPhase::Started,
            completed: 0,
            total: 0,
            subnet: 0,
            total_subnets,
            error: None,
            discovered_count: 0,
            daemon_id,
            started_at: Some(started_at),
            finished_at: None
        }).await?;

        let local_ip = self.utils.get_own_ip_address()?;

        let discovery_futures = subnets.iter()
            .enumerate()
            .map(|(index, subnet )| {
                self.scan_and_process_hosts(
                    subnet,
                    session_id,
                    daemon_id,
                    started_at,
                    cancel.clone(),
                    local_ip,
                    discovered_count.clone(),
                    index,
                    total_subnets.clone()
                )
            });

        let discovery_result = try_join_all(discovery_futures).await;

        let final_discovered_count = discovered_count.load(std::sync::atomic::Ordering::Relaxed);

        match discovery_result {
            Ok(_) => {
                tracing::info!("Discovery session {} completed successfully", session_id);
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Complete,
                    completed: final_discovered_count,
                    total: final_discovered_count,
                    total_subnets,
                    subnet: total_subnets,
                    error: None,
                    discovered_count: final_discovered_count,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: Some(Utc::now())
                }).await?;
            },
            Err(_) if cancel.is_cancelled() => {
                tracing::warn!("Discovery session {} was cancelled", session_id);
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Cancelled,
                    completed: final_discovered_count,
                    total: final_discovered_count,
                    total_subnets,
                    subnet: total_subnets,
                    error: None,
                    discovered_count: final_discovered_count,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: Some(Utc::now())
                }).await?;
            },
            Err(e) => {
                tracing::error!("Discovery session {} failed: {}", session_id, e);
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Failed,
                    completed: final_discovered_count,
                    total: final_discovered_count,
                    total_subnets,
                    subnet: total_subnets,
                    error: Some(e.to_string()),
                    discovered_count: final_discovered_count,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: Some(Utc::now())
                }).await?;
            }
        }
        
        tracing::info!("Discovery session {} finished with {} nodes discovered", session_id, final_discovered_count);
        Ok(())
    }

    /// Scan subnet concurrently and process hosts immediately as they're discovered
    async fn scan_and_process_hosts(
        &self,
        subnet: &Subnet,
        session_id: Uuid,
        daemon_id: Uuid,
        started_at: DateTime<Utc>,
        cancel: CancellationToken,
        local_ip: IpAddr,
        discovered_count: Arc<std::sync::atomic::AtomicUsize>,
        subnet_index: usize,
        total_subnets: usize
    ) -> Result<()> {
        tracing::info!("Scanning subnet {} concurrently for hosts with open ports", subnet.base.cidr);
        let subnet_size = subnet.base.cidr.iter().count();
        let scanned_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        // Report initial progress
        self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
            session_id,
            phase: DiscoveryPhase::Scanning,
            completed: 0,
            total_subnets,
            subnet: subnet_index,
            total: subnet_size,
            error: None,
            discovered_count: 0,
            daemon_id,
            started_at: Some(started_at),
            finished_at: None
        }).await?;

        // Process all IPs concurrently, combining discovery and processing
        let results = stream::iter(self.determine_scan_order(&subnet.base.cidr))
            .map({
                let cancel = cancel.clone();
                let scanned_count = scanned_count.clone();
                let discovered_count = discovered_count.clone();
                let session_id = session_id;
                
                move |ip| {
                    let cancel = cancel.clone();
                    let scanned_count = scanned_count.clone();
                    let discovered_count = discovered_count.clone();
                    let subnet = subnet.clone();
                    
                    async move {
                        if cancel.is_cancelled() {
                            return Err(Error::msg("Discovery was cancelled"))
                        }

                        // Skip local IP
                        if ip == local_ip {
                            scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            return Ok(());
                        }

                        

                        // Port scan directly - if ports are open, host is alive
                        match self.utils.scan_tcp_ports(ip).await {
                            Ok(open_ports) if !open_ports.is_empty() => {
                                // Process this host immediately
                                if let Err(e) = self.process_discovered_host(
                                    ip,
                                    open_ports,
                                    session_id,
                                    &subnet,
                                    discovered_count.clone()
                                ).await {
                                    tracing::warn!("Failed to process host {}: {}", ip, e);
                                }
                            },
                            Ok(_) => {
                                // No open ports, host might be alive but no interesting services
                                tracing::debug!("Host {} has no open discovery ports", ip);
                            },
                            Err(e) => {
                                tracing::debug!("Failed to scan host {}: {}", ip, e);
                            }
                        }

                        scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        Ok(())
                    }
                }
            })
            .buffer_unordered(CONCURRENT_SCANS);

        // Consume the stream and report progress periodically
        let mut stream_pin = Box::pin(results);
        let mut last_reported_scan_count: usize = 0;
        let mut last_reported_discovery_count: usize = 0;
        
        while let Some(result) = stream_pin.next().await {

            if cancel.is_cancelled() {
                tracing::warn!("Discovery session {} cancelled", session_id);
                return Err(Error::msg("Discovery was cancelled"));
            }

            if let Err(e) = result {
                tracing::warn!("Error during host scanning: {}", e);
            }

            let current_scanned = scanned_count.load(std::sync::atomic::Ordering::Relaxed);
            let current_discovered = discovered_count.load(std::sync::atomic::Ordering::Relaxed);

            // Report progress every 20 scans or when done
            if current_scanned >= last_reported_scan_count + 20 || last_reported_discovery_count > current_discovered {
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Scanning,
                    completed: current_scanned,
                    total: subnet_size,
                    total_subnets,
                    subnet: subnet_index,
                    error: None,
                    discovered_count: current_discovered,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: None
                }).await?;
                last_reported_scan_count = current_scanned;
                last_reported_discovery_count = current_discovered
            }
        }

        let final_discovered = discovered_count.load(std::sync::atomic::Ordering::Relaxed);
        tracing::info!("Scanned {} IPs and found {} hosts with open ports", subnet_size, final_discovered);
        Ok(())
    }
    /// Process a discovered host immediately (port scan, DNS lookup, node creation)
    async fn process_discovered_host(
        &self,
        host_ip: IpAddr,
        open_ports: Vec<u16>,
        session_id: Uuid,
        subnet: &Subnet,
        discovered_count: Arc<std::sync::atomic::AtomicUsize>
    ) -> Result<()> {
        
        if open_ports.is_empty() {
            return Ok(()); // Skip hosts with no interesting services
        }

        // Gather host information
        let hostname = self.utils.get_hostname_for_ip(host_ip).await?;
        let mac_address = self.utils.get_mac_address_for_ip(host_ip).await?;

        // Create node
        let mut node = Node::new(NodeBase {
            name: hostname.clone().unwrap_or_else(|| format!("Device-{}", host_ip)),
            hostname,
            status: NodeStatus::Unknown,
            target: NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: host_ip,
            }),
            description: Some("Discovered device".to_string()),
            discovery_status: Some(DiscoveryStatus::Discovered {
                session_id,
                discovered_at: Utc::now(),
            }),
            subnets: vec!(NodeSubnetMembership {
                subnet_id: subnet.id,
                ip_address: host_ip,
                mac_address
            }),
            capabilities: Vec::new(),
            dns_resolver_node_id: None,
            node_type: NodeType::UnknownDevice,
            monitoring_interval: 5,
            node_groups: Vec::new(),
        });

        for port in &open_ports {
            if let Some(capability) = Capability::from_port(*port) {
                node.add_capability(capability);
            }
        }
        
        self.create_discovered_node(session_id, &node).await?;
        discovered_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        tracing::info!("Processed and created node for host {} with {} open ports", host_ip, open_ports.len());
        Ok(())
    }

    /// Determine scan ordering - scan common host ranges first
    fn determine_scan_order(&self, subnet: &IpCidr) -> impl Iterator<Item = IpAddr> {
        let mut ips: Vec<IpAddr> = subnet.iter().map(|ip| ip.address()).collect();
        
        // Sort to put common ranges first (.10-.100, then .101-.200, etc.)
        ips.sort_by_key(|ip| {
            let last_octet = match ip {
                IpAddr::V4(ipv4) => ipv4.octets()[3],
                IpAddr::V6(_) => return 1000, // IPv6 gets lower priority for now
            };
            
            match last_octet {
                10..=100 => last_octet as u16,           // Priority range
                101..=200 => 1000 + last_octet as u16,   // Secondary range
                1..=9 => 2000 + last_octet as u16,       // Infrastructure range
                _ => 3000 + last_octet as u16,           // Everything else
            }
        });
        
        ips.into_iter()
    }

    /// Report discovery progress to server
    pub async fn report_discovery_update(&self, session_id: Uuid, update: &DaemonDiscoveryUpdate) -> Result<()> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/daemons/discovery_update", server_target.to_string()))
            .json(&update)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovery update: HTTP {}", response.status());
        }

        tracing::debug!("Discovery update reported for session {}", session_id);
        Ok(())
    }

    /// Report discovered node to server
    async fn create_discovered_node(&self, session_id: Uuid, node: &Node) -> Result<()> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/nodes", server_target.to_string()))
            .json(node)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovered node: HTTP {}", response.status());
        }

        tracing::debug!("Discovered node reported for session {}", session_id);
        Ok(())
    }
}