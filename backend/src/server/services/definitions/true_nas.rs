use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TrueNAS;

impl ServiceDefinition for TrueNAS {
    fn name(&self) -> &'static str {
        "TrueNAS"
    }
    fn description(&self) -> &'static str {
        "Open-source network attached storage system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortBase::Samba),
            Pattern::Endpoint(PortBase::Http, "/", "TrueNAS"),
        ])
    }

    fn icon(&self) -> &'static str {
        "truenas"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<TrueNAS>));
