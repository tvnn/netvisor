use itertools::Itertools;
use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::types::{
    base::{NodeLayout, Uxy},
    edges::EdgeHandle,
    nodes::SubnetChild,
};

pub struct ChildNodePlacement;

impl ChildNodePlacement {
    /// Calculate child positions using anchor-based layout within a subnet
    /// Layout uses Top/Bottom/Left/Right sections with center for no-handle nodes
    pub fn calculate_anchor_based_positions(
        children: &[SubnetChild],
        container_grid: &Uxy,
    ) -> Vec<Vec<(Uuid, NodeLayout)>> {
        if children.is_empty() {
            return vec![Vec::new(); container_grid.y];
        }

        // Create a 2D grid to track which positions are filled
        let mut grid: Vec<Vec<Option<&SubnetChild>>> =
            vec![vec![None; container_grid.x]; container_grid.y];

        // Group by primary handle
        let sorted_children: Vec<_> = children
            .iter()
            .sorted_by_key(|c| {
                (
                    c.primary_handle
                        .as_ref()
                        .map(|h| h.layout_priority())
                        .unwrap_or(255),
                    std::cmp::Reverse(c.anchor_count),
                    std::cmp::Reverse(c.size.y),
                )
            })
            .collect();

        let grouped: HashMap<Option<EdgeHandle>, Vec<_>> = sorted_children
            .into_iter()
            .into_group_map_by(|c| c.primary_handle);

        // Collect all unplaced nodes to handle overflow
        let mut unplaced: Vec<&SubnetChild> = Vec::new();

        // Top edge: place along first row
        if let Some(top_children) = grouped.get(&Some(EdgeHandle::Top)) {
            for (i, child) in top_children.iter().enumerate() {
                if i < container_grid.x {
                    grid[0][i] = Some(child);
                } else {
                    unplaced.push(child);
                }
            }
        }

        // Bottom edge: place along last row
        if let Some(bottom_children) = grouped.get(&Some(EdgeHandle::Bottom)) {
            if container_grid.y > 1 {
                let last_row = container_grid.y - 1;
                for (i, child) in bottom_children.iter().enumerate() {
                    if i < container_grid.x {
                        grid[last_row][i] = Some(child);
                    } else {
                        unplaced.push(child);
                    }
                }
            } else {
                // No space for bottom row, add all to unplaced
                unplaced.extend(bottom_children.iter().copied());
            }
        }

        // Left edge: place along first column (skip corners if already filled)
        if let Some(left_children) = grouped.get(&Some(EdgeHandle::Left)) {
            for (i, child) in left_children.iter().enumerate() {
                let row = i + 1; // Start from row 1 to avoid top corner
                if row < container_grid.y.saturating_sub(1) && grid[row][0].is_none() {
                    grid[row][0] = Some(child);
                } else {
                    unplaced.push(child);
                }
            }
        }

        // Right edge: place along last column (skip corners if already filled)
        if let Some(right_children) = grouped.get(&Some(EdgeHandle::Right)) {
            if container_grid.x > 1 {
                let last_col = container_grid.x - 1;
                for (i, child) in right_children.iter().enumerate() {
                    let row = i + 1; // Start from row 1 to avoid top corner
                    if row < container_grid.y.saturating_sub(1) && grid[row][last_col].is_none() {
                        grid[row][last_col] = Some(child);
                    } else {
                        unplaced.push(child);
                    }
                }
            } else {
                // No space for right column, add all to unplaced
                unplaced.extend(right_children.iter().copied());
            }
        }

        // Add center nodes to unplaced list
        if let Some(center_children) = grouped.get(&None) {
            unplaced.extend(center_children.iter().copied());
        }

        // Fill remaining positions with unplaced nodes (center nodes + overflow from edges)
        let mut unplaced_idx = 0;
        for grid_row in grid.iter_mut().take(container_grid.y) {
            for cell in grid_row.iter_mut().take(container_grid.x) {
                if cell.is_none() && unplaced_idx < unplaced.len() {
                    *cell = Some(unplaced[unplaced_idx]);
                    unplaced_idx += 1;
                }
            }
        }

        // Convert grid to the expected output format
        let mut rows: Vec<Vec<(Uuid, NodeLayout)>> = vec![Vec::new(); container_grid.y];
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
