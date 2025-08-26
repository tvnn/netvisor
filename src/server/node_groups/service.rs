use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use crate::server::node_groups::{
    storage::NodeGroupStorage,
    types::NodeGroup
};
use crate::server::nodes::{
    storage::NodeStorage,
    types::base::Node
};

pub struct NodeGroupService {
    group_storage: Arc<dyn NodeGroupStorage>,
    node_storage: Arc<dyn NodeStorage>,
}

impl NodeGroupService {
    pub fn new(
        group_storage: Arc<dyn NodeGroupStorage>,
        node_storage: Arc<dyn NodeStorage>,
    ) -> Self {
        Self {
            group_storage,
            node_storage,
        }
    }

    /// Create a new node group
    pub async fn create_group(&self, group: NodeGroup) -> Result<NodeGroup> {
        // Validate that all nodes in sequence exist
        for node_id in &group.base.node_sequence {
            if self.node_storage.get_by_id(node_id).await?.is_none() {
                return Err(anyhow::anyhow!("Node with id '{}' not found", node_id));
            }
        }

        self.group_storage.create(&group).await?;
        
        // Add group reference to all nodes in the sequence
        for node_id in &group.base.node_sequence {
            if let Some(mut node) = self.node_storage.get_by_id(node_id).await? {
                node.add_to_group(group.id.clone());
                self.node_storage.update(&node).await?;
            }
        }

        Ok(group)
    }

    /// Get group by ID
    pub async fn get_group(&self, id: &Uuid) -> Result<Option<NodeGroup>> {
        self.group_storage.get_by_id(id).await
    }

    /// Get all groups
    pub async fn get_all_groups(&self) -> Result<Vec<NodeGroup>> {
        self.group_storage.get_all().await
    }

    /// Update group
    pub async fn update_group(&self, mut group: NodeGroup) -> Result<NodeGroup> {
        let now = chrono::Utc::now();
        group.updated_at = now;
        
        // Validate that all nodes in sequence exist
        for node_id in &group.base.node_sequence {
            if self.node_storage.get_by_id(node_id).await?.is_none() {
                return Err(anyhow::anyhow!("Node with id '{}' not found", node_id));
            }
        }

        // Get old group to compare node sequences
        let old_group = self.get_group(&group.id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        self.group_storage.update(&group).await?;

        // Update node group references
        // Remove group from nodes no longer in sequence
        for old_node_id in &old_group.base.node_sequence {
            if !group.base.node_sequence.contains(old_node_id) {
                if let Some(mut node) = self.node_storage.get_by_id(old_node_id).await? {
                    node.remove_from_group(&group.id);
                    self.node_storage.update(&node).await?;
                }
            }
        }

        // Add group to new nodes in sequence
        for new_node_id in &group.base.node_sequence {
            if !old_group.base.node_sequence.contains(new_node_id) {
                if let Some(mut node) = self.node_storage.get_by_id(new_node_id).await? {
                    node.add_to_group(group.id.clone());
                    self.node_storage.update(&node).await?;
                }
            }
        }

        Ok(group)
    }

    /// Delete group
    pub async fn delete_group(&self, id: &Uuid) -> Result<()> {
        // Get group to find nodes to update
        let group = self.get_group(id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        // Remove group reference from all nodes
        for node_id in &group.base.node_sequence {
            if let Some(mut node) = self.node_storage.get_by_id(node_id).await? {
                node.remove_from_group(&group.id);
                self.node_storage.update(&node).await?;
            }
        }

        self.group_storage.delete(id).await?;
        Ok(())
    }

    /// Add node to group sequence
    pub async fn add_node_to_group(&self, group_id: &Uuid, node_id: &Uuid, position: Option<usize>) -> Result<()> {
        let mut group = self.get_group(group_id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        // Validate node exists
        if self.node_storage.get_by_id(node_id).await?.is_none() {
            return Err(anyhow::anyhow!("Node not found"));
        }

        // Check if node is already in sequence
        if group.base.node_sequence.contains(&node_id) {
            return Ok(()); // Already in group
        }

        // Add node at specified position or end
        match position {
            Some(pos) if pos <= group.base.node_sequence.len() => {
                group.base.node_sequence.insert(pos, *node_id);
            },
            _ => {
                group.base.node_sequence.push(*node_id);
            }
        }

        self.update_group(group).await?;
        Ok(())
    }

    /// Remove node from group sequence
    pub async fn remove_node_from_group(&self, group_id: &Uuid, node_id: &Uuid) -> Result<()> {
        let mut group = self.get_group(group_id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        group.base.node_sequence.retain(|n| n != node_id);
        self.update_group(group).await?;
        Ok(())
    }



    /// Get nodes in group sequence
    pub async fn get_group_nodes(&self, group_id: &Uuid) -> Result<Vec<Node>> {
        let group = self.get_group(group_id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        let mut nodes = Vec::new();
        for node_id in &group.base.node_sequence {
            if let Some(node) = self.node_storage.get_by_id(node_id).await? {
                nodes.push(node);
            }
        }

        Ok(nodes)
    }
}