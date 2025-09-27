use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::endpoints::{Endpoint};
use crate::server::services::types::patterns::{Pattern};
use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId};
use crate::server::{shared::{types::metadata::TypeMetadataProvider}};
use dyn_clone::DynClone;
use dyn_eq::DynEq;
use dyn_hash::DynHash;
use std::hash::Hash;
use serde::{Serialize, Deserialize};

// Main trait used in service definition implementation
pub trait ServiceDefinition: HasId + DynClone + DynHash + DynEq  + Send + Sync {
    
    /// Service name, will also be used as unique identifier. < 15 characters.
    fn name(&self) -> &'static str;

    /// Service description. < 60 characters.
    fn description(&self) -> &'static str;

    /// Category from ServiceCategory enum
    fn category(&self) -> ServiceCategory;

    /// How service should be identified during port scanning
    fn discovery_pattern(&self) -> Pattern;

    /// If service is not associated with a particular brand or vendor
    fn is_generic(&self) -> bool { false }

    /// If service is capable of acting as a gateway on the network
    fn is_gateway(&self) -> bool { false }

    /// Path of service on https://dashboardicons.com/. For example, Home Assistant -> https://dashboardicons.com/icons/home-assistant
    fn icon(&self) -> &'static str { "" }
}

// Helper methods to be used in rest of codebase, not overridable by definition implementations
pub trait ServiceDefinitionExt {
    fn discovery_ports(&self) -> Vec<PortBase>;
    fn discovery_endpoints(&self) -> Vec<Endpoint>;
    fn can_be_manually_added(&self) -> bool;
    fn is_dns_resolver(&self) -> bool;
    fn is_reverse_proxy(&self) -> bool;
    fn is_infra_service(&self) -> bool;
    fn contains_web_service_pattern(&self) -> bool;
}

impl<T: ServiceDefinition> HasId for T
where
    T: ServiceDefinition
{
    fn id(&self) -> &'static str {
        self.name()
    }
}

impl ServiceDefinition for Box<dyn ServiceDefinition> {
    fn name(&self) -> &'static str {
        ServiceDefinition::name(&**self)
    }
    
    fn description(&self) -> &'static str {
        ServiceDefinition::description(&**self)
    }

    fn icon(&self) -> &'static str {
        ServiceDefinition::icon(&**self)
    }
    
    fn category(&self) -> ServiceCategory {
        ServiceDefinition::category(&**self)
    }
    
    fn discovery_pattern(&self) -> Pattern {
        ServiceDefinition::discovery_pattern(&**self)
    }
    
    fn is_generic(&self) -> bool {
        ServiceDefinition::is_generic(&**self)
    }
    
    fn is_gateway(&self) -> bool {
        ServiceDefinition::is_gateway(&**self)
    }
}

impl ServiceDefinitionExt for Box<dyn ServiceDefinition> {

    fn is_infra_service(&self) -> bool {
        self.is_dns_resolver() || self.is_gateway() || self.is_dns_resolver()
    }

    fn discovery_ports(&self) -> Vec<PortBase> {
        self.discovery_pattern().ports()
    }
    
    fn discovery_endpoints(&self) -> Vec<Endpoint> {
        self.discovery_pattern().endpoints()
    }

    fn can_be_manually_added(&self) -> bool {
        !matches!(ServiceDefinition::category(self), ServiceCategory::Netvisor)
    }

    fn is_dns_resolver(&self) -> bool {
        matches!(ServiceDefinition::category(self), ServiceCategory::DNS | ServiceCategory::AdBlock)
    }

    fn is_reverse_proxy(&self) -> bool {
        ServiceDefinition::category(self) == ServiceCategory::ReverseProxy
    }

    fn contains_web_service_pattern(&self) -> bool {
        self.discovery_pattern().contains_web_service_pattern()
    }
}

impl EntityMetadataProvider for Box<dyn ServiceDefinition> {
    fn color(&self) -> &'static str {
       ServiceDefinition::category(self).color()
    }
    fn icon(&self) -> &'static str {
        let logo_icon = ServiceDefinition::icon(self);
        if logo_icon.len() > 0 {
            return logo_icon
        }
        ServiceDefinition::category(self).icon()
    }
}

impl TypeMetadataProvider for Box<dyn ServiceDefinition> {
    fn name(&self) -> &'static str {
       ServiceDefinition::name(self)
    }
    fn description(&self) -> &'static str {
       ServiceDefinition::description(self)
    }
    fn category(&self) -> &'static str {
       ServiceDefinition::category(self).id()
    }
    fn metadata(&self) -> serde_json::Value {
        // let default_ports = self.discovery_ports();
        // let default_endpoints = self.discovery_endpoints();
        let can_be_added = self.can_be_manually_added();    
        let is_dns_resolver = self.is_dns_resolver();
        let is_reverse_proxy = self.is_reverse_proxy();
        let is_gateway = self.is_gateway();
        let is_generic = self.is_generic();
        let has_homarr_icon = ServiceDefinition::icon(self).len() > 0;
        serde_json::json!({
            // "default_ports": default_ports, 
            // "default_endpoints": default_endpoints, 
            "can_be_added": can_be_added, 
            "is_dns_resolver": is_dns_resolver,
            "is_gateway": is_gateway,
            "is_reverse_proxy": is_reverse_proxy,
            "is_generic": is_generic,
            "has_homarr_icon": has_homarr_icon
        })
    }
}


dyn_eq::eq_trait_object!(ServiceDefinition);
dyn_hash::hash_trait_object!(ServiceDefinition);
dyn_clone::clone_trait_object!(ServiceDefinition);

impl Default for Box<dyn ServiceDefinition> {
    fn default() -> Self {
        Box::new(DefaultServiceDefinition)
    }
}

impl std::fmt::Debug for Box<dyn ServiceDefinition> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, category: {}, description: {}", ServiceDefinition::name(&**self), ServiceDefinition::category(&**self), ServiceDefinition::description(&**self))
    }
}

impl Serialize for Box<dyn ServiceDefinition> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.id())
    }
}

impl<'de> Deserialize<'de> for Box<dyn ServiceDefinition> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        ServiceDefinitionRegistry::find_by_id(&id)
            .ok_or_else(|| serde::de::Error::custom(format!("Service definition not found: {}", id)))
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct DefaultServiceDefinition;

impl ServiceDefinition for DefaultServiceDefinition {
    fn name(&self) -> &'static str { "Default Service" }
    fn description(&self) -> &'static str { "Default service implementation" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Unknown }
    fn discovery_pattern(&self) -> Pattern { Pattern::None }
}