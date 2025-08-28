use anyhow::{Error, Result};
use uuid::Uuid;
use std::sync::Arc;
use crate::server::{daemons::{
        storage::DaemonStorage, 
        types::{
            api::{
                DaemonDiscoveryCancellationRequest, DaemonDiscoveryRequest, DaemonDiscoveryResponse, DaemonTestRequest, DaemonTestResult
            }, base::Daemon
        }
    }, nodes::storage::NodeStorage, shared::types::api::ApiResponse};

pub struct DaemonService {
    daemon_storage: Arc<dyn DaemonStorage>,
    node_storage: Arc<dyn NodeStorage>,
    client: reqwest::Client,
}

impl DaemonService {
    pub fn new(daemon_storage: Arc<dyn DaemonStorage>, node_storage: Arc<dyn NodeStorage>,) -> Self {
        Self {
            daemon_storage,
            node_storage,
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
    pub async fn send_discovery_request(&self, daemon: &Daemon, request: DaemonDiscoveryRequest) -> Result<(), Error> {        
        
        let daemon_node = match self.node_storage.get_by_id(&daemon.base.node_id).await? {
            Some(node) => node,
            None => return Err(Error::msg(format!("Node '{}' for daemon {} not found", daemon.base.node_id, daemon.id)))
        };
        
        let response = self.client
            .post(format!("{}/api/discovery/initiate", daemon_node.base.target.to_string()))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to send discovery request: HTTP {}", response.status());
        }

        let api_response: ApiResponse<DaemonDiscoveryResponse> = response.json().await?;

        if !api_response.success {
            anyhow::bail!("Failed to send discovery request to daemon {}: {}", daemon.id, api_response.error.unwrap_or("Unknown error".to_string()));
        }

        tracing::info!("Discovery request sent to daemon {} for session {}", daemon.id, request.session_id);
        Ok(())
    }

    /// Send test execution request to daemon
    pub async fn send_test_request(&self, daemon: &Daemon, request: DaemonTestRequest) -> Result<()> {        
        
        let daemon_node = match self.node_storage.get_by_id(&daemon.base.node_id).await? {
            Some(node) => node,
            None => return Err(Error::msg(format!("Node '{}' for daemon {} not found", daemon.base.node_id, daemon.id)))
        };
        
        let response = self.client
            .post(format!("{}/api/tests/execute", daemon_node.base.target.to_string()))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to send test request to daemon {}: HTTP {}", 
                         daemon.id, response.status());
        }

        tracing::info!("Test request sent to daemon {} for session {}", 
                      daemon.id, request.session_id);
        Ok(())
    }

    pub async fn send_discovery_cancellation(&self, daemon: &Daemon, session_id: Uuid) -> Result<(), anyhow::Error> {

        let daemon_node = match self.node_storage.get_by_id(&daemon.base.node_id).await? {
            Some(node) => node,
            None => return Err(Error::msg(format!("Node '{}' for daemon {} not found", daemon.base.node_id, daemon.id)))
        };

        let response = self.client
            .post(format!("{}/api/discovery/cancel", daemon_node.base.target.to_string()))
            .json(&DaemonDiscoveryCancellationRequest { session_id })
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to send discovery cancellation to daemon {}: HTTP {}", 
                         daemon.id, response.status());
        }

        Ok(())
    }

    /// Process test result from daemon
    pub async fn process_test_result(&self, test_result: DaemonTestResult) -> Result<()> {
        tracing::info!(
            "Test result from session {}: {} - {}", 
            test_result.session_id,
            test_result.result.success,
            test_result.result.message
        );

        // TODO: Implement actual result processing
        // This could:
        // 1. Update node status based on result
        // 2. Store in diagnostic execution results
        // 3. Trigger follow-up actions based on result
        // 4. Update monitoring dashboards

        Ok(())
    }
}