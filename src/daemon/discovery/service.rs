use std::{net::IpAddr, sync::Arc};
use anyhow::{Error, Result};
use cidr::{IpCidr};
use chrono::Utc;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    daemon::{discovery::{manager::DaemonDiscoverySessionManager, types::base::DiscoveryPhase, utils::{arp_lookup, get_daemon_subnet, get_local_ip_address, ping_host, port_scan, reverse_dns_lookup}}, shared::storage::ConfigStore},
    server::{
        daemons::types::api::{DaemonDiscoveryProgressResponse, DaemonDiscoveryRequest},
        discovery::types::base::DiscoveryPort,
        nodes::types::{
            api::CreateNodeRequest, base::{DiscoveryStatus, Node, NodeBase}, status::NodeStatus, targets::{IpAddressTargetConfig, NodeTarget}, types::NodeType
        },
    },
};

pub struct DaemonDiscoveryService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
    pub discovery_manager: Arc<DaemonDiscoverySessionManager>,
}

impl DaemonDiscoveryService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        let discovery_manager = Arc::new(DaemonDiscoverySessionManager::new());
        
        Self {
            config_store,
            client: reqwest::Client::new(),
            discovery_manager,
        }
    }

    /// Main discovery session - discovers self and scans local subnet
    pub async fn run_discovery_session(&self, request: DaemonDiscoveryRequest) -> Result<(), Error> {
        let session_id = request.session_id;
        tracing::info!("Starting discovery session {}", session_id);

        let daemon_subnet = get_daemon_subnet()?;
        tracing::info!("Found daemon subnet {}", daemon_subnet);

        let discovery_ports: Vec<u16> = DiscoveryPort::iter().map(|p| p as u16).collect();
        
        // Scan subnet for active hosts using ICMP ping
        let active_hosts = self.scan_subnet_for_hosts(&daemon_subnet, session_id).await?;
        let host_count = active_hosts.len();
        
        // Process each discovered host
        let mut discovered_count = 1; // Start with 1 for self-discovery
        for host_ip in active_hosts {
            // Skip our own IP
            if host_ip == get_local_ip_address()? {
                continue;
            }
            
            // Report port scanning phase
            self.report_discovery_progress(session_id, &DaemonDiscoveryProgressResponse {
                session_id,
                phase: DiscoveryPhase::PortScanning,
                completed: discovered_count,
                total: host_count,
                discovered_count,
            }).await?;
            
            // Gather host information
            let hostname = reverse_dns_lookup(host_ip).await.ok();
            let open_ports = port_scan(host_ip, &discovery_ports).await?;

            if open_ports.is_empty() {
                continue; // Skip hosts with no interesting services
            }

            // Report info gathering phase
            self.report_discovery_progress(session_id, &DaemonDiscoveryProgressResponse {
                session_id,
                phase: DiscoveryPhase::GatheringInfo,
                completed: discovered_count,
                total: host_count,
                discovered_count,
            }).await?;

            let mut node = Node::new(NodeBase {
                name: hostname.clone().unwrap_or_else(|| format!("Device-{}", host_ip)),
                hostname: hostname,
                mac_address: arp_lookup(host_ip).await.ok(),
                status: NodeStatus::Unknown,
                target: NodeTarget::IpAddress(IpAddressTargetConfig {
                    ip: host_ip,
                    port: None,
                }),
                description: Some("Discovered device".to_string()),
                discovery_status: Some(DiscoveryStatus::Discovered {
                    session_id,
                    discovered_at: Utc::now(),
                }),
                subnets: vec![daemon_subnet],
                capabilities: Vec::new(),
                node_type: NodeType::UnknownDevice,
                assigned_tests: Vec::new(),
                monitoring_interval: 5,
                node_groups: Vec::new(),
            });


            for port in &open_ports {
                node.add_capability_from_port(*port);
            }
            
            self.create_discovered_node(session_id, &node).await?;
            discovered_count += 1;
        }
        
        // Report completion
        self.report_discovery_progress(session_id, &DaemonDiscoveryProgressResponse {
            session_id,
            phase: DiscoveryPhase::Complete,
            completed: discovered_count,
            total: discovered_count,
            discovered_count,
        }).await?;
        
        tracing::info!("Discovery session {} completed with {} nodes discovered", session_id, discovered_count);
        Ok(())
    }

    /// Scan subnet for active hosts using ICMP ping
    async fn scan_subnet_for_hosts(&self, subnet: &IpCidr, session_id: Uuid) -> Result<Vec<IpAddr>> {
        tracing::info!("Scanning subnet {} for active hosts using ICMP ping", subnet);
        let mut active_hosts = Vec::new();
        let mut scan_count = 0;
        let subnet_size = subnet.iter().count();

        // Report initial progress
        self.report_discovery_progress(session_id, &DaemonDiscoveryProgressResponse {
            session_id,
            phase: DiscoveryPhase::ScanningHosts,
            completed: 0,
            total: subnet_size,
            discovered_count: 0,
        }).await?;

        // Scan each IP in the subnet using ICMP ping
        for ip in subnet.iter() {
            let ip_addr = ip.address();
            scan_count += 1;
            
            if ping_host(ip_addr).await? {
                tracing::debug!("Found active host: {}", ip_addr);
                active_hosts.push(ip_addr);
            }
            
            // Report progress every 10 hosts
            if scan_count % 10 == 0 {
                self.report_discovery_progress(session_id, &DaemonDiscoveryProgressResponse {
                    session_id,
                    phase: DiscoveryPhase::ScanningHosts,
                    completed: scan_count,
                    total: subnet_size,
                    discovered_count: active_hosts.len(),
                }).await?;
            }
        }

        tracing::info!("Found {} active hosts in subnet {}", active_hosts.len(), subnet);
        Ok(active_hosts)
    }

    /// Report discovery progress to server
    async fn report_discovery_progress(&self, session_id: Uuid, progress: &DaemonDiscoveryProgressResponse) -> Result<()> {

        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/daemons/discovery_progress", server_target.to_string()))
            .json(&progress)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovery progress: HTTP {}", response.status());
        }

        tracing::debug!("Discovery progress reported for session {}", session_id);
        Ok(())
    }

    /// Report discovered node to server
    async fn create_discovered_node(&self, session_id: Uuid, node: &Node) -> Result<()> {

        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/nodes/create", server_target.to_string()))
            .json(&CreateNodeRequest {node: node.base.clone()})
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovered node: HTTP {}", response.status());
        }

        tracing::debug!("Discovered node reported for session {}", session_id);
        Ok(())
    }
}

