use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Traefik;

impl ServiceDefinition for Traefik {
    fn name(&self) -> &'static str {
        "Traefik"
    }
    fn description(&self) -> &'static str {
        "Modern reverse proxy and load balancer"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ReverseProxy
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::Http, "/dashboard", "traefik")
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "traefik"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Traefik>));
