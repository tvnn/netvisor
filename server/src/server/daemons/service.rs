use crate::server::{
    daemons::{
        storage::DaemonStorage,
        types::{
            api::{
                DaemonDiscoveryCancellationRequest, DaemonDiscoveryRequest, DaemonDiscoveryResponse,
            },
            base::Daemon,
        },
    },
    hosts::types::ports::PortBase,
    services::types::endpoints::{ApplicationProtocol, Endpoint},
    shared::types::api::ApiResponse,
};
use anyhow::{Error, Result};
use std::sync::Arc;
use uuid::Uuid;

pub struct DaemonService {
    daemon_storage: Arc<dyn DaemonStorage>,
    client: reqwest::Client,
}

impl DaemonService {
    pub fn new(daemon_storage: Arc<dyn DaemonStorage>) -> Self {
        Self {
            daemon_storage,
            client: reqwest::Client::new(),
        }
    }

    /// Register a new daemon
    pub async fn register_daemon(&self, daemon: Daemon) -> Result<Daemon> {
        self.daemon_storage.create(&daemon).await?;
        Ok(daemon)
    }

    /// Get daemon by ID
    pub async fn get_daemon(&self, id: &Uuid) -> Result<Option<Daemon>> {
        self.daemon_storage.get_by_id(id).await
    }

    /// Get all registered daemons
    pub async fn get_all_daemons(&self) -> Result<Vec<Daemon>> {
        self.daemon_storage.get_all().await
    }

    /// Update daemon heartbeat
    pub async fn receive_heartbeat(&self, mut daemon: Daemon) -> Result<Daemon> {
        daemon.last_seen = chrono::Utc::now();
        self.daemon_storage.update(&daemon).await?;
        Ok(daemon)
    }

    /// Send discovery request to daemon
    pub async fn send_discovery_request(
        &self,
        daemon: &Daemon,
        request: DaemonDiscoveryRequest,
    ) -> Result<(), Error> {
        let endpoint = Endpoint {
            ip: Some(daemon.base.ip),
            port_base: PortBase::new_tcp(daemon.base.port),
            protocol: ApplicationProtocol::Http,
            path: None,
        };

        let response = self
            .client
            .post(format!("{}/api/discovery/initiate", endpoint))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to send discovery request: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<DaemonDiscoveryResponse> = response.json().await?;

        if !api_response.success {
            anyhow::bail!(
                "Failed to send discovery request to daemon {}: {}",
                daemon.id,
                api_response.error.unwrap_or("Unknown error".to_string())
            );
        }

        tracing::info!(
            "Discovery request sent to daemon {} for session {}",
            daemon.id,
            request.session_id
        );
        Ok(())
    }

    pub async fn send_discovery_cancellation(
        &self,
        daemon: &Daemon,
        session_id: Uuid,
    ) -> Result<(), anyhow::Error> {
        let endpoint = Endpoint {
            ip: Some(daemon.base.ip),
            port_base: PortBase::new_tcp(daemon.base.port),
            protocol: ApplicationProtocol::Http,
            path: None,
        };

        let response = self
            .client
            .post(format!("{}/api/discovery/cancel", endpoint))
            .json(&DaemonDiscoveryCancellationRequest { session_id })
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to send discovery cancellation to daemon {}: HTTP {}",
                daemon.id,
                response.status()
            );
        }

        Ok(())
    }
}
