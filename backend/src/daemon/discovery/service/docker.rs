use anyhow::anyhow;
use anyhow::{Error, Result};
use axum::async_trait;
use bollard::{
    query_parameters::{InspectContainerOptions, ListContainersOptions, ListNetworksOptions},
    secret::{ContainerInspectResponse, ContainerSummary, PortTypeEnum},
    Docker,
};
use futures::future::try_join_all;
use itertools::Itertools;
use std::str::FromStr;
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{OnceLock},
};
use tokio_util::sync::CancellationToken;

use crate::daemon::discovery::service::base::InitiatesOwnDiscovery;
use crate::server::hosts::types::interfaces::ALL_INTERFACES_IP;
use crate::server::hosts::types::ports::Port;
use crate::server::services::types::base::Service;
use crate::server::services::types::bindings::Binding;
use crate::server::subnets::types::base::{Subnet, SubnetBase, SubnetType};
use crate::{
    daemon::discovery::service::base::{
        CreatesDiscoveredEntities, DiscoversNetworkedEntities, DiscoveryHandler,
    },
    server::{
        daemons::types::api::DaemonDiscoveryRequest,
        discovery::types::base::EntitySource,
        hosts::types::{
            base::{Host, HostBase},
            interfaces::{Interface, InterfaceBase},
            ports::PortBase,
            targets::HostTarget,
        },
    },
};
use cidr::IpCidr;
use mac_address::MacAddress;
use uuid::Uuid;

pub const DOCKER_PORT: u16 = 2375;

pub struct DockerScanDiscovery {
    docker_client: OnceLock<Docker>,
    host_id: Uuid,
}

impl DockerScanDiscovery {
    pub fn new(host_id: Uuid) -> Self {
        Self {
            docker_client: OnceLock::new(),
            host_id,
        }
    }
}

impl InitiatesOwnDiscovery for DiscoveryHandler<DockerScanDiscovery> {}

impl CreatesDiscoveredEntities for DiscoveryHandler<DockerScanDiscovery> {}

#[async_trait]
impl DiscoversNetworkedEntities for DiscoveryHandler<DockerScanDiscovery> {
    async fn start_discovery_session(
        &self,
        request: DaemonDiscoveryRequest,
        cancel: CancellationToken,
    ) -> Result<(), Error> {

        let docker = self.new_local_docker_client().await?;
        self.discovery_type.docker_client.set(docker.clone()).map_err(|_| anyhow!("Failed to set docker client"))?;

        let container_list = self.get_containers_to_scan().await?;

        self.start_host_discovery(container_list.len(), request)
            .await?;

        let subnets = self.discover_create_subnets().await?;
        let containers= self.get_containers_and_summaries().await?;

        let session_info = self
            .as_ref()
            .session_info
            .get()
            .ok_or_else(|| anyhow!("Session info unavailable"))?;

        // Containers are treated as services on a host with port and interface bindings.
        // This host will be assigned the existing host's ID so any updates will be upserted.
        let mut host = Host::new(HostBase {
            name: "".to_string(),
            hostname: None,
            description: None,
            target: HostTarget::None,
            interfaces: vec![],
            services: vec![],
            ports: vec![],
            source: EntitySource::Discovery(session_info.daemon_id),
        });

        host.id = self.discovery_type.host_id;

        // let scanned_count = self.as_ref().scanned_count.clone();
        // let discovered_count = self.as_ref().discovered_count.clone();
        // let mut last_reported_scan_count: usize = 0;
        // let mut last_reported_discovery_count: usize = 0;

        let services = self.get_services_from_container(&mut host, &subnets, containers, cancel.clone());
        let discovery_result = self.create_host(host, services).await.map(|_| ());

        self.finish_host_discovery(discovery_result, cancel.clone())
            .await?;

        Ok(())
    }

