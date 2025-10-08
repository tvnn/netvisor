use serde::{Deserialize, Serialize};
use std::hash::Hash;
use strum_macros::{EnumDiscriminants, IntoStaticStr};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Eq, Deserialize)]
pub struct ServiceBinding {
    pub binding_id: Uuid,
    pub service_id: Uuid,
}

impl PartialEq for ServiceBinding {
    fn eq(&self, other: &Self) -> bool {
        self.binding_id == other.binding_id && self.service_id == other.service_id
    }
}

impl Hash for ServiceBinding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.binding_id.hash(state);
        self.service_id.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, EnumDiscriminants)]
#[strum_discriminants(derive(IntoStaticStr))]
#[serde(tag = "type")]
pub enum Binding {
    Layer3 {
        id: Uuid,
        interface_id: Uuid,
    },
    Layer4 {
        id: Uuid,
        port_id: Uuid,
        interface_id: Uuid,
    },
}

impl Binding {
    pub fn id(&self) -> Uuid {
        match self {
            Binding::Layer3 { id, .. } => *id,
            Binding::Layer4 { id, .. } => *id,
        }
    }

    pub fn interface_id(&self) -> Uuid {
        match self {
            Binding::Layer3 { interface_id, .. } => *interface_id,
            Binding::Layer4 { interface_id, .. } => *interface_id,
        }
    }

    pub fn port_id(&self) -> Option<Uuid> {
        match self {
            Binding::Layer3 { .. } => None,
            Binding::Layer4 { port_id, .. } => Some(*port_id),
        }
    }
}

impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Binding::Layer3 {
                    interface_id: self_interface_id,
                    ..
                },
                Binding::Layer3 {
                    interface_id: other_interface_id,
                    ..
                },
            ) => self_interface_id == other_interface_id,
            (
                Binding::Layer4 {
                    port_id: self_port_id,
                    interface_id: self_interface_id,
                    ..
                },
                Binding::Layer4 {
                    port_id: other_port_id,
                    interface_id: other_interface_id,
                    ..
                },
            ) => self_interface_id == other_interface_id && self_port_id == other_port_id,
            _ => false,
        }
    }
}

impl Hash for Binding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Binding::Layer3 { interface_id, .. } => {
                interface_id.hash(state);
            }
            Binding::Layer4 {
                port_id,
                interface_id,
                ..
            } => {
                port_id.hash(state);
                interface_id.hash(state);
            }
        }
    }
}

impl Binding {
    pub fn new_l3(interface_id: Uuid) -> Self {
        Binding::Layer3 {
            id: Uuid::new_v4(),
            interface_id,
        }
    }

    pub fn new_l4(port_id: Uuid, interface_id: Uuid) -> Self {
        Binding::Layer4 {
            id: Uuid::new_v4(),
            port_id,
            interface_id,
        }
    }
}
