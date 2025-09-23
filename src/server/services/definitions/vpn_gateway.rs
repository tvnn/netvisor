use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct VpnGateway;

impl ServiceDefinition for VpnGateway {
    fn name(&self) -> &'static str { "Vpn Gateway" }
    fn description(&self) -> &'static str { "A generic VPN Gateway" }
    fn category(&self) -> ServiceCategory { ServiceCategory::VPN }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(
            Pattern::IsVpnSubnetGateway,
            Pattern::AnyPort(vec!(Port::SSH, Port::HTTP, Port::HTTPS))
        ))
    }

    fn is_gateway(&self) -> bool { true }    
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<VpnGateway>));
