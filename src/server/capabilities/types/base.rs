use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumDiscriminants, EnumIter};
use strum::{IntoDiscriminant, IntoEnumIterator};
use crate::server::{capabilities::types::{configs::{CompatibleTests, ConfigBase, DaemonConfig, DhcpConfig, DnsConfig, FromPort, HttpConfig, HttpsConfig, NodeConfig, SshConfig, WireguardConfig}, forms::{CapabilityConfigForm, TestSection}}, nodes::types::{base::{NodeContext}, criticality::TestCriticality}, shared::{forms::{field_factory::FieldFactory, types::fields::ConfigField}, types::metadata::TypeMetadataProvider}, tests::types::base::{Test, TestDiscriminants}};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Display, Hash, Serialize, Deserialize, EnumIter))]
pub enum Capability {
    // Real service capabilities (removable: true)
    Http(HttpConfig),
    Https(HttpsConfig),
    Ssh(SshConfig),
    Dns(DnsConfig),
    Dhcp(DhcpConfig),

    Wireguard(WireguardConfig),
    
    // Special system capabilities (removable: false)
    Node(NodeConfig),     // For DnsLookup, ReverseDns, manual tests
    Daemon(DaemonConfig),  // For daemon-based tests
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct CapabilityTest {
    pub test: Test,
    pub criticality: TestCriticality,  
    pub enabled: bool,        
}

impl Capability {
    pub fn generate_form(&self, node_context: &NodeContext) -> CapabilityConfigForm {
        CapabilityConfigForm {
            capability_info: self.to_metadata(),
            capability_fields:self.generate_capability_fields(),
            test_sections: self.generate_test_sections(node_context),
            warnings: vec![],
            errors: vec![],
            system_assigned: self.is_system_assigned()
        }
    }

    fn is_system_assigned(&self) -> bool {
        self.config_base().system_assigned
    }

    fn generate_capability_fields(&self) -> Vec<ConfigField> {
        match self {
            Capability::Http(config) => { vec![FieldFactory::port(config.base.port), FieldFactory::path()] },
            Capability::Node(_) => { vec![] },
            Capability::Daemon(config) => { vec![FieldFactory::port(config.base.port)] },
            Capability::Https(config) => { vec![ FieldFactory::port(config.base.port), FieldFactory::path() ] },
            Capability::Dhcp(config) => { vec![FieldFactory::port(config.base.port)] },
            Capability::Ssh(config) => { vec![FieldFactory::port(config.base.port)] },
            Capability::Wireguard(config) => { vec![FieldFactory::port(config.base.port)] },
            Capability::Dns(config) => { vec![FieldFactory::port(config.base.port)] }
        }
    }

    pub fn get_compatible_tests(&self, node_context: &NodeContext) -> Vec<CapabilityTest> {
        match self {
            Capability::Http(_) => { HttpConfig::compatible_tests(Some(node_context)) },
            Capability::Node(_) => { NodeConfig::compatible_tests(Some(node_context)) },
            Capability::Daemon(_) => { DaemonConfig::compatible_tests(Some(node_context)) },
            Capability::Https(_) => { HttpsConfig::compatible_tests(Some(node_context)) },
            Capability::Dhcp(_) => { DhcpConfig::compatible_tests(Some(node_context)) },
            Capability::Ssh(_) => { SshConfig::compatible_tests(Some(node_context)) },
            Capability::Wireguard(_) => { WireguardConfig::compatible_tests(Some(node_context)) },
            Capability::Dns(_) => { DnsConfig::compatible_tests(Some(node_context)) }
        }
    }

    fn generate_test_sections(&self, node_context: &NodeContext) -> Vec<TestSection> {
        let compatible_tests = self.get_compatible_tests(&node_context);

        compatible_tests
            .iter()
            .map(|capability_test| {
                TestSection {
                    test_type: capability_test.test.discriminant(),
                    test_info: capability_test.test.to_metadata(),
                    test_fields: Test::generate_fields(&capability_test.test),
                }
            })
            .collect()
    }


