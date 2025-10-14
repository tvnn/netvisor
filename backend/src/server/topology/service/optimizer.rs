use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::{
    service::{crossing_reducer::CrossingReducer, subnet_positioner::SubnetPositioner},
    types::{
        base::Ixy,
        edges::{Edge, EdgeHandle},
        nodes::{Node, NodeType},
    },
};

/// High-level optimizer that coordinates crossing reduction and subnet positioning
pub struct TopologyOptimizer {
    crossing_reducer: CrossingReducer,
    subnet_positioner: SubnetPositioner,
}

impl Default for TopologyOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl TopologyOptimizer {
    pub fn new() -> Self {
        Self {
            crossing_reducer: CrossingReducer::new(),
            subnet_positioner: SubnetPositioner::new(),
        }
    }

    /// Main entry point: optimize node positions and fix edge handles
    pub fn optimize_graph(
        &self,
        nodes: &mut [Node],
        edges: Vec<Edge>,
        relocation_map: &HashMap<Uuid, EdgeHandle>,
    ) -> Vec<Edge> {
        // Step 1: Relocate handles for nodes with opposing vertical edges
        let mut edges = Self::relocate_edge_handles(edges, relocation_map);

        // Step 2: Propagate relocated handles to connected nodes (respecting infra constraints)
        edges = Self::propagate_handle_relocation(&edges, relocation_map, nodes);

        // Step 3: Order nodes within subnets based on edge flow (topological ordering)
        self.order_nodes_by_flow(nodes, &edges);

        // Step 4: Reduce edge crossings within subnets
        self.reduce_edge_crossings(nodes, &edges);

        // Step 5: Optimize intra-subnet node positions to reduce edge lengths
        self.optimize_intra_subnet_positions(nodes, &edges);

        // Step 6: Fix intra-subnet edge handles based on actual positions
        Self::fix_intra_subnet_handles(edges, nodes)
    }

    /// Relocate edge handles for nodes that have opposing vertical edges
    /// This moves Top/Bottom handles to the side where the node is placed
    fn relocate_edge_handles(
        edges: Vec<Edge>,
        relocation_map: &HashMap<Uuid, EdgeHandle>,
    ) -> Vec<Edge> {
        edges
            .into_iter()
            .map(|mut edge| {
                // Check if source needs relocation
                if let Some(new_handle) = relocation_map.get(&edge.source) {
                    // If the edge has a vertical handle (Top/Bottom), relocate it to the side
                    if matches!(edge.source_handle, EdgeHandle::Top | EdgeHandle::Bottom) {
                        edge.source_handle = *new_handle;
                    }
                }

                // Check if target needs relocation
                if let Some(new_handle) = relocation_map.get(&edge.target) {
                    if matches!(edge.target_handle, EdgeHandle::Top | EdgeHandle::Bottom) {
                        edge.target_handle = *new_handle;
                    }
                }

                edge
            })
            .collect()
    }

    /// Choose Top or Bottom handle based on the relative positions of source and target subnets
    /// from the perspective of the node that will use this handle
    fn choose_vertical_handle_by_subnet_position(
        node_id: Uuid,
        other_node_id: Uuid,
        nodes: &[Node],
    ) -> EdgeHandle {
        // Get subnet positions
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        // Find the subnets for both nodes
        let node_subnet = Self::get_node_subnet(node_id, nodes);
        let other_subnet = Self::get_node_subnet(other_node_id, nodes);

        if let (Some(node_sub), Some(other_sub)) = (node_subnet, other_subnet) {
            if let (Some(node_pos), Some(other_pos)) = (
                subnet_positions.get(&node_sub),
                subnet_positions.get(&other_sub),
            ) {
                // From this node's perspective:
                // If the other node is below us, we use Bottom handle
                // If the other node is above us, we use Top handle
                if other_pos.y > node_pos.y {
                    return EdgeHandle::Bottom;
                } else {
                    return EdgeHandle::Top;
                }
            }
        }

        // Fallback to Top if we can't determine positions
        EdgeHandle::Top
    }

    /// Apply infra constraints to a handle, considering subnet positions
    /// - Infra nodes cannot use Right handle
    /// - Non-infra nodes cannot use Left handle
    /// - Choose Top/Bottom based on actual subnet positions FROM THE NODE'S PERSPECTIVE
    fn apply_infra_constraint(
        handle: EdgeHandle,
        is_infra: bool,
        node_id: Uuid,
        other_node_id: Uuid,
        nodes: &[Node],
    ) -> EdgeHandle {
        match (&handle, is_infra) {
            // Infra node trying to use Right -> use vertical based on subnet positions
            (EdgeHandle::Right, true) => {
                Self::choose_vertical_handle_by_subnet_position(node_id, other_node_id, nodes)
            }
            // Non-infra node trying to use Left -> use vertical based on subnet positions
            (EdgeHandle::Left, false) => {
                Self::choose_vertical_handle_by_subnet_position(node_id, other_node_id, nodes)
            }
            // All other cases are fine
            _ => handle,
        }
    }

