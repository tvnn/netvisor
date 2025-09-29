#[cfg(target_os = "linux")]
use crate::daemon::utils::base::DaemonUtils;
#[cfg(target_os = "linux")]
use crate::server::utils::base::NetworkUtils;

#[cfg(target_os = "linux")]
pub struct LinuxDaemonUtils;

#[cfg(target_os = "linux")]
impl LinuxDaemonUtils {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "linux")]
impl NetworkUtils for LinuxDaemonUtils {
    fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "linux")]
use anyhow::{anyhow, Error, Result};
#[cfg(target_os = "linux")]
use async_trait::async_trait;
#[cfg(target_os = "linux")]
use mac_address::MacAddress;
#[cfg(target_os = "linux")]
use std::net::IpAddr;
#[cfg(target_os = "linux")]
#[async_trait]
impl DaemonUtils for LinuxDaemonUtils {
    async fn get_mac_address_for_ip(&self, ip: IpAddr) -> Result<Option<MacAddress>, Error> {
        use procfs::net;

        let ipv4_addr = match ip {
            IpAddr::V4(addr) => addr,
            IpAddr::V6(_) => return Ok(None), // IPv6 ARP not supported yet
        };

        let arp_table = net::arp()
            .map_err(|e| anyhow!("Failed to read ARP table from /proc/net/arp: {}", e))?;

        for entry in arp_table {
            if entry.ip_address == ipv4_addr {
                if let Some(hw_addr) = entry.hw_address {
                    let mac = MacAddress::new(hw_addr);
                    return Ok(Some(mac));
                }
            }
        }

        Ok(None)
    }
}
