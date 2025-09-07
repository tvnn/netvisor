use anyhow::Result;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use crate::server::services::types::endpoints::Endpoint;
use crate::server::services::types::ports::{ApplicationProtocol, Port};
use crate::daemon::utils::base::{create_system_utils, PlatformSystemUtils, SystemUtils};
use crate::server::services::types::base::{Service, ServiceDiscriminants};
use crate::server::shared::types::metadata::TypeMetadataProvider;
use crate::{
    daemon::{shared::storage::ConfigStore}, server::{
        daemons::types::api::{
            DaemonRegistrationRequest, DaemonRegistrationResponse, 
        }, nodes::types::{base::{Node, NodeBase}, targets::{IpAddressTargetConfig, NodeTarget}}, shared::types::api::ApiResponse
    }
};

pub struct DaemonRuntimeService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
    pub utils: PlatformSystemUtils
}

impl DaemonRuntimeService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        Self {
            config_store,
            client: reqwest::Client::new(),
            utils: create_system_utils()
        }
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

    pub async fn create_self_as_node(&self, daemon_id: Uuid) -> Result<Node> {        
        // Get daemon configuration
        let config = &self.config_store;
        let own_port = config.get_port().await?;

        let local_ip = self.utils.get_own_ip_address()?;
        let hostname = self.utils.get_own_hostname();
        
        // Create node base
        let node_base = NodeBase {
            name: format!("NetVisor-Daemon-{}", local_ip),
            hostname,
            description: Some("NetVisor daemon for network diagnostics".to_string()),
            target: NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: local_ip,
            }),
            services: vec![],
            discovery_status: None,
            subnets: Vec::new(),
            node_groups: vec![],
        };

        let mut node = Node::new(node_base);

        node.add_service(Service::NetvisorDaemon { 
            confirmed: true, 
            name: ServiceDiscriminants::NetvisorDaemon.display_name().to_string(), 
            ports: vec!(),
            daemon_id, 
            endpoints: vec!(
                Endpoint { 
                    protocol: ApplicationProtocol::Http, 
                    ip: Some(local_ip), 
                    port: Port::new_tcp(own_port), 
                    path: None
                }
            )
        });

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

    /// Register daemon with server and return assigned ID
    pub async fn register_with_server(&self, node: Node, daemon_id: Uuid) -> Result<()> {
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
}