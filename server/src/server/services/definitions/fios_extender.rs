use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FiosExtender;

impl ServiceDefinition for FiosExtender {
    fn name(&self) -> &'static str { "Fios Extender" }
    fn description(&self) -> &'static str { "Fios device providing mesh networking services" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkAccess }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(Pattern::WebService("/#/login/", "fios"), Pattern::NotGatewayIp))
    }

    fn icon(&self) -> &'static str {
        "fios"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<FiosExtender>));
