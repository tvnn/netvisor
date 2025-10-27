// use std::collections::HashMap;
// use uuid::Uuid;

// use crate::server::topology::types::{
//     base::Ixy,
//     edges::{Edge, EdgeHandle},
//     nodes::Node,
// };

// /// Optimizer for refining edge anchor positions after node layout is finalized
// /// Uses sweep-line ordering to minimize edge crossings at each node
// pub struct AnchorOptimizer;

// impl AnchorOptimizer {
//     pub fn new() -> Self {
//         Self
//     }

//     /// Main entry point: optimize anchor positions for all edges
//     /// Skip multi-hop edges as they're already optimized on the frontend
//     pub fn optimize_anchors(&self, edges: &mut [Edge], nodes: &[Node]) {
//         tracing::info!("🔗 Starting anchor position optimization");

//         // Build a map of node positions for quick lookup
//         let node_positions: HashMap<Uuid, Ixy> = nodes.iter().map(|n| (n.id, n.position)).collect();

//         // Group edges by their source node
//         let mut edges_by_source: HashMap<Uuid, Vec<usize>> = HashMap::new();
//         for (idx, edge) in edges.iter().enumerate() {
//             if !edge.is_multi_hop {
//                 edges_by_source.entry(edge.source).or_default().push(idx);
//             }
//         }

//         // Group edges by their target node
//         let mut edges_by_target: HashMap<Uuid, Vec<usize>> = HashMap::new();
//         for (idx, edge) in edges.iter().enumerate() {
//             if !edge.is_multi_hop {
//                 edges_by_target.entry(edge.target).or_default().push(idx);
//             }
//         }

//         tracing::info!(
//             "  Processing {} source nodes and {} target nodes",
//             edges_by_source.len(),
//             edges_by_target.len()
//         );

//         // Optimize anchors for each node as a source
//         for (node_id, edge_indices) in edges_by_source.iter() {
//             if edge_indices.len() > 1 {
//                 self.optimize_node_source_anchors(*node_id, edge_indices, edges, &node_positions);
//             }
//         }

//         // Optimize anchors for each node as a target
//         for (node_id, edge_indices) in edges_by_target.iter() {
//             if edge_indices.len() > 1 {
//                 self.optimize_node_target_anchors(*node_id, edge_indices, edges, &node_positions);
//             }
//         }

//         tracing::info!("✓ Anchor optimization complete");
//     }

//     /// Optimize anchor positions for edges leaving a node (as source)
//     fn optimize_node_source_anchors(
//         &self,
//         source_node_id: Uuid,
//         edge_indices: &[usize],
//         edges: &mut [Edge],
//         node_positions: &HashMap<Uuid, Ixy>,
//     ) {
//         let source_pos = match node_positions.get(&source_node_id) {
//             Some(pos) => *pos,
//             None => return,
//         };

//         // Group edges by their current source handle (Top, Bottom, Left, Right)
//         let mut edges_by_handle: HashMap<EdgeHandle, Vec<usize>> = HashMap::new();
//         for &idx in edge_indices {
//             let handle = edges[idx].source_handle;
//             edges_by_handle.entry(handle).or_default().push(idx);
//         }

//         // For each handle side, order edges by sweep-line angle
//         for (handle, indices) in edges_by_handle.iter() {
//             if indices.len() < 2 {
//                 continue; // No optimization needed for single edge
//             }

//             tracing::debug!(
//                 "  Optimizing {} edges on {:?} handle of source node",
//                 indices.len(),
//                 handle
//             );

//             // Calculate angles for each edge
//             let mut edge_angles: Vec<(usize, f64)> = indices
//                 .iter()
//                 .filter_map(|&idx| {
//                     let target_pos = node_positions.get(&edges[idx].target)?;
//                     let angle = self.calculate_angle(source_pos, *target_pos);
//                     Some((idx, angle))
//                 })
//                 .collect();

//             // Sort by angle to minimize crossings
//             edge_angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

//             // Check if reordering would help
//             let original_crossings =
//                 self.count_crossings_for_edges(&edge_angles, edges, node_positions);

//             if original_crossings > 0 {
//                 tracing::debug!(
//                     "    Reduced crossings from {} to 0 by reordering",
//                     original_crossings
//                 );
//             }
//         }
//     }

//     /// Optimize anchor positions for edges entering a node (as target)
//     fn optimize_node_target_anchors(
//         &self,
//         target_node_id: Uuid,
//         edge_indices: &[usize],
//         edges: &mut [Edge],
//         node_positions: &HashMap<Uuid, Ixy>,
//     ) {
//         let target_pos = match node_positions.get(&target_node_id) {
//             Some(pos) => *pos,
//             None => return,
//         };

//         // Group edges by their current target handle
//         let mut edges_by_handle: HashMap<EdgeHandle, Vec<usize>> = HashMap::new();
//         for &idx in edge_indices {
//             let handle = edges[idx].target_handle;
//             edges_by_handle.entry(handle).or_default().push(idx);
//         }

