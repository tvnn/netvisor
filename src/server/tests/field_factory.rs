use crate::server::{nodes::types::{base::Node, capabilities::NodeCapability, targets::{HostnameTargetConfig, IpAddressTargetConfig, NodeTarget}}, tests::types::schema::*};
use crate::server::shared::types::metadata::TypeMetadataProvider;

pub fn generate_timeout_field() -> ConfigField {
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

pub fn generate_domain_to_resolve_field(help_text: String) -> ConfigField {
    ConfigField {
        id: "domain".to_string(),
        label: "Domain to Resolve".to_string(),
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

pub fn generate_expected_ip_field(help_text: String) -> ConfigField {
    ConfigField {
        id: "expected_ip".to_string(),
        label: "Expected IP Address".to_string(),
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

pub fn generate_capability_with_port_selection_field(node_context: &NodeContext) -> ConfigField {
    let capabilities: Vec<SelectOption> = node_context.capabilities.iter()
        .filter(|cap| cap.config().port.is_some())
        .map(|cap| {

            let port = cap.config().port.expect("Capabilities with None ports have been filtered out");
            
            SelectOption {
                value: cap.id(),
                label: cap.display_name().to_string(),
                description: Some(cap.description().to_string()),
                disabled: false,
                metadata: Some(serde_json::to_value( SelectMetadata {
                    tag: port.to_string()
                }).unwrap_or_default())
            }
        })
        .collect();

    ConfigField {
        id: "capability".to_string(),
        label: "Capabilities".to_string(),
        default_value: Some(serde_json::json!(&capabilities[0].value)),
        field_type: FieldType {
            base_type: "rich_select".to_string(),
            constraints: serde_json::json!({}),
            options: Some(capabilities),
        },
        required: true,
        help_text: Some("Select capability to determine port that test will target.".to_string()),
        placeholder: None,
        advanced: false,
    }
}

pub fn generate_capability_with_http_endpoint_selection_field(node_context: &NodeContext) -> ConfigField {
    let capabilities: Vec<SelectOption> = node_context.capabilities.iter()
        .filter(|cap| cap.build_http_endpoint(&node_context.target).is_ok())
        .map(|cap| {

            let port = cap.config().port.expect("Capabilities with None ports have been filtered out");
            // let protocol = cap.config().port.expect("Capabilities with None protocols have been filtered out");
            
            SelectOption {
                value: cap.id(),
                label: cap.display_name().to_string(),
                description: Some(cap.description().to_string()),
                disabled: false,
                metadata: Some(serde_json::to_value( SelectMetadata {
                    tag: port.to_string()
                }).unwrap_or_default())
            }
        })
        .collect();

    ConfigField {
        id: "capability".to_string(),
        label: "Capabilities".to_string(),
        default_value: Some(serde_json::json!(&capabilities[0].value)),
        field_type: FieldType {
            base_type: "rich_select".to_string(),
            constraints: serde_json::json!({}),
            options: Some(capabilities),
        },
        required: true,
        help_text: Some("Select capability to determine port that test will target.".to_string()),
        placeholder: None,
        advanced: false,
    }
}

pub fn generate_dns_resolver_selection_field(available_nodes: &[Node]) -> (Option<ValidationMessage>, ConfigField) {
    let dns_capable_nodes: Vec<SelectOption> = available_nodes.iter()
            .filter(|node| node.base.capabilities.iter().any(|c| matches!(c, NodeCapability::DnsService{ .. })))
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