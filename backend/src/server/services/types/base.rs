use crate::server::discovery::types::base::{DiscoveryType, EntitySource, MatchMetadata};
use crate::server::hosts::types::interfaces::Interface;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::types::bindings::{Binding, BindingDiscriminants};
use crate::server::services::types::definitions::ServiceDefinitionExt;
use crate::server::services::types::definitions::{DefaultServiceDefinition, ServiceDefinition};
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::patterns::MatchResult;
use crate::server::services::types::virtualization::{DockerVirtualization, ServiceVirtualization};
use crate::server::subnets::types::base::Subnet;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;
use std::net::IpAddr;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Validate, Deserialize, PartialEq, Eq, Hash)]
pub struct ServiceBase {
    pub host_id: Uuid,
    pub service_definition: Box<dyn ServiceDefinition>,
    #[validate(length(min = 0, max = 100))]
    pub name: String,
    pub bindings: Vec<Binding>,
    pub virtualization: Option<ServiceVirtualization>,
    /// Host IDs that are VMs managed by service
    pub vms: Vec<Uuid>,
    /// Service IDs that are VMs managed by service
    pub containers: Vec<Uuid>,
    pub source: EntitySource,
}

impl Default for ServiceBase {
    fn default() -> Self {
        Self {
            host_id: Uuid::nil(),
            service_definition: Box::new(DefaultServiceDefinition),
            name: String::new(),
            bindings: Vec::new(),
            virtualization: None,
            vms: vec![],
            containers: vec![],
            source: EntitySource::Unknown,
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct Service {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    #[validate(nested)]
    pub base: ServiceBase,
}

#[derive(Debug, Clone)]
pub struct DiscoverySessionServiceMatchParams<'a> {
    pub host_id: &'a Uuid,
    pub gateway_ips: &'a [IpAddr],
    pub daemon_id: &'a Uuid,
    pub discovery_type: &'a DiscoveryType,
    pub baseline_params: &'a ServiceMatchBaselineParams<'a>,
    pub service_params: ServiceMatchServiceParams<'a>,
}

#[derive(Debug, Clone)]
pub struct ServiceMatchBaselineParams<'a> {
    pub subnet: &'a Subnet,
    pub interface: &'a Interface,
    pub all_ports: &'a Vec<PortBase>,
    pub endpoint_responses: &'a Vec<EndpointResponse>,
    pub host_has_docker_client: &'a bool,
    pub virtualization: &'a Option<ServiceVirtualization>,
}

#[derive(Debug, Clone)]
pub struct ServiceMatchServiceParams<'a> {
    pub service_definition: Box<dyn ServiceDefinition>,
    pub l3_interface_bound: &'a bool,
    pub matched_services: &'a Vec<Service>,
    pub unbound_ports: &'a Vec<PortBase>,
}

impl PartialEq for Service {
    fn eq(&self, other: &Self) -> bool {
        let host_match = self.base.host_id == other.base.host_id;
        let definition_match =
            self.base.service_definition.id() == other.base.service_definition.id();
        let name_match = self.base.name == other.base.name;
        let id_match = self.id == other.id;

        (host_match && definition_match && name_match) || id_match
    }
}

impl Default for Service {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            base: ServiceBase::default(),
        }
    }
}

impl Hash for Service {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.base.service_definition.hash(state);
        self.base.name.hash(state);
        self.base.host_id.hash(state);
    }
}

impl Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.base.name, self.id)
    }
}

