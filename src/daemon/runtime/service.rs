use anyhow::Result;
use std::{net::IpAddr, sync::Arc};
use uuid::Uuid;
use crate::{
    daemon::shared::storage::ConfigStore, server::{
        daemons::types::api::{
            DaemonRegistrationRequest, DaemonRegistrationResponse, 
        },
        nodes::types::targets::NodeTarget,
        shared::types::api::ApiResponse,
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
    pub async fn register_with_server(&mut self, server_target: &NodeTarget, name: String, local_ip: IpAddr, local_port: u16, hostname: Option<String>) -> Result<Uuid> {
        let registration_request = DaemonRegistrationRequest {
            ip: local_ip,
            name: name,
            port: local_port,
            hostname,
        };

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

    /// Send heartbeat to server
    pub async fn send_heartbeat(&self, server_target: &NodeTarget, daemon_id: &Uuid) -> Result<()> {
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