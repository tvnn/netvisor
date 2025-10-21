use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::server::topology::{
    service::{
        context::TopologyContext, optimizer::utils::OptimizerUtils,
        subnet_layout_planner::NODE_PADDING,
    },
    types::{
        base::{Ixy, NodeBounds},
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

    pub fn resolve_overlaps_in_subnet(&self, nodes: &mut [Node], subnet_id: Uuid, is_infra: bool) {
        let mut subnet_nodes: Vec<&mut Node> = nodes
            .iter_mut()
            .filter(|n| match n.node_type {
                NodeType::HostNode {
                    subnet_id: sid,
                    is_infra: infra,
                    ..
                } => sid == subnet_id && infra == is_infra,
                _ => false,
            })
            .collect();

        println!(
            "Resolving overlaps for subnet {:?}, infra: {}, node count: {}",
            subnet_id,
            is_infra,
            subnet_nodes.len()
        );

        if subnet_nodes.len() < 2 {
            return;
        }

        const MAX_RESOLUTION_ITERATIONS: usize = 50;
        let mut iteration = 0;

        while iteration < MAX_RESOLUTION_ITERATIONS {
            let mut had_overlap = false;

            let mut node_bounds: Vec<(Uuid, NodeBounds)> = subnet_nodes
                .iter()
                .map(|n| (n.id, NodeBounds::new(n.position, n.size)))
                .collect();

            for i in 0..node_bounds.len() {
                for j in (i + 1)..node_bounds.len() {
                    let (id_a, bounds_a) = node_bounds[i];
                    let (id_b, bounds_b) = node_bounds[j];

                    if bounds_a.overlaps(&bounds_b) {
                        had_overlap = true;

                        let (dx, dy) = bounds_a.resolve_overlap(&bounds_b);

                        // Determine which node has more room to move
                        // Get subnet bounds from the first node in the group
                        let _subnet_bounds = if let Some(_first_node) = subnet_nodes.first() {
                            // Assuming subnet starts at a reasonable origin, calculate available space
                            // For vertical movement, check distance from top/bottom edges
                            let space_above_a = bounds_a.y;
                            let space_above_b = bounds_b.y;

                            // Move the node that has more space in the direction we need to move
                            if dy != 0 {
                                // If dy is negative (move up), prefer moving the node with more space above
                                // If dy is positive (move down), prefer moving the node with more space below
                                if dy < 0 {
                                    // Need to move up - move the node with more space above
                                    if space_above_a > space_above_b {
                                        if let Some(node_a) =
                                            subnet_nodes.iter_mut().find(|n| n.id == id_a)
                                        {
                                            node_a.position.y += dy;
                                        }
                                    } else if let Some(node_b) =
                                        subnet_nodes.iter_mut().find(|n| n.id == id_b)
                                    {
                                        node_b.position.y -= dy; // Opposite direction
                                    }
                                } else {
                                    // Need to move down - for now just move node_b
                                    if let Some(node_b) =
                                        subnet_nodes.iter_mut().find(|n| n.id == id_b)
                                    {
                                        node_b.position.y += dy;
                                    }
                                }
                            }

                            if dx != 0 {
                                // Similar logic for horizontal
                                let space_left_a = bounds_a.x;
                                let space_left_b = bounds_b.x;

                                if dx < 0 {
                                    if space_left_a > space_left_b {
                                        if let Some(node_a) =
                                            subnet_nodes.iter_mut().find(|n| n.id == id_a)
                                        {
                                            node_a.position.x += dx;
                                        }
                                    } else if let Some(node_b) =
                                        subnet_nodes.iter_mut().find(|n| n.id == id_b)
                                    {
                                        node_b.position.x -= dx;
                                    }
                                } else if let Some(node_b) =
                                    subnet_nodes.iter_mut().find(|n| n.id == id_b)
                                {
                                    node_b.position.x += dx;
                                }
                            }

                            true
                        } else {
                            false
                        };

                        let node_a = subnet_nodes.iter().find(|n| n.id == id_a);
                        let node_b = subnet_nodes.iter().find(|n| n.id == id_b);

                        // Update bounds
                        node_bounds[i].1 = NodeBounds::new(
                            node_a.map(|n| n.position).unwrap_or_default(),
                            node_a.map(|n| n.size).unwrap_or_default(),
                        );
                        node_bounds[j].1 = NodeBounds::new(
                            node_b.map(|n| n.position).unwrap_or_default(),
                            node_b.map(|n| n.size).unwrap_or_default(),
                        );
                    }
                }
            }

            if !had_overlap {
                println!("  No more overlaps found after {} iterations", iteration);
                break;
            }

            iteration += 1;
        }

        if iteration >= MAX_RESOLUTION_ITERATIONS {
            println!("  WARNING: Hit max iterations!");
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

        let inter_edges: Vec<&Edge> = edges
            .iter()
            .filter(|edge| !self.context.edge_is_intra_subnet(edge))
            .collect();

        // Group nodes by (subnet, infra)
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

        // For each subnet+infra group, optimize
        for ((subnet_id, is_infra), node_ids) in nodes_by_subnet_infra.iter() {
            if node_ids.len() < 2 {
                continue;
            }

            // Get edges for this group
            let group_edges: Vec<&Edge> = edges
                .iter()
                .filter(|e| node_ids.contains(&e.source) || node_ids.contains(&e.target))
                .collect();

            if !group_edges.is_empty() {
                // Group nodes by row and column
                let mut nodes_by_row: HashMap<isize, Vec<Uuid>> = HashMap::new();
                let mut nodes_by_col: HashMap<isize, Vec<Uuid>> = HashMap::new();

                for &node_id in node_ids {
                    if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                        // Determine if node can be swapped based on edge handles
                        let node_edge_handles: Vec<EdgeHandle> = edges
                            .iter()
                            .filter_map(|e| {
                                if node_id == e.source {
                                    Some(e.source_handle)
                                } else if node_id == e.target {
                                    Some(e.target_handle)
                                } else {
                                    None
                                }
                            })
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

                // Optimize within rows
                for row_nodes in nodes_by_row.values() {
                    if row_nodes.len() > 1 {
                        self.optimize_within_constraint(
                            nodes,
                            row_nodes,
                            &group_edges,
                            &inter_edges,
                            &subnet_positions,
                        );
                    }
                }

                // Optimize within columns
                for col_nodes in nodes_by_col.values() {
                    if col_nodes.len() > 1 {
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
            // After all swaps, reduce padding
            self.compress_to_minimum_spacing(nodes, *subnet_id, *is_infra);
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

            // Try all possible swaps within this constrained set
            for i in 0..constrained_nodes.len() {
                for j in (i + 1)..constrained_nodes.len() {
                    let node_a = constrained_nodes[i];
                    let node_b = constrained_nodes[j];

                    // Perform the swap
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

                    match best_swap {
                        Some(swap) => {
                            if crossings_after < swap.3
                                || (crossings_after == swap.3 && new_length < swap.2)
                            {
                                best_swap = Some((node_a, node_b, new_length, crossings_after))
                            }
                        }
                        None => {
                            if should_accept {
                                best_swap = Some((node_a, node_b, new_length, crossings_after))
                            }
                        }
                    }

                    // Revert the swap
                    self.utils.swap_node_positions(nodes, node_a, node_b);
                }
            }

            // Apply the best swap if found
            if let Some((node_a, node_b, _, _)) = best_swap {
                self.utils.swap_node_positions(nodes, node_a, node_b);
                improved = true;
            }
        }
    }

    /// Compress nodes to minimum spacing based on spatial neighbors
    pub fn compress_to_minimum_spacing(&self, nodes: &mut [Node], subnet_id: Uuid, is_infra: bool) {
        // Get the subnet's infra width BEFORE creating mutable borrow
        let infra_width = if !is_infra {
            nodes
                .iter()
                .find(|n| {
                    if let NodeType::SubnetNode { .. } = n.node_type {
                        n.id == subnet_id
                    } else {
                        false
                    }
                })
                .and_then(|subnet_node| {
                    if let NodeType::SubnetNode { infra_width, .. } = subnet_node.node_type {
                        Some(infra_width as isize)
                    } else {
                        None
                    }
                })
                .unwrap_or(0)
        } else {
            0
        };

        let start_x = if is_infra {
            NODE_PADDING.x as isize
        } else {
            infra_width + NODE_PADDING.x as isize
        };

        let mut subnet_nodes: Vec<&mut Node> = nodes
            .iter_mut()
            .filter(|n| match n.node_type {
                NodeType::HostNode {
                    subnet_id: sid,
                    is_infra: infra,
                    ..
                } => sid == subnet_id && infra == is_infra,
                _ => false,
            })
            .collect();

        if subnet_nodes.len() < 2 {
            return;
        }

        // STEP 1: Compress horizontally by rows
        // Group nodes by Y position (rows)
        let mut rows: HashMap<isize, Vec<usize>> = HashMap::new();
        for (i, _) in subnet_nodes.iter().enumerate() {
            let y = subnet_nodes[i].position.y;
            rows.entry(y).or_default().push(i);
        }

        // For each row, compress horizontally
        for row_indices in rows.values() {
            if row_indices.len() < 2 {
                continue;
            }

            // Sort nodes in row by X position
            let mut sorted_indices = row_indices.clone();
            sorted_indices.sort_by_key(|&idx| subnet_nodes[idx].position.x);

            // Compress: set first node to start_x, then pack subsequent nodes
            let mut current_x = start_x;
            for &idx in &sorted_indices {
                subnet_nodes[idx].position.x = current_x;
                current_x += subnet_nodes[idx].size.x as isize + NODE_PADDING.x as isize;
            }
        }

        // STEP 2: Compress vertically by columns
        // Group nodes by X position (columns) - use updated X positions from step 1
        let mut columns: HashMap<isize, Vec<usize>> = HashMap::new();
        for (i, _) in subnet_nodes.iter().enumerate() {
            let x = subnet_nodes[i].position.x;
            columns.entry(x).or_default().push(i);
        }

        // For each column, compress vertically
        for col_indices in columns.values() {
            if col_indices.len() < 2 {
                continue;
            }

            // Sort nodes in column by Y position
            let mut sorted_indices = col_indices.clone();
            sorted_indices.sort_by_key(|&idx| subnet_nodes[idx].position.y);

            // Compress: set first node to NODE_PADDING.y, then pack subsequent nodes
            let mut current_y = NODE_PADDING.y as isize;
            for &idx in &sorted_indices {
                subnet_nodes[idx].position.y = current_y;
                current_y += subnet_nodes[idx].size.y as isize + NODE_PADDING.y as isize;
            }
        }
    }
}
