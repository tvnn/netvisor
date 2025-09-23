use anyhow::{Error, Result};
use futures::future::{try_join_all};
use anyhow::anyhow;
use uuid::Uuid;
use std::sync::{Arc, OnceLock};
use crate::server::{groups::service::GroupService, hosts::service::HostService, services::{storage::ServiceStorage, types::base::Service}, shared::types::metadata::TypeMetadataProvider, subnets::service::SubnetService
};

pub struct ServiceService {
    storage: Arc<dyn ServiceStorage>,
    subnet_service: Arc<SubnetService>,
    host_service: OnceLock<Arc<HostService>>,
    group_service: OnceLock<Arc<GroupService>>
}

impl ServiceService {
    pub fn new(storage: Arc<dyn ServiceStorage>, subnet_service: Arc<SubnetService>) -> Self {
        Self { 
            storage,
            subnet_service,
            host_service: OnceLock::new(),
            group_service: OnceLock::new()
        }
    }

    pub fn set_host_service(&self, host_service: Arc<HostService>) -> Result<(), Arc<HostService>>{
        self.host_service.set(host_service)
    }

     pub fn set_group_service(&self, group_service: Arc<GroupService>) -> Result<(), Arc<GroupService>>{
        self.group_service.set(group_service)
    }

    pub async fn create_service(&self, service: Service) -> Result<Service> {
        let existing_services = self.get_services_for_host(&service.base.host_id).await?;
        
        // Check if a similar service already exists
        let duplicate = existing_services.into_iter().find(|existing| *existing == service);
        
        match duplicate {
            Some(existing_service) => {
                tracing::info!("Service {} already exists for host {}, skipping creation", 
                    service.base.service_definition.name(), service.base.host_id);
                Ok(existing_service.clone())
            }
            None => {
                self.storage.create(&service).await?;
                self.create_subnet_service_relationships(&service).await?;
                tracing::info!("Created service {} for host {}", 
                    service.base.service_definition.name(), service.base.host_id);
                Ok(service)
            }
        }
    }

    pub async fn get_service(&self, id: &Uuid) -> Result<Option<Service>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_services(&self) -> Result<Vec<Service>> {
        self.storage.get_all().await
    }

    pub async fn get_services_for_host(&self, host_id: &Uuid) -> Result<Vec<Service>> {
        self.storage.get_services_for_host(host_id).await
    }

    pub async fn update_service(&self, mut service: Service) -> Result<Service> {

        let current_service = self.get_service(&service.id).await?.ok_or_else(|| anyhow!("Could not find service"))?;
        
        self.remove_subnet_service_relationships(&current_service).await?;
        self.create_subnet_service_relationships(&service).await?;

        service.updated_at = chrono::Utc::now();
        
        self.storage.update(&service).await?;
        tracing::info!("Updated service {}: {} for host {}", service.base.name, service.id, service.base.host_id);
        Ok(service)
    }

    pub async fn remove_subnet_service_relationships(&self, service: &Service) -> Result<(), Error> {
        let all_subnets = self.subnet_service.get_all_subnets().await?;
        let affected_subnets: Vec<_> = all_subnets.into_iter()
            .filter(|subnet| {
                subnet.base.dns_resolvers.contains(&service.id) ||
                subnet.base.gateways.contains(&service.id) ||
                subnet.base.reverse_proxies.contains(&service.id)
            })
            .collect();
            
        let subnet_futures: Vec<_> = affected_subnets
            .into_iter()
            .map(|mut subnet| {
                subnet.remove_service_relationships(service);
                self.subnet_service.update_subnet(subnet)
            })
            .collect();

        try_join_all(subnet_futures).await?;

        Ok(())
    }

    pub async fn create_subnet_service_relationships(&self, service: &Service) -> Result<(), Error> {
        let host_service = self.host_service.get().ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;

        if let Some(host) = host_service.get_host(&service.base.host_id).await? {
            let subnet_ids: Vec<Uuid> = service.base.interface_bindings.iter()
                    .filter_map(|b| host.get_interface(b))
                    .map(|i| i.base.subnet_id)
                    .collect();
           
            if let Ok(subnets) = self.subnet_service.get_by_ids(&subnet_ids).await {
                let subnet_futures: Vec<_> = subnets
                    .into_iter()
                    .map(|mut subnet| {
                            subnet.create_service_relationships(service, &host);
                            return self.subnet_service.update_subnet(subnet)
                        }
                    )
                    .collect();

                try_join_all(subnet_futures).await?;
            };
        }
        Ok(())
    }

    pub async fn delete_service(&self, id: &Uuid, update_host: bool) -> Result<()> {

        let group_service = self.group_service.get().ok_or_else(|| anyhow::anyhow!("Group service not initialized"))?;
        let service = self.get_service(&id).await?.ok_or_else(|| anyhow::anyhow!("Service {} not found", id))?;
        self.remove_subnet_service_relationships(&service).await?;

        let all_groups = group_service.get_all_groups().await?;
    
        // Remove service from all groups that contain it
        for mut group in all_groups {
            if group.base.service_bindings.iter().any(|sb| sb.service_id == *id) {
                group.base.service_bindings.retain(|sb| sb.service_id != *id);
                group.updated_at = chrono::Utc::now();
                group_service.update_group(group).await?;
            }
        }

        if update_host {
            let host_service = self.host_service.get().ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;

            let mut host = host_service.get_host(&service.base.host_id).await?.ok_or_else(|| anyhow::anyhow!("Host {} not found", service.base.host_id))?;
            host.base.services = host.base.services.iter().filter(|s| *s != id).cloned().collect();

            host_service.update_host(host).await?;
        };

        self.storage.delete(id).await?;
        tracing::info!("Deleted service {}: {} for host {}", service.base.name, service.id, service.base.host_id);
        Ok(())
    }
}