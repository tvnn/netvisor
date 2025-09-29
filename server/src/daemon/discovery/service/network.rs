use crate::server::services::types::types::ServiceDefinitionExt;
use crate::{
    daemon::discovery::service::base::DaemonDiscoveryService,
    server::{
        hosts::types::{
            interfaces::{Interface, InterfaceBase},
            ports::{Port, PortBase, TransportProtocol},
            targets::ServiceBinding,
        },
        services::{
            definitions::{vpn_gateway::VpnGateway, ServiceDefinitionRegistry},
            types::types::ServiceDefinition,
        },
        shared::types::metadata::HasId,
    },
};
use crate::{
    daemon::{discovery::types::base::DiscoveryPhase, utils::base::DaemonUtils},
    server::{
        daemons::types::api::{DaemonDiscoveryRequest, DaemonDiscoveryUpdate},
        hosts::types::{
            base::{Host, HostBase},
            targets::HostTarget,
        },
        services::types::{base::Service, endpoints::EndpointResponse},
        subnets::types::base::{Subnet, SubnetType},
    },
};
use anyhow::{Error, Result};
use chrono::{DateTime, Utc};
use cidr::IpCidr;
use futures::{
    future::try_join_all,
    stream::{self, StreamExt},
};
use std::result::Result::Ok;
use std::{net::IpAddr, sync::Arc};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const CONCURRENT_SCANS: usize = 15;

