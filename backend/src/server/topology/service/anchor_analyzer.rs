use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::{
    service::context::TopologyContext,
    types::edges::{Edge, EdgeHandle},
};

pub struct AnchorAnalyzer;

impl AnchorAnalyzer {
    /// Analyze edges to determine optimal anchor placement for a child node
    /// Returns (primary_handle, total_edges, should_relocate_handles)
    pub fn analyze_child_anchors(
        interface_id: Uuid,
        edges: &[Edge],
        ctx: &TopologyContext,
    ) -> (Option<EdgeHandle>, usize, bool) {
        // Find all edges involving this child
        let child_edges: Vec<_> = edges
            .iter()
            .filter(|edge| edge.source == interface_id || edge.target == interface_id)
            .collect();

        let total_edges = child_edges.len();

        if child_edges.is_empty() {
            return (None, 0, false);
        }

        // Determine if this interface has infra services
        let is_infra = Self::is_interface_infra(interface_id, ctx);

        // Count anchors by handle direction
        let mut handle_counts: HashMap<EdgeHandle, usize> = HashMap::new();

        for edge in &child_edges {
            // Determine which handle applies to this child
            let relevant_handle = if edge.source == interface_id {
                &edge.source_handle
            } else {
                &edge.target_handle
            };

            *handle_counts.entry(*relevant_handle).or_insert(0) += 1;
        }

        let (primary_handle, should_relocate) =
            Self::calculate_optimal_handle(&handle_counts, is_infra, interface_id, edges, ctx);

        (primary_handle, total_edges, should_relocate)
    }

    fn is_interface_infra(interface_id: Uuid, ctx: &TopologyContext) -> bool {
        if let Some(subnet) = ctx.get_subnet_from_interface_id(interface_id) {
            let infra_interfaces = ctx.get_interfaces_with_infra_service(subnet);
            return infra_interfaces.contains(&Some(interface_id));
        }
        false
    }

    /// Calculate the optimal handle placement based on edge distribution
    /// Returns (handle, should_relocate_handles)
    /// should_relocate_handles is true when edges should be relocated to avoid crossing nodes
    fn calculate_optimal_handle(
        handle_counts: &HashMap<EdgeHandle, usize>,
        is_infra: bool,
        interface_id: Uuid,
        edges: &[Edge],
        ctx: &TopologyContext,
    ) -> (Option<EdgeHandle>, bool) {
        if handle_counts.is_empty() {
            return (None, false);
        }

        // Get edge counts
        let top_count = *handle_counts.get(&EdgeHandle::Top).unwrap_or(&0);
        let bottom_count = *handle_counts.get(&EdgeHandle::Bottom).unwrap_or(&0);
        let left_count = *handle_counts.get(&EdgeHandle::Left).unwrap_or(&0);
        let right_count = *handle_counts.get(&EdgeHandle::Right).unwrap_or(&0);

        // Check for opposing edges
        let has_opposing_vertical = top_count > 0 && bottom_count > 0;
        let has_opposing_horizontal = left_count > 0 && right_count > 0;

        // Get subnet info
        let subnet = ctx.get_subnet_from_interface_id(interface_id);
        let subnet_node_count = if let Some(subnet) = subnet {
            ctx.hosts
                .iter()
                .flat_map(|h| &h.base.interfaces)
                .filter(|i| i.base.subnet_id == subnet.id)
                .count()
        } else {
            0
        };

        // Key decision logic:
        // 1. If we have opposing vertical edges AND the subnet has multiple nodes (3+),
        //    we should place the node on the edge (Left for infra, Right for non-infra)
        //    to avoid edges crossing through the middle of the subnet
        // 2. Otherwise, prefer the handle with the most edges

        if has_opposing_vertical && subnet_node_count >= 3 {
            // Check if edges would actually cross nodes
            let would_cross = Self::would_vertical_edges_cross_middle(interface_id, edges, ctx);

            if would_cross {
                // Place on the appropriate edge to avoid crossing
                let preferred_handle = if is_infra {
                    EdgeHandle::Left
                } else {
                    EdgeHandle::Right
                };
                return (Some(preferred_handle), true);
            }
        }

        // If we have opposing horizontal edges and they would cross nodes,
        // prefer vertical placement
        if has_opposing_horizontal && subnet_node_count >= 3 {
            let would_cross = Self::would_horizontal_edges_cross_middle(interface_id, edges, ctx);

            if would_cross {
                return (
                    Some(if bottom_count >= top_count {
                        EdgeHandle::Bottom
                    } else {
                        EdgeHandle::Top
                    }),
                    false,
                );
            }
        }

        // Standard case: choose handle based on edge count
        // But prefer handles that don't conflict with subnet layout
        let max_count = top_count.max(bottom_count).max(left_count).max(right_count);

        if max_count == 0 {
            return (Some(EdgeHandle::Top), false);
        }

        // Prioritize vertical handles as they're safer
        if top_count == max_count {
            return (Some(EdgeHandle::Top), false);
        }
        if bottom_count == max_count {
            return (Some(EdgeHandle::Bottom), false);
        }

        // Only use horizontal handles if allowed and they're the clear winner
        if is_infra && left_count == max_count {
            return (Some(EdgeHandle::Left), false);
        }
        if !is_infra && right_count == max_count {
            return (Some(EdgeHandle::Right), false);
        }

        // Fallback to vertical with most edges
        if top_count >= bottom_count {
            (Some(EdgeHandle::Top), false)
        } else {
            (Some(EdgeHandle::Bottom), false)
        }
    }

