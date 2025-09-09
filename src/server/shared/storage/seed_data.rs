use std::net::{IpAddr, Ipv4Addr};
use crate::server::{
    nodes::types::{
        base::{Node, NodeBase}, targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget},
    },
};

pub fn create_internet_connectivity_node() -> Node {

    let node_target= NodeTarget::Hostname(HostnameTargetConfig { 
        hostname: "google.com".to_string(), 
    });

    let base = NodeBase {
        name: "Google.com".to_string(),
        hostname: None,
        description: Some("Google.com for connectivity testing".to_string()),
        subnets: Vec::new(),
        open_ports: Vec::new(),
        services: vec![
            // Service::GenericHttpsWebServer { 
            //     confirmed: true, 
            //     name: ServiceDiscriminants::GenericHttpsWebServer.display_name().to_string(), 
            //     ports: vec!(),
            //     endpoints: vec!(
            //         Endpoint::https(None, "/")
            //     )
            // }
        ],
        target: node_target,
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
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: node_target.clone(),
        subnets: Vec::new(),
        open_ports: Vec::new(),
        services: vec![
            // Service::GenericDnsServer { 
            //     confirmed: true, 
            //     name: ServiceDiscriminants::GenericDnsServer.display_name().to_string(), 
            //     ports: vec!(),
            //     endpoints: vec!(
            //         Endpoint { 
            //             protocol: ApplicationProtocol::Http, 
            //             ip: Some(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))), 
            //             port: Port::DNS, 
            //             path: None
            //         }
            //     )
            // }
        ],
        node_groups: Vec::new(),
    };
    
    Node::new(base)
}