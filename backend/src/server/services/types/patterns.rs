use std::net::IpAddr;

use anyhow::Error;
use mac_address::MacAddress;
use mac_oui::Oui;

use crate::server::{
    hosts::types::ports::{Port, PortBase},
    services::types::{
        definitions::ServiceDefinition,
        endpoints::{Endpoint, EndpointResponse},
    },
    subnets::types::base::{Subnet, SubnetType},
};

#[derive(Debug, Clone)]
pub enum Pattern {
    /// Whether or not a specific port is open on the host
    Port(PortBase),

    /// Whether or not an endpoint provided a specific response
    Endpoint(EndpointResponse),

    /// Match any of the listed patterns
    AnyOf(Vec<Pattern>),

    /// Must match all of the listed patterns
    AllOf(Vec<Pattern>),

    /// Match if at least one port is open
    AnyPort(Vec<PortBase>),

    /// Match if ALL of these ports are open
    AllPort(Vec<PortBase>),

    /// path, response - match on a string response from a path on endpoints using standard HTTP/HTTPS ports
    WebService(&'static str, &'static str),

    /// Whether the subnet that the host was found on matches a subnet type
    SubnetIsType(SubnetType),

    /// Whether the subnet that the host was found on does not match a subnet type
    SubnetIsNotType(SubnetType),

    /// Whether the host IP is found in the daemon's routing table. WARNING: Using this will automatically classify the service as a Layer3 service, and the service will only be able to bind to interfaces (ports and port bindings will be ignored)
    IsGateway,

    /// Inverse of IsGateway
    IsNotGateway,

    /// Whether the vendor derived from the mac address (https://gist.github.com/aallan/b4bb86db86079509e6159810ae9bd3e4) matches the provided str
    MacVendor(&'static str),

    /// Whether any service has been previously matched
    HasAnyMatchedService,

    /// Whether any previously matched services meets a condition
    AnyMatchedService(fn(&Box<dyn ServiceDefinition>) -> bool),

    /// Whether all previously matched services meet a condition
    AllMatchedService(fn(&Box<dyn ServiceDefinition>) -> bool),

    /// No match pattern (only added manually or by the system)
    None,
}

fn web_service_endpoint_responses(
    ip: Option<IpAddr>,
    path: &str,
    resp: &str,
) -> Vec<EndpointResponse> {
    vec![
        EndpointResponse {
            endpoint: Endpoint::http(ip, path),
            response: resp.to_string(),
        },
        EndpointResponse {
            endpoint: Endpoint::https(ip, path),
            response: resp.to_string(),
        },
        EndpointResponse {
            endpoint: Endpoint::http_alt(ip, path),
            response: resp.to_string(),
        },
        EndpointResponse {
            endpoint: Endpoint::https_alt(ip, path),
            response: resp.to_string(),
        },
    ]
}

// https://gist.github.com/aallan/b4bb86db86079509e6159810ae9bd3e4
pub struct Vendor;
impl Vendor {
    pub const PHILIPS: &'static str = "Philips Lighting BV";
    pub const HP: &'static str = "HP Inc.";
    pub const EERO: &'static str = "eero Inc";
    pub const TPLINK: &'static str = "TP-LINK TECHNOLOGIES CO.,LTD";
    pub const UBIQUITI: &'static str = "Ubiquiti Networks Inc";
}

#[derive(Debug, Clone)]
pub struct PatternParams<'a> {
    pub open_ports: &'a [PortBase],
    pub responses: &'a [EndpointResponse],
    pub ip: &'a IpAddr,
    pub subnet: &'a Subnet,
    pub mac_address: &'a Option<MacAddress>,
    pub gateway_ips: &'a [IpAddr],
    pub matched_service_definitions: &'a Vec<Box<dyn ServiceDefinition>>,
}

