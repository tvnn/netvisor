use std::collections::HashMap;

use crate::server::{
    shared::{
        constants::Entity,
        types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider},
    }, subnets::types::base::Subnet,
};
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter, IntoStaticStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType,
    pub label: String,
    pub source_handle: EdgeHandle,
    pub target_handle: EdgeHandle,
}

// Intermediate representation of edge information
pub struct EdgeInfo<'a> {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub source_subnet: &'a Subnet,
    pub target_subnet: &'a Subnet,
    pub edge_type: EdgeType,
    pub label: String,
}

impl<'a> EdgeInfo<'a> {
    // Convert to actual Edge when node_indices are available
    pub fn to_edge(&self, node_indices: &HashMap<Uuid, NodeIndex>) -> Option<(NodeIndex, NodeIndex, Edge)> {
        let source_idx = node_indices.get(&self.source_id)?;
        let target_idx = node_indices.get(&self.target_id)?;
        
        let (source_handle, target_handle) = EdgeHandle::from_subnet_layers(self.source_subnet, self.target_subnet);
        
        Some((
            *source_idx,
            *target_idx,
            Edge {
                edge_type: self.edge_type.clone(),
                label: self.label.clone(),
                source: self.source_id,
                target: self.target_id,
                source_handle,
                target_handle,
            }
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum EdgeHandle {
    Top,
    Bottom,
    Left,
    Right,
}

impl EdgeHandle {
    pub fn layout_priority(&self) -> u8 {
        match self {
            EdgeHandle::Top => 0,
            EdgeHandle::Bottom => 1,
            EdgeHandle::Left => 2,
            EdgeHandle::Right => 3,
        }
    }
    /// Determine edge handle orientations based on subnet layer and priority
    pub fn from_subnet_layers(
        source_subnet: &Subnet,
        target_subnet: &Subnet,
    ) -> (EdgeHandle, EdgeHandle) {

        let source_layer = source_subnet.base.subnet_type.default_layer();
        let source_priority = source_subnet.base.subnet_type.layer_priority();
        let target_layer = target_subnet.base.subnet_type.default_layer();
        let target_priority = target_subnet.base.subnet_type.layer_priority();

        match source_layer.cmp(&target_layer) {
            // Different layers - vertical flow
            std::cmp::Ordering::Less => {
                // Edge flows downward: source Bottom -> target Top
                (EdgeHandle::Bottom, EdgeHandle::Top)
            }
            std::cmp::Ordering::Greater => {
                // Edge flows upward: source Top -> target Bottom
                (EdgeHandle::Top, EdgeHandle::Bottom)
            }
            // Same layer - horizontal flow based on priority
            std::cmp::Ordering::Equal => {
                match source_priority.cmp(&target_priority) {
                    // Source has lower priority (leftmost) -> flows right
                    std::cmp::Ordering::Less => {
                        (EdgeHandle::Right, EdgeHandle::Left)
                    }
                    // Source has higher priority (rightmost) -> flows left
                    std::cmp::Ordering::Greater => {
                        (EdgeHandle::Left, EdgeHandle::Right)
                    }
                    // Same priority (shouldn't happen, but handle it)
                    std::cmp::Ordering::Equal => {
                        (EdgeHandle::Right, EdgeHandle::Left)
                    }
                }
            }
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    EnumDiscriminants,
    EnumIter,
    IntoStaticStr,
)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum EdgeType {
    Interface, // Connecting hosts with interfaces in multiple subnets
    Group,     // User-defined logical connection
}

impl HasId for EdgeType {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for EdgeType {
    fn color(&self) -> &'static str {
        match self {
            EdgeType::Group => Entity::Group.color(),
            EdgeType::Interface => Entity::Host.color(),
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            EdgeType::Group => Entity::Group.icon(),
            EdgeType::Interface => Entity::Host.icon(),
        }
    }
}

impl TypeMetadataProvider for EdgeType {
    fn name(&self) -> &'static str {
        match self {
            EdgeType::Group => "Host Group",
            EdgeType::Interface => "Host Interface",
        }
    }

    fn metadata(&self) -> serde_json::Value {
        let is_dashed = match &self {
            EdgeType::Group => false,
            EdgeType::Interface => true,
        };

        serde_json::json!({
            "is_dashed": is_dashed,
        })
    }
}
