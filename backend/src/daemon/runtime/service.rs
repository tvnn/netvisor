use crate::daemon::utils::base::DaemonUtils;
use crate::daemon::utils::base::{create_system_utils, PlatformDaemonUtils};
use crate::server::networks::types::Network;
use crate::{
    daemon::shared::storage::ConfigStore,
    server::{
        daemons::types::api::{DaemonRegistrationRequest, DaemonRegistrationResponse},
        shared::types::api::ApiResponse,
    },
};
use anyhow::anyhow;
use anyhow::Result;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

pub struct DaemonRuntimeService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
    pub utils: PlatformDaemonUtils,
}

impl DaemonRuntimeService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        Self {
            config_store,
            client: reqwest::Client::new(),
            utils: create_system_utils(),
        }
    }

    pub async fn heartbeat(&self) -> Result<()> {
        let daemon_id = self.config_store.get_id().await?;
        let interval = Duration::from_secs(self.config_store.get_heartbeat_interval().await?);

        let mut interval_timer = tokio::time::interval(interval);
        interval_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let server_target = self.config_store.get_server_endpoint().await?;

        loop {
            interval_timer.tick().await;

            let response = self
                .client
                .put(format!(
                    "{}/api/daemons/{}/heartbeat",
                    server_target, daemon_id
                ))
                .send()
                .await?;

            if !response.status().is_success() {
                let api_response: ApiResponse<()> = response.json().await?;

                if !api_response.success {
                    let error_msg = api_response
                        .error
                        .unwrap_or_else(|| "Unknown error".to_string());
                    tracing::warn!("❤️‍🩹 Heartbeat failed: {}", error_msg);
                }
            }

            if let Err(e) = self.config_store.update_heartbeat().await {
                tracing::warn!("Failed to update heartbeat timestamp: {}", e);
            }
            tracing::info!("💓 Heartbeat sent successfully");
        }
    }

    // /// Check if self is registered and host exists
    // pub async fn check_registry(&self, host_id: Uuid, daemon_id: Uuid) -> Result<()> {
    //     tracing::info!("Checking registration of daemon with ID: {}, host ID: {:?}", daemon_id, host_id);
    //     let registration_request = DaemonRegistrationRequest {daemon_id, host_id};

    //     let server_target = self.config_store.get_server_endpoint().await?;

    //     let response = self
    //         .client
    //         .post(format!("{}/api/daemons/register", server_target.to_string()))
    //         .json(&registration_request)
    //         .send()
    //         .await?;

    //     if !response.status().is_success() {
    //         anyhow::bail!("Registration failed: HTTP {}", response.status());
    //     }

    //     let api_response: ApiResponse<DaemonRegistrationResponse> = response.json().await?;

    //     if !api_response.success {
    //         let error_msg = api_response.error.unwrap_or_else(|| "Unknown registration error".to_string());
    //         anyhow::bail!("Registration failed: {}", error_msg);
    //     }

    //     let daemon_id = api_response.data
    //         .ok_or_else(|| anyhow::anyhow!("No daemon data in successful response"))?
    //         .daemon
    //         .id;

    //     tracing::info!("Successfully registered with server, assigned ID: {}", daemon_id);

    //     Ok(())
    // }

    /// Get default network from server
    pub async fn get_default_network(&self) -> Result<Network> {
        let server_target = self.config_store.get_server_endpoint().await?;

        tracing::info!("Getting default network from {}", server_target);

        let response = self
            .client
            .get(format!("{}/api/networks/default", server_target))
            .send()
            .await?;

        let status = response.status();
        let api_response: ApiResponse<Network> = response.json().await?;

        if !status.is_success() {
            anyhow::bail!(
                "Get default network failed: {}",
                api_response.error.unwrap_or("Unknown Error".to_string())
            );
        }

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to get default network: {}", error_msg);
        }

        let network = api_response
            .data
            .ok_or_else(|| anyhow!("No network in response"))?;

        Ok(network)
    }

    /// Register daemon with server and return assigned ID
    pub async fn register_with_server(
        &self,
        host_id: Uuid,
        daemon_id: Uuid,
        network_id: Uuid,
    ) -> Result<()> {
        let daemon_ip = self.utils.get_own_ip_address()?;
        let daemon_port = self.config_store.get_port().await?;
        tracing::info!(
            "Registering daemon with ID: {}, host ID: {:?}",
            daemon_id,
            host_id
        );
        let registration_request = DaemonRegistrationRequest {
            daemon_id,
            host_id,
            network_id,
            daemon_ip,
            daemon_port,
        };

        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/daemons/register", server_target))
            .json(&registration_request)
            .send()
            .await?;

        let status = response.status();
        let api_response: ApiResponse<DaemonRegistrationResponse> = response.json().await?;

        if !status.is_success() {
            anyhow::bail!(
                "Registration failed: {}",
                api_response.error.unwrap_or("Unknown Error".to_string())
            );
        }

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown registration error".to_string());
            anyhow::bail!("Registration failed: {}", error_msg);
        }

        let daemon_id = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No daemon data in successful response"))?
            .daemon
            .id;

        tracing::info!(
            "Successfully registered with server, assigned ID: {}",
            daemon_id
        );

        Ok(())
    }
}
