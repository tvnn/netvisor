use std::{
    net::IpAddr,
    sync::{atomic::AtomicUsize, Arc, OnceLock},
};

use crate::{
    daemon::discovery::manager::DaemonDiscoverySessionManager,
    server::discovery::types::api::InitiateDiscoveryRequest,
};
use anyhow::{anyhow, Error};
use axum::async_trait;
use chrono::{DateTime, Utc};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    daemon::{
        discovery::types::base::{
            DiscoveryPhase, DiscoverySessionInfo, DiscoverySessionUpdate, ProcessHostParams,
        },
        shared::storage::ConfigStore,
        utils::base::{create_system_utils, PlatformDaemonUtils},
    },
    server::{
        daemons::types::api::{DaemonDiscoveryRequest, DiscoveryUpdatePayload},
        discovery::types::base::EntitySource,
        hosts::types::{
            api::HostWithServicesRequest,
            base::{Host, HostBase},
            interfaces::Interface,
            ports::{Port, PortBase},
            targets::HostTarget,
        },
        services::{
            definitions::{gateway::Gateway, ServiceDefinitionRegistry},
            types::{
                base::{Service, ServiceFromDiscoveryParams},
                bindings::{Binding, BindingDiscriminants, ServiceBinding},
                definitions::{ServiceDefinition, ServiceDefinitionExt},
                endpoints::EndpointResponse,
            },
        },
        shared::types::{api::ApiResponse, metadata::HasId},
        subnets::types::base::Subnet,
    },
};

pub struct DiscoveryHandler<T> {
    pub service: Arc<DaemonDiscoveryService>,
    pub manager: Arc<DaemonDiscoverySessionManager>,
    pub discovery_type: T,
}

impl<T> DiscoveryHandler<T> {
    pub fn new(
        service: Arc<DaemonDiscoveryService>,
        manager: Arc<DaemonDiscoverySessionManager>,
        discovery_type: T,
    ) -> Self {
        Self {
            service,
            discovery_type,
            manager,
        }
    }
}

impl<T> DiscoveryHandler<T>
where
    T: 'static,
    Self: DiscoversNetworkedEntities,
{
    pub async fn discover_on_network(
        self: Arc<Self>,
        request: DaemonDiscoveryRequest,
    ) -> Result<(), Error> {
        if self.manager.is_discovery_running().await {
            Err(anyhow!("Discovery session already running"))
        } else {
            let cancel_token = self.manager.start_new_session().await;

            let handler = self.clone();
            let request_clone = request.clone();

            let inner_manager = self.manager.clone();
            let handle = tokio::spawn(async move {
                match handler
                    .start_discovery_session(request_clone, cancel_token)
                    .await
                {
                    Ok(()) => {
                        tracing::info!("Discovery completed successfully");
                    }
                    Err(e) => {
                        tracing::error!("Discovery failed: {}", e);
                    }
                }
                inner_manager.clear_completed_task().await;
            });
            self.manager.set_current_task(handle).await;

            // Return immediate acknowledgment
            Ok(())
        }
    }
}

impl<T> AsRef<DaemonDiscoveryService> for DiscoveryHandler<T> {
    fn as_ref(&self) -> &DaemonDiscoveryService {
        &self.service
    }
}

pub struct DaemonDiscoveryService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
    pub utils: PlatformDaemonUtils,
    pub gateway_ips: OnceLock<Vec<IpAddr>>,
    pub scanned_count: Arc<AtomicUsize>,
    pub discovered_count: Arc<AtomicUsize>,
    pub session_info: OnceLock<DiscoverySessionInfo>,
}

impl DaemonDiscoveryService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        Self {
            config_store,
            client: reqwest::Client::new(),
            utils: create_system_utils(),
            scanned_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            discovered_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            gateway_ips: OnceLock::new(),
            session_info: OnceLock::new(),
        }
    }
}

impl AsRef<DaemonDiscoveryService> for DaemonDiscoveryService {
    fn as_ref(&self) -> &DaemonDiscoveryService {
        self
    }
}

