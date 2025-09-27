use anyhow::{Error, Result};
use futures::future::{try_join_all};
use anyhow::anyhow;
use uuid::Uuid;
use std::sync::{Arc, OnceLock};
use crate::server::{groups::service::GroupService, hosts::{service::HostService, types::base::Host}, services::{storage::ServiceStorage, types::base::Service}, shared::types::metadata::TypeMetadataProvider, subnets::service::SubnetService
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
        let host_service = self.host_service.get().ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;
        let all_hosts = host_service.get_all_hosts().await?;
        let existing_services = self.get_services_for_host(&service.base.host_id).await?;
        
        let service_from_storage = match existing_services.into_iter().find(|existing: &Service| {
            if let (Some(existing_service_host), Some(new_service_host)) = (all_hosts.iter().find(|h| h.id == existing.base.host_id), all_hosts.iter().find(|h| h.id == service.base.host_id)) {
                let port_match = new_service_host.base.ports.iter().any(|p| existing_service_host.base.ports.contains(p));
                let definition_match = service.base.service_definition == existing.base.service_definition;
                return port_match && definition_match
            }
            false
        }) {
            Some(existing_service) => {
                tracing::warn!("Duplicate service for {}: {} found, {}: {} - upserting discovery data...", service.base.name, service.id, existing_service.base.name, existing_service.id);
                self.upsert_service(existing_service, service).await?
            }
            None => {
                self.storage.create(&service).await?;
                self.create_subnet_service_relationships(&service).await?;
                tracing::info!("Created service {} for host {}", 
                    service.base.service_definition.name(), service.base.host_id);
                service
            }
        };

        Ok(service_from_storage)
    }

    pub async fn upsert_service(&self, mut existing_service: Service, new_service: Service) -> Result<Service> {
        for new_service_binding in new_service.base.interface_bindings {
            if !existing_service.base.interface_bindings.contains(&new_service_binding) {
                existing_service.base.interface_bindings.push(new_service_binding);
            }
        }

        for new_service_group in new_service.base.groups {
            if !existing_service.base.groups.contains(&new_service_group) {
                existing_service.base.groups.push(new_service_group)
            }
        }

        for new_service_port in new_service.base.port_bindings {
            if !existing_service.base.port_bindings.contains(&new_service_port) {
                existing_service.base.port_bindings.push(new_service_port)
            }
        }

        self.storage.update(&existing_service).await?;
        tracing::info!("Upserted service {}: {} with new data", existing_service.base.name, existing_service.id);
        Ok(existing_service)
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

    pub fn transfer_service_to_new_host(&self, service: &mut Service, original_host: &Host, new_host: &Host) -> Service {
        
        service.base.interface_bindings = service.base.interface_bindings.iter().filter_map(|b| {
            if let Some(original_binding) = original_host.get_interface(b) {
                return new_host.base.interfaces.iter().find_map(|i| if i == original_binding {Some(i.id)} else {None});
            }
            None
        })
        .collect();

        service.base.port_bindings = service.base.port_bindings.iter().filter_map(|b| {
            if let Some(original_binding ) = original_host.get_port(b) {
                return new_host.base.ports.iter().find_map(|p| if p == original_binding {Some(p.id)} else {None});
            }
            None
        })
        .collect();

        service.base.host_id = new_host.id;

        service.clone()
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
            if group.base.service_bindings.iter().any(|sb| service.id == sb.service_id) {
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