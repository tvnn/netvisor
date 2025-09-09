// src/server/shared/services.rs
use std::sync::Arc;
use anyhow::Result;
use crate::server::{
    daemons::service::DaemonService, host_groups::service::HostGroupService, hosts::service::HostService, shared::types::storage::StorageFactory, subnets::service::SubnetService, topology::service::TopologyService, utils::base::{NetworkUtils, ServerNetworkUtils}
};

pub struct ServiceFactory {
    pub host_service: Arc<HostService>,
    pub host_group_service: Arc<HostGroupService>,
    pub subnet_service: Arc<SubnetService>,
    pub daemon_service: Arc<DaemonService>,
    pub topology_service: Arc<TopologyService>
}

impl ServiceFactory {
    pub async fn new(storage: &StorageFactory) -> Result<Self> {
        // Initialize services with proper dependencies
        let utils = ServerNetworkUtils::new();

        let subnet_service = Arc::new(SubnetService::new(storage.subnets.clone()));

        let host_service = Arc::new(HostService::new(
            storage.hosts.clone(),
            storage.host_groups.clone(),
            subnet_service.clone(),
            utils
        ));

        subnet_service.set_host_service(host_service.clone());
        
        let host_group_service = Arc::new(HostGroupService::new(
            storage.host_groups.clone(),
            host_service.clone(),
        ));

        let daemon_service = Arc::new(DaemonService::new(
            storage.daemons.clone(),
            host_service.clone(),
        ));

        let topology_service = Arc::new(TopologyService::new(
            host_service.clone(),
            subnet_service.clone(),
            host_group_service.clone()
        ));
        
        Ok(Self {
            host_service,
            host_group_service, 
            subnet_service,
            daemon_service,
            topology_service
        })
    }
}