use std::str::FromStr;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::capabilities::types::base::{Capability, CapabilityDiscriminants};
use crate::server::nodes::service::NodeService;
use crate::server::shared::forms::field_factory::FieldFactory;
use crate::server::shared::types::metadata::TypeMetadataProvider;
use crate::server::shared::forms::{
    types::fields::*,
};
use crate::server::tests::utilities::dns::DnsServerConfig;
use crate::server::{
    nodes::types::{
        base::Node, 
        targets::{NodeTarget}}, 
    tests::{
        implementations::*, 
        types::{
            configs::*, 
            execution::*,
        },
    }};
use strum_macros::{EnumIter, EnumDiscriminants, Display};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, EnumIter, Deserialize, Serialize, Hash))]
#[serde(tag="type", content="config")]
pub enum Test {
    // Basic Connectivity Tests
    Connectivity(ConnectivityConfig),
    
    // Service-Specific Tests
    ServiceHealth(ServiceHealthConfig),
    DnsResolution(DnsResolutionConfig),
    DnsLookup(DnsLookupConfig),
    ReverseDns(ReverseDnsConfig),
    
    // VPN-Specific Tests
    VpnSubnetAccess(VpnSubnetAccessConfig),
}

impl Test {
    pub fn variant_name(&self) -> String {
        TestDiscriminants::from(self).to_string()
    }

    async fn resolve_dns_server_config_from_node_uuid(&self, dns_node_id: &Option<Uuid>, node_service: &NodeService) -> Result<DnsServerConfig, Error> {
        
        let id = match dns_node_id {
            Some(node_id) => node_id,
            None => return Err(Error::msg("No DNS resolver set on node"))
        };
        
        let node = node_service.get_node(id).await?.ok_or_else(|| Error::msg("Node could not be resolved from id"))?;

        let dns_capability = match node.get_capability(CapabilityDiscriminants::Dns) {
            Some(cap) => cap,
            None => return Err(Error::msg("Node does not have DNS capability"))
        };

        let port = match dns_capability.config_base().port {
            Some(p) => p,
            None => return Err(Error::msg("DNS capability does not have a port"))
        };

        match node.base.target {
            NodeTarget::IpAddress(target) => Ok(DnsServerConfig {
                ip: target.ip,
                port,
                name: node.base.name,
            }),
            _ => Err(Error::msg("Provided DNS node does not have an IP address target")),
        }
    }

    pub async fn execute(&self, timer: &Timer, node: &Node, capability: &Capability, node_service: &NodeService) -> Result<TestResult, Error> {

        let uuid = match &node.base.dns_resolver_node_id {
            Some(id) => Uuid::from_str(&id)?,
            None => Uuid::new_v4()
        };

        match self {
            Test::Connectivity(config) => {
                let dns_server = self.resolve_dns_server_config_from_node_uuid(&Some(uuid), node_service).await.ok();
                connectivity::execute_connectivity_test(config, &timer, &node, dns_server.as_ref(), capability).await
            },
            Test::DnsLookup(config) => {
                let dns_server = self.resolve_dns_server_config_from_node_uuid(&Some(uuid), node_service).await?;
                dns::execute_dns_lookup_test(config, &timer, &node, &dns_server).await
            },
            Test::ReverseDns(config) => {
                let dns_server = self.resolve_dns_server_config_from_node_uuid(&Some(uuid), node_service).await?;
                dns::execute_reverse_dns_lookup_test(config, &timer, &node, &dns_server).await
            },
            Test::VpnSubnetAccess(config) => {
                let dns_server = self.resolve_dns_server_config_from_node_uuid(&Some(uuid), node_service).await.ok();
                vpn::execute_vpn_subnet_access_test(config, &timer, &node, dns_server.as_ref(), capability).await
            },
            Test::DnsResolution(config) => {
                let dns_server = &self.resolve_dns_server_config_from_node_uuid(&Some(node.id), node_service).await?;
                dns::execute_dns_resolution_test(config, &timer, dns_server).await
            },
            Test::ServiceHealth(config) => connectivity::execute_service_health_test(config, &timer, &node, capability).await,
        }
    }
        