    pub fn validate_node_capability_test_compatibility(&self, node_context: NodeContext) -> (Vec<CapabilityTest>, Vec<TestDiscriminants>) {
        
        let compatible_test_discriminants: Vec<TestDiscriminants> = self.get_compatible_tests(&node_context)
            .iter()
            .map(|ct| ct.test.discriminant())
            .collect();

        let existing_tests: Vec<TestDiscriminants> = self.config_base().tests
            .iter()
            .map(|test| test.test.discriminant())
            .collect();

        let newly_compatible: Vec<CapabilityTest> = self.get_compatible_tests(&node_context)
            .into_iter()
            .filter(|ct| !existing_tests.contains(&ct.test.discriminant()))
            .collect();

        let incompatible: Vec<TestDiscriminants> = self.config_base().tests
            .iter()
            .filter(|cap_test| !compatible_test_discriminants.contains(&cap_test.test.discriminant()))
            .map(|ct| ct.test.discriminant())
            .collect();

        (
            newly_compatible,
            incompatible
        )
    }

    pub fn from_port(port: u16) -> Option<Self> {
        CapabilityDiscriminants::iter()
            .find_map(|variant| {
                let default_capability = Self::default_for_discriminant(variant);

                match &default_capability.config_base().discovery_ports {
                    Some(discovery_ports) if discovery_ports.contains(&port) => Some(
                        match variant {
                                CapabilityDiscriminants::Http => Self::Http(HttpConfig::from_port(Some(port))),
                                CapabilityDiscriminants::Https => Self::Https(HttpsConfig::from_port(Some(port))),
                                CapabilityDiscriminants::Ssh => Self::Ssh(SshConfig::from_port(Some(port))),
                                CapabilityDiscriminants::Dns => Self::Dns(DnsConfig::from_port(Some(port))),
                                CapabilityDiscriminants::Dhcp => Self::Dhcp(DhcpConfig::from_port(Some(port))),
                                CapabilityDiscriminants::Wireguard => Self::Wireguard(WireguardConfig::from_port(Some(port))),
                                CapabilityDiscriminants::Node => Self::Node(NodeConfig::default()),
                                CapabilityDiscriminants::Daemon => Self::Daemon(DaemonConfig::default()),
                            }
                    ),
                    _ => None
                }
            })
    }
    
    pub fn default_for_discriminant(discriminant: CapabilityDiscriminants) -> Self {
        match discriminant {
            CapabilityDiscriminants::Http => Self::Http(HttpConfig::default()),
            CapabilityDiscriminants::Https => Self::Https(HttpsConfig::default()),
            CapabilityDiscriminants::Ssh => Self::Ssh(SshConfig::default()),
            CapabilityDiscriminants::Dns => Self::Dns(DnsConfig::default()),
            CapabilityDiscriminants::Dhcp => Self::Dhcp(DhcpConfig::default()),
            CapabilityDiscriminants::Wireguard => Self::Wireguard(WireguardConfig::default()),
            CapabilityDiscriminants::Node => Self::Node(NodeConfig::default()),
            CapabilityDiscriminants::Daemon => Self::Daemon(DaemonConfig::default()),
        }
    }

    pub fn discovery_ports() -> Vec<u16> {
        CapabilityDiscriminants::iter()
            .filter_map(|variant| {
                let capability = match variant {
                    CapabilityDiscriminants::Http => Self::Http(HttpConfig::default()),
                    CapabilityDiscriminants::Https => Self::Https(HttpsConfig::default()),
                    CapabilityDiscriminants::Ssh => Self::Ssh(SshConfig::default()),
                    CapabilityDiscriminants::Dns => Self::Dns(DnsConfig::default()),
                    CapabilityDiscriminants::Dhcp => Self::Dhcp(DhcpConfig::default()),
                    CapabilityDiscriminants::Wireguard => Self::Wireguard(WireguardConfig::default()),
                    CapabilityDiscriminants::Node => Self::Node(NodeConfig::default()),
                    CapabilityDiscriminants::Daemon => Self::Daemon(DaemonConfig::default()),
                };
                capability.config_base().discovery_ports.clone()
            })
            .flatten()
            .collect()
    }

