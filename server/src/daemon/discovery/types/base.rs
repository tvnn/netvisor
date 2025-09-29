use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryPhase {
    Initiated, // Initial state, set by server; all subsequent states until Finished are set by Daemon
    Started,
    Scanning,
    Complete,
    Failed,
    Cancelled,
    Finished, // Ultimate terminal state, set by server
}

impl std::fmt::Display for DiscoveryPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryPhase::Initiated => write!(f, "Session created in server"),
            DiscoveryPhase::Started => write!(f, "Session started in daemon"),
            DiscoveryPhase::Scanning => write!(f, "Scanning for active hosts"),
            DiscoveryPhase::Complete => write!(f, "Discovery complete"),
            DiscoveryPhase::Cancelled => write!(f, "Discovery cancelled"),
            DiscoveryPhase::Failed => write!(f, "Discovery failed"),
            DiscoveryPhase::Finished => write!(f, "Session finished in server"),
        }
    }
}
