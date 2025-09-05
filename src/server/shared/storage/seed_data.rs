use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;
use crate::server::{
    capabilities::types::base::Capability, nodes::types::{
        base::{Node, NodeBase}, status::NodeStatus, targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget}, types::NodeType
    },
};

pub fn create_internet_connectivity_node(dns_id: Uuid) -> Node {

    let base = NodeBase {
        name: "Google.com".to_string(),
        node_type: NodeType::WebServer,
        hostname: None,
        description: Some("Google.com for connectivity testing".to_string()),
        target: NodeTarget::Hostname(HostnameTargetConfig { 
            hostname: "google.com".to_string(), 
        }),
        subnets: Vec::new(),
        discovery_status: None,
        capabilities: vec![
            Capability::from_port(Some(443)).expect("HTTPS capability maps to 443")
        ],
        dns_resolver_node_id: Some(dns_id.to_string()),
        status: NodeStatus::Unknown,
        monitoring_interval: 0,
        node_groups: Vec::new(),
    };
    
    Node::new(base)
}

pub fn create_public_dns_node() -> Node {

    let base = NodeBase {
        name: "Cloudflare".to_string(),
        node_type: NodeType::DnsServer,
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: NodeTarget::IpAddress(IpAddressTargetConfig { 
            ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 
        }),
        subnets: Vec::new(),
        discovery_status: None,
        capabilities: vec![
            Capability::from_port(Some(53)).expect("DNS capability maps to 443")
        ],
        dns_resolver_node_id: None,
        status: NodeStatus::Unknown,
        monitoring_interval: 0,
        node_groups: Vec::new(),
    };
    
    Node::new(base)
}