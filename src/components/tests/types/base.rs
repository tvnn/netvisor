use anyhow::Error;
use serde::{Deserialize, Serialize};
use crate::components::{nodes::types::{base::Node, capabilities::NodeCapability, targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget, ServiceTargetConfig}}, tests::{implementations::*, types::{configs::*, execution::*}}};
use strum_macros::{EnumIter, EnumDiscriminants, Display};
use crate::shared::metadata::{TypeMetadataProvider};
use crate::shared::{schema::*};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, EnumIter))]
#[serde(tag="type", content="config")]
pub enum Test {
    // Basic Connectivity Tests
    Connectivity(ConnectivityConfig),
    DirectIp(DirectIpConfig),
    Ping(PingConfig),
    
    // Service-Specific Tests
    ServiceHealth(ServiceHealthConfig),
    DnsResolution(DnsResolutionConfig),
    DnsLookup(DnsLookupConfig),
    DnsOverHttps(DnsOverHttpsConfig),
    ReverseDns(ReverseDnsConfig),
    
    // VPN-Specific Tests
    VpnConnectivity(VpnConnectivityConfig),
    VpnTunnel(VpnTunnelConfig),
        
    // Remote tests
    // DaemonCommand(DaemonCommandConfig),
    // SshScript(SshScriptConfig),
}

impl Test {
    pub fn variant_name(&self) -> String {
        TestDiscriminants::from(self).to_string()
    }

