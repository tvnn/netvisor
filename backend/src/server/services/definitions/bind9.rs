use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Bind9;

impl ServiceDefinition for Bind9 {
    fn name(&self) -> &'static str {
        "Bind9"
    }
    fn description(&self) -> &'static str {
        "Berkeley Internet Name Domain DNS server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::DNS
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortBase::DnsUdp),
            Pattern::Port(PortBase::new_tcp(8053)),
        ])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Bind9>));
