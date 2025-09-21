use anyhow::{anyhow, Error, Result};
use futures::future::try_join_all;
use itertools::Itertools;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    groups::service::GroupService, hosts::{

        storage::HostStorage,
        types::{
            api::HostUpdateRequest, base::{Host, HostBase}
        }
    }, interfaces::types::base::{Interface, InterfaceBase}, services::{service::ServiceService, types::base::{Service, ServiceUpdateRequest}}, subnets::service::SubnetService
};

pub struct HostService {
    storage: Arc<dyn HostStorage>,
    host_group_service: Arc<GroupService>,
    subnet_service: Arc<SubnetService>,
    service_service: Arc<ServiceService>,
}

impl HostService {
    pub fn new(storage: Arc<dyn HostStorage>, host_group_service: Arc<GroupService>, subnet_service: Arc<SubnetService>, service_service: Arc<ServiceService>) -> Self {
        Self { 
            storage,
            host_group_service,
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

    pub async fn update_host(&self, id: &Uuid, updates: HostUpdateRequest) -> Result<Host, Error> {
        
        let mut host = self.get_host(&id).await?.ok_or_else(||anyhow!("Host '{}' not found", id))?;

        self.update_subnet_host_relationships(&host, true).await?;

        if let Some(name) = updates.name {
            host.base.name = name;
        }
        if let Some(description) = updates.description {
            host.base.description = description;
        }
        if let Some(target) = updates.target {
            host.base.target = target;
        }
        if let Some(hostname) = updates.hostname {
            host.base.hostname = hostname;
        }  

        if let Some(interfaces) = updates.interfaces {
            host.base.interfaces = interfaces;
        }
        if let Some(services) = updates.services {
            host.base.services = services;
        }

        self.update_subnet_host_relationships(&host, false).await?;
        
        host.updated_at = chrono::Utc::now();

        self.storage.update(&host).await?;
        Ok(host)
    }

    pub async fn consolidate_hosts(&self, destination_host: Host, other_host: Host) -> Result<Host> {
        let mut new_interfaces: Vec<Interface> = Vec::new();
        let other_host_services = self.service_service.get_services_for_host(&other_host.id).await?;

        let other_host_services_updates: Vec<(Service, ServiceUpdateRequest)> = other_host_services.into_iter().map(|s| {
                                    
            // Update bindings - check for subnet compatibility, not interface ID matching
            let updated_interface_bindings = s.base.interface_bindings.iter().filter_map(|binding_id| {
                
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

            (
                s,
                ServiceUpdateRequest {
                    host_id: Some(destination_host.id),
                    interface_bindings: Some(updated_interface_bindings),
                    service_type: None,
                    name: None,
                    ports: None,
                    groups: None
                }
            )
        })
        .collect();

        let update_request = HostUpdateRequest {
            name: None,
            hostname: None,
            description: None,
            target: None,
            interfaces: Some([destination_host.base.interfaces, new_interfaces].concat()),
            services: Some([destination_host.base.services, other_host_services_updates.iter().map(|(s,_)| s.id).collect()].concat()),
            open_ports: None,
        };

        let service_update_futures = other_host_services_updates.into_iter().map(|(s, update)| {
            self.service_service.update_service(s, update)
        });

        try_join_all(service_update_futures).await?;

        let updated_host = self.update_host(
            &destination_host.id, 
            update_request, 
        ).await?;

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

        let all_groups = self.host_group_service.get_all_groups().await?;
    
        // Remove host from all groups that contain it
        for mut group in all_groups {
            if group.base.services.contains(&id) {
                group.base.services.retain(|seq_id| seq_id != id);
                group.updated_at = chrono::Utc::now();
                self.host_group_service.update_group(group).await?;
            }
        }

        self.storage.delete(id).await
    }
}