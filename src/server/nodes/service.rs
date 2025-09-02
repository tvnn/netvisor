use anyhow::Result;
use futures::future::join_all;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    node_groups::storage::NodeGroupStorage, nodes::{
        storage::NodeStorage,
        types::{
            base::Node, 
            tests::NodeTestResults,
        }
    }, tests::{service::TestService, types::execution::{TestResult, Timer}}
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
    pub async fn create_node(&self, node: Node) -> Result<Node> {
        
        let all_nodes = self.storage.get_all().await?;

        match all_nodes.iter().find(|n| node.eq(n)) {
            Some(existing_node) => {
                Ok(existing_node.clone())
            }
            None => {
                self.storage.create(&node).await?;
                Ok(node)
            }
        }
    }

    pub async fn get_node(&self, id: &Uuid) -> Result<Option<Node>> {
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

    pub async fn delete_node(&self, id: &Uuid) -> Result<()> {

        let all_groups = self.group_storage.get_all().await?;
    
        // Remove node from all groups that contain it
        for mut group in all_groups {
            if group.base.node_sequence.contains(&id) {
                group.base.node_sequence.retain(|seq_id| seq_id != id);
                group.updated_at = chrono::Utc::now();
                self.group_storage.update(&group).await?;
            }
        }

        self.storage.delete(id).await
    }

    pub async fn execute_tests(&self, node: &mut Node) -> NodeTestResults {
        
        let timer = Timer::now();

        let test_futures: Vec<_> = node.base.capabilities.iter()
            .flat_map(|capability| 
                capability.config_base().tests.iter()
                    .filter(|test| test.enabled)
                    .map(|test| 
                        self.test_service.execute_test(test, node, capability, &self)
                    )
            )
            .collect();

        let test_results: Vec<TestResult> = join_all(test_futures).await;

        node.update_status_from_tests(&test_results);

        NodeTestResults {
            test_results,
            node_id: node.id,
            node_status: node.base.status.clone(),
            duration_ms: timer.elapsed_ms(),
            executed_at: timer.datetime(),
        }
    }
}