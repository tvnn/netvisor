use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::definitions::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Fortinet;

impl ServiceDefinition for Fortinet {
    fn name(&self) -> &'static str {
        "Fortinet"
    }
    fn description(&self) -> &'static str {
        "Fortinet security appliance"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkSecurity
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "fortinet")
    }

    fn is_gateway(&self) -> bool {
        true
    }

    fn icon(&self) -> &'static str {
        "fortinet"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Fortinet>));
