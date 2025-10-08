use crate::{
    daemon::discovery::service::base::DaemonDiscoveryService,
    server::{
        discovery::types::base::EntitySource,
        hosts::types::{
            interfaces::Interface,
            ports::{Port, PortBase},
        },
        services::{
            definitions::netvisor_daemon::NetvisorDaemon,
            types::{
                base::ServiceBase, bindings::Binding, definitions::ServiceDefinition
            },
        },
        utils::base::NetworkUtils,
    },
};
use crate::{
    daemon::utils::base::DaemonUtils,
    server::{
        hosts::types::{
            base::{Host, HostBase},
            targets::HostTarget,
        },
        services::types::base::Service,
    },
};
use anyhow::{Error, Result};
use futures::future::try_join_all;
use std::{
    net::{IpAddr, Ipv4Addr},
    result::Result::Ok,
};
use uuid::Uuid;

impl DaemonDiscoveryService {
    pub async fn run_self_report_discovery(&self) -> Result<(), Error> {
        // Get daemon configuration
        let daemon_id = self.config_store.get_id().await?;
        let binding_address = self.config_store.get_bind_address().await?;
        let binding_ip = IpAddr::V4(binding_address.parse::<Ipv4Addr>()?);
        let (mut interfaces, subnets) = self.utils.scan_interfaces(daemon_id).await?;

        let daemon_bound_subnet_ids: Vec<Uuid> = if binding_address == "0.0.0.0" {
            subnets.iter().map(|s| s.id).collect()
        } else {
            subnets
                .iter()
                .filter(|s| s.base.cidr.contains(&binding_ip))
                .map(|s| s.id)
                .collect()
        };

        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));
        let created_subnets = try_join_all(subnet_futures).await?;

        // Created subnets may differ from discovered if there are existing subnets with the same CIDR, so we need to update interface subnet_id references
        interfaces.iter_mut().for_each(|i| {
            if let Some(subnet) = created_subnets
                .iter()
                .find(|s| s.base.cidr.contains(&i.base.ip_address))
            {
                i.base.subnet_id = subnet.id
            }
        });

        let own_port = Port::new(PortBase::new_tcp(self.config_store.get_port().await?));
        let own_port_id = own_port.id;
        let local_ip = self.utils.get_own_ip_address()?;
        let hostname = self.utils.get_own_hostname();

        // Create host base
        let host_base = HostBase {
            name: hostname
                .clone()
                .unwrap_or(format!("Netvisor-Daemon-{}", local_ip)),
            hostname,
            description: Some("NetVisor daemon".to_string()),
            target: HostTarget::Hostname,
            services: Vec::new(),
            interfaces: interfaces.clone(),
            ports: vec![own_port],
            source: EntitySource::System,
        };

        let host = Host::new(host_base);

        let service_definition = NetvisorDaemon;

        let daemon_service_bound_interfaces: Vec<&Interface> = interfaces
            .iter()
            .filter(|i| daemon_bound_subnet_ids.contains(&i.base.subnet_id))
            .collect();

        let daemon_service = Service::new(ServiceBase {
            name: ServiceDefinition::name(&service_definition).to_string(),
            service_definition: Box::new(service_definition),
            bindings: daemon_service_bound_interfaces
                .iter()
                .map(|i| Binding::new_l4(own_port_id, i.id))
                .collect(),
            host_id: host.id,
        });

        let created_host = self.create_host(host, vec![daemon_service]).await?;

        tracing::info!(
            "Created host with local IP: {}, Hostname: {:?}",
            local_ip,
            created_host.base.hostname
        );

        self.config_store.set_host_id(created_host.id).await?;
        Ok(())
    }
}
