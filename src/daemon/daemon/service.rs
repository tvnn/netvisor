use anyhow::Result;
use std::{net::IpAddr};
use uuid::Uuid;
use crate::{
    daemon::daemon::storage::ConfigStore,
    server::{
        daemons::types::api::{
            DaemonRegistrationRequest, DaemonRegistrationResponse, 
            DaemonDiscoveryRequest, DaemonDiscoveryResponse,
            DaemonTestRequest, DaemonTestResponse, DaemonTestResult,
            DaemonDiscoveryProgress, DaemonNodeReport
        },
        nodes::types::{base::Node, targets::NodeTarget},
        shared::types::api::ApiResponse,
        tests::types::execution::TestResult,
    },
};

pub struct DaemonClientService {
    pub config_store: ConfigStore,
    pub client: reqwest::Client,
}

impl DaemonClientService {
    pub fn new(config_store: ConfigStore) -> Self {
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

        let server_url = self.build_server_url(server_target);
        let response = self
            .client
            .post(format!("{}/api/daemons/register", server_url))
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
        let server_url = self.build_server_url(server_target);
        let response = self
            .client
            .put(format!("{}/api/daemons/{}/heartbeat", server_url, daemon_id))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Heartbeat failed: HTTP {}", response.status());
        }

        tracing::debug!("Heartbeat sent successfully");
        Ok(())
    }

    /// Execute discovery request from server
    pub async fn execute_discovery(&self, request: DaemonDiscoveryRequest) -> Result<DaemonDiscoveryResponse> {
        tracing::info!("Starting discovery session {} on subnets: {:?}", request.session_id, request.target_subnets);
        
        // TODO: Implement actual discovery logic
        // This would:
        // 1. Set up discovery configuration from request
        // 2. Run network discovery in background
        // 3. Report progress via report_discovery_progress()
        // 4. Report discovered devices via report_discovered_node()
        
        Ok(DaemonDiscoveryResponse {
            success: true,
            session_id: request.session_id,
            message: "Discovery started".to_string(),
        })
    }

    /// Execute test request from server  
    pub async fn execute_test(&self, request: DaemonTestRequest) -> Result<DaemonTestResponse> {
        tracing::info!("Starting test execution for session {}", request.session_id);
        
        // TODO: Implement actual test execution logic
        // This would:
        // 1. Extract test configurations from request.node.assigned_tests
        // 2. Execute each test using existing test framework
        // 3. Report results via report_test_result()
        
        Ok(DaemonTestResponse {
            success: true,
            session_id: request.session_id,
            message: "Test execution started".to_string(),
        })
    }

    /// Report test result back to server
    pub async fn report_test_result(&self, server_target: &NodeTarget, session_id: Uuid, result: TestResult) -> Result<()> {
        let server_url = self.build_server_url(server_target);
        let test_result = DaemonTestResult {
            session_id,
            result,
        };

        let response = self
            .client
            .post(format!("{}/api/daemons/test_result", server_url))
            .json(&test_result)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report test result: HTTP {}", response.status());
        }

        tracing::debug!("Test result reported successfully");
        Ok(())
    }

    /// Report discovery progress to server
    pub async fn report_discovery_progress(&self, server_target: &NodeTarget, progress: DaemonDiscoveryProgress) -> Result<()> {
        let server_url = self.build_server_url(server_target);
        let response = self
            .client
            .post(format!("{}/api/daemons/discovery_progress", server_url))
            .json(&progress)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovery progress: HTTP {}", response.status());
        }

        tracing::debug!("Discovery progress reported successfully");
        Ok(())
    }

    /// Report discovered node to server
    pub async fn report_discovered_node(&self, server_target: &NodeTarget, session_id: Uuid, node: Node) -> Result<()> {
        let server_url = self.build_server_url(server_target);
        let node_report = DaemonNodeReport {
            session_id,
            node,
        };

        let response = self
            .client
            .post(format!("{}/api/daemons/discovered_node", server_url))
            .json(&node_report)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report discovered node: HTTP {}", response.status());
        }

        tracing::debug!("Discovered node reported successfully");
        Ok(())
    }

    /// Build server URL from NodeTarget
    fn build_server_url(&self, target: &NodeTarget) -> String {
        match target {
            NodeTarget::IpAddress(config) => {
                if let Some(port) = config.port {
                    format!("http://{}:{}", config.ip, port)
                } else {
                    format!("http://{}", config.ip)
                }
            }
            NodeTarget::Hostname(config) => {
                if let Some(port) = config.port {
                    format!("http://{}:{}", config.hostname, port)
                } else {
                    format!("http://{}", config.hostname)
                }
            }
            NodeTarget::Service(config) => {
                // Use the full service configuration
                format!("{}", config) // Service already implements Display
            }
        }
    }
}