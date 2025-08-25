use anyhow::Error;
use serde::{Deserialize, Serialize};
use crate::components::nodes::capabilities::base::NodeCapability;
use crate::components::nodes::capabilities::{
    http::*,
    dns::*,
    vpn::*,
};
use crate::components::nodes::service::NodeService;
use crate::components::tests::types::field_factory::{generate_timeout_field};
use crate::components::{
    nodes::types::{
        base::Node, 
        targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget, ServiceTargetConfig}}, 
    tests::{
        implementations::*, 
        types::{configs::*, execution::*, field_factory::{generate_dns_resolver_selection_field, generate_domain_to_resolve_field, generate_expected_ip_field}}}};
use strum_macros::{EnumIter, EnumDiscriminants, Display};
use crate::shared::metadata::{TypeMetadataProvider};
use crate::shared::{schema::*};

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
    DnsOverHttps(DnsOverHttpsConfig),
    ReverseDns(ReverseDnsConfig),
    
    // VPN-Specific Tests
    VpnSubnetAccess(VpnSubnetAccessConfig),
        
    // Remote tests
    // DaemonCommand(DaemonCommandConfig),
    // SshScript(SshScriptConfig),
}

impl Test {
    pub fn variant_name(&self) -> String {
        TestDiscriminants::from(self).to_string()
    }

    pub async fn execute(&self, timer: &Timer, node: &Node, node_service: &NodeService) -> Result<TestResult, Error> {
        match self {
            Test::Connectivity(config) => connectivity::execute_connectivity_test(config, &timer, &node, &node_service).await,
            Test::DnsResolution(config) => dns::execute_dns_resolution_test(config, &timer, &node, &node_service).await,
            Test::DnsOverHttps(config) => dns::execute_dns_over_https_test(config, &timer, &node, &node_service).await,
            Test::DnsLookup(config) => dns::execute_dns_lookup_test(config, &timer, &node, &node_service).await,
            Test::ReverseDns(config) => dns::execute_reverse_dns_lookup_test(config, &timer, &node, &node_service).await,
            Test::VpnSubnetAccess(config) => vpn::execute_vpn_subnet_access_test(config, &timer, &node, &node_service).await,
            Test::ServiceHealth(config) => connectivity::execute_service_health_test(config, &timer, &node, &node_service).await
        }
    }

    pub fn generate_schema(&self, context: &NodeContext, available_nodes: &[Node]) -> TestConfigSchema {
        let mut schema = TestConfigSchema {
            test_info: self.to_metadata(),
            contextual_description: self.generate_context_description_for_context(context),
            compatibility: CompatibilityStatus::Compatible,
            requirements_met: true,
            compatibility_reason: None,
            fields: vec![],
            warnings: vec![],
            errors: vec![],
        };
        
        self.assess_compatibility(context, &mut schema);
        self.generate_fields(context, &mut schema, available_nodes);
        self.check_node_configuration(context, &mut schema);
        
        schema
    }
    
