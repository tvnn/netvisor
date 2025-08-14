// src/components/tests/execution.rs
use anyhow::Result;
use crate::components::{
    tests::{
        implementations::*,
        types::{TestConfiguration, TestResult, TestType}
    },
    nodes::types::{Node, NodeStatus, AssignedTest, TestCriticality}
};

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
                target_node.base.name,
                target_node.base.node_type.as_ref().map(|t| t.display_name()).unwrap_or("Unknown")
            ),
            duration_ms: 0,
            executed_at: chrono::Utc::now(),
            details: Some(serde_json::json!({
                "error": "incompatible_test_node_combination",
                "test_type": test_type,
                "node_type": target_node.base.node_type
            })),
        });
    }

    // Validate test configuration matches test type
    if let Err(e) = validate_test_config(test_type, config) {
        return Ok(TestResult {
            test_type: test_type.clone(),
            success: false,
            message: format!("Invalid test configuration: {}", e),
            duration_ms: 0,
            executed_at: chrono::Utc::now(),
            details: Some(serde_json::json!({
                "error": "invalid_configuration",
                "details": e.to_string()
            })),
        });
    }

    // Execute test based on type and configuration
    match (test_type, config) {
        (TestType::Connectivity, TestConfiguration::Connectivity(config)) => {
            connectivity::execute_connectivity_test(config).await
        },
        (TestType::DirectIp, TestConfiguration::DirectIp(config)) => {
            connectivity::execute_direct_ip_test(config).await
        },
        (TestType::Ping, TestConfiguration::Ping(config)) => {
            connectivity::execute_ping_test(config).await
        },
        (TestType::WellknownIp, TestConfiguration::WellknownIp(config)) => {
            connectivity::execute_wellknown_ip_test(config).await
        },
        (TestType::DnsResolution, TestConfiguration::DnsResolution(config)) => {
            dns::execute_dns_resolution_test(config).await
        },
        (TestType::DnsOverHttps, TestConfiguration::DnsOverHttps(config)) => {
            dns::execute_dns_over_https_test(config).await
        },
        (TestType::VpnConnectivity, TestConfiguration::VpnConnectivity(config)) => {
            vpn::execute_vpn_connectivity_test(config).await
        },
        (TestType::VpnTunnel, TestConfiguration::VpnTunnel(config)) => {
            vpn::execute_vpn_tunnel_test(config).await
        },
        (TestType::ServiceHealth, TestConfiguration::ServiceHealth(config)) => {
            service::execute_service_health_test(config).await
        },
        // (TestType::DaemonCommand, TestConfiguration::DaemonCommand(config)) => {
        //     daemon::execute_daemon_command_test(config).await
        // },
        // (TestType::SshScript, TestConfiguration::SshScript(config)) => {
        //     daemon::execute_ssh_script_test(config).await
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
    
    for assigned_test in &node.base.assigned_tests {
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

/// Compute node status from test results based on criticality
pub fn compute_node_status_from_results(results: &[TestResult], assigned_tests: &[AssignedTest]) -> NodeStatus {
    
    if results.is_empty() {
        return NodeStatus::Unknown;
    }
    
    let mut has_critical_failure = false;
    let mut has_important_failure = false;
    
    for result in results {
        if !result.success {
            // Find the criticality for this test type
            if let Some(assigned_test) = assigned_tests.iter().find(|t| t.test_type == result.test_type) {
                match assigned_test.criticality {
                    TestCriticality::Critical => has_critical_failure = true,
                    TestCriticality::Important => has_important_failure = true,
                    TestCriticality::Informational => {}, // Doesn't affect status
                }
            }
        }
    }
    
    if has_critical_failure {
        NodeStatus::Failed
    } else if has_important_failure {
        NodeStatus::Degraded
    } else {
        NodeStatus::Healthy
    }
}