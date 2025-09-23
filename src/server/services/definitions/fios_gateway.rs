use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FiosGateway;

impl ServiceDefinition for FiosGateway {
    fn name(&self) -> &'static str { "Fios Gateway" }
    fn description(&self) -> &'static str { "Fios device providing routing and gateway services" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkAccess }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(Pattern::WebService("/#/login/", "fios"), Pattern::IsGatewayIp))
    }

    fn is_gateway(&self) -> bool { true }    
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<FiosGateway>));
