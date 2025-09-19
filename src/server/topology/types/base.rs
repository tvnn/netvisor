use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;
use crate::server::{hosts::types::base::Host, interfaces::types::base::Interface, shared::{constants::{DNS_COLOR, GATEWAY_COLOR, HOST_COLOR, HOST_GROUP_COLOR, MEDIA_COLOR, REVERSE_PROXY_COLOR}, types::metadata::TypeMetadataProvider}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub label: Option<String>,
    pub node_type: NodeType,
    pub parent_id: Option<Uuid>,
    pub position: XY,
    pub size: XY
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
    pub host: Host,
    pub interface: Option<Interface>,
    pub node_type: NodeType,
    pub id: Uuid,
    pub label: Option<String>,
    pub size: SubnetChildNodeSize,
    pub services: Vec<Uuid>
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
            0..=2 => SubnetChildNodeSize::Small,
            3..=5 => SubnetChildNodeSize::Medium,
            _ => SubnetChildNodeSize::Large
        }
    }

    pub fn size(&self) -> XY {
        match self {
            SubnetChildNodeSize::Small => XY { x: 75, y: 75 },
            SubnetChildNodeSize::Medium => XY { x: 100, y: 100 },
            SubnetChildNodeSize::Large => XY { x: 150, y: 150 },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum NodeType {
    SubnetNode,
    HostNode,
    InterfaceNode
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum EdgeType {
    HostInterface, // A host's non-primary interface participates in a different subnet
    HostGroup,     // User-defined logical connection
}

impl TypeMetadataProvider for EdgeType {
    fn id(&self) -> String {
        self.discriminant().to_string()
    }
    fn display_name(&self) -> &str {
        match self {
            EdgeType::HostGroup => "Host Group",
            EdgeType::HostInterface => "Host Interface"
        }
    }
    fn category(&self) -> &str {
        ""
    }
    fn color(&self) -> &str {
        match self {
            EdgeType::HostGroup => HOST_GROUP_COLOR,
            EdgeType::HostInterface => HOST_COLOR
        }
    }
    fn description(&self) -> &str {
        match self {
            EdgeType::HostGroup => "",
            EdgeType::HostInterface => ""
        }
    }
    fn icon(&self) -> &str {
        ""
    }
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}