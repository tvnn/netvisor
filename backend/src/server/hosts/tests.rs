use uuid::Uuid;

use crate::{
    server::{
        discovery::types::base::{DiscoveryType, EntitySource},
        services::types::bindings::Binding,
    },
    tests::*,
};

#[tokio::test]
async fn test_host_deduplication_on_create() {
    let (storage, services) = test_services().await;

    let start_host_count = storage.hosts.get_all().await.unwrap().len();

    // Create first host
    let mut host1 = host();
    host1.base.source = EntitySource::Discovery(DiscoveryType::Network, Uuid::new_v4());
    let (created1, _) = services
        .host_service
        .create_host_with_services(host1.clone(), vec![])
        .await
        .unwrap();

    // Try to create duplicate (same interfaces)
    let mut host2 = host();
    host2.base.source = EntitySource::Discovery(DiscoveryType::Network, Uuid::new_v4());
    let (created2, _) = services
        .host_service
        .create_host_with_services(host2.clone(), vec![])
        .await
        .unwrap();

    // Should return same host (upserted)
    assert_eq!(created1.id, created2.id);

    // Verify only one host in DB
    let end_host_count = storage.hosts.get_all().await.unwrap().len();
    assert_eq!(start_host_count + 1, end_host_count);
}

#[tokio::test]
async fn test_host_upsert_merges_new_data() {
    let (_, services) = test_services().await;

    // Create host with one interface
    let mut host1 = host();
    host1.base.source = EntitySource::Discovery(DiscoveryType::Network, Uuid::new_v4());
    let subnet1 = subnet();
    services
        .subnet_service
        .create_subnet(subnet1.clone())
        .await
        .unwrap();
    host1.base.interfaces = vec![interface(&subnet1.id)];

    let (created, _) = services
        .host_service
        .create_host_with_services(host1.clone(), vec![])
        .await
        .unwrap();

    // Create "duplicate" with additional interface
    let mut host2 = host();
    host2.base.source = EntitySource::Discovery(DiscoveryType::Network, Uuid::new_v4());
    let subnet2 = subnet();
    services
        .subnet_service
        .create_subnet(subnet2.clone())
        .await
        .unwrap();
    host2.base.interfaces = vec![interface(&subnet1.id), interface(&subnet2.id)];

    let (upserted, _) = services
        .host_service
        .create_host_with_services(host2.clone(), vec![])
        .await
        .unwrap();

    // Should have merged interfaces
    assert_eq!(upserted.id, created.id);
    assert_eq!(upserted.base.interfaces.len(), 2);
}

#[tokio::test]
async fn test_host_consolidation() {
    let (_, services) = test_services().await;

    let subnet_obj = subnet();
    services
        .subnet_service
        .create_subnet(subnet_obj.clone())
        .await
        .unwrap();

    let mut host1 = host();
    host1.base.interfaces = Vec::new();

    let (created1, _) = services
        .host_service
        .create_host_with_services(host1.clone(), vec![])
        .await
        .unwrap();

    let mut host2 = host();
    host2.base.interfaces = vec![interface(&subnet_obj.id)];

    let mut svc = service(&host2.id);
    svc.base.bindings = vec![Binding::new_l4(
        host2.base.ports[0].id,
        Some(host2.base.interfaces[0].id),
    )];

    let (created2, created_svcs) = services
        .host_service
        .create_host_with_services(host2.clone(), vec![svc])
        .await
        .unwrap();

    let created_svc = &created_svcs[0];

    // Consolidate host2 into host1
    let consolidated = services
        .host_service
        .consolidate_hosts(created1.clone(), created2.clone())
        .await
        .unwrap();

    // Host1 should have host2's service
    assert!(consolidated.base.services.contains(&created_svc.id));

    // Host2 should be deleted
    let host2_after = services.host_service.get_host(&created2.id).await.unwrap();
    assert!(host2_after.is_none());

    // Service should now belong to host1
    let svc_after = services
        .service_service
        .get_service(&created_svc.id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(svc_after.base.host_id, consolidated.id);
}

#[tokio::test]
async fn test_host_deletion_removes_subnet_relationships() {
    let (_, services) = test_services().await;

    let subnet_obj = subnet();
    let created_subnet = services
        .subnet_service
        .create_subnet(subnet_obj.clone())
        .await
        .unwrap();

    // Create host with interface on subnet
    let mut host_obj = host();
    host_obj.base.interfaces = vec![interface(&created_subnet.id)];
    let (created_host, _) = services
        .host_service
        .create_host_with_services(host_obj.clone(), vec![])
        .await
        .unwrap();

    // Subnet should have host relationship
    let subnet_after_create = services
        .subnet_service
        .get_subnet(&created_subnet.id)
        .await
        .unwrap()
        .unwrap();
    assert!(subnet_after_create.base.hosts.contains(&created_host.id));

    // Delete host (with services)
    services
        .host_service
        .delete_host(&created_host.id, true)
        .await
        .unwrap();

    // Subnet should no longer have host relationship
    let subnet_after_delete = services
        .subnet_service
        .get_subnet(&created_subnet.id)
        .await
        .unwrap()
        .unwrap();
    assert!(!subnet_after_delete.base.hosts.contains(&created_host.id));
}