impl Pattern {
    pub fn matches(&self, params: PatternParams) -> Result<Vec<Option<Port>>, Error> {
        // Return ports that matched if any

        let PatternParams {
            open_ports,
            responses,
            ip,
            subnet,
            mac_address,
            gateway_ips,
            matched_service_definitions,
        } = params;

        let no_match = Err(Error::msg("No match"));

        match self {
            Pattern::Port(port_base) => {
                if let Some(matched_port) = open_ports.iter().find(|p| **p == *port_base) {
                    Ok(vec![Some(Port::new(matched_port.clone()))])
                } else {
                    no_match
                }
            }

            Pattern::Endpoint(expected) => {
                // At matching time, both endpoints are resolved
                if let Some(actual) = responses.iter().find(|actual| {
                    actual.endpoint == expected.endpoint
                        && actual.response.contains(&expected.response)
                }) {
                    Ok(vec![Some(Port::new(actual.endpoint.port_base.clone()))])
                } else {
                    no_match
                }
            }

            Pattern::MacVendor(vendor_string) => {
                if let Some(mac) = mac_address {
                    let Ok(oui_db) = Oui::default() else {
                        return no_match;
                    };
                    let Ok(Some(entry)) = Oui::lookup_by_mac(&oui_db, &mac.to_string()) else {
                        return no_match;
                    };

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
            }

            Pattern::AnyOf(patterns) => {
                let mut any_matched = false;
                let results = patterns
                    .iter()
                    .filter_map(|p| match p.matches(params.clone()) {
                        Ok(results) => {
                            any_matched = true;
                            Some(results)
                        }
                        Err(_) => None,
                    })
                    .flatten()
                    .collect();

                if any_matched {
                    Ok(results)
                } else {
                    no_match
                }
            }

            Pattern::AllOf(patterns) => {
                let mut all_matched = true;
                let results = patterns
                    .iter()
                    .filter_map(|p| match p.matches(params.clone()) {
                        Ok(results) => Some(results),
                        Err(_) => {
                            all_matched = false;
                            None
                        }
                    })
                    .flatten()
                    .collect();

                if all_matched {
                    Ok(results)
                } else {
                    no_match
                }
            }

            Pattern::AnyPort(port_bases) => {
                let matched_ports: Vec<Option<Port>> = open_ports
                    .iter()
                    .filter(|p| port_bases.contains(p))
                    .map(|p| Some(Port::new(p.clone())))
                    .collect();

                if matched_ports.is_empty() {
                    no_match
                } else {
                    Ok(matched_ports)
                }
            }

            Pattern::AllPort(port_bases) => {
                let matched_ports: Vec<Option<Port>> = open_ports
                    .iter()
                    .filter(|p| port_bases.contains(p))
                    .map(|p| Some(Port::new(p.clone())))
                    .collect();

                if matched_ports.len() == port_bases.len() {
                    Ok(matched_ports)
                } else {
                    no_match
                }
            }

            Pattern::WebService(path, resp) => {
                let endpoints = web_service_endpoint_responses(Some(*ip), path, resp)
                    .into_iter()
                    .map(Pattern::Endpoint)
                    .collect();
                Pattern::AnyOf(endpoints).matches(params)
            }

            Pattern::IsGateway => {
                let gateway_ips_in_subnet: Vec<_> = gateway_ips
                    .iter()
                    .filter(|g| subnet.base.cidr.contains(g))
                    .collect();

                let count_gateways_in_subnet = gateway_ips_in_subnet.len();
                let host_ip_in_routing_table = gateway_ips_in_subnet.contains(&ip);

                let last_octet_1_or_254 = match ip {
                    IpAddr::V4(ipv4) => {
                        let octets = ipv4.octets();
                        octets[3] == 1 || octets[3] == 254
                    }
                    IpAddr::V6(ipv6) => {
                        let segments = ipv6.segments();
                        segments[7] == 1 || segments[7] == 254
                    }
                };

                let is_gateway = if host_ip_in_routing_table {
                    // Definitely a gateway if in routing table
                    true
                } else if last_octet_1_or_254 && count_gateways_in_subnet == 0 {
                    // Likely a gateway if common IP and no other gateways found
                    true
                } else {
                    false
                };

                if is_gateway {
                    Ok(vec![None])
                } else {
                    no_match
                }
            }

            Pattern::IsNotGateway => {
                if Pattern::IsGateway.matches(params).is_err() {
                    Ok(vec![None])
                } else {
                    no_match
                }
            }

            Pattern::SubnetIsType(subnet_type) => {
                if &subnet.base.subnet_type == subnet_type {
                    Ok(vec![None])
                } else {
                    no_match
                }
            }

            Pattern::SubnetIsNotType(subnet_type) => {
                if &subnet.base.subnet_type != subnet_type {
                    Ok(vec![None])
                } else {
                    no_match
                }
            }

            Pattern::HasAnyMatchedService => {
                if matched_service_definitions.is_empty() {
                    Ok(vec![None])
                } else {
                    no_match
                }
            }

            Pattern::AnyMatchedService(constraint_function) => {
                let any = matched_service_definitions.iter().any(constraint_function);
                if any {
                    Ok(vec![None])
                } else {
                    no_match
                }
            }

            Pattern::AllMatchedService(constraint_function) => {
                let any = matched_service_definitions.iter().all(constraint_function);
                if any {
                    Ok(vec![None])
                } else {
                    no_match
                }
            }

            Pattern::None => no_match,
        }
    }

    pub fn ports(&self) -> Vec<PortBase> {
        match self {
            Pattern::Port(port) => vec![port.clone()],
            Pattern::Endpoint(response) => vec![response.endpoint.port_base.clone()],
            Pattern::AnyPort(ports) => ports.clone(),
            Pattern::AllPort(ports) => ports.clone(),
            Pattern::AnyOf(patterns) => patterns.iter().flat_map(|p| p.ports().to_vec()).collect(),
            Pattern::AllOf(patterns) => patterns.iter().flat_map(|p| p.ports().to_vec()).collect(),
            _ => vec![],
        }
    }

    pub fn endpoints(&self) -> Vec<Endpoint> {
        match self {
            Pattern::Endpoint(endpoint_response) => vec![endpoint_response.endpoint.clone()],
            Pattern::WebService(path, resp) => web_service_endpoint_responses(None, path, resp)
                .iter()
                .map(|er| er.endpoint.clone())
                .collect(),
            Pattern::AnyOf(patterns) => patterns
                .iter()
                .flat_map(|p| p.endpoints().to_vec())
                .collect(),
            Pattern::AllOf(patterns) => patterns
                .iter()
                .flat_map(|p| p.endpoints().to_vec())
                .collect(),
            _ => vec![],
        }
    }

    pub fn contains_gateway_ip_pattern(&self) -> bool {
        match self {
            Pattern::IsGateway => true,
            Pattern::AllOf(patterns) | Pattern::AnyOf(patterns) => {
                patterns.iter().any(|p| p.contains_gateway_ip_pattern())
            }
            _ => false,
        }
    }

    pub fn contains_web_service_pattern(&self) -> bool {
        match self {
            Pattern::WebService(_, _) => true,
            Pattern::AllOf(patterns) | Pattern::AnyOf(patterns) => {
                patterns.iter().any(|p| p.contains_web_service_pattern())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use crate::{
        server::{
            hosts::types::ports::PortBase,
            services::{
                definitions::ServiceDefinitionRegistry,
                types::{
                    endpoints::{Endpoint, EndpointResponse},
                    patterns::{Pattern, PatternParams},
                },
            },
        },
        tests::subnet,
    };

    #[tokio::test]
    async fn test_pattern_port_matching() {
        // Test pi-hole service
        let pi =
            ServiceDefinitionRegistry::find_by_id("Pi-Hole").expect("Pi-hole service not found");

        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 4, 35));

        let mut params = PatternParams {
            open_ports: &vec![PortBase::DnsUdp, PortBase::DnsTcp],
            responses: &vec![EndpointResponse {
                endpoint: Endpoint::http(Some(ip), "/admin"),
                response: "Pi-hole".to_string(),
            }],
            ip: &ip,
            subnet: &subnet(),
            mac_address: &None,
            gateway_ips: &vec![],
            matched_service_definitions: &vec![],
        };

        let result = pi.discovery_pattern().matches(params.clone());

        assert!(
            result.is_ok(),
            "Pi-hole pattern should match port 53 and admin endpoint"
        );

        // Test with wrong port - should not match
        let open_ports = vec![PortBase::new_tcp(80)];
        params.open_ports = &open_ports;

        let result = pi.discovery_pattern().matches(params);

        assert!(result.is_err(), "SSH pattern should not match port 80");
    }

    #[test]
    fn test_pattern_and_logic() {
        // Create AND pattern requiring both port 80 and 443
        let pattern = Pattern::AllOf(vec![
            Pattern::Port(PortBase::new_tcp(80)),
            Pattern::Port(PortBase::new_tcp(443)),
        ]);

        let mut params = PatternParams {
            open_ports: &vec![PortBase::new_tcp(80), PortBase::new_tcp(443)],
            responses: &vec![],
            ip: &IpAddr::V4(Ipv4Addr::new(192, 168, 4, 35)),
            subnet: &subnet(),
            mac_address: &None,
            gateway_ips: &vec![],
            matched_service_definitions: &vec![],
        };

        let result = pattern.matches(params.clone());

        assert!(
            result.is_ok(),
            "AND pattern should match when both conditions met"
        );

        // Test with only one port - should not match
        let open_ports = vec![PortBase::new_tcp(80)];
        params.open_ports = &open_ports;

        let result = pattern.matches(params.clone());

        assert!(
            result.is_err(),
            "AND pattern should not match when only one condition met"
        );

        // Test with neither port - should not match
        let open_ports = vec![PortBase::new_tcp(22)];
        params.open_ports = &open_ports;

        let result = pattern.matches(params.clone());

        assert!(
            result.is_err(),
            "AND pattern should not match when no conditions met"
        );
    }

    #[test]
    fn test_pattern_or_logic() {
        // Create OR pattern for database ports (MySQL or PostgreSQL)
        let pattern = Pattern::AnyOf(vec![
            Pattern::Port(PortBase::new_tcp(3306)), // MySQL
            Pattern::Port(PortBase::new_tcp(5432)), // PostgreSQL
        ]);

        let mut params = PatternParams {
            open_ports: &vec![PortBase::new_tcp(3306)],
            responses: &vec![],
            ip: &IpAddr::V4(Ipv4Addr::new(192, 168, 4, 35)),
            subnet: &subnet(),
            mac_address: &None,
            gateway_ips: &vec![],
            matched_service_definitions: &vec![],
        };

        let result = pattern.matches(params.clone());
        assert!(result.is_ok(), "OR pattern should match MySQL port");

        // Test with PostgreSQL port - should match
        let open_ports = vec![PortBase::new_tcp(5432)];
        params.open_ports = &open_ports;
        let result = pattern.matches(params.clone());
        assert!(result.is_ok(), "OR pattern should match PostgreSQL port");

        // Test with both ports - should match
        let open_ports = vec![PortBase::new_tcp(3306), PortBase::new_tcp(5432)];
        params.open_ports = &open_ports;
        let result = pattern.matches(params.clone());
        assert!(result.is_ok(), "OR pattern should match with both ports");

        // Test with neither port - should not match
        let open_ports = vec![PortBase::new_tcp(22)];
        params.open_ports = &open_ports;
        let result = pattern.matches(params.clone());
        assert!(
            result.is_err(),
            "OR pattern should not match when no conditions met"
        );
    }
}
