use strum_macros::{Display, EnumDiscriminants, EnumIter, IntoStaticStr};

use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumDiscriminants, EnumIter, IntoStaticStr)]
#[strum_discriminants(derive(Display))]
pub enum Entity {
    Host,
    Service,
    Port,
    Interface,

    Subnet,
    Group,
    Topology,

    Dns,
    Vpn,
    Gateway,
    ReverseProxy,
}

impl HasId for Entity {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for Entity {
    fn color(&self) -> &'static str {
        match self {
            Entity::Host => "blue",
            Entity::Service => "indigo",
            Entity::Interface => "purple",
            Entity::Port => "purple",

            Entity::Dns => "emerald",
            Entity::Vpn => "green",
            Entity::Gateway => "teal",
            Entity::ReverseProxy => "cyan",

            Entity::Subnet => "orange",
            Entity::Group => "rose",
            Entity::Topology => "pink"
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Entity::Host => "Server",
            Entity::Service => "Layers",
            Entity::Interface => "Binary",
            Entity::Dns => "Search",
            Entity::Vpn => "VenetianMask",
            Entity::Port => "EthernetPort",
            Entity::Gateway => "Router",
            Entity::ReverseProxy => "Split",
            Entity::Subnet => "Network",
            Entity::Group => "Group",
            Entity::Topology => "ChartNetwork"
        }
    }
}