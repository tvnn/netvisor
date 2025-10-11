use anyhow::anyhow;
use anyhow::{Error, Result};
use axum::async_trait;
use bollard::{
    query_parameters::{InspectContainerOptions, ListContainersOptions, ListNetworksOptions},
    secret::{ContainerInspectResponse, ContainerSummary, PortTypeEnum},
    Docker,
};
use futures::future::try_join_all;
use futures::stream::{self, StreamExt};
use std::str::FromStr;
use std::sync::Arc;
use std::{collections::HashMap, net::IpAddr, sync::OnceLock};
use tokio_util::sync::CancellationToken;

use crate::daemon::discovery::service::base::{InitiatesOwnDiscovery, CONCURRENT_SCANS};
use crate::daemon::discovery::types::base::DiscoverySessionUpdate;
use crate::daemon::utils::base::DaemonUtils;
use crate::server::hosts::types::interfaces::ALL_INTERFACES_IP;
use crate::server::hosts::types::ports::Port;
use crate::server::services::definitions::docker_container::DockerContainer;
use crate::server::services::types::base::ServiceDiscoveryBaselineParams;
use crate::server::services::types::bindings::Binding;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::subnets::types::base::{Subnet, SubnetBase, SubnetType};
use crate::{
    daemon::discovery::service::base::{
        CreatesDiscoveredEntities, DiscoversNetworkedEntities, DiscoveryHandler,
    },
    server::{
        daemons::types::api::DaemonDiscoveryRequest,
        discovery::types::base::EntitySource,
        hosts::types::{
            base::Host,
            interfaces::{Interface, InterfaceBase},
            ports::PortBase,
        },
    },
};
use cidr::IpCidr;
use mac_address::MacAddress;
use uuid::Uuid;

pub const DOCKER_PORT: u16 = 2375;

type IpPortHashMap = HashMap<IpAddr, Vec<PortBase>>;

pub struct DockerScanDiscovery {
    docker_client: OnceLock<Docker>,
    host_id: Uuid,
}

impl Default for DockerScanDiscovery {
    fn default() -> Self {
        Self {
            docker_client: OnceLock::new(),
            host_id: Uuid::nil(),
        }
    }
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
        let daemon_id = self.as_ref().config_store.get_id().await?;
        let docker = self.new_local_docker_client().await?;
        self.discovery_type
            .docker_client
            .set(docker.clone())
            .map_err(|_| anyhow!("Failed to set docker client"))?;

        let container_list = self.get_containers_to_scan().await?;

        self.start_discovery(container_list.len(), request).await?;

        let subnets = self.discover_create_subnets().await?;
        let (mut host_interfaces, _) = self.as_ref().utils.scan_interfaces(daemon_id).await?;
        let containers = self.get_containers_and_summaries().await?;
        let containers_interfaces_and_subnets =
            self.get_container_interfaces(&containers, &subnets, &mut host_interfaces);

        let discovery_result = self
            .scan_and_process_containers(
                cancel.clone(),
                containers,
                &containers_interfaces_and_subnets,
            )
            .await
            .map(|_| ());

        self.finish_discovery(discovery_result, cancel.clone())
            .await?;

        Ok(())
    }

    async fn get_gateway_ips(&self) -> Result<Vec<IpAddr>, Error> {
        let docker = self
            .discovery_type
            .docker_client
            .get()
            .ok_or_else(|| anyhow!("Docker client unavailable"))?;

        let gateway_ips: Vec<IpAddr> = docker
            .list_networks(None::<ListNetworksOptions>)
            .await?
            .iter()
            .filter_map(|n| {
                if let Some(ipam) = &n.ipam {
                    if let Some(config) = &ipam.config {
                        return Some(
                            config
                                .iter()
                                .filter_map(|c| c.gateway.as_ref())
                                .filter_map(|g| g.parse::<IpAddr>().ok())
                                .collect::<Vec<IpAddr>>(),
                        );
                    }
                }
                None
            })
            .flatten()
            .collect();

        Ok(gateway_ips)
    }

    async fn discover_create_subnets(&self) -> Result<Vec<Subnet>, Error> {
        let daemon_id = self.as_ref().config_store.get_id().await?;

        let (_, host_subnets) = self.as_ref().utils.scan_interfaces(daemon_id).await?;

        let docker_subnets = self.get_subnets_from_docker_networks(daemon_id).await?;

        let subnets = [host_subnets, docker_subnets].concat();

        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));
        let subnets = try_join_all(subnet_futures).await?;

        Ok(subnets)
    }
}

