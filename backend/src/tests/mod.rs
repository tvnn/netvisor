use crate::server::{
    config::{AppState, ServerConfig},
    daemons::types::base::{Daemon, DaemonBase},
    discovery::{manager::DiscoverySessionManager, types::base::EntitySource},
    groups::types::{Group, GroupBase, GroupType},
    hosts::types::{
        base::{Host, HostBase},
        interfaces::{Interface, InterfaceBase},
        ports::{Port, PortBase},
        targets::HostTarget,
    },
    services::{
        definitions::ServiceDefinitionRegistry,
        types::base::{Service, ServiceBase},
    },
    shared::{services::ServiceFactory, types::storage::StorageFactory},
    subnets::types::base::{Subnet, SubnetBase, SubnetType},
    utils::base::{NetworkUtils, ServerNetworkUtils},
};
use axum::Router;
use cidr::IpCidr;
use cidr::Ipv4Cidr;
use mac_address::MacAddress;
use sqlx::SqlitePool;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
pub mod database;

pub const DAEMON_CONFIG_FIXTURE: &str = "src/tests/daemon_config.json";
pub const SERVER_DB_FIXTURE: &str = "src/tests/netvisor.db";

pub fn host() -> Host {
    Host::new(HostBase {
        name: "Test Host".to_string(),
        hostname: Some("test.local".to_string()),
        description: None,
        target: HostTarget::Hostname,
        interfaces: vec![interface(&Uuid::new_v4())],
        services: vec![],
        ports: vec![Port::new(PortBase::new_tcp(22))],
        source: EntitySource::System,
    })
}

pub fn interface(subnet_id: &Uuid) -> Interface {
    Interface::new(InterfaceBase {
        subnet_id: *subnet_id,
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        mac_address: Some(MacAddress::new([1, 2, 3, 4, 5, 6])),
        name: Some("eth0".to_string()),
    })
}

pub fn subnet() -> Subnet {
    Subnet::new(SubnetBase {
        name: "Test Subnet".to_string(),
        description: None,
        cidr: IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap()),
        subnet_type: SubnetType::Lan,
        source: EntitySource::System,
        hosts: vec![],
        dns_resolvers: vec![],
        gateways: vec![],
        reverse_proxies: vec![],
    })
}

pub fn service(host_id: &Uuid) -> Service {
    let service_def = ServiceDefinitionRegistry::find_by_id("Dns Server")
        .unwrap_or_else(|| ServiceDefinitionRegistry::all_service_definitions()[0].clone());

    Service::new(ServiceBase {
        name: "Test Service".to_string(),
        host_id: *host_id,
        port_bindings: vec![],
        interface_bindings: vec![],
        service_definition: service_def,
    })
}

pub fn group() -> Group {
    Group::new(GroupBase {
        name: "Test Group".to_string(),
        description: None,
        group_type: GroupType::NetworkPath,
        service_bindings: vec![],
    })
}

pub fn daemon(host_id: &Uuid) -> Daemon {
    Daemon::new(
        Uuid::new_v4(),
        DaemonBase {
            host_id: *host_id,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 50)),
            port: 60073,
        },
    )
}

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    crate::server::shared::storage::DatabaseMigrations::initialize(&pool)
        .await
        .unwrap();
    pool
}

pub async fn test_storage() -> StorageFactory {
    StorageFactory::new_sqlite(":memory:").await.unwrap()
}

pub async fn test_services() -> (StorageFactory, ServiceFactory) {
    let storage = test_storage().await;
    let services = ServiceFactory::new(&storage).await.unwrap();
    (storage, services)
}

pub async fn setup_test_app() -> Router<Arc<AppState>> {
    let config = ServerConfig::default();
    let discovery_manager = DiscoverySessionManager::new();
    let utils = ServerNetworkUtils::new();

    let state = AppState::new(config, discovery_manager, utils)
        .await
        .unwrap();

    crate::server::shared::handlers::create_router().with_state(state)
}
