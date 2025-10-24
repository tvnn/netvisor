use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::types::{
    base::Ixy,
    edges::Edge,
    nodes::{Node, NodeType},
};

pub struct OptimizerUtils {}

impl Default for OptimizerUtils {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizerUtils {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculate total edge length for a set of edges
    pub fn calculate_edge_length(
        &self,
        nodes: &[Node],
        edges: &[&Edge],
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) -> f64 {
        let node_map: HashMap<Uuid, &Node> = nodes.iter().map(|n| (n.id, n)).collect();
        let mut total_length = 0.0;

        for edge in edges {
            if let (Some(src_node), Some(tgt_node)) =
                (node_map.get(&edge.source), node_map.get(&edge.target))
            {
                let src_pos = self.get_absolute_node_position(src_node, subnet_positions);
                let tgt_pos = self.get_absolute_node_position(tgt_node, subnet_positions);

                let dx = (tgt_pos.x - src_pos.x) as f64;
                let dy = (tgt_pos.y - src_pos.y) as f64;
                total_length += (dx * dx + dy * dy).sqrt();
            }
        }

        total_length
    }

    pub fn swap_node_positions(&self, nodes: &mut [Node], node_id_1: Uuid, node_id_2: Uuid) {
        let mut pos1: Option<Ixy> = None;
        let mut pos2: Option<Ixy> = None;

        for node in nodes.iter() {
            if node.id == node_id_1 {
                pos1 = Some(node.position);
            } else if node.id == node_id_2 {
                pos2 = Some(node.position);
            }
        }

        if let (Some(p1), Some(p2)) = (pos1, pos2) {
            for node in nodes.iter_mut() {
                if node.id == node_id_1 {
                    node.position = p2;
                } else if node.id == node_id_2 {
                    node.position = p1;
                }
            }
        }
    }

    pub fn count_edge_crossings(
        &self,
        edges: &[&Edge],
        node_map: &HashMap<Uuid, Node>,
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

    pub fn edges_cross(
        &self,
        edge1: &Edge,
        edge2: &Edge,
        node_map: &HashMap<Uuid, Node>,
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

        match (node1_src, node1_tgt, node2_src, node2_tgt) {
            (Some(node1_src), Some(node1_tgt), Some(node2_src), Some(node2_tgt)) => {
                let pos1 = self.get_absolute_node_center(node1_src, subnet_positions);
                let pos2 = self.get_absolute_node_center(node1_tgt, subnet_positions);
                let pos3 = self.get_absolute_node_center(node2_src, subnet_positions);
                let pos4 = self.get_absolute_node_center(node2_tgt, subnet_positions);

                self.segments_intersect(pos1, pos2, pos3, pos4)
            }
            _ => false,
        }
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

    /// Get absolute position of a node (including subnet offset)
    pub fn get_absolute_node_position(
        &self,
        node: &Node,
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) -> Ixy {
        let mut pos = node.position;

        if let NodeType::InterfaceNode { subnet_id, .. } = node.node_type
            && let Some(subnet_pos) = subnet_positions.get(&subnet_id)
        {
            pos.x += subnet_pos.x;
            pos.y += subnet_pos.y;
        }

        pos
    }

    /// Get absolute center point of a node (including subnet offset)
    pub fn get_absolute_node_center(
        &self,
        node: &Node,
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) -> Ixy {
        let mut abs_pos = Ixy {
            x: node.position.x + (node.size.x as isize / 2),
            y: node.position.y + (node.size.y as isize / 2),
        };

        if let NodeType::InterfaceNode { subnet_id, .. } = node.node_type
            && let Some(subnet_pos) = subnet_positions.get(&subnet_id)
        {
            abs_pos.x += subnet_pos.x;
            abs_pos.y += subnet_pos.y;
        }

        abs_pos
    }
}
