use std::net::IpAddr;

use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use strum::IntoEnumIterator;
use uuid::Uuid;
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::patterns::{Pattern, TPLINK_MAC, UBIQUITI_MAC};
use crate::server::services::types::ports::Port;
use crate::server::subnets::types::base::Subnet;
use crate::server::{shared::{types::metadata::TypeMetadataProvider}};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[serde(tag = "type")]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum Service {
    // Services with a single specific port they can generally be identified on
    HomeAssistant{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Plex{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Synology{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    UnifiController{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Proxmox{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Jellyfin{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Emby{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    NetvisorDaemon{confirmed: bool, name: String, ports: Vec<Port>, daemon_id: Uuid, endpoints: Vec<Endpoint>},
    NetvisorServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Unbound{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Bind9{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    PowerDNS{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Portainer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    DockerSwarm{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Kubernetes{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Prometheus{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Duplicati{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Syncthing{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Restic{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},

    // Services that can be inferred from a more complex pattern
    TrueNAS{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Grafana{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    UptimeKuma{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    PiHole{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    AdguardHome{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
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
    Traefik{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    NginxProxyManager{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    Cloudflared{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},

    // Generic categories (non-vendor specific)
    GenericRouter{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericVpnGateway{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericNasDevice{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericFileServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericPrintServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericDnsServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericDhcpServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericHttpWebServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericHttpsWebServer{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericSwitch{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>}, 
    GenericAccessPoint{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
    GenericFirewall{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>}, 
    
    
    Workstation{confirmed: bool, name: String, ports: Vec<Port>, endpoints: Vec<Endpoint>},
}

impl Service {

    pub fn discovery_ports() -> Vec<Port> {
        let mut ports: Vec<Port> = ServiceDiscriminants::iter()
            .flat_map(|discriminant| discriminant.discovery_ports())
            .collect();
            
        ports.sort_by_key(|p| (p.number, p.tcp, p.udp));
        ports.dedup();
        ports
    }

    pub fn discovery_endpoints() -> Vec<Endpoint> {
        ServiceDiscriminants::iter()
            .flat_map(|discriminant| discriminant.discovery_endpoints())
            .collect()
    }

    pub fn from_discovery(discriminant: ServiceDiscriminants, ip: IpAddr, open_ports: &Vec<Port>, endpoint_responses: &Vec<EndpointResponse>, subnet: &Subnet, mac_address: Option<MacAddress>) -> Option<Self> where Self: Sized {

        let patterns = discriminant.discovery_patterns();

        if patterns.is_empty() {
            return None;
        }

        let matches = patterns.iter().all(|p| {
            let result = p.matches(open_ports.clone(), endpoint_responses.clone(), ip, subnet, mac_address);
            result
        });

        if matches { 
            tracing::info!("✅ Service {:?} matched for {} with ports {:?}", discriminant, ip, open_ports);
            let name = discriminant.to_string();
            let ports = discriminant.discovery_ports();
            let endpoints = discriminant.discovery_endpoints().iter().map(|e| e.new_with_ip(ip)).collect();
            return match discriminant {
                        // Services with a single specific port they can be identified on
                        ServiceDiscriminants::HomeAssistant => Some(Self::HomeAssistant{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Plex => Some(Self::Plex{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Synology => Some(Self::Synology{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::UnifiController => Some(Self::UnifiController{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Proxmox => Some(Self::Proxmox{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Jellyfin => Some(Self::Jellyfin{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Emby => Some(Self::Emby{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::NetvisorDaemon => Some(Self::NetvisorDaemon{confirmed: false, name, ports, endpoints, daemon_id: Uuid::nil()}),
                        ServiceDiscriminants::NetvisorServer => Some(Self::NetvisorServer{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Unbound => Some(Self::Unbound{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Bind9 => Some(Self::Bind9{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::PowerDNS => Some(Self::PowerDNS{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::DockerSwarm => Some(Self::DockerSwarm{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Portainer => Some(Self::Portainer{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Kubernetes => Some(Self::Kubernetes{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Duplicati => Some(Self::Duplicati{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Restic => Some(Self::Restic{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Syncthing => Some(Self::Syncthing{confirmed: false, name, ports, endpoints}),

                        // Services that can be inferred from a combination of open ports and HTTP responses to endpoint requests
                        ServiceDiscriminants::Grafana => Some(Self::Grafana{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Prometheus => Some(Self::Prometheus{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::UptimeKuma => Some(Self::UptimeKuma{confirmed: false, name, ports, endpoints}),
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
                        ServiceDiscriminants::Cloudflared => Some(Self::Cloudflared {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::NginxProxyManager => Some(Self::NginxProxyManager {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Traefik => Some(Self::Traefik {confirmed: false, name, ports, endpoints}),
                        

                        // Generic categories for unknown devices with specific ports
                        ServiceDiscriminants::GenericRouter => Some(Self::GenericRouter {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericVpnGateway => Some(Self::GenericVpnGateway {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericHttpWebServer => Some(Self::GenericHttpWebServer{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericHttpsWebServer => Some(Self::GenericHttpsWebServer{confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericDnsServer => Some(Self::GenericDnsServer {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericDhcpServer => Some(Self::GenericDhcpServer {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericPrintServer => Some(Self::GenericPrintServer {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericNasDevice => Some(Self::GenericNasDevice {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericFileServer => Some(Self::GenericFileServer {confirmed: false, name, ports, endpoints}),

                        // Services that don't have reliable fingerprinting methods available, and can only be manually added
                        ServiceDiscriminants::GenericSwitch => Some(Self::GenericSwitch {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericAccessPoint => Some(Self::GenericAccessPoint {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::GenericFirewall => Some(Self::GenericFirewall {confirmed: false, name, ports, endpoints}),
                        ServiceDiscriminants::Workstation => Some(Self::Workstation {confirmed: false, name, ports, endpoints}),
                }
        } else {
            return None
        }
    }

}

impl ServiceDiscriminants {

    pub fn is_generic_service(&self) -> bool {
        self.to_string().contains("Generic")
    }

    pub fn discovery_ports(&self) -> Vec<Port> {
        self.discovery_patterns().into_iter().flat_map(|p| p.ports()).collect::<Vec<Port>>()
    }

    fn discovery_endpoints(&self) -> Vec<Endpoint> {
        self.discovery_patterns().into_iter().flat_map(|p| p.endpoints()).collect::<Vec<Endpoint>>()
    }

    fn discovery_patterns(&self) -> Vec<Pattern> {
        match &self {

            // Discovery - specific ports
            ServiceDiscriminants::HomeAssistant => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8123))) ),
            ServiceDiscriminants::Plex => vec!( Pattern::AnyPort(vec!(Port::new_tcp(32400))) ),
            ServiceDiscriminants::UnifiController => vec!( Pattern::AnyPort(vec!(Port::new_tcp(2049))) ),
            ServiceDiscriminants::Proxmox => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8006))) ),
            ServiceDiscriminants::Jellyfin => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8096))) ),
            ServiceDiscriminants::Emby => vec!( Pattern::AnyPort(vec!(Port::new_tcp(8920))) ),
            ServiceDiscriminants::NetvisorDaemon => vec!( Pattern::AnyPort(vec!(Port::new_tcp(60073))) ),
            ServiceDiscriminants::NetvisorServer => vec!( Pattern::AnyPort(vec!(Port::new_tcp(60072))) ),
            ServiceDiscriminants::Unbound => vec!( Pattern::AllPort(vec!(Port::DNS, Port::new_tcp(8953))) ),
            ServiceDiscriminants::Bind9 => vec!( Pattern::AllPort(vec!(Port::DNS, Port::new_tcp(8053))) ),
            ServiceDiscriminants::PowerDNS => vec!( Pattern::AllPort(vec!(Port::DNS, Port::new_tcp(8081))) ),
            ServiceDiscriminants::DockerSwarm => vec!(Pattern::AllPort(vec![Port::new_tcp(2377), Port::new_tcp(7946)])),
            ServiceDiscriminants::Kubernetes => vec!(Pattern::AllOf(vec!(
                Pattern::AllPort(vec![Port::new_tcp(6443)]),
                Pattern::AnyPort(vec!(Port::new_tcp(10250), Port::new_tcp(10259), Port::new_tcp(10257), Port::new_tcp(10256)))
            ))),
            ServiceDiscriminants::Portainer => vec!(Pattern::AnyPort(vec![Port::new_tcp(9000), Port::new_tcp(9443)])),
            ServiceDiscriminants::Prometheus => vec!(Pattern::AnyPort(vec![Port::new_tcp(9090)])),
            ServiceDiscriminants::Restic => vec!(Pattern::AnyPort(vec![Port::new_tcp(8000)])),
            ServiceDiscriminants::Duplicati => vec!(Pattern::AnyPort(vec![Port::new_tcp(8200)])),
            ServiceDiscriminants::Syncthing => vec!(Pattern::AnyPort(vec![Port::new_tcp(8384)])),

            // Discovery - generic ports but look for match on specific HTTP response, see discovery_request_expected_response
            ServiceDiscriminants::Synology => vec!(Pattern::WebService("/", "Synology")),
            ServiceDiscriminants::Grafana => vec!(Pattern::WebService("/", "Grafana")),
            ServiceDiscriminants::UptimeKuma => vec!(Pattern::WebService("/", "Uptime Kuma")),
            ServiceDiscriminants::Traefik => vec!(Pattern::WebService("/dashboard/", "Traefik")),
            ServiceDiscriminants::NginxProxyManager => vec!(Pattern::WebService("/", "Nginx Proxy Manager")),
            ServiceDiscriminants::Cloudflared => vec!(Pattern::WebService("/metrics", "cloudflared")),
            ServiceDiscriminants::PiHole => vec!(Pattern::AllPort(vec!(Port::DNS)), Pattern::WebService("/admin", "Pi-hole")),
            ServiceDiscriminants::TrueNAS => vec!(Pattern::AllPort(vec!(Port::SAMBA)), Pattern::WebService("/", "TrueNAS")),
            ServiceDiscriminants::PfSense => vec!(Pattern::WebService("/", "pfSense")),
            ServiceDiscriminants::OpnSense => vec!(Pattern::WebService("/", "OPNSense")),
            ServiceDiscriminants::Fortigate => vec!(Pattern::WebService("/", "FortiGate")),
            ServiceDiscriminants::UnifiAccessPoint => vec!(Pattern::MacVendor(UBIQUITI_MAC), Pattern::WebService("/", "UniFi")),
            ServiceDiscriminants::TpLinkEap => vec!(Pattern::MacVendor(TPLINK_MAC), Pattern::WebService("/", "TP-LINK")),
            ServiceDiscriminants::AdguardHome => vec!(Pattern::AllPort(vec!(Port::DNS)), Pattern::WebService("/", "AdGuard Home")),
            ServiceDiscriminants::QNAP => vec!(Pattern::WebService("/", "QNAP")),
            ServiceDiscriminants::OpenMediaVault => vec!(Pattern::AllPort(vec!(Port::SAMBA)),Pattern::WebService("/", "OpenMediaVault")),
            ServiceDiscriminants::NextCloud => vec!(Pattern::WebService("/", "Nextcloud")),
            ServiceDiscriminants::PfBlockerNg => vec!(Pattern::AllPort(vec!(Port::DNS)), Pattern::WebService("/pfblockerng", "pfBlockerNG")),
            ServiceDiscriminants::CUPS => vec!(Pattern::AnyPort(vec!(Port::IPP)), Pattern::WebService("/", "CUPS")),
            
            // Generic services
            ServiceDiscriminants::GenericRouter => vec!(
                Pattern::AnyOf(vec![
                    // Primary: Gateway IP with management interface
                    Pattern::AllOf(vec![
                        Pattern::IsGatewayIp,
                        Pattern::AnyPort(vec![Port::SSH, Port::HTTP, Port::HTTPS])
                    ]),
                    // Secondary: Gateway with core services
                    Pattern::AllOf(vec![
                        Pattern::IsGatewayIp, 
                        Pattern::AnyPort(vec![Port::DNS, Port::DHCP])
                    ]),
                    // Tertiary: SNMP + management (managed router, not gateway)
                    Pattern::AllOf(vec![
                        Pattern::AllPort(vec![Port::SNMP, Port::SSH]),
                        Pattern::AnyPort(vec![Port::HTTP, Port::HTTPS])
                    ])
                ])
            ),
            ServiceDiscriminants::GenericSwitch => vec!(
                Pattern::AllOf(vec![
                    Pattern::NotGatewayIp,
                    Pattern::AnyOf(vec![
                        // Managed switch with SNMP
                        Pattern::AllPort(vec![Port::SNMP, Port::HTTP]),
                        // SSH-managed switch
                        Pattern::AllPort(vec![Port::SSH, Port::HTTP]),
                        // Basic web-managed switch
                        Pattern::AllPort(vec![Port::HTTP, Port::TELNET]) // HTTP + Telnet
                    ])
                ])
            ),
            ServiceDiscriminants::GenericVpnGateway => vec!(
                Pattern::IsVpnSubnetGateway,
                Pattern::AnyPort(vec!(Port::SSH, Port::HTTP, Port::HTTPS))
            ),
            
            ServiceDiscriminants::GenericDnsServer => vec!( Pattern::AnyPort(vec!(Port::DNS)) ),
            ServiceDiscriminants::GenericDhcpServer => vec!( Pattern::AnyPort(vec!(Port::DHCP)) ),
            ServiceDiscriminants::GenericPrintServer => vec!( Pattern::AnyPort(vec!(Port::IPP)) ),
            ServiceDiscriminants::GenericNasDevice => vec!( Pattern::AnyPort(vec!(Port::NFS)) ),
            ServiceDiscriminants::GenericFileServer => vec!( Pattern::AnyPort(vec!(Port::FTP)) ),
            ServiceDiscriminants::Workstation => vec!( Pattern::AllPort(vec!(Port::RDP, Port::SAMBA)) ),
            ServiceDiscriminants::GenericHttpWebServer => vec!( Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPALT)) ),
            ServiceDiscriminants::GenericHttpsWebServer => vec!( Pattern::AnyPort(vec!(Port::HTTPS, Port::HTTPSALT)) ),

            // No unique enough match pattern
            ServiceDiscriminants::GenericAccessPoint | ServiceDiscriminants::GenericFirewall => vec!( Pattern::None )
        }
    }

    pub fn service_category(&self) -> ServiceCategory {
        match self {
            // VPN Services
            ServiceDiscriminants::GenericVpnGateway => ServiceCategory::VPN,

            // DNS Services
            ServiceDiscriminants::Unbound => ServiceCategory::DNS,
            ServiceDiscriminants::Bind9 => ServiceCategory::DNS,
            ServiceDiscriminants::PowerDNS => ServiceCategory::DNS,
            ServiceDiscriminants::GenericDnsServer => ServiceCategory::DNS,

            // Home Automation
            ServiceDiscriminants::HomeAssistant => ServiceCategory::HomeAutomation,

            // Ad-block Services
            ServiceDiscriminants::PiHole => ServiceCategory::AdBlock,
            ServiceDiscriminants::AdguardHome => ServiceCategory::AdBlock,
            ServiceDiscriminants::PfBlockerNg => ServiceCategory::AdBlock,

            // Media Services
            ServiceDiscriminants::Plex => ServiceCategory::Media,
            ServiceDiscriminants::Jellyfin => ServiceCategory::Media,
            ServiceDiscriminants::Emby => ServiceCategory::Media,

            // Network Infrastructure
            ServiceDiscriminants::GenericRouter => ServiceCategory::NetworkCore,
            ServiceDiscriminants::UnifiController => ServiceCategory::NetworkAccess,
            ServiceDiscriminants::UnifiAccessPoint => ServiceCategory::NetworkAccess,
            ServiceDiscriminants::TpLinkEap => ServiceCategory::NetworkAccess,
            ServiceDiscriminants::GenericDhcpServer => ServiceCategory::NetworkCore,
            ServiceDiscriminants::GenericSwitch => ServiceCategory::NetworkCore,
            ServiceDiscriminants::GenericAccessPoint => ServiceCategory::NetworkAccess,

            // Storage & NAS
            ServiceDiscriminants::Synology => ServiceCategory::Storage,
            ServiceDiscriminants::TrueNAS => ServiceCategory::Storage,
            ServiceDiscriminants::QNAP => ServiceCategory::Storage,
            ServiceDiscriminants::OpenMediaVault => ServiceCategory::Storage,
            ServiceDiscriminants::GenericNasDevice => ServiceCategory::Storage,
            ServiceDiscriminants::GenericFileServer => ServiceCategory::Storage,
            
            // Backup
            ServiceDiscriminants::Duplicati => ServiceCategory::Backup,
            ServiceDiscriminants::Restic => ServiceCategory::Backup,
            ServiceDiscriminants::Syncthing => ServiceCategory::Backup,

            // Virtualization
            ServiceDiscriminants::Proxmox => ServiceCategory::Virtualization,
            ServiceDiscriminants::DockerSwarm => ServiceCategory::Virtualization,
            ServiceDiscriminants::Portainer => ServiceCategory::Virtualization,
            ServiceDiscriminants::Kubernetes => ServiceCategory::Virtualization,

            // Monitoring
            ServiceDiscriminants::Grafana => ServiceCategory::Monitoring,
            ServiceDiscriminants::Prometheus => ServiceCategory::Monitoring,
            ServiceDiscriminants::UptimeKuma => ServiceCategory::Monitoring,

            // Reverse proxy
            ServiceDiscriminants::Traefik => ServiceCategory::ReverseProxy,
            ServiceDiscriminants::NginxProxyManager => ServiceCategory::ReverseProxy,
            ServiceDiscriminants::Cloudflared => ServiceCategory::ReverseProxy,

            // Print Services
            ServiceDiscriminants::CUPS => ServiceCategory::Printer,
            ServiceDiscriminants::GenericPrintServer => ServiceCategory::Printer,

            // Cloud
            ServiceDiscriminants::NextCloud => ServiceCategory::Web,

            // Security
            ServiceDiscriminants::PfSense => ServiceCategory::NetworkSecurity,
            ServiceDiscriminants::OpnSense => ServiceCategory::NetworkSecurity,
            ServiceDiscriminants::Fortigate => ServiceCategory::NetworkSecurity,
            ServiceDiscriminants::GenericFirewall => ServiceCategory::NetworkSecurity,

            // NetVisor
            ServiceDiscriminants::NetvisorDaemon => ServiceCategory::Netvisor,
            ServiceDiscriminants::NetvisorServer => ServiceCategory::Netvisor,

            // Device Types
            ServiceDiscriminants::Workstation => ServiceCategory::Workstation,

            // Generic Services
            ServiceDiscriminants::GenericHttpWebServer => ServiceCategory::Web,
            ServiceDiscriminants::GenericHttpsWebServer => ServiceCategory::Web,
        }
    }
}

impl TypeMetadataProvider for ServiceDiscriminants {
    fn id(&self) -> String { 
        self.to_string()
    } 

    fn display_name(&self) -> &str {
        match self {            
            // DNS Services
            ServiceDiscriminants::Unbound => "Unbound DNS",
            ServiceDiscriminants::Bind9 => "BIND9 DNS",
            ServiceDiscriminants::PowerDNS => "PowerDNS",

            // Home Automation
            ServiceDiscriminants::HomeAssistant => "Home Assistant",

            // Ad-block Services
            ServiceDiscriminants::PiHole => "Pi-hole",
            ServiceDiscriminants::AdguardHome => "AdGuard Home",
            ServiceDiscriminants::PfBlockerNg => "pfBlockerNG",
            
            // Media Services
            ServiceDiscriminants::Plex => "Plex Media Server",
            ServiceDiscriminants::Jellyfin => "Jellyfin",
            ServiceDiscriminants::Emby => "Emby",
            
            // Network Infrastructure
            ServiceDiscriminants::GenericRouter => "Router",
            ServiceDiscriminants::GenericSwitch => "Network Switch",
            ServiceDiscriminants::GenericAccessPoint => "Access Point",
            ServiceDiscriminants::GenericFirewall => "Firewall",
            ServiceDiscriminants::PfSense => "pfSense",
            ServiceDiscriminants::OpnSense => "OPNsense",
            ServiceDiscriminants::Fortigate => "FortiGate",
            ServiceDiscriminants::UnifiController => "UniFi Controller",
            ServiceDiscriminants::UnifiAccessPoint => "UniFi Access Point",
            ServiceDiscriminants::TpLinkEap => "TP-Link EAP",

            // Reverse proxy
            ServiceDiscriminants::Traefik => "Traefik",
            ServiceDiscriminants::NginxProxyManager => "Nginx Proxy Manager",
            ServiceDiscriminants::Cloudflared => "Cloudflared",
            
            // Storage & NAS
            ServiceDiscriminants::TrueNAS => "TrueNAS",
            ServiceDiscriminants::Synology => "Synology DSM",
            ServiceDiscriminants::QNAP => "QNAP NAS",
            ServiceDiscriminants::OpenMediaVault => "OpenMediaVault",
            ServiceDiscriminants::NextCloud => "Nextcloud",

            // Backup
            ServiceDiscriminants::Duplicati => "Duplicati",
            ServiceDiscriminants::Restic => "Restic",
            ServiceDiscriminants::Syncthing => "Syncthing",

            // Virtualization
            ServiceDiscriminants::Proxmox => "Proxmox VE",
            ServiceDiscriminants::DockerSwarm => "Docker Swarm",
            ServiceDiscriminants::Portainer => "Portainer",
            ServiceDiscriminants::Kubernetes => "Kubernetes",

            // Monitoring
            ServiceDiscriminants::Grafana => "Grafana",
            ServiceDiscriminants::Prometheus => "Prometheus",
            ServiceDiscriminants::UptimeKuma => "UptimeKuma",
            
            // Print Services
            ServiceDiscriminants::CUPS => "CUPS Print Server",
            
            // NetVisor
            ServiceDiscriminants::NetvisorDaemon => "NetVisor Daemon",
            ServiceDiscriminants::NetvisorServer => "NetVisor Server",
            
            // Device Types
            ServiceDiscriminants::Workstation => "Workstation",
            
            // Generic Services
            ServiceDiscriminants::GenericVpnGateway => "VPN Server",
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
            ServiceDiscriminants::GenericVpnGateway => "Generic VPN service",
            
            // DNS Services
            ServiceDiscriminants::Unbound => "Validating DNS resolver with control interface",
            ServiceDiscriminants::Bind9 => "Berkeley Internet Name Domain DNS server",
            ServiceDiscriminants::PowerDNS => "Authoritative DNS server with API",
            ServiceDiscriminants::PiHole => "Network-wide ad blocking DNS service",
            ServiceDiscriminants::AdguardHome => "Network-wide ad and tracker blocking",
            ServiceDiscriminants::PfBlockerNg => "pfSense package for DNS/IP blocking",
            ServiceDiscriminants::GenericDnsServer => "Generic DNS resolution service",
            
            // Home Automation
            ServiceDiscriminants::HomeAssistant => "Open-source home automation platform",

            // Media Services
            ServiceDiscriminants::Plex => "Media server for streaming personal content",
            ServiceDiscriminants::Jellyfin => "Free media server for personal streaming",
            ServiceDiscriminants::Emby => "Personal media server with streaming capabilities",
            
            // Network Infrastructure
            ServiceDiscriminants::GenericRouter => "Network router providing routing and gateway services",
            ServiceDiscriminants::GenericSwitch => "Network switch for local area networking",
            ServiceDiscriminants::GenericAccessPoint => "Wireless access point for WiFi connectivity",
            ServiceDiscriminants::GenericFirewall => "Network security appliance",
            ServiceDiscriminants::PfSense => "Open-source firewall and router platform",
            ServiceDiscriminants::OpnSense => "Open-source firewall and routing platform",
            ServiceDiscriminants::Fortigate => "Fortinet security appliance",
            ServiceDiscriminants::UnifiController => "Ubiquiti UniFi network controller",
            ServiceDiscriminants::UnifiAccessPoint => "Ubiquiti UniFi wireless access point",
            ServiceDiscriminants::TpLinkEap => "TP-Link EAP wireless access point",

            // Reverse proxy
            ServiceDiscriminants::Traefik => "Modern reverse proxy and load balancer",
            ServiceDiscriminants::NginxProxyManager => "Web-based Nginx proxy management interface",
            ServiceDiscriminants::Cloudflared => "Cloudflare tunnel daemon",
            
            // Storage & NAS
            ServiceDiscriminants::TrueNAS => "Open-source network attached storage system",
            ServiceDiscriminants::Synology => "Synology DiskStation Manager NAS system",
            ServiceDiscriminants::QNAP => "QNAP network attached storage system",
            ServiceDiscriminants::OpenMediaVault => "Debian-based NAS solution",
            ServiceDiscriminants::NextCloud => "Self-hosted cloud storage and collaboration platform",

            // Backup
            ServiceDiscriminants::Duplicati => "Cross-platform backup client with encryption",
            ServiceDiscriminants::Restic => "Fast and secure backup program",
            ServiceDiscriminants::Syncthing => "Continuous file synchronization service",
            
            // Virtualization
            ServiceDiscriminants::Proxmox => "Open-source virtualization management platform",
            ServiceDiscriminants::DockerSwarm => "Docker native clustering and orchestration",
            ServiceDiscriminants::Portainer => "Container management web interface",
            ServiceDiscriminants::Kubernetes => "Container orchestration platform",

            // Monitoring
            ServiceDiscriminants::Grafana => "Analytics and monitoring visualization platform",
            ServiceDiscriminants::Prometheus => "Time-series monitoring and alerting system",
            ServiceDiscriminants::UptimeKuma => "Self-hosted uptime monitoring tool",
            
            // Print Services
            ServiceDiscriminants::CUPS => "Common Unix Printing System",
            ServiceDiscriminants::GenericPrintServer => "Generic printing service",
            
            // NetVisor
            ServiceDiscriminants::NetvisorDaemon => "NetVisor daemon for enhanced network diagnostics",
            ServiceDiscriminants::NetvisorServer => "NetVisor server for network management",
            
            // Device Types
            ServiceDiscriminants::Workstation => "Desktop computer for productivity work",
            
            // Generic Services
            ServiceDiscriminants::GenericHttpWebServer => "Generic HTTP web server",
            ServiceDiscriminants::GenericHttpsWebServer => "Generic HTTPS web server",
            ServiceDiscriminants::GenericDhcpServer => "Generic DHCP service",
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
        let default_ports = self.discovery_ports();
        let default_endpoints = self.discovery_ports();
        let can_be_added = match self {
            ServiceDiscriminants::NetvisorDaemon | ServiceDiscriminants::NetvisorServer => false,
            _ => true
        };
        let can_be_dns_resolver = match self.service_category() {
            ServiceCategory::DNS | ServiceCategory::AdBlock => true,
            _ => false
        };
        let can_be_gateway = match self {
            ServiceDiscriminants::GenericVpnGateway | ServiceDiscriminants::GenericRouter => true,
            _ => false
        };
        let is_generic = self.is_generic_service();
        serde_json::json!({
            "default_ports": default_ports, 
            "default_endpoints": default_endpoints, 
            "can_be_added": can_be_added, 
            "can_be_dns_resolver": can_be_dns_resolver,
            "can_be_gateway": can_be_gateway,
            "is_generic": is_generic
        })
    }
}