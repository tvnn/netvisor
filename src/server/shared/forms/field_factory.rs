use serde_json::json;
use strum::IntoEnumIterator;

use crate::server::{capabilities::types::base::Capability, nodes::types::{base::Node, criticality::{TestCriticality}, targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget}}, shared::forms::types::fields::*};
use crate::server::shared::types::metadata::TypeMetadataProvider;

pub struct FieldFactory {}

impl FieldFactory {
    pub fn port(value: Option<u16>) -> ConfigField {
        ConfigField {
            id: "port".to_string(),
            label: "Port".to_string(),
            field_type: FieldType {
                base_type: "number".to_string(),
                constraints: json!({"min": 1, "max": 65535}),
                options: None,
            },
            required: true,
            default_value: Some(json!(value.unwrap_or(80))),
            help_text: Some("Port where service is running".to_string()),
            placeholder: Some("80".to_string()),
            advanced: false,
        }
    }

    pub fn path() -> ConfigField {
        ConfigField {
            id: "path".to_string(),
            label: "Path".to_string(),
            field_type: FieldType {
                base_type: "string".to_string(),
                constraints: json!({}),
                options: None,
            },
            required: true,
            default_value: Some(json!("/")),
            help_text: Some("URL path for HTTP requests".to_string()),
            placeholder: Some("/api/health".to_string()),
            advanced: false,
        }
    }

    pub fn timeout() -> ConfigField {
        ConfigField {
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
        }
    }

    pub fn domain(help_text: String) -> ConfigField {
        ConfigField {
            id: "domain".to_string(),
            label: "Domain".to_string(),
            field_type: FieldType {
                base_type: "string".to_string(),
                constraints: serde_json::json!({}),
                options: None,
            },
            required: true,
            default_value: Some(serde_json::json!("google.com")),
            help_text: Some(help_text),
            placeholder: Some("example.com".to_string()),
            advanced: false,
        }
    }

    pub fn ip(help_text: String) -> ConfigField {
        ConfigField {
            id: "expected_ip".to_string(),
            label: "IP Address".to_string(),
            field_type: FieldType {
                base_type: "string".to_string(),
                constraints: serde_json::json!({}),
                options: None,
            },
            required: true,
            default_value: None,
            help_text: Some(help_text),
            placeholder: Some("192.168.1.100".to_string()),
            advanced: false,
        }
    }

    pub fn http_status_code(help_text: String) -> ConfigField {
        ConfigField {
            id: "expected_status_code".to_string(),
            label: "Status Code".to_string(),
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
            help_text: Some(help_text),
            placeholder: Some("200".to_string()),
            advanced: false,
        }
    }

    pub fn dns_resolver_selection(available_nodes: &[Node]) -> (Option<ValidationMessage>, ConfigField) {
        let dns_capable_nodes: Vec<SelectOption> = available_nodes.iter()
                .filter(|node| node.base.capabilities.iter().any(|c| matches!(c, Capability::Dns{ .. })))
                .map(|node| {
                    let target_summary = match &node.base.target {
                        NodeTarget::IpAddress(IpAddressTargetConfig{ ip, .. }) => ip.to_string(),
                        NodeTarget::Hostname(HostnameTargetConfig{ hostname, .. })=> hostname.clone(),
                    };
                    
                    SelectOption {
                        value: node.id.to_string(),
                        label: node.base.name.clone(),
                        description: Some(format!("{} - {}", 
                            node.base.node_type.display_name(),
                            target_summary
                        )),
                        disabled: false,
                        metadata: None
                    }
                })
                .collect();
            
            let mut validation_message: Option<ValidationMessage> = None;

            if dns_capable_nodes.is_empty() {
                validation_message = Some(ValidationMessage {
                    message: "No DNS servers available. Create a node with DNS Service capability first.".to_string(),
                    field_id: Some("dns_resolver_node".to_string()),
                    severity: MessageSeverity::Warning,
                });
            }
        (
            validation_message,
            ConfigField {
                id: "dns_resolver_node".to_string(),
                label: "DNS Server".to_string(),
                default_value: Some(serde_json::json!(&dns_capable_nodes[0].value)),
                field_type: FieldType {
                    base_type: "rich_select".to_string(),
                    constraints: serde_json::json!({}),
                    options: Some(dns_capable_nodes),
                },
                required: true,
                help_text: Some("DNS server to use for resolving this node's domain".to_string()),
                placeholder: None,
                advanced: false,
            }
        )
    }

    pub fn criticality() -> ConfigField {
        ConfigField {
            id: "criticality".to_string(),
            label: "Criticality".to_string(),
            field_type: FieldType {
                base_type: "select".to_string(),
                constraints: json!({}),
                options: Some(TestCriticality::iter().map(|criticality| {
                    SelectOption {
                        value: criticality.display_name().to_string(),
                        label: criticality.display_name().to_string(),
                        description: Some(criticality.description().to_string()),
                        disabled: false,
                        metadata: None
                    }
                }).collect())
            },
            required: true,
            default_value: Some(json!(TestCriticality::Important.display_name())),
            help_text: Some("How test failures affect overall node status".to_string()),
            placeholder: None,
            advanced: false,
        }
    }
}