    /// Propagate handle relocation to connected nodes
    /// If a node was relocated to the side, update connected nodes' handles to match
    /// WHILE respecting infra constraints and subnet positions
    fn propagate_handle_relocation(
        edges: &[Edge],
        relocation_map: &HashMap<Uuid, EdgeHandle>,
        nodes: &[Node],
    ) -> Vec<Edge> {
        // Build a map of node ID to infra status
        let node_infra_status: HashMap<Uuid, bool> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::HostNode { is_infra, .. } => Some((n.id, is_infra)),
                _ => None,
            })
            .collect();

        edges
            .iter()
            .map(|edge| {
                let mut new_edge = edge.clone();

                // Get infra status for both source and target
                let source_is_infra = node_infra_status
                    .get(&edge.source)
                    .copied()
                    .unwrap_or(false);
                let target_is_infra = node_infra_status
                    .get(&edge.target)
                    .copied()
                    .unwrap_or(false);

                // If source was relocated, update target handle to match (respecting constraints)
                // IMPORTANT: Pass target_id first, then source_id, because we're determining
                // the handle FROM THE TARGET'S PERSPECTIVE
                if let Some(relocated_handle) = relocation_map.get(&edge.source) {
                    new_edge.target_handle = Self::apply_infra_constraint(
                        *relocated_handle,
                        target_is_infra,
                        edge.target, // The node that will use this handle
                        edge.source, // The other end of the edge
                        nodes,
                    );
                }

                // If target was relocated, update source handle to match (respecting constraints)
                // IMPORTANT: Pass source_id first, then target_id, because we're determining
                // the handle FROM THE SOURCE'S PERSPECTIVE
                if let Some(relocated_handle) = relocation_map.get(&edge.target) {
                    new_edge.source_handle = Self::apply_infra_constraint(
                        *relocated_handle,
                        source_is_infra,
                        edge.source, // The node that will use this handle
                        edge.target, // The other end of the edge
                        nodes,
                    );
                }

                new_edge
            })
            .collect()
    }

    /// Order nodes within subnets based on edge flow using topological sorting
    fn order_nodes_by_flow(&self, nodes: &mut [Node], edges: &[Edge]) {
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

    /// Optimize positions of nodes within subnets to reduce intra-subnet edge lengths
    fn optimize_intra_subnet_positions(&self, nodes: &mut [Node], edges: &[Edge]) {
        // Get subnet positions for reference
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

        // Find all intra-subnet edges
        let intra_edges: Vec<&Edge> = edges
            .iter()
            .filter(|edge| {
                let source_subnet = Self::get_node_subnet(edge.source, nodes);
                let target_subnet = Self::get_node_subnet(edge.target, nodes);
                source_subnet.is_some() && source_subnet == target_subnet
            })
            .collect();

        if intra_edges.is_empty() {
            return;
        }

        // Group ALL nodes by subnet and infra status (not just those with intra edges)
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

        // For each group, try to reduce edge lengths by repositioning
        for ((_subnet_id, _is_infra), node_ids) in nodes_by_subnet_infra.iter() {
            if node_ids.len() < 2 {
                continue;
            }

            // Get ALL edges involving nodes in this group (not just both endpoints)
            let group_edges: Vec<&Edge> = intra_edges
                .iter()
                .filter(|e| node_ids.contains(&e.source) || node_ids.contains(&e.target))
                .copied()
                .collect();

            if group_edges.is_empty() {
                continue;
            }

            // Try swapping ANY two nodes to reduce edge lengths
            let mut improved = true;
            let mut iterations = 0;

            while improved && iterations < 10 {
                improved = false;
                iterations += 1;

                let current_length =
                    Self::calculate_intra_group_edge_length(nodes, &group_edges, &subnet_positions);

                // Try all possible pairs of swaps
                let mut best_swap: Option<(Uuid, Uuid, f64)> = None;

                for i in 0..node_ids.len() {
                    for j in (i + 1)..node_ids.len() {
                        let node_a = node_ids[i];
                        let node_b = node_ids[j];

                        // Swap positions
                        Self::swap_node_positions_by_id(nodes, node_a, node_b);

                        let new_length = Self::calculate_intra_group_edge_length(
                            nodes,
                            &group_edges,
                            &subnet_positions,
                        );

                        if new_length < current_length {
                            // Track best improvement
                            if best_swap.is_none() || new_length < best_swap.unwrap().2 {
                                best_swap = Some((node_a, node_b, new_length));
                            }
                        }

                        // Always revert to try next pair
                        Self::swap_node_positions_by_id(nodes, node_a, node_b);
                    }
                }

                // Apply best swap if found
                if let Some((node_a, node_b, _new_length)) = best_swap {
                    Self::swap_node_positions_by_id(nodes, node_a, node_b);
                    improved = true;
                }
            }
        }
    }

    /// Calculate total edge length for intra-group edges
    fn calculate_intra_group_edge_length(
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
                let src_pos = Self::get_absolute_position(src_node, subnet_positions);
                let tgt_pos = Self::get_absolute_position(tgt_node, subnet_positions);

                let dx = (tgt_pos.x - src_pos.x) as f64;
                let dy = (tgt_pos.y - src_pos.y) as f64;
                total_length += (dx * dx + dy * dy).sqrt();
            }
        }

        total_length
    }

    /// Get absolute position of a node (including subnet offset)
    fn get_absolute_position(node: &Node, subnet_positions: &HashMap<Uuid, Ixy>) -> Ixy {
        let mut pos = node.position;

        if let NodeType::HostNode { subnet_id, .. } = node.node_type {
            if let Some(subnet_pos) = subnet_positions.get(&subnet_id) {
                pos.x += subnet_pos.x;
                pos.y += subnet_pos.y;
            }
        }

        pos
    }

    /// Swap positions of two nodes by their IDs
    fn swap_node_positions_by_id(nodes: &mut [Node], node_a: Uuid, node_b: Uuid) {
        let mut pos_a = None;
        let mut pos_b = None;

        for node in nodes.iter() {
            if node.id == node_a {
                pos_a = Some(node.position);
            } else if node.id == node_b {
                pos_b = Some(node.position);
            }
        }

        if let (Some(pa), Some(pb)) = (pos_a, pos_b) {
            for node in nodes.iter_mut() {
                if node.id == node_a {
                    node.position = pb;
                } else if node.id == node_b {
                    node.position = pa;
                }
            }
        }
    }

    /// Main entry point: optimize node positions to reduce edge crossings
    pub fn reduce_edge_crossings(&self, nodes: &mut [Node], edges: &[Edge]) {
        let subnet_positions: HashMap<Uuid, Ixy> = nodes
            .iter()
            .filter_map(|n| match n.node_type {
                NodeType::SubnetNode { .. } => Some((n.id, n.position)),
                _ => None,
            })
            .collect();

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

        // Step 1: Optimize node positions within subnets to reduce edge crossings
        self.crossing_reducer
            .reduce_crossings(nodes, edges, &subnet_positions);

        // Step 2: Optimize subnet positions to minimize edge lengths
        self.subnet_positioner
            .optimize_positions(nodes, &inter_subnet_edges);
    }

    /// Fix intra-subnet edge handles based on actual node positions
    fn fix_intra_subnet_handles(edges: Vec<Edge>, nodes: &[Node]) -> Vec<Edge> {
        // Build a map of node ID to position
        let node_positions: HashMap<Uuid, (Ixy, Uuid, bool)> = nodes
            .iter()
            .filter_map(|n| match &n.node_type {
                NodeType::HostNode {
                    subnet_id,
                    is_infra,
                    ..
                } => Some((n.id, (n.position, *subnet_id, *is_infra))),
                _ => None,
            })
            .collect();

        edges
            .into_iter()
            .map(|mut edge| {
                // Check if this is an intra-subnet edge
                if let (
                    Some((src_pos, src_subnet, src_is_infra)),
                    Some((tgt_pos, tgt_subnet, _tgt_is_infra)),
                ) = (
                    node_positions.get(&edge.source),
                    node_positions.get(&edge.target),
                ) {
                    if src_subnet == tgt_subnet {
                        // This is an intra-subnet edge - adjust handles based on relative position
                        let dx = tgt_pos.x - src_pos.x;
                        let dy = tgt_pos.y - src_pos.y;

                        // Determine primary direction
                        if dx.abs() > dy.abs() {
                            // Horizontal flow is dominant
                            if dx > 0 {
                                // Target is to the right
                                edge.source_handle = if *src_is_infra && dx.abs() < 100 {
                                    EdgeHandle::Bottom // Very close, use bottom to avoid overlap
                                } else {
                                    EdgeHandle::Right
                                };
                                edge.target_handle = EdgeHandle::Left;
                            } else {
                                // Target is to the left
                                edge.source_handle = EdgeHandle::Left;
                                edge.target_handle = EdgeHandle::Right;
                            }
                        } else {
                            // Vertical flow is dominant
                            if dy > 0 {
                                // Target is below
                                edge.source_handle = EdgeHandle::Bottom;
                                edge.target_handle = EdgeHandle::Top;
                            } else {
                                // Target is above
                                edge.source_handle = EdgeHandle::Top;
                                edge.target_handle = EdgeHandle::Bottom;
                            }
                        }
                    }
                }

                edge
            })
            .collect()
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
