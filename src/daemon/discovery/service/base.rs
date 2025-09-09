use std::sync::Arc;

use anyhow::Error;
use uuid::Uuid;

use crate::{daemon::{discovery::{manager::DaemonDiscoverySessionManager}, shared::storage::ConfigStore, utils::base::{create_system_utils, PlatformDaemonUtils}}, server::{daemons::types::api::{DaemonDiscoveryUpdate}, nodes::types::{api::{NodeUpdateRequest, UpdateNodeResponse}, base::Node}, shared::types::api::ApiResponse, subnets::types::base::Subnet}};

pub struct DaemonDiscoveryService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
    pub discovery_manager: Arc<DaemonDiscoverySessionManager>,
    pub utils: PlatformDaemonUtils,
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

    pub async fn create_node(&self, node: &Node) -> Result<Node, Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        tracing::info!("Creating node {}", node.base.target.to_string());

        let response = self.client
            .post(format!("{}/api/nodes", server_target.to_string()))
            .json(node)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovered node: HTTP {}", response.status());
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

    pub async fn update_node(&self, node_id: Uuid, update: &NodeUpdateRequest) -> Result<Option<Node>, Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self.client
            .put(format!("{}/api/nodes/{}", server_target.to_string(), node_id))
            .json(update)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to update node: HTTP {}", response.status());
        }

        let api_response: ApiResponse<UpdateNodeResponse> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response.error.unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to update node: {}", error_msg);
        }

        Ok(match api_response.data {
            Some(d) => Some(d.node),
            None => None
        })
    }

    pub async fn create_subnet(&self, subnet: &Subnet) -> Result<Subnet, Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self.client
            .post(format!("{}/api/subnets", server_target.to_string()))
            .json(&subnet)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovered subnet: HTTP {}", response.status());
        }

        let api_response: ApiResponse<Subnet> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response.error.unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create subnet: {}", error_msg);
        }

        let created_subnet = api_response.data
            .ok_or_else(|| anyhow::anyhow!("No subnet data in successful response"))?;

        Ok(created_subnet)
    }

    /// Report discovery progress to server
    pub async fn report_discovery_update(&self, session_id: Uuid, update: &DaemonDiscoveryUpdate) -> Result<(), Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self.client
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
}