use cidr::Ipv4Cidr;
use std::net::{IpAddr, Ipv4Addr};

use crate::server::{
    discovery::types::base::EntitySource,
    hosts::types::{
        base::{Host, HostBase},
        interfaces::{Interface, InterfaceBase},
        ports::{Port, PortBase},
        targets::{HostTarget, ServiceBinding},
    },
    services::{
        definitions::{client::Client, dns_server::DnsServer, web_service::WebService},
        types::base::{Service, ServiceBase},
    },
    subnets::types::base::{Subnet, SubnetBase, SubnetType},
};

pub fn create_wan_subnet() -> Subnet {
    let base = SubnetBase {
        name: "Internet".to_string(),
        cidr: cidr::IpCidr::V4(
            Ipv4Cidr::new(Ipv4Addr::new(0, 0, 0, 0), 0).expect("Cidr for internet subnet"),
        ),
        description: Some(
            "This subnet uses the 0.0.0.0/0 CIDR as an organizational container for \
       services running on the internet (e.g., public DNS servers, cloud services, etc.)."
                .to_string(),
        ),
        dns_resolvers: vec![],
        gateways: vec![],
        reverse_proxies: vec![],
        hosts: vec![],
        subnet_type: SubnetType::Internet,
        source: EntitySource::System,
    };

    Subnet::new(base)
}

pub fn create_remote_subnet() -> Subnet {
    let base = SubnetBase {
        name: "Remote Network".to_string(),
        cidr: cidr::IpCidr::V4(
            Ipv4Cidr::new(Ipv4Addr::new(0, 0, 0, 0), 0).expect("Cidr for internet subnet"),
        ),
        description: Some(
            "This subnet uses the 0.0.0.0/0 CIDR as an organizational container \
        for hosts on remote networks (e.g., mobile connections, \
        friend's networks, public WiFi, etc.)."
                .to_string(),
        ),
        dns_resolvers: vec![],
        gateways: vec![],
        reverse_proxies: vec![],
        hosts: vec![],
        subnet_type: SubnetType::Remote,
        source: EntitySource::System,
    };

    Subnet::new(base)
}

pub fn create_remote_host(remote_subnet: &Subnet) -> (Host, Service) {
    let interface = Interface::new(InterfaceBase::new_conceptual(remote_subnet));

    let dynamic_port = Port::new(PortBase::new_tcp(0)); // Ephemeral port
    let port_bindings = vec![dynamic_port.id];
    let interface_bindings = vec![interface.id];

    let base = HostBase {
        name: "Mobile Phone".to_string(), // Device type in name, not service
        hostname: None,
        description: Some("A mobile device connecting from a remote network".to_string()),
        interfaces: vec![interface.clone()],
        ports: vec![dynamic_port.clone()],
        services: Vec::new(),
        target: HostTarget::None,
        source: EntitySource::System,
    };

    let mut host = Host::new(base);

    let client_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Client".to_string(),
        service_definition: Box::new(Client),
        port_bindings,
        interface_bindings,
        // groups: Vec::new(),
    });

    host.base.target = HostTarget::ServiceBinding(ServiceBinding {
        port_id: dynamic_port.id,
        interface_id: interface.id,
        service_id: client_service.id,
    });

    host.add_service(client_service.id);
    (host, client_service)
}

pub fn create_internet_connectivity_host(internet_subnet: &Subnet) -> (Host, Service) {
    let interface = Interface::new(InterfaceBase::new_conceptual(internet_subnet));

    let https_port = Port::new(PortBase::Https);
    let port_bindings = vec![https_port.id];
    let interface_bindings = vec![interface.id];

    let base = HostBase {
        name: "Google".to_string(),
        hostname: Some("google.com".to_string()),
        description: Some("Google.com".to_string()),
        interfaces: vec![interface.clone()],
        ports: vec![https_port.clone()],
        services: Vec::new(),
        target: HostTarget::Hostname,
        source: EntitySource::System,
    };

    let mut host = Host::new(base);

    let web_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Google.com".to_string(),
        service_definition: Box::new(WebService),
        port_bindings,
        interface_bindings,
        // groups: Vec::new(),
    });

    host.base.target = HostTarget::ServiceBinding(ServiceBinding {
        port_id: https_port.id,
        interface_id: interface.id,
        service_id: web_service.id,
    });

    host.add_service(web_service.id);

    (host, web_service)
}

pub fn create_public_dns_host(internet_subnet: &Subnet) -> (Host, Service) {
    let mut interface = Interface::new(InterfaceBase::new_conceptual(internet_subnet));
    interface.base.ip_address = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let dns_udp_port = Port::new(PortBase::DnsUdp);
    let port_bindings = vec![dns_udp_port.id];
    let interface_bindings = vec![interface.id];

    let base = HostBase {
        name: "Cloudflare".to_string(),
        hostname: None,
        description: Some("Cloudflare DNS for DNS resolution testing".to_string()),
        target: HostTarget::Hostname,
        interfaces: vec![interface.clone()],
        ports: vec![dns_udp_port.clone()],
        services: Vec::new(),
        source: EntitySource::System,
    };

    let mut host = Host::new(base);

    let dns_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "DNS".to_string(),
        service_definition: Box::new(DnsServer),
        port_bindings,
        interface_bindings,
    });

    host.base.target = HostTarget::ServiceBinding(ServiceBinding {
        port_id: dns_udp_port.id,
        interface_id: interface.id,
        service_id: dns_service.id,
    });

    host.add_service(dns_service.id);

    (host, dns_service)
}
