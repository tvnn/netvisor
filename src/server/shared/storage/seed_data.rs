use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;
use crate::server::services::types::base::{Service, ServiceDiscriminants};
use crate::server::services::types::endpoints::Endpoint;
use crate::server::services::types::ports::{ApplicationProtocol, Port};
use crate::server::shared::types::metadata::TypeMetadataProvider;
use crate::server::{
    nodes::types::{
        base::{Node, NodeBase}, status::NodeStatus, targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget}, types::NodeType
    },
};

pub fn create_internet_connectivity_node(dns_id: Uuid) -> Node {

    let node_target= NodeTarget::Hostname(HostnameTargetConfig { 
        hostname: "google.com".to_string(), 
    });

    let base = NodeBase {
        name: "Google.com".to_string(),
        node_type: NodeType::WebServer,
        hostname: None,
        description: Some("Google.com for connectivity testing".to_string()),
        subnets: Vec::new(),
        discovery_status: None,
        services: vec![
            Service::GenericHttpsWebServer { 
                confirmed: true, 
                name: ServiceDiscriminants::GenericHttpsWebServer.display_name().to_string(), 
                ports: vec!(),
                endpoints: vec!(
                    Endpoint::https(None, "/")
                )
            }
        ],
        target: node_target,
        dns_resolver_node_id: Some(dns_id.to_string()),
        status: NodeStatus::Unknown,
        monitoring_interval: 0,
        node_groups: Vec::new(),
    };
    
    Node::new(base)
}

pub fn create_public_dns_node() -> Node {

    let node_target = NodeTarget::IpAddress(IpAddressTargetConfig { 
        ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 
    });

    let base = NodeBase {
        name: "Cloudflare".to_string(),
        node_type: NodeType::DnsServer,
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: node_target.clone(),
        subnets: Vec::new(),
        discovery_status: None,
        services: vec![
            Service::GenericDnsServer { 
                confirmed: true, 
                name: ServiceDiscriminants::GenericDnsServer.display_name().to_string(), 
                ports: vec!(),
                endpoints: vec!(
                    Endpoint { 
                        protocol: ApplicationProtocol::Http, 
                        ip: Some(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))), 
                        port: Port::DNS, 
                        path: None
                    }
                )
            }
        ],
        dns_resolver_node_id: None,
        status: NodeStatus::Unknown,
        monitoring_interval: 0,
        node_groups: Vec::new(),
    };
    
    Node::new(base)
}