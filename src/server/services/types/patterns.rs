use std::net::{IpAddr};

use anyhow::Error;
use mac_address::MacAddress;
use mac_oui::Oui;

use crate::server::{services::types::{endpoints::{ApplicationProtocol, Endpoint, EndpointResponse}, ports::Port, types::ServiceDefinition}, subnets::types::base::{Subnet, SubnetType}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Port(Port),                 // Whether or not a specific port matches
    Endpoint(EndpointResponse),         // Whether or not a specific endpoint matches
    AnyOf(Vec<Pattern>),        // Match any of the listed patterns
    AllOf(Vec<Pattern>),        // Must match all of the listed patterns
    AnyPort(Vec<Port>),         // Match if at least one port is open
    AllPort(Vec<Port>),           // Match if ALL of these ports are open
    AnyEndpoints(Vec<EndpointResponse>), // Match if at least one endpoint response contains the response string
    WebService(&'static str, &'static str), // Match on a string response from a path on endpoints using standard HTTP/HTTPS ports
    CustomPortWebService(u16, &'static str, &'static str), // Match on a string response from a path on endpoints using a custom port
    SubnetIsType(SubnetType),
    SubnetIsNotType(SubnetType),
    IsGatewayIp,
    NotGatewayIp,
    IsVpnSubnetGateway,
    IsDockerHost,
    MacVendor(&'static str),
    HasAnyMatchedService,
    AnyMatchedService(fn(&Box<dyn ServiceDefinition>) -> bool),
    AllMatchedService(fn(&Box<dyn ServiceDefinition>) -> bool),
    None,
}

fn custom_port_web_service_endpoints(ip: Option<IpAddr>, port: &u16, path: &&str, resp: &&str) -> Vec<EndpointResponse> {
    vec!{
        EndpointResponse{ endpoint: Endpoint{ ip, protocol: ApplicationProtocol::Http, port: Port::new_tcp(*port), path: Some(path.to_string())}, response: resp.to_string()}
    }
}

fn web_service_endpoint_responses(ip: Option<IpAddr>, path: &&str, resp: &&str) -> Vec<EndpointResponse> {
    vec!(
        EndpointResponse{ endpoint: Endpoint::http(ip, path), response: resp.to_string() },
        EndpointResponse{ endpoint: Endpoint::https(ip, path), response: resp.to_string() },
        EndpointResponse{ endpoint: Endpoint::http_alt(ip, path), response: resp.to_string() },
        EndpointResponse{ endpoint: Endpoint::https_alt(ip, path), response: resp.to_string() },
    )
}

// https://gist.github.com/aallan/b4bb86db86079509e6159810ae9bd3e4
pub struct Vendor {}
impl Vendor {
    pub const PHILIPS: &'static str = "Philips Lighting BV";
    pub const HP: &'static str = "HP Inc.";
    pub const EERO: &'static str = "eero Inc";
    pub const TPLINK: &'static str = "TP-LINK TECHNOLOGIES CO.,LTD";
    pub const UBIQUITI: &'static str = "Ubiquiti Networks Inc";
}

impl Pattern {
    pub fn matches(
        &self, 
        open_ports: Vec<Port>, 
        responses: Vec<EndpointResponse>, 
        ip: IpAddr, 
        subnet: &Subnet, 
        mac_address: Option<MacAddress>,
        matched_service_definitions: &Vec<Box<dyn ServiceDefinition>>) -> Result<Vec<Option<Port>>, Error> {

        let no_match = Err(Error::msg("No match"));

        match self {
            Pattern::Port(port) => {
                if open_ports.contains(port) {
                    Ok(vec![Some(port.clone())])
                } else {
                    no_match
                }
            },

            Pattern::Endpoint(expected) => {
                if responses.iter().any(|actual| {
                    let resolved_expected_endpoint = if !expected.endpoint.is_resolved() {expected.endpoint.use_ip(ip)} else {expected.endpoint.clone()};
                    let resolved_actual_endpoint = if !actual.endpoint.is_resolved() {actual.endpoint.use_ip(ip)} else {actual.endpoint.clone()};

                    resolved_actual_endpoint == resolved_expected_endpoint && actual.response.contains(&expected.response)
                }) {
                    Ok(vec![Some(expected.endpoint.port.clone())])
                } else {
                    no_match
                }
            },

            Pattern::MacVendor(vendor_string) => {
                if let Some(mac) = mac_address {
                    let Ok(oui_db) = Oui::default() else {return no_match};
                    let Ok(Some(entry)) = Oui::lookup_by_mac(&oui_db, &mac.to_string()) else {return no_match};

                    let normalize = |s: &str| -> String {
                        s.trim()
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect()
                    };

                    let vendor_string = normalize(vendor_string);
                    let entry_string = normalize(&entry.company_name);

                    if vendor_string == entry_string {
                        Ok(vec![None])
                    } else {
                        no_match
                    }
                } else {
                    no_match
                }
            },

            Pattern::AnyOf(patterns) => {
                let mut any_matched = false;
                let results = patterns.iter()
                    .filter_map(|p| {
                        match p.matches(open_ports.clone(), responses.clone(), ip, subnet, mac_address, matched_service_definitions) {
                            Ok(results) => {
                                any_matched = true;
                                Some(results)
                            },
                            Err(_) => None
                        }
                    })
                    .flatten()
                    .collect();
                
                if any_matched {
                    Ok(results)
                } else {
                    no_match
                }
            },

            Pattern::AllOf(patterns) => {
                let mut all_matched = true;
                let results = patterns.iter()
                    .filter_map(|p| {
                        match p.matches(open_ports.clone(), responses.clone(), ip, subnet, mac_address, matched_service_definitions) {
                            Ok(results) => Some(results),
                            Err(_) => {
                                all_matched = false;
                                None
                            }
                        }
                    })
                    .flatten()
                    .collect();
                    

                if all_matched {
                    Ok(results)
                } else {
                    no_match
                }
            },

            Pattern::AnyPort(ports) => {
                let matched_ports: Vec<_> = ports.iter()
                    .filter(|p| open_ports.contains(p))
                    .map(|p| Some(p.clone()))
                    .collect();
                
                if matched_ports.is_empty() {
                    no_match
                } else {
                    Ok(matched_ports)
                }
            },

            Pattern::AllPort(ports) => {
                let matched_ports: Vec<_> = ports.iter()
                    .filter(|p| open_ports.contains(p))
                    .map(|p| Some(p.clone()))
                    .collect();
                
                if matched_ports.len() == ports.len() {
                    Ok(matched_ports)
                } else {
                    no_match
                }
            },

            Pattern::AnyEndpoints(endpoint_responses) => {
                let matched_responses: Vec<_> = endpoint_responses.iter()
                    .filter_map(|expected| {
                        match Pattern::Endpoint(expected.clone()).matches(open_ports.clone(), responses.clone(), ip, subnet, mac_address, matched_service_definitions) {
                            Ok(result) => Some(result),
                            Err(_) => None
                        }
                    })
                    .flatten()
                    .collect();
                
                if matched_responses.is_empty() {
                    no_match
                } else {
                    Ok(matched_responses)
                }
            },

            Pattern::WebService(path, resp) => {
                let endpoints = web_service_endpoint_responses(Some(ip), path, resp);
                Pattern::AnyEndpoints(endpoints).matches(open_ports, responses, ip, subnet, mac_address, matched_service_definitions)
            },

            Pattern::CustomPortWebService(port, path, resp) => {
                let endpoints = custom_port_web_service_endpoints(Some(ip), port, path, resp);
                Pattern::AnyEndpoints(endpoints).matches(open_ports, responses, ip, subnet, mac_address, matched_service_definitions)
            },

            Pattern::IsGatewayIp => {
                let is_gateway = match ip {
                    IpAddr::V4(ipv4) => {
                        let octets = ipv4.octets();
                        octets[3] == 1 || octets[3] == 254
                    }
                    IpAddr::V6(ipv6) => {
                        let segments = ipv6.segments();
                        segments[0..7].iter().all(|&s| s == 0) && 
                        (segments[7] == 1 || segments[7] == 254)
                    }
                };
                if is_gateway {Ok(vec!(None))} else {no_match}
            },

            Pattern::NotGatewayIp => {
                let gateway_result = Pattern::IsGatewayIp.matches(open_ports, responses, ip, subnet, mac_address, matched_service_definitions);
                if gateway_result.is_err() {Ok(vec!(None))} else {no_match}
            },

            Pattern::SubnetIsType(subnet_type) => {
                if &subnet.base.subnet_type == subnet_type {Ok(vec!(None))} else {no_match}
            },

            Pattern::SubnetIsNotType(subnet_type) => {
                if &subnet.base.subnet_type != subnet_type {Ok(vec!(None))} else {no_match}
            },

            Pattern::IsVpnSubnetGateway => {
                let gateway_result = Pattern::IsGatewayIp.matches(open_ports.clone(), responses.clone(), ip, subnet, mac_address, matched_service_definitions);
                let is_vpn_subnet = matches!(subnet.base.subnet_type, SubnetType::VpnTunnel);
                if gateway_result.is_ok() && is_vpn_subnet {Ok(vec!(None))} else {no_match}
            },

            Pattern::IsDockerHost => {
                let gateway_result = Pattern::IsGatewayIp.matches(open_ports.clone(), responses.clone(), ip, subnet, mac_address, matched_service_definitions);
                let is_docker_subnet = matches!(subnet.base.subnet_type, SubnetType::DockerBridge);
                if gateway_result.is_ok() && is_docker_subnet {Ok(vec!(None))} else {no_match}
            },

            Pattern::HasAnyMatchedService => {
                if matched_service_definitions.len() > 0  {Ok(vec!(None))} else {no_match}
            },

            Pattern::AnyMatchedService(constraint_function) => {
                let any = matched_service_definitions.iter().any(|s| constraint_function(s));
                if any {Ok(vec!(None))} else {no_match}
            },

            Pattern::AllMatchedService(constraint_function) => {
                let any = matched_service_definitions.iter().all(|s| constraint_function(s));
                if any {Ok(vec!(None))} else {no_match}
            },

            Pattern::None => no_match,
        }
    }

    pub fn ports(&self) -> Vec<Port> {
        match self {
            Pattern::Port(port) => vec!(port.clone()),
            Pattern::AnyPort(ports) => ports.to_vec(),
            Pattern::AllPort(ports) => ports.to_vec(),
            Pattern::AnyOf(patterns) => patterns.iter().flat_map(|p| p.ports().to_vec()).collect(),
            Pattern::AllOf(patterns) => patterns.iter().flat_map(|p| p.ports().to_vec()).collect(),
            _ => vec!()
        }
    }

    pub fn endpoints(&self) -> Vec<Endpoint> {
        match self {
            Pattern::Endpoint(endpoint_response) => vec!(endpoint_response.endpoint.clone()),
            Pattern::AnyEndpoints(endpoint_response) => endpoint_response.iter().map(|er| er.endpoint.clone()).collect(),
            Pattern::WebService(path, resp) => web_service_endpoint_responses(None, path, resp).iter().map(|er| er.endpoint.clone()).collect(),
            Pattern::CustomPortWebService(port, path, resp) => custom_port_web_service_endpoints(None, port, path, resp).iter().map(|er|er.endpoint.clone()).collect(),
            Pattern::AnyOf(patterns) => patterns.iter().flat_map(|p| p.endpoints().to_vec()).collect(),
            Pattern::AllOf(patterns) => patterns.iter().flat_map(|p| p.endpoints().to_vec()).collect(),
            _ => vec!()
        }
    }

    pub fn contains_web_service_pattern(&self) -> bool {
        match self {
            Pattern::WebService(_, _) => true,
            Pattern::AllOf(patterns) | Pattern::AnyOf(patterns) => {
                patterns.iter().any(|p| p.contains_web_service_pattern())
            }
            _ => false
        }
    }
}