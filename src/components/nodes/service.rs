use anyhow::Result;
use std::sync::Arc;
use crate::core::{Node, NodeType, NodeCapability, AssignedTest, TestType, TestConfiguration, TestCriticality};
use crate::components::nodes::storage::NodeStorage;

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
        let now = chrono::Utc::now().to_rfc3339();
        node.created_at = now.clone();
        node.updated_at = now;

        // Auto-detect node type and capabilities if not set
        if node.node_type.is_none() {
            if let Some(_ip) = &node.ip {
                // TODO: Integrate with discovery service to detect open ports
                // For now, set as unknown
                node.node_type = Some(NodeType::UnknownDevice);
            }
        }

        // Set default capabilities based on node type
        if node.capabilities.is_empty() {
            if let Some(node_type) = &node.node_type {
                node.capabilities = node_type.default_capabilities();
            }
        }

        self.storage.create(&node).await?;
        Ok(node)
    }

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
        node.updated_at = chrono::Utc::now().to_rfc3339();
        self.storage.update(&node).await?;
        Ok(node)
    }

    /// Delete node
    pub async fn delete_node(&self, id: &str) -> Result<()> {
        self.storage.delete(id).await
    }

    /// Assign a test to a node
    pub async fn assign_test_to_node(
        &self,
        node_id: &str,
        test_type: TestType,
        config: TestConfiguration,
        criticality: TestCriticality,
        monitor_interval_minutes: Option<u32>,
    ) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        // Validate test compatibility
        if !test_type.is_compatible_with_node(&node) {
            return Err(anyhow::anyhow!(
                "Test {} is not compatible with node {} ({})",
                test_type.display_name(),
                node.name,
                node.node_type.as_ref().map(|t| t.display_name()).unwrap_or("Unknown")
            ));
        }

        // Remove existing test of same type
        node.assigned_tests.retain(|t| t.test_type != test_type);

        // Add new test
        let assigned_test = AssignedTest {
            test_type,
            test_config: config,
            monitor_interval_minutes,
            enabled: true,
            criticality,
        };

        node.assigned_tests.push(assigned_test);
        self.update_node(node).await?;

        Ok(())
    }

    /// Remove a test from a node
    pub async fn remove_test_from_node(&self, node_id: &str, test_type: &TestType) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        node.assigned_tests.retain(|t| &t.test_type != test_type);
        self.update_node(node).await?;

        Ok(())
    }

    /// Enable/disable monitoring for a node
    pub async fn set_monitoring_enabled(&self, node_id: &str, enabled: bool) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        node.monitoring_enabled = enabled;
        self.update_node(node).await?;

        Ok(())
    }

    /// Add node to a group
    pub async fn add_to_group(&self, node_id: &str, group_id: &str) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        if !node.node_groups.contains(&group_id.to_string()) {
            node.node_groups.push(group_id.to_string());
            self.update_node(node).await?;
        }

        Ok(())
    }

    /// Remove node from a group
    pub async fn remove_from_group(&self, node_id: &str, group_id: &str) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        node.node_groups.retain(|g| g != group_id);
        self.update_node(node).await?;

        Ok(())
    }

    /// Get nodes that have monitoring enabled
    pub async fn get_monitoring_nodes(&self) -> Result<Vec<Node>> {
        self.storage.get_monitoring_enabled().await
    }

    /// Get nodes in a specific group
    pub async fn get_nodes_in_group(&self, group_id: &str) -> Result<Vec<Node>> {
        self.storage.get_by_group(group_id).await
    }

    /// Update node type and auto-assign default capabilities
    pub async fn update_node_type(&self, node_id: &str, node_type: NodeType) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        node.node_type = Some(node_type.clone());
        
        // Add default capabilities (don't overwrite existing ones)
        let default_caps = node_type.default_capabilities();
        for cap in default_caps {
            if !node.capabilities.contains(&cap) {
                node.capabilities.push(cap);
            }
        }

        self.update_node(node).await?;
        Ok(())
    }

    /// Add capability to node
    pub async fn add_capability(&self, node_id: &str, capability: NodeCapability) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        if !node.capabilities.contains(&capability) {
            node.capabilities.push(capability);
            self.update_node(node).await?;
        }

        Ok(())
    }

    /// Remove capability from node
    pub async fn remove_capability(&self, node_id: &str, capability: &NodeCapability) -> Result<()> {
        let mut node = self.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        node.capabilities.retain(|c| c != capability);
        self.update_node(node).await?;

        Ok(())
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