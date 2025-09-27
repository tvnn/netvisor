use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::patterns::{Pattern, Vendor};
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::types::types::ServiceDefinition;
use crate::server::services::types::categories::ServiceCategory;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HpPrinter;

impl ServiceDefinition for HpPrinter {
    fn name(&self) -> &'static str { "Hp Printer" }
    fn description(&self) -> &'static str { "An HP Printer" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Printer }

    fn discovery_pattern(&self) -> Pattern {
        Pattern::AllOf(vec!(
            Pattern::MacVendor(Vendor::HP), 
            Pattern::AnyPort(vec![PortBase::Ipp, PortBase::LdpTcp, PortBase::LdpUdp])
        ))
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HpPrinter>));
