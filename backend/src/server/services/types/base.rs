use crate::server::hosts::types::interfaces::Interface;
use crate::server::hosts::types::ports::{Port, PortBase};
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::types::bindings::{Binding, BindingDiscriminants, ServiceBinding};
use crate::server::services::types::definitions::ServiceDefinitionExt;
use crate::server::services::types::definitions::{DefaultServiceDefinition, ServiceDefinition};
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
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
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub bindings: Vec<Binding>,
}

impl Default for ServiceBase {
    fn default() -> Self {
        Self {
            host_id: Uuid::nil(),
            service_definition: Box::new(DefaultServiceDefinition),
            name: String::new(),
            bindings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Hash)]
pub struct Service {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    #[validate(nested)]
    pub base: ServiceBase,
}

#[derive(Debug, Clone)]
pub struct ServiceDiscoveryParams<'a> {
    pub host_id: &'a Uuid,
    pub gateway_ips: &'a [IpAddr],
    pub baseline_params: &'a ServiceDiscoveryBaselineParams<'a>,
    pub discovery_state_params: ServiceDiscoveryStateParams<'a>,
}

#[derive(Debug, Clone)]
pub struct ServiceDiscoveryBaselineParams<'a> {
    pub subnet: &'a Subnet,
    pub interface: &'a Interface,
    pub open_ports: &'a Vec<PortBase>,
    pub endpoint_responses: &'a Vec<EndpointResponse>,
    pub host_has_docker_client: &'a bool,
    pub docker_container_name: &'a Option<String>,
}

#[derive(Debug, Clone)]
pub struct ServiceDiscoveryStateParams<'a> {
    pub service_definition: Box<dyn ServiceDefinition>,
    pub l3_interface_bound: &'a bool,
    pub matched_services: &'a Vec<Service>,
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

    pub fn to_bindings(&self) -> Vec<ServiceBinding> {
        self.base
            .bindings
            .iter()
            .map(|b| ServiceBinding {
                service_id: self.id,
                binding_id: b.id(),
            })
            .collect()
    }

    pub fn all_discovery_ports() -> Vec<PortBase> {
        let mut ports: Vec<PortBase> = ServiceDefinitionRegistry::all_service_definitions()
            .iter()
            .flat_map(|s| s.discovery_ports())
            .collect();

        ports.sort_by_key(|p| (p.number(), p.protocol()));
        ports.dedup();
        ports
    }

    pub fn all_discovery_endpoints() -> Vec<Endpoint> {
        let mut endpoints: Vec<Endpoint> = ServiceDefinitionRegistry::all_service_definitions()
            .iter()
            .flat_map(|s| s.discovery_endpoints())
            .collect();

        endpoints.sort_by_key(|e| {
            (
                e.protocol.to_string(),
                e.port_base.number(),
                e.path.clone().unwrap_or("".to_string()),
            )
        });
        endpoints.dedup();
        endpoints
    }

    /// Matches scanned data and returns service, vec of matched ports
    pub fn from_discovery(params: ServiceDiscoveryParams) -> (Option<Self>, Vec<Port>) {
        let ServiceDiscoveryParams {
            host_id,
            baseline_params,
            discovery_state_params,
            ..
        } = params.clone();

        let ServiceDiscoveryBaselineParams {
            interface,
            open_ports,
            docker_container_name,
            ..
        } = baseline_params;

        let ServiceDiscoveryStateParams {
            service_definition,
            l3_interface_bound,
            ..
        } = discovery_state_params;

        if let Ok(result) = service_definition.discovery_pattern().matches(&params) {
            let name = if let Some(container_name) = docker_container_name {
                container_name.clone()
            } else {
                service_definition.name().to_string()
            };
            let matched_ports: Vec<Port> = result.into_iter().flatten().collect();

            if service_definition.layer() == BindingDiscriminants::Layer3 && !l3_interface_bound {
                tracing::debug!("Matched service with params {:?}", params);
                tracing::info!(
                    "{}: L3 service {:?} matched",
                    interface.base.ip_address,
                    service_definition.name(),
                );

                (
                    Some(Service::new(ServiceBase {
                        host_id: *host_id,
                        service_definition,
                        name,
                        bindings: vec![Binding::new_l3(interface.id)],
                    })),
                    Vec::new(),
                )
            } else if service_definition.layer() == BindingDiscriminants::Layer4 {
                tracing::debug!("Matched service with params {:?}", params);
                tracing::info!(
                    "{}: L4 service {:?} matched with ports {:?}",
                    interface.base.ip_address,
                    service_definition,
                    matched_ports
                );

                (
                    Some(Service::new(ServiceBase {
                        host_id: *host_id,
                        service_definition,
                        name,
                        bindings: matched_ports
                            .iter()
                            .map(|p| Binding::new_l4(p.id, Some(interface.id)))
                            .collect(),
                    })),
                    matched_ports,
                )
            } else {
                tracing::warn!(
                    "{}: No services matched. L3 interface already bound: {}, open ports on host: {:?}",
                    interface.base.ip_address,
                    l3_interface_bound,
                    open_ports
                );

                (None, Vec::new())
            }
        } else {
            (None, Vec::new())
        }
    }
}
