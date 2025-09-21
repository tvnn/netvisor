use anyhow::{Error, Result};
use futures::future::{try_join_all};
use uuid::Uuid;
use std::sync::{Arc, OnceLock};
use crate::server::{hosts::{service::HostService, types::{api::HostUpdateRequest}}, services::{storage::ServiceStorage, types::base::{Service, ServiceUpdateRequest}}, subnets::service::SubnetService
};

pub struct ServiceService {
    storage: Arc<dyn ServiceStorage>,
    subnet_service: Arc<SubnetService>,
    host_service: OnceLock<Arc<HostService>>
}

impl ServiceService {
    pub fn new(storage: Arc<dyn ServiceStorage>, subnet_service: Arc<SubnetService>) -> Self {
        Self { 
            storage,
            subnet_service,
            host_service: OnceLock::new()
        }
    }

    pub fn set_host_service(&self, host_service: Arc<HostService>) -> Result<(), Arc<HostService>>{
        self.host_service.set(host_service)
    }

    pub async fn create_service(&self, service: Service) -> Result<Service> {
        self.storage.create(&service).await?;
        self.update_subnet_service_relationships(&service, false).await?;
        Ok(service)
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

    pub async fn update_service(&self, mut service: Service, updates: ServiceUpdateRequest) -> Result<Service> {

        self.update_subnet_service_relationships(&service, true).await?;

        if let Some(host_id) = updates.host_id {
            service.base.host_id = host_id;
        };

        if let Some(service_type) = updates.service_type {
            service.base.service_type = service_type;
        };

        if let Some(groups) = updates.groups {
            service.base.groups = groups;
        }

        if let Some(name) = updates.name {
            service.base.name = name;
        };

        if let Some(ports) = updates.ports {
            service.base.ports = ports;
        };

        if let Some(interface_bindings) = updates.interface_bindings {
            service.base.interface_bindings = interface_bindings;
        };

        self.update_subnet_service_relationships(&service, false).await?;

        service.updated_at = chrono::Utc::now();
        
        self.storage.update(&service).await?;
        Ok(service)
    }

    pub async fn update_subnet_service_relationships(&self, service: &Service, remove: bool) -> Result<(), Error> {
        let host_service = self.host_service.get().ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;

        let subnet_ids: Vec<Uuid> = match host_service.get_host(&service.base.host_id).await? {
            Some(host) => {
                 service.base.interface_bindings.iter()
                    .filter_map(|b| host.get_interface(b))
                    .map(|i| i.base.subnet_id)
                    .collect()
            },
            None => Vec::new()
        };       
        
        if let Ok(mut subnets) = self.subnet_service.get_by_ids(&subnet_ids).await {
            let subnet_futures: Vec<_> = subnets
                .iter_mut()
                .map(|subnet| {
                        
                        if remove { subnet.remove_service_relationships(service) }
                        else { subnet.create_service_relationships(service) };
                        
                        return self.subnet_service.update_subnet(subnet.clone())
                    }
                )
                .collect();

            try_join_all(subnet_futures).await?;
        };
        Ok(())
    }

    pub async fn delete_service(&self, id: &Uuid, update_host: bool) -> Result<()> {

        let service = self.get_service(&id).await?.ok_or_else(|| anyhow::anyhow!("Service {} not found", id))?;
        self.update_subnet_service_relationships(&service, true).await?;

        if update_host {
            let host_service = self.host_service.get().ok_or_else(|| anyhow::anyhow!("Host service not initialized"))?;

            let host = host_service.get_host(&service.base.host_id).await?.ok_or_else(|| anyhow::anyhow!("Host {} not found", service.base.host_id))?;

            let host_update = HostUpdateRequest {
                name: None,
                hostname: None,
                description: None,
                target: None,
                interfaces: None,
                services: Some(host.base.services.iter().filter(|s| *s != id).cloned().collect()),
                open_ports: None,
            };

            host_service.update_host(&host.id, host_update).await?;
        };

        self.storage.delete(id).await
    }
}