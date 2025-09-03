use std::sync::Arc;
use anyhow::Result;
use uuid::Uuid;
use super::{
    storage::DiagnosticStorage,
    types::*,
};
use crate::{server::{
    node_groups::{
        service::NodeGroupService
    }, nodes::{
        service::NodeService
    },
}};

pub struct DiagnosticService {
    diagnostic_storage: Arc<dyn DiagnosticStorage>,
    node_service: Arc<NodeService>,
    node_group_service: Arc<NodeGroupService>,
}

impl DiagnosticService {
    pub fn new(
        diagnostic_storage: Arc<dyn DiagnosticStorage>,
        node_service: Arc<NodeService>,
        node_group_service: Arc<NodeGroupService>,
    ) -> Self {
        Self {
            diagnostic_storage,
            node_service,
            node_group_service,
        }
    }

    /// Execute diagnostics on a node group and return completed results
    pub async fn execute_group_diagnostic(
        &self,
        group_id: &Uuid,
        trigger_reason: DiagnosticTrigger,
    ) -> Result<DiagnosticExecution> {
        // Get the node group using the service
        let group = self.node_group_service.get_group(group_id).await?
            .ok_or_else(|| anyhow::anyhow!("Node group not found: {}", group_id))?;

        // Create diagnostic execution
        let mut execution = DiagnosticExecution::new(
            DiagnosticExecutionBase {
                group_id: group.id.to_string(),
                trigger_reason,
            }
        );

        self.diagnostic_storage.create(execution.clone()).await?;

        // Execute tests on each node in sequence
        for node_id in &group.base.node_sequence {

            if let Some(mut node) = self.node_service.get_node(&node_id).await? {
                let node_results = self.node_service.execute_tests(&mut node).await;                
                execution.add_node_results(node_results);
            }
        }

        execution.complete();

        self.diagnostic_storage.update(execution.clone()).await?;
        
        Ok(execution)
    }

    /// Get all diagnostic executions
    pub async fn get_all_executions(&self) -> Result<Vec<DiagnosticExecution>> {
        self.diagnostic_storage.get_all().await
    }

    /// Delete a diagnostic execution
    pub async fn delete_execution(&self, execution_id: &str) -> Result<()> {
        self.diagnostic_storage.delete(execution_id).await
    }
}