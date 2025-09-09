use std::{net::IpAddr, sync::Arc};
use anyhow::{Error, Result};
use cidr::{IpCidr};
use chrono::{DateTime, Utc};
use crate::daemon::discovery::service::base::{DaemonDiscoveryService};
use tokio_util::sync::CancellationToken;
use strum::{IntoDiscriminant, IntoEnumIterator};
use uuid::Uuid;
use futures::{future::{try_join_all}, stream::{self, StreamExt}};
use std::result::Result::Ok;
use crate::{
    daemon::{discovery::{types::base::DiscoveryPhase}, utils::base::{DaemonUtils}},
    server::{
        daemons::types::api::{DaemonDiscoveryRequest, DaemonDiscoveryUpdate}, nodes::types::{
            base::{Node, NodeBase}, targets::{IpAddressTargetConfig, NodeTarget}
        }, services::types::{base::{Service, ServiceDiscriminants}, endpoints::EndpointResponse, ports::Port}, shared::types::{metadata::TypeMetadataProvider}, subnets::types::base::{NodeSubnetMembership, Subnet, SubnetType}
    },
};

const CONCURRENT_SCANS: usize = 15;

impl DaemonDiscoveryService {

    pub async fn run_network_discovery(&self, request: DaemonDiscoveryRequest, cancel: CancellationToken) -> Result<(), Error> {
        
        let daemon_id = self.config_store.get_id().await?;
        let session_id = request.session_id;
        let started_at = Utc::now();
        tracing::info!("Starting discovery session {}", session_id);
        
        let (_, subnets) = self.utils.scan_subnets(daemon_id).await?;

        let subnet_futures = subnets.iter().map(|subnet| self.create_subnet(subnet));

        let subnets = try_join_all(subnet_futures).await?;

        let total_ips_across_subnets: usize = subnets.iter()
            .map(|subnet| subnet.base.cidr.iter().count())
            .sum();

        let scanned_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let discovered_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
            session_id,
            phase: DiscoveryPhase::Started,
            completed: 0,
            total: 0,
            error: None,
            discovered_count: 0,
            daemon_id,
            started_at: Some(started_at),
            finished_at: None
        }).await?;

        let discovery_futures = subnets.iter()
            .map(|subnet | {
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
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Complete,
                    completed: final_scanned_count,
                    total: total_ips_across_subnets,
                    error: None,
                    discovered_count: final_discovered_count,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: Some(Utc::now())
                }).await?;
            },
            Err(_) if cancel.is_cancelled() => {
                tracing::warn!("Discovery session {} was cancelled", session_id);
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Cancelled,
                    completed: final_scanned_count,
                    total: total_ips_across_subnets,
                    error: None,
                    discovered_count: final_discovered_count,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: Some(Utc::now())
                }).await?;
            },
            Err(e) => {
                tracing::error!("Discovery session {} failed: {}", session_id, e);
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Failed,
                    completed: final_scanned_count,
                    total: total_ips_across_subnets,
                    error: Some(e.to_string()),
                    discovered_count: final_discovered_count,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: Some(Utc::now())
                }).await?;
            }
        }

        if cancel.is_cancelled() {
            tracing::info!("Discovery session {} was cancelled", session_id);
            return Ok(());
        }
        
        tracing::info!("Discovery session {} finished with {} nodes discovered", session_id, final_discovered_count);
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
    ) -> Result<Vec<Node>> {
        tracing::info!("Scanning subnet {} concurrently for hosts with open ports", subnet.base.cidr);

        // Report initial progress
        self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
            session_id,
            phase: DiscoveryPhase::Scanning,
            completed: 0,
            total: total_ips_across_subnets,
            error: None,
            discovered_count: 0,
            daemon_id,
            started_at: Some(started_at),
            finished_at: None
        }).await?;

        // Process all IPs concurrently, combining discovery and processing
       let results = stream::iter(self.determine_scan_order(&subnet.base.cidr))
            .map( async |ip | {
                let cancel = cancel.clone();
                let subnet = subnet.clone();
                let scanned_count = scanned_count.clone();
                
                if let Ok(Some((open_ports, endpoint_responses))) = self.scan_host(ip, scanned_count, cancel).await {
                    
                    if let Ok(Some(node)) = self.process_host(
                        ip,
                        subnet,
                        open_ports,
                        endpoint_responses,
                    ).await {
                        discovered_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        return Ok::<std::option::Option<Node>, Error>(self.create_node(&node).await.ok())
                    }
                }
                Ok(None)
            })
            .buffer_unordered(CONCURRENT_SCANS);

        // Consume the stream and report progress periodically
        tracing::info!("🌊 Stream created for subnet {}, starting consumption", subnet.base.cidr);
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
                Ok(Some(node)) => successful_discoveries.push(node),
                Ok(None) => {},
                Err(e) => tracing::warn!("Stream: error during scanning/processing: {}", e)
            }

            let current_scanned = scanned_count.load(std::sync::atomic::Ordering::Relaxed);
            let current_discovered = discovered_count.load(std::sync::atomic::Ordering::Relaxed);

            // Report progress every 20 scans or when done
            if current_scanned >= last_reported_scan_count + 20 || last_reported_discovery_count > current_discovered {
                self.report_discovery_update(session_id, &DaemonDiscoveryUpdate {
                    session_id,
                    phase: DiscoveryPhase::Scanning,
                    completed: current_scanned,
                    total: total_ips_across_subnets,
                    error: None,
                    discovered_count: current_discovered,
                    daemon_id,
                    started_at: Some(started_at),
                    finished_at: None
                }).await?;
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
        cancel: CancellationToken
    ) -> Result<Option<(Vec<Port>, Vec<EndpointResponse>)>> {

        // Check cancellation at the start
        if cancel.is_cancelled() {
            return Err(Error::msg("Discovery was cancelled"));
        }

        // Scan ports and endpoints with cancellation support
        let scan_result = self.utils.scan_ports_and_endpoints(ip, cancel.clone()).await;

        // Check cancellation after network operation
        if cancel.is_cancelled() {
            scanned_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Err(Error::msg("Discovery was cancelled"));
        }

        match scan_result {
            Ok((open_ports, endpoint_responses)) => {
                if !open_ports.is_empty() || !endpoint_responses.is_empty() {
                    tracing::info!("Processing host {} with {} open ports and {} endpoint responses", ip, open_ports.len(), endpoint_responses.len());
                    
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
            },
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
        open_ports: Vec<Port>,
        endpoint_responses: Vec<EndpointResponse>,
    ) -> Result<Option<Node>, Error> {
        
        if open_ports.is_empty() && endpoint_responses.is_empty() {
            return Ok(None); // Skip hosts with no interesting services
        }

        // Gather host information
        let hostname = self.utils.get_hostname_for_ip(host_ip).await?;
        let mac_address = match subnet.base.subnet_type {
            SubnetType::VpnTunnel => None, // ARP doesn't work through VPN tunnels
            _ => self.utils.get_mac_address_for_ip(host_ip).await?
        };

        // Create node
        let mut node = Node::new(NodeBase {
            name: hostname.clone().unwrap_or_else(|| host_ip.to_string()),
            hostname,
            target: NodeTarget::IpAddress(IpAddressTargetConfig {
                ip: host_ip,
            }),
            description: Some("Discovered device".to_string()),
            subnets: vec!(NodeSubnetMembership {
                subnet_id: subnet.id,
                ip_address: host_ip,
                mac_address,
                default: false
            }),
            services: Vec::new(),
            open_ports: Vec::new(),
            node_groups: Vec::new(),
        });

        let mut unclaimed_ports = open_ports.clone();
        
        let mut sorted_discriminants: Vec<ServiceDiscriminants> = ServiceDiscriminants::iter()
            .collect::<Vec<ServiceDiscriminants>>();
        
        sorted_discriminants.sort_unstable_by_key(|discriminant| {
            discriminant.to_string().contains("Generic")
        });

        // Add services from detected ports
        for discriminant in sorted_discriminants {
            let non_generic_service_count = node.base.services.iter().filter(|s| !s.discriminant().is_generic_service()).count();
            // Once a distinct vendor service has been identified, skip other services
            if discriminant.is_generic_service() && non_generic_service_count > 0 {
                continue;
            }
            if let (Some(service), Some(service_ports)) = Service::from_discovery(discriminant, host_ip, &open_ports, &endpoint_responses, &subnet, mac_address) {
                if !discriminant.is_generic_service() && non_generic_service_count == 0 {
                    node.base.name = service.discriminant().display_name().to_string();
                }

                unclaimed_ports.retain(|p| !service_ports.contains(p));
                node.add_service(service);
            }
        };

        node.base.open_ports = unclaimed_ports;
        
        tracing::info!("Processed node for host {} with {} open ports", host_ip, open_ports.len());
        Ok(Some(node))
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
                1 => 1,      // Default gateway (.1)
                254 => 2,    // Alternative gateway (.254)
                
                // Tier 2: Very common infrastructure and static assignments
                2 => 10,     // Secondary router/switch
                3 => 11,     // Tertiary infrastructure  
                10 => 12,    // Common DHCP start
                100 => 13,   // Common DHCP end
                253 => 14,   // Alt gateway range
                252 => 15,   // Alt gateway range
                
                // Tier 3: Common static device ranges
                4..=9 => 20 + last_octet as u16,      // Infrastructure devices
                11..=20 => 30 + last_octet as u16,    // Servers, printers
                21..=30 => 50 + last_octet as u16,    // Network devices
                
                // Tier 4: Active DHCP ranges (most devices live here)
                31..=50 => 100 + last_octet as u16,   // Early DHCP range
                51..=100 => 200 + last_octet as u16,  // Mid DHCP range
                101..=150 => 400 + last_octet as u16, // Late DHCP range
                
                // Tier 5: Less common but still viable
                151..=200 => 600 + last_octet as u16, // Extended DHCP
                201..=251 => 800 + last_octet as u16, // High static range
                
                // Skip entirely - reserved addresses
                0 | 255 => 9998,  // Network/broadcast addresses
            }
        });
        
        ips.into_iter()
    }
}
