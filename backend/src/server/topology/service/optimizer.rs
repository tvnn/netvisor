use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::types::{
    base::Ixy,
    edges::{Edge, EdgeHandle},
    nodes::{Node, NodeType},
};

pub struct TopologyOptimizer {
    max_iterations: usize,
}

impl Default for TopologyOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl TopologyOptimizer {
    pub fn new() -> Self {
        Self { max_iterations: 10 }
    }

    pub fn reduce_edge_crossings(&self, nodes: &mut [Node], edges: &[Edge]) {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position.clone())),
                _ => None,
            })
            .collect();

        let inter_subnet_edges: Vec<&Edge> = edges
            .iter()
            .filter(|edge| {
                let source_subnet = self.get_node_subnet(edge.source, nodes);
                let target_subnet = self.get_node_subnet(edge.target, nodes);
                source_subnet.is_some() && target_subnet.is_some() && source_subnet != target_subnet
            })
            .collect();

        if inter_subnet_edges.is_empty() {
            return;
        }

        // Step 1: Optimize node positions within subnets to reduce edge crossings
        let node_handles = self.determine_node_handles(&inter_subnet_edges);
        let subnet_groups = self.group_nodes_by_subnet_type_and_handle(nodes, &node_handles);

        let mut improved = true;
        let mut iteration = 0;

        while improved && iteration < self.max_iterations {
            improved = false;
            iteration += 1;

            subnet_groups.iter().for_each(|(_sub_id, type_groups)| {
                type_groups.iter().for_each(|(_is_infra, handle_groups)| {
                    handle_groups.iter().for_each(|(_handle, node_ids)| {
                        if node_ids.len() >= 2
                            && self.try_swaps_in_group(
                                nodes,
                                node_ids,
                                &inter_subnet_edges,
                                &subnet_positions,
                            )
                        {
                            improved = true;
                        }
                    });
                });
            });
        }

        // Step 2: Optimize subnet positions to minimize edge lengths
        self.optimize_subnet_positions(nodes, &inter_subnet_edges);
    }

    /// Move a subnet to a new position
    fn move_subnet(&self, nodes: &mut [Node], subnet_id: Uuid, new_position: Ixy) {
        // Only move the subnet container - child nodes are in relative coordinates
        // so they don't need to be updated when the subnet moves
        for node in nodes.iter_mut() {
            if node.id == subnet_id {
                node.position = new_position;
                break;
            }
        }
    }

    /// Optimize subnet positions starting from bottom row, moving up
    fn optimize_subnet_positions(&self, nodes: &mut [Node], edges: &[&Edge]) {
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
                    .map(|n| (*id, n.position.clone()))
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
                NodeType::SubnetNode { .. } => Some((n.id, n.position.clone())),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let mut current_x_positions: Vec<f64> = Vec::new();
        let mut target_x_positions: Vec<f64> = Vec::new();

        // For each subnet in the row, find all connected HostNodes and their targets
        for subnet_id in row_subnets {
            for edge in edges {
                let source_subnet = self.get_node_subnet(edge.source, nodes);
                let target_subnet = self.get_node_subnet(edge.target, nodes);

                // Edge from this row to another row
                if source_subnet == Some(*subnet_id)
                    && !row_subnets.contains(&target_subnet.unwrap_or_default())
                {
                    if let Some(source_node) = node_map.get(&edge.source) {
                        let source_pos =
                            self.get_absolute_node_center(source_node, &subnet_positions);
                        current_x_positions.push(source_pos.x as f64);
                    }
                    if let Some(target_node) = node_map.get(&edge.target) {
                        let target_pos =
                            self.get_absolute_node_center(target_node, &subnet_positions);
                        target_x_positions.push(target_pos.x as f64);
                    }
                }
                // Edge from another row to this row
                else if target_subnet == Some(*subnet_id)
                    && !row_subnets.contains(&source_subnet.unwrap_or_default())
                {
                    if let Some(target_node) = node_map.get(&edge.target) {
                        let target_pos =
                            self.get_absolute_node_center(target_node, &subnet_positions);
                        current_x_positions.push(target_pos.x as f64);
                    }
                    if let Some(source_node) = node_map.get(&edge.source) {
                        let source_pos =
                            self.get_absolute_node_center(source_node, &subnet_positions);
                        target_x_positions.push(source_pos.x as f64);
                    }
                }
            }
        }

        if target_x_positions.is_empty() || current_x_positions.is_empty() {
            return 0;
        }

        // Calculate median of current connected HostNode positions in this row
        current_x_positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_current = if current_x_positions.len() % 2 == 0 {
            let mid = current_x_positions.len() / 2;
            (current_x_positions[mid - 1] + current_x_positions[mid]) / 2.0
        } else {
            current_x_positions[current_x_positions.len() / 2]
        };

        // Calculate median of target positions
        target_x_positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_target = if target_x_positions.len() % 2 == 0 {
            let mid = target_x_positions.len() / 2;
            (target_x_positions[mid - 1] + target_x_positions[mid]) / 2.0
        } else {
            target_x_positions[target_x_positions.len() / 2]
        };

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
        let original_pos = nodes
            .iter()
            .find(|n| n.id == subnet_id)
            .map(|n| n.position.clone());

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
                NodeType::SubnetNode { .. } => Some((n.id, n.position.clone())),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let mut current_x_positions: Vec<f64> = Vec::new();
        let mut target_x_positions: Vec<f64> = Vec::new();

        for edge in edges {
            let source_subnet = self.get_node_subnet(edge.source, nodes);
            let target_subnet = self.get_node_subnet(edge.target, nodes);

            // Edge from this subnet to another
            if source_subnet == Some(subnet_id) && target_subnet != Some(subnet_id) {
                if let Some(source_node) = node_map.get(&edge.source) {
                    let source_pos = self.get_absolute_node_center(source_node, &subnet_positions);
                    current_x_positions.push(source_pos.x as f64);
                }
                if let Some(target_node) = node_map.get(&edge.target) {
                    let target_pos = self.get_absolute_node_center(target_node, &subnet_positions);
                    target_x_positions.push(target_pos.x as f64);
                }
            }
            // Edge from another subnet to this one
            else if target_subnet == Some(subnet_id) && source_subnet != Some(subnet_id) {
                if let Some(target_node) = node_map.get(&edge.target) {
                    let target_pos = self.get_absolute_node_center(target_node, &subnet_positions);
                    current_x_positions.push(target_pos.x as f64);
                }
                if let Some(source_node) = node_map.get(&edge.source) {
                    let source_pos = self.get_absolute_node_center(source_node, &subnet_positions);
                    target_x_positions.push(source_pos.x as f64);
                }
            }
        }

        if target_x_positions.is_empty() || current_x_positions.is_empty() {
            return subnet_positions.get(&subnet_id).map(|p| p.x).unwrap_or(0);
        }

        // Calculate median of current connected HostNode positions
        current_x_positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_current = if current_x_positions.len() % 2 == 0 {
            let mid = current_x_positions.len() / 2;
            (current_x_positions[mid - 1] + current_x_positions[mid]) / 2.0
        } else {
            current_x_positions[current_x_positions.len() / 2]
        };

        // Calculate median of target positions
        target_x_positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_target = if target_x_positions.len() % 2 == 0 {
            let mid = target_x_positions.len() / 2;
            (target_x_positions[mid - 1] + target_x_positions[mid]) / 2.0
        } else {
            target_x_positions[target_x_positions.len() / 2]
        };

        // Calculate shift and apply to current subnet position
        let shift = (median_target - median_current).round() as isize;
        let current_subnet_x = subnet_positions.get(&subnet_id).map(|p| p.x).unwrap_or(0);
        current_subnet_x + shift
    }

    fn calculate_total_edge_length(&self, nodes: &[Node], edges: &[&Edge]) -> f64 {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position.clone())),
                _ => None,
            })
            .collect();

        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();

        let mut total_length = 0.0;

        for edge in edges {
            if let (Some(src_node), Some(tgt_node)) =
                (node_map.get(&edge.source), node_map.get(&edge.target))
            {
                let pos1 = self.get_absolute_node_center(src_node, &subnet_positions);
                let pos2 = self.get_absolute_node_center(tgt_node, &subnet_positions);

                let dx = pos2.x as f64 - pos1.x as f64;
                let dy = pos2.y as f64 - pos1.y as f64;
                total_length += (dx * dx + dy * dy).sqrt();
            }
        }

        total_length
    }

    fn determine_node_handles(&self, edges: &[&Edge]) -> HashMap<Uuid, EdgeHandle> {
        let mut node_handles: HashMap<Uuid, HashMap<EdgeHandle, usize>> = HashMap::new();

        for edge in edges {
            *node_handles
                .entry(edge.source)
                .or_default()
                .entry(edge.source_handle.clone())
                .or_insert(0) += 1;

            *node_handles
                .entry(edge.target)
                .or_default()
                .entry(edge.target_handle.clone())
                .or_insert(0) += 1;
        }

        node_handles
            .into_iter()
            .filter_map(|(node_id, handles)| {
                handles
                    .into_iter()
                    .max_by_key(|(_, count)| *count)
                    .map(|(handle, _)| (node_id, handle))
            })
            .collect()
    }

    fn group_nodes_by_subnet_type_and_handle(
        &self,
        nodes: &[Node],
        node_handles: &HashMap<Uuid, EdgeHandle>,
    ) -> HashMap<Uuid, HashMap<bool, HashMap<EdgeHandle, Vec<Uuid>>>> {
        let mut groups: HashMap<Uuid, HashMap<bool, HashMap<EdgeHandle, Vec<Uuid>>>> =
            HashMap::new();

        for node in nodes {
            match node.node_type {
                NodeType::SubnetNode { .. } => continue,
                NodeType::HostNode {
                    subnet_id,
                    is_infra,
                    ..
                } => {
                    if let Some(handle) = node_handles.get(&node.id) {
                        groups
                            .entry(subnet_id)
                            .or_default()
                            .entry(is_infra)
                            .or_default()
                            .entry(handle.clone())
                            .or_default()
                            .push(node.id);
                    }
                }
            }
        }

        groups
    }

    fn get_absolute_node_center(&self, node: &Node, subnet_positions: &HashMap<Uuid, Ixy>) -> Ixy {
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

    fn get_node_subnet(&self, node_id: Uuid, nodes: &[Node]) -> Option<Uuid> {
        nodes
            .iter()
            .find(|n| n.id == node_id)
            .and_then(|node| match node.node_type {
                NodeType::HostNode { subnet_id, .. } => Some(subnet_id),
                _ => None,
            })
    }

    fn try_swaps_in_group(
        &self,
        all_nodes: &mut [Node],
        node_ids: &[Uuid],
        edges: &[&Edge],
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) -> bool {
        if node_ids.len() < 2 {
            return false;
        }

        let node_map: HashMap<Uuid, &Node> = all_nodes.iter().map(|n| (n.id, n)).collect();

        let current_crossings = self.count_edge_crossings(edges, &node_map, subnet_positions);

        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                let node_id_1 = node_ids[i];
                let node_id_2 = node_ids[j];

                self.swap_node_positions(all_nodes, node_id_1, node_id_2);

                let node_map: HashMap<Uuid, &Node> = all_nodes.iter().map(|n| (n.id, n)).collect();

                let new_crossings = self.count_edge_crossings(edges, &node_map, subnet_positions);

                if new_crossings < current_crossings {
                    return true;
                } else {
                    self.swap_node_positions(all_nodes, node_id_1, node_id_2);
                }
            }
        }

        false
    }

    fn swap_node_positions(&self, nodes: &mut [Node], node_id_1: Uuid, node_id_2: Uuid) {
        let mut pos1: Option<Ixy> = None;
        let mut pos2: Option<Ixy> = None;

        for node in nodes.iter() {
            if node.id == node_id_1 {
                pos1 = Some(node.position.clone());
            } else if node.id == node_id_2 {
                pos2 = Some(node.position.clone());
            }
        }

        if let (Some(p1), Some(p2)) = (pos1, pos2) {
            for node in nodes.iter_mut() {
                if node.id == node_id_1 {
                    node.position = p2.clone();
                } else if node.id == node_id_2 {
                    node.position = p1.clone();
                }
            }
        }
    }

    fn count_edge_crossings(
        &self,
        edges: &[&Edge],
        node_map: &HashMap<Uuid, &Node>,
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) -> usize {
        let mut crossings = 0;

        for i in 0..edges.len() {
            for j in (i + 1)..edges.len() {
                if self.edges_cross(edges[i], edges[j], node_map, subnet_positions) {
                    crossings += 1;
                }
            }
        }

        crossings
    }

    fn edges_cross(
        &self,
        edge1: &Edge,
        edge2: &Edge,
        node_map: &HashMap<Uuid, &Node>,
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) -> bool {
        if edge1.source == edge2.source
            || edge1.source == edge2.target
            || edge1.target == edge2.source
            || edge1.target == edge2.target
        {
            return false;
        }

        let node1_src = node_map.get(&edge1.source);
        let node1_tgt = node_map.get(&edge1.target);
        let node2_src = node_map.get(&edge2.source);
        let node2_tgt = node_map.get(&edge2.target);

        if node1_src.is_none() || node1_tgt.is_none() || node2_src.is_none() || node2_tgt.is_none()
        {
            return false;
        }

        let pos1 = self.get_absolute_node_center(node1_src.unwrap(), subnet_positions);
        let pos2 = self.get_absolute_node_center(node1_tgt.unwrap(), subnet_positions);
        let pos3 = self.get_absolute_node_center(node2_src.unwrap(), subnet_positions);
        let pos4 = self.get_absolute_node_center(node2_tgt.unwrap(), subnet_positions);

        self.segments_intersect(pos1, pos2, pos3, pos4)
    }

    fn segments_intersect(&self, p1: Ixy, p2: Ixy, p3: Ixy, p4: Ixy) -> bool {
        let x1 = p1.x as f64;
        let y1 = p1.y as f64;
        let x2 = p2.x as f64;
        let y2 = p2.y as f64;
        let x3 = p3.x as f64;
        let y3 = p3.y as f64;
        let x4 = p4.x as f64;
        let y4 = p4.y as f64;

        let d1 = self.direction(x3, y3, x4, y4, x1, y1);
        let d2 = self.direction(x3, y3, x4, y4, x2, y2);
        let d3 = self.direction(x1, y1, x2, y2, x3, y3);
        let d4 = self.direction(x1, y1, x2, y2, x4, y4);

        if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
            && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
        {
            return true;
        }

        if d1.abs() < f64::EPSILON && self.on_segment(x3, y3, x4, y4, x1, y1) {
            return true;
        }
        if d2.abs() < f64::EPSILON && self.on_segment(x3, y3, x4, y4, x2, y2) {
            return true;
        }
        if d3.abs() < f64::EPSILON && self.on_segment(x1, y1, x2, y2, x3, y3) {
            return true;
        }
        if d4.abs() < f64::EPSILON && self.on_segment(x1, y1, x2, y2, x4, y4) {
            return true;
        }

        false
    }

    fn direction(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> f64 {
        (x3 - x1) * (y2 - y1) - (y3 - y1) * (x2 - x1)
    }

    fn on_segment(&self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> bool {
        x <= x1.max(x2) && x >= x1.min(x2) && y <= y1.max(y2) && y >= y1.min(y2)
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
