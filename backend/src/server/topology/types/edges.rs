use crate::server::{
    groups::types::{GroupType, GroupTypeDiscriminants},
    shared::{
        constants::Entity,
        types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider},
    },
    subnets::types::base::Subnet,
    topology::types::base::Ixy,
};
use serde::{Deserialize, Serialize};
use strum::{IntoDiscriminant, IntoEnumIterator};
use strum_macros::{Display, EnumDiscriminants, EnumIter, IntoStaticStr};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType,
    pub label: Option<String>,
    pub source_handle: EdgeHandle,
    pub target_handle: EdgeHandle,
}

#[derive(
    Serialize, Copy, Deserialize, Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Default,
)]
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

    pub fn direction(&self) -> Ixy {
        match self {
            EdgeHandle::Top => Ixy { x: 0, y: 1 },
            EdgeHandle::Bottom => Ixy { x: 0, y: -1 },
            EdgeHandle::Left => Ixy { x: -1, y: 0 },
            EdgeHandle::Right => Ixy { x: 1, y: 0 },
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

        let source_vertical_order = source_subnet.base.subnet_type.vertical_order();
        let source_horizontal_order = source_subnet.base.subnet_type.horizontal_order();
        let target_vertical_order = target_subnet.base.subnet_type.vertical_order();
        let target_horizontal_order = target_subnet.base.subnet_type.horizontal_order();

        match source_vertical_order.cmp(&target_vertical_order) {
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
                match source_horizontal_order.cmp(&target_horizontal_order) {
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
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, IntoStaticStr,
)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum EdgeType {
    Interface, // Connecting hosts with interfaces in multiple subnets
    HostVirtualization,
    ServiceVirtualization,
    #[serde(untagged)]
    Group(GroupTypeDiscriminants), // User-defined logical connection
}

impl EdgeType {
    pub fn all_variants() -> Vec<EdgeType> {
        let mut variants = Vec::new();

        EdgeTypeDiscriminants::iter().for_each(|discriminant| match discriminant {
            EdgeTypeDiscriminants::Interface => {
                variants.push(EdgeType::Interface);
            }
            EdgeTypeDiscriminants::Group => {
                variants.extend(GroupType::iter().map(|g| EdgeType::Group(g.discriminant())));
            }
            EdgeTypeDiscriminants::HostVirtualization => {
                variants.push(EdgeType::HostVirtualization)
            }
            EdgeTypeDiscriminants::ServiceVirtualization => {
                variants.push(EdgeType::ServiceVirtualization)
            }
        });

        variants
    }
}

impl HasId for EdgeType {
    fn id(&self) -> &'static str {
        match self {
            EdgeType::Interface => self.into(),
            EdgeType::Group(group_type) => group_type.into(),
            EdgeType::HostVirtualization => self.into(),
            EdgeType::ServiceVirtualization => self.into(),
        }
    }
}

impl EntityMetadataProvider for EdgeType {
    fn color(&self) -> &'static str {
        match self {
            EdgeType::Group(_) => Entity::Group.color(),
            EdgeType::Interface => Entity::Host.color(),
            EdgeType::HostVirtualization => Entity::Virtualization.color(),
            EdgeType::ServiceVirtualization => Entity::Virtualization.color(),
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            EdgeType::Group(group_type) => group_type.icon(),
            EdgeType::Interface => Entity::Host.icon(),
            EdgeType::HostVirtualization => Entity::Virtualization.icon(),
            EdgeType::ServiceVirtualization => Entity::Virtualization.icon(),
        }
    }
}

impl TypeMetadataProvider for EdgeType {
    fn name(&self) -> &'static str {
        match self {
            EdgeType::Group(group_type) => group_type.name(),
            EdgeType::Interface => "Host Interface",
            EdgeType::HostVirtualization => "Virtualized Host",
            EdgeType::ServiceVirtualization => "Virtualized Service",
        }
    }

    fn metadata(&self) -> serde_json::Value {
        let is_dashed = match &self {
            EdgeType::Group(_) => false,
            EdgeType::Interface => true,
            EdgeType::HostVirtualization => true,
            EdgeType::ServiceVirtualization => true,
        };

        let has_start_marker = false;

        let has_end_marker = match &self {
            EdgeType::Group(_) => true,
            EdgeType::Interface => false,
            EdgeType::HostVirtualization => false,
            EdgeType::ServiceVirtualization => false,
        };

        serde_json::json!({
            "is_dashed": is_dashed,
            "has_start_marker": has_start_marker,
            "has_end_marker": has_end_marker,
        })
    }
}
