use serde::{Deserialize, Serialize};
use std::hash::Hash;
use strum_macros::IntoStaticStr;
use validator::Validate;

use crate::server::shared::{
    constants::Entity,
    types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, IntoStaticStr)]
#[serde(tag = "type", content = "details")]
pub enum Virtualization {
    Docker(DockerVirtualization),
    Proxmox(ProxmoxVirtualization),
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize, PartialEq, Eq, Hash)]
pub struct DockerVirtualization {
    pub container_name: Option<String>,
    pub container_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize, PartialEq, Eq, Hash)]
pub struct ProxmoxVirtualization {
    pub vm_name: Option<String>,
    pub vm_id: Option<String>,
}

impl HasId for Virtualization {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for Virtualization {
    fn color(&self) -> &'static str {
        Entity::Virtualization.color()
    }
    fn icon(&self) -> &'static str {
        Entity::Virtualization.icon()
    }
}

impl TypeMetadataProvider for Virtualization {
    fn name(&self) -> &'static str {
        "Docker"
    }

    fn description(&self) -> &'static str {
        "A service running in a docker container"
    }
}
