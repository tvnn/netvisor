use std::net::IpAddr;

use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use strum::{IntoEnumIterator};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::ports::{Port};
use crate::server::services::types::types::ServiceType;
use crate::server::subnets::types::base::{Subnet};
use crate::server::{shared::{types::metadata::TypeMetadataProvider}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ServiceBase {
    pub host_id: Uuid,
    pub service_type: ServiceType,
    pub name: String,
    pub ports: Vec<Port>,
    pub interface_bindings: Vec<Uuid>
}

impl Default for ServiceBase {
    fn default() -> Self {
        Self {
            host_id: Uuid::nil(),
            service_type: ServiceType::Unknown,
            name: String::new(),
            ports: Vec::new(),
            interface_bindings: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
}

impl Service {

    pub fn all_discovery_ports() -> Vec<Port> {
        let mut ports: Vec<Port> = ServiceType::iter()
            .flat_map(|s| s.discovery_ports())
            .collect();
            
        ports.sort_by_key(|p| (p.number, p.protocol.to_string()));
        ports.dedup();
        ports
    }

    pub fn all_discovery_endpoints() -> Vec<Endpoint> {
        let mut endpoints: Vec<Endpoint> = ServiceType::iter()
            .flat_map(|s| s.discovery_endpoints())
            .collect();

        endpoints.sort_by_key(|e| (e.protocol.to_string(), e.port.number, e.path.clone().unwrap_or("".to_string())) );
        endpoints.dedup();
        endpoints
    }

    pub fn from_discovery(
        service_type: ServiceType, 
        ip: IpAddr, 
        open_ports: &Vec<Port>, 
        endpoint_responses: &Vec<EndpointResponse>, 
        subnet: &Subnet, 
        mac_address: Option<MacAddress>,
        host_id: &Uuid,
        interface_bindings: &Vec<Uuid>) -> (Option<Self>, Option<Vec<Port>>) {

        if let Ok(result) = service_type.discovery_pattern().matches(open_ports.clone(), endpoint_responses.clone(), ip, subnet, mac_address) {

            tracing::info!("✅ Service {:?} matched for {} with ports {:?}", service_type, ip, open_ports);

            let ports: Vec<Port> = result.into_iter().filter_map(|p| p).collect();
            let ports_for_return = ports.clone();
            let name = service_type.display_name().to_string();

            let service_type = match service_type {
                ServiceType::NetvisorDaemon { .. } => ServiceType::NetvisorDaemon { daemon_id: Uuid::nil() },
                other => other
            };

            let service = Service::new(ServiceBase { host_id: *host_id, service_type, name, ports, interface_bindings: interface_bindings.clone() });
            
            return (Some(service), Some(ports_for_return))
        } else {
            return (None, None)
        }
    }
}