impl Service {
    pub fn new(base: ServiceBase) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            base,
        }
    }

    pub fn get_binding(&self, id: Uuid) -> Option<&Binding> {
        self.base.bindings.iter().find(|b| b.id() == id)
    }

    pub fn to_bound_interface_ids(&self) -> Vec<Uuid> {
        self.base
            .bindings
            .iter()
            .filter_map(|i| i.interface_id())
            .collect()
    }

    pub fn to_bound_port_ids(&self) -> Vec<Uuid> {
        self.base
            .bindings
            .iter()
            .filter_map(|i| i.port_id())
            .collect()
    }

    pub fn all_discovery_ports() -> Vec<PortBase> {
        let mut ports: Vec<PortBase> = ServiceDefinitionRegistry::all_service_definitions()
            .iter()
            .flat_map(|s| s.discovery_pattern().ports())
            .collect();

        ports.sort_by_key(|p| (p.number(), p.protocol()));
        ports.dedup();
        ports
    }

    pub fn all_discovery_endpoints() -> Vec<Endpoint> {
        let mut endpoints: Vec<Endpoint> = ServiceDefinitionRegistry::all_service_definitions()
            .iter()
            .flat_map(|s| s.discovery_pattern().endpoints())
            .collect();

        endpoints.sort_by_key(|e| (e.protocol.to_string(), e.port_base.number(), e.path.clone()));
        endpoints.dedup();
        endpoints
    }

    /// Matches scanned data and returns service, vec of matched ports
    pub fn from_discovery(
        params: DiscoverySessionServiceMatchParams,
    ) -> Option<(Self, MatchResult)> {
        let DiscoverySessionServiceMatchParams {
            host_id,
            baseline_params,
            service_params,
            daemon_id,
            discovery_type,
            ..
        } = params.clone();

        let ServiceMatchBaselineParams {
            interface,
            all_ports,
            virtualization,
            ..
        } = baseline_params;

        let virtualization = *virtualization;

        let ServiceMatchServiceParams {
            service_definition,
            l3_interface_bound,
            ..
        } = service_params;

        if let Ok(result) = service_definition.discovery_pattern().matches(&params) {
            let mut name = service_definition.name().to_string();

            let details = if service_definition.is_generic() {
                if let Some(ServiceVirtualization::Docker(DockerVirtualization {
                    container_name: Some(c_name),
                    ..
                })) = virtualization
                {
                    name = c_name.clone()
                }

                // Don't show match details for generic services, confidence doesn't matter because they are fallbacks anyway
                None
            } else {
                Some(result.details.clone())
            };

            let discovery_metadata = MatchMetadata {
                discovery_type: *discovery_type,
                daemon_id: *daemon_id,
                details,
            };

            if service_definition.layer() == BindingDiscriminants::Layer3 && !l3_interface_bound {
                tracing::debug!("Matched service with params {:?}", params);
                tracing::info!(
                    "{}: L3 service {:?} matched",
                    interface.base.ip_address,
                    service_definition.name(),
                );

                Some((
                    Service::new(ServiceBase {
                        host_id: *host_id,
                        service_definition,
                        name,
                        bindings: vec![Binding::new_l3(interface.id)],
                        virtualization: virtualization.clone(),
                        vms: vec![],
                        containers: vec![],
                        source: EntitySource::Discovery(discovery_metadata),
                    }),
                    result,
                ))
            } else if service_definition.layer() == BindingDiscriminants::Layer4 {
                tracing::debug!("Matched service with params {:?}", params);
                tracing::info!(
                    "{}: L4 service {:?} matched with ports {:?}",
                    interface.base.ip_address,
                    service_definition,
                    result.ports
                );

                Some((
                    Service::new(ServiceBase {
                        host_id: *host_id,
                        service_definition,
                        name,
                        virtualization: virtualization.clone(),
                        bindings: result
                            .ports
                            .iter()
                            .map(|p| Binding::new_l4(p.id, Some(interface.id)))
                            .collect(),
                        vms: vec![],
                        containers: vec![],
                        source: EntitySource::Discovery(discovery_metadata),
                    }),
                    result,
                ))
            } else {
                tracing::warn!(
                    "{}: No services matched. L3 interface already bound: {}, open ports on host: {:?}",
                    interface.base.ip_address,
                    l3_interface_bound,
                    all_ports
                );

                None
            }
        } else {
            None
        }
    }
}
