use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct VpnClient;

impl ServiceDefinition for VpnClient {
    fn name(&self) -> &'static str {
        "Vpn Client"
    }
    fn description(&self) -> &'static str {
        "A generic VPN Client"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::VPN
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::None
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<VpnClient>));