impl DiscoveryHandler<DockerScanDiscovery> {
    /// Create a new Docker discovery instance connecting to a remote Docker daemon
    pub async fn new_local_docker_client(&self) -> Result<Docker, Error> {
        tracing::debug!("Connecting to Docker daemon");

        let client = Docker::connect_with_local_defaults()
            .map_err(|e| anyhow::anyhow!("Failed to connect to Docker: {}", e))?;

        client.ping().await?;

        Ok(client)
    }

    async fn scan_and_process_containers(
        &self,
        cancel: CancellationToken,
        containers: Vec<(ContainerInspectResponse, ContainerSummary)>,
        containers_interfaces_and_subnets: &HashMap<String, Vec<(Interface, Subnet)>>,
    ) -> Result<Vec<Host>> {
        let session = self.as_ref().get_session().await?;
        let scanned_count = session.scanned_count.clone();
        let discovered_count = session.discovered_count.clone();

        self.report_discovery_update(DiscoverySessionUpdate::scanning(0, 0))
            .await?;

        // Process containers concurrently using streams
        let results = stream::iter(containers.into_iter())
            .map(|(container, container_summary)| {
                let scanned = scanned_count.clone();
                let discovered = discovered_count.clone();
                let cancel = cancel.clone();

                async move {
                    self.process_single_container(
                        containers_interfaces_and_subnets,
                        container,
                        container_summary,
                        scanned,
                        discovered,
                        cancel,
                    )
                    .await
                }
            })
            .buffer_unordered(CONCURRENT_SCANS); // Use same concurrency as network discovery

        let mut stream_pin = Box::pin(results);
        let mut last_reported_scan_count: usize = 0;
        let mut last_reported_discovery_count: usize = 0;
        let mut all_container_data = Vec::new();

        while let Some(result) = stream_pin.next().await {
            if cancel.is_cancelled() {
                tracing::warn!("Docker discovery session was cancelled");
                return Err(Error::msg("Docker discovery session was cancelled"));
            }

            match result {
                Ok(Some(container_data)) => all_container_data.push(container_data),
                Ok(None) => {}
                Err(e) => tracing::warn!("Error processing container: {}", e),
            }

            (last_reported_scan_count, last_reported_discovery_count) = self
                .periodic_scan_update(5, last_reported_scan_count, last_reported_discovery_count)
                .await?;
        }

        Ok(all_container_data)
    }

