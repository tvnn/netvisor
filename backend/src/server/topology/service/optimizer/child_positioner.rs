use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::server::topology::{
    service::{context::TopologyContext, optimizer::utils::OptimizerUtils},
    types::{
        base::Ixy,
        edges::{Edge, EdgeHandle},
        nodes::{Node, NodeType},
    },
};

enum HorizontalDirection {
    Left,
    Right,
    Neutral,
}

enum VerticalDirection {
    Up,
    Down,
    Neutral,
}

/// High-level optimizer that coordinates crossing reduction and subnet positioning
pub struct ChildPositioner<'a> {
    context: &'a TopologyContext<'a>,
    utils: OptimizerUtils,
}

impl<'a> ChildPositioner<'a> {
    pub fn new(ctx: &'a TopologyContext<'a>) -> Self {
        Self {
            context: ctx,
            utils: OptimizerUtils::new(),
        }
    }

    /// Order nodes within subnets based on edge flow using topological sorting
    pub fn order_nodes_by_flow(&self, nodes: &mut [Node], edges: &[Edge]) {
        // Group nodes by subnet AND infra status
        let mut nodes_by_subnet_and_infra: HashMap<(Uuid, bool), Vec<Uuid>> = HashMap::new();

        // Collect node IDs by subnet and infra status
        for node in nodes.iter() {
            if let NodeType::HostNode {
                subnet_id,
                is_infra,
                ..
            } = node.node_type
            {
                nodes_by_subnet_and_infra
                    .entry((subnet_id, is_infra))
                    .or_default()
                    .push(node.id);
            }
        }

        // For each subnet+infra group, compute a flow-based ordering
        for ((subnet_id, is_infra), subnet_node_ids) in nodes_by_subnet_and_infra.iter() {
            if subnet_node_ids.len() < 2 {
                continue;
            }

            // Only process nodes that are part of intra-subnet edges
            let nodes_in_intra_edges: std::collections::HashSet<Uuid> = edges
                .iter()
                .filter_map(|edge| {
                    // Check if both source and target are in this subnet+infra group
                    if subnet_node_ids.contains(&edge.source)
                        && subnet_node_ids.contains(&edge.target)
                    {
                        Some([edge.source, edge.target])
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();

            // If no intra-subnet edges in this group, skip reordering
            if nodes_in_intra_edges.is_empty() {
                continue;
            }

            // Build adjacency for intra-subnet edges only within this group
            let mut adjacency: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
            let mut in_degree: HashMap<Uuid, usize> = HashMap::new();

            for node_id in &nodes_in_intra_edges {
                adjacency.entry(*node_id).or_default();
                in_degree.entry(*node_id).or_insert(0);
            }

            for edge in edges {
                if nodes_in_intra_edges.contains(&edge.source)
                    && nodes_in_intra_edges.contains(&edge.target)
                {
                    adjacency.entry(edge.source).or_default().push(edge.target);
                    *in_degree.entry(edge.target).or_insert(0) += 1;
                }
            }

            // Topological sort (Kahn's algorithm)
            let mut queue: Vec<Uuid> = in_degree
                .iter()
                .filter(|(_, &deg)| deg == 0)
                .map(|(id, _)| *id)
                .collect();

            let mut order = Vec::new();
            let mut visited = std::collections::HashSet::new();

            while let Some(node_id) = queue.pop() {
                if visited.contains(&node_id) {
                    continue;
                }
                visited.insert(node_id);
                order.push(node_id);

                if let Some(neighbors) = adjacency.get(&node_id) {
                    for &neighbor in neighbors {
                        if let Some(deg) = in_degree.get_mut(&neighbor) {
                            *deg = deg.saturating_sub(1);
                            if *deg == 0 {
                                queue.push(neighbor);
                            }
                        }
                    }
                }
            }

            // Only reorder if we have a meaningful chain (2+ nodes)
            if order.len() > 1 {
                Self::reposition_nodes_by_order(nodes, *subnet_id, *is_infra, &order);
            }
        }
    }

    /// Reposition nodes within a subnet+infra section to follow the given order
    fn reposition_nodes_by_order(
        nodes: &mut [Node],
        subnet_id: Uuid,
        is_infra: bool,
        order: &[Uuid],
    ) {
        // Collect current positions of nodes that are being reordered
        let reordered_node_ids: std::collections::HashSet<Uuid> = order.iter().copied().collect();

        let mut reordered_positions: Vec<(Uuid, Ixy)> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::HostNode {
                    subnet_id: sid,
                    is_infra: infra,
                    ..
                } if sid == subnet_id
                    && infra == is_infra
                    && reordered_node_ids.contains(&n.id) =>
                {
                    Some((n.id, n.position))
                }
                _ => None,
            })
            .collect();

        if reordered_positions.is_empty() {
            return;
        }

        // Sort positions by X coordinate (left to right)
        reordered_positions.sort_by_key(|(_, pos)| pos.x);

        // Create a mapping from order index to position
        let position_values: Vec<Ixy> = reordered_positions.iter().map(|(_, pos)| *pos).collect();

        // Assign new positions based on topological order
        for (new_idx, &node_id) in order.iter().enumerate() {
            if new_idx < position_values.len() {
                if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
                    node.position = position_values[new_idx];
                }
            }
        }
    }

