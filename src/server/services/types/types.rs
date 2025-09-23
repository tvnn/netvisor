use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::endpoints::{Endpoint};
use crate::server::services::types::patterns::{Pattern};
use crate::server::services::types::ports::{Port};
use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId};
use crate::server::{shared::{types::metadata::TypeMetadataProvider}};
use dyn_clone::DynClone;
use dyn_eq::DynEq;
use dyn_hash::DynHash;
use std::hash::Hash;
use serde::{Serialize, Deserialize};

pub trait ServiceDefinition: HasId + TypeMetadataProvider + EntityMetadataProvider + DynClone + DynHash + DynEq  + Send + Sync{
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn category(&self) -> ServiceCategory;
    fn discovery_pattern(&self) -> Pattern;

    fn is_generic(&self) -> bool { false }
    fn is_gateway(&self) -> bool { false }
}

pub trait ServiceDefinitionHelpers: ServiceDefinition  {

    fn discovery_ports(&self) -> Vec<Port> {
        self.discovery_pattern().ports()
    }
    
    fn discovery_endpoints(&self) -> Vec<Endpoint> {
        self.discovery_pattern().endpoints()
    }
    
    fn is_end_device(&self) -> bool { 
        matches!(ServiceDefinition::category(self), ServiceCategory::Workstation | ServiceCategory::Mobile)
     }

    fn can_be_manually_added(&self) -> bool {
        matches!(ServiceDefinition::category(self), ServiceCategory::Netvisor)
    }

    fn is_dns_resolver(&self) -> bool {
        matches!(ServiceDefinition::category(self), ServiceCategory::DNS | ServiceCategory::AdBlock)
    }

    fn is_reverse_proxy(&self) -> bool {
        ServiceDefinition::category(self) == ServiceCategory::ReverseProxy
    }
}

impl<T: ServiceDefinition> ServiceDefinitionHelpers for T 
where 
    T: ServiceDefinition + 'static
{}

impl ServiceDefinition for Box<dyn ServiceDefinition> {
    fn name(&self) -> &'static str {
        ServiceDefinition::name(&**self)
    }
    
    fn description(&self) -> &'static str {
        ServiceDefinition::description(&**self)
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

impl<T: ServiceDefinition> HasId for T {
    fn id(&self) -> &'static str {
        <T as ServiceDefinition>::name(self)
    }
}

impl<T: ServiceDefinition> EntityMetadataProvider for T 
where 
    T: ServiceDefinition + HasId,
{
    fn color(&self) -> &'static str {
        <T as ServiceDefinition>::category(self).color()
    }
    fn icon(&self) -> &'static str {
        <T as ServiceDefinition>::category(self).icon()
    }
}

impl<T: ServiceDefinition> TypeMetadataProvider for T 
where 
    T: ServiceDefinition + HasId + Clone + 'static,
{
    fn name(&self) -> &'static str {
        <T as ServiceDefinition>::name(self)
    }
    fn description(&self) -> &'static str {
        <T as ServiceDefinition>::description(self)
    }
    fn category(&self) -> &'static str {
        <T as ServiceDefinition>::category(self).id()
    }
    fn metadata(&self) -> serde_json::Value {
        let default_ports = self.discovery_ports();
        let default_endpoints = self.discovery_endpoints();
        let can_be_added = self.can_be_manually_added();    
        let is_dns_resolver = self.is_dns_resolver();
        let is_reverse_proxy = self.is_reverse_proxy();
        let is_gateway = self.is_gateway();
        let is_generic = self.is_generic();
        serde_json::json!({
            "default_ports": default_ports, 
            "default_endpoints": default_endpoints, 
            "can_be_added": can_be_added, 
            "is_dns_resolver": is_dns_resolver,
            "is_gateway": is_gateway,
            "is_reverse_proxy": is_reverse_proxy,
            "is_generic": is_generic
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