#[async_trait]
pub trait DiscoversNetworkedEntities: AsRef<DaemonDiscoveryService> + Send + Sync {
    async fn start_discovery_session(
        &self,
        request: DaemonDiscoveryRequest,
        cancel: CancellationToken,
    ) -> Result<(), Error>;

    async fn set_gateway_ips(&self) -> Result<(), Error>;

    async fn discover_create_subnets(&self) -> Result<Vec<Subnet>, Error>;

    fn set_discovery_state(
        &self,
        total_to_scan: usize,
        request: DaemonDiscoveryRequest,
        daemon_id: Uuid,
        started_at: DateTime<Utc>,
    ) -> Result<(), Error> {
        tracing::debug!(
            "Setting session info for {} discovery session {}",
            request.discovery_type,
            request.session_id
        );
        self.as_ref()
            .session_info
            .set(DiscoverySessionInfo {
                total_to_scan,
                discovery_type: request.discovery_type,
                session_id: request.session_id,
                daemon_id,
                started_at: Some(started_at),
            })
            .map_err(|_| anyhow!("Failed to set session info"))?;

        Ok(())
    }

    async fn start_host_discovery(
        &self,
        total_to_scan: usize,
        request: DaemonDiscoveryRequest,
    ) -> Result<(), Error> {
        let daemon_id = self.as_ref().config_store.get_id().await?;

        tracing::info!(
            "Starting {} discovery session {}",
            request.discovery_type,
            request.session_id
        );

        self.set_gateway_ips().await?;
        self.set_discovery_state(total_to_scan, request, daemon_id, Utc::now())?;

        self.report_discovery_update(DiscoverySessionUpdate {
            phase: DiscoveryPhase::Started,
            completed: 0,
            error: None,
            discovered_count: 0,
            finished_at: None,
        })
        .await?;

        Ok(())
    }

