use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::types::{
    base::Ixy,
    edges::Edge,
    nodes::{Node, NodeType},
};

pub struct SubnetPositioner {
    max_iterations: usize,
}

impl Default for SubnetPositioner {
    fn default() -> Self {
        Self::new()
    }
}

impl SubnetPositioner {
    pub fn new() -> Self {
        Self { max_iterations: 10 }
    }

    /// Optimize subnet positions starting from bottom row, moving up
    pub fn optimize_positions(&self, nodes: &mut [Node], edges: &[&Edge]) {
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

        // Group subnets by their row (Y position), sorted from bottom to top (descending Y)
        let mut subnets_by_row: Vec<(isize, Vec<Uuid>)> = HashMap::new()
            .tap_mut(|map: &mut HashMap<isize, Vec<Uuid>>| {
                for subnet_id in &subnet_ids {
                    if let Some(subnet) = nodes.iter().find(|n| n.id == *subnet_id) {
                        map.entry(subnet.position.y).or_default().push(*subnet_id);
                    }
                }
            })
            .into_iter()
            .collect();

        // Sort by Y descending (bottom to top)
        subnets_by_row.sort_by(|a, b| b.0.cmp(&a.0));

        let mut improved = true;
        let mut iteration = 0;

        while improved && iteration < self.max_iterations {
            improved = false;
            iteration += 1;

            // Process each row from bottom to top
            for (_row_y, row_subnets) in &subnets_by_row {
                // Step 1: Move entire row to optimal aggregate position
                if self.optimize_row_position(nodes, row_subnets, edges) {
                    improved = true;
                }

                // Step 2: Optimize leftmost and rightmost subnets independently
                if row_subnets.len() > 1 {
                    // Sort by X position
                    let mut sorted_subnets = row_subnets.clone();
                    sorted_subnets.sort_by_key(|id| {
                        nodes
                            .iter()
                            .find(|n| n.id == *id)
                            .map(|n| n.position.x)
                            .unwrap_or(0)
                    });

                    let leftmost = sorted_subnets.first().unwrap();
                    let rightmost = sorted_subnets.last().unwrap();

                    if self.optimize_edge_subnet(nodes, *leftmost, edges, true) {
                        improved = true;
                    }
                    if self.optimize_edge_subnet(nodes, *rightmost, edges, false) {
                        improved = true;
                    }
                }
            }
        }
    }

    /// Move a subnet to a new position
    fn move_subnet(&self, nodes: &mut [Node], subnet_id: Uuid, new_position: Ixy) {
        for node in nodes.iter_mut() {
            if node.id == subnet_id {
                node.position = new_position;
                break;
            }
        }
    }

    /// Move all subnets in a row together to minimize total edge length
    fn optimize_row_position(
        &self,
        nodes: &mut [Node],
        row_subnets: &[Uuid],
        edges: &[&Edge],
    ) -> bool {
        if row_subnets.is_empty() {
            return false;
        }

        let current_length = self.calculate_total_edge_length(nodes, edges);

        // Capture original positions BEFORE calculating optimal shift
        let original_positions: Vec<(Uuid, Ixy)> = row_subnets
            .iter()
            .filter_map(|id| {
                nodes
                    .iter()
                    .find(|n| n.id == *id)
                    .map(|n| (*id, n.position))
            })
            .collect();

        // Calculate optimal X shift for the entire row
        let optimal_shift = self.calculate_optimal_row_shift(nodes, row_subnets, edges);

        if optimal_shift == 0 {
            return false;
        }

        // Move all subnets in the row by the optimal shift
        for subnet_id in row_subnets {
            if let Some(subnet) = nodes.iter().find(|n| n.id == *subnet_id) {
                let new_x = subnet.position.x + optimal_shift;
                let new_pos = Ixy {
                    x: new_x,
                    y: subnet.position.y,
                };
                self.move_subnet(nodes, *subnet_id, new_pos);
            }
        }

        let new_length = self.calculate_total_edge_length(nodes, edges);

        if new_length < current_length {
            true
        } else {
            // Revert using captured original positions
            for (subnet_id, original_pos) in original_positions {
                self.move_subnet(nodes, subnet_id, original_pos);
            }
            false
        }
    }

