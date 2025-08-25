use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::shared::types::metadata::TypeMetadataProvider;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum NodeStatus {
    Healthy,
    Degraded,
    Failed,
    Unknown,
}

impl TypeMetadataProvider for NodeStatus {
    fn id(&self) -> String { format!("{:?}", self) }
    
    fn display_name(&self) -> &str {
        match self {
            NodeStatus::Healthy => "Healthy",
            NodeStatus::Degraded => "Degraded",
            NodeStatus::Failed => "Failed",
            NodeStatus::Unknown => "Unknown",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            NodeStatus::Healthy => "All tests passing",
            NodeStatus::Degraded => "Some non-critical tests failing",
            NodeStatus::Failed => "Critical tests failing",
            NodeStatus::Unknown => "No recent test data",
        }
    }
    
    fn category(&self) -> &str { "Node Health" }
    
    fn icon(&self) -> &str {
        match self {
            NodeStatus::Healthy => "CheckCircle",
            NodeStatus::Degraded => "AlertTriangle",
            NodeStatus::Failed => "XCircle",
            NodeStatus::Unknown => "HelpCircle",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            NodeStatus::Healthy => "green",
            NodeStatus::Degraded => "yellow",
            NodeStatus::Failed => "red",
            NodeStatus::Unknown => "gray",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}