    async fn finish_host_discovery(
        &self,
        discovery_result: Result<(), Error>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let session_info = self
            .as_ref()
            .session_info
            .get()
            .ok_or_else(|| anyhow!("Session info not set"))?;
        let session_id = session_info.session_id;

        let final_scanned_count = self
            .as_ref()
            .scanned_count
            .load(std::sync::atomic::Ordering::Relaxed);
        let final_discovered_count = self
            .as_ref()
            .discovered_count
            .load(std::sync::atomic::Ordering::Relaxed);

        match &discovery_result {
            Ok(_) => {
                tracing::info!("Discovery session {} completed successfully", session_id);
                self.report_discovery_update(DiscoverySessionUpdate {
                    phase: DiscoveryPhase::Complete,
                    completed: final_scanned_count,
                    error: None,
                    discovered_count: final_discovered_count,
                    finished_at: Some(Utc::now()),
                })
                .await?;
            }
            Err(_) if cancel.is_cancelled() => {
                tracing::warn!("Discovery session {} was cancelled", session_id);
                self.report_discovery_update(DiscoverySessionUpdate {
                    phase: DiscoveryPhase::Cancelled,
                    completed: final_scanned_count,
                    error: None,
                    discovered_count: final_discovered_count,
                    finished_at: Some(Utc::now()),
                })
                .await?;
            }
            Err(e) => {
                tracing::error!("Discovery session {} failed: {}", session_id, e);
                self.report_discovery_update(DiscoverySessionUpdate {
                    phase: DiscoveryPhase::Failed,
                    completed: final_scanned_count,
                    error: Some(e.to_string()),
                    discovered_count: final_discovered_count,
                    finished_at: Some(Utc::now()),
                })
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

    async fn process_host(
        &self,
        params: ProcessHostParams,
    ) -> Result<Option<(Host, Vec<Service>)>, Error> {
        let ProcessHostParams {
            host_ip,
            hostname,
            subnet,
            interface,
            open_ports,
            endpoint_responses,
            host_has_docker_client,
        } = params;

        if open_ports.is_empty() && endpoint_responses.is_empty() {
            return Ok(None); // Skip hosts with no interesting services
        }

        let daemon_id = self.as_ref().config_store.get_id().await?;

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
            interfaces: vec![interface.clone()],
            services: Vec::new(),
            ports: Vec::new(),
            source: EntitySource::Discovery(daemon_id),
        });

        let services = self.discover_host_services(
            &mut host,
            &interface,
            &open_ports,
            &endpoint_responses,
            &subnet,
            &host_has_docker_client,
        )?;

        tracing::info!("Processed host for host {}", host_ip);
        Ok(Some((host, services)))
    }

    fn discover_host_services(
        &self,
        host: &mut Host,
        interface: &Interface,
        open_ports: &[PortBase],
        endpoint_responses: &[EndpointResponse],
        subnet: &Subnet,
        host_has_docker_client: &bool,
    ) -> Result<Vec<Service>, Error> {
        let gateway_ips = self
            .as_ref()
            .gateway_ips
            .get()
            .ok_or_else(|| anyhow!("Gateway IPs not set, aborting"))?;

        let mut services = Vec::new();
        let mut matched_service_definitions = Vec::new();

        // Only one interface, so only one L3 binding possible
        let mut l3_interface_bound = false;

        // Need to track which ports are bound vs open for services to bind to
        let mut l4_unbound_ports = open_ports.to_vec();

        let mut sorted_service_definitions: Vec<Box<dyn ServiceDefinition>> =
            ServiceDefinitionRegistry::all_service_definitions()
                .into_iter()
                .collect();

        sorted_service_definitions.sort_unstable_by_key(|s| {
            if !s.is_generic() {
                0 // Highest priority - non-generic services
            } else if s.is_gateway() && s.id() != Gateway.id() {
                1 // Non-generic and subnet-typed gateways need to go before generic Gateway, otherwise will likely be classified as Gateway
            } else if s.is_infra_service() {
                2 // Infra services
            } else {
                3 // Lowest priority - non-infra generic services last
            }
        });

        // Add services from detected ports
        for service_definition in sorted_service_definitions {
            if let (Some(service), mut bound_ports) =
                Service::from_discovery(ServiceFromDiscoveryParams {
                    service_definition,
                    ip: &interface.base.ip_address,
                    open_ports,
                    endpoint_responses,
                    subnet,
                    mac_address: &interface.base.mac_address,
                    host_id: &host.id,
                    l3_interface_bound,
                    interface_id: &interface.id,
                    gateway_ips,
                    matched_service_definitions: &matched_service_definitions,
                    host_has_docker_client,
                })
            {
                if service.base.service_definition.layer() == BindingDiscriminants::Layer3 {
                    l3_interface_bound = true;
                }

                if !service.base.service_definition.is_generic() {
                    host.base.name = service.base.service_definition.name().to_string();
                }

                // If there's an http or https port binding + host target is hostname or none, use a binding as the host target
                if let (Some(binding), true) = (
                    service.base.bindings.iter().find(|b| {
                        match b {
                            Binding::Layer3 { .. } => false,
                            Binding::Layer4 { port_id, .. } => {
                                if let Some(port) = host.get_port(port_id) {
                                    return [
                                        PortBase::Http,
                                        PortBase::HttpAlt,
                                        PortBase::Https,
                                        PortBase::HttpsAlt,
                                    ]
                                    .contains(&port.base);
                                }
                                false
                            }
                        };
                        false
                    }),
                    matches!(host.base.target, HostTarget::Hostname | HostTarget::None),
                ) {
                    host.base.target = HostTarget::ServiceBinding(ServiceBinding {
                        binding_id: binding.id(),
                        service_id: service.id,
                    })
                }

                // Add any bound ports to host ports array, remove from open ports
                let bound_port_bases: Vec<PortBase> =
                    bound_ports.iter().map(|p| p.base.clone()).collect();

                host.base.ports.append(&mut bound_ports);

                // Add new service
                matched_service_definitions.push(service.base.service_definition.clone());
                host.add_service(service.id);
                l4_unbound_ports.retain(|p| !bound_port_bases.contains(p));
                services.push(service);
            }
        }

        host.base
            .ports
            .extend(l4_unbound_ports.into_iter().map(Port::new));

        Ok(services)
    }

    async fn periodic_scan_update(
        &self,
        frequency: usize,
        last_reported_scanned: usize,
        last_reported_discovered: usize,
    ) -> Result<(usize, usize), Error> {
        let scanned_count = self.as_ref().scanned_count.clone();
        let discovered_count = self.as_ref().discovered_count.clone();

        let current_scanned = scanned_count.load(std::sync::atomic::Ordering::Relaxed);
        let current_discovered = discovered_count.load(std::sync::atomic::Ordering::Relaxed);

        if current_scanned >= last_reported_scanned + frequency
            || last_reported_discovered > current_discovered
        {
            self.report_discovery_update(DiscoverySessionUpdate::scanning(
                current_scanned,
                current_discovered,
            ))
            .await?;

            return Ok((current_scanned, current_discovered));
        }

        Ok((last_reported_scanned, last_reported_discovered))
    }

    /// Report discovery progress to server
    async fn report_discovery_update(&self, update: DiscoverySessionUpdate) -> Result<(), Error> {
        let server_target = self.as_ref().config_store.get_server_endpoint().await?;
        let state = self
            .as_ref()
            .session_info
            .get()
            .ok_or_else(|| anyhow!("Session state unavailable"))?;
        let payload = DiscoveryUpdatePayload::from_state_and_update(state.clone(), update);

        let response = self
            .as_ref()
            .client
            .post(format!("{}/api/discovery/update", server_target))
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovery update: HTTP {}",
                response.status()
            );
        }

