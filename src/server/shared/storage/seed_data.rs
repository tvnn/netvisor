use std::net::{IpAddr, Ipv4Addr};
use crate::server::{
    hosts::types::{
        base::{Host, HostBase}, targets::{HostnameTargetConfig, IpAddressTargetConfig, HostTarget},
    },
};

pub fn create_internet_connectivity_host() -> Host {

    let host_target = HostTarget::Hostname(HostnameTargetConfig { 
        hostname: "google.com".to_string(), 
    });

    let base = HostBase {
        name: "Google.com".to_string(),
        hostname: None,
        description: Some("Google.com for connectivity testing".to_string()),
        interfaces: Vec::new(),
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
        target: host_target ,
        groups: Vec::new(),
    };
    
    Host::new(base)
}

pub fn create_public_dns_host() -> Host {

    let host_target  = HostTarget::IpAddress(IpAddressTargetConfig { 
        ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 
    });

    let base = HostBase {
        name: "Cloudflare".to_string(),
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: host_target .clone(),
        interfaces: Vec::new(),
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
        groups: Vec::new(),
    };
    
    Host::new(base)
}