use std::net::{IpAddr, Ipv4Addr};
use cidr::{Ipv4Cidr};

use crate::server::{
    hosts::types::{
        base::{Host, HostBase}, interfaces::{Interface, InterfaceBase}, ports::{Port, PortBase}, targets::{HostTarget, ServiceBinding}
    }, services::{definitions::{dns_server::DnsServer, web_service::WebService}, types::base::{Service, ServiceBase}}, subnets::types::base::{Subnet, SubnetBase, SubnetSource, SubnetType}
};

pub fn create_wan_subnet() -> Subnet {
    let base = SubnetBase {
        name: "Internet".to_string(),
        cidr: cidr::IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(0, 0, 0, 0), 0).expect("Cidr for internet subnet")),
        description: Some("This subnet uses the 0.0.0.0/0 CIDR as an organizational container for \
       services outside the local network (e.g., public DNS servers, cloud services, etc.).".to_string()),
        dns_resolvers: vec!(),
        gateways: vec!(),
        reverse_proxies: vec!(),
        hosts: vec!(),
        subnet_type: SubnetType::Internet,
        source: SubnetSource::System,
    };

    Subnet::new(base)
}

pub fn create_internet_connectivity_host(internet_subnet: &Subnet) -> (Host, Service) {

    let interface = Interface::new(InterfaceBase::new_internet(internet_subnet));

    let https_port = Port::new(PortBase::Https);
    let port_bindings = vec!(https_port.id);
    let interface_bindings= vec!(interface.id);

    let base = HostBase {
        name: "Google".to_string(),
        hostname: Some("google.com".to_string()),
        description: Some("Google.com".to_string()),
        interfaces: vec!(interface.clone()),
        ports: vec!(https_port.clone()),
        services: Vec::new(),
        target: HostTarget::Hostname,
    };

    let mut host = Host::new(base);

    let web_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Google.com".to_string(),
        service_definition: Box::new(WebService),
        port_bindings,
        interface_bindings,
        groups: Vec::new()
    });

    host.base.target = HostTarget::ServiceBinding(ServiceBinding{port_id: https_port.id, interface_id: interface.id, service_id: web_service.id});

    host.add_service(web_service.id);

    (host, web_service)
}

pub fn create_public_dns_host(internet_subnet: &Subnet) -> (Host, Service) {

    let mut interface = Interface::new(InterfaceBase::new_internet(internet_subnet));
    interface.base.ip_address = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let dns_tcp_port = Port::new(PortBase::DnsTcp);
    let dns_udp_port = Port::new(PortBase::DnsUdp);
    let port_bindings = vec!(dns_tcp_port.id, dns_udp_port.id);
    let interface_bindings = vec!(interface.id);

    let base = HostBase {
        name: "Cloudflare".to_string(),
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: HostTarget::Hostname,
        interfaces: vec!(interface.clone()),
        ports: vec!(dns_tcp_port.clone(), dns_udp_port.clone()),
        services: Vec::new(),
    };

    let mut host = Host::new(base);

    let dns_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Cloudflare DNS".to_string(),
        service_definition: Box::new(DnsServer),
        port_bindings,
        interface_bindings,
        groups: Vec::new()
    });

    host.base.target = HostTarget::ServiceBinding(ServiceBinding{port_id: dns_tcp_port.id, interface_id: interface.id, service_id: dns_service.id});

    host.add_service(dns_service.id);
    
    (host, dns_service)
}