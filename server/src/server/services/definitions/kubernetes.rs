use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Kubernetes;

impl ServiceDefinition for Kubernetes {
    fn name(&self) -> &'static str { "Kubernetes" }
    fn description(&self) -> &'static str { "Container orchestration platform" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Virtualization }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(
            Pattern::AllPort(vec![PortBase::new_tcp(6443)]),
            Pattern::AnyPort(vec!(PortBase::new_tcp(10250), PortBase::new_tcp(10259), PortBase::new_tcp(10257), PortBase::new_tcp(10256)))
        ))
    }

    fn icon(&self) -> &'static str {
        "kubernetes"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Kubernetes>));
