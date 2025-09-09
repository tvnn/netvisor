use std::net::IpAddr;

use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use strum::IntoEnumIterator;
use uuid::Uuid;
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::endpoints::{Endpoint, EndpointResponse};
use crate::server::services::types::patterns::{Pattern, Vendor};
use crate::server::services::types::ports::{Port};
use crate::server::subnets::types::base::{Subnet, SubnetType};
use crate::server::{shared::{types::metadata::TypeMetadataProvider}};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[serde(tag = "type")]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum Service {
    // Services with a single specific port they can generally be identified on
    HomeAssistant{name: String, ports: Vec<Port>},
    Plex{name: String, ports: Vec<Port>},
    Synology{name: String, ports: Vec<Port>},
    UnifiController{name: String, ports: Vec<Port>},
    Proxmox{name: String, ports: Vec<Port>},
    Jellyfin{name: String, ports: Vec<Port>},
    Emby{name: String, ports: Vec<Port>},
    NetvisorDaemon{name: String, ports: Vec<Port>, daemon_id: Uuid},
    NetvisorServer{name: String, ports: Vec<Port>},
    Unbound{name: String, ports: Vec<Port>},
    Bind9{name: String, ports: Vec<Port>},
    PowerDNS{name: String, ports: Vec<Port>},
    Portainer{name: String, ports: Vec<Port>},
    DockerSwarm{name: String, ports: Vec<Port>},
    Kubernetes{name: String, ports: Vec<Port>},
    Prometheus{name: String, ports: Vec<Port>},
    Duplicati{name: String, ports: Vec<Port>},
    Syncthing{name: String, ports: Vec<Port>},
    Restic{name: String, ports: Vec<Port>},
    WgDashboard{name: String, ports: Vec<Port>},

    // Services that can be inferred from a more complex pattern
    TrueNAS{name: String, ports: Vec<Port>},
    Grafana{name: String, ports: Vec<Port>},
    UptimeKuma{name: String, ports: Vec<Port>},
    PiHole{name: String, ports: Vec<Port>},
    AdguardHome{name: String, ports: Vec<Port>},
    PfSense{name: String, ports: Vec<Port>},
    OpnSense{name: String, ports: Vec<Port>},
    Fortigate{name: String, ports: Vec<Port>},
    UnifiAccessPoint{name: String, ports: Vec<Port>},
    TpLinkEap{name: String, ports: Vec<Port>},
    QNAP{name: String, ports: Vec<Port>},
    OpenMediaVault{name: String, ports: Vec<Port>},
    NextCloud{name: String, ports: Vec<Port>},
    PfBlockerNg{name: String, ports: Vec<Port>},
    CUPS{name: String, ports: Vec<Port>},
    Traefik{name: String, ports: Vec<Port>},
    NginxProxyManager{name: String, ports: Vec<Port>},
    Cloudflared{name: String, ports: Vec<Port>},
    HpPrinter{name: String, ports: Vec<Port>},
    EeroGateway{name: String, ports: Vec<Port>},
    EeroRepeater{name: String, ports: Vec<Port>},
    PhilipsHueBridge{name: String, ports: Vec<Port>},

    // Generic categories (non-vendor specific)
    GenericRouter{name: String, ports: Vec<Port>},
    GenericVpnGateway{name: String, ports: Vec<Port>},
    GenericNasDevice{name: String, ports: Vec<Port>},
    GenericFileServer{name: String, ports: Vec<Port>},
    GenericPrintServer{name: String, ports: Vec<Port>},
    GenericDnsServer{name: String, ports: Vec<Port>},
    GenericDhcpServer{name: String, ports: Vec<Port>},
    GenericHttpWebServer{name: String, ports: Vec<Port>},
    GenericHttpsWebServer{name: String, ports: Vec<Port>},
    GenericSwitch{name: String, ports: Vec<Port>}, 
    GenericAccessPoint{name: String, ports: Vec<Port>},
    GenericFirewall{name: String, ports: Vec<Port>}, 
    Workstation{name: String, ports: Vec<Port>},
}

