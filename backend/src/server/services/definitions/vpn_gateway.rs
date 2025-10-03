use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct VpnGateway;

impl ServiceDefinition for VpnGateway {
    fn name(&self) -> &'static str {
        "Vpn Gateway"
    }
    fn description(&self) -> &'static str {
        "A generic VPN Gateway"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::VPN
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::IsVpnSubnetGateway,
            Pattern::AnyPort(vec![PortBase::Ssh, PortBase::Http, PortBase::Https]),
        ])
    }

    fn is_gateway(&self) -> bool {
        true
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<VpnGateway>));
