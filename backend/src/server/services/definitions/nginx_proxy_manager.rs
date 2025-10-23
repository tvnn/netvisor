use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NginxProxyManager;

impl ServiceDefinition for NginxProxyManager {
    fn name(&self) -> &'static str {
        "Nginx Proxy Manager"
    }
    fn description(&self) -> &'static str {
        "Web-based Nginx proxy management interface"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ReverseProxy
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::Http, "", "nginx proxy manager")
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "nginx-proxy-manager"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<NginxProxyManager>
));
