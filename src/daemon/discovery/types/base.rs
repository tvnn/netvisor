use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryPhase {
    ScanningHosts,
    PortScanning,
    GatheringInfo,
    Complete,
}

impl std::fmt::Display for DiscoveryPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryPhase::ScanningHosts => write!(f, "Scanning for active hosts"),
            DiscoveryPhase::PortScanning => write!(f, "Port scanning discovered hosts"),
            DiscoveryPhase::GatheringInfo => write!(f, "Gathering device information"),
            DiscoveryPhase::Complete => write!(f, "Discovery complete"),
        }
    }
}
