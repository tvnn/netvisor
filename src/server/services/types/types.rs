use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter, FromRepr};
use strum::{IntoDiscriminant};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::endpoints::{Endpoint};
use crate::server::services::types::patterns::{Pattern, Vendor};
use crate::server::services::types::ports::{Port};
use crate::server::shared::types::metadata::EntityMetadataProvider;
use crate::server::subnets::types::base::{SubnetType};
use crate::server::{shared::{types::metadata::TypeMetadataProvider}};

#[derive(Debug, Clone, PartialEq, Eq, FromRepr, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum ServiceType {
    // Services with a single specific port they can generally be identified on
    HomeAssistant,
    Plex,
    Synology,
    UnifiController,
    Proxmox,
    Jellyfin,
    Emby,
    NetvisorDaemon,
    NetvisorServer,
    Unbound,
    Bind9,
    PowerDNS,
    Portainer,
    DockerSwarm,
    Kubernetes,
    Prometheus,
    Duplicati,
    Syncthing,
    Restic,
    WgDashboard,

    // Services that can be inferred from a more complex pattern
    TrueNAS,
    Grafana,
    UptimeKuma,
    PiHole,
    AdguardHome,
    PfSense,
    OpnSense,
    Fortigate,
    UnifiAccessPoint,
    TpLinkEap,
    QNAP,
    OpenMediaVault,
    NextCloud,
    PfBlockerNg,
    CUPS,
    Traefik,
    NginxProxyManager,
    Cloudflared,
    HpPrinter,
    EeroGateway,
    EeroRepeater,
    PhilipsHueBridge,

    // Generic categories (non-vendor specific)
    GenericRouter,
    GenericVpnGateway,
    GenericNasDevice,
    GenericFileServer,
    GenericPrintServer,
    GenericDnsServer,
    GenericDhcpServer,
    GenericHttpWebServer,
    GenericHttpsWebServer,
    GenericSwitch, 
    GenericAccessPoint,
    GenericFirewall, 
    Workstation,
    Unknown
}

impl ServiceType {

    pub fn discovery_ports(&self) -> Vec<Port> {
        self.discovery_pattern().ports()
    }

    pub fn discovery_endpoints(&self) -> Vec<Endpoint> {
        self.discovery_pattern().endpoints()
    }

    pub fn is_generic_service(&self) -> bool {
        self.discriminant().to_string().contains("Generic")
    }

    pub fn can_be_manually_added(&self) -> bool {
        match self {
            ServiceType::NetvisorDaemon{..} | ServiceType::NetvisorServer => false,
            _ => true
        }
    }

    pub fn is_dns_resolver(&self) -> bool {
        match self.service_category() {
            ServiceCategory::DNS | ServiceCategory::AdBlock => true,
            _ => false
        }
    }

    pub fn is_reverse_proxy(&self) -> bool {
        match self {
            ServiceType::NginxProxyManager => true,
            _ => false
        }
    }

    pub fn is_end_device(&self) -> bool {
        match self.service_category() {
            ServiceCategory::Workstation | ServiceCategory::Mobile => true,
            _ => false
        }
    }

    pub fn is_gateway(&self) -> bool {
        match self {
            ServiceType::GenericVpnGateway 
            | ServiceType::GenericRouter 
            | ServiceType::PfSense 
            | ServiceType::OpnSense 
            | ServiceType::Fortigate
            | ServiceType::GenericFirewall
            | ServiceType::EeroGateway => true,
            _ => false
        }
    }

