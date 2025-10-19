use crate::daemon::discovery::service::base::{
    CreatesDiscoveredEntities, DiscoversNetworkedEntities, Discovery, HasDiscoveryType,
};
use crate::daemon::discovery::types::base::DiscoverySessionUpdate;
use crate::server::discovery::types::base::DiscoveryType;
use crate::server::hosts::types::{
    interfaces::{Interface, InterfaceBase},
    ports::PortBase,
};
use crate::server::services::types::base::ServiceMatchBaselineParams;
use crate::{
    daemon::utils::base::DaemonUtils,
    server::{
        daemons::types::api::DaemonDiscoveryRequest,
        hosts::types::base::Host,
        services::types::endpoints::EndpointResponse,
        subnets::types::base::{Subnet, SubnetType},
    },
};
use anyhow::{Error, Result};
use axum::async_trait;
use cidr::IpCidr;
use futures::{
    future::try_join_all,
    stream::{self, StreamExt},
};
use std::result::Result::Ok;
use std::{net::IpAddr, sync::Arc};
use tokio_util::sync::CancellationToken;

#[derive(Default)]
pub struct NetworkScanDiscovery {}

impl HasDiscoveryType for Discovery<NetworkScanDiscovery> {
    fn discovery_type(&self) -> DiscoveryType {
        DiscoveryType::Network
    }
}

impl CreatesDiscoveredEntities for Discovery<NetworkScanDiscovery> {}

#[async_trait]
impl DiscoversNetworkedEntities for Discovery<NetworkScanDiscovery> {
    async fn start_discovery_session(
        &self,
        request: DaemonDiscoveryRequest,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let subnets = self.discover_create_subnets().await?;

        let total_ips_across_subnets: usize = subnets
            .iter()
            .map(|subnet| subnet.base.cidr.iter().count())
            .sum();

        self.start_discovery(total_ips_across_subnets, request)
            .await?;

        let discovery_futures = subnets
            .iter()
            .map(|subnet| self.scan_and_process_hosts(subnet, cancel.clone()));

        let discovery_result = try_join_all(discovery_futures).await.map(|_| ());

        self.finish_discovery(discovery_result, cancel.clone())
            .await?;

        Ok(())
    }

    async fn get_gateway_ips(&self) -> Result<Vec<IpAddr>, Error> {
        self.as_ref().utils.get_routing_table_gateway_ips().await
    }

    async fn discover_create_subnets(&self) -> Result<Vec<Subnet>, Error> {
        let daemon_id = self.as_ref().config_store.get_id().await?;
        let (_, subnets) = self
            .as_ref()
            .utils
            .scan_interfaces(self.discovery_type(), daemon_id)
            .await?;
        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));
        let subnets = try_join_all(subnet_futures).await?;

        Ok(subnets)
    }
}

impl Discovery<NetworkScanDiscovery> {
    /// Scan subnet concurrently and process hosts immediately as they're discovered
    async fn scan_and_process_hosts(
        &self,
        subnet: &Subnet,
        cancel: CancellationToken,
    ) -> Result<Vec<Host>> {
        tracing::info!(
            "Scanning subnet {} concurrently for hosts with open ports",
            subnet.base.cidr
        );

        let concurrent_scans = self.as_ref().config_store.get_concurrent_scans().await?;

        let session = self.as_ref().get_session().await?;

        let scanned_count = session.scanned_count.clone();
        let discovered_count: Arc<std::sync::atomic::AtomicUsize> =
            session.discovered_count.clone();

        // Report initial progress
        self.report_discovery_update(DiscoverySessionUpdate::scanning(0, 0))
            .await?;

        // Process all IPs concurrently, combining discovery and processing
        let results = stream::iter(self.determine_scan_order(&subnet.base.cidr))
            .map(async |ip| {
                let cancel = cancel.clone();
                let subnet = subnet.clone();
                let scanned_count = scanned_count.clone();

                if let Ok(Some((all_ports, endpoint_responses, host_has_docker_client))) =
                    self.scan_host(ip, scanned_count, cancel).await
                {
                    let hostname = self.as_ref().utils.get_hostname_for_ip(ip).await?;

                    let mac = match subnet.base.subnet_type {
                        SubnetType::VpnTunnel => None, // ARP doesn't work through VPN tunnels
                        _ => self.as_ref().utils.get_mac_address_for_ip(ip).await?,
                    };

                    let interface = Interface::new(InterfaceBase {
                        name: None,
                        subnet_id: subnet.id,
                        ip_address: ip,
                        mac_address: mac,
                    });

                    if let Ok(Some((host, services))) = self
                        .process_host(
                            ServiceMatchBaselineParams {
                                subnet: &subnet,
                                interface: &interface,
                                all_ports: &all_ports,
                                endpoint_responses: &endpoint_responses,
                                host_has_docker_client: &host_has_docker_client,
                                virtualization: &None,
                            },
                            hostname,
                        )
                        .await
                    {
                        discovered_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if let Ok((created_host, _)) = self.create_host(host, services).await {
                            return Ok::<Option<Host>, Error>(Some(created_host));
                        }
                        return Ok(None);
                    }
                }
                Ok(None)
            })
            .buffer_unordered(concurrent_scans);

        // Consume the stream and report progress periodically
        tracing::info!(
            "Stream created for subnet {}, starting consumption",
            subnet.base.cidr
        );
        let mut stream_pin = Box::pin(results);
        let mut last_reported_scan_count: usize = 0;
        let mut last_reported_discovery_count: usize = 0;
        let mut successful_discoveries = Vec::new();

        while let Some(result) = stream_pin.next().await {
            if cancel.is_cancelled() {
                tracing::warn!("Discovery session was cancelled");
                return Err(Error::msg("Discovery session was cancelled"));
            }

            match result {
                Ok(Some(host)) => successful_discoveries.push(host),
                Ok(None) => {}
                Err(e) => tracing::warn!("Stream: error during scanning/processing: {}", e),
            }

            (last_reported_scan_count, last_reported_discovery_count) = self
                .periodic_scan_update(20, last_reported_scan_count, last_reported_discovery_count)
                .await?;
        }

        tracing::info!("Completed scanning subnet {}", subnet.base.cidr);
        Ok(successful_discoveries)
    }

    pub async fn scan_host(
        &self,
        ip: IpAddr,
        scanned_count: Arc<std::sync::atomic::AtomicUsize>,
        cancel: CancellationToken,
    ) -> Result<Option<(Vec<PortBase>, Vec<EndpointResponse>, bool)>> {
        // Check cancellation at the start
        if cancel.is_cancelled() {
            return Err(Error::msg("Discovery was cancelled"));
        }

        // Scan ports and endpoints
        let scan_result = self
            .as_ref()
            .utils
            .scan_ports_and_endpoints(ip, cancel.clone())
            .await;

        // Check cancellation after network operation
        if cancel.is_cancelled() {
            scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Err(Error::msg("Discovery was cancelled"));
        }

        match scan_result {
            Ok((open_ports, endpoint_responses, host_has_docker_client)) => {
                if !open_ports.is_empty()
                    || !endpoint_responses.is_empty()
                    || host_has_docker_client
                {
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

                    Ok(Some((
                        open_ports,
                        endpoint_responses,
                        host_has_docker_client,
                    )))
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
