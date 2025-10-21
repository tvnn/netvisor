use std::{cmp::Ordering, collections::HashMap};
use uuid::Uuid;

use crate::server::topology::{
    service::context::TopologyContext,
    types::{
        base::{Ixy, NodeBounds, NodeLayout, Uxy},
        nodes::SubnetChild,
    },
};

pub struct ChildNodePlacement;

impl ChildNodePlacement {
    /// Calculate child positions using grid-based layout with continuous coordinates
    /// Places nodes in a grid pattern but uses actual pixel positions for overlap resolution
    pub fn calculate_anchor_based_positions(
        children: &[SubnetChild],
        padding: &Uxy,
        ctx: &TopologyContext,
    ) -> HashMap<Uuid, NodeLayout> {
        if children.is_empty() {
            return HashMap::new();
        }

        // Calculate grid dimensions (same as before)
        let grid_w = ((children.len() as f64).sqrt().ceil() as usize).max(1);
        let grid_h = ((children.len() as f64 / grid_w as f64).ceil() as usize).max(1);

        // Create force directed map of subnet children
        let force_directed_children: Vec<_> = children
            .iter()
            .map(|c| {
                let force_direction = c.edges.iter().fold(Ixy::default(), |mut acc, e| {
                    if !ctx.edge_is_intra_subnet(e) {
                        if c.interface_id == Some(e.source) {
                            acc.x += e.source_handle.direction().x;
                            acc.y += e.source_handle.direction().y;
                        } else if c.interface_id == Some(e.target) {
                            acc.x += e.target_handle.direction().x;
                            acc.y += e.target_handle.direction().y;
                        }
                    }
                    acc
                });

                (c, force_direction)
            })
            .collect();

        // Find force extremes
        let (min_x, max_x) = force_directed_children
            .iter()
            .map(|(_, f)| f.x)
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), x| {
                (min.min(x as f32), max.max(x as f32))
            });
        let (min_y, max_y) = force_directed_children
            .iter()
            .map(|(_, f)| f.y)
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), y| {
                (min.min(y as f32), max.max(y as f32))
            });

        // Normalize to [0,1] space
        let normalize = |v: isize, min: f32, max: f32| {
            if (max - min).abs() < f32::EPSILON {
                0.5
            } else {
                (v as f32 - min) / (max - min)
            }
        };

        // Sort children by force strength (stronger forces get placed first)
        let mut sorted = force_directed_children.to_vec();
        sorted.sort_by(|(_, a), (_, b)| {
            b.x.abs()
                .max(b.y.abs())
                .partial_cmp(&a.x.abs().max(a.y.abs()))
                .unwrap_or(Ordering::Equal)
        });

        // Create a grid to track which cells are occupied
        let mut grid: Vec<Vec<Option<SubnetChild>>> = vec![vec![None; grid_w]; grid_h];
        let mut placed_positions: HashMap<Uuid, (usize, usize)> = HashMap::new();

        // Place nodes in grid cells based on force direction
        for (child, force) in sorted {
            let norm_x = normalize(force.x, min_x, max_x);
            let norm_y = normalize(force.y, min_y, max_y);

            // Map to grid coordinates
            let ideal_gx = (norm_x * (grid_w as f32 - 1.0)).round() as isize;
            let ideal_gy = ((1.0 - norm_y) * (grid_h as f32 - 1.0)).round() as isize;

            let ideal_gx = ideal_gx.clamp(0, (grid_w - 1) as isize) as usize;
            let ideal_gy = ideal_gy.clamp(0, (grid_h - 1) as isize) as usize;

            // Find nearest available grid cell
            let mut found_slot = None;
            let mut radius = 0;
            while found_slot.is_none() && radius < grid_w.max(grid_h) {
                for dy in -(radius as isize)..=(radius as isize) {
                    for dx in -(radius as isize)..=(radius as isize) {
                        let gx = (ideal_gx as isize + dx).clamp(0, (grid_w - 1) as isize) as usize;
                        let gy = (ideal_gy as isize + dy).clamp(0, (grid_h - 1) as isize) as usize;

                        if grid[gy][gx].is_none() {
                            found_slot = Some((gx, gy));
                            break;
                        }
                    }
                    if found_slot.is_some() {
                        break;
                    }
                }
                radius += 1;
            }

            if let Some((gx, gy)) = found_slot {
                grid[gy][gx] = Some((*child).clone());
                placed_positions.insert(child.id, (gx, gy));
            }
        }

        // Convert grid positions to actual pixel coordinates with overlap resolution
        let mut result: HashMap<Uuid, NodeLayout> = HashMap::new();
        let mut placed_nodes: Vec<(Uuid, NodeBounds)> = Vec::new();

        // FIXED: Calculate actual positions row by row for tighter packing
        let mut row_heights: Vec<usize> = vec![0; grid_h];
        let mut col_widths: Vec<usize> = vec![0; grid_w];

        // First pass: calculate maximum width/height for each row/column
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Some(child) = cell {
                    row_heights[row_idx] = row_heights[row_idx].max(child.size.y);
                    col_widths[col_idx] = col_widths[col_idx].max(child.size.x);
                }
            }
        }

        // Second pass: place nodes using cumulative positions
        let mut current_y = padding.y as isize;
        for (row_idx, row) in grid.iter().enumerate() {
            let mut current_x = padding.x as isize;

            for (col_idx, cell) in row.iter().enumerate() {
                if let Some(child) = cell {
                    let position = Ixy {
                        x: current_x,
                        y: current_y,
                    };

                    // No overlap resolution needed - grid spacing handles it
                    let final_bounds = NodeBounds::new(position, child.size);
                    placed_nodes.push((child.id, final_bounds));

                    result.insert(
                        child.id,
                        NodeLayout {
                            size: child.size,
                            position,
                        },
                    );
                }

                current_x += col_widths[col_idx] as isize + padding.x as isize;
            }

            current_y += row_heights[row_idx] as isize + padding.y as isize;
        }
        result
    }
}
