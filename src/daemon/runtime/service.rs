use anyhow::Result;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use hostname::get as get_hostname;
use mac_address::get_mac_address;
use crate::server::capabilities::types::base::Capability;
use crate::server::capabilities::types::configs::{DaemonConfig};
use crate::{
    daemon::{discovery::{utils::{get_daemon_subnet, get_local_ip_address, port_scan}}, shared::storage::ConfigStore}, server::{
        daemons::types::api::{
            DaemonRegistrationRequest, DaemonRegistrationResponse, 
        }, nodes::types::{base::{Node, NodeBase}, status::NodeStatus, targets::{IpAddressTargetConfig, NodeTarget}, types::NodeType}, shared::types::api::ApiResponse
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
    pub async fn register_with_server(&mut self, node: Node, daemon_id: Uuid) -> Result<()> {
        tracing::info!("Registering daemon with ID: {}, Node ID: {:?}", daemon_id, node.id);
        let registration_request = DaemonRegistrationRequest {daemon_id, node};

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
        
        Ok(())
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

    pub async fn create_self_as_node(&self, daemon_id: Uuid) -> Result<Node> {        
        // Get daemon configuration
        let config = &self.config_store;
        let own_port = config.get_port().await?;

        // Get local IP address using proper method
        let local_ip = get_local_ip_address()?;
        
        // Scan own ports to detect capabilities
        let open_ports = port_scan(local_ip).await?;
        
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
            name: format!("NetVisor-Daemon-{}", local_ip),
            hostname,
            node_type: NodeType::UnknownDevice,
            description: Some("NetVisor daemon for network diagnostics".to_string()),
            target: NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: local_ip,
            }),
            mac_address,
            capabilities: vec![], // Will be populated below
            dns_resolver_node_id: None,
            discovery_status: None,
            subnets: vec![get_daemon_subnet()?],
            status: NodeStatus::Unknown,
            monitoring_interval: 10,
            node_groups: vec![],
        };

        let mut node = Node::new(node_base);
        node.base.capabilities = Vec::new();

        // Add capabilities from detected ports using existing method
        for port in &open_ports {
            if let Some(capability) = Capability::from_port(*port) {
                node.add_capability(capability);
            }
        };

        node.add_capability(Capability::Daemon(DaemonConfig::new(&node, own_port, daemon_id)));

        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/nodes", server_target.to_string()))
            .json(&node)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report daemon as node: HTTP {}", response.status());
        }

        let api_response: ApiResponse<Node> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response.error.unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create node: {}", error_msg);
        }

        let created_node = api_response.data
            .ok_or_else(|| anyhow::anyhow!("No node data in successful response"))?;

        Ok(created_node)
    }
}