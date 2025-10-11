use crate::server::{
    shared::{
        constants::Entity,
        types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider},
    },
    subnets::types::base::Subnet,
};
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub enum EdgeHandle {
    #[default]
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
        source_is_infra: bool,
        target_is_infra: bool,
    ) -> (EdgeHandle, EdgeHandle) {
        // Special case: edges within the same subnet
        if source_subnet.id == target_subnet.id {
            return Self::from_same_subnet(source_is_infra, target_is_infra);
        }

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
            // Same layer - horizontal flow based on priority and infra status
            std::cmp::Ordering::Equal => {
                match source_priority.cmp(&target_priority) {
                    // Source has lower priority (leftmost) -> flows right
                    std::cmp::Ordering::Less => {
                        let source_handle = if source_is_infra {
                            EdgeHandle::Bottom
                        } else {
                            EdgeHandle::Right
                        };
                        let target_handle = if target_is_infra {
                            EdgeHandle::Bottom
                        } else {
                            EdgeHandle::Left
                        };
                        (source_handle, target_handle)
                    }
                    // Source has higher priority (rightmost) -> flows left
                    std::cmp::Ordering::Greater => {
                        let source_handle = if source_is_infra {
                            EdgeHandle::Bottom
                        } else {
                            EdgeHandle::Left
                        };
                        let target_handle = if target_is_infra {
                            EdgeHandle::Bottom
                        } else {
                            EdgeHandle::Right
                        };
                        (source_handle, target_handle)
                    }
                    // Same priority
                    std::cmp::Ordering::Equal => {
                        let source_handle = if source_is_infra {
                            EdgeHandle::Bottom
                        } else {
                            EdgeHandle::Right
                        };
                        let target_handle = if target_is_infra {
                            EdgeHandle::Bottom
                        } else {
                            EdgeHandle::Left
                        };
                        (source_handle, target_handle)
                    }
                }
            }
        }
    }

    /// Handle edges within the same subnet - defer to anchor analysis
    /// For intra-subnet edges, we can't know the optimal handles until nodes are positioned
    /// So we return neutral defaults that will be overridden by anchor analysis
    fn from_same_subnet(
        _source_is_infra: bool,
        _target_is_infra: bool,
    ) -> (EdgeHandle, EdgeHandle) {
        // For intra-subnet edges, use Top as a neutral default
        // The anchor analyzer will determine the actual optimal placement
        // based on the node's actual position and all its edges
        (EdgeHandle::Top, EdgeHandle::Top)
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

        let has_start_marker = false;

        let has_end_marker = match &self {
            EdgeType::Group => true,
            EdgeType::Interface => false,
        };

        serde_json::json!({
            "is_dashed": is_dashed,
            "has_start_marker": has_start_marker,
            "has_end_marker": has_end_marker
        })
    }
}
