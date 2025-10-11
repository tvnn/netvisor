use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::types::{
    base::Ixy,
    edges::{Edge, EdgeHandle},
    nodes::{Node, NodeType},
};

pub struct CrossingReducer {
    max_iterations: usize,
}

impl Default for CrossingReducer {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossingReducer {
    pub fn new() -> Self {
        Self { max_iterations: 10 }
    }

    /// Reduce edge crossings by swapping nodes within subnets
    pub fn reduce_crossings(
        &self,
        nodes: &mut [Node],
        edges: &[Edge],
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) {
        let inter_subnet_edges: Vec<&Edge> = edges
            .iter()
            .filter(|edge| {
                let source_subnet = Self::get_node_subnet(edge.source, nodes);
                let target_subnet = Self::get_node_subnet(edge.target, nodes);
                source_subnet.is_some() && target_subnet.is_some() && source_subnet != target_subnet
            })
            .collect();

        if inter_subnet_edges.is_empty() {
            return;
        }

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
                                subnet_positions,
                            )
                        {
                            improved = true;
                        }
                    });
                });
            });
        }
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

        let pos1 = Self::get_absolute_node_center(node1_src.unwrap(), subnet_positions);
        let pos2 = Self::get_absolute_node_center(node1_tgt.unwrap(), subnet_positions);
        let pos3 = Self::get_absolute_node_center(node2_src.unwrap(), subnet_positions);
        let pos4 = Self::get_absolute_node_center(node2_tgt.unwrap(), subnet_positions);

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
