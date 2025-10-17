use cidr::Ipv4Cidr;
use std::net::{IpAddr, Ipv4Addr};

use crate::server::{
    discovery::types::base::EntitySource,
    hosts::types::{
        base::{Host, HostBase},
        interfaces::{Interface, InterfaceBase},
        ports::{Port, PortBase},
        targets::HostTarget,
    },
    services::{
        definitions::{client::Client, dns_server::DnsServer, web_service::WebService},
        types::{
            base::{Service, ServiceBase},
            bindings::Binding,
        },
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
        subnet_type: SubnetType::Remote,
        source: EntitySource::System,
    };

    Subnet::new(base)
}

pub fn create_remote_host(remote_subnet: &Subnet) -> (Host, Service) {
    let interface = Interface::new(InterfaceBase::new_conceptual(remote_subnet));

    let dynamic_port = Port::new(PortBase::new_tcp(0)); // Ephemeral port
    let binding = Binding::new_l4(dynamic_port.id, Some(interface.id));
    let binding_id = binding.id();

    let base = HostBase {
        name: "Mobile Device".to_string(), // Device type in name, not service
        hostname: None,
        description: Some("A mobile device connecting from a remote network".to_string()),
        interfaces: vec![interface],
        ports: vec![dynamic_port],
        services: Vec::new(),
        target: HostTarget::None,
        source: EntitySource::System,
        virtualization: None,
    };

    let mut host = Host::new(base);

    let client_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Mobile Device".to_string(),
        service_definition: Box::new(Client),
        bindings: vec![binding],
        virtualization: None,
        vms: vec![],
        containers: vec![],
        source: EntitySource::System,
    });

    host.base.target = HostTarget::ServiceBinding(binding_id);

    host.add_service(client_service.id);
    (host, client_service)
}

pub fn create_internet_connectivity_host(internet_subnet: &Subnet) -> (Host, Service) {
    let interface = Interface::new(InterfaceBase::new_conceptual(internet_subnet));

    let https_port = Port::new(PortBase::Https);
    let binding = Binding::new_l4(https_port.id, Some(interface.id));
    let binding_id = binding.id();

    let base = HostBase {
        name: "Google.com".to_string(),
        hostname: Some("google.com".to_string()),
        description: Some("Google.com".to_string()),
        interfaces: vec![interface],
        ports: vec![https_port],
        services: Vec::new(),
        target: HostTarget::Hostname,
        source: EntitySource::System,
        virtualization: None,
    };

    let mut host = Host::new(base);

    let web_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Google.com".to_string(),
        service_definition: Box::new(WebService),
        bindings: vec![binding],
        virtualization: None,
        vms: vec![],
        containers: vec![],
        source: EntitySource::System,
    });

    host.base.target = HostTarget::ServiceBinding(binding_id);

    host.add_service(web_service.id);

    (host, web_service)
}

pub fn create_public_dns_host(internet_subnet: &Subnet) -> (Host, Service) {
    let mut interface = Interface::new(InterfaceBase::new_conceptual(internet_subnet));
    interface.base.ip_address = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let dns_udp_port = Port::new(PortBase::DnsUdp);
    let binding = Binding::new_l4(dns_udp_port.id, Some(interface.id));
    let binding_id = binding.id();

    let base = HostBase {
        name: "Cloudflare DNS".to_string(),
        hostname: None,
        description: Some("Cloudflare DNS".to_string()),
        target: HostTarget::None,
        interfaces: vec![interface],
        ports: vec![dns_udp_port],
        services: Vec::new(),
        source: EntitySource::System,
        virtualization: None,
    };

    let mut host = Host::new(base);

    let dns_service = Service::new(ServiceBase {
        host_id: host.id,
        name: "Cloudflare DNS".to_string(),
        service_definition: Box::new(DnsServer),
        bindings: vec![binding],
        virtualization: None,
        vms: vec![],
        containers: vec![],
        source: EntitySource::System,
    });

    host.base.target = HostTarget::ServiceBinding(binding_id);

    host.add_service(dns_service.id);

    (host, dns_service)
}
