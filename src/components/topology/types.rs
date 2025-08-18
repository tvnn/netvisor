use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::components::nodes::types::topology::{SubnetGroup};
use crate::components::node_groups::types::NodeGroup;

// Network topology generated from node groups and subnet membership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    pub id: String,
    pub name: String,
    pub node_groups: Vec<NodeGroup>,     // Logical network paths
    pub subnets: Vec<SubnetGroup>,       // Network clustering
    pub last_updated: DateTime<Utc>,
}