impl Service {

    pub fn discovery_ports() -> Vec<Port> {
        let mut ports: Vec<Port> = ServiceDiscriminants::iter()
            .flat_map(|discriminant| discriminant.discovery_ports())
            .collect();
            
        ports.sort_by_key(|p| (p.number, p.protocol.to_string()));
        ports.dedup();
        ports
    }

    pub fn discovery_endpoints() -> Vec<Endpoint> {
        let mut endpoints: Vec<Endpoint> = ServiceDiscriminants::iter()
            .flat_map(|discriminant| discriminant.discovery_endpoints())
            .collect();

        endpoints.sort_by_key(|e| (e.protocol.to_string(), e.port.number, e.path.clone().unwrap_or("".to_string())) );
        endpoints.dedup();
        endpoints
    }

    pub fn from_discovery(
        discriminant: ServiceDiscriminants, 
        ip: IpAddr, 
        open_ports: &Vec<Port>, 
        endpoint_responses: &Vec<EndpointResponse>, 
        subnet: &Subnet, 
        mac_address: Option<MacAddress>) -> (Option<Self>, Option<Vec<Port>>) {

        if let Ok(result) = discriminant.discovery_pattern().matches(open_ports.clone(), endpoint_responses.clone(), ip, subnet, mac_address) {

            let ports: Vec<Port> = result.into_iter().filter_map(|p| p).collect();
            let ports_for_return = ports.clone();

            tracing::info!("✅ Service {:?} matched for {} with ports {:?}", discriminant, ip, open_ports);
            let name = discriminant.display_name().to_string();
            let service = match discriminant {
                        // Services with a single specific port they can be identified on
                        ServiceDiscriminants::HomeAssistant => Some(Self::HomeAssistant{name, ports}),
                        ServiceDiscriminants::Plex => Some(Self::Plex{name, ports}),
                        ServiceDiscriminants::Synology => Some(Self::Synology{name, ports}),
                        ServiceDiscriminants::UnifiController => Some(Self::UnifiController{name, ports}),
                        ServiceDiscriminants::Proxmox => Some(Self::Proxmox{name, ports}),
                        ServiceDiscriminants::Jellyfin => Some(Self::Jellyfin{name, ports}),
                        ServiceDiscriminants::Emby => Some(Self::Emby{name, ports}),
                        ServiceDiscriminants::NetvisorDaemon => Some(Self::NetvisorDaemon{name, ports, daemon_id: Uuid::nil()}),
                        ServiceDiscriminants::NetvisorServer => Some(Self::NetvisorServer{name, ports}),
                        ServiceDiscriminants::Unbound => Some(Self::Unbound{name, ports}),
                        ServiceDiscriminants::Bind9 => Some(Self::Bind9{name, ports}),
                        ServiceDiscriminants::PowerDNS => Some(Self::PowerDNS{name, ports}),
                        ServiceDiscriminants::DockerSwarm => Some(Self::DockerSwarm{name, ports}),
                        ServiceDiscriminants::Portainer => Some(Self::Portainer{name, ports}),
                        ServiceDiscriminants::Kubernetes => Some(Self::Kubernetes{name, ports}),
                        ServiceDiscriminants::Duplicati => Some(Self::Duplicati{name, ports}),
                        ServiceDiscriminants::Restic => Some(Self::Restic{name, ports}),
                        ServiceDiscriminants::Syncthing => Some(Self::Syncthing{name, ports}),
                        ServiceDiscriminants::WgDashboard => Some(Self::WgDashboard{name, ports}),

                        // Services that can be inferred from a combination of open ports and HTTP responses to endpoint requests
                        ServiceDiscriminants::Grafana => Some(Self::Grafana{name, ports}),
                        ServiceDiscriminants::Prometheus => Some(Self::Prometheus{name, ports}),
                        ServiceDiscriminants::UptimeKuma => Some(Self::UptimeKuma{name, ports}),
                        ServiceDiscriminants::PiHole => Some(Self::PiHole {name, ports}),
                        ServiceDiscriminants::AdguardHome => Some(Self::AdguardHome {name, ports}),
                        ServiceDiscriminants::TrueNAS => Some(Self::TrueNAS {name, ports}),
                        ServiceDiscriminants::PfSense => Some(Self::PfSense{name, ports}),
                        ServiceDiscriminants::OpnSense => Some(Self::OpnSense{name, ports}),
                        ServiceDiscriminants::Fortigate => Some(Self::Fortigate{name, ports}),
                        ServiceDiscriminants::UnifiAccessPoint => Some(Self::UnifiAccessPoint{name, ports}),
                        ServiceDiscriminants::TpLinkEap => Some(Self::TpLinkEap{name, ports}),
                        ServiceDiscriminants::QNAP => Some(Self::QNAP{name, ports}),
                        ServiceDiscriminants::OpenMediaVault => Some(Self::OpenMediaVault{name, ports}),
                        ServiceDiscriminants::NextCloud => Some(Self::NextCloud{name, ports}),
                        ServiceDiscriminants::PfBlockerNg => Some(Self::PfBlockerNg{name, ports}),
                        ServiceDiscriminants::CUPS => Some(Self::CUPS {name, ports}),
                        ServiceDiscriminants::Cloudflared => Some(Self::Cloudflared {name, ports}),
                        ServiceDiscriminants::NginxProxyManager => Some(Self::NginxProxyManager {name, ports}),
                        ServiceDiscriminants::Traefik => Some(Self::Traefik {name, ports}),
                        ServiceDiscriminants::HpPrinter => Some(Self::HpPrinter {name, ports}),
                        ServiceDiscriminants::EeroGateway => Some(Self::EeroGateway {name, ports}),
                        ServiceDiscriminants::EeroRepeater => Some(Self::EeroRepeater {name, ports}),
                        ServiceDiscriminants::PhilipsHueBridge => Some(Self::PhilipsHueBridge {name, ports}),
                        

                        // Generic categories for unknown devices with specific ports
                        ServiceDiscriminants::GenericRouter => Some(Self::GenericRouter {name, ports}),
                        ServiceDiscriminants::GenericVpnGateway => Some(Self::GenericVpnGateway {name, ports}),
                        ServiceDiscriminants::GenericHttpWebServer => Some(Self::GenericHttpWebServer{name, ports}),
                        ServiceDiscriminants::GenericHttpsWebServer => Some(Self::GenericHttpsWebServer{name, ports}),
                        ServiceDiscriminants::GenericDnsServer => Some(Self::GenericDnsServer {name, ports}),
                        ServiceDiscriminants::GenericDhcpServer => Some(Self::GenericDhcpServer {name, ports}),
                        ServiceDiscriminants::GenericPrintServer => Some(Self::GenericPrintServer {name, ports}),
                        ServiceDiscriminants::GenericNasDevice => Some(Self::GenericNasDevice {name, ports}),
                        ServiceDiscriminants::GenericFileServer => Some(Self::GenericFileServer {name, ports}),

                        // Services that don't have reliable fingerprinting methods available, and can only be manually added
                        ServiceDiscriminants::GenericSwitch => Some(Self::GenericSwitch {name, ports}),
                        ServiceDiscriminants::GenericAccessPoint => Some(Self::GenericAccessPoint {name, ports}),
                        ServiceDiscriminants::GenericFirewall => Some(Self::GenericFirewall {name, ports}),
                        ServiceDiscriminants::Workstation => Some(Self::Workstation {name, ports}),
                };

            return (service, Some(ports_for_return))
        } else {
            return (None, None)
        }
    }

}

