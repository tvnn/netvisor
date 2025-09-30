use crate::server::groups::{storage::GroupStorage, types::Group};
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

pub struct GroupService {
    group_storage: Arc<dyn GroupStorage>,
}

impl GroupService {
    pub fn new(group_storage: Arc<dyn GroupStorage>) -> Self {
        Self { group_storage }
    }

    /// Create a new group
    pub async fn create_group(&self, group: Group) -> Result<Group> {
        self.group_storage.create(&group).await?;

        tracing::info!("Created group {}: {}", group.base.name, group.id);
        Ok(group)
    }

    /// Get group by ID
    pub async fn get_group(&self, id: &Uuid) -> Result<Option<Group>> {
        self.group_storage.get_by_id(id).await
    }

    /// Get all groups
    pub async fn get_all_groups(&self) -> Result<Vec<Group>> {
        self.group_storage.get_all().await
    }

    /// Update group
    pub async fn update_group(&self, mut group: Group) -> Result<Group> {
        let now = chrono::Utc::now();
        group.updated_at = now;

        self.group_storage.update(&group).await?;

        tracing::info!("Updated group {}: {}", group.base.name, group.id);
        Ok(group)
    }

    /// Delete group
    pub async fn delete_group(&self, id: &Uuid) -> Result<()> {
        // Get group to find hosts to update
        let group = self
            .get_group(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        self.group_storage.delete(id).await?;
        tracing::info!("Deleted group {}: {}", group.base.name, group.id);
        Ok(())
    }
}
