use std::net::{IpAddr};

use mac_address::MacAddress;

use crate::server::{services::types::{endpoints::{Endpoint, EndpointResponse}, ports::Port}, subnets::types::base::{Subnet, SubnetType}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    AnyOf(Vec<Pattern>),
    AllOf(Vec<Pattern>),
    AnyPort(Vec<Port>),      // Match if at least one port is open
    AllPort(Vec<Port>),           // Match if ALL of these ports are open
    AnyResponse(Vec<EndpointResponse>), // Match if at least one endpoint response contains the response string
    WebService(&'static str, &'static str), // Match on a string response from a path on endpoints using standard HTTP/HTTPS ports
    IsGatewayIp,
    IsVpnSubnetGateway,
    IsDockerHost,
    NotGatewayIp,
    MacVendor([&'static str; 2]),
    None,
}

pub const UBIQUITI_MAC: [&str; 2] =  ["F09FC2", "788A20"];
pub const TPLINK_MAC: [&str; 2] = ["C46E1F", "AC84C6"];

fn web_service_endpoint_responses(ip: Option<IpAddr>, path: &&str, resp: &&str) -> Vec<EndpointResponse> {
    vec!(
        EndpointResponse{ endpoint: Endpoint::http(ip, path), response: resp.to_string() },
        EndpointResponse{ endpoint: Endpoint::https(ip, path), response: resp.to_string() },
        EndpointResponse{ endpoint: Endpoint::http_alt(ip, path), response: resp.to_string() },
        EndpointResponse{ endpoint: Endpoint::https_alt(ip, path), response: resp.to_string() },
    )
}


impl Pattern {
    pub fn matches(&self, open_ports: Vec<Port>, responses: Vec<EndpointResponse>, ip: IpAddr, subnet: &Subnet, mac_address: Option<MacAddress>) -> bool {
        match self {
            Pattern::AnyOf(patterns) => patterns.iter().any(|p| p.matches(open_ports.clone(), responses.clone(), ip, subnet, mac_address)),
            Pattern::AllOf(patterns) => patterns.iter().all(|p| p.matches(open_ports.clone(), responses.clone(), ip, subnet, mac_address)),
            Pattern::AnyPort(ports) => ports.iter().any(|p| open_ports.contains(p)),
            Pattern::AllPort(ports) => ports.iter().all(|p| open_ports.contains(p)),
            Pattern::AnyResponse(endpoint_responses) => {
                endpoint_responses.iter().any(|expected| {
                    responses.iter().any(|actual| {
                        
                        let resolved_expected_endpoint = if !expected.endpoint.is_resolved() {expected.endpoint.new_with_ip(ip)} else {expected.endpoint.clone()};
                        let resolved_actual_endpoint = if !actual.endpoint.is_resolved() {actual.endpoint.new_with_ip(ip)} else {actual.endpoint.clone()};

                        resolved_actual_endpoint == resolved_expected_endpoint && 
                        actual.response.contains(&expected.response)
                    })
                })
            },
            Pattern::WebService(path, resp) => {
                let endpoints = web_service_endpoint_responses(Some(ip), path, resp);
                Pattern::AnyResponse(endpoints).matches(open_ports, responses, ip, subnet, mac_address)
            }
            Pattern::IsGatewayIp => match ip {
                IpAddr::V4(ipv4) => {
                    let octets = ipv4.octets();
                    octets[3] == 1 || octets[3] == 254
                }
                IpAddr::V6(ipv6) => {
                    let segments = ipv6.segments();
                                                            
                    // Any address ending in ::1 or ::254
                    segments[0..7].iter().all(|&s| s == 0) && 
                    (segments[7] == 1 || segments[7] == 254)
                }
            },
            Pattern::IsVpnSubnetGateway => Pattern::IsGatewayIp.matches(open_ports, responses, ip, subnet, mac_address) && matches!(subnet.base.subnet_type, SubnetType::VpnTunnel),
            Pattern::IsDockerHost => Pattern::IsGatewayIp.matches(open_ports, responses, ip, subnet, mac_address) && matches!(subnet.base.subnet_type, SubnetType::DockerBridge),
            Pattern::NotGatewayIp => !Pattern::IsGatewayIp.matches(open_ports, responses, ip, subnet, mac_address),
            Pattern::MacVendor(vendor_strings) => {
                match mac_address {
                    Some(mac) => vendor_strings.iter().any(|s: &&str| mac.to_string().contains(s)),
                    None => false
                }
            }
            Pattern::None => false
        }
    }

    pub fn ports(&self) -> Vec<Port> {
        match self {
            Pattern::AnyPort(ports) => ports.to_vec(),
            Pattern::AllPort(ports) => ports.to_vec(),
            Pattern::AnyOf(patterns) => patterns.iter().flat_map(|p| p.ports().to_vec()).collect(),
            Pattern::AllOf(patterns) => patterns.iter().flat_map(|p| p.ports().to_vec()).collect(),
            _ => vec!()
        }
    }

    pub fn endpoints(&self) -> Vec<Endpoint> {
        match self {
            Pattern::AnyResponse(endpoint_response) => endpoint_response.iter().map(|er| er.endpoint.clone()).collect(),
            Pattern::WebService(path, resp) => web_service_endpoint_responses(None, path, resp).iter().map(|er| er.endpoint.clone()).collect(),
            Pattern::AnyOf(patterns) => patterns.iter().flat_map(|p| p.endpoints().to_vec()).collect(),
            Pattern::AllOf(patterns) => patterns.iter().flat_map(|p| p.endpoints().to_vec()).collect(),
            _ => vec!()
        }
    }
}