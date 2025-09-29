use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::definitions::ServiceDefinition;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Grafana;

impl ServiceDefinition for Grafana {
    fn name(&self) -> &'static str {
        "Grafana"
    }
    fn description(&self) -> &'static str {
        "Analytics and monitoring visualization platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "Grafana")
    }

    fn icon(&self) -> &'static str {
        "grafana"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Grafana>));
