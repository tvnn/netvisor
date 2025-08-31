use anyhow::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::capabilities::test_factory::CapabilityTestFactory;
use crate::server::nodes::types::base::{Node, NodeContext};
use crate::server::nodes::types::targets::{HostnameTargetConfig, IpAddressTargetConfig};
use crate::server::{capabilities::types::base::{CapabilityTest}, nodes::types::{targets::{NodeTarget}}};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ConfigBase {
    pub name: String,  // "API Server", "Node-Level Tests", "Daemon Service"
    removable: bool,  // false for Node and DaemonService capabilities
    pub tests: Vec<CapabilityTest>,
    pub port: Option<u16>,
    pub process: Option<String>,
    pub discovery_ports: Option<Vec<u16>>
}

pub trait HttpEndpointCompatible {
    fn as_endpoint(&self, target: &NodeTarget) -> Result<String, Error>;
}

pub trait FromPort {
    fn from_port(port: Option<u16>) -> Self;
}

pub trait CompatibleTests {
    fn compatible_tests(node_context: Option<&NodeContext>) -> Vec<CapabilityTest>;
}

// Special Node capability for node-level tests
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "Node".to_string(), 
                removable: false, 
                tests: Vec::new(), 
                port: None, 
                process: None,
                discovery_ports: None
            }
        }
    }
}

impl CompatibleTests for NodeConfig {
    fn compatible_tests(node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        if let Some(context) = node_context {
            return match context.target {
                NodeTarget::Hostname(..) => vec!(CapabilityTestFactory::reverse_dns()),
                NodeTarget::IpAddress(..) => vec!(CapabilityTestFactory::dns_lookup())
            }
        }
        Vec::new()
    }
}

impl NodeConfig {
    pub fn new(node: &Node) -> Self{
        let tests = NodeConfig::compatible_tests(Some(&node.as_context()));

        Self {
            base: ConfigBase { 
                name: "Node".to_string(), 
                removable: false, 
                tests, 
                port: None, 
                process: None,
                discovery_ports: None
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
    pub path: Option<String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "HTTP".to_string(), 
                removable: true, 
                tests: Vec::new(),
                port: Some(80), 
                process: None,
                discovery_ports: Some(vec!(80, 8080))
            },
            path: Some("/".to_string())
        }
    }
}

impl CompatibleTests for HttpConfig {
    fn compatible_tests(_node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        vec!(CapabilityTestFactory::connectivity(), CapabilityTestFactory::service_health())
    }
}

impl FromPort for HttpConfig {
    fn from_port(port: Option<u16>) -> Self {
        Self {
            base: ConfigBase {
                name: "HTTP".to_string(),
                removable: true,
                tests: HttpConfig::compatible_tests(None),
                port,
                process: None,
                discovery_ports: None
            },
            path: Some("/".to_string())
        }
    }
}

impl HttpEndpointCompatible for HttpConfig {
    fn as_endpoint(&self, target: &NodeTarget) -> Result<String, Error> {
        let port = match self.base.port {
            Some(p) => p,
            None => return Err(Error::msg("Selected capability does not have a port"))
        };

        let target = match target {
            NodeTarget::Hostname(HostnameTargetConfig{hostname}) => hostname.to_string(),
            NodeTarget::IpAddress(IpAddressTargetConfig{ip}) => ip.to_string()
        };

        let path_str = self.path.as_deref().unwrap_or("/");
        Ok(format!("http://{}:{}{}", target, port, path_str))
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct HttpsConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
    pub path: Option<String>,
}

impl Default for HttpsConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "HTTPS".to_string(), 
                removable: true, 
                tests: Vec::new(), 
                port: Some(443), 
                process: None,
                discovery_ports: Some(vec!(443, 8443))
            },
            path: Some("/".to_string())
        }
    }
}

impl CompatibleTests for HttpsConfig {
    fn compatible_tests(_node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        vec!(CapabilityTestFactory::connectivity(), CapabilityTestFactory::service_health())
    }
}

impl FromPort for HttpsConfig {
    fn from_port(port: Option<u16>) -> Self {
        Self {
            base: ConfigBase {
                name: "HTTPS".to_string(),
                removable: true,
                tests: HttpsConfig::compatible_tests(None),
                port,
                process: None,
                discovery_ports: None
            },
            path: Some("/".to_string())
        }
    }
}

