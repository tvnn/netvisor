use anyhow::{Error, Result};
use futures::future::try_join_all;
use crate::{daemon::discovery::service::base::DaemonDiscoveryService, server::{services::types::ports::Port, utils::base::NetworkUtils}};
use std::result::Result::Ok;
use crate::{
    daemon::{utils::base::{DaemonUtils}},
    server::{
        nodes::types::{
            base::{Node, NodeBase}, targets::{IpAddressTargetConfig, NodeTarget}
        }, services::types::{base::{Service, ServiceDiscriminants}}, shared::types::{metadata::TypeMetadataProvider}
    },
};

impl DaemonDiscoveryService {

    pub async fn run_self_report_discovery(&self) -> Result<(), Error> {    
        // Get daemon configuration
        let daemon_id = self.config_store.get_id().await?;
        let (node_subnet_memberships, subnets) = self.utils.scan_subnets(daemon_id).await?;

        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));

        try_join_all(subnet_futures).await?;

        let own_port = Port::new_tcp(self.config_store.get_port().await?);
        
        let local_ip = self.utils.get_own_ip_address()?;
        let hostname = self.utils.get_own_hostname();
        
        // Create node base
        let node_base = NodeBase {
            name: hostname.clone().unwrap_or(format!("Netvisor-Daemon-{}", local_ip)),
            hostname,
            description: Some("NetVisor daemon for network diagnostics".to_string()),
            target: NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: local_ip,
            }),
            services: Vec::new(),
            subnets: node_subnet_memberships,
            node_groups: Vec::new(),
            open_ports: Vec::new(),
        };

        let mut node = Node::new(node_base);

        node.add_service(Service::NetvisorDaemon { 
            name: ServiceDiscriminants::NetvisorDaemon.display_name().to_string(), 
            ports: vec!(own_port),
            daemon_id
        });

        let created_node = self.create_node(&node).await?;

        tracing::info!("Created node with local IP: {}, Hostname: {:?}", local_ip, node.base.hostname);

        self.config_store.set_node_id(created_node.id).await?;
        Ok(())
    }
}