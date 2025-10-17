use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PiHole;

impl ServiceDefinition for PiHole {
    fn name(&self) -> &'static str {
        "Pi-Hole"
    }
    fn description(&self) -> &'static str {
        "Network-wide ad blocking DNS service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::AdBlock
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AllOf(vec![
                Pattern::Port(PortBase::DnsUdp),
                Pattern::Port(PortBase::DnsTcp),
            ]),
            Pattern::Endpoint(PortBase::Http, "/admin", "pi-hole"),
        ])
    }

    fn icon(&self) -> &'static str {
        "pi-hole"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PiHole>));
