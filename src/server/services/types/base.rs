use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use strum::IntoEnumIterator;
use uuid::Uuid;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::{shared::{types::metadata::TypeMetadataProvider}};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum Service {
    // Services with a single specific port they can be identified on
    HomeAssistant{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Plex{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Synology{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    UnifiController{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Proxmox{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Jellyfin{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Emby{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Wireguard{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    NetvisorDaemon{confirmed: bool, name: String, ports: Vec<Port>, daemon_id: Uuid, endpoints: Vec<Endpoint>},
    NetvisorServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Unbound{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},

    Bind9{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    PowerDNS{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    OpenVPN{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    IPsec{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    TailscaleRelay{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    SoftEther{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},

    // Services that can be inferred from a combination of open ports and HTTP responses to endpoint requests
    TrueNAS{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    PiHole{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    AdguardHome{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Router{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    PfSense{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    OpnSense{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Fortigate{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    UnifiAccessPoint{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    TpLinkEap{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    QNAP{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    OpenMediaVault{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    NextCloud{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    PfBlockerNg{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    CUPS{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},

    // Generic categories for unknown devices with specific ports
    GenericNasDevice{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericFileServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericPrintServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericDnsServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericDhcpServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericHttpWebServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericHttpsWebServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},

    // Services that don't have reliable fingerprinting methods available, and can only be manually added
    Workstation{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Switch{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>}, 
    AccessPoint{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Firewall{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>}, 
}

impl Service {

    // fn discovery_patterns(ip: &IpAddr) -> Vec<Pattern> {
    //     ServiceDiscriminants::iter()
    //         .flat_map(|discriminant| discriminant.discovery_patterns(ip))
    //         .collect()
    // }

    pub fn discovery_ports(ip: IpAddr) -> Vec<Port> {
        ServiceDiscriminants::iter()
            .flat_map(|discriminant| discriminant.discovery_ports(ip))
            .collect()
    }

    pub fn discovery_endpoints(ip: IpAddr) -> Vec<Endpoint> {
        ServiceDiscriminants::iter()
            .flat_map(|discriminant| discriminant.discovery_endpoints(ip))
            .collect()
    }

    pub fn from_discovery(discriminant: ServiceDiscriminants, ip: IpAddr, open_ports: &Vec<Port>, endpoint_responses: &Vec<EndpointResponse>) -> Option<Self> where Self: Sized {

        if discriminant.discovery_patterns(ip).iter().all(|p| p.matches(open_ports.clone(), endpoint_responses.clone())) { 
            let name = discriminant.to_string();
            let ports = discriminant.discovery_ports(ip);
            let endpoints = discriminant.discovery_endpoints(ip);
            return
                match discriminant {
                        // Services with a single specific port they can be identified on
                        ServiceDiscriminants::HomeAssistant => Some(Self::HomeAssistant{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Plex => Some(Self::Plex{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Synology => Some(Self::Synology{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::UnifiController => Some(Self::UnifiController{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Proxmox => Some(Self::Proxmox{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Jellyfin => Some(Self::Jellyfin{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Emby => Some(Self::Emby{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Wireguard => Some(Self::Wireguard{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::NetvisorDaemon => Some(Self::NetvisorDaemon{confirmed: false, name, ports, endpoints, daemon_id: Uuid::nil()}),
                        ServiceDiscriminants::NetvisorServer => Some(Self::NetvisorServer{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Unbound => Some(Self::Unbound{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Bind9 => Some(Self::Bind9{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::PowerDNS => Some(Self::PowerDNS{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::OpenVPN => Some(Self::OpenVPN{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::IPsec => Some(Self::IPsec{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::TailscaleRelay => Some(Self::TailscaleRelay{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::SoftEther => Some(Self::SoftEther{confirmed: false, name, ports, endpoints}),

                        // Services that can be inferred from a combination of open ports and HTTP responses to endpoint requests
                        ServiceDiscriminants::PiHole => Some(Self::PiHole {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::AdguardHome => Some(Self::AdguardHome {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::TrueNAS => Some(Self::TrueNAS {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::PfSense => Some(Self::PfSense{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::OpnSense => Some(Self::OpnSense{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Fortigate => Some(Self::Fortigate{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::UnifiAccessPoint => Some(Self::UnifiAccessPoint{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::TpLinkEap => Some(Self::TpLinkEap{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::QNAP => Some(Self::QNAP{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::OpenMediaVault => Some(Self::OpenMediaVault{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::NextCloud => Some(Self::NextCloud{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::PfBlockerNg => Some(Self::PfBlockerNg{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::CUPS => Some(Self::CUPS {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Router => Some(Self::Router {confirmed: false, name, ports, endpoints}),

                        // Generic categories for unknown devices with specific ports
                        ServiceDiscriminants::GenericHttpWebServer => Some(Self::GenericHttpWebServer{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericHttpsWebServer => Some(Self::GenericHttpsWebServer{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericDnsServer => Some(Self::GenericDnsServer {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericDhcpServer => Some(Self::GenericDhcpServer {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericPrintServer => Some(Self::GenericPrintServer {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericNasDevice => Some(Self::GenericNasDevice {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericFileServer => Some(Self::GenericFileServer {confirmed: false, name, ports, endpoints}),

                        // Services that don't have reliable fingerprinting methods available, and can only be manually added
                        ServiceDiscriminants::Switch => Some(Self::Switch {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::AccessPoint => Some(Self::AccessPoint {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Firewall => Some(Self::Firewall {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Workstation => Some(Self::Workstation {confirmed: false, name, ports, endpoints}),
                }
        }
        None
    }

}

impl ServiceDiscriminants {

    fn discovery_ports(&self, ip: IpAddr) -> Vec<Port> {
        self.discovery_patterns(ip).into_iter().flat_map(|p| p.ports()).collect::<Vec<Port>>()
    }

    fn discovery_endpoints(&self, ip: IpAddr) -> Vec<Endpoint> {
        self.discovery_patterns(ip).into_iter().flat_map(|p| p.endpoints()).collect::<Vec<Endpoint>>()
    }

    fn discovery_patterns(&self, ip: IpAddr) -> Vec<Pattern> {
        match &self {

            // Discovery - specific ports
            ServiceDiscriminants::HomeAssistant => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8123))) ),
            ServiceDiscriminants::Plex => vec!( Pattern::AnyPort(vec!(Port::new_tcp(32400))) ),
            ServiceDiscriminants::Synology => vec!( Pattern::AnyPort(vec!(Port::new_tcp(5000), Port::new_tcp(5001))) ),
            ServiceDiscriminants::UnifiController => vec!( Pattern::AnyPort(vec!(Port::new_tcp(2049))) ),
            ServiceDiscriminants::Proxmox => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8006))) ),
            ServiceDiscriminants::Jellyfin => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8096))) ),
            ServiceDiscriminants::Emby => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8920))) ),
            ServiceDiscriminants::Wireguard => vec!( Pattern::AnyPort(vec!(Port::new_udp(51820))) ),
            ServiceDiscriminants::NetvisorDaemon => vec!( Pattern::AnyPort(vec!(Port::new_tcp(3001))) ),
            ServiceDiscriminants::NetvisorServer => vec!( Pattern::AnyPort(vec!(Port::new_tcp(3000))) ),
            ServiceDiscriminants::Unbound => vec!( Pattern::AllPort(vec!(Port::DNS, Port::new_tcp(8953))) ),
            ServiceDiscriminants::Bind9 => vec!( Pattern::AllPort(vec!(Port::DNS, Port::new_tcp(8053))) ),
            ServiceDiscriminants::PowerDNS => vec!( Pattern::AllPort(vec!(Port::DNS, Port::new_tcp(8081))) ),
            ServiceDiscriminants::OpenVPN => vec!( Pattern::AnyPort(vec!(Port::new_udp(1194))) ),
            ServiceDiscriminants::IPsec => vec!( Pattern::AllPort(vec!(Port::new_udp(500), Port::new_udp(4500))) ),
            ServiceDiscriminants::TailscaleRelay => vec!( Pattern::AnyPort(vec!(Port::new_tcp(3478), Port::new_udp(3478))) ),
            ServiceDiscriminants::SoftEther => vec!( Pattern::AnyPort(vec!(Port::new_tcp(443), Port::new_tcp(5555))) ),

            // Discovery - generic ports but look for match on specific HTTP response, see discovery_request_expected_response
            ServiceDiscriminants::PiHole => vec!(
                Pattern::AllPort(vec!(Port::DNS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/admin"),
                        response: "Pi-hole".to_string()
                    },
                    EndpointResponse {
                        endpoint: Endpoint::https(Some(ip), "/admin"),
                        response: "Pi-hole".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::TrueNAS => vec!(
                Pattern::AllPort(vec!(Port::SAMBA)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/"),
                        response: "TrueNAS".to_string()
                    },
                    EndpointResponse {
                        endpoint: Endpoint::https(Some(ip), "/"),
                        response: "TrueNAS".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::PfSense => vec!(
                Pattern::AnyPort(vec!(Port::HTTPS, Port::HTTP)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::https(Some(ip), "/"),
                        response: "pfSense".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::OpnSense => vec!(
                Pattern::AnyPort(vec!(Port::HTTPS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::https(Some(ip), "/"),
                        response: "OPNsense".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::Fortigate => vec!(
                Pattern::AnyPort(vec!(Port::HTTPS, Port::new_tcp(8443))),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::https(Some(ip), "/"),
                        response: "FortiGate".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::UnifiAccessPoint => vec!(
                Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/"),
                        response: "UniFi".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::TpLinkEap => vec!(
                Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/"),
                        response: "TP-LINK".to_string()
                    }
                ))
            ),
            // AdGuard Home
            ServiceDiscriminants::AdguardHome => vec!(
                Pattern::AllPort(vec!(Port::DNS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/"),
                        response: "AdGuard Home".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::QNAP => vec!(
                Pattern::AnyPort(vec!(Port::new_tcp(8080), Port::HTTPS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/"),
                        response: "QNAP".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::OpenMediaVault => vec!(
                Pattern::AllPort(vec!(Port::SAMBA)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/"),
                        response: "OpenMediaVault".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::NextCloud => vec!(
                Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::https(Some(ip), "/"),
                        response: "Nextcloud".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::PfBlockerNg => vec!(
                Pattern::AllPort(vec!(Port::DNS)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::https(Some(ip), "/pfblockerng"),
                        response: "pfBlockerNG".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::CUPS => vec!(
                Pattern::AnyPort(vec!(Port::IPP)),
                Pattern::AnyResponse(vec!(
                    EndpointResponse {
                        endpoint: Endpoint::http(Some(ip), "/"),
                        response: "CUPS".to_string()
                    }
                ))
            ),
            ServiceDiscriminants::Router => vec!(
                Pattern::AllPort(vec!(Port::DHCP)),
                Pattern::AnyPort(vec!(Port::SSH, Port::HTTP, Port::HTTPS))
            ),
            
            ServiceDiscriminants::Switch => vec!( Pattern::AllPort(vec!(Port::SNMP, Port::HTTP)) ),
            ServiceDiscriminants::GenericHttpWebServer => vec!( Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPALT)) ),
            ServiceDiscriminants::GenericHttpsWebServer => vec!( Pattern::AnyPort(vec!(Port::HTTPS, Port::HTTPSALT)) ),
            ServiceDiscriminants::GenericDnsServer => vec!( Pattern::AnyPort(vec!(Port::DNS)) ),
            ServiceDiscriminants::GenericDhcpServer => vec!( Pattern::AnyPort(vec!(Port::DHCP)) ),
            ServiceDiscriminants::GenericPrintServer => vec!( Pattern::AnyPort(vec!(Port::IPP)) ),
            ServiceDiscriminants::GenericNasDevice => vec!( Pattern::AnyPort(vec!(Port::NFS)) ),
            ServiceDiscriminants::GenericFileServer => vec!( Pattern::AnyPort(vec!(Port::FTP)) ),
            ServiceDiscriminants::Workstation => vec!( Pattern::AllPort(vec!(Port::RDP, Port::SAMBA)) ),

            // No unique enough match pattern
            ServiceDiscriminants::AccessPoint | ServiceDiscriminants::Firewall => vec!( Pattern::None )
        }
    }

    pub fn service_category(&self) -> ServiceCategory {
        match self {
            ServiceDiscriminants::HomeAssistant => ServiceCategory::HomeAutomation,
            ServiceDiscriminants::Plex => ServiceCategory::Media,
            ServiceDiscriminants::Synology => ServiceCategory::Storage,
            ServiceDiscriminants::UnifiController => ServiceCategory::NetworkInfrastructure,
            ServiceDiscriminants::Proxmox => ServiceCategory::Media,
            ServiceDiscriminants::Jellyfin => ServiceCategory::Media,
            ServiceDiscriminants::Emby => ServiceCategory::Media,
            ServiceDiscriminants::Wireguard => ServiceCategory::VPN,
            ServiceDiscriminants::NetvisorDaemon => ServiceCategory::Netvisor,
            ServiceDiscriminants::NetvisorServer => ServiceCategory::Netvisor,
            ServiceDiscriminants::Unbound => ServiceCategory::DNS,
            ServiceDiscriminants::Bind9 => ServiceCategory::DNS,
            ServiceDiscriminants::PowerDNS => ServiceCategory::DNS,
            ServiceDiscriminants::OpenVPN => ServiceCategory::VPN,
            ServiceDiscriminants::IPsec => ServiceCategory::VPN,
            ServiceDiscriminants::TailscaleRelay => ServiceCategory::VPN,
            ServiceDiscriminants::SoftEther => ServiceCategory::VPN,

            // Services that can be inferred from a combination of open ports and HTTP responses to endpoint requests
            ServiceDiscriminants::PiHole => ServiceCategory::AdBlock,
            ServiceDiscriminants::AdguardHome => ServiceCategory::AdBlock,
            ServiceDiscriminants::TrueNAS => ServiceCategory::Storage,
            ServiceDiscriminants::PfSense => ServiceCategory::Security,
            ServiceDiscriminants::OpnSense => ServiceCategory::Security,
            ServiceDiscriminants::Fortigate => ServiceCategory::Security,
            ServiceDiscriminants::UnifiAccessPoint => ServiceCategory::NetworkInfrastructure,
            ServiceDiscriminants::TpLinkEap => ServiceCategory::NetworkInfrastructure,
            ServiceDiscriminants::QNAP => ServiceCategory::Storage,
            ServiceDiscriminants::OpenMediaVault => ServiceCategory::Storage,
            ServiceDiscriminants::NextCloud => ServiceCategory::Cloud,
            ServiceDiscriminants::PfBlockerNg => ServiceCategory::AdBlock,
            ServiceDiscriminants::CUPS => ServiceCategory::Print,
            ServiceDiscriminants::Router => ServiceCategory::NetworkInfrastructure,

            // Generic categories for unknown devices with specific ports
            ServiceDiscriminants::GenericHttpWebServer => ServiceCategory::Web,
            ServiceDiscriminants::GenericHttpsWebServer => ServiceCategory::Web,
            ServiceDiscriminants::GenericDnsServer => ServiceCategory::DNS,
            ServiceDiscriminants::GenericDhcpServer => ServiceCategory::NetworkInfrastructure,
            ServiceDiscriminants::GenericPrintServer => ServiceCategory::Print,
            ServiceDiscriminants::GenericNasDevice => ServiceCategory::Storage,
            ServiceDiscriminants::GenericFileServer => ServiceCategory::Files,

            // Services that don't have reliable fingerprinting methods available, and can only be manually added
            ServiceDiscriminants::Switch => ServiceCategory::NetworkInfrastructure,
            ServiceDiscriminants::AccessPoint => ServiceCategory::NetworkInfrastructure,
            ServiceDiscriminants::Firewall => ServiceCategory::Security,
            ServiceDiscriminants::Workstation => ServiceCategory::Device
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceCategory {
    Web,
    NetworkInfrastructure,
    Files,
    Print,
    VPN,
    DNS,
    AdBlock,
    HomeAutomation,
    Media,
    Security,
    Storage,
    Cloud,
    Virtualization,
    Netvisor,
    Device
}

impl ServiceCategory {
    fn category_str(&self) -> &'static str{
        match self {
            ServiceCategory::Web => "Web",
            ServiceCategory::NetworkInfrastructure => "Network Infrastructure",
            ServiceCategory::Files => "Files",
            ServiceCategory::Print => "Print",
            ServiceCategory::VPN => "VPN",
            ServiceCategory::DNS => "DNS",
            ServiceCategory::AdBlock => "Ad Blocker",
            ServiceCategory::HomeAutomation => "Home Automation",
            ServiceCategory::Media => "Media",
            ServiceCategory::Security => "Shield",
            ServiceCategory::Storage => "Storage",
            ServiceCategory::Cloud => "Cloud",
            ServiceCategory::Virtualization => "Virtualization",
            ServiceCategory::Netvisor => "NetVisor",
            ServiceCategory::Device => "Device",
        }
    }
    fn icon(&self) -> &'static str {
        match self {
            ServiceCategory::Web => "Globe",
            ServiceCategory::NetworkInfrastructure => "Network",
            ServiceCategory::Files => "Folder",
            ServiceCategory::Print => "Printer",
            ServiceCategory::VPN => "Shield",
            ServiceCategory::DNS => "Search",
            ServiceCategory::AdBlock => "ShieldCheck",
            ServiceCategory::HomeAutomation => "Home",
            ServiceCategory::Media => "PlayCircle",
            ServiceCategory::Security => "Shield",
            ServiceCategory::Storage => "HardDrive",
            ServiceCategory::Cloud => "Cloud",
            ServiceCategory::Virtualization => "Server",
            ServiceCategory::Netvisor => "Goggles",
            ServiceCategory::Device => "Monitor",
        }
    }
    fn color(&self) -> &'static str {
        match self {
            ServiceCategory::Web => "blue",
            ServiceCategory::NetworkInfrastructure => "yellow",
            ServiceCategory::Files => "purple",
            ServiceCategory::Print => "gray",
            ServiceCategory::VPN => "orange",
            ServiceCategory::DNS => "yellow",
            ServiceCategory::AdBlock => "red",
            ServiceCategory::HomeAutomation => "blue",
            ServiceCategory::Media => "purple",
            ServiceCategory::Security => "red",
            ServiceCategory::Storage => "green",
            ServiceCategory::Cloud => "blue",
            ServiceCategory::Virtualization => "orange",
            ServiceCategory::Netvisor => "purple",
            ServiceCategory::Device => "gray",
        }
    }
}

impl TypeMetadataProvider for ServiceDiscriminants {
    fn id(&self) -> String { 
        self.to_string()
    } 

    fn display_name(&self) -> &str {
        match self {            
            // VPN Services
            ServiceDiscriminants::Wireguard => "WireGuard VPN",
            ServiceDiscriminants::OpenVPN => "OpenVPN",
            ServiceDiscriminants::IPsec => "IPsec VPN",
            ServiceDiscriminants::SoftEther => "SoftEther VPN",
            ServiceDiscriminants::TailscaleRelay => "Tailscale Relay",
            
            // DNS Services
            ServiceDiscriminants::Unbound => "Unbound DNS",
            ServiceDiscriminants::Bind9 => "BIND9 DNS",
            ServiceDiscriminants::PowerDNS => "PowerDNS",
            ServiceDiscriminants::PiHole => "Pi-hole",
            ServiceDiscriminants::AdguardHome => "AdGuard Home",
            ServiceDiscriminants::PfBlockerNg => "pfBlockerNG",
            
            // Media Services
            ServiceDiscriminants::HomeAssistant => "Home Assistant",
            ServiceDiscriminants::Plex => "Plex Media Server",
            ServiceDiscriminants::Jellyfin => "Jellyfin",
            ServiceDiscriminants::Emby => "Emby",
            
            // Network Infrastructure
            ServiceDiscriminants::Router => "Router",
            ServiceDiscriminants::Switch => "Network Switch",
            ServiceDiscriminants::AccessPoint => "Access Point",
            ServiceDiscriminants::Firewall => "Firewall",
            ServiceDiscriminants::PfSense => "pfSense",
            ServiceDiscriminants::OpnSense => "OPNsense",
            ServiceDiscriminants::Fortigate => "FortiGate",
            ServiceDiscriminants::UnifiController => "UniFi Controller",
            ServiceDiscriminants::UnifiAccessPoint => "UniFi Access Point",
            ServiceDiscriminants::TpLinkEap => "TP-Link EAP",
            
            // Storage & NAS
            ServiceDiscriminants::TrueNAS => "TrueNAS",
            ServiceDiscriminants::Synology => "Synology DSM",
            ServiceDiscriminants::QNAP => "QNAP NAS",
            ServiceDiscriminants::OpenMediaVault => "OpenMediaVault",
            ServiceDiscriminants::NextCloud => "Nextcloud",
            
            // Virtualization
            ServiceDiscriminants::Proxmox => "Proxmox VE",
            
            // Print Services
            ServiceDiscriminants::CUPS => "CUPS Print Server",
            
            // NetVisor
            ServiceDiscriminants::NetvisorDaemon => "NetVisor Daemon",
            ServiceDiscriminants::NetvisorServer => "NetVisor Server",
            
            // Device Types
            ServiceDiscriminants::Workstation => "Workstation",
            
            // Generic Services
            ServiceDiscriminants::GenericHttpWebServer => "HTTP Web Server",
            ServiceDiscriminants::GenericHttpsWebServer => "HTTPS Web Server",
            ServiceDiscriminants::GenericDnsServer => "DNS Server",
            ServiceDiscriminants::GenericDhcpServer => "DHCP Server",
            ServiceDiscriminants::GenericPrintServer => "Print Server",
            ServiceDiscriminants::GenericNasDevice => "NAS Device",
            ServiceDiscriminants::GenericFileServer => "File Server",
        }
    }
    
    fn description(&self) -> &str {
        match self {            
            // VPN Services
            ServiceDiscriminants::Wireguard => "Modern VPN service using WireGuard protocol",
            ServiceDiscriminants::OpenVPN => "Open-source VPN solution",
            ServiceDiscriminants::IPsec => "Internet Protocol Security VPN",
            ServiceDiscriminants::SoftEther => "Multi-protocol VPN software",
            ServiceDiscriminants::TailscaleRelay => "Tailscale DERP relay server",
            
            // DNS Services
            ServiceDiscriminants::Unbound => "Validating DNS resolver with control interface",
            ServiceDiscriminants::Bind9 => "Berkeley Internet Name Domain DNS server",
            ServiceDiscriminants::PowerDNS => "Authoritative DNS server with API",
            ServiceDiscriminants::PiHole => "Network-wide ad blocking DNS service",
            ServiceDiscriminants::AdguardHome => "Network-wide ad and tracker blocking",
            ServiceDiscriminants::PfBlockerNg => "pfSense package for DNS/IP blocking",
            
            // Media Services
            ServiceDiscriminants::HomeAssistant => "Open-source home automation platform",
            ServiceDiscriminants::Plex => "Media server for streaming personal content",
            ServiceDiscriminants::Jellyfin => "Free media server for personal streaming",
            ServiceDiscriminants::Emby => "Personal media server with streaming capabilities",
            
            // Network Infrastructure
            ServiceDiscriminants::Router => "Network router providing routing and gateway services",
            ServiceDiscriminants::Switch => "Network switch for local area networking",
            ServiceDiscriminants::AccessPoint => "Wireless access point for WiFi connectivity",
            ServiceDiscriminants::Firewall => "Network security appliance",
            ServiceDiscriminants::PfSense => "Open-source firewall and router platform",
            ServiceDiscriminants::OpnSense => "Open-source firewall and routing platform",
            ServiceDiscriminants::Fortigate => "Fortinet security appliance",
            ServiceDiscriminants::UnifiController => "Ubiquiti UniFi network controller",
            ServiceDiscriminants::UnifiAccessPoint => "Ubiquiti UniFi wireless access point",
            ServiceDiscriminants::TpLinkEap => "TP-Link EAP wireless access point",
            
            // Storage & NAS
            ServiceDiscriminants::TrueNAS => "Open-source network attached storage system",
            ServiceDiscriminants::Synology => "Synology DiskStation Manager NAS system",
            ServiceDiscriminants::QNAP => "QNAP network attached storage system",
            ServiceDiscriminants::OpenMediaVault => "Debian-based NAS solution",
            ServiceDiscriminants::NextCloud => "Self-hosted cloud storage and collaboration platform",
            
            // Virtualization
            ServiceDiscriminants::Proxmox => "Open-source virtualization management platform",
            
            // Print Services
            ServiceDiscriminants::CUPS => "Common Unix Printing System",
            
            // NetVisor
            ServiceDiscriminants::NetvisorDaemon => "NetVisor daemon for enhanced network diagnostics",
            ServiceDiscriminants::NetvisorServer => "NetVisor server for network management",
            
            // Device Types
            ServiceDiscriminants::Workstation => "Desktop computer for productivity work",
            
            // Generic Services
            ServiceDiscriminants::GenericHttpWebServer => "Generic HTTP web server",
            ServiceDiscriminants::GenericHttpsWebServer => "Generic HTTPS web server",
            ServiceDiscriminants::GenericDnsServer => "Generic DNS resolution service",
            ServiceDiscriminants::GenericDhcpServer => "Generic DHCP service",
            ServiceDiscriminants::GenericPrintServer => "Generic printing service",
            ServiceDiscriminants::GenericNasDevice => "Generic network storage device",
            ServiceDiscriminants::GenericFileServer => "Generic file sharing service",
        }
    }
    
    fn category(&self) -> &str {
        self.service_category().category_str()
    }
    
    fn icon(&self) -> &str {
        self.service_category().icon()
    }
    
    fn color(&self) -> &str {
       self.service_category().color()
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}