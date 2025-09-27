use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct QNAP;

impl ServiceDefinition for QNAP {
    fn name(&self) -> &'static str { "QNAP NAS" }
    fn description(&self) -> &'static str { "QNAP network attached storage system" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Storage }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "QNAP")
    }

    fn icon(&self) -> &'static str {
        "qnap"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<QNAP>));
