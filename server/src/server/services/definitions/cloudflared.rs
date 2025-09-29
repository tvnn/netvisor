use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Cloudflared;

impl ServiceDefinition for Cloudflared {
    fn name(&self) -> &'static str { "Cloudflared" }
    fn description(&self) -> &'static str { "Cloudflare tunnel daemon" }
    fn category(&self) -> ServiceCategory { ServiceCategory::ReverseProxy }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/metrics", "cloudflared")
    }

    fn icon(&self) -> &'static str {
        "cloudflare"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Cloudflared>));
