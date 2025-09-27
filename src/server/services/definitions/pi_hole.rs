use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::Pattern;
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PiHole;

impl ServiceDefinition for PiHole {
    fn name(&self) -> &'static str { "Pi-Hole" }
    fn description(&self) -> &'static str { "Network-wide ad blocking DNS service" }
    fn category(&self) -> ServiceCategory { ServiceCategory::AdBlock }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(Pattern::AllPort(vec!(PortBase::DnsUdp, PortBase::DnsTcp)), Pattern::WebService("/admin", "Pi-hole")))
    }

    fn icon(&self) -> &'static str {
        "pi-hole"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PiHole>));
