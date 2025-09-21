// src/server/shared/services.rs
use std::sync::Arc;
use anyhow::Result;
use crate::server::{
    daemons::service::DaemonService, groups::service::GroupService, hosts::service::HostService, services::service::ServiceService, shared::types::storage::StorageFactory, subnets::service::SubnetService, topology::service::TopologyService
};

pub struct ServiceFactory {
    pub host_service: Arc<HostService>,
    pub group_service: Arc<GroupService>,
    pub subnet_service: Arc<SubnetService>,
    pub daemon_service: Arc<DaemonService>,
    pub topology_service: Arc<TopologyService>,
    pub service_service: Arc<ServiceService>
}

impl ServiceFactory {
    pub async fn new(storage: &StorageFactory) -> Result<Self> {
        // Initialize services with proper dependencies
        let daemon_service = Arc::new(DaemonService::new(storage.daemons.clone()));

        let subnet_service = Arc::new(SubnetService::new(storage.subnets.clone()));
        let service_service = Arc::new(ServiceService::new(storage.services.clone(), subnet_service.clone()));
        let group_service = Arc::new(GroupService::new(storage.host_groups.clone(), service_service.clone()));

        let host_service = Arc::new(HostService::new(
            storage.hosts.clone(),
            group_service.clone(),
            subnet_service.clone(),
            service_service.clone(),
        ));

        let _ = subnet_service.set_host_service(host_service.clone());
        let _ = service_service.set_host_service(host_service.clone());
        let _ = service_service.set_group_service(group_service.clone());

        let topology_service = Arc::new(TopologyService::new(
            host_service.clone(),
            subnet_service.clone(),
            group_service.clone(),
            service_service.clone()
        ));
        
        Ok(Self {
            host_service,
            group_service, 
            subnet_service,
            daemon_service,
            topology_service,
            service_service
        })
    }
}