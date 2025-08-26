use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct GraphPosition {
    pub x: i32,
    pub y: i32,
    pub z: Option<i32>, // For 3D layouts if needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetGroup {
    pub id: String,
    pub name: String,
    pub cidr: String,           // "192.168.1.0/24"
    pub node_ids: Vec<String>,  // Nodes in this subnet
    pub vlan_id: Option<u16>,   // VLAN identifier if applicable
}