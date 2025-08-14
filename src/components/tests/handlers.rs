use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::{
    api::{ApiResult, ApiResponse},
    components::{
        tests::{
            service::TestService,
            types::{TestType, TestConfiguration,TestResult}
        },
        nodes::types::{Node},
    },
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/execute-adhoc", post(execute_adhoc_test_handler))
        .route("/execute-node/:node_id", post(execute_node_tests_handler))
        .route("/types", get(get_test_types))
        .route("/compatibility/:node_id", get(get_compatible_tests))
}

#[derive(serde::Deserialize)]
pub struct ExecuteAdhocTestRequest {
    pub node_id: String,
    pub test_type: TestType,
    pub test_config: TestConfiguration,
}

#[derive(serde::Serialize)]
pub struct TestExecutionResponse {
    pub result: TestResult,
}

#[derive(serde::Serialize)]
pub struct NodeTestExecutionResponse {
    pub node_id: String,
    pub node_name: String,
    pub results: Vec<TestResult>,
    pub previous_status: String,
    pub new_status: String,
    pub executed_at: String,
}

#[derive(serde::Serialize)]
pub struct TestTypeInfo {
    pub test_type: TestType,
    pub display_name: String,
    pub description: String,
    pub required_capabilities: Vec<String>,
    pub required_node_types: Vec<String>,
    pub forbidden_node_types: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct TestAssignmentApiResponse {
    pub node: Node,
    pub warning: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CompatibilityResponse {
    pub node_id: String,
    pub node_name: String,
    pub node_type: Option<String>,
    pub recommended_tests: Vec<TestTypeCompatibilityInfo>,
    pub other_tests: Vec<TestTypeCompatibilityInfo>,
}

#[derive(serde::Serialize)]
pub struct TestTypeCompatibilityInfo {
    pub test_type: TestType,
    pub display_name: String,
    pub description: String,
    pub is_assigned: bool,
    pub warning: Option<String>,
    pub is_recommended: bool,
}

/// Execute a single ad-hoc test
async fn execute_adhoc_test_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ExecuteAdhocTestRequest>,
) -> ApiResult<Json<ApiResponse<TestExecutionResponse>>> {
    let test_service = TestService::new(state.node_storage.clone());
    
    let result = test_service.execute_adhoc_test(
        &request.node_id,
        request.test_type,
        request.test_config,
    ).await?;
    
    Ok(Json(ApiResponse::success(TestExecutionResponse { result })))
}

/// Execute all assigned tests on a node
async fn execute_node_tests_handler(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<String>,
) -> ApiResult<Json<ApiResponse<NodeTestExecutionResponse>>> {
    let test_service = TestService::new(state.node_storage.clone());
    
    let execution_result = test_service.execute_node_tests(&node_id).await?;
    
    Ok(Json(ApiResponse::success(NodeTestExecutionResponse {
        node_id: execution_result.node_id,
        node_name: execution_result.node_name,
        results: execution_result.results,
        previous_status: execution_result.previous_status.display_name().to_string(),
        new_status: execution_result.new_status.display_name().to_string(),
        executed_at: execution_result.executed_at.to_rfc3339(),
    })))
}

/// Get all available test types with their metadata
async fn get_test_types(
    State(_state): State<Arc<AppState>>,
) -> ApiResult<Json<ApiResponse<Vec<TestTypeInfo>>>> {
    
    let test_types = vec![
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
    
    let test_info: Vec<TestTypeInfo> = test_types.into_iter().map(|test_type| {
        let recommendations = test_type.get_recommendations();
        
        TestTypeInfo {
            display_name: test_type.display_name().to_string(),
            description: get_test_description(&test_type),
            required_capabilities: recommendations.helpful_capabilities.iter().map(|c| format!("{:?}", c)).collect(),
            required_node_types: recommendations.ideal_node_types.iter().map(|t| t.display_name().to_string()).collect(),
            forbidden_node_types: vec![], // No longer relevant with warning system
            test_type,
        }
    }).collect();
    
    Ok(Json(ApiResponse::success(test_info)))
}

/// Get compatible tests for a specific node
async fn get_compatible_tests(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<String>,
) -> ApiResult<Json<ApiResponse<CompatibilityResponse>>> {
    let test_service = TestService::new(state.node_storage.clone());
    
    let compatibility = test_service.get_node_test_compatibility(&node_id).await?;
    
    let recommended_tests: Vec<TestTypeCompatibilityInfo> = compatibility.recommended_tests
        .into_iter()
        .map(|test_info| TestTypeCompatibilityInfo {
            test_type: test_info.test_type,
            display_name: test_info.display_name,
            description: test_info.description,
            is_assigned: test_info.is_assigned,
            warning: test_info.warning,
            is_recommended: test_info.is_recommended,
        })
        .collect();
    
    let other_tests: Vec<TestTypeCompatibilityInfo> = compatibility.other_tests
        .into_iter()
        .map(|test_info| TestTypeCompatibilityInfo {
            test_type: test_info.test_type,
            display_name: test_info.display_name,
            description: test_info.description,
            is_assigned: test_info.is_assigned,
            warning: test_info.warning,
            is_recommended: test_info.is_recommended,
        })
        .collect();
    
    Ok(Json(ApiResponse::success(CompatibilityResponse {
        node_id: compatibility.node_id,
        node_name: compatibility.node_name,
        node_type: compatibility.node_type,
        recommended_tests,
        other_tests,
    })))
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