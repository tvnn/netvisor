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
    networks::types::{Network, NetworkBase},
    services::{
        definitions::ServiceDefinitionRegistry,
        types::base::{Service, ServiceBase},
    },
    shared::{services::ServiceFactory, types::storage::StorageFactory},
    subnets::types::base::{Subnet, SubnetBase, SubnetType},
    users::types::{User, UserBase},
    utils::base::{NetworkUtils, ServerNetworkUtils},
};
use axum::Router;
use cidr::IpCidr;
use cidr::Ipv4Cidr;
use mac_address::MacAddress;
use sqlx::{PgPool};
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::sync::Arc;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use uuid::Uuid;

#[cfg(test)]
pub mod database;

pub const DAEMON_CONFIG_FIXTURE: &str = "src/tests/daemon_config.json";
pub const SERVER_DB_FIXTURE: &str = "src/tests/netvisor.sql";

pub async fn setup_test_db() -> (PgPool, String) {
    let postgres_image = GenericImage::new("postgres", "16-alpine")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_PASSWORD", "password")
        .with_env_var("POSTGRES_DB", "netvisor_test");

    let container = postgres_image.start().await.unwrap();
    
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let database_url = format!(
        "postgresql://postgres:password@localhost:{}/netvisor_test",
        port
    );

    // Leak the container so it lives for the entire test
    std::mem::forget(container);

    let pool = PgPool::connect(&database_url).await.unwrap();
    (pool, database_url)
}

pub async fn test_storage() -> StorageFactory {
    let (pool, database_url) = setup_test_db().await;
    pool.close().await;
    let factory = StorageFactory::new(&database_url).await.unwrap();
    factory
}

pub fn user() -> User {
    User::new(UserBase {
        name: "Test User".to_string(),
    })
}

pub fn network(user_id: &Uuid) -> Network {
    Network::new(NetworkBase::new(*user_id))
}

pub fn host(network_id: &Uuid) -> Host {
    Host::new(HostBase {
        name: "Test Host".to_string(),
        hostname: Some("test.local".to_string()),
        network_id: *network_id,
        description: None,
        target: HostTarget::Hostname,
        interfaces: vec![interface(&Uuid::new_v4())],
        services: vec![],
        ports: vec![Port::new(PortBase::new_tcp(22))],
        source: EntitySource::System,
        virtualization: None,
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

pub fn subnet(network_id: &Uuid) -> Subnet {
    Subnet::new(SubnetBase {
        name: "Test Subnet".to_string(),
        description: None,
        network_id: *network_id,
        cidr: IpCidr::V4(Ipv4Cidr::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap()),
        subnet_type: SubnetType::Lan,
        source: EntitySource::System,
    })
}

pub fn service(network_id: &Uuid, host_id: &Uuid) -> Service {
    let service_def = ServiceDefinitionRegistry::find_by_id("Dns Server")
        .unwrap_or_else(|| ServiceDefinitionRegistry::all_service_definitions()[0].clone());

    Service::new(ServiceBase {
        name: "Test Service".to_string(),
        host_id: *host_id,
        bindings: vec![],
        network_id: *network_id,
        service_definition: service_def,
        virtualization: None,
        vms: vec![],
        containers: vec![],
        source: EntitySource::System,
    })
}

pub fn group(network_id: &Uuid) -> Group {
    Group::new(GroupBase {
        name: "Test Group".to_string(),
        description: None,
        network_id: *network_id,
        group_type: GroupType::NetworkPath,
        service_bindings: vec![],
        source: EntitySource::System,
    })
}

pub fn daemon(network_id: &Uuid, host_id: &Uuid) -> Daemon {
    Daemon::new(
        Uuid::new_v4(),
        DaemonBase {
            host_id: *host_id,
            network_id: *network_id,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 50)),
            port: 60073,
        },
    )
}

pub async fn test_services() -> (StorageFactory, ServiceFactory) {
    let storage = test_storage().await;
    println!("Storage factory created");
    let services = ServiceFactory::new(&storage).await.unwrap();
    println!("Service factory created");
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
