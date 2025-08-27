use anyhow::{Error, Result};
use uuid::Uuid;
use std::sync::Arc;
use crate::server::{daemons::{
        storage::DaemonStorage, 
        types::{
            api::{
                DaemonDiscoveryProgress, DaemonDiscoveryRequest, DaemonDiscoveryResponse, DaemonNodeReport, DaemonTestRequest, DaemonTestResult
            }, base::Daemon
        }
    }, shared::types::api::ApiResponse};

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
    pub async fn send_discovery_request(&self, daemon: &Daemon, request: DaemonDiscoveryRequest) -> Result<(), Error> {        
        let response: ApiResponse<DaemonDiscoveryResponse> = self.client
            .post(format!("{}/discover", daemon.endpoint_url()))
            .json(&request)
            .send()
            .await?
            .error_for_status()?  
            .json()
            .await?;

        if !response.success {
            anyhow::bail!("Failed to send discovery request to daemon {}: {}", daemon.id, response.error.unwrap_or("Unknown error".to_string()));
        }

        tracing::info!("Discovery request sent to daemon {} for session {}", daemon.id, request.session_id);
        Ok(())
    }

    /// Send test execution request to daemon
    pub async fn send_test_request(&self, daemon: &Daemon, request: DaemonTestRequest) -> Result<()> {        
        let response = self.client
            .post(format!("{}/execute_test", daemon.endpoint_url()))
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

    /// Check daemon health
    pub async fn check_daemon_health(&self, daemon: &Daemon) -> Result<bool> {
        
        match self.client
            .get(format!("{}/health", daemon.endpoint_url()))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await 
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Get list of healthy daemons
    pub async fn get_healthy_daemons(&self) -> Result<Vec<Daemon>> {
        let all_daemons = self.get_all_daemons().await?;
        let mut healthy_daemons = Vec::new();

        for daemon in all_daemons {
            if self.check_daemon_health(&daemon).await.unwrap_or(false) {
                healthy_daemons.push(daemon);
            }
        }

        Ok(healthy_daemons)
    }

    /// Process discovery progress update from daemon
    pub async fn process_discovery_progress(&self, progress: DaemonDiscoveryProgress) -> Result<()> {
        tracing::info!(
            "Discovery progress from session {}: {}/{} completed, {} discovered", 
            progress.session_id,
            progress.completed,
            progress.total,
            progress.discovered_count
        );

        // TODO: Implement actual progress tracking
        // This could:
        // 1. Update active discovery session status
        // 2. Notify WebSocket clients of progress
        // 3. Store progress in database for later retrieval

        Ok(())
    }

    /// Process discovered node from daemon
    pub async fn process_discovered_node(&self, node_report: DaemonNodeReport) -> Result<()> {
        tracing::info!(
            "Discovered node from session {}: {} ({})", 
            node_report.session_id,
            node_report.node.base.name,
            node_report.node.base.target
        );

        // TODO: Implement actual node processing
        // This could:
        // 1. Auto-create node in database
        // 2. Queue for user review and approval  
        // 3. Merge with existing node if duplicate detected
        // 4. Trigger automatic capability detection

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