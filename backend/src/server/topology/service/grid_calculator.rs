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
                child_positions.insert(
                    id,
                    Ixy {
                        x: current_x as isize,
                        y: current_y as isize,
                    },
                );
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