impl DaemonDiscoveryService {
    pub async fn run_network_discovery(
        &self,
        request: DaemonDiscoveryRequest,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let daemon_id = self.config_store.get_id().await?;
        let session_id = request.session_id;
        let started_at = Utc::now();
        tracing::info!("Starting discovery session {}", session_id);

        let (_, subnets) = self.utils.scan_interfaces(daemon_id).await?;

        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));

        let subnets = try_join_all(subnet_futures).await?;

        let total_ips_across_subnets: usize = subnets
            .iter()
            .map(|subnet| subnet.base.cidr.iter().count())
            .sum();

        let scanned_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let discovered_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        self.report_discovery_update(
            session_id,
            &DaemonDiscoveryUpdate {
                session_id,
                phase: DiscoveryPhase::Started,
                completed: 0,
                total: 0,
                error: None,
                discovered_count: 0,
                daemon_id,
                started_at: Some(started_at),
                finished_at: None,
            },
        )
        .await?;

        let discovery_futures = subnets.iter().map(|subnet| {
            self.scan_and_process_hosts(
                subnet,
                session_id,
                daemon_id,
                started_at,
                cancel.clone(),
                discovered_count.clone(),
                scanned_count.clone(),
                total_ips_across_subnets,
            )
        });

        let discovery_result = try_join_all(discovery_futures).await;

        let final_scanned_count = scanned_count.load(std::sync::atomic::Ordering::Relaxed);
        let final_discovered_count = discovered_count.load(std::sync::atomic::Ordering::Relaxed);

        match &discovery_result {
            Ok(_) => {
                tracing::info!("Discovery session {} completed successfully", session_id);
                self.report_discovery_update(
                    session_id,
                    &DaemonDiscoveryUpdate {
                        session_id,
                        phase: DiscoveryPhase::Complete,
                        completed: final_scanned_count,
                        total: total_ips_across_subnets,
                        error: None,
                        discovered_count: final_discovered_count,
                        daemon_id,
                        started_at: Some(started_at),
                        finished_at: Some(Utc::now()),
                    },
                )
                .await?;
            }
            Err(_) if cancel.is_cancelled() => {
                tracing::warn!("Discovery session {} was cancelled", session_id);
                self.report_discovery_update(
                    session_id,
                    &DaemonDiscoveryUpdate {
                        session_id,
                        phase: DiscoveryPhase::Cancelled,
                        completed: final_scanned_count,
                        total: total_ips_across_subnets,
                        error: None,
                        discovered_count: final_discovered_count,
                        daemon_id,
                        started_at: Some(started_at),
                        finished_at: Some(Utc::now()),
                    },
                )
                .await?;
            }
            Err(e) => {
                tracing::error!("Discovery session {} failed: {}", session_id, e);
                self.report_discovery_update(
                    session_id,
                    &DaemonDiscoveryUpdate {
                        session_id,
                        phase: DiscoveryPhase::Failed,
                        completed: final_scanned_count,
                        total: total_ips_across_subnets,
                        error: Some(e.to_string()),
                        discovered_count: final_discovered_count,
                        daemon_id,
                        started_at: Some(started_at),
                        finished_at: Some(Utc::now()),
                    },
                )
                .await?;
            }
        }

        if cancel.is_cancelled() {
            tracing::info!("Discovery session {} was cancelled", session_id);
            return Ok(());
        }

        tracing::info!(
            "Discovery session {} finished with {} hosts discovered",
            session_id,
            final_discovered_count
        );
        Ok(())
    }

    /// Scan subnet concurrently and process hosts immediately as they're discovered
    async fn scan_and_process_hosts(
        &self,
        subnet: &Subnet,
        session_id: Uuid,
        daemon_id: Uuid,
        started_at: DateTime<Utc>,
        cancel: CancellationToken,
        discovered_count: Arc<std::sync::atomic::AtomicUsize>,
        scanned_count: Arc<std::sync::atomic::AtomicUsize>,
        total_ips_across_subnets: usize,
    ) -> Result<Vec<Host>> {
        tracing::info!(
            "Scanning subnet {} concurrently for hosts with open ports",
            subnet.base.cidr
        );

        // Report initial progress
        self.report_discovery_update(
            session_id,
            &DaemonDiscoveryUpdate {
                session_id,
                phase: DiscoveryPhase::Scanning,
                completed: 0,
                total: total_ips_across_subnets,
                error: None,
                discovered_count: 0,
                daemon_id,
                started_at: Some(started_at),
                finished_at: None,
            },
        )
        .await?;

        // Process all IPs concurrently, combining discovery and processing
        let results = stream::iter(self.determine_scan_order(&subnet.base.cidr))
            .map(async |ip| {
                let cancel = cancel.clone();
                let subnet = subnet.clone();
                let scanned_count = scanned_count.clone();

                if let Ok(Some((open_ports, endpoint_responses))) =
                    self.scan_host(ip, scanned_count, cancel).await
                {
                    if let Ok(Some((host, services))) = self
                        .process_host(ip, subnet, open_ports, endpoint_responses)
                        .await
                    {
                        discovered_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if let Ok(created_host) = self.create_host(host, services).await {
                            return Ok::<Option<Host>, Error>(Some(created_host));
                        }
                        return Ok(None);
                    }
                }
                Ok(None)
            })
            .buffer_unordered(CONCURRENT_SCANS);

        // Consume the stream and report progress periodically
        tracing::info!(
            "🌊 Stream created for subnet {}, starting consumption",
            subnet.base.cidr
        );
        let mut stream_pin = Box::pin(results);
        let mut last_reported_scan_count: usize = 0;
        let mut last_reported_discovery_count: usize = 0;
        let mut successful_discoveries = Vec::new();

        while let Some(result) = stream_pin.next().await {
            if cancel.is_cancelled() {
                tracing::warn!("Discovery session was {} cancelled", session_id);
                return Err(Error::msg("Discovery was cancelled"));
            }

            match result {
                Ok(Some(host)) => successful_discoveries.push(host),
                Ok(None) => {}
                Err(e) => tracing::warn!("Stream: error during scanning/processing: {}", e),
            }

            let current_scanned = scanned_count.load(std::sync::atomic::Ordering::Relaxed);
            let current_discovered = discovered_count.load(std::sync::atomic::Ordering::Relaxed);

            // Report progress every 20 scans or when done
            if current_scanned >= last_reported_scan_count + 20
                || last_reported_discovery_count > current_discovered
            {
                self.report_discovery_update(
                    session_id,
                    &DaemonDiscoveryUpdate {
                        session_id,
                        phase: DiscoveryPhase::Scanning,
                        completed: current_scanned,
                        total: total_ips_across_subnets,
                        error: None,
                        discovered_count: current_discovered,
                        daemon_id,
                        started_at: Some(started_at),
                        finished_at: None,
                    },
                )
                .await?;
                last_reported_scan_count = current_scanned;
                last_reported_discovery_count = current_discovered
            }
        }

        tracing::info!("Completed scanning subnet {}", subnet.base.cidr);
        Ok(successful_discoveries)
    }

    async fn scan_host(
        &self,
        ip: IpAddr,
        scanned_count: Arc<std::sync::atomic::AtomicUsize>,
        cancel: CancellationToken,
    ) -> Result<Option<(Vec<PortBase>, Vec<EndpointResponse>)>> {
        // Check cancellation at the start
        if cancel.is_cancelled() {
            return Err(Error::msg("Discovery was cancelled"));
        }

        // Scan ports and endpoints with cancellation support
        let scan_result = self
            .utils
            .scan_ports_and_endpoints(ip, cancel.clone())
            .await;

        // Check cancellation after network operation
        if cancel.is_cancelled() {
            scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Err(Error::msg("Discovery was cancelled"));
        }

        match scan_result {
            Ok((open_ports, endpoint_responses)) => {
                if !open_ports.is_empty() || !endpoint_responses.is_empty() {
                    tracing::info!(
                        "Processing host {} with {} open ports and {} endpoint responses",
                        ip,
                        open_ports.len(),
                        endpoint_responses.len()
                    );

                    // Check cancellation before processing
                    if cancel.is_cancelled() {
                        scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        return Err(Error::msg("Discovery was cancelled"));
                    }

                    Ok(Some((open_ports, endpoint_responses)))
                } else {
                    tracing::debug!("No open ports found on {}", ip);
                    scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    Ok(None)
                }
            }
            Err(e) => {
                tracing::debug!("Error scanning host {}: {}", ip, e);
                scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Ok(None)
            }
        }
    }

    /// Process a discovered host
    async fn process_host(
        &self,
        host_ip: IpAddr,
        subnet: Subnet,
        open_ports: Vec<PortBase>,
        endpoint_responses: Vec<EndpointResponse>,
    ) -> Result<Option<(Host, Vec<Service>)>, Error> {
        if open_ports.is_empty() && endpoint_responses.is_empty() {
            return Ok(None); // Skip hosts with no interesting services
        }

        let hostname = self.utils.get_hostname_for_ip(host_ip).await?;

        let mac = match subnet.base.subnet_type {
            SubnetType::VpnTunnel => None, // ARP doesn't work through VPN tunnels
            _ => self.utils.get_mac_address_for_ip(host_ip).await?,
        };

        let interface = Interface::new(InterfaceBase {
            name: None,
            subnet_id: subnet.id,
            ip_address: host_ip,
            mac_address: mac,
        });

        let interface_id = interface.id.clone();

        let interfaces = vec![interface];
        let interface_bindings = vec![interface_id];

        let (name, target) = match hostname.clone() {
            Some(hostname) => (hostname, HostTarget::Hostname),
            None => ("Unknown Device".to_owned(), HostTarget::None),
        };

        // Create host
        let mut host = Host::new(HostBase {
            name,
            hostname,
            target,
            description: Some("Discovered device".to_owned()),
            interfaces,
            services: Vec::new(),
            ports: Vec::new(),
        });

        let mut services = Vec::new();
        let mut matched_service_definitions = Vec::new();
        let mut unclaimed_ports = open_ports.clone();

        let mut sorted_service_definitions: Vec<Box<dyn ServiceDefinition>> =
            ServiceDefinitionRegistry::all_service_definitions()
                .into_iter()
                .collect();

        sorted_service_definitions.sort_unstable_by_key(|s| {
            if !s.is_generic() {
                0 // Highest priority - non-generic services
            } else if s.id() == VpnGateway.id() {
                1 // Needs to go before non-VPN gateways, otherwise will likely be classified as non-VPN gateway
            } else if s.is_infra_service() {
                2 // Infra services
            } else {
                3 // Lowest priority - non-infra generic services last
            }
        });

        // Add services from detected ports
        for service_definition in sorted_service_definitions {
            if let (Some(service), mut matched_ports) = Service::from_discovery(
                service_definition,
                host_ip,
                &unclaimed_ports,
                &endpoint_responses,
                &subnet,
                mac,
                &host.id,
                &interface_bindings,
                &matched_service_definitions,
            ) {
                if !service.base.service_definition.is_generic() {
                    host.base.name = service.base.service_definition.name().to_string();
                }

                if let (Some(port), Some(interface_id), true) = (
                    matched_ports
                        .iter()
                        .find(|p| p.base.protocol() == TransportProtocol::Tcp),
                    service.base.interface_bindings.first(),
                    matches!(host.base.target, HostTarget::Hostname | HostTarget::None),
                ) {
                    host.base.target = HostTarget::ServiceBinding(ServiceBinding {
                        port_id: port.id,
                        interface_id: *interface_id,
                        service_id: service.id,
                    })
                }

                // Add any matched ports to host ports array, remove from unclaimed ports
                let matched_port_bases: Vec<PortBase> =
                    matched_ports.iter().map(|p| p.base.clone()).collect();
                unclaimed_ports.retain(|p| !matched_port_bases.contains(p));
                host.base.ports.append(&mut matched_ports);

                // Add new service
                matched_service_definitions.push(service.base.service_definition.clone());
                host.add_service(service.id);
                services.push(service);
            }
        }

        host.base
            .ports
            .extend(unclaimed_ports.into_iter().map(|p| Port::new(p)));

        tracing::info!("Processed host for host {}", host_ip);
        Ok(Some((host, services)))
    }

    /// Figure out what order to scan IPs in given allocation patterns
    fn determine_scan_order(&self, subnet: &IpCidr) -> impl Iterator<Item = IpAddr> {
        let mut ips: Vec<IpAddr> = subnet.iter().map(|ip| ip.address()).collect();

        // Sort by likelihood of being active hosts - highest probability first
        ips.sort_by_key(|ip| {
            let last_octet = match ip {
                IpAddr::V4(ipv4) => ipv4.octets()[3],
                IpAddr::V6(_) => return 9999, // IPv6 gets lowest priority for now
            };

            match last_octet {
                // Tier 1: Almost guaranteed to be active infrastructure
                1 => 1,   // Default gateway (.1)
                254 => 2, // Alternative gateway (.254)

                // Tier 2: Very common infrastructure and static assignments
                2 => 10,   // Secondary router/switch
                3 => 11,   // Tertiary infrastructure
                10 => 12,  // Common DHCP start
                100 => 13, // Common DHCP end
                253 => 14, // Alt gateway range
                252 => 15, // Alt gateway range

                // Tier 3: Common static device ranges
                4..=9 => 20 + last_octet as u16, // Infrastructure devices
                11..=20 => 30 + last_octet as u16, // Servers, printers
                21..=30 => 50 + last_octet as u16, // Network devices

                // Tier 4: Active DHCP ranges (most devices live here)
                31..=50 => 100 + last_octet as u16, // Early DHCP range
                51..=100 => 200 + last_octet as u16, // Mid DHCP range
                101..=150 => 400 + last_octet as u16, // Late DHCP range

                // Tier 5: Less common but still viable
                151..=200 => 600 + last_octet as u16, // Extended DHCP
                201..=251 => 800 + last_octet as u16, // High static range

                // Skip entirely - reserved addresses
                0 | 255 => 9998, // Network/broadcast addresses
            }
        });

        ips.into_iter()
    }
}
