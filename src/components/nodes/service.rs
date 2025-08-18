use anyhow::Result;
use std::{sync::Arc};
use crate::components::{
    node_groups::storage::NodeGroupStorage, nodes::{
        storage::NodeStorage,
        types::{
            base::Node, 
            tests::NodeTestResults,
            types_capabilities::NodeType
        }
    }, tests::{service::TestService, types::{TestResult, Timer}}
};

pub struct NodeService {
    storage: Arc<dyn NodeStorage>,
    group_storage: Arc<dyn NodeGroupStorage>,
    test_service: TestService,
}

impl NodeService {
    pub fn new(storage: Arc<dyn NodeStorage>, group_storage: Arc<dyn NodeGroupStorage>) -> Self {
        Self { 
            storage,
            group_storage,
            test_service: TestService::new(),
        }
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

    pub async fn get_node(&self, id: &str) -> Result<Option<Node>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_nodes(&self) -> Result<Vec<Node>> {
        self.storage.get_all().await
    }

    pub async fn update_node(&self, mut node: Node) -> Result<Node> {
        node.updated_at = chrono::Utc::now();
        self.storage.update(&node).await?;
        Ok(node)
    }

    pub async fn delete_node(&self, id: &str) -> Result<()> {

        let all_groups = self.group_storage.get_all().await?;
    
        // Remove node from all groups that contain it
        for mut group in all_groups {
            if group.base.node_sequence.contains(&id.to_string()) {
                group.base.node_sequence.retain(|seq_id| seq_id != &id.to_string());
                group.updated_at = chrono::Utc::now();
                self.group_storage.update(&group).await?;
            }
        }

        self.storage.delete(id).await
    }

    pub async fn execute_tests(&self, node: &mut Node) -> NodeTestResults {
        let tests = &node.base.assigned_tests;
        let mut test_results: Vec<TestResult> = Vec::new();

        let timer = Timer::now();

        for test in tests{
            let result = self.test_service.execute_test(&test.test_type, &test.test_config).await;
            test_results.push(result)
        };

        node.update_status_from_tests(&test_results);

        NodeTestResults {
            test_results,
            node_status: node.base.current_status.clone(),
            duration_ms: timer.elapsed_ms(),
            executed_at: timer.datetime(),
        }
    }

    // Auto-detect and assign node type + capabilities from discovery results
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
}