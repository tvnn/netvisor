use std::net::{IpAddr};
use async_trait::async_trait;
use mac_address::MacAddress;
use anyhow::{anyhow, Error, Result};

#[cfg(any(target_os = "macos"))]
pub struct MacOsSystemUtils;

#[cfg(any(target_os = "macos"))]
impl MacOsSystemUtils {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse MAC address from macOS format (handles missing leading zeros)
    fn parse_macos_mac_address(&self, mac_str: &str) -> Result<MacAddress, Error> {
        let parts: Vec<&str> = mac_str.split(':').collect();
        if parts.len() != 6 {
            return Err(anyhow!("Invalid MAC address format: {}", mac_str));
        }
        
        let mut mac_bytes = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            // Handle macOS format where leading zeros are omitted (e.g., "0:22:7" instead of "00:22:07")
            mac_bytes[i] = u8::from_str_radix(part, 16)
                .map_err(|_| anyhow!("Invalid hex in MAC address: {}", part))?;
        }
        
        Ok(MacAddress::new(mac_bytes))
    }
}

#[cfg(target_os = "macos")]
use crate::daemon::utils::base::SystemUtils;

#[async_trait]
impl SystemUtils for MacOsSystemUtils {
    async fn get_mac_address_for_ip(&self, ip: IpAddr) -> Result<Option<MacAddress>, Error> {
        use tokio::process::Command;
        
        let output = Command::new("arp")
            .args(&["-n", &ip.to_string()])
            .output()
            .await?;
            
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            // Parse macOS arp output: "? (192.168.1.1) at 0:22:7:4a:21:d5 on en0 ifscope [ethernet]"
            for line in output_str.lines() {
                if line.contains(&ip.to_string()) {
                    // Look for "at MAC_ADDRESS" pattern
                    if let Some(at_pos) = line.find(" at ") {
                        let after_at = &line[at_pos + 4..];
                        if let Some(space_pos) = after_at.find(' ') {
                            let mac_str = &after_at[..space_pos];
                            if mac_str.contains(':') && mac_str.matches(':').count() == 5 {
                                return Ok(Some(self.parse_macos_mac_address(mac_str)?));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(None)
    }

    async fn scan_own_tcp_ports(&self) -> Result<Vec<u16>, Error> {
        self.scan_tcp_ports(self.get_own_ip_address()?).await
    }

    async fn scan_own_udp_ports(&self) -> Result<Vec<u16>, Error> {
        use tokio::process::Command;
        
        let output = Command::new("lsof")
            .args(&["-Pn", "-i4UDP"])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to run lsof command"));
        }
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut ports = Vec::new();
        
        for line in output_str.lines() {
            // UDP doesn't have LISTEN state, look for local bindings
            if line.contains("UDP") && !line.contains("->") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(name_part) = parts.last() {
                    if let Some(port_start) = name_part.rfind(':') {
                        let port_str = &name_part[port_start + 1..];
                        if let Ok(port) = port_str.parse::<u16>() {
                            ports.push(port);
                        }
                    }
                }
            }
        }
        
        ports.sort_unstable();
        ports.dedup();
        
        Ok(ports)
    }
}

// =====