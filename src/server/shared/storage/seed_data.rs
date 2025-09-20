use std::net::{IpAddr, Ipv4Addr};

use crate::server::{
    hosts::types::{
        base::{Host, HostBase}, targets::HostTarget,
    }, services::types::{base::{Service, ServiceBase}, ports::Port, types::ServiceType}
};

pub fn create_internet_connectivity_host() -> Host {

    let base = HostBase {
        name: "Google.com".to_string(),
        hostname: Some("google.com".to_string()),
        description: Some("Google.com".to_string()),
        interfaces: Vec::new(),
        open_ports: Vec::new(),
        services: Vec::new(),
        target: HostTarget::Hostname,
        groups: Vec::new(),
    };
    
    Host::new(base)
}

pub fn create_public_dns_host() -> (Host, Service) {

    let base = HostBase {
        name: "Cloudflare".to_string(),
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: HostTarget::ExternalIp(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))),
        interfaces: Vec::new(),
        open_ports: Vec::new(),
        services: Vec::new(),
        groups: Vec::new(),
    };

    let mut host = Host::new(base);

    let dns_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Cloudflare DNS".to_string(),
        service_type: ServiceType::GenericDnsServer,
        ports: vec!(Port::DNS_UDP, Port::DNS_TCP),
        interface_bindings: Vec::new()
    });

    host.add_service(dns_service.id);
    
    (host, dns_service)
}