    pub fn generate_fields(&self) -> Vec<ConfigField> {
        // Common timeout field for all tests
        let mut fields = Vec::new();
        fields.push(FieldFactory::timeout());
        fields.push(FieldFactory::criticality());
        
        fields.extend(
            match self {
                Test::ServiceHealth(_) => { vec!(FieldFactory::http_status_code("HTTP status code the service should return (200, 204, 404, etc.)".to_string())) },
                Test::DnsResolution(_) => { vec!(FieldFactory::domain("Domain name to resolve using this DNS server".to_string()), FieldFactory::ip("IP address the domain should resolve to".to_string()))},
                Test::DnsLookup(_) => { vec!(FieldFactory::ip("IP address this node's domain should resolve to".to_string())) },         
                Test::ReverseDns(_) => { vec!(FieldFactory::domain("Domain name this node's IP address should resolve to".to_string())) },
                Test::Connectivity(_) => { vec!() },
                Test::VpnSubnetAccess(_) => { vec!() },
            }
        );
        fields
    }
}

impl TypeMetadataProvider for Test {
    fn id(&self) -> String { 
        self.variant_name()
    }
    
    /// Get display name for this test type
    fn display_name(&self) -> &str {
        match self {
            Test::Connectivity(_) => "Connectivity",
            Test::DnsResolution(_) => "DNS Resolution",
            Test::DnsLookup(_) => "DNS Lookup",
            Test::ReverseDns(_) => "Reverse DNS Lookup",
            Test::VpnSubnetAccess(_) => "VPN Subnet Access",
            Test::ServiceHealth(_) => "Service Health",
            // Test::DaemonCommand(_) => "Daemon Command",
            // Test::SshScript(_) => "SSH Script",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            Test::Connectivity(_) => "Connectivity",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::ReverseDns(_) => "DNS",
            Test::VpnSubnetAccess(_) => "VPN",
            Test::ServiceHealth(_) => "Service",
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            Test::Connectivity(_) => "Wifi",
            Test::VpnSubnetAccess(_) => "Shield",
            Test::ServiceHealth(_) => "Heart",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::ReverseDns(_) => "Search",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            Test::Connectivity(_) => "blue",
            Test::VpnSubnetAccess(_) => "orange",
            Test::ServiceHealth(_) => "green",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::ReverseDns(_) => "purple",
        }
    }

    fn description(&self) -> &str {
        match &self {
            Test::Connectivity(_) => "Test TCP connectivity to a target host and port",
            Test::DnsResolution(_) => "Test DNS name resolution capabilities",
            Test::DnsLookup(_) => "Test whether domain can be resolved to IP via DNS",
            Test::ReverseDns(_) => "Test whether IP can be resolved to domain via DNS",
            Test::VpnSubnetAccess(_) => "Test network accessibility to remote subnet via VPN routing",
            Test::ServiceHealth(_) => "Test HTTP/HTTPS service health and response",
            // Test::DaemonCommand(_) => "Execute system commands via daemon",
            // Test::SshScript(_) => "Execute commands via SSH connection",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        // Get default config for each test type
        let default_test = match self {
            Test::Connectivity(_) => Test::Connectivity(ConnectivityConfig::default()),
            Test::ServiceHealth(_) => Test::ServiceHealth(ServiceHealthConfig::default()),
            Test::DnsResolution(_) => Test::DnsResolution(DnsResolutionConfig::default()),
            Test::DnsLookup(_) => Test::DnsLookup(DnsLookupConfig::default()),
            Test::ReverseDns(_) => Test::ReverseDns(ReverseDnsConfig::default()),
            Test::VpnSubnetAccess(_) => Test::VpnSubnetAccess(VpnSubnetAccessConfig::default()),
        };
        
        serde_json::json!({
            "default_config": default_test,
        })
    }
}



