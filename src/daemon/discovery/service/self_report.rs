use anyhow::{Error, Result};
use futures::future::try_join_all;
use crate::{daemon::{discovery::service::base::DaemonDiscoveryService}, server::{services::types::{base::ServiceBase, ports::Port, types::ServiceType}, utils::base::NetworkUtils}};
use std::{result::Result::Ok};
use crate::{
    daemon::{utils::base::{DaemonUtils}},
    server::{
        hosts::types::{
            base::{Host, HostBase}, targets::{HostTarget}
        }, services::types::{base::{Service}}, shared::types::{metadata::TypeMetadataProvider}
    },
};

impl DaemonDiscoveryService {

    pub async fn run_self_report_discovery(&self) -> Result<(), Error> {    
        // Get daemon configuration
        let daemon_id = self.config_store.get_id().await?;
        let (interfaces, subnets) = self.utils.scan_interfaces(daemon_id).await?;

        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));
        let subnets = try_join_all(subnet_futures).await?;
        let server_ip = self.config_store.get_server_ip().await?;
        let server_subnet_interface = if let Some(server_ip) = server_ip {
            if let Some(server_subnet) = subnets.iter().find(|s| s.base.cidr.contains(&server_ip)).cloned() {
                interfaces.iter().find_map(|i| if i.base.subnet_id == server_subnet.id {Some(i)} else {None})
            } else {
                None
            }
        } else {
            None
        };

        let (target, interface_bindings) = if let Some(interface) = server_subnet_interface {
            (HostTarget::Interface(interface.id), vec!(interface.id))
        } else if !interfaces.is_empty() {
            (HostTarget::Interface(interfaces[0].id), vec!(interfaces[0].id))
        } else {
            (HostTarget::Hostname, vec!())
        };

        let own_port = Port::new_tcp(self.config_store.get_port().await?);
        let local_ip = self.utils.get_own_ip_address()?;
        let hostname = self.utils.get_own_hostname();

        // Create host base
        let host_base = HostBase {
            name: hostname.clone().unwrap_or(format!("Netvisor-Daemon-{}", local_ip)),
            hostname,
            description: Some("NetVisor daemon".to_string()),
            target,
            services: Vec::new(),
            interfaces,
            open_ports: Vec::new(),
        };

        let host = Host::new(host_base);

        let service_type = ServiceType::NetvisorDaemon;
        let daemon_service = Service::new(ServiceBase { 
            name: service_type.display_name().to_string(), 
            service_type,
            ports: vec!(own_port),
            host_id: host.id,
            interface_bindings,
            groups: Vec::new()
        });
        
        let created_host = self.create_host(host, vec!(daemon_service)).await?;

        tracing::info!("Created host with local IP: {}, Hostname: {:?}", local_ip, created_host.base.hostname);

        self.config_store.set_host_id(created_host.id).await?;
        Ok(())
    }
}