    /// Optimize node positions within subnets to reduce edge lengths
    /// Only swap nodes that are in the same row or column
    pub fn optimize_positions(&self, nodes: &mut [Node], edges: &[Edge]) {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        // Separate edges by type for crossing detection
        let inter_edges: Vec<&Edge> = edges
            .iter()
            .filter(|edge| !self.context.edge_is_intra_subnet(edge))
            .collect();

        // Group ALL nodes by (subnet, infra, position)
        let mut nodes_by_subnet_infra: HashMap<(Uuid, bool), Vec<Uuid>> = HashMap::new();
        for node in nodes.iter() {
            if let NodeType::HostNode {
                subnet_id,
                is_infra,
                ..
            } = node.node_type
            {
                nodes_by_subnet_infra
                    .entry((subnet_id, is_infra))
                    .or_default()
                    .push(node.id);
            }
        }

        // For each subnet+infra group, optimize within row/column constraints
        for ((_, _), node_ids) in nodes_by_subnet_infra.iter() {
            if node_ids.len() < 2 {
                continue;
            }

            // Get ALL edges involving nodes in this group
            let group_edges: Vec<&Edge> = edges
                .iter()
                .filter(|e| node_ids.contains(&e.source) || node_ids.contains(&e.target))
                .collect();

            if group_edges.is_empty() {
                continue;
            }

            // Group nodes by row (Y position) and column (X position)
            let mut nodes_by_row: HashMap<isize, Vec<Uuid>> = HashMap::new();
            let mut nodes_by_col: HashMap<isize, Vec<Uuid>> = HashMap::new();

            for &node_id in node_ids {
                if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                    // Don't allow for optimization that would break handle constraints
                    let node_edge_source_handles: Vec<EdgeHandle> = edges
                        .iter()
                        .filter_map(|e| {
                            if node_id == e.source {
                                Some(e.source_handle)
                            } else {
                                None
                            }
                        })
                        .collect();
                    let node_edge_target_handles: Vec<EdgeHandle> = edges
                        .iter()
                        .filter_map(|e| {
                            if node_id == e.target {
                                Some(e.target_handle)
                            } else {
                                None
                            }
                        })
                        .collect();

                    let node_edge_handles: Vec<EdgeHandle> = node_edge_source_handles
                        .into_iter()
                        .chain(node_edge_target_handles)
                        .collect();

                    if node_edge_handles
                        .iter()
                        .all(|h| !matches!(h, EdgeHandle::Bottom | EdgeHandle::Top))
                    {
                        nodes_by_col
                            .entry(node.position.x)
                            .or_default()
                            .push(node_id);
                    }

                    if node_edge_handles
                        .iter()
                        .all(|h| !matches!(h, EdgeHandle::Left | EdgeHandle::Right))
                    {
                        nodes_by_row
                            .entry(node.position.y)
                            .or_default()
                            .push(node_id);
                    }
                }
            }

            // Optimize within each row
            for (_, row_nodes) in nodes_by_row.iter() {
                if row_nodes.len() >= 2 {
                    self.optimize_within_constraint(
                        nodes,
                        row_nodes,
                        &group_edges,
                        &inter_edges,
                        &subnet_positions,
                    );
                }
            }

            // Optimize within each column
            for (_, col_nodes) in nodes_by_col.iter() {
                if col_nodes.len() >= 2 {
                    self.optimize_within_constraint(
                        nodes,
                        col_nodes,
                        &group_edges,
                        &inter_edges,
                        &subnet_positions,
                    );
                }
            }
        }
    }

    /// Fix intra-subnet edge handles based on actual node positions and existing edge handles
    pub fn fix_intra_subnet_handles(&self, edges: &[Edge], nodes: &[Node]) -> Vec<Edge> {
        // Build a map of node ID to position
        let node_positions: HashMap<Uuid, (&Node, Uuid)> = nodes
            .iter()
            .filter_map(|n| match &n.node_type {
                NodeType::HostNode { subnet_id, .. } => Some((n.id, (n, *subnet_id))),
                _ => None,
            })
            .collect();

        // Initialize handle tracking with existing edges
        let mut source_handles_used: HashMap<Uuid, HashSet<EdgeHandle>> = HashMap::new();
        let mut target_handles_used: HashMap<Uuid, HashSet<EdgeHandle>> = HashMap::new();

        for edge in edges {
            source_handles_used
                .entry(edge.source)
                .or_default()
                .insert(edge.source_handle);
            target_handles_used
                .entry(edge.target)
                .or_default()
                .insert(edge.target_handle);
        }

        edges
            .iter()
            .cloned()
            .map(|mut edge| {
                // Remove the old handles from tracking before we reassign
                if let Some(handles) = source_handles_used.get_mut(&edge.source) {
                    handles.remove(&edge.source_handle);
                }
                if let Some(handles) = target_handles_used.get_mut(&edge.target) {
                    handles.remove(&edge.target_handle);
                }

                // Check if this is an intra-subnet edge
                if let (Some((src_node, src_subnet)), Some((tgt_node, tgt_subnet))) = (
                    node_positions.get(&edge.source),
                    node_positions.get(&edge.target),
                ) {
                    if src_subnet == tgt_subnet {
                        let dx = tgt_node.position.x - src_node.position.x;
                        let dy = tgt_node.position.y - src_node.position.y;

                        let (primary_src, alt_src, primary_tgt, alt_tgt) =
                            Self::determine_handle_preferences(dx as f32, dy as f32);

                        // Choose source handle (prefer primary, fall back to alternative)
                        let src_used = source_handles_used.entry(edge.source).or_default();
                        edge.source_handle = if !src_used.contains(&primary_src) {
                            primary_src
                        } else {
                            alt_src
                        };
                        src_used.insert(edge.source_handle);

                        // Choose target handle (prefer primary, fall back to alternative)
                        let tgt_used = target_handles_used.entry(edge.target).or_default();
                        edge.target_handle = if !tgt_used.contains(&primary_tgt) {
                            primary_tgt
                        } else {
                            alt_tgt
                        };
                        tgt_used.insert(edge.target_handle);
                    }
                }
                edge
            })
            .collect()
    }

    fn determine_handle_preferences(
        dx: f32,
        dy: f32,
    ) -> (EdgeHandle, EdgeHandle, EdgeHandle, EdgeHandle) {
        const BOUND: f32 = 10.0;

        let horizontal = if dx > BOUND {
            HorizontalDirection::Right
        } else if dx < -BOUND {
            HorizontalDirection::Left
        } else {
            HorizontalDirection::Neutral
        };

        let vertical = if dy > BOUND {
            VerticalDirection::Down
        } else if dy < -BOUND {
            VerticalDirection::Up
        } else {
            VerticalDirection::Neutral
        };

        match (horizontal, vertical) {
            // Target is to the right and down
            (HorizontalDirection::Right, VerticalDirection::Down) => (
                EdgeHandle::Bottom,
                EdgeHandle::Right,
                EdgeHandle::Left,
                EdgeHandle::Bottom,
            ),
            // Target is to the right and up
            (HorizontalDirection::Right, VerticalDirection::Up) => (
                EdgeHandle::Top,
                EdgeHandle::Right,
                EdgeHandle::Left,
                EdgeHandle::Top,
            ),
            // Target is to the right
            (HorizontalDirection::Right, VerticalDirection::Neutral) => (
                EdgeHandle::Right,
                EdgeHandle::Top,
                EdgeHandle::Left,
                EdgeHandle::Bottom,
            ),

            // Target is to the left and down
            (HorizontalDirection::Left, VerticalDirection::Down) => (
                EdgeHandle::Left,
                EdgeHandle::Bottom,
                EdgeHandle::Top,
                EdgeHandle::Right,
            ),
            // Target is to the left and up
            (HorizontalDirection::Left, VerticalDirection::Up) => (
                EdgeHandle::Left,
                EdgeHandle::Top,
                EdgeHandle::Bottom,
                EdgeHandle::Right,
            ),
            // Target is to the left
            (HorizontalDirection::Left, VerticalDirection::Neutral) => (
                EdgeHandle::Left,
                EdgeHandle::Top,
                EdgeHandle::Right,
                EdgeHandle::Bottom,
            ),

            // Target is directly below
            (HorizontalDirection::Neutral, VerticalDirection::Down) => (
                EdgeHandle::Bottom,
                EdgeHandle::Right,
                EdgeHandle::Top,
                EdgeHandle::Left,
            ),
            // Target is directly above
            (HorizontalDirection::Neutral, VerticalDirection::Up) => (
                EdgeHandle::Top,
                EdgeHandle::Right,
                EdgeHandle::Bottom,
                EdgeHandle::Left,
            ),
            // Target is overlapping
            (HorizontalDirection::Neutral, VerticalDirection::Neutral) => (
                EdgeHandle::Top,
                EdgeHandle::Right,
                EdgeHandle::Top,
                EdgeHandle::Bottom,
            ),
        }
    }

    /// Optimize positions within a constrained set (same row or column)
    fn optimize_within_constraint(
        &self,
        nodes: &mut [Node],
        constrained_nodes: &[Uuid],
        group_edges: &[&Edge],
        inter_edges: &[&Edge],
        subnet_positions: &HashMap<Uuid, Ixy>,
    ) {
        let mut improved = true;
        let mut iterations = 0;

        while improved && iterations < 10 {
            improved = false;
            iterations += 1;

            let current_length =
                self.utils
                    .calculate_edge_length(nodes, group_edges, subnet_positions);
            let node_map: HashMap<Uuid, Node> = nodes.iter().map(|n| (n.id, n.clone())).collect();
            let current_crossings = if !inter_edges.is_empty() {
                self.utils
                    .count_edge_crossings(inter_edges, &node_map, subnet_positions)
            } else {
                0
            };

            let mut best_swap: Option<(Uuid, Uuid, f64, usize)> = None;

            // Only try swaps within this constrained set
            for i in 0..constrained_nodes.len() {
                for j in (i + 1)..constrained_nodes.len() {
                    let node_a = constrained_nodes[i];
                    let node_b = constrained_nodes[j];

                    self.utils.swap_node_positions(nodes, node_a, node_b);

                    let node_map: HashMap<Uuid, Node> =
                        nodes.iter().map(|n| (n.id, n.clone())).collect();
                    let crossings_after = if !inter_edges.is_empty() {
                        self.utils
                            .count_edge_crossings(inter_edges, &node_map, subnet_positions)
                    } else {
                        0
                    };
                    let new_length =
                        self.utils
                            .calculate_edge_length(nodes, group_edges, subnet_positions);

                    // Accept if: fewer crossings OR (same crossings AND shorter length)
                    let should_accept = crossings_after < current_crossings
                        || (crossings_after == current_crossings && new_length < current_length);

                    if (should_accept && best_swap.is_none())
                        || (best_swap.is_some() && crossings_after < best_swap.unwrap().3)
                        || (best_swap.is_some()
                            && crossings_after == best_swap.unwrap().3
                            && new_length < best_swap.unwrap().2)
                    {
                        best_swap = Some((node_a, node_b, new_length, crossings_after));
                    }

                    self.utils.swap_node_positions(nodes, node_a, node_b);
                }
            }

            if let Some((node_a, node_b, _, _)) = best_swap {
                self.utils.swap_node_positions(nodes, node_a, node_b);
                improved = true;
            }
        }
    }
}
