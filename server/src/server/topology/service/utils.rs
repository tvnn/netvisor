use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

use crate::server::{
    subnets::types::base::Subnet,
    topology::types::{
        base::{NodeLayout, SubnetLayout, XY},
        edges::EdgeHandle,
        nodes::SubnetChild,
    },
};

pub struct TopologyUtils {}

impl Default for TopologyUtils {
    fn default() -> Self {
        Self::new()
    }
}

impl TopologyUtils {
    pub fn new() -> Self {
        Self {}
    }

    /// Figure out closest shape to square that can contain children
    pub fn calculate_container_grid_dimensions(&self, children: usize) -> XY {
        if children == 0 {
            return XY { x: 1, y: 1 };
        }

        let x = (children as f64).sqrt().ceil() as usize;
        let y = ((children as f64) / x as f64).ceil() as usize;
        XY { x, y }
    }

    /// Calculate the coordinates of a child in a grid given its index
    pub fn calculate_child_coordinates_in_grid(&self, grid: &XY, child_idx: usize) -> XY {
        XY {
            x: child_idx % grid.x,
            y: ((child_idx / grid.x) as f64).floor() as usize,
        }
    }

    // Calculate child positions using anchor-based layout within a subnet
    /// Layout uses Top/Bottom/Left/Right sections with center for no-handle nodes
    pub fn calculate_anchor_based_child_positions(
        &self,
        children: &[SubnetChild],
        container_grid: &XY,
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
                    std::cmp::Reverse(c.size.size().y),
                )
            })
            .collect();

        let grouped: HashMap<Option<EdgeHandle>, Vec<_>> = sorted_children
            .into_iter()
            .into_group_map_by(|c| c.primary_handle.clone());

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
                            size: child.size.size(),
                            grid_position: XY {
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

    /// Calculate positions of subnets given layer values
    pub fn calculate_subnet_grid_positions_by_layer(
        &self,
        subnets: &[Subnet],
        layouts: &HashMap<Uuid, SubnetLayout>,
    ) -> Vec<Vec<(Uuid, NodeLayout)>> {
        // Group subnets by layer
        let sorted: Vec<(&Subnet, &SubnetLayout)> = subnets
            .iter()
            .sorted_by_key(|s| {
                (
                    s.base.subnet_type.default_layer(),
                    s.base.subnet_type.layer_priority(),
                    s.base.name.clone(),
                )
            })
            .filter_map(|s| {
                if let Some(layout) = layouts.get(&s.id) {
                    return Some((s, layout));
                }
                None
            })
            .collect();

        let mut subnets_by_layer: BTreeMap<usize, Vec<(&Uuid, &SubnetLayout)>> = BTreeMap::new();
        for (subnet, layout) in sorted {
            subnets_by_layer
                .entry(subnet.base.subnet_type.default_layer())
                .or_default()
                .push((&subnet.id, layout));
        }

        // Use enumerate to get sequential row numbers (collapsed, no gaps)
        subnets_by_layer
            .into_iter()
            .enumerate() // This gives us 0, 1, 2... for actual rows
            .map(|(row_index, (_layer, row))| {
                row.into_iter()
                    .enumerate()
                    .map(|(y, (id, layout))| {
                        (
                            *id,
                            NodeLayout {
                                size: layout.size.clone(),
                                grid_position: XY { x: row_index, y }, // Use sequential row index
                            },
                        )
                    })
                    .collect()
            })
            .collect()
    }

    /// Calculate the size of a container and positions of arbitrarily-sized children in that container
    pub fn calculate_container_size(
        &self,
        rows: Vec<Vec<(Uuid, NodeLayout)>>,
        padding: &XY,
    ) -> (HashMap<Uuid, XY>, XY) {
        let mut child_positions = HashMap::new();

        let mut current_y = padding.y;
        let mut max_x = 0;
        let mut max_y = 0;

        for row in rows {
            if row.is_empty() {
                continue;
            }

            let mut current_x = padding.x;
            let mut max_height_in_row = 0;

            for (id, layout) in row {
                child_positions.insert(
                    id,
                    XY {
                        x: current_x,
                        y: current_y,
                    },
                );
                current_x += layout.size.x + padding.x;
                max_height_in_row = max_height_in_row.max(layout.size.y);
            }

            current_y += max_height_in_row + padding.y;
            max_x = max_x.max(current_x);
            max_y = max_y.max(current_y);
        }

        let container_size = XY { x: max_x, y: max_y };

        (child_positions, container_size)
    }
}
