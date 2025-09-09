use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use crate::server::host_groups::{
    storage::HostGroupStorage,
    types::HostGroup
};
use crate::server::hosts::service::HostService;
use crate::server::hosts::types::api::HostUpdateRequest;

pub struct HostGroupService {
    group_storage: Arc<dyn HostGroupStorage>,
    host_service: Arc<HostService>,
}

impl HostGroupService {
    pub fn new(
        group_storage: Arc<dyn HostGroupStorage>,
        host_service: Arc<HostService>,
    ) -> Self {
        Self {
            group_storage,
            host_service,
        }
    }

    /// Create a new host group
    pub async fn create_group(&self, group: HostGroup) -> Result<HostGroup> {
        // Validate that all hosts in sequence exist
        for host_id in &group.base.hosts {
            if self.host_service.get_host(host_id).await?.is_none() {
                return Err(anyhow::anyhow!("Host with id '{}' not found", host_id));
            }
        }

        self.group_storage.create(&group).await?;
        
        // Add group reference to all hosts in the sequence
        for host_id in &group.base.hosts {
            if let Some(host) = self.host_service.get_host(host_id).await? {

                if host.base.groups.contains(&group.id) {
                    continue; // Already in group
                }
                let mut groups = host.base.groups;
                groups.push(group.id);
                let update = HostUpdateRequest::from_group_change(groups);

                self.host_service.update_host(&host.id, update).await?;
            }
        }

        Ok(group)
    }

    /// Get group by ID
    pub async fn get_group(&self, id: &Uuid) -> Result<Option<HostGroup>> {
        self.group_storage.get_by_id(id).await
    }

    /// Get all groups
    pub async fn get_all_groups(&self) -> Result<Vec<HostGroup>> {
        self.group_storage.get_all().await
    }

    /// Update group
    pub async fn update_group(&self, mut group: HostGroup) -> Result<HostGroup> {
        let now = chrono::Utc::now();
        group.updated_at = now;
        
        // Validate that all hosts in sequence exist
        for host_id in &group.base.hosts {
            if self.host_service.get_host(host_id).await?.is_none() {
                return Err(anyhow::anyhow!("Host with id '{}' not found", host_id));
            }
        }

        // Get old group to compare host sequences
        let old_group = self.get_group(&group.id).await?
            .ok_or_else(|| anyhow::anyhow!("Group not found"))?;

        self.group_storage.update(&group).await?;

        // Update host group references
        // Remove group from hosts no longer in sequence
        for old_host_id in &old_group.base.hosts {
            if !group.base.hosts.contains(old_host_id) {
                if let Some(host) = self.host_service.get_host(old_host_id).await? {
                    if !host.base.groups.contains(&group.id) {
                        continue; // Not in group
                    }
                    let groups = host.base.groups.into_iter().filter(|g| g != &group.id).collect();
                    let update = HostUpdateRequest::from_group_change(groups);
                    self.host_service.update_host(&host.id, update).await?;
                }
            }
        }

        // Add group to new hosts in sequence
        for new_host_id in &group.base.hosts {
            if !old_group.base.hosts.contains(new_host_id) {
                if let Some(host) = self.host_service.get_host(new_host_id).await? {
                    
                    if host.base.groups.contains(&group.id) {
                        continue; // Already in group
                    }
                    let mut groups = host.base.groups;
                    groups.push(group.id);
                    let update = HostUpdateRequest::from_group_change(groups);

                    self.host_service.update_host(&host.id, update).await?;
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
        for host_id in &group.base.hosts {
            if let Some(host) = self.host_service.get_host(host_id).await? {
                if !host.base.groups.contains(&group.id) {
                    continue; // Not in group
                }
                let groups = host.base.groups.into_iter().filter(|g| g != &group.id).collect();
                let update = HostUpdateRequest::from_group_change(groups);
                self.host_service.update_host(&host.id, update).await?;
            }
        }

        self.group_storage.delete(id).await?;
        Ok(())
    }
}