    async fn set_gateway_ips(&self) -> Result<(), Error> {
        let docker = self.discovery_type.docker_client.get().ok_or_else(|| anyhow!("Docker client unavailable"))?;

        let gateway_ips: Vec<IpAddr> = docker.list_networks(None::<ListNetworksOptions>).await?
            .iter()
            .filter_map(|n| {
                if let Some(ipam) = &n.ipam {
                    if let Some(config) = &ipam.config {
                        return Some(config.iter().filter_map(|c| c.gateway.as_ref()).filter_map(|g| g.parse::<IpAddr>().ok()).collect::<Vec<IpAddr>>());
                    }
                }
                None
            })
            .flatten()
            .collect();

        self.as_ref().gateway_ips.set(gateway_ips)
            .map_err(|_| anyhow!("Failed to set gateway_ips"))?;
        Ok(())
    }

    async fn discover_create_subnets(&self) -> Result<Vec<Subnet>, Error> {

        let session_info = self
            .service
            .session_info
            .get()
            .ok_or_else(|| anyhow!("Session info unavailable"))?;

        let subnets = self
            .get_subnets_from_docker_networks(session_info.daemon_id)
            .await?;
        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));
        let subnets = try_join_all(subnet_futures).await?;

        Ok(subnets)
    }
}

impl DiscoveryHandler<DockerScanDiscovery> {
    /// Create a new Docker discovery instance connecting to a remote Docker daemon
    pub async fn new_local_docker_client(&self) -> Result<Docker, Error> {
        
        tracing::debug!("Connecting to Docker daemon");

        let client = Docker::connect_with_local_defaults().map_err(|e| anyhow::anyhow!("Failed to connect to Docker: {}", e))?;

        client.ping().await?;

        Ok(client)
    }

