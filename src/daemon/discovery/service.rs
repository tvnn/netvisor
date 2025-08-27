use std::sync::Arc;

use anyhow::Error;
use cidr::IpCidr;
use strum::IntoEnumIterator;

use crate::{daemon::shared::storage::ConfigStore, server::daemons::types::api::DaemonDiscoveryRequest};

pub struct DaemonDiscoveryService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
}

impl DaemonDiscoveryService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        Self {
            config_store,
            client: reqwest::Client::new(),
        }
    }

    /// Background task for discovery session with progress reporting
    pub async fn run_discovery_session(&self, request: DaemonDiscoveryRequest) -> Result<(), Error> {

        let session_id = request.session_id;

        tracing::info!("Starting discovery session {}", session_id);

        let daemon_subnet = self.get_daemon_subnet().map_err(|e| -> Error::msg(format!("Could not determine daemon subnet: {}", e)));
        tracing::info!("Found daemon subnet {}", daemon_subnet);
        
        // 0. Always discover self
        if let Ok(self_node) = self.discover_self_as_node().await {
            self.report_discovered_node(&session_id, &self_node).await?;
        }
        
        let active_hosts = self.scan_subnet_for_hosts(&daemon_subnet).await?;
        
        for host_ip in active_hosts {
            
            // 1. Gather basic info
            let hostname = reverse_dns_lookup(host_ip).await.ok();
            let device_info = snmp_device_info(host_ip, &["public", "private"]).await.ok();
            let open_ports = port_scan(host_ip, &DiscoveryPort::iter(), DISCOVERY_TIMEOUT_PER_HOST).await?;

            // 2. Create node
            
            let node = Node::from_name( hostname.clone().unwrap_or_else(|| format!("Device-{}", host_ip)) );

            node.base.hostname = hostname;
            node.base.mac_address = arp_lookup(host_ip).await.ok();
            node.base.status = NodeStatus::Unknown;
            node.base.target = NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: host_ip,
                port: None,
            });
            node.base.node_type = NodeType::UnknownDevice;
            node.base.description = device_info.as_ref().map(|info| format!("Discovered device: {}", info.description));
            node.base.discovery_status = Some(DiscoveryStatus::Discovered {
                session_id: config.session_id,
                discovered_at: Utc::now(),
            });
            node.base.subnets = Vec::from(daemon_subnet);

            // 3. Detect capabilities from ports
            for port in &open_ports {
                node.add_capability_from_port(port);
            }
            
            // 6. Immediately report discovered node to server via SSE
            self.report_discovered_node(&session_id, &node).await?;
        }
        
        Ok(())
        
        tracing::info!("Discovery session {} completed", &session_id);
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


    fn scan_subnet_for_hosts(&self) -> () {
    }

    fn discover_self_as_node(&self) -> () {
    }

    fn get_daemon_subnet(&self) -> Result<IpCidr, Error> {
        let interfaces = get_network_interfaces()?;
        let primary_interface = interfaces.iter()
            .find(|iface| !iface.is_loopback() && iface.is_up())?
            .ok_or_else(|| Error::msg("No active network interface found"))?;
        Ok(primary_interface.subnet)
    }
}