impl ServiceDiscriminants {

    pub fn is_generic_service(&self) -> bool {
        self.to_string().contains("Generic")
    }

    pub fn can_be_manually_added(&self) -> bool {
        match self {
            ServiceDiscriminants::NetvisorDaemon | ServiceDiscriminants::NetvisorServer => false,
            _ => true
        }
    }

    pub fn can_be_dns_resolver(&self) -> bool {
        match self.service_category() {
            ServiceCategory::DNS | ServiceCategory::AdBlock => true,
            _ => false
        }
    }

    pub fn can_be_gateway(&self) -> bool {
        match self {
            ServiceDiscriminants::GenericVpnGateway 
            | ServiceDiscriminants::GenericRouter 
            | ServiceDiscriminants::PfSense 
            | ServiceDiscriminants::OpnSense 
            | ServiceDiscriminants::Fortigate
            | ServiceDiscriminants::GenericFirewall
            | ServiceDiscriminants::EeroGateway => true,
            _ => false
        }
    }

    pub fn discovery_ports(&self) -> Vec<Port> {
        self.discovery_pattern().ports()
    }

    fn discovery_endpoints(&self) -> Vec<Endpoint> {
        self.discovery_pattern().endpoints()
    }

    fn discovery_pattern(&self) -> Pattern {
        match &self {

            // Discovery - specific ports or endpoints
            ServiceDiscriminants::HomeAssistant => Pattern::AnyPort(vec!(Port::new_tcp(8123))),
            ServiceDiscriminants::Plex => Pattern::AnyPort(vec!(Port::new_tcp(32400))),
            ServiceDiscriminants::UnifiController => Pattern::AnyPort(vec!(Port::new_tcp(2049))),
            ServiceDiscriminants::Proxmox => Pattern::AnyPort(vec!(Port::new_tcp(8006))),
            ServiceDiscriminants::Jellyfin => Pattern::AnyPort(vec!(Port::new_tcp(8096))),
            ServiceDiscriminants::Emby => Pattern::AnyPort(vec!(Port::new_tcp(8920))),
            ServiceDiscriminants::NetvisorDaemon => Pattern::CustomPortWebService(60072, "", "Netvisor Daemon Running"),
            ServiceDiscriminants::NetvisorServer => Pattern::AnyPort(vec!(Port::new_tcp(60072))),
            ServiceDiscriminants::Unbound => Pattern::AllPort(vec!(Port::DNS_UDP, Port::new_tcp(8953))),
            ServiceDiscriminants::Bind9 => Pattern::AllPort(vec!(Port::DNS_UDP, Port::new_tcp(8053))),
            ServiceDiscriminants::PowerDNS => Pattern::AllPort(vec!(Port::DNS_UDP, Port::new_tcp(8081))),
            ServiceDiscriminants::DockerSwarm => Pattern::AllPort(vec![Port::new_tcp(2377), Port::new_tcp(7946)]),
            ServiceDiscriminants::Kubernetes => Pattern::AllOf(vec!(
                Pattern::AllPort(vec![Port::new_tcp(6443)]),
                Pattern::AnyPort(vec!(Port::new_tcp(10250), Port::new_tcp(10259), Port::new_tcp(10257), Port::new_tcp(10256)))
            )),
            ServiceDiscriminants::Portainer => Pattern::AnyPort(vec![Port::new_tcp(9000), Port::new_tcp(9443)]),
            ServiceDiscriminants::Prometheus => Pattern::Port(Port::new_tcp(9090)),
            ServiceDiscriminants::Restic => Pattern::Port(Port::new_tcp(8000)),
            ServiceDiscriminants::Duplicati => Pattern::Port(Port::new_tcp(8200)),
            ServiceDiscriminants::Syncthing => Pattern::Port(Port::new_tcp(8384)),
            ServiceDiscriminants::WgDashboard => Pattern::AllOf(vec!(
                Pattern::AnyPort(vec![Port::new_tcp(10086)]), 
                Pattern::SubnetIsNotType(SubnetType::VpnTunnel)
            )),

            // Discovery - generic ports but look for match on specific HTTP response, see discovery_request_expected_response
            ServiceDiscriminants::Synology => Pattern::WebService("/", "Synology"),
            ServiceDiscriminants::Grafana => Pattern::WebService("/", "Grafana"),
            ServiceDiscriminants::UptimeKuma => Pattern::WebService("/", "Uptime Kuma"),
            ServiceDiscriminants::Traefik => Pattern::WebService("/dashboard/", "Traefik"),
            ServiceDiscriminants::NginxProxyManager => Pattern::WebService("/", "Nginx Proxy Manager"),
            ServiceDiscriminants::Cloudflared => Pattern::WebService("/metrics", "cloudflared"),
            ServiceDiscriminants::PiHole => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::DNS_UDP)), Pattern::WebService("/admin", "Pi-hole"))),
            ServiceDiscriminants::TrueNAS => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::SAMBA)), Pattern::WebService("/", "TrueNAS"))),
            ServiceDiscriminants::PfSense => Pattern::WebService("/", "pfSense"),
            ServiceDiscriminants::OpnSense => Pattern::WebService("/", "OPNSense"),
            ServiceDiscriminants::Fortigate => Pattern::WebService("/", "FortiGate"),
            ServiceDiscriminants::UnifiAccessPoint => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::UBIQUITI), Pattern::WebService("/", "UniFi"))),
            ServiceDiscriminants::TpLinkEap => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::TPLINK), Pattern::WebService("/", "TP-LINK"))),
            ServiceDiscriminants::AdguardHome => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::DNS_UDP)), Pattern::WebService("/", "AdGuard Home"))),
            ServiceDiscriminants::QNAP => Pattern::WebService("/", "QNAP"),
            ServiceDiscriminants::OpenMediaVault => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::SAMBA)),Pattern::WebService("/", "OpenMediaVault"))),
            ServiceDiscriminants::NextCloud => Pattern::WebService("/", "Nextcloud"),
            ServiceDiscriminants::PfBlockerNg => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::DNS_UDP)), Pattern::WebService("/pfblockerng", "pfBlockerNG"))),
            ServiceDiscriminants::CUPS => Pattern::AllOf(vec!(Pattern::AnyPort(vec!(Port::IPP)), Pattern::WebService("/", "CUPS"))),
            ServiceDiscriminants::HpPrinter => Pattern::AllOf(vec!(
                Pattern::MacVendor(Vendor::HP), 
                Pattern::AnyPort(vec![Port::IPP, Port::LDP_UDP, Port::LDP_TCP])
            )),
            ServiceDiscriminants::EeroGateway => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::EERO), Pattern::IsGatewayIp)),
            ServiceDiscriminants::EeroRepeater => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::EERO), Pattern::NotGatewayIp)),
            ServiceDiscriminants::PhilipsHueBridge => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::PHILIPS), Pattern::WebService("/", "hue"))),
            
            // Generic services
            ServiceDiscriminants::GenericRouter => Pattern::AnyOf(vec![
                // Primary: Gateway IP with management interface
                Pattern::AllOf(vec![
                    Pattern::IsGatewayIp,
                    Pattern::AnyPort(vec![Port::SSH, Port::HTTP, Port::HTTPS])
                ]),
                // Secondary: Gateway with core services
                Pattern::AllOf(vec![
                    Pattern::IsGatewayIp, 
                    Pattern::AnyPort(vec![Port::DNS_UDP, Port::DHCP])
                ]),
                // Tertiary: SNMP + management (managed router, not gateway)
                Pattern::AllOf(vec![
                    Pattern::AllPort(vec![Port::SNMP, Port::SSH]),
                    Pattern::AnyPort(vec![Port::HTTP, Port::HTTPS])
                ])
            ]),
            ServiceDiscriminants::GenericSwitch => Pattern::AllOf(vec![
                Pattern::NotGatewayIp,
                Pattern::AnyOf(vec![
                    // Managed switch with SNMP
                    Pattern::AllPort(vec![Port::SNMP, Port::HTTP]),
                    // SSH-managed switch
                    Pattern::AllPort(vec![Port::SSH, Port::HTTP]),
                    // Basic web-managed switch
                    Pattern::AllPort(vec![Port::HTTP, Port::TELNET]) // HTTP + Telnet
                ])
            ]),
            ServiceDiscriminants::GenericVpnGateway => Pattern::AllOf(vec!(
                Pattern::IsVpnSubnetGateway,
                Pattern::AnyPort(vec!(Port::SSH, Port::HTTP, Port::HTTPS))
            )),
            ServiceDiscriminants::GenericDnsServer => Pattern::Port(Port::DNS_UDP),
            ServiceDiscriminants::GenericDhcpServer => Pattern::Port(Port::DHCP),
            ServiceDiscriminants::GenericPrintServer => Pattern::Port(Port::IPP),
            ServiceDiscriminants::GenericNasDevice => Pattern::Port(Port::NFS),
            ServiceDiscriminants::GenericFileServer => Pattern::Port(Port::FTP),
            ServiceDiscriminants::Workstation => Pattern::AllPort(vec!(Port::RDP, Port::SAMBA)),
            ServiceDiscriminants::GenericHttpWebServer => Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPALT)),
            ServiceDiscriminants::GenericHttpsWebServer => Pattern::AnyPort(vec!(Port::HTTPS, Port::HTTPSALT)),

            // No unique enough match pattern
            ServiceDiscriminants::GenericAccessPoint | ServiceDiscriminants::GenericFirewall => Pattern::None
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
            ServiceDiscriminants::EeroGateway => ServiceCategory::NetworkAccess,
            ServiceDiscriminants::EeroRepeater => ServiceCategory::NetworkAccess,

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
            ServiceDiscriminants::HpPrinter => ServiceCategory::Printer,

            // Dashboard
            ServiceDiscriminants::WgDashboard => ServiceCategory::Dashboard,

            // IoT
            ServiceDiscriminants::PhilipsHueBridge => ServiceCategory::IoT,

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

            // Web Services
            ServiceDiscriminants::GenericHttpWebServer => ServiceCategory::Web,
            ServiceDiscriminants::GenericHttpsWebServer => ServiceCategory::Web,
            ServiceDiscriminants::NextCloud => ServiceCategory::Web,
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
            ServiceDiscriminants::EeroGateway => "Eero Gateway",
            ServiceDiscriminants::EeroRepeater => "Eero Repeater",

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

            // Dashboard
            ServiceDiscriminants::WgDashboard => "Wireguard Dashboard",

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
            ServiceDiscriminants::HpPrinter => "HP Printer",
            
            // NetVisor
            ServiceDiscriminants::NetvisorDaemon => "NetVisor Daemon",
            ServiceDiscriminants::NetvisorServer => "NetVisor Server",

            // IoT
            ServiceDiscriminants::PhilipsHueBridge => "Philips Hue Bridge",
            
            // Device Types
            ServiceDiscriminants::Workstation => "Workstation",
            
            // Generic Services
            ServiceDiscriminants::GenericVpnGateway => "VPN Gateway",
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
            ServiceDiscriminants::EeroGateway => "Eero providing routing and gateway services",
            ServiceDiscriminants::EeroRepeater => "Eero providing mesh network services",

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

            // Dashboard
            ServiceDiscriminants::WgDashboard => "Dashboard for visualizing and managing wireguard clients and server",

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
            ServiceDiscriminants::HpPrinter => "HP Printer",
            
            // NetVisor
            ServiceDiscriminants::NetvisorDaemon => "NetVisor daemon for enhanced network diagnostics",
            ServiceDiscriminants::NetvisorServer => "NetVisor server for network management",

            // IoT
            ServiceDiscriminants::PhilipsHueBridge => "Philips Hue Bridge for lighting control",
            
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
        let default_endpoints = self.discovery_endpoints();
        let can_be_added = self.can_be_manually_added();    
        let can_be_dns_resolver = self.can_be_dns_resolver();
        let can_be_gateway = self.can_be_gateway();
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