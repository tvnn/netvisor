use std::net::IpAddr;

use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::hosts::types::ports::{Port, PortBase};
use crate::server::services::types::types::ServiceDefinitionExt;
use crate::server::services::types::types::{DefaultServiceDefinition, ServiceDefinition};
use crate::server::subnets::types::base::{Subnet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ServiceBase {
    pub host_id: Uuid,
    pub service_definition: Box<dyn ServiceDefinition>,
    pub name: String,
    pub port_bindings: Vec<Uuid>,
    pub interface_bindings: Vec<Uuid>,
    pub groups: Vec<Uuid>
}

impl Default for ServiceBase {
    fn default() -> Self {
        Self {
            host_id: Uuid::nil(),
            service_definition: Box::new(DefaultServiceDefinition),
            name: String::new(),
            port_bindings: Vec::new(),
            interface_bindings: Vec::new(),
            groups: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Service {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub base: ServiceBase,
}

impl Service {

    pub fn new(base: ServiceBase) -> Self{
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            base
        }
    }

    pub fn all_discovery_ports() -> Vec<PortBase> {
        let mut ports: Vec<PortBase> = ServiceDefinitionRegistry::all_service_definitions().iter()
            .flat_map(|s| s.discovery_ports())
            .collect();
            
        ports.sort_by_key(|p| (p.number(), p.protocol()));
        ports.dedup();
        ports
    }

    pub fn all_discovery_endpoints() -> Vec<Endpoint> {
        let mut endpoints: Vec<Endpoint> = ServiceDefinitionRegistry::all_service_definitions().iter()
            .flat_map(|s| s.discovery_endpoints())
            .collect();

        endpoints.sort_by_key(|e| (e.protocol.to_string(), e.port_base.number(), e.path.clone().unwrap_or("".to_string())) );
        endpoints.dedup();
        endpoints
    }

    pub fn from_discovery(
        service_definition: Box<dyn ServiceDefinition>, 
        ip: IpAddr, 
        open_ports: &Vec<PortBase>, 
        endpoint_responses: &Vec<EndpointResponse>, 
        subnet: &Subnet, 
        mac_address: Option<MacAddress>,
        host_id: &Uuid,
        interface_bindings: &Vec<Uuid>,
        matched_service_definitions: &Vec<Box<dyn ServiceDefinition>>) -> (Option<Self>, Vec<Port>) {

        if let Ok(result) = service_definition.discovery_pattern().matches(open_ports.clone(), endpoint_responses.clone(), ip, subnet, mac_address, matched_service_definitions) {

            let matched_ports: Vec<Port> = result.into_iter().filter_map(|p| p).collect();

            tracing::info!("✅ Service {:?} matched for {} with ports {:?}", service_definition, ip, matched_ports);

            let port_bindings: Vec<Uuid> = matched_ports.iter().map(|p| p.id).collect();
            let name = service_definition.name().to_string();

            let service = Service::new(ServiceBase { host_id: *host_id, service_definition, name, port_bindings, interface_bindings: interface_bindings.clone(), groups: Vec::new() });
            
            return (Some(service), matched_ports)
        } else {
            return (None, Vec::new())
        }
    }
}