    /// Check if vertical edges (top/bottom) would cross through the middle of the subnet
    fn would_vertical_edges_cross_middle(
        interface_id: Uuid,
        edges: &[Edge],
        ctx: &TopologyContext,
    ) -> bool {
        // If this node has edges going both up and down to different subnets,
        // and the subnet has other nodes, then vertical edges would cross the middle
        let mut has_upward_edge = false;
        let mut has_downward_edge = false;

        for edge in edges {
            if edge.source != interface_id && edge.target != interface_id {
                continue;
            }

            // Get the other end
            let other_interface = if edge.source == interface_id {
                edge.target
            } else {
                edge.source
            };

            // Check if it's in a different subnet
            let this_subnet = ctx.get_subnet_from_interface_id(interface_id);
            let other_subnet = ctx.get_subnet_from_interface_id(other_interface);

            if this_subnet.map(|s| s.id) == other_subnet.map(|s| s.id) {
                continue; // Same subnet, skip
            }

            // Check the handle direction
            let relevant_handle = if edge.source == interface_id {
                &edge.source_handle
            } else {
                &edge.target_handle
            };

            match relevant_handle {
                EdgeHandle::Top => has_upward_edge = true,
                EdgeHandle::Bottom => has_downward_edge = true,
                _ => {}
            }
        }

        has_upward_edge && has_downward_edge
    }

    /// Check if horizontal edges (left/right) would cross through the middle of the subnet
    fn would_horizontal_edges_cross_middle(
        interface_id: Uuid,
        edges: &[Edge],
        ctx: &TopologyContext,
    ) -> bool {
        let mut has_leftward_edge = false;
        let mut has_rightward_edge = false;

        for edge in edges {
            if edge.source != interface_id && edge.target != interface_id {
                continue;
            }

            let other_interface = if edge.source == interface_id {
                edge.target
            } else {
                edge.source
            };

            let this_subnet = ctx.get_subnet_from_interface_id(interface_id);
            let other_subnet = ctx.get_subnet_from_interface_id(other_interface);

            if this_subnet.map(|s| s.id) == other_subnet.map(|s| s.id) {
                continue;
            }

            let relevant_handle = if edge.source == interface_id {
                &edge.source_handle
            } else {
                &edge.target_handle
            };

            match relevant_handle {
                EdgeHandle::Left => has_leftward_edge = true,
                EdgeHandle::Right => has_rightward_edge = true,
                _ => {}
            }
        }

        has_leftward_edge && has_rightward_edge
    }
}