    pub async fn get_containers_to_scan(
        &self,
    ) -> Result<Vec<ContainerSummary>, Error> {
        
        let docker = self.discovery_type.docker_client.get().ok_or_else(|| anyhow!("Docker client unavailable"))?;
        
        docker
            .list_containers(None::<ListContainersOptions>)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_subnets_from_docker_networks(&self, daemon_id: Uuid) -> Result<Vec<Subnet>> {

        let docker = self.discovery_type.docker_client.get().ok_or_else(|| anyhow!("Docker client unavailable"))?;

        let subnets: Vec<Subnet> = docker
            .list_networks(None::<ListNetworksOptions>)
            .await?
            .into_iter()
            .filter_map(|n| {
                let network_name = n.name.clone().unwrap_or("Unknown Network".to_string());
                n.ipam.clone().map(|ipam| (network_name, ipam))
            })
            .filter_map(|(network_name, ipam)| ipam.config.map(|config| (network_name, config)))
            .flat_map(|(network_name, configs)| {
                configs
                    .iter()
                    .filter_map(|c| {
                        if let Some(cidr) = &c.subnet {
                            return Some(Subnet::new(SubnetBase {
                                cidr: IpCidr::from_str(&cidr).ok()?,
                                description: None,
                                name: network_name.clone(),
                                subnet_type: SubnetType::DockerBridge,
                                source: EntitySource::Discovery(daemon_id),
                                hosts: Vec::new(),
                                dns_resolvers: Vec::new(),
                                gateways: Vec::new(),
                                reverse_proxies: Vec::new(),
                            }))
                        }
                        None
                    })
                    .collect::<Vec<Subnet>>()
            })
            .collect();

        Ok(subnets)
    }
    
    pub async fn get_containers_and_summaries(&self) -> Result<Vec<(ContainerInspectResponse, ContainerSummary)>, Error> {
        let docker = self.discovery_type.docker_client.get().ok_or_else(|| anyhow!("Docker client unavailable"))?;

        let container_summaries = self.get_containers_to_scan().await?;

        let containers_to_inspect: Vec<_> = container_summaries
            .iter()
            .filter_map(|c| {
                if let Some(id) = &c.id {
                    return Some(
                        docker
                            .inspect_container(id, None::<InspectContainerOptions>),
                    );
                }
                None
            })
            .collect();

        let inspected_containers: Vec<ContainerInspectResponse> =
            try_join_all(containers_to_inspect).await?;

        Ok(inspected_containers.into_iter().zip(container_summaries).collect())

    }
    
    pub fn get_services_from_container(
        &self,
        host: &mut Host,
        subnets: &[Subnet],
        containers: Vec<(ContainerInspectResponse, ContainerSummary)>,
        _cancel: CancellationToken,
    ) -> Vec<Service> {

        containers
            .into_iter()
            .flat_map(|(container, container_summary)| {
                
                if container.id != container_summary.id {
                    tracing::warn!("Container inspection failure; inspected container does not match container summary");
                    return vec!()
                }

                let (host_ip_to_own_ports, host_ip_to_container_ports, container_port_to_host_port_and_ip) = self.get_ports_from_container_summary(container_summary);

                let host_ports: Vec<Port> = host_ip_to_own_ports
                    .values()
                    .into_iter()
                    .flat_map(|p| p.into_iter().map(|pb| Port::new(pb.clone())))
                    .dedup_by(|x,y| x.base == y.base)
                    .collect();

                host.base.ports.extend(host_ports);

                if let Some(interfaces_and_ports) = self.get_interfaces_from_container(&container, subnets, &host_ip_to_own_ports, &host_ip_to_container_ports) {

                    let services: Vec<Service> = interfaces_and_ports
                        .iter()
                        .flat_map(|(_host_ports, container_ports, interface, subnet)| {
                                                    
                            host.base.interfaces.push(interface.clone());
    
                            let mut services = self.discover_host_services(host, interface, &container_ports, &vec!(), subnet, &false).unwrap_or(vec!());

                            // Add bindings for host ports
                            services.iter_mut().for_each(|s| {
                                let container_ports: Vec<&Port> = s.base.bindings.iter().filter_map(|b| b.port_id()).filter_map(|id| host.get_port(&id)).collect();
                                let host_port_bases_and_ips: Vec<&(PortBase, IpAddr)> = container_ports.iter().filter_map(|p| container_port_to_host_port_and_ip.get(&p.base)).flatten().collect();

                                host_port_bases_and_ips.iter().for_each(|(pb,ip)| {

                                    let port = host.base.ports.iter().find(|p| p.base == *pb);
                                    let interface = host.base.interfaces.iter().find(|i| i.base.ip_address == *ip);

                                    match (port, interface) {
                                        (Some(port), Some(interface)) => {

                                            s.base.bindings.push(
                                                Binding::new_l4(port.id, Some(interface.id))
                                            );
                                        },
                                        (Some(port), None) if *ip == ALL_INTERFACES_IP => {

                                            s.base.bindings.push(
                                                Binding::new_l4(port.id, None)
                                            );
                                        },
                                        _ => ()
                                    }

                                });
                            });

                            services

                        })
                        .collect();

                    return services
                }
                      
                vec!()    
            })
            .collect()

    }

    fn get_ports_from_container_summary(
        &self, 
        container_summary: ContainerSummary
    ) -> (HashMap<IpAddr, Vec<PortBase>>, HashMap<IpAddr, Vec<PortBase>>, HashMap<PortBase, Vec<(PortBase, IpAddr)>>) {

        let mut host_ip_to_own_ports: HashMap<IpAddr, Vec<PortBase>> = HashMap::new();
        let mut host_ip_to_container_ports: HashMap<IpAddr, Vec<PortBase>> = HashMap::new();
        let mut container_port_to_host_port_and_ip: HashMap<PortBase, Vec<(PortBase, IpAddr)>> = HashMap::new();

        if let Some(ports) = &container_summary.ports {
            ports
                .iter()
                .for_each(|p| {
                    let ip = p.ip.clone().unwrap_or_default().parse::<IpAddr>().ok();

                    match (p.typ, ip) {
                        (Some(port_type @ (PortTypeEnum::TCP | PortTypeEnum::UDP)), Some(ip)) => {
                            let private_port = match port_type {
                                PortTypeEnum::TCP => PortBase::new_tcp(p.private_port),
                                PortTypeEnum::UDP => PortBase::new_udp(p.private_port),
                                _ => unreachable!("Already matched TCP/UDP in outer pattern"),
                            };

                            host_ip_to_container_ports
                                .entry(ip)
                                .or_insert_with(Vec::new)
                                .push(private_port.clone());

                            if let Some(public) = p.public_port {
                                let public_port = match port_type {
                                    PortTypeEnum::TCP => PortBase::new_tcp(public),
                                    PortTypeEnum::UDP => PortBase::new_udp(public),
                                    _ => unreachable!("Already matched TCP/UDP in outer pattern"),
                                };
                                
                                host_ip_to_own_ports
                                    .entry(ip)
                                    .or_insert_with(Vec::new)
                                    .push(public_port.clone());
                                
                                container_port_to_host_port_and_ip
                                    .entry(private_port)
                                    .or_insert_with(Vec::new)
                                    .push((public_port, ip));
                            }
                        }
                        _ => ()
                    }
                });
            
            return (host_ip_to_own_ports, host_ip_to_container_ports, container_port_to_host_port_and_ip)
        };

        (host_ip_to_own_ports, host_ip_to_container_ports, container_port_to_host_port_and_ip)
    }

    fn get_interfaces_from_container(
        &self, 
        container: &ContainerInspectResponse, 
        subnets: &[Subnet], 
        host_ip_to_own_ports: &HashMap<IpAddr, Vec<PortBase>>, 
        host_ip_to_container_ports: &HashMap<IpAddr, Vec<PortBase>>
    ) -> Option<Vec<(Vec<PortBase>, Vec<PortBase>, Interface, Subnet)>> {
        if let Some(network_settings) = &container.network_settings {
            if let Some(networks) = &network_settings.networks {
                
                let ports_and_interfaces: Vec<(Vec<PortBase>, Vec<PortBase>, Interface, Subnet)> = networks
                    .iter()
                    .filter_map(|(network_name, endpoint)| {

                        // Parse interface if IP
                        if let Some(ip_string) = &endpoint.ip_address {
                            let ip_address = ip_string.parse::<IpAddr>().ok();
                            if let Some(ip_address) = ip_address {
                                if let Some(subnet) = subnets.iter().find(|s| s.base.cidr.contains(&ip_address)) {
                                    // Parse MAC address
                                    let mac_address = if let Some(mac_string) = &endpoint.mac_address {
                                        mac_string.parse::<MacAddress>().ok()
                                    } else {
                                        None
                                    };

                                    let host_ports_on_interface = host_ip_to_own_ports.get(&ip_address).cloned().unwrap_or(Vec::new());
                                    let container_ports_on_interface = host_ip_to_container_ports.get(&ip_address).cloned().unwrap_or(Vec::new());

                                    let host_ports_on_all_interfaces = host_ip_to_own_ports.get(&ALL_INTERFACES_IP).cloned().unwrap_or(Vec::new());
                                    let container_ports_on_all_interfaces = host_ip_to_container_ports.get(&ALL_INTERFACES_IP).cloned().unwrap_or(Vec::new());

                                    return Some((
                                        [host_ports_on_interface, host_ports_on_all_interfaces].concat(),
                                        [container_ports_on_interface, container_ports_on_all_interfaces].concat(),
                                        Interface::new(InterfaceBase { 
                                            subnet_id: subnet.id, 
                                            ip_address, 
                                            mac_address, 
                                            name: Some(network_name.to_owned())
                                        }),
                                        subnet.clone()
                                    ));
                                }
                            }
                        }

                        None
                    })
                    .collect();

                return Some(ports_and_interfaces);
            }
        }
        None
    }
}
