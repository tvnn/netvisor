use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HomeAssistant;

impl ServiceDefinition for HomeAssistant {

    fn name(&self) -> &'static str { "Home Assistant" }
    fn description(&self) -> &'static str { "Open-source home automation platform" }
    fn category(&self) -> ServiceCategory { ServiceCategory::HomeAutomation }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(PortBase::new_tcp(8123)))
    }

    fn icon(&self) -> &'static str {
        "home-assistant"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HomeAssistant>));
