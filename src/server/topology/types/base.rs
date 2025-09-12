use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use uuid::Uuid;
use crate::server::shared::{constants::{DNS_COLOR, GATEWAY_COLOR, HOST_GROUP_COLOR, MEDIA_COLOR, REVERSE_PROXY_COLOR}, types::metadata::TypeMetadataProvider};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub color: String,
    pub icon: String,
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
pub struct SubnetLayout {
    pub position: XY,
    pub size: XY,
    pub grid_dimensions: XY
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeType {
    SubnetNode,
    HostNode
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
    Gateway,           // Host serves as gateway for subnet
    DnsResolution,     // Host provides DNS for subnet
    ReverseProxy,
    HostGroup,     // User-defined logical connection
    MediaStream
}

impl TypeMetadataProvider for EdgeType {
    fn id(&self) -> String {
        self.discriminant().to_string()
    }
    fn display_name(&self) -> &str {
        match self {
            EdgeType::Gateway => "Gateway",
            EdgeType::DnsResolution => "DNS Resolution",
            EdgeType::HostGroup => "Host Group",
            EdgeType::ReverseProxy => "Reverse Proxy",
            EdgeType::MediaStream => "Media Streaming"
        }
    }
    fn category(&self) -> &str {
        ""
    }
    fn color(&self) -> &str {
        match self {
            EdgeType::Gateway =>GATEWAY_COLOR,
            EdgeType::DnsResolution => DNS_COLOR,
            EdgeType::HostGroup => HOST_GROUP_COLOR,
            EdgeType::ReverseProxy => REVERSE_PROXY_COLOR,
            EdgeType::MediaStream => MEDIA_COLOR
        }
    }
    fn description(&self) -> &str {
        match self {
            EdgeType::Gateway => "",
            EdgeType::DnsResolution => "",
            EdgeType::HostGroup => "",
            EdgeType::ReverseProxy => "",
            EdgeType::MediaStream => ""
        }
    }
    fn icon(&self) -> &str {
        ""
    }
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}