    /// Calculate optimal horizontal shift for an entire row
    fn calculate_optimal_row_shift(
        &self,
        nodes: &[Node],
        row_subnets: &[Uuid],
        edges: &[&Edge],
    ) -> isize {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let mut current_x_positions: Vec<f64> = Vec::new();
        let mut target_x_positions: Vec<f64> = Vec::new();

        // For each subnet in the row, find all connected HostNodes and their targets
        for subnet_id in row_subnets {
            for edge in edges {
                let source_subnet = Self::get_node_subnet(edge.source, nodes);
                let target_subnet = Self::get_node_subnet(edge.target, nodes);

                // Edge from this row to another row
                if source_subnet == Some(*subnet_id)
                    && !row_subnets.contains(&target_subnet.unwrap_or_default())
                {
                    if let Some(source_node) = node_map.get(&edge.source) {
                        let source_pos =
                            Self::get_absolute_node_center(source_node, &subnet_positions);
                        current_x_positions.push(source_pos.x as f64);
                    }
                    if let Some(target_node) = node_map.get(&edge.target) {
                        let target_pos =
                            Self::get_absolute_node_center(target_node, &subnet_positions);
                        target_x_positions.push(target_pos.x as f64);
                    }
                }
                // Edge from another row to this row
                else if target_subnet == Some(*subnet_id)
                    && !row_subnets.contains(&source_subnet.unwrap_or_default())
                {
                    if let Some(target_node) = node_map.get(&edge.target) {
                        let target_pos =
                            Self::get_absolute_node_center(target_node, &subnet_positions);
                        current_x_positions.push(target_pos.x as f64);
                    }
                    if let Some(source_node) = node_map.get(&edge.source) {
                        let source_pos =
                            Self::get_absolute_node_center(source_node, &subnet_positions);
                        target_x_positions.push(source_pos.x as f64);
                    }
                }
            }
        }

        if target_x_positions.is_empty() || current_x_positions.is_empty() {
            return 0;
        }

        let median_current = Self::calculate_median(&mut current_x_positions);
        let median_target = Self::calculate_median(&mut target_x_positions);

        (median_target - median_current).round() as isize
    }

    /// Optimize position of leftmost or rightmost subnet in a row
    fn optimize_edge_subnet(
        &self,
        nodes: &mut [Node],
        subnet_id: Uuid,
        edges: &[&Edge],
        is_left: bool,
    ) -> bool {
        let current_length = self.calculate_total_edge_length(nodes, edges);

        // Capture original position BEFORE any changes
        let original_pos = nodes.iter().find(|n| n.id == subnet_id).map(|n| n.position);

        if let Some(pos) = original_pos {
            let optimal_x = self.calculate_optimal_subnet_x(nodes, subnet_id, edges);

            // Only allow movement in the appropriate direction
            let new_x = if is_left {
                optimal_x.min(pos.x) // Can only move left
            } else {
                optimal_x.max(pos.x) // Can only move right
            };

            if new_x == pos.x {
                return false;
            }

            let new_pos = Ixy { x: new_x, y: pos.y };
            self.move_subnet(nodes, subnet_id, new_pos);

            let new_length = self.calculate_total_edge_length(nodes, edges);

            if new_length < current_length {
                true
            } else {
                self.move_subnet(nodes, subnet_id, pos);
                false
            }
        } else {
            false
        }
    }

