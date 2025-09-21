use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;
use crate::server::{shared::{constants::Entity, types::metadata::{EntityMetadataProvider, TypeMetadataProvider}}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub id: Uuid, // Principal ID used primarily to key off of for backend operations, will be the same as one of the below
    pub parent_id: Option<Uuid>,
    pub interface_id: Option<Uuid>,
    pub host_id: Option<Uuid>,
    pub position: XY,
    pub size: XY,
    pub infra_width: Option<usize>
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum NodeType {
    SubnetNode,
    HostNode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XY {
    pub x: usize,
    pub y: usize
}

impl Default for XY {
    fn default() -> Self {
        Self {x: 0, y:0}
    }
}

#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub size: XY,
    pub grid_position: XY
}

#[derive(Debug, Clone)]
pub struct SubnetChild {
    pub id: Uuid,
    pub host_id: Uuid,
    pub interface_id: Option<Uuid>,
    pub size: SubnetChildNodeSize,
}

#[derive(Debug, Clone)]
pub enum SubnetChildNodeSize {
    Small,
    Medium,
    Large
}

impl SubnetChildNodeSize {
    pub fn from_service_count(count: usize) -> Self {
        match count {
            0..=1 => SubnetChildNodeSize::Small,
            2..=3 => SubnetChildNodeSize::Medium,
            _ => SubnetChildNodeSize::Large
        }
    }

    pub fn size(&self) -> XY {
        match self {
            SubnetChildNodeSize::Small => XY { x: 175, y: 100 },
            SubnetChildNodeSize::Medium => XY { x: 175, y: 125 },
            SubnetChildNodeSize::Large => XY { x: 175, y: 150 },
        }
    }
}

pub struct SubnetLayout {
    pub size: XY,
    pub infra_width: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType,
    pub source_handle: EdgeHandle,
    pub target_handle: EdgeHandle
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EdgeHandle {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum EdgeType {
    Interface, // Connecting hosts with interfaces in multiple subnets
    Group,     // User-defined logical connection
}

impl EntityMetadataProvider for EdgeType {
    fn color(&self) -> &'static str {
        match self {
            EdgeType::Group => Entity::Group.color(),
            EdgeType::Interface => Entity::Host.color()
        }
    }
    
    fn icon(&self) -> &'static str {
        match self {
            EdgeType::Group => Entity::Group.icon(),
            EdgeType::Interface => Entity::Host.icon()
        }
    }
}

impl TypeMetadataProvider for EdgeType {
    fn display_name(&self) -> &'static str {
        match self {
            EdgeType::Group => "Host Group",
            EdgeType::Interface => "Host Interface"
        }
    }
}