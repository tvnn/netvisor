#[cfg(target_os = "linux")]
pub struct LinuxSystemUtils;

#[cfg(target_os = "linux")]
impl LinuxSystemUtils {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "linux")]
#[async_trait]
impl SystemUtils for LinuxSystemUtils {
    async fn get_arp_entry_for_ip(&self, ip: IpAddr) -> Result<Option<MacAddress>> {
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

    async fn scan_own_tcp_ports(&self) -> Result<Vec<u16>> {
        use procfs::net;
        
        let mut listening_ports = Vec::new();
        
        // Get IPv4 TCP listening ports
        let tcp4_table = net::tcp()
            .map_err(|e| anyhow!("Failed to read TCP table from /proc/net/tcp: {}", e))?;
            
        for entry in tcp4_table {
            if entry.state == procfs::net::TcpState::Listen {
                listening_ports.push(entry.local_address.port());
            }
        }
        
        // Get IPv6 TCP listening ports
        let tcp6_table = net::tcp6()
            .map_err(|e| anyhow!("Failed to read TCP6 table from /proc/net/tcp6: {}", e))?;
            
        for entry in tcp6_table {
            if entry.state == procfs::net::TcpState::Listen {
                listening_ports.push(entry.local_address.port());
            }
        }
        
        listening_ports.sort_unstable();
        listening_ports.dedup();
        
        Ok(listening_ports)
    }

    async fn scan_own_udp_ports(&self) -> Result<Vec<u16>> {
        use procfs::net;
        
        let mut listening_ports = Vec::new();
        
        // Get IPv4 UDP listening ports
        let udp4_table = net::udp()
            .map_err(|e| anyhow!("Failed to read UDP table from /proc/net/udp: {}", e))?;
        
        for entry in udp4_table {
            listening_ports.push(entry.local_address.port());
        }
        
        // Get IPv6 UDP listening ports
        let udp6_table = net::udp6()
            .map_err(|e| anyhow!("Failed to read UDP6 table from /proc/net/udp6: {}", e))?;
        
        for entry in udp6_table {
            listening_ports.push(entry.local_address.port());
        }
        
        listening_ports.sort_unstable();
        listening_ports.dedup();
        
        Ok(listening_ports)
    }
}