    fn calculate_optimal_subnet_x(
        &self,
        nodes: &[Node],
        subnet_id: Uuid,
        edges: &[&Edge],
    ) -> isize {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let mut current_x_positions: Vec<f64> = Vec::new();
        let mut target_x_positions: Vec<f64> = Vec::new();

        for edge in edges {
            let source_subnet = Self::get_node_subnet(edge.source, nodes);
            let target_subnet = Self::get_node_subnet(edge.target, nodes);

            // Edge from this subnet to another
            if source_subnet == Some(subnet_id) && target_subnet != Some(subnet_id) {
                if let Some(source_node) = node_map.get(&edge.source) {
                    let source_pos = Self::get_absolute_node_center(source_node, &subnet_positions);
                    current_x_positions.push(source_pos.x as f64);
                }
                if let Some(target_node) = node_map.get(&edge.target) {
                    let target_pos = Self::get_absolute_node_center(target_node, &subnet_positions);
                    target_x_positions.push(target_pos.x as f64);
                }
            }
            // Edge from another subnet to this one
            else if target_subnet == Some(subnet_id) && source_subnet != Some(subnet_id) {
                if let Some(target_node) = node_map.get(&edge.target) {
                    let target_pos = Self::get_absolute_node_center(target_node, &subnet_positions);
                    current_x_positions.push(target_pos.x as f64);
                }
                if let Some(source_node) = node_map.get(&edge.source) {
                    let source_pos = Self::get_absolute_node_center(source_node, &subnet_positions);
                    target_x_positions.push(source_pos.x as f64);
                }
            }
        }

        if target_x_positions.is_empty() || current_x_positions.is_empty() {
            return subnet_positions.get(&subnet_id).map(|p| p.x).unwrap_or(0);
        }

        let median_current = Self::calculate_median(&mut current_x_positions);
        let median_target = Self::calculate_median(&mut target_x_positions);

        // Calculate shift and apply to current subnet position
        let shift = (median_target - median_current).round() as isize;
        let current_subnet_x = subnet_positions.get(&subnet_id).map(|p| p.x).unwrap_or(0);
        current_subnet_x + shift
    }

    fn calculate_total_edge_length(&self, nodes: &[Node], edges: &[&Edge]) -> f64 {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let mut total_length = 0.0;

        for edge in edges {
            if let (Some(src_node), Some(tgt_node)) =
                (node_map.get(&edge.source), node_map.get(&edge.target))
            {
                let pos1 = Self::get_absolute_node_center(src_node, &subnet_positions);
                let pos2 = Self::get_absolute_node_center(tgt_node, &subnet_positions);

                let dx = pos2.x as f64 - pos1.x as f64;
                let dy = pos2.y as f64 - pos1.y as f64;
                total_length += (dx * dx + dy * dy).sqrt();
            }
        }

        total_length
    }

    fn calculate_median(values: &mut [f64]) -> f64 {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if values.len().is_multiple_of(2) {
            let mid = values.len() / 2;
            (values[mid - 1] + values[mid]) / 2.0
        } else {
            values[values.len() / 2]
        }
    }

    fn get_absolute_node_center(node: &Node, subnet_positions: &HashMap<Uuid, Ixy>) -> Ixy {
        let mut abs_pos = Ixy {
            x: node.position.x + (node.size.x as isize / 2),
            y: node.position.y + (node.size.y as isize / 2),
        };

        if let NodeType::HostNode { subnet_id, .. } = node.node_type {
            if let Some(subnet_pos) = subnet_positions.get(&subnet_id) {
                abs_pos.x += subnet_pos.x;
                abs_pos.y += subnet_pos.y;
            }
        }

        abs_pos
    }

    fn get_node_subnet(node_id: Uuid, nodes: &[Node]) -> Option<Uuid> {
        nodes
            .iter()
            .find(|n| n.id == node_id)
            .and_then(|node| match node.node_type {
                NodeType::HostNode { subnet_id, .. } => Some(subnet_id),
                _ => None,
            })
    }
}

trait TapMut {
    fn tap_mut<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self);
}

impl<T> TapMut for T {
    fn tap_mut<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}