    async fn process_single_container(
        &self,
        containers_interfaces_and_subnets: &HashMap<String, Vec<(Interface, Subnet)>>,
        container: ContainerInspectResponse,
        container_summary: ContainerSummary,
        scanned_count: Arc<std::sync::atomic::AtomicUsize>,
        discovered_count: Arc<std::sync::atomic::AtomicUsize>,
        cancel: CancellationToken,
    ) -> Result<Option<Host>> {
        scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        if cancel.is_cancelled() {
            return Err(Error::msg("Discovery was cancelled"));
        }

        if container.id != container_summary.id {
            tracing::warn!("Container inspection failure; inspected container does not match container summary");
            return Ok(None);
        }

        let container_name = Some(
            container
                .name
                .clone()
                .unwrap_or(DockerContainer.name().to_string())
                .trim_start_matches("/")
                .to_string(),
        );

        let empty_vec_ref: &Vec<_> = &Vec::new();

        let container_interfaces_and_subnets = if let Some(id) = container.id {
            containers_interfaces_and_subnets
                .get(&id)
                .unwrap_or(empty_vec_ref)
        } else {
            empty_vec_ref
        };

        let (host_ip_to_host_ports, container_ips_to_container_ports) =
            self.get_ports_from_container(container_summary, container_interfaces_and_subnets);

        for (interface, subnet) in container_interfaces_and_subnets {
            if cancel.is_cancelled() {
                return Err(Error::msg("Discovery was cancelled"));
            }

            // Scan endpoints for this container interface
            tracing::debug!(
                "Scanning endpoints for container at {}",
                interface.base.ip_address
            );

            let endpoint_responses = self
                .as_ref()
                .utils
                .scan_endpoints(interface.base.ip_address, cancel.clone())
                .await
                .unwrap_or_else(|e| {
                    tracing::debug!(
                        "Failed to scan endpoints for {}: {}",
                        interface.base.ip_address,
                        e
                    );
                    vec![]
                });

            if !endpoint_responses.is_empty() {
                tracing::info!(
                    "Found {} endpoint responses for container at {}",
                    endpoint_responses.len(),
                    interface.base.ip_address
                );
            }

            let empty_vec_ref: &Vec<_> = &Vec::new();
            let container_ports_on_interface = container_ips_to_container_ports
                .get(&interface.base.ip_address)
                .unwrap_or(empty_vec_ref);

            if let Ok(Some((mut host, mut services))) = self
                .process_host(
                    ServiceDiscoveryBaselineParams {
                        subnet,
                        interface,
                        open_ports: container_ports_on_interface,
                        endpoint_responses: &endpoint_responses,
                        host_has_docker_client: &false,
                        docker_container_name: &container_name,
                    },
                    None,
                )
                .await
            {
                host.id = self.discovery_type.host_id;

                container_interfaces_and_subnets.iter().for_each(|(i, _)| {
                    if !host.base.interfaces.contains(i) {
                        host.base.interfaces.push(i.clone())
                    }
                });

                services.iter_mut().for_each(|s| {
                    // Add all host port + IPs and any container ports which weren't matched
                    // We know they are open on this host even if no services matched them
                    container_ports_on_interface
                        .iter()
                        .for_each(|container_port| {
                            // Add bindings for container ports which weren't matched
                            match host.base.ports.iter().find(|p| p.base == *container_port) {
                                Some(unmatched_container_port)
                                    if !s
                                        .base
                                        .bindings
                                        .iter()
                                        .filter_map(|b| b.port_id())
                                        .any(|port_id| port_id == unmatched_container_port.id) =>
                                {
                                    s.base.bindings.push(Binding::new_l4(
                                        unmatched_container_port.id,
                                        Some(interface.id),
                                    ))
                                }
                                _ => (),
                            }
                        });

                    // Add bindings for all host ports, provided there's an interface
                    host_ip_to_host_ports.iter().for_each(|(ip, pbs)| {
                        pbs.iter().for_each(|pb| {
                            // If there's an existing port and existing bindings, we may need to replace those bindings if port is bound to All Interfaces on host
                            let (port, existing_bindings) =
                                match host.base.ports.iter().find(|p| p.base == *pb) {
                                    Some(existing_port) => (
                                        existing_port.clone(),
                                        s.base
                                            .bindings
                                            .iter()
                                            .filter_map(|b| match b.port_id() {
                                                Some(port_id) if port_id == existing_port.id => {
                                                    Some(b.id())
                                                }
                                                _ => None,
                                            })
                                            .collect(),
                                    ),
                                    None => (Port::new(pb.clone()), vec![]),
                                };

                            let interface = host
                                .base
                                .interfaces
                                .iter()
                                .find(|i| i.base.ip_address == *ip);

                            match interface {
                                Some(interface) => {
                                    s.base
                                        .bindings
                                        .push(Binding::new_l4(port.id, Some(interface.id)));
                                    host.base.ports.push(port);
                                }
                                None if *ip == ALL_INTERFACES_IP => {
                                    s.base.bindings = s
                                        .base
                                        .bindings
                                        .iter()
                                        .filter(|b| !existing_bindings.contains(&b.id()))
                                        .cloned()
                                        .collect();
                                    s.base.bindings.push(Binding::new_l4(port.id, None));
                                    host.base.ports.push(port);
                                }
                                _ => {}
                            }
                        });
                    });
                });

                discovered_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if let Ok(created_host) = self.create_host(host, services).await {
                    return Ok::<Option<Host>, Error>(Some(created_host));
                }
                return Ok(None);
            }
        }

        Ok(None)
    }