    pub fn service_category(&self) -> ServiceCategory {
        match self {
            // VPN Services
            ServiceType::GenericVpnGateway => ServiceCategory::VPN,

            // DNS Services
            ServiceType::Unbound => ServiceCategory::DNS,
            ServiceType::Bind9 => ServiceCategory::DNS,
            ServiceType::PowerDNS => ServiceCategory::DNS,
            ServiceType::GenericDnsServer => ServiceCategory::DNS,

            // Home Automation
            ServiceType::HomeAssistant => ServiceCategory::HomeAutomation,

            // Ad-block Services
            ServiceType::PiHole => ServiceCategory::AdBlock,
            ServiceType::AdguardHome => ServiceCategory::AdBlock,
            ServiceType::PfBlockerNg => ServiceCategory::AdBlock,

            // Media Services
            ServiceType::Plex => ServiceCategory::Media,
            ServiceType::Jellyfin => ServiceCategory::Media,
            ServiceType::Emby => ServiceCategory::Media,

            // Network Infrastructure
            ServiceType::GenericRouter => ServiceCategory::NetworkCore,
            ServiceType::UnifiController => ServiceCategory::NetworkAccess,
            ServiceType::UnifiAccessPoint => ServiceCategory::NetworkAccess,
            ServiceType::TpLinkEap => ServiceCategory::NetworkAccess,
            ServiceType::GenericDhcpServer => ServiceCategory::NetworkCore,
            ServiceType::GenericSwitch => ServiceCategory::NetworkCore,
            ServiceType::GenericAccessPoint => ServiceCategory::NetworkAccess,
            ServiceType::EeroGateway => ServiceCategory::NetworkAccess,
            ServiceType::EeroRepeater => ServiceCategory::NetworkAccess,

            // Storage & NAS
            ServiceType::Synology => ServiceCategory::Storage,
            ServiceType::TrueNAS => ServiceCategory::Storage,
            ServiceType::QNAP => ServiceCategory::Storage,
            ServiceType::OpenMediaVault => ServiceCategory::Storage,
            ServiceType::GenericNasDevice => ServiceCategory::Storage,
            ServiceType::GenericFileServer => ServiceCategory::Storage,
            
            // Backup
            ServiceType::Duplicati => ServiceCategory::Backup,
            ServiceType::Restic => ServiceCategory::Backup,
            ServiceType::Syncthing => ServiceCategory::Backup,

            // Virtualization
            ServiceType::Proxmox => ServiceCategory::Virtualization,
            ServiceType::DockerSwarm => ServiceCategory::Virtualization,
            ServiceType::Portainer => ServiceCategory::Virtualization,
            ServiceType::Kubernetes => ServiceCategory::Virtualization,

            // Monitoring
            ServiceType::Grafana => ServiceCategory::Monitoring,
            ServiceType::Prometheus => ServiceCategory::Monitoring,
            ServiceType::UptimeKuma => ServiceCategory::Monitoring,

            // Reverse proxy
            ServiceType::Traefik => ServiceCategory::ReverseProxy,
            ServiceType::NginxProxyManager => ServiceCategory::ReverseProxy,
            ServiceType::Cloudflared => ServiceCategory::ReverseProxy,

            // Print Services
            ServiceType::CUPS => ServiceCategory::Printer,
            ServiceType::GenericPrintServer => ServiceCategory::Printer,
            ServiceType::HpPrinter => ServiceCategory::Printer,

            // Dashboard
            ServiceType::WgDashboard => ServiceCategory::Dashboard,

            // IoT
            ServiceType::PhilipsHueBridge => ServiceCategory::IoT,

            // Security
            ServiceType::PfSense => ServiceCategory::NetworkSecurity,
            ServiceType::OpnSense => ServiceCategory::NetworkSecurity,
            ServiceType::Fortigate => ServiceCategory::NetworkSecurity,
            ServiceType::GenericFirewall => ServiceCategory::NetworkSecurity,

            // NetVisor
            ServiceType::NetvisorDaemon{..} => ServiceCategory::Netvisor,
            ServiceType::NetvisorServer => ServiceCategory::Netvisor,

            // Device Types
            ServiceType::Workstation => ServiceCategory::Workstation,

            // Web Services
            ServiceType::GenericHttpWebServer => ServiceCategory::Web,
            ServiceType::GenericHttpsWebServer => ServiceCategory::Web,
            ServiceType::NextCloud => ServiceCategory::Web,

            // Unknown
            ServiceType::Unknown => ServiceCategory::Unknown
        }
    }

