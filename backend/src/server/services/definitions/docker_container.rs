use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DockerContainer;

impl ServiceDefinition for DockerContainer {
    fn name(&self) -> &'static str {
        "Docker Container"
    }
    fn description(&self) -> &'static str {
        "A generic docker container"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Virtualization
    }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec![
            Pattern::DockerContainer,
            Pattern::Custom(|p| {
                // If there's a matched service with the name of the container, the container was already detected as a non-generic service
                let default_name = &String::new();
                let container_name = p
                    .baseline_params
                    .docker_container_name
                    .as_ref()
                    .unwrap_or(default_name);
                p.discovery_state_params
                    .matched_services
                    .iter()
                    .all(|s| s.base.name != *container_name)
            }),
        ])
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<DockerContainer>
));
