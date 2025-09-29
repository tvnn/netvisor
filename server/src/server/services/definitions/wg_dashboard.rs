use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;
use crate::server::subnets::types::base::SubnetType;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WgDashboard;

impl ServiceDefinition for WgDashboard {
    fn name(&self) -> &'static str {
        "WGDashboard"
    }
    fn description(&self) -> &'static str {
        "Wireguard dashboard for visualizing and managing wireguard clients and server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::AnyPort(vec![PortBase::new_tcp(10086)]),
            Pattern::SubnetIsNotType(SubnetType::VpnTunnel),
        ])
    }

    fn icon(&self) -> &'static str {
        "wireguard"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<WgDashboard>));