    pub fn discovery_pattern(&self) -> Pattern {
        match &self {

            // Discovery - specific ports or endpoints
            ServiceType::HomeAssistant => Pattern::AnyPort(vec!(Port::new_tcp(8123))),
            ServiceType::Plex => Pattern::AnyPort(vec!(Port::new_tcp(32400))),
            ServiceType::UnifiController => Pattern::AnyPort(vec!(Port::new_tcp(2049))),
            ServiceType::Proxmox => Pattern::AnyPort(vec!(Port::new_tcp(8006))),
            ServiceType::Jellyfin => Pattern::AnyPort(vec!(Port::new_tcp(8096))),
            ServiceType::Emby => Pattern::AnyPort(vec!(Port::new_tcp(8920))),
            ServiceType::NetvisorDaemon{..} => Pattern::CustomPortWebService(60072, "", "Netvisor Daemon Running"),
            ServiceType::NetvisorServer => Pattern::AnyPort(vec!(Port::new_tcp(60072))),
            ServiceType::Unbound => Pattern::AllPort(vec!(Port::DNS_UDP, Port::new_tcp(8953))),
            ServiceType::Bind9 => Pattern::AllPort(vec!(Port::DNS_UDP, Port::new_tcp(8053))),
            ServiceType::PowerDNS => Pattern::AllPort(vec!(Port::DNS_UDP, Port::new_tcp(8081))),
            ServiceType::DockerSwarm => Pattern::AllPort(vec![Port::new_tcp(2377), Port::new_tcp(7946)]),
            ServiceType::Kubernetes => Pattern::AllOf(vec!(
                Pattern::AllPort(vec![Port::new_tcp(6443)]),
                Pattern::AnyPort(vec!(Port::new_tcp(10250), Port::new_tcp(10259), Port::new_tcp(10257), Port::new_tcp(10256)))
            )),
            ServiceType::Portainer => Pattern::AnyPort(vec![Port::new_tcp(9000), Port::new_tcp(9443)]),
            ServiceType::Prometheus => Pattern::Port(Port::new_tcp(9090)),
            ServiceType::Restic => Pattern::Port(Port::new_tcp(8000)),
            ServiceType::Duplicati => Pattern::Port(Port::new_tcp(8200)),
            ServiceType::Syncthing => Pattern::Port(Port::new_tcp(8384)),
            ServiceType::WgDashboard => Pattern::AllOf(vec!(
                Pattern::AnyPort(vec![Port::new_tcp(10086)]), 
                Pattern::SubnetIsNotType(SubnetType::VpnTunnel)
            )),

            // Discovery - generic ports but look for match on specific HTTP response, see discovery_request_expected_response
            ServiceType::Synology => Pattern::WebService("/", "Synology"),
            ServiceType::Grafana => Pattern::WebService("/", "Grafana"),
            ServiceType::UptimeKuma => Pattern::WebService("/", "Uptime Kuma"),
            ServiceType::Traefik => Pattern::WebService("/dashboard/", "Traefik"),
            ServiceType::NginxProxyManager => Pattern::WebService("/", "Nginx Proxy Manager"),
            ServiceType::Cloudflared => Pattern::WebService("/metrics", "cloudflared"),
            ServiceType::PiHole => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::DNS_UDP)), Pattern::WebService("/admin", "Pi-hole"))),
            ServiceType::TrueNAS => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::SAMBA)), Pattern::WebService("/", "TrueNAS"))),
            ServiceType::PfSense => Pattern::WebService("/", "pfSense"),
            ServiceType::OpnSense => Pattern::WebService("/", "OPNSense"),
            ServiceType::Fortigate => Pattern::WebService("/", "FortiGate"),
            ServiceType::UnifiAccessPoint => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::UBIQUITI), Pattern::WebService("/", "UniFi"))),
            ServiceType::TpLinkEap => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::TPLINK), Pattern::WebService("/", "TP-LINK"))),
            ServiceType::AdguardHome => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::DNS_UDP)), Pattern::WebService("/", "AdGuard Home"))),
            ServiceType::QNAP => Pattern::WebService("/", "QNAP"),
            ServiceType::OpenMediaVault => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::SAMBA)),Pattern::WebService("/", "OpenMediaVault"))),
            ServiceType::NextCloud => Pattern::WebService("/", "Nextcloud"),
            ServiceType::PfBlockerNg => Pattern::AllOf(vec!(Pattern::AllPort(vec!(Port::DNS_UDP)), Pattern::WebService("/pfblockerng", "pfBlockerNG"))),
            ServiceType::CUPS => Pattern::AllOf(vec!(Pattern::AnyPort(vec!(Port::IPP)), Pattern::WebService("/", "CUPS"))),
            ServiceType::HpPrinter => Pattern::AllOf(vec!(
                Pattern::MacVendor(Vendor::HP), 
                Pattern::AnyPort(vec![Port::IPP, Port::LDP_UDP, Port::LDP_TCP])
            )),
            ServiceType::EeroGateway => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::EERO), Pattern::IsGatewayIp)),
            ServiceType::EeroRepeater => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::EERO), Pattern::NotGatewayIp)),
            ServiceType::PhilipsHueBridge => Pattern::AllOf(vec!(Pattern::MacVendor(Vendor::PHILIPS), Pattern::WebService("/", "hue"))),
            
            // Generic services
            ServiceType::GenericRouter => Pattern::AnyOf(vec![
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
            ServiceType::GenericSwitch => Pattern::AllOf(vec![
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
            ServiceType::GenericVpnGateway => Pattern::AllOf(vec!(
                Pattern::IsVpnSubnetGateway,
                Pattern::AnyPort(vec!(Port::SSH, Port::HTTP, Port::HTTPS))
            )),
            ServiceType::GenericDnsServer => Pattern::Port(Port::DNS_UDP),
            ServiceType::GenericDhcpServer => Pattern::Port(Port::DHCP),
            ServiceType::GenericPrintServer => Pattern::Port(Port::IPP),
            ServiceType::GenericNasDevice => Pattern::Port(Port::NFS),
            ServiceType::GenericFileServer => Pattern::Port(Port::FTP),
            ServiceType::Workstation => Pattern::AllPort(vec!(Port::RDP, Port::SAMBA)),
            ServiceType::GenericHttpWebServer => Pattern::AnyPort(vec!(Port::HTTP, Port::HTTPALT)),
            ServiceType::GenericHttpsWebServer => Pattern::AnyPort(vec!(Port::HTTPS, Port::HTTPSALT)),

            // No unique enough match pattern
            ServiceType::GenericAccessPoint | ServiceType::GenericFirewall | ServiceType::Unknown => Pattern::None
        }
    }
}

impl EntityMetadataProvider for ServiceType {
    fn color(&self) -> &'static str {
       self.service_category().color()
    }
    fn icon(&self) -> &'static str {
        self.service_category().icon()
    }
}

