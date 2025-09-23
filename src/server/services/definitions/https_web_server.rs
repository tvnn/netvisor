use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::services::types::ports::Port;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HttpsWebServer;

impl ServiceDefinition for HttpsWebServer {
    fn name(&self) -> &'static str { "Https Web Server" }
    fn description(&self) -> &'static str { "A generic HTTPS Web Server" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Web }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AnyPort(vec!(Port::HTTPS, Port::HTTPSALT))
    }

    fn is_generic(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HttpsWebServer>));