        tracing::debug!("Discovery update reported for session {}", state.session_id);
        Ok(())
    }
}

#[async_trait]
pub trait InitiatesOwnDiscovery: AsRef<DaemonDiscoveryService> + Send + Sync {
    async fn initiate_own_discovery(&self) -> Result<Uuid, Error> {
        let server_target = self.as_ref().config_store.get_server_endpoint().await?;
        let daemon_id = self.as_ref().config_store.get_id().await?;

        tracing::info!("Initiating discovery");

        let url = format!("{}/api/discovery/daemon-initiate", server_target);
        tracing::info!("Initiating discovery at URL: {}", url); // Add this line

        let response = self
            .as_ref()
            .client
            .post(format!("{}/api/discovery/daemon-initiate", server_target))
            .json(&InitiateDiscoveryRequest { daemon_id })
            .send()
            .await?;

        tracing::info!("Response status: {}", response.status());

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to initiate own discovery: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<Uuid> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to initiate discovery: {}", error_msg);
        }

        let session_id = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No session id in successful response"))?;

        Ok(session_id)
    }
}

#[async_trait]
pub trait CreatesDiscoveredEntities: AsRef<DaemonDiscoveryService> + Send + Sync {
    async fn create_host(&self, host: Host, services: Vec<Service>) -> Result<Host, Error> {
        let server_target = self.as_ref().config_store.get_server_endpoint().await?;

        tracing::info!("Creating host {}", host.base.name);

        let response = self
            .as_ref()
            .client
            .post(format!("{}/api/hosts", server_target))
            .json(&HostWithServicesRequest { host, services })
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovered host: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<Host> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create host: {}", error_msg);
        }

        let created_host = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No host data in successful response"))?;

        Ok(created_host)
    }

    async fn create_subnet(&self, subnet: &Subnet) -> Result<Subnet, Error> {
        let server_target = self.as_ref().config_store.get_server_endpoint().await?;

        let response = self
            .as_ref()
            .client
            .post(format!("{}/api/subnets", server_target))
            .json(&subnet)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovered subnet: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<Subnet> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create subnet: {}", error_msg);
        }

        let created_subnet = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No subnet data in successful response"))?;

        Ok(created_subnet)
    }

    async fn create_service(&self, service: &Service) -> Result<Service, Error> {
        let server_target = self.as_ref().config_store.get_server_endpoint().await?;

        let response = self
            .as_ref()
            .client
            .post(format!("{}/api/services", server_target))
            .json(&service)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to report discovered service: HTTP {}",
                response.status()
            );
        }

        let api_response: ApiResponse<Service> = response.json().await?;

        if !api_response.success {
            let error_msg = api_response
                .error
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Failed to create service: {}", error_msg);
        }

        let created_service = api_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No service data in successful response"))?;

        Ok(created_service)
    }
}
