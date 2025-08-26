use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;
use crate::server::{
    nodes::{capabilities::{base::NodeCapability, dns::DnsServiceCapability, http::{HttpServiceCapability, HttpsServiceCapability}}, types::{
        base::Node, criticality::TestCriticality, targets::{IpAddressTargetConfig, NodeTarget, ServiceTargetConfig}, tests::AssignedTest, types::NodeType
    }}, shared::types::protocols::ApplicationProtocol, tests::types::{
        base::Test,
        configs::ConnectivityConfig,
    }
};

pub fn create_internet_connectivity_node(dns_id: Uuid) -> Node {

    let mut node = Node::from_name("Google.com".to_string());
    
    node.base.description = Some("Google.com for connectivity testing".to_string());
    node.base.target = NodeTarget::Service(ServiceTargetConfig { 
        protocol: ApplicationProtocol::Https, 
        hostname: "google.com".to_string(), 
        port: Some(443), 
        path: Some("/".to_string()) });
    node.base.node_type = NodeType::WebServer;
    node.base.monitoring_interval = 0;
    node.base.capabilities = vec![
        NodeCapability::HttpsService(HttpsServiceCapability {  }),
        NodeCapability::HttpService(HttpServiceCapability {  })
    ];
    node.base.assigned_tests = vec![
        AssignedTest {
            test: Test::Connectivity(ConnectivityConfig { timeout_ms: Some(5000), dns_resolver_node: Some(dns_id) }),
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
        port: Some(53)
    }); 
    node.base.node_type = NodeType::DnsServer;
    node.base.monitoring_interval = 0;
    node.base.capabilities = vec![
        NodeCapability::DnsService(DnsServiceCapability {  }),
    ];

    node
}