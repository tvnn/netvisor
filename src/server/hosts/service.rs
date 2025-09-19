use anyhow::{anyhow, Error, Result};
use futures::future::try_join_all;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    host_groups::{service::HostGroupService}, hosts::{

        storage::HostStorage,
        types::{
            api::{HostUpdateRequest}, base::{Host, HostBase}
        }
    }, services::service::ServiceService, subnets::{service::SubnetService}, utils::base::ServerNetworkUtils
};

pub struct HostService {
    storage: Arc<dyn HostStorage>,
    host_group_service: Arc<HostGroupService>,
    subnet_service: Arc<SubnetService>,
    service_service: Arc<ServiceService>,
    utils: ServerNetworkUtils
}

impl HostService {
    pub fn new(storage: Arc<dyn HostStorage>, host_group_service: Arc<HostGroupService>, subnet_service: Arc<SubnetService>, service_service: Arc<ServiceService>, utils: ServerNetworkUtils) -> Self {
        Self { 
            storage,
            host_group_service,
            subnet_service,
            service_service,
            utils
        }
    }

    /// Create a new host
    pub async fn create_host(&self, host_base: HostBase) -> Result<Host> {
        
        let host = Host::new(host_base);

        let all_hosts = self.storage.get_all().await?;

        let host_from_storage = match all_hosts.iter().find(|h| host.eq(h)) {
            Some(existing_host) => {
                existing_host.clone()
            }
            None => {
                self.storage.create(&host).await?;
                self.update_subnet_host_relationships(&host, false).await?;
                host
            }
        };

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

        if let Some(name) = updates.name {
            host.base.name = name;
        }
        if let Some(description) = updates.description {
            host.base.description = description;
        }
        if let Some(target) = updates.target {
            host.base.target = target;
        }
        if let Some(groups) = updates.groups {
            host.base.groups = groups;
        }
        if let Some(hostname) = updates.hostname {
            host.base.hostname = hostname;
        }  

        self.update_subnet_host_relationships(&host, true).await?;

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

    pub async fn update_subnet_host_relationships(&self, host: &Host, remove: bool) -> Result<(), Error> {
        let subnet_ids: Vec<Uuid> = host.base.interfaces.iter().map(|interface| interface.base.subnet_id).collect();
        let services = self.service_service.get_services_for_host(&host.id).await?;

        if let Ok(mut subnets) = self.subnet_service.get_by_ids(&subnet_ids).await {
            let subnet_futures: Vec<_> = subnets
                .iter_mut()
                .map(|subnet| {
                        
                        if remove { subnet.remove_host_relationships(host) }
                        else { subnet.create_host_relationships(host, services.iter().map(|s| s).collect()) };
                        
                        return self.subnet_service.update_subnet(subnet.clone())
                    }
                )
                .collect();

            try_join_all(subnet_futures).await;
        };
        Ok(())
    }

    pub async fn delete_host(&self, id: &Uuid) -> Result<()> {

        let mut subnets = self.subnet_service.get_all_subnets().await?;
        let update_futures = subnets
            .iter_mut()
            .filter_map(|s| {
                let has_host_as_dns = s.base.dns_resolvers.iter().find(|n_id| n_id == &id).is_some();
                let has_host_as_gateway = s.base.gateways.iter().find(|n_id| n_id == &id).is_some();
                let has_host = s.base.hosts.iter().find(|n_id| n_id == &id).is_some();
                if has_host_as_dns {
                    s.base.dns_resolvers = s.base.dns_resolvers.iter().filter(|n_id| n_id != &id).cloned().collect();
                }
                if has_host_as_gateway {
                    s.base.gateways = s.base.gateways.iter().filter(|n_id| n_id != &id).cloned().collect();
                }
                if has_host {
                    s.base.hosts = s.base.hosts.iter().filter(|n_id| n_id != &id).cloned().collect();
                }
                if has_host_as_dns || has_host_as_gateway || has_host {
                    return Some(self.subnet_service.update_subnet(s.clone()));
                }
                None
            });

        try_join_all(update_futures).await?;

        let all_groups = self.host_group_service.get_all_groups().await?;
    
        // Remove host from all groups that contain it
        for mut group in all_groups {
            if group.base.hosts.contains(&id) {
                group.base.hosts.retain(|seq_id| seq_id != id);
                group.updated_at = chrono::Utc::now();
                self.host_group_service.update_group(group).await?;
            }
        }

        self.storage.delete(id).await
    }
}