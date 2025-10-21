use std::collections::HashMap;
use uuid::Uuid;

use crate::server::topology::types::base::{Ixy, NodeLayout, Uxy};

pub struct GridCalculator;

impl GridCalculator {
    /// Figure out closest shape to square that can contain children
    pub fn calculate_grid_dimensions(children_count: usize) -> Uxy {
        if children_count == 0 {
            return Uxy { x: 1, y: 1 };
        }

        let x = (children_count as f64).sqrt().ceil() as usize;
        let y = ((children_count as f64) / x as f64).ceil() as usize;
        Uxy { x, y }
    }

    /// Calculate the coordinates of a child in a grid given its index
    pub fn calculate_child_coordinates_in_grid(grid: &Uxy, child_idx: usize) -> Uxy {
        Uxy {
            x: child_idx % grid.x,
            y: ((child_idx / grid.x) as f64).floor() as usize,
        }
    }

    /// Calculate container size from positioned nodes
    /// CHANGED: Now takes HashMap<Uuid, NodeLayout> instead of rows
    pub fn calculate_container_size_from_layouts(
        layouts: &HashMap<Uuid, NodeLayout>,
        padding: &Uxy,
    ) -> Uxy {
        if layouts.is_empty() {
            return Uxy {
                x: padding.x * 2,
                y: padding.y * 2,
            };
        }

        let mut max_x = 0;
        let mut max_y = 0;

        for layout in layouts.values() {
            let right_edge =
                (layout.position.x + layout.size.x as isize + padding.x as isize) as usize;
            let bottom_edge =
                (layout.position.y + layout.size.y as isize + padding.y as isize) as usize;

            max_x = max_x.max(right_edge);
            max_y = max_y.max(bottom_edge);
        }

        Uxy { x: max_x, y: max_y }
    }

    /// Legacy version for subnet positioning (still uses row-based layout)
    /// Calculate the size of a container and positions of arbitrarily-sized children in that container
    pub fn calculate_container_size(
        rows: Vec<Vec<(Uuid, NodeLayout)>>,
        padding: &Uxy,
    ) -> (HashMap<Uuid, Ixy>, Uxy) {
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
                // For backward compatibility with subnet positioning,
                // use the position from layout if it's already set, otherwise calculate
                let position = Ixy {
                    x: current_x as isize,
                    y: current_y as isize,
                };

                child_positions.insert(id, position);
                current_x += layout.size.x + padding.x;
                max_height_in_row = max_height_in_row.max(layout.size.y);
            }

            current_y += max_height_in_row + padding.y;
            max_x = max_x.max(current_x);
            max_y = max_y.max(current_y);
        }

        let container_size = Uxy { x: max_x, y: max_y };

        (child_positions, container_size)
    }
}
