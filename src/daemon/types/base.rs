use std::net::IpAddr;

use reqwest::Response;
use uuid::Uuid;

use crate::server::{daemons::types::api::{DaemonRegistrationRequest, DaemonRegistrationResponse}, shared::types::api::{ApiResponse, ApiResult}};

#[derive(Clone)]
pub struct DaemonState {
    pub id: Uuid,
    pub ip: IpAddr,
    pub port: u16,
    pub server_url: IpAddr,
    pub client: reqwest::Client,
    pub hostname: Option<String>,
}

impl DaemonState {
    fn new(server_url: IpAddr, ip: IpAddr, port: u16, hostname: Option<String>) -> Self {
        Self {
            server_url,
            client: reqwest::Client::new(),
            ip,
            port,
            hostname,
        }
    }

    async fn register_with_server(&self) -> anyhow::Result<()> {
        let registration_request = DaemonRegistrationRequest {
            ip: self.ip.clone(),
            port: self.port,
            hostname: self.hostname.clone(),
        };

        let response: Response<ApiResult<DaemonRegistrationResponse>> = self
            .client
            .post(format!("{}/api/daemons/register", self.server_url))
            .json(&registration_request)
            .send()
            .await?;

        if response.success {
            let reg_response: ApiResponse<DaemonRegistrationResponse> = response.json().await?;
            if reg_response.data.success {
                tracing::info!("Successfully registered with server");
                Ok(())
            } else {
                anyhow::bail!("Registration failed: {}", reg_response.data.message);
            }
        } else {
            anyhow::bail!("Failed to register with server: HTTP {}", response.status());
        }
    }

    async fn discover_self_and_create_node(&self) -> anyhow::Result<()> {
        tracing::info!("Starting self-discovery...");
        
        let discovery_config = DiscoveryConfig {
            target_subnets: vec![format!("{}/32", self.my_ip)],
            discovery_depth: DiscoveryDepth::Standard,
            include_services: true,
            snmp_communities: vec!["public".to_string()],
            max_concurrent: 1,
            timeout_ms: 5000,
            port_scan_enabled: true,
            common_ports: vec![22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3389, 5900, 8080, 8443],
        };

        let discovery = NetworkDiscovery::new();
        if let Err(e) = discovery.start_discovery(discovery_config).await {
            tracing::warn!("Self-discovery failed: {}", e);
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
        let discovered = discovery.get_discovered_devices().await;
        
        let mut capabilities = Vec::new();
        
        // Add discovered capabilities based on services
        if let Some(discovered_self) = discovered.iter().find(|d| d.ip == self.my_ip) {
            tracing::info!("Found self in discovery results with {} services", discovered_self.services.len());
            
            for service in &discovered_self.services {
                match service.as_str() {
                    s if s.contains("ssh") => {
                        // Could add SSH capability here when implemented
                    }
                    s if s.contains("http") => {
                        // Could add HTTP capability here when implemented
                    }
                    _ => {}
                }
            }
        }
        
        // Always add daemon capability
        capabilities.push(NodeCapability::DaemonService(DaemonServiceCapability::new(self.daemon_id)));
        
        let node = Node::new(
            self.hostname.clone().unwrap_or_else(|| format!("daemon-{}", &self.daemon_id.to_string()[..8])),
            Some(NodeTarget::IpAddress(netfrog_server::components::nodes::types::targets::IpAddressTarget {
                ip: self.my_ip.clone(),
                port: Some(self.my_port as i64),
            })),
            capabilities,
        );

        let response = self
            .client
            .post(format!("{}/api/nodes", self.server_url))
            .json(&node)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("Successfully created daemon node on server");
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to create daemon node: HTTP {} - {}", status, text);
        }
    }

    pub async fn send_heartbeat(&self) -> anyhow::Result<()> {
        let response = self
            .client
            .post(format!("{}/api/daemons/{}/heartbeat", self.server_url, self.daemon_id))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Heartbeat failed: HTTP {}", response.status());
        }

        Ok(())
    }

    async fn report_test_result(&self, session_id: Uuid, result: TestResult) -> anyhow::Result<()> {
        let test_result = DaemonTestResult {
            session_id,
            result,
        };

        let response = self
            .client
            .post(format!("{}/api/tests/result", self.server_url))
            .json(&test_result)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report test result: HTTP {}", response.status());
        }

        Ok(())
    }
}
