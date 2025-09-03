#[cfg(target_family = "windows")]
pub struct WindowsSystemUtils;

#[cfg(target_family = "windows")]
impl WindowsSystemUtils {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_family = "windows")]
#[async_trait]
impl SystemUtils for WindowsSystemUtils {
    async fn get_mac_address_for_ip(&self, ip: IpAddr) -> Result<Option<MacAddress>> {
        use windows::Win32::NetworkManagement::IpHelper::{GetIpNetTable, MIB_IPNETTABLE};
        
        let ipv4_addr = match ip {
            IpAddr::V4(addr) => addr,
            IpAddr::V6(_) => return Ok(None), // IPv6 ARP not supported in this implementation
        };
        
        // First call to get required buffer size
        let mut size: u32 = 0;
        let result = unsafe {
            GetIpNetTable(None, &mut size, true)
        };
        
        if size == 0 {
            return Ok(None);
        }
        
        // Allocate buffer and get the actual table
        let mut buffer = vec![0u8; size as usize];
        let table_ptr = buffer.as_mut_ptr() as *mut MIB_IPNETTABLE;
        
        let result = unsafe {
            GetIpNetTable(Some(table_ptr), &mut size, true)
        };
        
        if result != 0 {
            return Err(anyhow!("GetIpNetTable failed with error code: {}", result));
        }
        
        // Parse the table
        let table = unsafe { &*table_ptr };
        let entries = unsafe {
            std::slice::from_raw_parts(
                table.table.as_ptr(),
                table.dwNumEntries as usize
            )
        };
        
        // Find matching IP
        for entry in entries {
            let entry_ip = Ipv4Addr::from(u32::from_be(entry.dwAddr));
            if entry_ip == ipv4_addr {
                // Extract MAC address bytes (only use first 6 bytes)
                let mac_bytes = [
                    entry.bPhysAddr[0],
                    entry.bPhysAddr[1], 
                    entry.bPhysAddr[2],
                    entry.bPhysAddr[3],
                    entry.bPhysAddr[4],
                    entry.bPhysAddr[5],
                ];
                
                return Ok(Some(MacAddress::new(mac_bytes)));
            }
        }
        
        Ok(None)
    }

    async fn scan_own_tcp_ports(&self) -> Result<Vec<u16>> {
        use windows::Win32::NetworkManagement::IpHelper::{GetTcpTable, MIB_TCPTABLE};
        
        // First call to get required buffer size
        let mut size: u32 = 0;
        let result = unsafe {
            GetTcpTable(None, &mut size, true)
        };
        
        if size == 0 {
            return Ok(Vec::new());
        }
        
        // Allocate buffer and get the actual table
        let mut buffer = vec![0u8; size as usize];
        let table_ptr = buffer.as_mut_ptr() as *mut MIB_TCPTABLE;
        
        let result = unsafe {
            GetTcpTable(Some(table_ptr), &mut size, true)
        };
        
        if result != 0 {
            return Err(anyhow!("GetTcpTable failed with error code: {}", result));
        }
        
        // Parse the table
        let table = unsafe { &*table_ptr };
        let entries = unsafe {
            std::slice::from_raw_parts(
                table.table.as_ptr(),
                table.dwNumEntries as usize
            )
        };
        
        let mut listening_ports = Vec::new();
        
        for entry in entries {
            // Only include listening sockets (state 2 = MIB_TCP_STATE_LISTEN)
            if entry.dwState == 2 {
                // Convert Windows port format (byte-swapped)
                let port = ((entry.dwLocalPort & 0xFF) << 8) | ((entry.dwLocalPort >> 8) & 0xFF);
                listening_ports.push(port as u16);
            }
        }
        
        listening_ports.sort_unstable();
        listening_ports.dedup();
        
        Ok(listening_ports)
    }

    async fn scan_own_udp_ports(&self) -> Result<Vec<u16>> {
        use windows::Win32::NetworkManagement::IpHelper::{GetUdpTable, MIB_UDPTABLE};
        
        // First call to get required buffer size
        let mut size: u32 = 0;
        let result = unsafe {
            GetUdpTable(None, &mut size, true)
        };
        
        if size == 0 {
            return Ok(Vec::new());
        }
        
        // Allocate buffer and get the actual table
        let mut buffer = vec![0u8; size as usize];
        let table_ptr = buffer.as_mut_ptr() as *mut MIB_UDPTABLE;
        
        let result = unsafe {
            GetUdpTable(Some(table_ptr), &mut size, true)
        };
        
        if result != 0 {
            return Err(anyhow!("GetUdpTable failed with error code: {}", result));
        }
        
        // Parse the table
        let table = unsafe { &*table_ptr };
        let entries = unsafe {
            std::slice::from_raw_parts(
                table.table.as_ptr(),
                table.dwNumEntries as usize
            )
        };
        
        let mut listening_ports = Vec::new();
        
        for entry in entries {
            // Convert Windows port format (byte-swapped)
            let port = ((entry.dwLocalPort & 0xFF) << 8) | ((entry.dwLocalPort >> 8) & 0xFF);
            listening_ports.push(port as u16);
        }
        
        listening_ports.sort_unstable();
        listening_ports.dedup();
        
        Ok(listening_ports)
    }
}