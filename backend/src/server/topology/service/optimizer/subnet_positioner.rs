use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::{
    service::{
        context::TopologyContext, optimizer::utils::OptimizerUtils,
        subnet_layout_planner::SUBNET_PADDING,
    },
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
            max_iterations: 10,
            context: ctx,
            utils: OptimizerUtils::new(),
        }
    }

    /// Snap a position to the nearest grid point
    fn snap_to_grid(value: isize) -> isize {
        ((value as f64 / GRID_SIZE as f64).round() as isize) * GRID_SIZE
    }

    /// Optimize subnet positions starting from top row, cascading downward
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

        // Group subnets by their row (Y position), sorted from top to bottom (ascending Y)
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

        // Sort by Y ascending (top to bottom)
        subnets_by_row.sort_by(|a, b| a.0.cmp(&b.0));

        let mut improved = true;
        let mut iteration = 0;

        while improved && iteration < self.max_iterations {
            improved = false;
            iteration += 1;

            // Process cascading groups: optimize rows 0..i relative to rows below them
            for cascade_depth in 1..=subnets_by_row.len() {
                // Get the top `cascade_depth` rows
                let rows_to_optimize: Vec<Uuid> = subnets_by_row
                    .iter()
                    .take(cascade_depth)
                    .flat_map(|(_, subnets)| subnets.clone())
                    .collect();

                // Step 1: Move this group of rows to optimal aggregate position
                // Only consider edges going from these rows to rows below them
                if self.optimize_cascading_rows(
                    nodes,
                    &rows_to_optimize,
                    edges,
                    cascade_depth,
                    &subnets_by_row,
                ) {
                    improved = true;
                }

                // Step 2: Optimize leftmost and rightmost subnets in the bottommost row of this cascade
                if cascade_depth > 0 {
                    let current_row_subnets = &subnets_by_row[cascade_depth - 1].1;

                    if current_row_subnets.len() > 1 {
                        let mut sorted_subnets = current_row_subnets.clone();
                        sorted_subnets.sort_by_key(|id| {
                            nodes
                                .iter()
                                .find(|n| n.id == *id)
                                .map(|n| n.position.x)
                                .unwrap_or(0)
                        });

                        if let Some(leftmost) = sorted_subnets.first()
                            && self.optimize_edge_subnet(nodes, *leftmost, edges, true)
                        {
                            improved = true;
                        }

                        if let Some(rightmost) = sorted_subnets.last()
                            && self.optimize_edge_subnet(nodes, *rightmost, edges, false)
                        {
                            improved = true;
                        }
                    }
                }
            }

            for (_, row_subnets) in &subnets_by_row {
                if row_subnets.len() == 1 {
                    // Single subnet in row - optimize without constraints
                    let subnet_id = row_subnets[0];
                    if self.optimize_single_subnet(nodes, subnet_id, edges) {
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

    /// Move a group of rows together to minimize edge length to rows below them
    fn optimize_cascading_rows(
        &self,
        nodes: &mut [Node],
        rows_to_optimize: &[Uuid],
        edges: &[Edge],
        cascade_depth: usize,
        all_rows: &[(isize, Vec<Uuid>)],
    ) -> bool {
        if rows_to_optimize.is_empty() {
            return false;
        }

        let current_length = self.calculate_total_edge_length(nodes, edges);

        // Capture original positions
        let original_positions: Vec<(Uuid, Ixy)> = rows_to_optimize
            .iter()
            .filter_map(|id| {
                nodes
                    .iter()
                    .find(|n| n.id == *id)
                    .map(|n| (*id, n.position))
            })
            .collect();

        // Get subnets in rows below the current cascade
        let subnets_below: Vec<Uuid> = all_rows
            .iter()
            .skip(cascade_depth)
            .flat_map(|(_, subnets)| subnets.clone())
            .collect();

        // Calculate optimal X shift for these rows relative to rows below
        let optimal_shift =
            self.calculate_cascading_shift(nodes, rows_to_optimize, &subnets_below, edges);

        if optimal_shift == 0 {
            return false;
        }

        // Move all subnets in these rows by the optimal shift
        for subnet_id in rows_to_optimize {
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

    fn calculate_cascading_shift(
        &self,
        nodes: &[Node],
        rows_to_optimize: &[Uuid],
        subnets_below: &[Uuid],
        edges: &[Edge],
    ) -> isize {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let mut target_x_positions: Vec<f64> = Vec::new();

        // For each edge from rows_to_optimize to subnets_below, collect target positions
        for edge in edges {
            let source_subnet = self.context.get_node_subnet(edge.source, nodes);
            let target_subnet = self.context.get_node_subnet(edge.target, nodes);

            // Edge from optimizing rows to rows below
            if rows_to_optimize.contains(&source_subnet.unwrap_or_default())
                && subnets_below.contains(&target_subnet.unwrap_or_default())
            {
                if let Some(target_node) = node_map.get(&edge.target) {
                    let target_pos = self
                        .utils
                        .get_absolute_node_center(target_node, &subnet_positions);
                    target_x_positions.push(target_pos.x as f64);
                }
            }
            // Edge from rows below to optimizing rows
            else if subnets_below.contains(&source_subnet.unwrap_or_default())
                && rows_to_optimize.contains(&target_subnet.unwrap_or_default())
                && let Some(source_node) = node_map.get(&edge.source)
            {
                let source_pos = self
                    .utils
                    .get_absolute_node_center(source_node, &subnet_positions);
                target_x_positions.push(source_pos.x as f64);
            }
        }

        if target_x_positions.is_empty() {
            return 0;
        }

        // Calculate median of connection points to rows below
        let median_target = Self::calculate_median(&mut target_x_positions);

        // Calculate current center X of rows being optimized
        let current_center: f64 = rows_to_optimize
            .iter()
            .filter_map(|id| subnet_positions.get(id))
            .map(|pos| pos.x as f64)
            .sum::<f64>()
            / rows_to_optimize.len() as f64;

        Self::snap_to_grid((median_target - current_center).round() as isize)
    }

    fn optimize_edge_subnet(
        &self,
        nodes: &mut [Node],
        subnet_id: Uuid,
        edges: &[Edge],
        is_left: bool,
    ) -> bool {
        let current_length = self.calculate_total_edge_length(nodes, edges);

        let original_pos = nodes.iter().find(|n| n.id == subnet_id).map(|n| n.position);

        if let Some(pos) = original_pos {
            let optimal_x = self.calculate_optimal_subnet_x(nodes, subnet_id, edges);

            // Find neighboring subnet constraint
            let neighbor_constraint = self.find_neighbor_constraint(nodes, subnet_id, is_left);

            // Apply constraint based on whether there's a neighbor
            let new_x = match neighbor_constraint {
                Some(constraint_x) => {
                    // Has neighbor: clamp optimal position to not overlap neighbor
                    if is_left {
                        // Leftmost subnet: can't move right past the constraint
                        Self::snap_to_grid(optimal_x.min(constraint_x))
                    } else {
                        // Rightmost subnet: can't move left past the constraint
                        Self::snap_to_grid(optimal_x.max(constraint_x))
                    }
                }
                None => {
                    // No neighbor: move freely to optimal position
                    Self::snap_to_grid(optimal_x)
                }
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

    fn optimize_single_subnet(&self, nodes: &mut [Node], subnet_id: Uuid, edges: &[Edge]) -> bool {
        let current_length = self.calculate_total_edge_length(nodes, edges);
        let original_pos = match nodes.iter().find(|n| n.id == subnet_id).map(|n| n.position) {
            Some(pos) => pos,
            None => return false,
        };

        let optimal_x =
            Self::snap_to_grid(self.calculate_optimal_subnet_x(nodes, subnet_id, edges));

        if optimal_x == original_pos.x {
            return false;
        }

        let new_pos = Ixy {
            x: optimal_x,
            y: original_pos.y,
        };
        self.move_subnet(nodes, subnet_id, new_pos);

        let new_length = self.calculate_total_edge_length(nodes, edges);

        if new_length < current_length {
            true
        } else {
            self.move_subnet(nodes, subnet_id, original_pos);
            false
        }
    }

    fn find_neighbor_constraint(
        &self,
        nodes: &[Node],
        subnet_id: Uuid,
        is_left: bool,
    ) -> Option<isize> {
        let current_subnet = nodes.iter().find(|n| n.id == subnet_id)?;
        let current_y = current_subnet.position.y;
        let current_x = current_subnet.position.x;
        let current_width = current_subnet.size.x;

        let constraints: Vec<isize> = nodes
            .iter()
            .filter(|n| {
                matches!(n.node_type, NodeType::SubnetNode { .. })
                    && n.id != subnet_id
                    && n.position.y == current_y
            })
            .filter_map(|n| {
                if is_left && n.position.x > current_x {
                    let constraint =
                        n.position.x - SUBNET_PADDING.x as isize - current_width as isize;
                    Some(constraint)
                } else if !is_left && n.position.x < current_x {
                    let constraint = n.position.x + n.size.x as isize + SUBNET_PADDING.x as isize;
                    Some(constraint)
                } else {
                    None
                }
            })
            .collect();

        if constraints.is_empty() {
            return None;
        }

        if is_left {
            constraints.into_iter().min()
        } else {
            constraints.into_iter().max()
        }
    }

    fn calculate_optimal_subnet_x(&self, nodes: &[Node], subnet_id: Uuid, edges: &[Edge]) -> isize {
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
            let source_subnet = self.context.get_node_subnet(edge.source, nodes);
            let target_subnet = self.context.get_node_subnet(edge.target, nodes);

            // Edge from this subnet to another
            if source_subnet == Some(subnet_id) && target_subnet != Some(subnet_id) {
                if let Some(source_node) = node_map.get(&edge.source) {
                    let source_pos = self
                        .utils
                        .get_absolute_node_center(source_node, &subnet_positions);
                    current_x_positions.push(source_pos.x as f64);
                }
                if let Some(target_node) = node_map.get(&edge.target) {
                    let target_pos = self
                        .utils
                        .get_absolute_node_center(target_node, &subnet_positions);
                    target_x_positions.push(target_pos.x as f64);
                }
            }
            // Edge from another subnet to this one
            else if target_subnet == Some(subnet_id) && source_subnet != Some(subnet_id) {
                if let Some(target_node) = node_map.get(&edge.target) {
                    let target_pos = self
                        .utils
                        .get_absolute_node_center(target_node, &subnet_positions);
                    current_x_positions.push(target_pos.x as f64);
                }
                if let Some(source_node) = node_map.get(&edge.source) {
                    let source_pos = self
                        .utils
                        .get_absolute_node_center(source_node, &subnet_positions);
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

        Self::snap_to_grid(current_subnet_x + shift)
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
