use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::{
    service::{context::TopologyContext, optimizer::utils::OptimizerUtils},
    types::{
        base::Ixy,
        edges::Edge,
        nodes::{Node, NodeType},
    },
};

const GRID_SIZE: isize = 25;

/// Subnet positioner using barycenter/median heuristic
///
/// ALGORITHM: Barycenter Heuristic (from Sugiyama Framework)
///
/// This positions subnets by aligning them with their external connection targets.
/// The algorithm is now immune to internal topology changes within subnets.
///
/// Key principles:
/// - Uses median (not mean) for robustness against outliers
/// - Positions based on external subnet centers, not internal node positions
/// - Optimizes all subnets simultaneously in each iteration
/// - Applies non-overlap constraints to prevent subnet collisions
/// - Iterates until convergence or max iterations reached
pub struct SubnetPositioner<'a> {
    max_iterations: usize,
    context: &'a TopologyContext<'a>,
    utils: OptimizerUtils,
}

impl<'a> SubnetPositioner<'a> {
    pub fn new(ctx: &'a TopologyContext<'a>) -> Self {
        Self {
            max_iterations: 20,
            context: ctx,
            utils: OptimizerUtils::new(),
        }
    }

    /// Snap a position to the nearest grid point for visual alignment
    fn snap_to_grid(value: f64) -> isize {
        ((value / GRID_SIZE as f64).round() as isize) * GRID_SIZE
    }

