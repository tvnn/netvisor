use anyhow::{Result};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use crate::server::{hosts::{service::HostService, types::api::HostUpdateRequest}, services::{storage::ServiceStorage, types::base::Service}
};

pub struct ServiceService {
    storage: Arc<dyn ServiceStorage>,
    host_service: Arc<Mutex<Option<Arc<HostService>>>>
}

impl ServiceService {
    pub fn new(storage: Arc<dyn ServiceStorage>) -> Self {
        Self { 
            storage,
            host_service: Arc::new(Mutex::new(None))
        }
    }

    pub fn set_host_service(&self, host_service: Arc<HostService>) {
        *self.host_service.lock().unwrap() = Some(host_service);
    }

    pub async fn create_service(&self, service: Service) -> Result<Service> {
        self.storage.create(&service).await?;
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

    pub async fn update_service(&self, mut service: Service) -> Result<Service> {
        service.updated_at = chrono::Utc::now();
        self.storage.update(&service).await?;
        Ok(service)
    }

    pub async fn update_multiple_services(&self, mut services: Vec<Service>) -> Result<Vec<Service>> {
        let now = chrono::Utc::now();
        services.iter_mut().for_each(|s| s.updated_at = now);
        self.storage.update_multiple(services.as_ref()).await?;
        Ok(services)
    }

    pub async fn delete_service(&self, id: &Uuid) -> Result<()> {

        let host_service = self.host_service
            .lock()
            .unwrap()
            .as_ref()
            .expect("Host service not initialized")
            .clone();

        let service = self.get_service(&id).await?.ok_or_else(|| anyhow::anyhow!("Service {} not found", id))?;
        let host = host_service.get_host(&service.base.host_id).await?.ok_or_else(|| anyhow::anyhow!("Host {} not found", service.base.host_id))?;
        
        let host_update = HostUpdateRequest {
            name: None,
            hostname: None,
            description: None,
            target: None,
            interfaces: None,
            services: Some(host.base.services.iter().filter(|s| *s != id).cloned().collect()),
            open_ports: None,
            groups: None,
        };

        host_service.update_host(&host.id, host_update).await?;

        self.storage.delete(id).await
    }
}