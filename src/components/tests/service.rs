use anyhow::Result;
use std::sync::Arc;
use crate::{
    components::{
        tests::{
            types::{TestType, TestConfiguration, TestResult},
            execution::{execute_adhoc_test, execute_node_tests, compute_node_status_from_results},
        },
        nodes::{
            types::{NodeStatus},
            service::NodeService
        },
    },
};

pub struct TestService {
    node_service: NodeService,
}

impl TestService {
    pub fn new(node_storage: Arc<dyn crate::components::nodes::storage::NodeStorage>) -> Self {
        Self {
            node_service: NodeService::new(node_storage),
        }
    }

    /// Execute an ad-hoc test on a node
    pub async fn execute_adhoc_test(
        &self,
        node_id: &str,
        test_type: TestType,
        test_config: TestConfiguration,
    ) -> Result<TestResult> {
        let node = self.node_service.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found: {}", node_id))?;
        
        execute_adhoc_test(test_type, test_config, &node).await
    }

    /// Execute all assigned tests on a node and update its status
    pub async fn execute_node_tests(&self, node_id: &str) -> Result<NodeTestExecutionResult> {
        let node = self.node_service.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found: {}", node_id))?;
        
        let previous_status = node.base.current_status.clone();
        
        // Execute all assigned tests
        let results = execute_node_tests(&node).await?;
        
        // Compute new status based on results
        let new_status = compute_node_status_from_results(&results, &node.base.assigned_tests);
        
        // Update node with new status and last seen time
        let mut updated_node = node.clone();
        updated_node.update_status_and_last_seen(&new_status);
        
        self.node_service.update_node(updated_node).await?;
        
        Ok(NodeTestExecutionResult {
            node_id: node.id,
            node_name: node.base.name,
            results,
            previous_status,
            new_status,
            executed_at: chrono::Utc::now(),
        })
    }

    /// Get all available test types with recommendation info for a node
    pub async fn get_node_test_compatibility(&self, node_id: &str) -> Result<NodeTestCompatibility> {
        let node = self.node_service.get_node(node_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node not found: {}", node_id))?;
        
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
        
        let mut recommended_tests = Vec::new();
        let mut other_tests = Vec::new();
        
        for test_type in all_test_types {
            let is_assigned = node.base.assigned_tests.iter().any(|t| t.test_type == test_type);
            let warning = test_type.get_assignment_warning(&node);
            
            let test_info = TestTypeCompatibilityInfo {
                test_type: test_type.clone(),
                display_name: test_type.display_name().to_string(),
                description: get_test_description(&test_type),
                is_assigned,
                warning: warning.clone(),
                is_recommended: warning.is_none(),
            };
            
            if warning.is_none() {
                recommended_tests.push(test_info);
            } else {
                other_tests.push(test_info);
            }
        }
        
        Ok(NodeTestCompatibility {
            node_id: node.id,
            node_name: node.base.name,
            node_type: node.base.node_type.as_ref().map(|t| t.display_name().to_string()),
            recommended_tests,
            other_tests,
        })
    }

}

#[derive(Debug, Clone)]
pub struct NodeTestExecutionResult {
    pub node_id: String,
    pub node_name: String,
    pub results: Vec<TestResult>,
    pub previous_status: NodeStatus,
    pub new_status: NodeStatus,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct NodeTestCompatibility {
    pub node_id: String,
    pub node_name: String,
    pub node_type: Option<String>,
    pub recommended_tests: Vec<TestTypeCompatibilityInfo>,
    pub other_tests: Vec<TestTypeCompatibilityInfo>,
}

#[derive(Debug, Clone)]
pub struct TestTypeCompatibilityInfo {
    pub test_type: TestType,
    pub display_name: String,
    pub description: String,
    pub is_assigned: bool,
    pub warning: Option<String>,
    pub is_recommended: bool,
}

fn get_test_description(test_type: &TestType) -> String {
    match test_type {
        TestType::Connectivity => "Test TCP connectivity to a target host and port".to_string(),
        TestType::DirectIp => "Test direct IP connectivity bypassing DNS resolution".to_string(),
        TestType::Ping => "Test network reachability using ICMP ping".to_string(),
        TestType::WellknownIp => "Test connectivity to well-known public services".to_string(),
        TestType::DnsResolution => "Test DNS name resolution capabilities".to_string(),
        TestType::DnsOverHttps => "Test DNS resolution using DNS over HTTPS".to_string(),
        TestType::VpnConnectivity => "Test VPN server reachability and connection".to_string(),
        TestType::VpnTunnel => "Test VPN tunnel functionality and subnet access".to_string(),
        TestType::ServiceHealth => "Test HTTP/HTTPS service health and response".to_string(),
        TestType::DaemonCommand => "Execute system commands via NetFrog daemon (Phase 5)".to_string(),
        TestType::SshScript => "Execute commands via SSH connection (Phase 5)".to_string(),
    }
}