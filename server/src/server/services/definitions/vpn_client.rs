use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct VpnClient;

impl ServiceDefinition for VpnClient {
    fn name(&self) -> &'static str { "Vpn Client" }
    fn description(&self) -> &'static str { "A generic VPN Client" }
    fn category(&self) -> ServiceCategory { ServiceCategory::VPN }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::None
    }   

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<VpnClient>));
