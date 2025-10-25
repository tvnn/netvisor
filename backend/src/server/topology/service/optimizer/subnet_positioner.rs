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

    /// Snap a position to the nearest grid point
    fn snap_to_grid(value: f64) -> isize {
        ((value / GRID_SIZE as f64).round() as isize) * GRID_SIZE
    }

    /// Main optimization: optimize all subnets simultaneously based on their connections
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

        let mut improved = true;
        let mut iteration = 0;

        while improved && iteration < self.max_iterations {
            iteration += 1;

            let initial_length = self.calculate_total_edge_length(nodes, edges);

            // Save original positions to revert if worse
            let original_positions: HashMap<Uuid, isize> = nodes
                .iter()
                .filter_map(|n| match n.node_type {
                    NodeType::SubnetNode { .. } => Some((n.id, n.position.x)),
                    _ => None,
                })
                .collect();

            // Calculate optimal position for ALL subnets simultaneously
            let mut new_positions: HashMap<Uuid, isize> = HashMap::new();

            for &subnet_id in &subnet_ids {
                let optimal_x = self.calculate_optimal_x(nodes, edges, subnet_id);

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

            let new_length = self.calculate_total_edge_length(nodes, edges);

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

    /// Calculate optimal X position for a subnet based on median of connected nodes
    fn calculate_optimal_x(&self, nodes: &[Node], edges: &[Edge], subnet_id: Uuid) -> isize {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        // Collect relative positions of nodes within this subnet
        let mut relative_positions: Vec<f64> = Vec::new();
        // Collect absolute positions of external targets
        let mut target_positions: Vec<f64> = Vec::new();

        for edge in edges {
            let source_subnet = self.context.get_node_subnet(edge.source, nodes);
            let target_subnet = self.context.get_node_subnet(edge.target, nodes);

            // Skip intra-subnet edges
            if source_subnet == target_subnet {
                continue;
            }

            // Edge from this subnet to another
            if source_subnet == Some(subnet_id) {
                if let Some(src_node) = node_map.get(&edge.source) {
                    let relative_x = src_node.position.x + (src_node.size.x as isize / 2);
                    relative_positions.push(relative_x as f64);
                }
                if let Some(tgt_node) = node_map.get(&edge.target) {
                    let target_pos = self
                        .utils
                        .get_absolute_node_center(tgt_node, &subnet_positions);
                    target_positions.push(target_pos.x as f64);
                }
            }
            // Edge from another subnet to this one
            else if target_subnet == Some(subnet_id) {
                if let Some(tgt_node) = node_map.get(&edge.target) {
                    let relative_x = tgt_node.position.x + (tgt_node.size.x as isize / 2);
                    relative_positions.push(relative_x as f64);
                }
                if let Some(src_node) = node_map.get(&edge.source) {
                    let source_pos = self
                        .utils
                        .get_absolute_node_center(src_node, &subnet_positions);
                    target_positions.push(source_pos.x as f64);
                }
            }
        }

        if target_positions.is_empty() || relative_positions.is_empty() {
            // No connections, keep current position
            return subnet_positions.get(&subnet_id).map(|p| p.x).unwrap_or(0);
        }

        let median_relative = Self::calculate_median(&mut relative_positions);
        let median_target = Self::calculate_median(&mut target_positions);

        // Optimal position: align median of internal nodes with median of targets
        let optimal_x = median_target - median_relative;

        Self::snap_to_grid(optimal_x)
    }

    /// Apply constraint to prevent overlapping with other subnets in the same row
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
            let other_x = already_positioned
                .get(&other.id)
                .copied()
                .unwrap_or(other.position.x);
            let other_width = other.size.x as isize;

            // Check for overlap
            let proposed_right = bounded_proposed_x + width;
            let other_right = other_x + other_width;

            // Would we overlap?
            if bounded_proposed_x < other_right + padding && proposed_right + padding > other_x {
                // Determine which side to push to based on current position
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

    fn calculate_total_edge_length(&self, nodes: &[Node], edges: &[Edge]) -> f64 {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let inter_subnet_edges: Vec<&Edge> = edges
            .iter()
            .filter(|e| !self.context.edge_is_intra_subnet(e))
            .collect();

        let mut total_length = 0.0;

        for edge in inter_subnet_edges {
            if let (Some(src_node), Some(tgt_node)) =
                (node_map.get(&edge.source), node_map.get(&edge.target))
            {
                let pos1 = self
                    .utils
                    .get_absolute_node_center(src_node, &subnet_positions);
                let pos2 = self
                    .utils
                    .get_absolute_node_center(tgt_node, &subnet_positions);

                let dx = pos2.x as f64 - pos1.x as f64;
                let dy = pos2.y as f64 - pos1.y as f64;
                total_length += (dx * dx + dy * dy).sqrt();
            }
        }

        total_length
    }

    fn calculate_median(values: &mut [f64]) -> f64 {
        values.sort_by(|a, b| a.total_cmp(b));
        if values.len().is_multiple_of(2) {
            let mid = values.len() / 2;
            (values[mid - 1] + values[mid]) / 2.0
        } else {
            values[values.len() / 2]
        }
    }
}
