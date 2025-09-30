use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct XY {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub size: XY,
    pub grid_position: XY,
}

pub struct SubnetLayout {
    pub size: XY,
    pub infra_width: usize,
}

