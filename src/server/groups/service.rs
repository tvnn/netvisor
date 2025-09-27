use anyhow::Result;
use uuid::Uuid;
use std::sync::{Arc};
use crate::server::groups::{
    storage::GroupStorage,
    types::Group
};
use crate::server::services::service::ServiceService;

pub struct GroupService {
    group_storage: Arc<dyn GroupStorage>,
    service_service: Arc<ServiceService>
}

impl GroupService {
    pub fn new(
        group_storage: Arc<dyn GroupStorage>,
        service_service: Arc<ServiceService>
    ) -> Self {
        Self {
            group_storage,
            service_service
        }
    }

    /// Create a new group
    pub async fn create_group(&self, group: Group) -> Result<Group> {

        self.group_storage.create(&group).await?;
        
        // Add group reference to all services in the sequence
        for binding in &group.base.service_bindings {
            if let Some(mut service) = self.service_service.get_service(&binding.service_id).await? {

                if service.base.groups.contains(&group.id) {
                    continue; // Already in group
                }
                service.base.groups = { service.base.groups.push(group.id); service.base.groups };

                self.service_service.update_service(service).await?;
            }
        }
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
        
        // Get old group to compare services
        let old_group = self.get_group(&group.id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        self.group_storage.update(&group).await?;

        // Update group references
        // Remove group from services no longer in group
        for old_binding in &old_group.base.service_bindings {
            if !group.base.service_bindings.contains(old_binding) {
                if let Some(mut service) = self.service_service.get_service(&old_binding.service_id).await? {
                    if !service.base.groups.contains(&group.id) {
                        continue; // Not in group
                    }
                    service.base.groups = service.base.groups.iter().filter(|g| *g != &group.id).cloned().collect();
                    self.service_service.update_service(service).await?;
                }
            }
        }

        // Add group to new services in sequence
        for new_binding in &group.base.service_bindings {
            if !old_group.base.service_bindings.contains(new_binding) {
                if let Some(mut service) = self.service_service.get_service(&new_binding.service_id).await? {
                    
                    if service.base.groups.contains(&group.id) {
                        continue; // Already in group
                    }
                    service.base.groups = { service.base.groups.push(group.id); service.base.groups };

                    self.service_service.update_service(service).await?;
                }
            }
        }
        tracing::info!("Updated group {}: {}", group.base.name, group.id);
        Ok(group)
    }

    /// Delete group
    pub async fn delete_group(&self, id: &Uuid) -> Result<()> {

        // Get group to find hosts to update
        let group = self.get_group(id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        // Remove group reference from all hosts
        for binding in &group.base.service_bindings {
            if let Some(mut service) = self.service_service.get_service(&binding.service_id).await? {
                if !service.base.groups.contains(&group.id) {
                    continue; // Not in group
                }
                service.base.groups = service.base.groups.iter().filter(|g| *g != &group.id).cloned().collect();
                self.service_service.update_service(service).await?;
            }
        }

        self.group_storage.delete(id).await?;
        tracing::info!("Deleted group {}: {}", group.base.name, group.id);
        Ok(())
    }
}