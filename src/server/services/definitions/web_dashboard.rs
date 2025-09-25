use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::types::ServiceDefinitionExt;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WebDashboard;

impl ServiceDefinition for WebDashboard {
    fn name(&self) -> &'static str { "Web Dashboard" }
    fn description(&self) -> &'static str { "A generic web UI for managing a service" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Dashboard }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::AnyPort(vec![Port::HTTP, Port::HTTPS]),
            Pattern::HasAnyMatchedService,
            Pattern::AllMatchedService(|s| !s.is_generic() && !s.contains_web_service_pattern())
        ])
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<WebDashboard>));
