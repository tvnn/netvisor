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

            *handle_counts.entry(relevant_handle.clone()).or_insert(0) += 1;
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

        // Determine which handles are forbidden based on infra status
        let forbidden_handle = if is_infra {
            EdgeHandle::Left
        } else {
            EdgeHandle::Right
        };

        // Check for opposing vertical edges (Top + Bottom)
        let has_top = handle_counts.get(&EdgeHandle::Top).unwrap_or(&0) > &0;
        let has_bottom = handle_counts.get(&EdgeHandle::Bottom).unwrap_or(&0) > &0;
        let has_opposing_vertical = has_top && has_bottom;

        // Check for opposing horizontal edges (Left + Right)
        let has_left = handle_counts.get(&EdgeHandle::Left).unwrap_or(&0) > &0;
        let has_right = handle_counts.get(&EdgeHandle::Right).unwrap_or(&0) > &0;
        let has_opposing_horizontal = has_left && has_right;

        // Check if edges would actually cross nodes before relocating
        let would_cross = Self::would_edges_cross_nodes(
            interface_id,
            edges,
            ctx,
            has_opposing_vertical,
            has_opposing_horizontal,
        );

        // Special case: If node has edges on both top and bottom AND they would cross nodes,
        // place it on the side to avoid vertical edges traversing the subnet
        if has_opposing_vertical && would_cross {
            let preferred_side = if forbidden_handle == EdgeHandle::Left {
                EdgeHandle::Right
            } else {
                EdgeHandle::Left
            };
            return (Some(preferred_side), true); // true = relocate handles
        }

        // Special case: If node has edges on both left and right AND they would cross nodes,
        // place it on top or bottom based on which has more edges
        if has_opposing_horizontal && would_cross {
            let top_count = handle_counts.get(&EdgeHandle::Top).unwrap_or(&0);
            let bottom_count = handle_counts.get(&EdgeHandle::Bottom).unwrap_or(&0);

            return (
                Some(if bottom_count > top_count {
                    EdgeHandle::Bottom
                } else {
                    EdgeHandle::Top
                }),
                false,
            );
        }

        // Standard case: place node on the side with the most edges
        // This minimizes edge length by placing the node close to its connections
        let top_count = *handle_counts.get(&EdgeHandle::Top).unwrap_or(&0);
        let bottom_count = *handle_counts.get(&EdgeHandle::Bottom).unwrap_or(&0);
        let left_count = *handle_counts.get(&EdgeHandle::Left).unwrap_or(&0);
        let right_count = *handle_counts.get(&EdgeHandle::Right).unwrap_or(&0);

        // Find the handle with the most edges
        let max_count = top_count.max(bottom_count).max(left_count).max(right_count);

        // If the forbidden handle has the most edges, use the second best
        if max_count == 0 {
            return (Some(EdgeHandle::Top), false);
        }

        // Try to use the handle with most edges
        if top_count == max_count {
            return (Some(EdgeHandle::Top), false);
        }
        if bottom_count == max_count {
            return (Some(EdgeHandle::Bottom), false);
        }
        if right_count == max_count && forbidden_handle != EdgeHandle::Right {
            return (Some(EdgeHandle::Right), false);
        }
        if left_count == max_count && forbidden_handle != EdgeHandle::Left {
            return (Some(EdgeHandle::Left), false);
        }

        // Forbidden handle had the most - fall back to vertical
        Self::fallback_to_vertical(handle_counts)
            .map(|h| (Some(h), false))
            .unwrap_or((Some(EdgeHandle::Top), false))
    }

    /// Fallback to most common vertical handle when horizontal is forbidden
    fn fallback_to_vertical(handle_counts: &HashMap<EdgeHandle, usize>) -> Option<EdgeHandle> {
        let top_count = handle_counts.get(&EdgeHandle::Top).unwrap_or(&0);
        let bottom_count = handle_counts.get(&EdgeHandle::Bottom).unwrap_or(&0);

        if top_count >= bottom_count && *top_count > 0 {
            Some(EdgeHandle::Top)
        } else if *bottom_count > 0 {
            Some(EdgeHandle::Bottom)
        } else {
            // No vertical edges, default to Top
            Some(EdgeHandle::Top)
        }
    }

    /// Check if edges would cross over any nodes in the subnet
    /// Returns true if relocation would help avoid crossings
    fn would_edges_cross_nodes(
        interface_id: Uuid,
        edges: &[Edge],
        ctx: &TopologyContext,
        has_opposing_vertical: bool,
        has_opposing_horizontal: bool,
    ) -> bool {
        // Get the subnet this interface belongs to
        let subnet = match ctx.get_subnet_from_interface_id(interface_id) {
            Some(s) => s,
            None => return false,
        };

        // Find all interfaces in the same subnet
        let subnet_interfaces: Vec<Uuid> = ctx
            .hosts
            .iter()
            .flat_map(|h| &h.base.interfaces)
            .filter(|i| i.base.subnet_id == subnet.id)
            .map(|i| i.id)
            .collect();

        // If there are fewer than 3 nodes in the subnet, edges likely won't cross nodes
        if subnet_interfaces.len() < 3 {
            return false;
        }

        // Count inter-subnet edges that would traverse the subnet
        let mut vertical_edge_count = 0;
        let mut horizontal_edge_count = 0;

        for edge in edges {
            if edge.source != interface_id && edge.target != interface_id {
                continue;
            }

            // Get the other end of the edge
            let other_interface = if edge.source == interface_id {
                edge.target
            } else {
                edge.source
            };

            // Check if the other interface is in a different subnet
            let other_subnet = ctx.get_subnet_from_interface_id(other_interface);
            if other_subnet.map(|s| s.id) == Some(subnet.id) {
                continue; // Intra-subnet edge, skip
            }

            // Determine if this is a vertical or horizontal edge based on handle
            let relevant_handle = if edge.source == interface_id {
                &edge.source_handle
            } else {
                &edge.target_handle
            };

            match relevant_handle {
                EdgeHandle::Top | EdgeHandle::Bottom => vertical_edge_count += 1,
                EdgeHandle::Left | EdgeHandle::Right => horizontal_edge_count += 1,
            }
        }

        // If we have opposing vertical edges and multiple vertical edges exist, likely to cross
        if has_opposing_vertical && vertical_edge_count >= 2 {
            return true;
        }

        // If we have opposing horizontal edges and multiple horizontal edges exist, likely to cross
        if has_opposing_horizontal && horizontal_edge_count >= 2 {
            return true;
        }

        // Additional heuristic: if subnet has many nodes (5+) and opposing edges exist,
        // it's very likely edges will cross nodes
        if subnet_interfaces.len() >= 5 && (has_opposing_vertical || has_opposing_horizontal) {
            return true;
        }

        false
    }
}
