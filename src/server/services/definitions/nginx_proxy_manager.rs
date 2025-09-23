use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NginxProxyManager;

impl ServiceDefinition for NginxProxyManager {
    fn name(&self) -> &'static str { "Nginx Proxy Manager" }
    fn description(&self) -> &'static str { "Web-based Nginx proxy management interface" }
    fn category(&self) -> ServiceCategory { ServiceCategory::ReverseProxy }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::WebService("/", "Nginx Proxy Manager")
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NginxProxyManager>));