    fn assess_compatibility(&self, context: &NodeContext, schema: &mut TestConfigSchema) {
        match self {
            Test::ServiceHealth(_) => {
                let has_http_capability = context.capabilities.iter()
                    .any(|cap| matches!(cap, NodeCapability::HttpService(HttpServiceCapability {  }) | NodeCapability::HttpsService(HttpsServiceCapability {  })));
                
                let has_service_target = matches!(context.target, NodeTarget::Service { .. });
                
                if !has_http_capability {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("ServiceHealth test requires HTTP or HTTPS capability".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Add HTTP Service or HTTPS Service capability to this node".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                } else if !has_service_target {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("ServiceHealth test requires Service target configuration".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Configure this node with a Service target (hostname, protocol, path)".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                }
            },
            
            Test::DnsResolution(_) => {
                let has_dns_capability = context.capabilities.contains(&NodeCapability::DnsService(DnsServiceCapability {  }));
                
                if !has_dns_capability {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("DNS Resolution test requires DNS service capability".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Add DNS Service capability to this node".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                }
            },
            
            Test::DnsLookup(_) => {
                // Requires hostname-based target
                let has_hostname_target = matches!(context.target, 
                    NodeTarget::Service { .. } | NodeTarget::Hostname { .. }
                );
                
                if !has_hostname_target {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("DNS Lookup test requires hostname-based target".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Configure this node with a Hostname or Service target to enable DNS lookup testing".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                }
            },
            
            Test::ReverseDns(_) => {
                // Requires IP-based target
                let has_ip_target = matches!(context.target, NodeTarget::IpAddress { .. });
                
                if !has_ip_target {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("Reverse DNS test requires IP address target".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Configure this node with an IP Address target to enable reverse DNS testing".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                }
            },
            
            Test::DnsOverHttps(_) => {
                let has_dns_capability = context.capabilities.contains(&NodeCapability::DnsService(DnsServiceCapability {  }));
                let has_service_target = matches!(context.target, NodeTarget::Service { .. });
                
                if !has_dns_capability {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("DNS over HTTPS test requires DNS service capability".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Add DNS Service capability to this node".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                } else if !has_service_target {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("DNS over HTTPS test requires Service target for HTTPS endpoint".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Configure this node with a Service target (HTTPS endpoint for DoH)".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                }
            },
            
            Test::VpnSubnetAccess(_) => {
                let has_vpn_capability = context.capabilities.contains(&NodeCapability::VpnService(VpnServiceCapability {  }));
                
                if !has_vpn_capability {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("VPN tests require VPN service capability".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Add VPN Service capability to this node".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                }
            },
            
            // Basic connectivity works on any node with any target
            Test::Connectivity(_) => {
                schema.compatibility = CompatibilityStatus::Compatible;
            },
        }
    }
    
    fn generate_fields(&self, context: &NodeContext, schema: &mut TestConfigSchema, available_nodes: &[Node]) {
        // Common timeout field for all tests
        schema.fields.push(generate_timeout_field());
        
        match self {
            Test::ServiceHealth(_) => {
                schema.fields.push(ConfigField {
                    id: "expected_status_code".to_string(),
                    label: "Expected Status Code".to_string(),
                    field_type: FieldType {
                        base_type: "integer".to_string(),
                        constraints: serde_json::json!({
                            "min": 100,
                            "max": 599
                        }),
                        options: None,
                    },
                    required: true,
                    default_value: Some(serde_json::json!(200)),
                    help_text: Some("HTTP status code the service should return (200, 204, 404, etc.)".to_string()),
                    placeholder: Some("200".to_string()),
                    advanced: false,
                });
            },
            
            Test::DnsResolution(_) => {
                schema.fields.push(generate_domain_to_resolve_field("Domain name to resolve using this DNS server".to_string()));
                schema.fields.push(generate_expected_ip_field("IP address the domain should resolve to".to_string()));
            },
            
            Test::DnsLookup(_) => {
                // DNS Resolver Selection
                let (validation_message, dns_resolver_field) = generate_dns_resolver_selection_field(available_nodes);

                match validation_message {
                    Some(message) => schema.warnings.push(message),
                    _ => ()
                }

                schema.fields.push(dns_resolver_field);
                schema.fields.push(generate_expected_ip_field("IP address this node's domain should resolve to".to_string()));
            },
            
            Test::DnsOverHttps(_) => {
                schema.fields.push(generate_domain_to_resolve_field("Domain name to resolve using DNS over HTTPS".to_string()));
                schema.fields.push(generate_expected_ip_field("IP address the domain should resolve to".to_string()));
            },
            
            Test::ReverseDns(_) => {

                let (validation_message, dns_resolver_field) = generate_dns_resolver_selection_field(available_nodes);

                match validation_message {
                    Some(message) => schema.warnings.push(message),
                    _ => ()
                }

                schema.fields.push(dns_resolver_field);

                schema.fields.push(ConfigField {
                    id: "expected_domain".to_string(),
                    label: "Expected Domain".to_string(),
                    field_type: FieldType {
                        base_type: "string".to_string(),
                        constraints: serde_json::json!({}),
                        options: None,
                    },
                    required: true,
                    default_value: None,
                    help_text: Some("Domain name this IP address should resolve to".to_string()),
                    placeholder: Some("example.com".to_string()),
                    advanced: false,
                });
            },
            
            Test::Connectivity(_) => {
                // Port field for targets that don't already specify port
                let port_specified = match &context.target {
                    NodeTarget::IpAddress(IpAddressTargetConfig{ port: Some(_), .. }) => true,
                    NodeTarget::Service(ServiceTargetConfig{ port: Some(_), .. }) => true,
                    NodeTarget::Hostname(HostnameTargetConfig{ port: Some(_), .. }) => true,
                    _ => false,
                };
                
                if !port_specified {
                    schema.fields.push(ConfigField {
                        id: "port".to_string(),
                        label: "Target Port".to_string(),
                        field_type: FieldType {
                            base_type: "integer".to_string(),
                            constraints: serde_json::json!({
                                "min": 1,
                                "max": 65535
                            }),
                            options: None,
                        },
                        required: true,
                        default_value: None,
                        help_text: Some("Network port to test connectivity to".to_string()),
                        placeholder: Some("80".to_string()),
                        advanced: false,
                    });
                }

                let needs_dns_resolution = match &context.target {
                    NodeTarget::IpAddress(IpAddressTargetConfig{ .. }) => false,
                    NodeTarget::Service(ServiceTargetConfig{  .. }) => true,
                    NodeTarget::Hostname(HostnameTargetConfig{ .. }) => true,
                };

                if needs_dns_resolution {                
                    let (validation_message, dns_resolver_field) = generate_dns_resolver_selection_field(available_nodes);

                    match validation_message {
                        Some(message) => schema.warnings.push(message),
                        _ => ()
                    }

                    schema.fields.push(dns_resolver_field);
                }
            },
            
            Test::VpnSubnetAccess(_) => {
                
                let needs_dns_resolution = match &context.target {
                    NodeTarget::IpAddress(IpAddressTargetConfig{ .. }) => false,
                    NodeTarget::Service(ServiceTargetConfig{  .. }) => true,
                    NodeTarget::Hostname(HostnameTargetConfig{ .. }) => true,
                };

                if needs_dns_resolution {                
                    let (validation_message, dns_resolver_field) = generate_dns_resolver_selection_field(available_nodes);

                    match validation_message {
                        Some(message) => schema.warnings.push(message),
                        _ => ()
                    }

                    schema.fields.push(dns_resolver_field);
                }

                schema.fields.push(ConfigField {
                    id: "expected_subnet".to_string(),
                    label: "Expected VPN Subnet".to_string(),
                    field_type: FieldType {
                        base_type: "string".to_string(),
                        constraints: serde_json::json!({}),
                        options: None,
                    },
                    required: true,
                    default_value: Some(serde_json::json!("10.100.0.0/24")),
                    help_text: Some("VPN subnet that should be accessible through this tunnel".to_string()),
                    placeholder: Some("10.100.0.0/24".to_string()),
                    advanced: false,
                });
            },
        }
    }
    
    fn check_node_configuration(&self, context: &NodeContext, schema: &mut TestConfigSchema) {
        // Only add warnings for optional improvements, not blocking issues
        // Blocking issues are handled in assess_compatibility
        
        match self {
            Test::Connectivity(_) => {
                // Suggest adding port to node target for convenience
                let needs_port = match &context.target {
                    NodeTarget::IpAddress(IpAddressTargetConfig{ port: None, .. }) => true,
                    NodeTarget::Service(ServiceTargetConfig{ port: None, .. }) => true,
                    NodeTarget::Hostname(HostnameTargetConfig{ port: None, .. }) => true,
                    _ => false,
                };
                
                if needs_port {
                    schema.warnings.push(ValidationMessage {
                        message: "Consider adding a port to this node's target configuration to avoid specifying it for each test".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Info,
                    });
                }
            },
            _ => {}
        }
        
        // Set requirements_met based on compatibility and field availability
        let missing_required_fields = schema.fields.iter()
            .filter(|field| field.required && field.default_value.is_none())
            .count();
            
        schema.requirements_met = missing_required_fields == 0 && schema.compatibility == CompatibilityStatus::Compatible;
    }
    
    fn generate_context_description_for_context(&self, context: &NodeContext) -> String {
        let node_type_name = context.node_type.display_name();
        
        match self {
            Test::Connectivity(_) => format!("Can we connect to your {}?", node_type_name),
            Test::DnsResolution(_) => format!("Can your {} resolve DNS queries?", node_type_name),
            Test::DnsLookup(_) => format!("Can your {} domain be resolved to the correct IP?", node_type_name),
            Test::DnsOverHttps(_) => format!("Can your {} resolve DNS over HTTPS?", node_type_name),
            Test::ReverseDns(_) => format!("Can your {} IP be resolved to the correct domain?", node_type_name),
            Test::VpnSubnetAccess(_) => format!("Is your {} tunnel working correctly?", node_type_name),
            Test::ServiceHealth(_) => format!("Is your {} responding properly?", node_type_name),
        }
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
            Test::DnsOverHttps(_) => "DNS over HTTPS",
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
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::DnsOverHttps(_) | Test::ReverseDns(_) => "DNS",
            Test::VpnSubnetAccess(_) => "VPN",
            Test::ServiceHealth(_) => "Service",
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            Test::Connectivity(_) => "Wifi",
            Test::VpnSubnetAccess(_) => "Shield",
            Test::ServiceHealth(_) => "Heart",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::DnsOverHttps(_) | Test::ReverseDns(_) => "Search",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            Test::Connectivity(_) => "blue",
            Test::VpnSubnetAccess(_) => "orange",
            Test::ServiceHealth(_) => "green",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::DnsOverHttps(_) | Test::ReverseDns(_) => "purple",
        }
    }

    fn description(&self) -> &str {
        match &self {
            Test::Connectivity(_) => "Test TCP connectivity to a target host and port",
            Test::DnsResolution(_) => "Test DNS name resolution capabilities",
            Test::DnsOverHttps(_) => "Test DNS resolution using DNS over HTTPS",
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
            Test::DnsOverHttps(_) => Test::DnsOverHttps(DnsOverHttpsConfig::default()),
            Test::ReverseDns(_) => Test::ReverseDns(ReverseDnsConfig::default()),
            Test::VpnSubnetAccess(_) => Test::VpnSubnetAccess(VpnSubnetAccessConfig::default()),
        };
        
        serde_json::json!({
            "default_config": default_test,
        })
    }
}



