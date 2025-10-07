use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Uxy {
    pub x: usize,
    pub y: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Ixy {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub size: Uxy,
    pub grid_position: Uxy,
}

pub struct SubnetLayout {
    pub size: Uxy,
    pub infra_width: usize,
}
