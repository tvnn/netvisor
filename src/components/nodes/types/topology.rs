use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPosition {
    pub x: f32,
    pub y: f32,
    pub z: Option<f32>, // For 3D layouts if needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetGroup {
    pub id: String,
    pub name: String,
    pub cidr: String,           // "192.168.1.0/24"
    pub node_ids: Vec<String>,  // Nodes in this subnet
    pub vlan_id: Option<u16>,   // VLAN identifier if applicable
}