use std::net::{IpAddr, Ipv4Addr};
use anyhow::Error;
use cidr::{IpCidr, Ipv4Cidr};

use crate::server::{
    hosts::types::{
        base::{Host, HostBase}, targets::HostTarget,
    }, interfaces::types::base::{Interface, InterfaceBase}, services::types::{base::{Service, ServiceBase}, ports::Port, types::ServiceType}, subnets::types::base::{Subnet, SubnetBase, SubnetSource, SubnetType}
};

pub fn create_internet_subnet() -> Result<Subnet, Error> {
    let base = SubnetBase {
        name: "Internet".to_string(),
        cidr: IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(0, 0, 0, 0), 0)?),
        subnet_type: SubnetType::Internet,
        description: Some("Internet subnet for external connectivity".to_string()),
        dns_resolvers: Vec::new(),
        gateways: Vec::new(),
        reverse_proxies: Vec::new(),
        hosts: Vec::new(),
        source: SubnetSource::System,
    };

    Ok(Subnet::new(base))
}

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

pub fn create_public_dns_host(internet_subnet: &Subnet) -> (Host, Service) {

    let interface = Interface::new(InterfaceBase {
        subnet_id: internet_subnet.id,
        ip_address: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
        mac_address: None,
        name: Some("Cloudflare DNS".to_string()),
    });

    let base = HostBase {
        name: "Cloudflare".to_string(),
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: HostTarget::Hostname,
        interfaces: vec!(interface.clone()),
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
        interface_bindings: vec!(interface.id)
    });

    host.add_service(dns_service.id);
    
    (host, dns_service)
}