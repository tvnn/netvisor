use inventory;
use crate::server::{services::types::types::ServiceDefinition};
use crate::server::shared::types::metadata::HasId;

#[derive(Debug, Clone, Copy)]
pub struct ServiceDefinitionFactory(pub fn() -> Box<dyn ServiceDefinition>);

impl ServiceDefinitionFactory {
    pub const fn new(factory: fn() -> Box<dyn ServiceDefinition>) -> Self {
        Self(factory)
    }
    
    pub fn create(&self) -> Box<dyn ServiceDefinition> {
        (self.0)()
    }
}

pub fn create_service<T>() -> Box<dyn ServiceDefinition>
where
    T: ServiceDefinition + Default + 'static,
{
    Box::new(T::default())
}

inventory::collect!(ServiceDefinitionFactory);

pub struct ServiceDefinitionRegistry;

impl ServiceDefinitionRegistry {
    /// Get all registered services as instances
    pub fn all_service_definitions() -> Vec<Box<dyn ServiceDefinition>> {
        inventory::iter::<ServiceDefinitionFactory>()
            .map(|factory| factory.create())
            .collect()
    }

    pub fn service_exists(id: &str) -> bool {
        inventory::iter::<ServiceDefinitionFactory>()
            .any(|factory| factory.create().id() == id)
    }

    pub fn find_by_id(id: &str) -> Option<Box<dyn ServiceDefinition>> {
        inventory::iter::<ServiceDefinitionFactory>()
            .find_map(|factory| {
                let service_definition = factory.create();
                if service_definition.id() == id { Some(service_definition) } else { None }
            })
    }
}

pub mod home_assistant;
pub mod plex;
pub mod synology;
pub mod unifi_controller;
pub mod proxmox;
pub mod jellyfin;
pub mod emby;
pub mod netvisor_daemon;
pub mod netvisor_server;
pub mod unbound;
pub mod bind9;
pub mod power_dns;
pub mod portainer;
pub mod custom;
pub mod docker_swarm;
pub mod kubernetes;
pub mod prometheus;
pub mod duplicati;
pub mod syncthing;
pub mod restic;
pub mod wg_dashboard;
pub mod true_nas;
pub mod grafana;
pub mod uptime_kuma;
pub mod pi_hole;
pub mod adguard_home;
pub mod pf_sense;
pub mod opn_sense;
pub mod fortigate;
pub mod unifi_access_point;
pub mod tp_link_eap;
pub mod qnap;
pub mod open_media_vault;
pub mod next_cloud;
pub mod pf_blocker_ng;
pub mod cups;
pub mod traefik;
pub mod nginx_proxy_manager;
pub mod cloudflared;
pub mod hp_printer;
pub mod eero_gateway;
pub mod eero_repeater;
pub mod fios_gateway;
pub mod fios_extender;
pub mod philips_hue_bridge;
pub mod router;
pub mod vpn_gateway;
pub mod nas_device;
pub mod file_server;
pub mod print_server;
pub mod dns_server;
pub mod web_service;
pub mod dhcp_server;
pub mod switch;
pub mod access_point;
pub mod firewall;
pub mod workstation;