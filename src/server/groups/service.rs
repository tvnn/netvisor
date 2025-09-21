use anyhow::Result;
use uuid::Uuid;
use std::sync::{Arc};
use crate::server::groups::{
    storage::GroupStorage,
    types::Group
};
use crate::server::services::service::ServiceService;
use crate::server::services::types::base::ServiceUpdateRequest;

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

        for service_id in &group.base.services {
            if self.service_service.get_service(service_id).await?.is_none() {
                return Err(anyhow::anyhow!("Service with id '{}' not found", service_id));
            }
        }

        self.group_storage.create(&group).await?;
        
        // Add group reference to all services in the sequence
        for service_id in &group.base.services {
            if let Some(service) = self.service_service.get_service(service_id).await? {

                if service.base.groups.contains(&group.id) {
                    continue; // Already in group
                }
                let mut groups = service.base.groups.clone();
                groups.push(group.id);
                let update = ServiceUpdateRequest::from_group_change(groups);

                self.service_service.update_service(service, update).await?;
            }
        }

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
        
        // Validate that all services in sequence exist
        for service_id in &group.base.services {
            if self.service_service.get_service(service_id).await?.is_none() {
                return Err(anyhow::anyhow!("Host with id '{}' not found", service_id));
            }
        }

        // Get old group to compare host sequences
        let old_group = self.get_group(&group.id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        self.group_storage.update(&group).await?;

        // Update group references
        // Remove group from services no longer in sequence
        for old_service_id in &old_group.base.services {
            if !group.base.services.contains(old_service_id) {
                if let Some(service) = self.service_service.get_service(old_service_id).await? {
                    if !service.base.groups.contains(&group.id) {
                        continue; // Not in group
                    }
                    let groups = service.base.groups.iter().filter(|g| *g != &group.id).cloned().collect();
                    let update = ServiceUpdateRequest::from_group_change(groups);
                    self.service_service.update_service(service, update).await?;
                }
            }
        }

        // Add group to new services in sequence
        for new_service_id in &group.base.services {
            if !old_group.base.services.contains(new_service_id) {
                if let Some(service) = self.service_service.get_service(new_service_id).await? {
                    
                    if service.base.groups.contains(&group.id) {
                        continue; // Already in group
                    }
                    let mut groups = service.base.groups.clone();
                    groups.push(group.id);
                    let update = ServiceUpdateRequest::from_group_change(groups);

                    self.service_service.update_service(service, update).await?;
                }
            }
        }

        Ok(group)
    }

    /// Delete group
    pub async fn delete_group(&self, id: &Uuid) -> Result<()> {

        // Get group to find hosts to update
        let group = self.get_group(id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        // Remove group reference from all hosts
        for service_id in &group.base.services {
            if let Some(service) = self.service_service.get_service(service_id).await? {
                if !service.base.groups.contains(&group.id) {
                    continue; // Not in group
                }
                let groups = service.base.groups.iter().filter(|g| *g != &group.id).cloned().collect();
                let update = ServiceUpdateRequest::from_group_change(groups);
                self.service_service.update_service(service, update).await?;
            }
        }

        self.group_storage.delete(id).await?;
        Ok(())
    }
}