impl TypeMetadataProvider for ServiceType {
    fn display_name(&self) -> &'static str {
        match self {            
            // DNS Services
            ServiceType::Unbound => "Unbound DNS",
            ServiceType::Bind9 => "BIND9 DNS",
            ServiceType::PowerDNS => "PowerDNS",

            // Home Automation
            ServiceType::HomeAssistant => "Home Assistant",

            // Ad-block Services
            ServiceType::PiHole => "Pi-hole",
            ServiceType::AdguardHome => "AdGuard Home",
            ServiceType::PfBlockerNg => "pfBlockerNG",
            
            // Media Services
            ServiceType::Plex => "Plex Media Server",
            ServiceType::Jellyfin => "Jellyfin",
            ServiceType::Emby => "Emby",
            
            // Network Infrastructure
            ServiceType::GenericRouter => "Generic Router",
            ServiceType::GenericSwitch => "Generic Network Switch",
            ServiceType::GenericAccessPoint => "Generic Access Point",
            ServiceType::GenericFirewall => "Generic Firewall",
            ServiceType::PfSense => "pfSense",
            ServiceType::OpnSense => "OPNsense",
            ServiceType::Fortigate => "FortiGate",
            ServiceType::UnifiController => "UniFi Controller",
            ServiceType::UnifiAccessPoint => "UniFi Access Point",
            ServiceType::TpLinkEap => "TP-Link EAP",
            ServiceType::EeroGateway => "Eero Gateway",
            ServiceType::EeroRepeater => "Eero Repeater",

            // Reverse proxy
            ServiceType::Traefik => "Traefik",
            ServiceType::NginxProxyManager => "Nginx Proxy Manager",
            ServiceType::Cloudflared => "Cloudflared",
            
            // Storage & NAS
            ServiceType::TrueNAS => "TrueNAS",
            ServiceType::Synology => "Synology DSM",
            ServiceType::QNAP => "QNAP NAS",
            ServiceType::OpenMediaVault => "OpenMediaVault",
            ServiceType::NextCloud => "Nextcloud",

            // Backup
            ServiceType::Duplicati => "Duplicati",
            ServiceType::Restic => "Restic",
            ServiceType::Syncthing => "Syncthing",

            // Dashboard
            ServiceType::WgDashboard => "Wireguard Dashboard",

            // Virtualization
            ServiceType::Proxmox => "Proxmox VE",
            ServiceType::DockerSwarm => "Docker Swarm",
            ServiceType::Portainer => "Portainer",
            ServiceType::Kubernetes => "Kubernetes",

            // Monitoring
            ServiceType::Grafana => "Grafana",
            ServiceType::Prometheus => "Prometheus",
            ServiceType::UptimeKuma => "UptimeKuma",
            
            // Print Services
            ServiceType::CUPS => "CUPS Print Server",
            ServiceType::HpPrinter => "HP Printer",
            
            // NetVisor
            ServiceType::NetvisorDaemon{..} => "NetVisor Daemon",
            ServiceType::NetvisorServer => "NetVisor Server",

            // IoT
            ServiceType::PhilipsHueBridge => "Philips Hue Bridge",
            
            // Device Types
            ServiceType::Workstation => "Workstation",
            
            // Generic Services
            ServiceType::GenericVpnGateway => "Generic VPN Gateway",
            ServiceType::GenericHttpWebServer => "Generic HTTP Web Server",
            ServiceType::GenericHttpsWebServer => "Generic HTTPS Web Server",
            ServiceType::GenericDnsServer => "Generic DNS Server",
            ServiceType::GenericDhcpServer => "Generic DHCP Server",
            ServiceType::GenericPrintServer => "Generic Print Server",
            ServiceType::GenericNasDevice => "Generic NAS Device",
            ServiceType::GenericFileServer => "Generic File Server",
            ServiceType::Unknown => "Unknown"
        }
    }
    
    fn description(&self) -> &'static str {
        match self {            
            // VPN Services
            ServiceType::GenericVpnGateway => "Generic VPN service",
            
            // DNS Services
            ServiceType::Unbound => "Validating DNS resolver with control interface",
            ServiceType::Bind9 => "Berkeley Internet Name Domain DNS server",
            ServiceType::PowerDNS => "Authoritative DNS server with API",
            ServiceType::PiHole => "Network-wide ad blocking DNS service",
            ServiceType::AdguardHome => "Network-wide ad and tracker blocking",
            ServiceType::PfBlockerNg => "pfSense package for DNS/IP blocking",
            ServiceType::GenericDnsServer => "Generic DNS resolution service",
            
            // Home Automation
            ServiceType::HomeAssistant => "Open-source home automation platform",

            // Media Services
            ServiceType::Plex => "Media server for streaming personal content",
            ServiceType::Jellyfin => "Free media server for personal streaming",
            ServiceType::Emby => "Personal media server with streaming capabilities",
            
            // Network Infrastructure
            ServiceType::GenericRouter => "Network router providing routing and gateway services",
            ServiceType::GenericSwitch => "Network switch for local area networking",
            ServiceType::GenericAccessPoint => "Wireless access point for WiFi connectivity",
            ServiceType::GenericFirewall => "Network security appliance",
            ServiceType::PfSense => "Open-source firewall and router platform",
            ServiceType::OpnSense => "Open-source firewall and routing platform",
            ServiceType::Fortigate => "Fortinet security appliance",
            ServiceType::UnifiController => "Ubiquiti UniFi network controller",
            ServiceType::UnifiAccessPoint => "Ubiquiti UniFi wireless access point",
            ServiceType::TpLinkEap => "TP-Link EAP wireless access point",
            ServiceType::EeroGateway => "Eero providing routing and gateway services",
            ServiceType::EeroRepeater => "Eero providing mesh network services",

            // Reverse proxy
            ServiceType::Traefik => "Modern reverse proxy and load balancer",
            ServiceType::NginxProxyManager => "Web-based Nginx proxy management interface",
            ServiceType::Cloudflared => "Cloudflare tunnel daemon",
            
            // Storage & NAS
            ServiceType::TrueNAS => "Open-source network attached storage system",
            ServiceType::Synology => "Synology DiskStation Manager NAS system",
            ServiceType::QNAP => "QNAP network attached storage system",
            ServiceType::OpenMediaVault => "Debian-based NAS solution",
            ServiceType::NextCloud => "Self-hosted cloud storage and collaboration platform",

            // Dashboard
            ServiceType::WgDashboard => "Dashboard for visualizing and managing wireguard clients and server",

            // Backup
            ServiceType::Duplicati => "Cross-platform backup client with encryption",
            ServiceType::Restic => "Fast and secure backup program",
            ServiceType::Syncthing => "Continuous file synchronization service",
            
            // Virtualization
            ServiceType::Proxmox => "Open-source virtualization management platform",
            ServiceType::DockerSwarm => "Docker native clustering and orchestration",
            ServiceType::Portainer => "Container management web interface",
            ServiceType::Kubernetes => "Container orchestration platform",

            // Monitoring
            ServiceType::Grafana => "Analytics and monitoring visualization platform",
            ServiceType::Prometheus => "Time-series monitoring and alerting system",
            ServiceType::UptimeKuma => "Self-hosted uptime monitoring tool",
            
            // Print Services
            ServiceType::CUPS => "Common Unix Printing System",
            ServiceType::GenericPrintServer => "Generic printing service",
            ServiceType::HpPrinter => "HP Printer",
            
            // NetVisor
            ServiceType::NetvisorDaemon{..} => "NetVisor daemon",
            ServiceType::NetvisorServer => "NetVisor server for network management",

            // IoT
            ServiceType::PhilipsHueBridge => "Philips Hue Bridge for lighting control",
            
            // Device Types
            ServiceType::Workstation => "Desktop computer for productivity work",
            
            // Generic Services
            ServiceType::GenericHttpWebServer => "Generic HTTP web server",
            ServiceType::GenericHttpsWebServer => "Generic HTTPS web server",
            ServiceType::GenericDhcpServer => "Generic DHCP service",
            ServiceType::GenericNasDevice => "Generic network storage device",
            ServiceType::GenericFileServer => "Generic file sharing service",
            ServiceType::Unknown => "Unknown service"
        }
    }
    
    fn category(&self) -> &'static str {
        self.service_category().category_str()
    }
    
    fn metadata(&self) -> serde_json::Value {
        let default_ports = self.discovery_ports();
        let default_endpoints = self.discovery_endpoints();
        let can_be_added = self.can_be_manually_added();    
        let is_dns_resolver = self.is_dns_resolver();
        let is_reverse_proxy = self.is_reverse_proxy();
        let is_gateway = self.is_gateway();
        let is_generic = self.is_generic_service();
        serde_json::json!({
            "default_ports": default_ports, 
            "default_endpoints": default_endpoints, 
            "can_be_added": can_be_added, 
            "is_dns_resolver": is_dns_resolver,
            "is_gateway": is_gateway,
            "is_reverse_proxy": is_reverse_proxy,
            "is_generic": is_generic
        })
    }
}