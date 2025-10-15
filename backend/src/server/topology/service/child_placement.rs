use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::{
    service::context::TopologyContext,
    types::{
        base::{Ixy, NodeLayout, Uxy},
        nodes::SubnetChild,
    },
};

pub struct ChildNodePlacement;

impl ChildNodePlacement {
    /// Calculate child positions using anchor-based layout within a subnet
    /// Layout uses Top/Bottom/Left/Right sections with center for no-handle nodes
    pub fn calculate_anchor_based_positions(
        children: &[SubnetChild],
        container_grid: &Uxy,
        ctx: &TopologyContext,
    ) -> Vec<Vec<(Uuid, NodeLayout)>> {
        if children.is_empty() {
            return vec![Vec::new(); container_grid.y];
        }

        let grid_w = container_grid.x;
        let grid_h = container_grid.y;

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

        // Find force extremes ---
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

        // Normalize to [0,1] space ---
        let normalize = |v: isize, min: f32, max: f32| {
            if (max - min).abs() < f32::EPSILON {
                0.5 // avoid division by zero; center everything
            } else {
                (v as f32 - min) / (max - min)
            }
        };

        // Build grid ---
        let mut grid = vec![vec![None; grid_w]; grid_h];
        let mut placed = HashMap::new();

        // Fill stronger directional children first
        let mut sorted = force_directed_children.to_vec();
        sorted.sort_by(|(_, a), (_, b)| {
            b.x.abs()
                .max(b.y.abs())
                .partial_cmp(&a.x.abs().max(a.y.abs()))
                .unwrap()
        });

        for (child, f) in sorted {
            // --- 4️⃣ Map to grid coordinates separately for width & height ---
            let gx = (normalize(f.x, min_x, max_x) * (grid_w as f32 - 1.0)).round() as isize;
            let gy = (normalize(f.y, min_y, max_y) * (grid_h as f32 - 1.0)).round() as isize;

            // Flip Y so positive Y means “up” visually
            let gx = gx.clamp(0, (grid_w - 1) as isize);
            let gy = ((grid_h - 1) as isize - gy).clamp(0, (grid_h - 1) as isize);

            // --- 5️⃣ Find nearest available slot ---
            let mut found_slot = None;
            let mut radius = 0;
            while found_slot.is_none() && radius < grid_w.max(grid_h) as isize {
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let nx = gx + dx;
                        let ny = gy + dy;
                        if nx >= 0
                            && ny >= 0
                            && nx < grid_w as isize
                            && ny < grid_h as isize
                            && grid[ny as usize][nx as usize].is_none()
                        {
                            found_slot = Some((nx as usize, ny as usize));
                            break;
                        }
                    }
                    if found_slot.is_some() {
                        break;
                    }
                }
                radius += 1;
            }

            if let Some((x, y)) = found_slot {
                grid[y][x] = Some(child.clone());
                placed.insert(child.id, (x, y));
            }
        }

        // Convert grid to the expected output format
        let mut rows: Vec<Vec<(Uuid, NodeLayout)>> = vec![Vec::new(); grid_h];
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Some(child) = cell {
                    rows[row_idx].push((
                        child.id,
                        NodeLayout {
                            size: child.size,
                            grid_position: Uxy {
                                x: col_idx,
                                y: row_idx,
                            },
                        },
                    ));
                }
            }
        }

        rows
    }
}