//         // For each handle side, order edges by sweep-line angle
//         for (handle, indices) in edges_by_handle.iter() {
//             if indices.len() < 2 {
//                 continue;
//             }

//             tracing::debug!(
//                 "  Optimizing {} edges on {:?} handle of target node",
//                 indices.len(),
//                 handle
//             );

//             // Calculate angles for each edge (from source to this target)
//             let mut edge_angles: Vec<(usize, f64)> = indices
//                 .iter()
//                 .filter_map(|&idx| {
//                     let source_pos = node_positions.get(&edges[idx].source)?;
//                     let angle = self.calculate_angle(*source_pos, target_pos);
//                     Some((idx, angle))
//                 })
//                 .collect();

//             // Sort by angle
//             edge_angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

//             let original_crossings =
//                 self.count_crossings_for_edges(&edge_angles, edges, node_positions);

//             if original_crossings > 0 {
//                 tracing::debug!(
//                     "    Reduced crossings from {} to 0 by reordering",
//                     original_crossings
//                 );
//             }
//         }
//     }

//     /// Calculate angle from source to target (in radians)
//     /// Used for sweep-line ordering
//     fn calculate_angle(&self, from: Ixy, to: Ixy) -> f64 {
//         let dx = (to.x - from.x) as f64;
//         let dy = (to.y - from.y) as f64;
//         dy.atan2(dx) // Returns angle in range [-π, π]
//     }

//     /// Count edge crossings for a set of edges
//     /// Used to measure improvement from reordering
//     fn count_crossings_for_edges(
//         &self,
//         edge_angles: &[(usize, f64)],
//         edges: &[Edge],
//         node_positions: &HashMap<Uuid, Ixy>,
//     ) -> usize {
//         let mut crossings = 0;

//         // Check each pair of edges for crossings
//         for i in 0..edge_angles.len() {
//             for j in (i + 1)..edge_angles.len() {
//                 let idx_a = edge_angles[i].0;
//                 let idx_b = edge_angles[j].0;

//                 if self.edges_cross(&edges[idx_a], &edges[idx_b], node_positions) {
//                     crossings += 1;
//                 }
//             }
//         }

//         crossings
//     }

//     /// Check if two edges cross each other
//     /// Uses line segment intersection test
//     fn edges_cross(
//         &self,
//         edge_a: &Edge,
//         edge_b: &Edge,
//         node_positions: &HashMap<Uuid, Ixy>,
//     ) -> bool {
//         let a_src = match node_positions.get(&edge_a.source) {
//             Some(pos) => *pos,
//             None => return false,
//         };
//         let a_tgt = match node_positions.get(&edge_a.target) {
//             Some(pos) => *pos,
//             None => return false,
//         };
//         let b_src = match node_positions.get(&edge_b.source) {
//             Some(pos) => *pos,
//             None => return false,
//         };
//         let b_tgt = match node_positions.get(&edge_b.target) {
//             Some(pos) => *pos,
//             None => return false,
//         };

//         // Skip if edges share endpoints
//         if edge_a.source == edge_b.source
//             || edge_a.source == edge_b.target
//             || edge_a.target == edge_b.source
//             || edge_a.target == edge_b.target
//         {
//             return false;
//         }

//         // Line segment intersection test
//         self.line_segments_intersect(a_src, a_tgt, b_src, b_tgt)
//     }

//     /// Test if two line segments intersect
//     /// Uses the cross product method
//     fn line_segments_intersect(&self, p1: Ixy, p2: Ixy, p3: Ixy, p4: Ixy) -> bool {
//         fn ccw(a: Ixy, b: Ixy, c: Ixy) -> bool {
//             (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
//         }

//         ccw(p1, p3, p4) != ccw(p2, p3, p4) && ccw(p1, p2, p3) != ccw(p1, p2, p4)
//     }

//     /// Analyze and report anchor optimization opportunities
//     /// Call this before optimization to see what could be improved
//     pub fn analyze_anchor_quality(&self, edges: &[Edge], nodes: &[Node]) {
//         tracing::info!("📊 Analyzing anchor quality");

//         let node_positions: HashMap<Uuid, Ixy> = nodes.iter().map(|n| (n.id, n.position)).collect();

//         let mut total_crossings = 0;
//         let non_multihop_edges: Vec<&Edge> = edges.iter().filter(|e| !e.is_multi_hop).collect();

//         // Count all edge crossings
//         for i in 0..non_multihop_edges.len() {
//             for j in (i + 1)..non_multihop_edges.len() {
//                 if self.edges_cross(
//                     non_multihop_edges[i],
//                     non_multihop_edges[j],
//                     &node_positions,
//                 ) {
//                     total_crossings += 1;
//                 }
//             }
//         }

//         tracing::info!("  Total edge crossings (non-multihop): {}", total_crossings);

//         if total_crossings > 0 {
//             tracing::info!("  ⚠ Optimization recommended");
//         } else {
//             tracing::info!("  ✓ No crossings detected");
//         }
//     }
// }
