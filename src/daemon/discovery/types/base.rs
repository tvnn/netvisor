use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryPhase {
    Created, // Initial state, set by server; all subsequent states until Finished are set by Daemon
    Started,
    ScanningHosts,
    PortScanning,
    GatheringInfo,
    Complete,
    Failed,
    Cancelled,
    Finished // Ultimate terminal state, set by server
}

impl std::fmt::Display for DiscoveryPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryPhase::Created => write!(f, "Session created in server"),
            DiscoveryPhase::Started => write!(f, "Session started in daemon"),
            DiscoveryPhase::ScanningHosts => write!(f, "Scanning for active hosts"),
            DiscoveryPhase::PortScanning => write!(f, "Port scanning discovered hosts"),
            DiscoveryPhase::GatheringInfo => write!(f, "Gathering device information"),
            DiscoveryPhase::Complete => write!(f, "Discovery complete"),
            DiscoveryPhase::Cancelled => write!(f, "Discovery cancelled"),
            DiscoveryPhase::Failed => write!(f, "Discovery failed"),
            DiscoveryPhase::Finished => write!(f, "Session finished in server"),
        }
    }
}
