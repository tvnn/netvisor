use std::sync::Arc;
use anyhow::Result;
use super::{
    storage::DiagnosticStorage,
    types::*,
};
use crate::{components::{
    node_groups::{
        service::NodeGroupService,
        storage::NodeGroupStorage
    }, nodes::{
        service::NodeService, storage::NodeStorage
    },
}};

pub struct DiagnosticService {
    diagnostic_storage: Arc<dyn DiagnosticStorage>,
    node_service: NodeService,
    node_group_service: NodeGroupService,
}

impl DiagnosticService {
    pub fn new(
        diagnostic_storage: Arc<dyn DiagnosticStorage>,
        node_storage: Arc<dyn NodeStorage>,
        node_group_storage: Arc<dyn NodeGroupStorage>,
    ) -> Self {
        Self {
            diagnostic_storage,
            node_service: NodeService::new(node_storage.clone(), node_group_storage.clone()),
            node_group_service: NodeGroupService::new(node_group_storage, node_storage.clone()),
        }
    }

    /// Execute diagnostics on a node group and return completed results
    pub async fn execute_group_diagnostic(
        &self,
        group_id: &str,
        trigger_reason: DiagnosticTrigger,
    ) -> Result<DiagnosticExecution> {
        // Get the node group using the service
        let group = self.node_group_service.get_group(group_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node group not found: {}", group_id))?;

        // Create diagnostic execution
        let mut execution = DiagnosticExecution::new(
            DiagnosticExecutionBase {
                group_id: group.id.clone(),
                trigger_reason,
            }
        );

        println!("Starting diagnostic execution {} for {} nodes", 
                execution.id, group.base.node_sequence.len());

        // Execute tests on each node in sequence
        for node_id in &group.base.node_sequence {

            if let Some(mut node) = self.node_service.get_node(&node_id).await? {
                let node_results = self.node_service.execute_tests(&mut node).await;
                println!("Completed tests for node {} ({}): {} tests, status: {:?}", 
                                &node.base.name, &node.id, 
                                node_results.test_results.len(), &node.base.current_status);
                
                execution.add_node_results(node_results);
            }
        }

        // Store the completed execution
        self.diagnostic_storage.create(execution.clone()).await?;

        println!("Diagnostic execution {} completed with status: {:?}", 
                execution.id, execution.status);

        Ok(execution)
    }

    /// Get a diagnostic execution by ID
    pub async fn get_execution(&self, execution_id: &str) -> Result<Option<DiagnosticExecution>> {
        self.diagnostic_storage.get(execution_id).await
    }

    /// Get all diagnostic executions
    pub async fn get_all_executions(&self) -> Result<Vec<DiagnosticExecution>> {
        self.diagnostic_storage.get_all().await
    }

    /// Get diagnostic executions for a specific group
    pub async fn get_group_executions(&self, group_id: &str) -> Result<Vec<DiagnosticExecution>> {
        self.diagnostic_storage.get_by_group(group_id).await
    }
    
    /// Get diagnostic executions with filters
    pub async fn get_executions_with_filters(&self, query: DiagnosticListQuery) -> Result<Vec<DiagnosticExecution>> {
        self.diagnostic_storage.get_with_filters(query).await
    }

    /// Delete a diagnostic execution
    pub async fn delete_execution(&self, execution_id: &str) -> Result<()> {
        self.diagnostic_storage.delete(execution_id).await
    }

    /// Get the latest execution status for a group
    pub async fn get_latest_group_status(&self, group_id: &str) -> Result<Option<DiagnosticStatus>> {
        let executions = self.get_group_executions(group_id).await?;
        Ok(executions.first().map(|e| e.status.clone()))
    }

    /// Get diagnostic statistics
    pub async fn get_statistics(&self) -> Result<DiagnosticStatistics> {
        let all_executions = self.get_all_executions().await?;
        
        let total_executions = all_executions.len();
        let successful = all_executions.iter().filter(|e| e.status == DiagnosticStatus::Success).count();
        let failed = all_executions.iter().filter(|e| e.status == DiagnosticStatus::Failed).count();
        let partial_failures = all_executions.iter().filter(|e| e.status == DiagnosticStatus::PartialFail).count();
        
        // Calculate average execution time for completed diagnostics
        let completed_executions: Vec<_> = all_executions.iter()
            .filter(|e| e.completed_at.is_some())
            .collect();
        
        let average_execution_time_ms = if !completed_executions.is_empty() {
            let total_time: i64 = completed_executions.iter()
                .map(|e| {
                    let duration = e.completed_at.unwrap() - e.started_at;
                    duration.num_milliseconds()
                })
                .sum();
            Some(total_time / completed_executions.len() as i64)
        } else {
            None
        };

        Ok(DiagnosticStatistics {
            total_executions,
            successful_executions: successful,
            failed_executions: failed,
            partial_failure_executions: partial_failures,
            average_execution_time_ms,
        })
    }
}