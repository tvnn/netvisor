use anyhow::{Error, Result};
use futures::future::try_join_all;
use uuid::Uuid;
use std::{sync::Arc};
use crate::server::{
    host_groups::storage::HostGroupStorage, hosts::{

        storage::HostStorage,
        types::{
            api::{HostSubnetRelationshipChange, HostUpdateRequest}, base::{Host, HostBase}
        }
    }, subnets::{service::SubnetService, types::base::{Subnet}}, utils::base::{NetworkUtils, ServerNetworkUtils}
};

pub struct HostService {
    storage: Arc<dyn HostStorage>,
    group_storage: Arc<dyn HostGroupStorage>,
    subnet_service: Arc<SubnetService>,
    utils: ServerNetworkUtils
}

impl HostService {
    pub fn new(storage: Arc<dyn HostStorage>, group_storage: Arc<dyn HostGroupStorage>, subnet_service: Arc<SubnetService>, utils: ServerNetworkUtils) -> Self {
        Self { 
            storage,
            group_storage,
            subnet_service,
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
                let host = self.update_primary_interface(host).await?;
                self.storage.create(&host).await?;
                self.update_subnet_host_relationships(&host).await?;
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

    pub async fn update_host(&self, id: &Uuid, updates: HostUpdateRequest) -> Result<(Host, HostSubnetRelationshipChange), Error> {
        
        let mut host = match self.get_host(&id).await? {
            Some(n) => n,
            None => {
                let msg = format!("Host '{}' not found", id);
                return Err(Error::msg(msg));
            },
        };

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
        if let Some(interfaces) = updates.interfaces {
            host.base.interfaces = interfaces;
        }
        if let Some(services) = updates.services {
            host.base.services = services;
        }

        let subnet_relationship_changes = self.update_subnet_host_relationships(&host).await?;
        
        host.updated_at = chrono::Utc::now();

        let host = self.update_primary_interface(host).await?;

        self.storage.update(&host).await?;
        Ok((host, subnet_relationship_changes))
    }

    pub async fn update_primary_interface(&self, mut host: Host) -> Result<Host> {
        let server_ip = self.utils.get_own_ip_address()?;
        let subnet_ids: Vec<Uuid> = host.base.interfaces.iter().map(|interface| interface.base.subnet_id).collect();
        let subnets = self.subnet_service.get_by_ids(&subnet_ids).await?;

        if host.base.interfaces.len() == 1 { 
            host.base.interfaces[0].base.is_primary = true;
            return Ok(host)
        } else {
            host.base.interfaces.iter_mut().for_each( |interface| {
                if let Some(cidr) = subnets.iter().find_map(|sub| if interface.base.subnet_id == sub.id {Some(sub.base.cidr)} else {None}) {
                    if cidr.contains(&server_ip) {
                        interface.base.is_primary = true;
                    }
                }
            });

            return Ok(host)
        }
    }

    pub async fn update_subnet_host_relationships(&self, host: &Host) -> Result<HostSubnetRelationshipChange, Error> {
        let subnet_ids: Vec<Uuid> = host.base.interfaces.iter().map(|interface| interface.base.subnet_id).collect();

        let mut new_gateway: Vec<Subnet> = Vec::new();
        let mut no_longer_gateway: Vec<Subnet>  = Vec::new();
        let mut new_dns_resolver: Vec<Subnet> = Vec::new();
        let mut no_longer_dns_resolver: Vec<Subnet>  = Vec::new();

        if let Ok(mut subnets) = self.subnet_service.get_by_ids(&subnet_ids).await {
            subnets.iter_mut()
                .for_each(|subnet| {

                let original_dns_resolver_count = subnet.base.dns_resolvers.len();
                let original_gateway_count = subnet.base.gateways.len();
                // let original_reverse_proxy_count = subnet.base.reverse_proxies.len();
                                
                subnet.update_host_relationships(host);

                let new_dns_resolver_count = subnet.base.dns_resolvers.len();
                let new_gateway_count = subnet.base.gateways.len();
                // let new_reverse_proxy_count = subnet.base.reverse_proxies.len();

                if original_dns_resolver_count < new_dns_resolver_count {new_dns_resolver.push(subnet.clone())} else if original_dns_resolver_count > new_dns_resolver_count {no_longer_dns_resolver.push(subnet.clone())}
                if original_gateway_count < new_gateway_count {new_gateway.push(subnet.clone())} else if original_gateway_count > new_gateway_count {no_longer_gateway.push(subnet.clone())}
            });

            let subnet_futures = subnets.into_iter().map(|subnet| self.subnet_service.update_subnet(subnet));
            try_join_all(subnet_futures).await?;
        };

        Ok(HostSubnetRelationshipChange {
            new_gateway,
            no_longer_gateway,
            new_dns_resolver,
            no_longer_dns_resolver,
        })
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

        let all_groups = self.group_storage.get_all().await?;
    
        // Remove host from all groups that contain it
        for mut group in all_groups {
            if group.base.hosts.contains(&id) {
                group.base.hosts.retain(|seq_id| seq_id != id);
                group.updated_at = chrono::Utc::now();
                self.group_storage.update(&group).await?;
            }
        }

        self.storage.delete(id).await
    }
}