use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::shared::types::metadata::TypeMetadataProvider;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum TestCriticality {
    Critical,        // Failure = NodeStatus::Failed
    Important,       // Failure = NodeStatus::Degraded  
    Informational,   // Failure = NodeStatus::Healthy (just logged)
}

impl TypeMetadataProvider for TestCriticality {
    fn id(&self) -> String { format!("{:?}", self) }
    
    fn display_name(&self) -> &str {
        match self {
            TestCriticality::Critical => "Critical",
            TestCriticality::Important => "Important", 
            TestCriticality::Informational => "Informational",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            TestCriticality::Critical => "Failure results in node status: Failed",
            TestCriticality::Important => "Failure results in node status: Degraded",
            TestCriticality::Informational => "Failure is logged but does not affect node status",
        }
    }
    
    fn category(&self) -> &str { "" }
    
    fn icon(&self) -> &str {
        match self {
            TestCriticality::Critical => "OctagonAlert",
            TestCriticality::Important => "TriangleAlert",
            TestCriticality::Informational => "CircleAlert",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            TestCriticality::Critical => "red",
            TestCriticality::Important => "yellow",
            TestCriticality::Informational => "blue",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({
            "priority": match self {
                TestCriticality::Critical => 1,
                TestCriticality::Important => 2,
                TestCriticality::Informational => 3,
            }
        })
    }
}