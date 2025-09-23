use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DockerSwarm;

impl ServiceDefinition for DockerSwarm {
    fn name(&self) -> &'static str { "Docker Swarm" }
    fn description(&self) -> &'static str { "Docker native clustering and orchestration" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Virtualization }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllPort(vec![Port::new_tcp(2377), Port::new_tcp(7946)])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<DockerSwarm>));
