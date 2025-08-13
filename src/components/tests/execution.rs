use anyhow::Result;
use crate::core::{TestType, TestConfiguration, TestResult, Node};
use crate::components::tests::implementations::*;

/// Execute a test with type-safe configuration
pub async fn execute_test(
    test_type: &TestType,
    config: &TestConfiguration,
    target_node: &Node,
) -> Result<TestResult> {
    // Validate test compatibility with node
    if !test_type.is_compatible_with_node(target_node) {
        return Ok(TestResult {
            test_type: test_type.clone(),
            success: false,
            message: format!(
                "Test {} is not compatible with node {} ({})",
                test_type.display_name(),
                target_node.name,
                target_node.node_type.as_ref().map(|t| t.display_name()).unwrap_or("Unknown")
            ),
            duration_ms: 0,
            executed_at: chrono::Utc::now(),
            details: Some(serde_json::json!({
                "error": "incompatible_test_node_combination",
                "test_type": test_type,
                "node_type": target_node.node_type
            })),
        });
    }

    // Execute test based on type and configuration
    match (test_type, config) {
        (TestType::Connectivity, TestConfiguration::Connectivity(config)) => {
            execute_connectivity_test(config).await
        },
        (TestType::DirectIp, TestConfiguration::DirectIp(config)) => {
            execute_direct_ip_test(config).await
        },
        (TestType::Ping, TestConfiguration::Ping(config)) => {
            execute_ping_test(config).await
        },
        (TestType::WellknownIp, TestConfiguration::WellknownIp(config)) => {
            execute_wellknown_ip_test(config).await
        },
        // (TestType::DnsResolution, TestConfiguration::DnsResolution(config)) => {
        //     execute_dns_resolution_test(config).await
        // },
        // (TestType::DnsOverHttps, TestConfiguration::DnsOverHttps(config)) => {
        //     execute_dns_over_https_test(config).await
        // },
        // (TestType::VpnConnectivity, TestConfiguration::VpnConnectivity(config)) => {
        //     execute_vpn_connectivity_test(config).await
        // },
        // (TestType::VpnTunnel, TestConfiguration::VpnTunnel(config)) => {
        //     execute_vpn_tunnel_test(config).await
        // },
        // (TestType::ServiceHealth, TestConfiguration::ServiceHealth(config)) => {
        //     execute_service_health_test(config).await
        // },
        // (TestType::DaemonCommand, TestConfiguration::DaemonCommand(config)) => {
        //     execute_daemon_command_test(config).await
        // },
        // (TestType::SshScript, TestConfiguration::SshScript(config)) => {
        //     execute_ssh_script_test(config).await
        // },
        // Type safety ensures this should never happen, but handle gracefully
        _ => {
            Err(anyhow::anyhow!(
                "Test type {:?} does not match configuration type",
                test_type
            ))
        }
    }
}

/// Execute multiple tests on a single node
pub async fn execute_node_tests(
    node: &Node,
) -> Result<Vec<TestResult>> {
    let mut results = Vec::new();
    
    for assigned_test in &node.assigned_tests {
        if !assigned_test.enabled {
            continue;
        }
        
        let result = execute_test(
            &assigned_test.test_type,
            &assigned_test.test_config,
            node,
        ).await?;
        
        results.push(result);
    }
    
    Ok(results)
}

/// Execute a single ad-hoc test (not part of node's assigned tests)
pub async fn execute_adhoc_test(
    test_type: TestType,
    config: TestConfiguration,
    target_node: &Node,
) -> Result<TestResult> {
    execute_test(&test_type, &config, target_node).await
}

/// Validate test configuration for a specific test type
pub fn validate_test_config(
    test_type: &TestType,
    config: &TestConfiguration,
) -> Result<()> {
    match (test_type, config) {
        (TestType::Connectivity, TestConfiguration::Connectivity(_)) => Ok(()),
        (TestType::DirectIp, TestConfiguration::DirectIp(_)) => Ok(()),
        (TestType::Ping, TestConfiguration::Ping(_)) => Ok(()),
        (TestType::WellknownIp, TestConfiguration::WellknownIp(_)) => Ok(()),
        (TestType::DnsResolution, TestConfiguration::DnsResolution(_)) => Ok(()),
        (TestType::DnsOverHttps, TestConfiguration::DnsOverHttps(_)) => Ok(()),
        (TestType::VpnConnectivity, TestConfiguration::VpnConnectivity(_)) => Ok(()),
        (TestType::VpnTunnel, TestConfiguration::VpnTunnel(_)) => Ok(()),
        (TestType::ServiceHealth, TestConfiguration::ServiceHealth(_)) => Ok(()),
        (TestType::DaemonCommand, TestConfiguration::DaemonCommand(_)) => Ok(()),
        (TestType::SshScript, TestConfiguration::SshScript(_)) => Ok(()),
        _ => Err(anyhow::anyhow!(
            "Test type {:?} does not match configuration type",
            test_type
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{NodeType, NodeCapability};
    use crate::components::tests::configs::*;

    #[tokio::test]
    async fn test_connectivity_validation() {
        let mut node = Node::new("test-node".to_string());
        node.node_type = Some(NodeType::WebServer);
        node.capabilities = vec![NodeCapability::HttpService];

        let config = TestConfiguration::Connectivity(ConnectivityConfig::default());
        
        let result = validate_test_config(&TestType::Connectivity, &config);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_incompatible_test_node() {
        let mut node = Node::new("printer".to_string());
        node.node_type = Some(NodeType::Printer);

        let config = TestConfiguration::VpnConnectivity(VpnConnectivityConfig::default());
        
        let result = execute_test(&TestType::VpnConnectivity, &config, &node).await;
        assert!(result.is_ok());
        let test_result = result.unwrap();
        assert!(!test_result.success);
        assert!(test_result.message.contains("not compatible"));
    }
}