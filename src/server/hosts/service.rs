use anyhow::{anyhow, Error, Result};
use futures::future::try_join_all;
use itertools::Itertools;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    hosts::{

        storage::HostStorage,
        types::{
            base::{Host, HostBase}
        }
    }, interfaces::types::base::{Interface, InterfaceBase}, services::{service::ServiceService, types::base::{Service}}, subnets::service::SubnetService
};

pub struct HostService {
    storage: Arc<dyn HostStorage>,
    subnet_service: Arc<SubnetService>,
    service_service: Arc<ServiceService>,
}

impl HostService {
    pub fn new(storage: Arc<dyn HostStorage>, subnet_service: Arc<SubnetService>, service_service: Arc<ServiceService>) -> Self {
        Self { 
            storage,
            subnet_service,
            service_service,
        }
    }

    /// Create a new host
    pub async fn create_host(&self, host_base: HostBase) -> Result<Host> {
        
        let host = Host::new(host_base);

        let all_hosts = self.storage.get_all().await?;

        let host_from_storage = match all_hosts.into_iter().find(|h| host.eq(h)) {
            Some(existing_host) => {
                self.update_subnet_host_relationships(&existing_host, true).await?;
                self.consolidate_hosts(existing_host, host).await?
            }
            None => {
                self.storage.create(&host).await?;
                host
            }
        };

        self.update_subnet_host_relationships(&host_from_storage, false).await?;

        Ok(host_from_storage)
    }

    pub async fn get_host(&self, id: &Uuid) -> Result<Option<Host>> {
        self.storage.get_by_id(id).await
    }

    pub async fn get_all_hosts(&self) -> Result<Vec<Host>> {
        self.storage.get_all().await
    }

    pub async fn update_host(&self, mut host: Host) -> Result<Host, Error> {
        
        let current_host = self.get_host(&host.id).await?.ok_or_else(||anyhow!("Host '{}' not found", host.id))?;

        self.update_subnet_host_relationships(&current_host, true).await?;
        self.update_subnet_host_relationships(&host, false).await?;
        
        host.updated_at = chrono::Utc::now();

        self.storage.update(&host).await?;
        Ok(host)
    }

    pub async fn consolidate_hosts(&self, mut destination_host: Host, other_host: Host) -> Result<Host> {
        let mut new_interfaces: Vec<Interface> = Vec::new();
        let other_host_services = self.service_service.get_services_for_host(&other_host.id).await?;

        let other_host_services_updates: Vec<Service> = other_host_services.into_iter().map(|mut s| {
                                    
            // Update bindings - check for subnet compatibility, not interface ID matching
            s.base.interface_bindings = s.base.interface_bindings.iter().filter_map(|binding_id| {
                
                // Get the original interface from the other host
                if let Some(origin_interface) = other_host.get_interface(binding_id) {
                    
                    // Check if destination host already has an interface on the same subnet
                    if let Some(dest_interface) = destination_host.base.interfaces.iter()
                        .find(|dest_iface| dest_iface.base.subnet_id == origin_interface.base.subnet_id) {
                        
                        // Reuse existing destination interface
                        Some(dest_interface.id)
                        
                    } else {
                        
                        // Check if we've already decided to create this interface
                        if let Some(existing_new) = new_interfaces.iter()
                            .find(|new_iface| new_iface.base.subnet_id == origin_interface.base.subnet_id) {
                                                    
                            Some(existing_new.id)
                            
                        } else {
                            
                            // Create new interface based on origin, but with new ID
                            let new_interface = Interface::new(InterfaceBase {
                                name: origin_interface.base.name.clone(),
                                subnet_id: origin_interface.base.subnet_id,
                                ip_address: origin_interface.base.ip_address,
                                mac_address: None,
                            });
                            
                            let interface_id = new_interface.id;
                            new_interfaces.push(new_interface);
                            Some(interface_id)
                        }
                    }
                } else {
                    tracing::warn!("Could not find interface {} on origin host", binding_id);
                    None
                }
            })
            .collect();

            s.base.host_id = destination_host.id;

            s
        })
        .collect();

        destination_host.base.interfaces = [destination_host.base.interfaces, new_interfaces].concat();
        destination_host.base.services = [destination_host.base.services, other_host_services_updates.iter().map(|s| s.id).collect()].concat();

        let service_update_futures = other_host_services_updates.into_iter().map(|s| {
            self.service_service.update_service(s)
        });

        try_join_all(service_update_futures).await?;

        let updated_host = self.update_host(destination_host).await?;

        self.delete_host(&other_host.id).await?;

        Ok(updated_host)
    }

    pub async fn update_subnet_host_relationships(&self, host: &Host, remove: bool) -> Result<(), Error>{
        let subnet_ids: Vec<Uuid> = host.base.interfaces.iter().map(|i| i.base.subnet_id).unique().collect();
        
        if let Ok(mut subnets) = self.subnet_service.get_by_ids(&subnet_ids).await {
            let subnet_futures: Vec<_> = subnets
                .iter_mut()
                .map(|subnet| {
                        
                        if remove { subnet.remove_host_relationship(host) }
                        else { subnet.create_host_relationship(host) };
                        
                        return self.subnet_service.update_subnet(subnet.clone())
                    }
                )
                .collect();

            try_join_all(subnet_futures).await?;
        };
        Ok(())
    }

    pub async fn delete_host(&self, id: &Uuid) -> Result<()> {

        let host = self.get_host(id).await?.ok_or_else(|| anyhow::anyhow!("Host {} not found", id))?;

        let service_deletion_futures = host.base.services.iter().map(|s| {
            self.service_service.delete_service(s, false)
        });

        try_join_all(service_deletion_futures).await?;
        
        self.update_subnet_host_relationships(&host, true).await?;

        self.storage.delete(id).await
    }
}