    pub fn config_base(&self) -> &ConfigBase {
        match self {
            Capability::Ssh(config) => &config.base,
            Capability::Http(config) => &config.base,
            Capability::Https(config) => &config.base,
            Capability::Wireguard(config) => &config.base,
            Capability::Daemon(config) => &config.base,
            Capability::Dns(config) => &config.base,
            Capability::Dhcp(config) => &config.base,
            Capability::Node(config) => &config.base,
        }
    }

    pub fn config_base_mut(&mut self) -> &mut ConfigBase {
        match self {
            Capability::Ssh(config) => &mut config.base,
            Capability::Http(config) => &mut config.base,
            Capability::Https(config) => &mut config.base,
            Capability::Wireguard(config) => &mut config.base,
            Capability::Daemon(config) => &mut config.base,
            Capability::Dns(config) => &mut config.base,
            Capability::Dhcp(config) => &mut config.base,
            Capability::Node(config) => &mut config.base,
        }
    }
    
}

impl TypeMetadataProvider for Capability {
    fn id(&self) -> String { 
        self.discriminant().to_string()
    } 

    fn display_name(&self) -> &str {
        match self {
            Capability::Ssh{ .. } => "SSH",
            Capability::Http{ .. } => "HTTP",
            Capability::Https{ .. } => "HTTPS",
            Capability::Wireguard{ .. } => "Wireguard VPN",
            Capability::Daemon{ .. } => "NetVisor Daemon",
            Capability::Dns{ .. } => "DNS",
            Capability::Dhcp{ .. } => "DHCP",
            Capability::Node{ .. } => "Node"
        }
    }
    
    fn description(&self) -> &str {
        match self {
            Capability::Ssh { .. } => "Remote command-line access for management and troubleshooting",
            Capability::Http { .. } => "Web service providing HTTP content",
            Capability::Https { .. } => "Secure web service providing HTTPS content", 
            Capability::Dns { .. } => "Domain name resolution service",
            Capability::Wireguard { .. } => "Modern VPN service using WireGuard protocol",
            Capability::Daemon { .. } => "NetVisor daemon for enhanced network diagnostics",
            Capability::Dhcp{ .. } => "Dynamic host configuration protocol service",
            Capability::Node{ .. } => "Tests and settings which apply to the node as a whole, rather than any particular capability"
        }
    }
    
    fn category(&self) -> &str {
        match self {
            Capability::Ssh{ .. }  => "Remote Access",
            Capability::Http{ .. } | Capability::Https{ .. } => "Web Services",
            Capability::Wireguard{ .. } => "Security",
            Capability::Dns{ .. } | Capability::Dhcp{ .. } => "Network Infrastructure",
            Capability::Daemon { .. } | Capability::Node{ .. } => "NetVisor",
        }
    }
    
    fn icon(&self) -> &str {
        match self {
            Capability::Ssh{ .. }   => "Terminal",
            Capability::Http{ .. } | Capability::Https{ .. } => "Globe",
            Capability::Wireguard{ .. } => "Lock",
            Capability::Dns{ .. } => "Search",
            Capability::Daemon { .. } | Capability::Node{ .. } => "RectangleGoggles",
            Capability::Dhcp{ .. } => "Router"
        }
    }
    
    fn color(&self) -> &str {
        match self {
            Capability::Ssh{ .. }  => "green",
            Capability::Http{ .. } | Capability::Https{ .. } => "blue",
            Capability::Wireguard{ .. } => "orange",
            Capability::Dns{ .. } | Capability::Dhcp{ .. } => "yellow",
            Capability::Daemon { .. } |  Capability::Node{ .. } => "purple",
        }
    }
    
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}