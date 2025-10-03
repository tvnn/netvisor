use std::sync::Arc;

use anyhow::Error;
use uuid::Uuid;

use crate::{
    daemon::{
        discovery::manager::DaemonDiscoverySessionManager,
        shared::storage::ConfigStore,
        utils::base::{create_system_utils, PlatformDaemonUtils},
    },
    server::{
        daemons::types::api::DaemonDiscoveryUpdate,
        hosts::types::{api::HostWithServicesRequest, base::Host},
        services::types::base::Service,
        shared::types::api::ApiResponse,
        subnets::types::base::Subnet,
    },
};

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
            utils: create_system_utils(),
        }
    }

    pub async fn create_host(&self, host: Host, services: Vec<Service>) -> Result<Host, Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        tracing::info!("Creating host {}", host.base.name);

        let response = self
            .client
            .post(format!("{}/api/hosts", server_target))
            .json(&HostWithServicesRequest { host, services })
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovered host: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<Host> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create host: {}", error_msg);
        }

        let created_host = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No host data in successful response"))?;

        Ok(created_host)
    }

    pub async fn create_subnet(&self, subnet: &Subnet) -> Result<Subnet, Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/subnets", server_target))
            .json(&subnet)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovered subnet: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<Subnet> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create subnet: {}", error_msg);
        }

        let created_subnet = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No subnet data in successful response"))?;

        Ok(created_subnet)
    }

    pub async fn create_service(&self, service: &Service) -> Result<Service, Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/services", server_target))
            .json(&service)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovered service: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<Service> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create service: {}", error_msg);
        }

        let created_service = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No service data in successful response"))?;

        Ok(created_service)
    }

    /// Report discovery progress to server
    pub async fn report_discovery_update(
        &self,
        session_id: Uuid,
        update: &DaemonDiscoveryUpdate,
    ) -> Result<(), Error> {
        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/daemons/discovery_update", server_target))
            .json(&update)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovery update: HTTP {}",
                response.status()
            );
        }

        tracing::debug!("Discovery update reported for session {}", session_id);
        Ok(())
    }
}
