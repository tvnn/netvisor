use crate::server::{
    subnets::types::base::SubnetType,
    topology::types::{
        base::{Ixy, Uxy},
        edges::EdgeHandle,
    },
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter, IntoStaticStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    #[serde(flatten)]
    pub node_type: NodeType,
    pub id: Uuid,
    pub position: Ixy,
    pub size: Uxy,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants, IntoStaticStr,
)]
#[serde(tag = "node_type")]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum NodeType {
    SubnetNode {
        infra_width: usize,
        subnet_type: SubnetType,
    },
    HostNode {
        subnet_id: Uuid,
        host_id: Uuid,
        interface_id: Option<Uuid>,
        is_infra: bool,
    },
}

#[derive(Debug, Clone)]
pub struct SubnetChild {
    pub id: Uuid,
    pub host_id: Uuid,
    pub interface_id: Option<Uuid>,
    pub size: SubnetChildNodeSize,
    pub primary_handle: Option<EdgeHandle>,
    pub anchor_count: usize,
    pub should_relocate_handles: bool,
}

#[derive(Debug, Clone)]
pub enum SubnetChildNodeSize {
    Small,
    Medium,
    Large,
}

impl SubnetChildNodeSize {
    pub fn from_service_count(count: usize) -> Self {
        match count {
            0..=1 => SubnetChildNodeSize::Small,
            2..=3 => SubnetChildNodeSize::Medium,
            _ => SubnetChildNodeSize::Large,
        }
    }

    pub fn size(&self) -> Uxy {
        match self {
            SubnetChildNodeSize::Small => Uxy { x: 175, y: 100 },
            SubnetChildNodeSize::Medium => Uxy { x: 175, y: 125 },
            SubnetChildNodeSize::Large => Uxy { x: 175, y: 150 },
        }
    }
}

impl SubnetType {
    pub fn default_layer(&self) -> usize {
        match self {
            // Layer 0: External
            SubnetType::Internet => 0,
            SubnetType::Remote => 0,

            // Layer 1: Gateway/DMZ
            SubnetType::Gateway => 1,
            SubnetType::Dmz => 1, // Same layer as Gateway
            SubnetType::VpnTunnel => 1,

            // Layer 2: Internal
            SubnetType::Lan => 2,
            SubnetType::WiFi => 2,
            SubnetType::Guest => 2,
            SubnetType::IoT => 2,

            // Layer 3: Infrastructure
            SubnetType::DockerBridge => 3,
            SubnetType::Management => 3,
            SubnetType::Storage => 3,

            // Special
            SubnetType::Unknown => 999,
            SubnetType::None => 999,
        }
    }

    pub fn layer_priority(&self) -> usize {
        match self {
            // Layer 0
            SubnetType::Internet => 0,
            SubnetType::Remote => 1,

            // Layer 1 - Gateway is central, DMZ to the side
            SubnetType::Gateway => 0,   // Center/left
            SubnetType::Dmz => 1,       // Right of gateway
            SubnetType::VpnTunnel => 2, // Further right

            // Layer 2
            SubnetType::Lan => 0,
            SubnetType::WiFi => 1,
            SubnetType::IoT => 2,
            SubnetType::Guest => 3,

            // Layer 3
            SubnetType::Storage => 0,
            SubnetType::Management => 1,
            SubnetType::DockerBridge => 2,

            // Special
            SubnetType::Unknown => 999,
            SubnetType::None => 999,
        }
    }
}
