use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DockerDaemon;

impl ServiceDefinition for DockerDaemon {
    fn name(&self) -> &'static str {
        "Docker Daemon"
    }
    fn description(&self) -> &'static str {
        "Docker daemon"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Virtualization
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::DockerClient
    }

    fn icon(&self) -> &'static str {
        "docker"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<DockerDaemon>
));
