use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;
use crate::server::{
    nodes::types::{
        base::Node, capabilities::{CapabilityConfig, NodeCapability}, criticality::TestCriticality, targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget}, tests::AssignedTest, types::NodeType
    }, tests::types::{
        base::Test,
        configs::ConnectivityConfig,
    }
};

pub fn create_internet_connectivity_node(dns_id: Uuid) -> Node {

    let mut node = Node::from_name("Google.com".to_string());

    let connectivity_capability = NodeCapability::HttpsService{ path: Some("/".to_string()), config: CapabilityConfig::from_port(443) };
    
    node.base.description = Some("Google.com for connectivity testing".to_string());
    node.base.target = NodeTarget::Hostname(HostnameTargetConfig { 
        hostname: "google.com".to_string(), 
    });
    node.base.node_type = NodeType::WebServer;
    node.base.monitoring_interval = 0;
    node.base.capabilities = vec![connectivity_capability.clone()];
    node.base.assigned_tests = vec![
        AssignedTest {
            test: Test::Connectivity(ConnectivityConfig { capability: connectivity_capability, timeout_ms: Some(5000), dns_resolver_node: Some(dns_id) }),
            criticality: TestCriticality::Critical
        }
    ];

    node
}

pub fn create_public_dns_node() -> Node {

    let mut node = Node::from_name("Cloudflare DNS".to_string());
    
    node.base.description = Some("Cloudflare DNS for DNS resolution testing".to_string());
    node.base.target = NodeTarget::IpAddress(IpAddressTargetConfig { 
        ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 
    }); 
    node.base.node_type = NodeType::DnsServer;
    node.base.monitoring_interval = 0;
    node.base.capabilities = vec![
        NodeCapability::DnsService{ config: CapabilityConfig::from_port(53) },
    ];

    node
}