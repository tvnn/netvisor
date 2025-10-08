use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;
use crate::server::subnets::types::base::SubnetType;

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
            Pattern::IsGatewayIp,
            Pattern::SubnetIsType(SubnetType::VpnTunnel),
        ])
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<VpnGateway>));
