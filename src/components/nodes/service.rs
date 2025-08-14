use anyhow::Result;
use std::sync::Arc;
use crate::components::{
    nodes::{
        storage::NodeStorage,
        types::{Node, NodeType}
    },
    tests::types::{TestType}
};

pub struct NodeService {
    storage: Arc<dyn NodeStorage>,
}

impl NodeService {
    pub fn new(storage: Arc<dyn NodeStorage>) -> Self {
        Self { storage }
    }

    /// Create a new node
    pub async fn create_node(&self, mut node: Node) -> Result<Node> {
        // Generate ID and timestamps
        node.id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        node.created_at = now.clone();
        node.updated_at = now;

        // Auto-detect node type and capabilities if not set
        if node.base.node_type.is_none() {
            if let Some(_ip) = &node.base.ip {
                // TODO: Integrate with discovery service to detect open ports
                // For now, set as unknown
                node.base.node_type = Some(NodeType::UnknownDevice);
            }
        }

        // Set default capabilities based on node type
        if node.base.capabilities.is_empty() {
            if let Some(node_type) = &node.base.node_type {
                node.base.capabilities = node_type.typical_capabilities();
            }
        }

        self.storage.create(&node).await?;
        Ok(node)
    }

    /// Auto-detect and assign node type + capabilities from discovery results
    // pub async fn auto_detect_from_discovery(
    //     &self, 
    //     node_id: &str, 
    //     discovered_ports: &[u16]
    // ) -> Result<Node> {
    //     let mut node = self.get_node(node_id).await?
    //         .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

    //     // 1. Detect capabilities from open ports
    //     let detected_capabilities: Vec<NodeCapability> = discovered_ports
    //         .iter()
    //         .filter_map(|&port| NodeCapability::from_port(port))
    //         .collect();

    //     // 2. Auto-detect node type from ports
    //     let detected_type = NodeType::detect_from_open_ports(discovered_ports);

    //     // 3. Update node if detection provides better info than current
    //     if node.base.node_type.is_none() || node.base.node_type == Some(NodeType::UnknownDevice) {
    //         node.base.node_type = Some(detected_type.clone());
    //     }

    //     // 4. Add detected capabilities (don't remove existing ones)
    //     for capability in detected_capabilities {
    //         if !node.base.capabilities.contains(&capability) {
    //             node.base.capabilities.push(capability);
    //         }
    //     }

    //     // 5. If no capabilities detected, use typical ones for the node type
    //     if node.base.capabilities.is_empty() {
    //         if let Some(node_type) = &node.base.node_type {
    //             node.base.capabilities = node_type.typical_capabilities();
    //         }
    //     }

    //     self.update_node(node).await
    // }

    /// Get node by ID
    pub async fn get_node(&self, id: &str) -> Result<Option<Node>> {
        self.storage.get_by_id(id).await
    }

    /// Get all nodes
    pub async fn get_all_nodes(&self) -> Result<Vec<Node>> {
        self.storage.get_all().await
    }

    /// Update node
    pub async fn update_node(&self, mut node: Node) -> Result<Node> {
        node.updated_at = chrono::Utc::now();
        self.storage.update(&node).await?;
        Ok(node)
    }

    /// Delete node
    pub async fn delete_node(&self, id: &str) -> Result<()> {
        self.storage.delete(id).await
    }

    /// Get compatible test types for a node
    pub fn get_compatible_test_types(&self, node: &Node) -> Vec<TestType> {
        let all_test_types = vec![
            TestType::Connectivity,
            TestType::DirectIp,
            TestType::Ping,
            TestType::WellknownIp,
            TestType::DnsResolution,
            TestType::DnsOverHttps,
            TestType::VpnConnectivity,
            TestType::VpnTunnel,
            TestType::ServiceHealth,
            TestType::DaemonCommand,
            TestType::SshScript,
        ];

        all_test_types
            .into_iter()
            .filter(|test_type| test_type.is_compatible_with_node(node))
            .collect()
    }
}