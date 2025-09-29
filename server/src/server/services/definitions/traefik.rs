use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Traefik;

impl ServiceDefinition for Traefik {
    fn name(&self) -> &'static str { "Traefik" }
    fn description(&self) -> &'static str { "Modern reverse proxy and load balancer" }
    fn category(&self) -> ServiceCategory { ServiceCategory::ReverseProxy }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/dashboard/", "Traefik")
    }

    fn icon(&self) -> &'static str {
        "traefik"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Traefik>));