    /// Main optimization: optimize all subnets simultaneously based on their connections
    ///
    /// This implements an iterative refinement approach:
    /// 1. For each subnet, calculate optimal X position using median heuristic
    /// 2. Apply non-overlap constraints to ensure subnets don't collide
    /// 3. Evaluate if total edge length improved
    /// 4. If yes, keep changes and continue; if no, revert and stop
    ///
    /// Stops when: no improvement OR max iterations reached
    pub fn optimize_positions(&self, nodes: &mut [Node], edges: &[Edge]) {
        let subnet_ids: Vec<Uuid> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some(n.id),
                _ => None,
            })
            .collect();

        if subnet_ids.is_empty() {
            return;
        }

        let inter_subnet_edges: Vec<Edge> = edges
            .iter()
            .filter(|e| !self.context.edge_is_intra_subnet(e))
            .cloned()
            .collect();

        let mut improved = true;
        let mut iteration = 0;

        while improved && iteration < self.max_iterations {
            iteration += 1;

            let initial_length = self
                .utils
                .calculate_total_edge_length(nodes, &inter_subnet_edges);

            // Save original positions to revert if worse
            let original_positions: HashMap<Uuid, isize> = nodes
                .iter()
                .filter_map(|n| match n.node_type {
                    NodeType::SubnetNode { .. } => Some((n.id, n.position.x)),
                    _ => None,
                })
                .collect();

            // Calculate optimal position for ALL subnets simultaneously
            // This avoids sequential bias where early subnets get better positions
            let mut new_positions: HashMap<Uuid, isize> = HashMap::new();

            for &subnet_id in &subnet_ids {
                let optimal_x = self.calculate_optimal_x(nodes, &inter_subnet_edges, subnet_id);

                // Apply non-overlap constraints
                let constrained_x =
                    self.apply_non_overlap_constraint(nodes, subnet_id, optimal_x, &new_positions);

                new_positions.insert(subnet_id, constrained_x);
            }

            // Apply all new positions at once
            for (subnet_id, new_x) in &new_positions {
                if let Some(subnet) = nodes.iter_mut().find(|n| n.id == *subnet_id) {
                    subnet.position.x = *new_x;
                }
            }

            let new_length = self
                .utils
                .calculate_total_edge_length(nodes, &inter_subnet_edges);

            if new_length < initial_length {
                improved = true;
            } else {
                // Revert - this move made things worse
                for (subnet_id, original_x) in original_positions {
                    if let Some(subnet) = nodes.iter_mut().find(|n| n.id == subnet_id) {
                        subnet.position.x = original_x;
                    }
                }
                // Stop if we can't improve
                break;
            }
        }
    }

    /// Calculate optimal X position for a subnet using median heuristic
    ///
    /// ALGORITHM: Simplified Median Barycenter
    ///
    /// This version is immune to internal topology changes:
    /// 1. Find all external subnets this subnet connects to
    /// 2. Calculate median of those external subnet CENTER positions
    /// 3. Position this subnet's CENTER at that median
    ///
    /// By using subnet centers rather than individual node positions,
    /// internal topology changes (like VM host edges) don't affect subnet positioning.
    fn calculate_optimal_x(&self, nodes: &[Node], edges: &[Edge], subnet_id: Uuid) -> isize {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        // Collect the center X positions of external subnets this subnet connects to
        let mut external_subnet_positions: Vec<f64> = Vec::new();

        for edge in edges {
            let source_subnet = self.context.get_node_subnet(edge.source, nodes);
            let target_subnet = self.context.get_node_subnet(edge.target, nodes);

            // Skip if not an inter-subnet edge
            if source_subnet == target_subnet {
                continue;
            }

            // Ignore edges with horizontal connections; vertical edges with unecessary steps result in worse visual quality than
            // longer horizontal edges
            if edge.source_handle.is_horizontal() || edge.target_handle.is_horizontal() {
                continue;
            }

            // Find the external subnet we're connected to
            let external_subnet_id = if source_subnet == Some(subnet_id) {
                target_subnet
            } else if target_subnet == Some(subnet_id) {
                source_subnet
            } else {
                continue; // Edge doesn't involve this subnet
            };

            // Get the center position of the external subnet
            if let Some(ext_subnet_id) = external_subnet_id
                && let Some(&ext_pos) = subnet_positions.get(&ext_subnet_id)
                && let Some(ext_subnet) = nodes.iter().find(|n| n.id == ext_subnet_id)
            {
                let center_x = ext_pos.x + (ext_subnet.size.x as isize / 2);
                external_subnet_positions.push(center_x as f64);
            }
        }

        if external_subnet_positions.is_empty() {
            // No external connections, keep current position
            return subnet_positions.get(&subnet_id).map(|p| p.x).unwrap_or(0);
        }

        // Calculate median of external subnet centers
        let median_external = self.utils.calculate_median(&mut external_subnet_positions);

        // Get our subnet's width to center it properly
        let subnet_width = nodes
            .iter()
            .find(|n| n.id == subnet_id)
            .map(|n| n.size.x as isize)
            .unwrap_or(0);

        // Position subnet so its center aligns with median of external connections
        let optimal_x = median_external as isize - (subnet_width / 2);

        Self::snap_to_grid(optimal_x as f64)
    }

    /// Apply constraint to prevent overlapping with other subnets in the same row
    ///
    /// This ensures subnets remain non-overlapping while still moving toward
    /// their optimal positions. Uses already-positioned subnets to avoid
    /// order-dependent behavior.
    fn apply_non_overlap_constraint(
        &self,
        nodes: &[Node],
        subnet_id: Uuid,
        proposed_x: isize,
        already_positioned: &HashMap<Uuid, isize>,
    ) -> isize {
        let current_subnet = match nodes.iter().find(|n| n.id == subnet_id) {
            Some(s) => s,
            None => return proposed_x,
        };

        let current_x = current_subnet.position.x;
        let y = current_subnet.position.y;
        let width = current_subnet.size.x as isize;
        let padding = 50;

        // Limit maximum movement per iteration to prevent wild swings
        // This adds stability and prevents oscillation
        let max_move = 200;
        let bounded_proposed_x = if (proposed_x - current_x).abs() > max_move {
            if proposed_x > current_x {
                current_x + max_move
            } else {
                current_x - max_move
            }
        } else {
            proposed_x
        };

        // Check against other subnets in the same row
        for other in nodes.iter() {
            if !matches!(other.node_type, NodeType::SubnetNode { .. })
                || other.id == subnet_id
                || other.position.y != y
            {
                continue;
            }

            // Use the new position if already calculated, otherwise use current position
            // This ensures we check against the most up-to-date positions
            let other_x = already_positioned
                .get(&other.id)
                .copied()
                .unwrap_or(other.position.x);
            let other_width = other.size.x as isize;

            // Check for overlap using bounding boxes
            let proposed_right = bounded_proposed_x + width;
            let other_right = other_x + other_width;

            // Would we overlap?
            if bounded_proposed_x < other_right + padding && proposed_right + padding > other_x {
                // Determine which side to push to based on current position
                // This maintains the current spatial relationship
                if current_x < other_x {
                    // We're on the left, stay on the left
                    return (other_x - width - padding).min(bounded_proposed_x);
                } else {
                    // We're on the right, stay on the right
                    return (other_right + padding).max(bounded_proposed_x);
                }
            }
        }

        bounded_proposed_x
    }
}
