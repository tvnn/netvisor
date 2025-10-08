use std::net::IpAddr;

use crate::server::hosts::types::ports::{Port, PortBase};
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::types::definitions::ServiceDefinitionExt;
use crate::server::services::types::definitions::{DefaultServiceDefinition, ServiceDefinition};
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::subnets::types::base::Subnet;
use crate::server::services::types::bindings::{Binding, BindingDiscriminants, ServiceBinding};
use chrono::{DateTime, Utc};
use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use std::hash::Hash;
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

pub struct ServiceFromDiscoveryParams<'a> {
    pub service_definition: Box<dyn ServiceDefinition>,
    pub ip: &'a IpAddr,
    pub open_ports: &'a [PortBase],
    pub endpoint_responses: &'a [EndpointResponse],
    pub subnet: &'a Subnet,
    pub mac_address: &'a Option<MacAddress>,
    pub host_id: &'a Uuid,
    pub interface_id: &'a Uuid,
    pub matched_service_definitions: &'a Vec<Box<dyn ServiceDefinition>>,
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

    pub fn get_l3_bindings(&self) -> Vec<&Binding> {
        self.base.bindings.iter().filter(|b| b.discriminant() == BindingDiscriminants::Layer3).collect()
    }

    pub fn get_l4_bindings(&self) -> Vec<&Binding> {
        self.base.bindings.iter().filter(|b| b.discriminant() == BindingDiscriminants::Layer4).collect()
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

    pub fn from_discovery(params: ServiceFromDiscoveryParams) -> (Option<Self>, Vec<Port>) {
        let ServiceFromDiscoveryParams {
            service_definition,
            ip,
            open_ports,
            endpoint_responses,
            subnet,
            mac_address,
            host_id,
            interface_id,
            matched_service_definitions,
        } = params;

        if let Ok(result) = service_definition.discovery_pattern().matches(
            open_ports,
            endpoint_responses,
            ip,
            subnet,
            mac_address,
            matched_service_definitions,
        ) {
            let matched_ports: Vec<Port> = result.into_iter().flatten().collect();

            tracing::info!(
                "✅ Service {:?} matched for {} with ports {:?}",
                service_definition,
                ip,
                matched_ports
            );

            let bindings: Vec<Binding> = matched_ports
                .iter()
                .map(|p| Binding::new_l4(p.id, *interface_id))
                .collect();

            let name = service_definition.name().to_string();

            let service = Service::new(ServiceBase {
                host_id: *host_id,
                service_definition,
                name,
                bindings,
            });

            (Some(service), matched_ports)
        } else {
            (None, Vec::new())
        }
    }
}
