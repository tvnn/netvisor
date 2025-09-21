use strum_macros::{Display, EnumDiscriminants, EnumIter};

use crate::server::shared::types::metadata::EntityMetadataProvider;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display))]
pub enum Entity {
    Host,
    Service,
    Interface,
    Subnet,
    Group,
    Topology,

    Media,
    Dns,
    Vpn,
    Gateway,
    ReverseProxy,
}

impl EntityMetadataProvider for Entity {
    fn color(&self) -> &'static str {
        match self {
            Entity::Host => "blue",
            Entity::Service => "indigo",
            Entity::Interface => "purple",

            Entity::Media => "teal",
            Entity::Dns => "emerald",
            Entity::Vpn => "green",
            Entity::Gateway => "yellow",
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
            Entity::Interface => "EthernetPort",
            Entity::Media => "Music",
            Entity::Dns => "Search",
            Entity::Vpn => "HatGlasses",
            Entity::Gateway => "Router",
            Entity::ReverseProxy => "Split",
            Entity::Subnet => "Network",
            Entity::Group => "Group",
            Entity::Topology => "ChartNetwork"
        }
    }
}