impl HttpEndpointCompatible for HttpsConfig {
    fn as_endpoint(&self, target: &NodeTarget) -> Result<String, Error> {
        let port = match self.base.port {
            Some(p) => p,
            None => return Err(Error::msg("Selected capability does not have a port"))
        };

        let target = match target {
            NodeTarget::Hostname(HostnameTargetConfig{hostname}) => hostname.to_string(),
            NodeTarget::IpAddress(IpAddressTargetConfig{ip}) => ip.to_string()
        };

        let path_str = self.path.as_deref().unwrap_or("/");
        Ok(format!("https://{}:{}{}", target, port, path_str))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "SSH".to_string(), 
                removable: true, 
                tests: Vec::new(), 
                port: Some(22), 
                process: None,
                discovery_ports: Some(vec!(22))
            }
        }
    }
}

impl CompatibleTests for SshConfig {
    fn compatible_tests(_node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        vec!(CapabilityTestFactory::connectivity())
    }
}

impl FromPort for SshConfig {
    fn from_port(port: Option<u16>) -> Self {
        Self {
            base: ConfigBase {
                name: "SSH".to_string(),
                removable: true,
                tests: SshConfig::compatible_tests(None),
                port,
                process: None,
                discovery_ports: None
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "DNS".to_string(), 
                removable: true, 
                tests: Vec::new(), 
                port: Some(53), 
                process: None,
                discovery_ports: Some(vec!(53))
            }
        }
    }
}

impl CompatibleTests for DnsConfig {
    fn compatible_tests(_node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        vec!(CapabilityTestFactory::dns_resolution())
    }
}

impl FromPort for DnsConfig {
    fn from_port(port: Option<u16>) -> Self {
        Self {
            base: ConfigBase {
                name: "DNS".to_string(),
                removable: true,
                tests: DnsConfig::compatible_tests(None),
                port,
                process: None,
                discovery_ports: None
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct WireguardConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
    // pub subnets: Vec<IpCidr>
}

impl Default for WireguardConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "Wireguard".to_string(), 
                removable: true, 
                tests: Vec::new(), 
                port: Some(51820), 
                process: None,
                discovery_ports: Some(vec!(51820))
            },
            // subnets: Vec::new()
        }
    }
}

impl CompatibleTests for WireguardConfig {
    fn compatible_tests(_node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        Vec::new()
        // let vpn_subnet_test = vpn_subnet();
        // Needs subnet support + udp
    }
}

impl FromPort for WireguardConfig {
    fn from_port(port: Option<u16>) -> Self {
        Self {
            base: ConfigBase {
                name: "Wireguard".to_string(),
                removable: true,
                tests: WireguardConfig::compatible_tests(None),
                port,
                process: None,
                discovery_ports: None
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DhcpConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
}

impl Default for DhcpConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "DHCP".to_string(), 
                removable: true, 
                tests: Vec::new(), 
                port: Some(67), 
                process: None,
                discovery_ports: Some(vec!(67))
            }
        }
    }
}

impl CompatibleTests for DhcpConfig {
    fn compatible_tests(_node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        Vec::new()
    }
}

impl FromPort for DhcpConfig {
    fn from_port(port: Option<u16>) -> Self {
        Self {
            base: ConfigBase {
                name: "DHCP".to_string(),
                removable: true,
                tests: DhcpConfig::compatible_tests(None),
                port,
                process: None,
                discovery_ports: None
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    #[serde(flatten)]
    pub base: ConfigBase,
    pub daemon_id: Uuid
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            base: ConfigBase { 
                name: "NetVisor Daemon".to_string(), 
                removable: false, 
                tests: Vec::new(), 
                port: Some(5621), 
                process: None,
                discovery_ports: None
            },
            daemon_id: Uuid::nil()
        }
    }
}

impl CompatibleTests for DaemonConfig {
    fn compatible_tests(_node_context: Option<&NodeContext>) -> Vec<CapabilityTest> {
        Vec::new()
    }
}

impl DaemonConfig {
    pub fn new(_node: &Node, port: u16, daemon_id: Uuid) -> Self {
        Self {
            base: ConfigBase {
                name: "NetVisor Daemon".to_string(),
                removable: false,
                tests: DaemonConfig::compatible_tests(None),
                port: Some(port),
                process: None,
                discovery_ports: None
            },
            daemon_id
        }
    }
}

impl HttpEndpointCompatible for DaemonConfig {
    fn as_endpoint(&self, target: &NodeTarget) -> Result<String, Error> {
        let port = match self.base.port {
            Some(p) => p,
            None => return Err(Error::msg("Selected capability does not have a port"))
        };

        let target = match target {
            NodeTarget::Hostname(HostnameTargetConfig{hostname}) => hostname.to_string(),
            NodeTarget::IpAddress(IpAddressTargetConfig{ip}) => ip.to_string()
        };

        Ok(format!("http://{}:{}", target, port))
    }
}
