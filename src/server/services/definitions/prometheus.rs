use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Prometheus;

impl ServiceDefinition for Prometheus {
    fn name(&self) -> &'static str { "Prometheus" }
    fn description(&self) -> &'static str { "Time-series monitoring and alerting system" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Monitoring }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::Port(PortBase::new_tcp(9090))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Prometheus>));
