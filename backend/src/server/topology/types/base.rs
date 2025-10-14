use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Uxy {
    pub x: usize,
    pub y: usize,
}

const SUBNET_CHILD_HEADER_HEIGHT: usize = 25;
const SUBNET_CHILD_FOOTER_HEIGHT: usize = 25;
const HEIGHT_PER_SERVICE_IN_SUBNET_CHILD: usize = 50;
const SUBNET_CHILD_WIDTH: usize = 250;

impl Uxy {
    pub fn default_subnet_child_size() -> Self {
        Self {
            x: SUBNET_CHILD_WIDTH,
            y: HEIGHT_PER_SERVICE_IN_SUBNET_CHILD + SUBNET_CHILD_FOOTER_HEIGHT,
        }
    }

    pub fn subnet_child_size_from_service_count(service_count: usize, has_header: bool) -> Self {
        Self {
            x: SUBNET_CHILD_WIDTH,
            y: HEIGHT_PER_SERVICE_IN_SUBNET_CHILD * service_count
                + SUBNET_CHILD_FOOTER_HEIGHT
                + if has_header {
                    SUBNET_CHILD_HEADER_HEIGHT
                } else {
                    0
                },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Ixy {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Copy, Clone)]
pub struct NodeLayout {
    pub size: Uxy,
    pub grid_position: Uxy,
}

#[derive(Debug, Copy, Clone)]
pub struct SubnetLayout {
    pub size: Uxy,
    pub infra_width: usize,
}