    pub async fn get_containers_to_scan(&self) -> Result<Vec<ContainerSummary>, Error> {
        let docker = self
            .discovery_type
            .docker_client
            .get()
            .ok_or_else(|| anyhow!("Docker client unavailable"))?;

        docker
            .list_containers(None::<ListContainersOptions>)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn get_subnets_from_docker_networks(&self, daemon_id: Uuid) -> Result<Vec<Subnet>> {
        let docker = self
            .discovery_type
            .docker_client
            .get()
            .ok_or_else(|| anyhow!("Docker client unavailable"))?;

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
                                cidr: IpCidr::from_str(cidr).ok()?,
                                description: None,
                                name: network_name.clone(),
                                subnet_type: SubnetType::DockerBridge,
                                source: EntitySource::Discovery(daemon_id),
                                hosts: Vec::new(),
                            }));
                        }
                        None
                    })
                    .collect::<Vec<Subnet>>()
            })
            .collect();

        Ok(subnets)
    }

    pub async fn get_containers_and_summaries(
        &self,
    ) -> Result<Vec<(ContainerInspectResponse, ContainerSummary)>, Error> {
        let docker = self
            .discovery_type
            .docker_client
            .get()
            .ok_or_else(|| anyhow!("Docker client unavailable"))?;

        let container_summaries = self.get_containers_to_scan().await?;

        let containers_to_inspect: Vec<_> = container_summaries
            .iter()
            .filter_map(|c| {
                if let Some(id) = &c.id {
                    return Some(docker.inspect_container(id, None::<InspectContainerOptions>));
                }
                None
            })
            .collect();

        let inspected_containers: Vec<ContainerInspectResponse> =
            try_join_all(containers_to_inspect).await?;

        Ok(inspected_containers
            .into_iter()
            .zip(container_summaries)
            .collect())
    }

    fn get_ports_from_container(
        &self,
        container_summary: ContainerSummary,
        container_interfaces_and_subnets: &[(Interface, Subnet)],
    ) -> (IpPortHashMap, IpPortHashMap) {
        let mut host_ip_to_host_ports: IpPortHashMap = HashMap::new();
        let mut container_ips_to_container_ports: IpPortHashMap = HashMap::new();

        let container_ips: Vec<IpAddr> = container_interfaces_and_subnets
            .iter()
            .map(|(i, _)| i.base.ip_address)
            .collect();

        if let Some(ports) = &container_summary.ports {
            ports.iter().for_each(|p| {
                let ip = p.ip.clone().unwrap_or_default().parse::<IpAddr>().ok();

                if let (Some(port_type @ (PortTypeEnum::TCP | PortTypeEnum::UDP)), Some(ip)) =
                    (p.typ, ip)
                {
                    let private_port = match port_type {
                        PortTypeEnum::TCP => PortBase::new_tcp(p.private_port),
                        PortTypeEnum::UDP => PortBase::new_udp(p.private_port),
                        _ => unreachable!("Already matched TCP/UDP in outer pattern"),
                    };

                    container_ips.iter().for_each(|ip| {
                        container_ips_to_container_ports
                            .entry(*ip)
                            .or_default()
                            .push(private_port.clone());
                    });

                    if let Some(public) = p.public_port {
                        let public_port = match port_type {
                            PortTypeEnum::TCP => PortBase::new_tcp(public),
                            PortTypeEnum::UDP => PortBase::new_udp(public),
                            _ => unreachable!("Already matched TCP/UDP in outer pattern"),
                        };

                        host_ip_to_host_ports
                            .entry(ip)
                            .or_default()
                            .push(public_port.clone());
                    }
                }
            });

            return (host_ip_to_host_ports, container_ips_to_container_ports);
        };

        (host_ip_to_host_ports, container_ips_to_container_ports)
    }

    fn get_container_interfaces(
        &self,
        containers: &[(ContainerInspectResponse, ContainerSummary)],
        subnets: &[Subnet],
        host_interfaces: &mut [Interface],
    ) -> HashMap<String, Vec<(Interface, Subnet)>> {
        // Created subnets may differ from discovered if there are existing subnets with the same CIDR, so we need to update interface subnet_id references
        let host_interfaces_and_subnets = host_interfaces
            .iter_mut()
            .filter_map(|i| {
                if let Some(subnet) = subnets
                    .iter()
                    .find(|s| s.base.cidr.contains(&i.base.ip_address))
                {
                    i.base.subnet_id = subnet.id;

                    return Some((i.clone(), subnet.clone()));
                }

                None
            })
            .collect::<Vec<(Interface, Subnet)>>();

        // Collect interfaces from container
        containers
            .iter()
            .filter_map(|(container, _)| {
                let mut interfaces_and_subnets: Vec<(Interface, Subnet)> =
                    if let Some(network_settings) = &container.network_settings {
                        if let Some(networks) = &network_settings.networks {
                            networks
                                .iter()
                                .filter_map(|(network_name, endpoint)| {
                                    // Parse interface if IP
                                    if let Some(ip_string) = &endpoint.ip_address {
                                        let ip_address = ip_string.parse::<IpAddr>().ok();

                                        if let Some(ip_address) = ip_address {
                                            if let Some(subnet) = subnets
                                                .iter()
                                                .find(|s| s.base.cidr.contains(&ip_address))
                                            {
                                                // Parse MAC address
                                                let mac_address = if let Some(mac_string) =
                                                    &endpoint.mac_address
                                                {
                                                    mac_string.parse::<MacAddress>().ok()
                                                } else {
                                                    None
                                                };

                                                return Some((
                                                    Interface::new(InterfaceBase {
                                                        subnet_id: subnet.id,
                                                        ip_address,
                                                        mac_address,
                                                        name: Some(network_name.to_owned()),
                                                    }),
                                                    subnet.clone(),
                                                ));
                                            }
                                        }
                                    }
                                    tracing::warn!(
                                "No matching subnet found for container {:?} on network '{}'",
                                container.name,
                                network_name
                            );

                                    None
                                })
                                .collect::<Vec<(Interface, Subnet)>>()
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    };

                // Merge in host interfaces
                interfaces_and_subnets.extend(host_interfaces_and_subnets.clone());

                container
                    .id
                    .as_ref()
                    .map(|id| (id.clone(), interfaces_and_subnets))
            })
            .collect()
    }
}
