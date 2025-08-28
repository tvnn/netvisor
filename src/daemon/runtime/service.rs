use anyhow::Result;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use strum::IntoEnumIterator;
use hostname::get as get_hostname;
use mac_address::get_mac_address;
use crate::{
    daemon::{discovery::{utils::{get_daemon_subnet, get_local_ip_address, port_scan}}, shared::storage::ConfigStore}, server::{
        daemons::types::api::{
            DaemonRegistrationRequest, DaemonRegistrationResponse, 
        }, discovery::types::base::DiscoveryPort, nodes::types::{base::{Node, NodeBase}, capabilities::CapabilitySource, status::NodeStatus, targets::{IpAddressTargetConfig, NodeTarget}, types::NodeType}, shared::types::api::ApiResponse
    }
};

pub struct DaemonRuntimeService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
}

impl DaemonRuntimeService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        Self {
            config_store,
            client: reqwest::Client::new(),
        }
    }

    /// Register daemon with server and return assigned ID
    pub async fn register_with_server(&mut self, node: Node) -> Result<Uuid> {
        let registration_request = DaemonRegistrationRequest {node};

        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/daemons/register", server_target.to_string()))
            .json(&registration_request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Registration failed: HTTP {}", response.status());
        }

        let api_response: ApiResponse<DaemonRegistrationResponse> = response.json().await?;
        
        if !api_response.success {
            let error_msg = api_response.error.unwrap_or_else(|| "Unknown registration error".to_string());
            anyhow::bail!("Registration failed: {}", error_msg);
        }

        let daemon_id = api_response.data
            .ok_or_else(|| anyhow::anyhow!("No daemon data in successful response"))?
            .daemon
            .id;
        
        tracing::info!("Successfully registered with server, assigned ID: {}", daemon_id);
        
        Ok(daemon_id)
    }

    pub async fn heartbeat(&self) -> Result<()> {

        let daemon_id = self.config_store.get_id().await?.expect("By the time heartbeat is running, ID will be assigned");
        let interval = Duration::from_secs(self.config_store.get_heartbeat_interval().await?);

        let mut interval_timer = tokio::time::interval(interval);
        interval_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        
        loop {
            interval_timer.tick().await;
            
            match self.send_heartbeat(&daemon_id).await {
                Ok(()) => {
                    // Update last heartbeat timestamp in config
                    if let Err(e) = self.config_store.update_heartbeat().await {
                        tracing::warn!("Failed to update heartbeat timestamp: {}", e);
                    }
                    tracing::trace!("💓 Heartbeat sent successfully");
                }
                Err(e) => {
                    tracing::warn!("❤️‍🩹 Heartbeat failed: {}", e);
                    // Continue trying - don't exit on heartbeat failures
                }
            }
        }
    }

    /// Send heartbeat to server
    pub async fn send_heartbeat(&self, daemon_id: &Uuid) -> Result<()> {

        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .put(format!("{}/api/daemons/{}/heartbeat", server_target.to_string(), daemon_id))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Heartbeat failed: HTTP {}", response.status());
        }

        tracing::debug!("Heartbeat sent successfully");
        Ok(())
    }

    pub async fn create_self_as_node(&self) -> Result<Node> {
        tracing::info!("Discovering self as node");
        
        // Get daemon configuration
        let config = &self.config_store;

        // Get local IP address using proper method
        let local_ip = get_local_ip_address()?;
        
        // Scan own ports to detect capabilities
        let discovery_ports: Vec<u16> = DiscoveryPort::iter().map(|p| p as u16).collect();
        let open_ports = port_scan(local_ip, &discovery_ports).await?;
        
        // Get hostname
        let hostname = get_hostname()
            .ok()
            .map(|os_str| os_str.to_string_lossy().into_owned());

        // Get mac address
        let mac_address = match get_mac_address()? {
            Some(mac) => Some(mac.to_string()),
            None => None
        };
        
        // Create node base
        let node_base = NodeBase {
            name: hostname.clone().unwrap_or_else(|| format!("NetVisor-Daemon-{}", local_ip)),
            hostname,
            node_type: NodeType::UnknownDevice,
            description: Some("NetVisor daemon for network diagnostics".to_string()),
            target: NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: local_ip,
                port: Some(config.get_port().await?),
            }),
            mac_address,
            capabilities: vec![], // Will be populated below
            discovery_status: None,
            subnets: vec![get_daemon_subnet()?],
            status: NodeStatus::Unknown,
            assigned_tests: vec![],
            monitoring_interval: 10,
            node_groups: vec![],
        };

        let mut node = Node::new(node_base);

        // Add capabilities from detected ports using existing method
        for port in &open_ports {
            node.add_capability_from_port(*port);
        }

        // Always add daemon service capability for self
        node.base.capabilities.push(crate::server::nodes::types::capabilities::NodeCapability::DaemonService { 
            source: CapabilitySource::system(),
            daemon_id: config.get_id().await?.expect("ID should have a value at this point, either from config or assigned"),
        });

        Ok(node)
    }
}