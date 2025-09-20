use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumIter};

use crate::server::shared::{constants::Entity, types::metadata::EntityMetadataProvider};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
pub enum ServiceCategory {
    // Infrastructure (always-on, core network services)
    NetworkCore,        // Routers, switches, core infrastructure
    NetworkAccess,      // WiFi APs, switches for end devices
    NetworkSecurity,    // Firewalls, security appliances
    
    // Server Services
    Storage,           // NAS, file servers
    Backup,
    Media,             // Plex, Jellyfin
    HomeAutomation,    // Home Assistant
    Virtualization,    // Proxmox, ESXi
    
    // Network Services
    DNS,               // All DNS services
    VPN,               // All VPN services
    Monitoring,        // SNMP, monitoring tools
    AdBlock,
    ReverseProxy,
    
    // End Devices
    Workstation,       // Desktops, laptops
    Mobile,            // Phones, tablets
    IoT,               // Smart devices, sensors
    Printer,           // All printing devices
    
    // Applications
    Web,               // Web servers
    Database,          // DB servers
    Development,       // Dev tools, CI/CD
    Dashboard,
    
    // Special
    Unknown,
    Netvisor,
}

impl ServiceCategory {
    pub fn category_str(&self) -> &'static str{
        match self {
            // Infrastructure (always-on, core network services)
            ServiceCategory::NetworkCore => "Network Core",
            ServiceCategory::NetworkAccess => "Network Access",
            ServiceCategory::NetworkSecurity => "Network Infrastructure",

            // Server Services
            ServiceCategory::Storage => "Storage",
            ServiceCategory::Media => "Media",
            ServiceCategory::HomeAutomation => "Home Automation",
            ServiceCategory::Virtualization => "Virtualization",
            ServiceCategory::Backup => "Backup",
            
            // Network Services
            ServiceCategory::DNS => "DNS",
            ServiceCategory::VPN => "VPN",
            ServiceCategory::Monitoring => "Monitoring",
            ServiceCategory::AdBlock => "Ad Blocker",
            ServiceCategory::ReverseProxy => "Reverse Proxy",

            // End devices
            ServiceCategory::Workstation => "Workstation",
            ServiceCategory::Mobile => "Mobile",
            ServiceCategory::IoT => "IoT",
            ServiceCategory::Printer => "Printer",

            // Application
            ServiceCategory::Web => "Web",
            ServiceCategory::Database => "Database",
            ServiceCategory::Development => "Development",
            ServiceCategory::Dashboard => "Dashboard",
            
            // Unknown
            ServiceCategory::Netvisor => "NetVisor",
            ServiceCategory::Unknown => "Unknown",
        }
    }
    pub fn icon(&self) -> &'static str {
        match self {
            // Infrastructure (always-on, core network services)
            ServiceCategory::NetworkCore => "Network",
            ServiceCategory::NetworkAccess => "Router",
            ServiceCategory::NetworkSecurity => "BrickWallShield",

            // Server Services
            ServiceCategory::Storage => "HardDrive",
            ServiceCategory::Media => "PlayCircle",
            ServiceCategory::HomeAutomation => "Home",
            ServiceCategory::Virtualization => "MonitorCog",
            
            // Network Services
            ServiceCategory::DNS => Entity::Dns.icon(),
            ServiceCategory::VPN => Entity::Vpn.icon(),
            ServiceCategory::Monitoring => "Activity",
            ServiceCategory::AdBlock => "ShieldCheck",
            ServiceCategory::Backup => "DatabaseBackup",
            ServiceCategory::ReverseProxy => Entity::ReverseProxy.icon(),

            // End devices
            ServiceCategory::Workstation => "Monitor",
            ServiceCategory::Mobile => "Smartphone",
            ServiceCategory::IoT => "Cpu",
            ServiceCategory::Printer => "Printer",

            // Application
            ServiceCategory::Web => "Globe",
            ServiceCategory::Database => "Database",
            ServiceCategory::Development => "Code",
            ServiceCategory::Dashboard => "LayoutDashboard",
            
            // Unknown
            ServiceCategory::Netvisor => "Zap",
            ServiceCategory::Unknown => "CircleQuestionMark",
        }
    }
    pub fn color(&self) -> &'static str {
        match self {
            // Infrastructure (always-on, core network services)
            ServiceCategory::NetworkCore => "yellow",
            ServiceCategory::NetworkAccess => "green",
            ServiceCategory::NetworkSecurity => "red",

            // Server Services
            ServiceCategory::Storage => "green",
            ServiceCategory::Media => Entity::Media.color(),
            ServiceCategory::HomeAutomation => "blue",
            ServiceCategory::Virtualization => "orange",
            ServiceCategory::Backup => "gray",
            
            // Network Services
            ServiceCategory::DNS => Entity::Dns.color(),
            ServiceCategory::VPN => Entity::Vpn.color(),
            ServiceCategory::Monitoring => "orange",
            ServiceCategory::AdBlock => Entity::Dns.color(),
            ServiceCategory::ReverseProxy => Entity::ReverseProxy.color(),

            // End devices
            ServiceCategory::Workstation => "green",
            ServiceCategory::Mobile => "blue",
            ServiceCategory::IoT => "yellow",
            ServiceCategory::Printer => "gray",

            // Application
            ServiceCategory::Web => "blue",
            ServiceCategory::Database => "gray",
            ServiceCategory::Development => "red",
            ServiceCategory::Dashboard => "purple",
            
            // Unknown
            ServiceCategory::Netvisor => "purple",
            ServiceCategory::Unknown => "gray",
        }
    }
}