    pub async fn execute(&self, timer: &Timer, node: &Node) -> Result<TestResult, Error> {
        match self {
            Test::Connectivity(config) => connectivity::execute_connectivity_test(config, &timer, &node).await,
            Test::DirectIp(config) => connectivity::execute_direct_ip_test(config, &timer, &node).await,
            Test::Ping(config) => connectivity::execute_ping_test(config, &timer, &node).await,
            Test::DnsResolution(config) => dns::execute_dns_resolution_test(config, &timer, &node).await,
            Test::DnsOverHttps(config) => dns::execute_dns_over_https_test(config, &timer, &node).await,
            Test::DnsLookup(config) => dns::execute_dns_lookup_test(config, &timer, &node).await,
            Test::ReverseDns(config) => dns::execute_reverse_dns_lookup_test(config, &timer, &node).await,
            Test::VpnConnectivity(config) => vpn::execute_vpn_connectivity_test(config, &timer, &node).await,
            Test::VpnTunnel(config) => vpn::execute_vpn_tunnel_test(config, &timer, &node).await,
            Test::ServiceHealth(config) => service::execute_service_health_test(config, &timer, &node).await
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
                    .any(|cap| matches!(cap, NodeCapability::HttpService | NodeCapability::HttpsService));
                
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
                let has_dns_capability = context.capabilities.contains(&NodeCapability::DnsService);
                
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
                let has_dns_capability = context.capabilities.contains(&NodeCapability::DnsService);
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
            
            Test::VpnTunnel(_) | Test::VpnConnectivity(_) => {
                let has_vpn_capability = context.capabilities.contains(&NodeCapability::VpnService);
                
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
            
            Test::DirectIp(_) => {
                // Requires IP-based target
                let has_ip_target = matches!(context.target, NodeTarget::IpAddress { .. });
                
                if !has_ip_target {
                    schema.compatibility = CompatibilityStatus::Incompatible;
                    schema.compatibility_reason = Some("Direct IP test requires IP address target".to_string());
                    schema.errors.push(ValidationMessage {
                        message: "Configure this node with an IP Address target to enable direct IP testing".to_string(),
                        field_id: None,
                        severity: MessageSeverity::Error,
                    });
                }
            },
            
            // Basic connectivity and ping tests work on any node with any target
            Test::Connectivity(_) | Test::Ping(_) => {
                schema.compatibility = CompatibilityStatus::Compatible;
            },
        }
    }
    
    fn generate_fields(&self, context: &NodeContext, schema: &mut TestConfigSchema, available_nodes: &[Node]) {
        // Common timeout field for all tests
        schema.fields.push(ConfigField {
            id: "timeout_ms".to_string(),
            label: "Timeout (milliseconds)".to_string(),
            field_type: FieldType {
                base_type: "integer".to_string(),
                constraints: serde_json::json!({
                    "min": 1000,
                    "max": 120000,
                    "step": 1000
                }),
                options: None,
            },
            required: false,
            default_value: Some(serde_json::json!(30000)),
            help_text: Some("Maximum time to wait for test completion".to_string()),
            placeholder: Some("30000".to_string()),
            advanced: true,
        });
        
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
                schema.fields.push(ConfigField {
                    id: "domain".to_string(),
                    label: "Domain to Resolve".to_string(),
                    field_type: FieldType {
                        base_type: "string".to_string(),
                        constraints: serde_json::json!({}),
                        options: None,
                    },
                    required: true,
                    default_value: Some(serde_json::json!("google.com")),
                    help_text: Some("Domain name to resolve using this DNS server".to_string()),
                    placeholder: Some("example.com".to_string()),
                    advanced: false,
                });
                
                schema.fields.push(ConfigField {
                    id: "expected_ip".to_string(),
                    label: "Expected IP Address".to_string(),
                    field_type: FieldType {
                        base_type: "string".to_string(),
                        constraints: serde_json::json!({}),
                        options: None,
                    },
                    required: true,
                    default_value: Some(serde_json::json!("8.8.8.8")),
                    help_text: Some("IP address the domain should resolve to".to_string()),
                    placeholder: Some("8.8.8.8".to_string()),
                    advanced: false,
                });
            },
            
            Test::DnsLookup(_) => {
                // DNS Resolver Selection
                let dns_capable_nodes: Vec<SelectOption> = available_nodes.iter()
                    .filter(|node| node.base.capabilities.contains(&NodeCapability::DnsService))
                    .map(|node| {
                        let target_summary = match &node.base.target {
                            NodeTarget::IpAddress(IpAddressTargetConfig{ ip, .. }) => ip.to_string(),
                            NodeTarget::Service(ServiceTargetConfig{ hostname, .. })=> hostname.clone(),
                            NodeTarget::Hostname(HostnameTargetConfig{ hostname, .. }) => hostname.clone(),
                        };
                        
                        SelectOption {
                            value: node.id.clone(),
                            label: node.base.name.clone(),
                            description: Some(format!("{} - {}", 
                                node.base.node_type.display_name(),
                                target_summary
                            )),
                            disabled: false,
                        }
                    })
                    .collect();
                
                if dns_capable_nodes.is_empty() {
                    schema.warnings.push(ValidationMessage {
                        message: "No DNS servers available. Create a node with DNS Service capability first.".to_string(),
                        field_id: Some("dns_resolver".to_string()),
                        severity: MessageSeverity::Warning,
                    });
                }
                
                schema.fields.push(ConfigField {
                    id: "dns_resolver".to_string(),
                    label: "DNS Server".to_string(),
                    field_type: FieldType {
                        base_type: "node_selector".to_string(),
                        constraints: serde_json::json!({
                            "filter_capabilities": ["DnsService"]
                        }),
                        options: Some(dns_capable_nodes),
                    },
                    required: true,
                    default_value: None,
                    help_text: Some("DNS server to use for resolving this node's domain".to_string()),
                    placeholder: None,
                    advanced: false,
                });
                
                schema.fields.push(ConfigField {
                    id: "expected_ip".to_string(),
                    label: "Expected IP Address".to_string(),
                    field_type: FieldType {
                        base_type: "string".to_string(),
                        constraints: serde_json::json!({}),
                        options: None,
                    },
                    required: true,
                    default_value: None,
                    help_text: Some("IP address this node's domain should resolve to".to_string()),
                    placeholder: Some("192.168.1.100".to_string()),
                    advanced: false,
                });
            },
            
            Test::DnsOverHttps(_) => {
                schema.fields.push(ConfigField {
                    id: "domain".to_string(),
                    label: "Domain to Resolve".to_string(),
                    field_type: FieldType {
                        base_type: "string".to_string(),
                        constraints: serde_json::json!({}),
                        options: None,
                    },
                    required: true,
                    default_value: Some(serde_json::json!("google.com")),
                    help_text: Some("Domain name to resolve using DNS over HTTPS".to_string()),
                    placeholder: Some("example.com".to_string()),
                    advanced: false,
                });
                
                schema.fields.push(ConfigField {
                    id: "expected_ip".to_string(),
                    label: "Expected IP Address".to_string(),
                    field_type: FieldType {
                        base_type: "string".to_string(),
                        constraints: serde_json::json!({}),
                        options: None,
                    },
                    required: true,
                    default_value: Some(serde_json::json!("8.8.8.8")),
                    help_text: Some("IP address the domain should resolve to".to_string()),
                    placeholder: Some("8.8.8.8".to_string()),
                    advanced: false,
                });
            },
            
            Test::ReverseDns(_) => {
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
                
                // Protocol selection
                schema.fields.push(ConfigField {
                    id: "protocol".to_string(),
                    label: "Protocol".to_string(),
                    field_type: FieldType {
                        base_type: "select".to_string(),
                        constraints: serde_json::json!({}),
                        options: Some(vec![
                            SelectOption {
                                value: "tcp".to_string(),
                                label: "TCP".to_string(),
                                description: Some("Transmission Control Protocol".to_string()),
                                disabled: false,
                            },
                            SelectOption {
                                value: "udp".to_string(),
                                label: "UDP".to_string(),
                                description: Some("User Datagram Protocol".to_string()),
                                disabled: false,
                            },
                        ]),
                    },
                    required: false,
                    default_value: Some(serde_json::json!("tcp")),
                    help_text: Some("Network protocol to use for connectivity test".to_string()),
                    placeholder: None,
                    advanced: false,
                });
            },
            
            Test::DirectIp(_) => {
                // Protocol selection (same as connectivity)
                schema.fields.push(ConfigField {
                    id: "protocol".to_string(),
                    label: "Protocol".to_string(),
                    field_type: FieldType {
                        base_type: "select".to_string(),
                        constraints: serde_json::json!({}),
                        options: Some(vec![
                            SelectOption {
                                value: "tcp".to_string(),
                                label: "TCP".to_string(),
                                description: Some("Transmission Control Protocol".to_string()),
                                disabled: false,
                            },
                            SelectOption {
                                value: "udp".to_string(),
                                label: "UDP".to_string(),
                                description: Some("User Datagram Protocol".to_string()),
                                disabled: false,
                            },
                        ]),
                    },
                    required: false,
                    default_value: Some(serde_json::json!("tcp")),
                    help_text: Some("Network protocol to use for direct IP test".to_string()),
                    placeholder: None,
                    advanced: false,
                });
            },
            
            Test::Ping(_) => {
                schema.fields.push(ConfigField {
                    id: "packet_count".to_string(),
                    label: "Packet Count".to_string(),
                    field_type: FieldType {
                        base_type: "integer".to_string(),
                        constraints: serde_json::json!({
                            "min": 1,
                            "max": 20
                        }),
                        options: None,
                    },
                    required: false,
                    default_value: Some(serde_json::json!(4)),
                    help_text: Some("Number of ping packets to send".to_string()),
                    placeholder: Some("4".to_string()),
                    advanced: false,
                });
            },
            
            Test::VpnTunnel(_) => {
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
            
            Test::VpnConnectivity(_) => {
                // No additional fields beyond timeout - just tests basic VPN server reachability
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
            Test::DirectIp(_) => format!("Can we reach your {} directly by IP?", node_type_name),
            Test::Ping(_) => format!("Can we ping your {}?", node_type_name),
            Test::DnsResolution(_) => format!("Can your {} resolve DNS queries?", node_type_name),
            Test::DnsLookup(_) => format!("Can your {} domain be resolved to the correct IP?", node_type_name),
            Test::DnsOverHttps(_) => format!("Can your {} resolve DNS over HTTPS?", node_type_name),
            Test::ReverseDns(_) => format!("Can your {} IP be resolved to the correct domain?", node_type_name),
            Test::VpnConnectivity(_) => format!("Can we reach your {}?", node_type_name),
            Test::VpnTunnel(_) => format!("Is your {} tunnel working correctly?", node_type_name),
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
            Test::DirectIp(_) => "Direct IP",
            Test::Ping(_) => "Ping",
            Test::DnsResolution(_) => "DNS Resolution",
            Test::DnsOverHttps(_) => "DNS over HTTPS",
            Test::DnsLookup(_) => "DNS Lookup",
            Test::ReverseDns(_) => "Reverse DNS Lookup",
            Test::VpnConnectivity(_) => "VPN Connectivity",
            Test::VpnTunnel(_) => "VPN Tunnel",
            Test::ServiceHealth(_) => "Service Health",
            // Test::DaemonCommand(_) => "Daemon Command",
            // Test::SshScript(_) => "SSH Script",
        }
    }
    
    fn category(&self) -> &str {
        match self {
            Test::Connectivity(_) | Test::DirectIp(_) | Test::Ping(_) => "Connectivity",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::DnsOverHttps(_) | Test::ReverseDns(_) => "DNS",
            Test::VpnTunnel(_) | Test::VpnConnectivity(_) => "VPN",
            Test::ServiceHealth(_) => "Service",
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            Test::Connectivity(_) | Test::DirectIp(_) => "Wifi",
            Test::Ping(_) => "Radio",
            Test::VpnTunnel(_) | Test::VpnConnectivity(_) => "Shield",
            Test::ServiceHealth(_) => "Heart",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::DnsOverHttps(_) | Test::ReverseDns(_) => "Search",
        }
    }
    
    fn color(&self) -> &str {
        match self {
            Test::Connectivity(_) | Test::DirectIp(_) | Test::Ping(_) => "blue",
            Test::VpnTunnel(_) | Test::VpnConnectivity(_) => "orange",
            Test::ServiceHealth(_) => "green",
            Test::DnsResolution(_) | Test::DnsLookup(_) | Test::DnsOverHttps(_) | Test::ReverseDns(_) => "purple",
        }
    }

    fn description(&self) -> &str {
        match &self {
            Test::Connectivity(_) => "Test TCP connectivity to a target host and port",
            Test::DirectIp(_) => "Test direct IP connectivity bypassing DNS resolution",
            Test::Ping(_) => "Test network reachability using ICMP ping",
            Test::DnsResolution(_) => "Test DNS name resolution capabilities",
            Test::DnsOverHttps(_) => "Test DNS resolution using DNS over HTTPS",
            Test::DnsLookup(_) => "Test whether domain can be resolved to IP via DNS",
            Test::ReverseDns(_) => "Test whether IP can be resolved to domain via DNS",
            Test::VpnConnectivity(_) => "Test VPN server reachability and connection",
            Test::VpnTunnel(_) => "Test VPN tunnel functionality and subnet access",
            Test::ServiceHealth(_) => "Test HTTP/HTTPS service health and response",
            // Test::DaemonCommand(_) => "Execute system commands via daemon",
            // Test::SshScript(_) => "Execute commands via SSH connection",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        // Get default config for each test type
        let default_test = match self {
            Test::Connectivity(_) => Test::Connectivity(ConnectivityConfig::default()),
            Test::DirectIp(_) => Test::DirectIp(DirectIpConfig::default()),
            Test::Ping(_) => Test::Ping(PingConfig::default()),
            Test::ServiceHealth(_) => Test::ServiceHealth(ServiceHealthConfig::default()),
            Test::DnsResolution(_) => Test::DnsResolution(DnsResolutionConfig::default()),
            Test::DnsLookup(_) => Test::DnsLookup(DnsLookupConfig::default()),
            Test::DnsOverHttps(_) => Test::DnsOverHttps(DnsOverHttpsConfig::default()),
            Test::ReverseDns(_) => Test::ReverseDns(ReverseDnsConfig::default()),
            Test::VpnConnectivity(_) => Test::VpnConnectivity(VpnConnectivityConfig::default()),
            Test::VpnTunnel(_) => Test::VpnTunnel(VpnTunnelConfig::default()),
        };
        
        serde_json::json!({
            "default_config": default_test,
        })
    }
}



