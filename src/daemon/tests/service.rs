use std::sync::Arc;

use anyhow::Error;
use uuid::Uuid;

use crate::{daemon::shared::storage::ConfigStore, server::{daemons::types::api::{DaemonTestRequest, DaemonTestResult}, tests::types::execution::TestResult}};

pub struct DaemonTestService {
    pub config_store: Arc<ConfigStore>,
    pub client: reqwest::Client,
}

impl DaemonTestService {
    pub fn new(config_store: Arc<ConfigStore>) -> Self {
        Self {
            config_store,
            client: reqwest::Client::new(),
        }
    }

    /// Report test result back to server
    pub async fn report_test_result(&self, session_id: Uuid, result: TestResult) -> Result<(), Error> {
        let test_result = DaemonTestResult {
            session_id,
            result,
        };

        let server_target = self.config_store.get_server_endpoint().await?;

        let response = self
            .client
            .post(format!("{}/api/daemons/test_result", server_target.to_string()))
            .json(&test_result)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to report test result: HTTP {}", response.status());
        }

        tracing::debug!("Test result reported successfully");
        Ok(())
    }

    /// Background task for test execution session
    pub async fn run_test_execution(&self, request: DaemonTestRequest) -> anyhow::Result<()> {
        tracing::info!("Executing tests for session {} on node {}", request.session_id, request.node.base.name);
        
        // Get server target from config for reporting back
        // let server_target = service.config_store.get_server_endpoint().await?
        //     .ok_or_else(|| anyhow::anyhow!("No server endpoint configured"))?;
        
        // TODO: Implement actual test execution
        // This would:
        // 1. Extract test configurations from request.node.assigned_tests
        // 2. Execute each test using existing test execution framework
        // 3. For each completed test, call service.report_test_result()
        
        // Placeholder: simulate test execution
        // for (i, assigned_test) in request.node.base.assigned_tests.iter().enumerate() {
        //     tracing::info!("Executing test {}/{}: {:?}", 
        //                   i + 1, 
        //                   request.node.base.assigned_tests.len(), 
        //                   assigned_test.test);
            
        //     // Simulate test execution time
        //     tokio::time::sleep(std::time::Duration::from_millis(800)).await;
            
        //     // Create placeholder result
        //     let test_result = crate::server::tests::types::execution::TestResult {
        //         success: true,
        //         message: Some(format!("Test completed successfully")),
        //         details: None,
        //         criticality: Some(assigned_test.criticality.clone()),
        //         duration_ms: 800,
        //         timestamp: chrono::Utc::now(),
        //     };
            
        //     // Report result back to server
        //     if let Err(e) = service.report_test_result(&server_target, request.session_id, test_result).await {
        //         tracing::error!("Failed to report test result: {}", e);
        //     }
        // }
        
        tracing::info!("Test execution session {} completed", request.session_id);
        Ok(())
    }
}
