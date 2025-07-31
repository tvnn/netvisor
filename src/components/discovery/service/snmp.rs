// src/discovery/snmp.rs
use snmp2::{SyncSession, Value as SnmpValue};
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::components::discovery::types::DiscoveryConfig;

const SYSTEM_DESCR: &str = "1.3.6.1.2.1.1.1.0";
const SYSTEM_NAME: &str = "1.3.6.1.2.1.1.5.0";
const SYSTEM_CONTACT: &str = "1.3.6.1.2.1.1.4.0";
const SYSTEM_LOCATION: &str = "1.3.6.1.2.1.1.6.0";

fn create_snmp_session(target: &str, community: &str, timeout_ms: u64) -> Result<SyncSession, String> {
    let session = SyncSession::new(
        target, 
        community.as_bytes(), 
        Some(Duration::from_millis(timeout_ms)), 
        0
    ).map_err(|e| format!("Failed to create SNMP session: {}", e))?;
    
    Ok(session)
}

pub async fn test_snmp_connectivity(ip: &str, community: &str, timeout_ms: u64) -> Result<bool, Box<dyn std::error::Error>> {
    // Test SNMP connectivity by querying system description
    match query_snmp_get(ip, community, SYSTEM_DESCR, timeout_ms).await {
        Ok(_) => Ok(true),
        Err(_) => {
            // Fallback: try to connect to SNMP port
            let addr = format!("{}:161", ip);
            match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
                Ok(Ok(_)) => Ok(true),
                _ => Ok(false)
            }
        }
    }
}

async fn query_snmp_get(target: &str, community: &str, oid: &str, timeout_ms: u64) -> Result<String, String> {
    let session = create_snmp_session(target, community, timeout_ms)?;
    
    match session.get(oid.as_bytes()) {
        Ok(response) => {
            if let Some((_, value)) = response.first() {
                let value_str = snmp_value_to_string(value);
                Ok(value_str)
            } else {
                Err("No response received".to_string())
            }
        },
        Err(e) => Err(format!("SNMP get failed: {}", e))
    }
}

pub async fn query_snmp_system_info(ip: &str, community: &str, timeout_ms: u64) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut info = HashMap::new();
    
    // Query system name (hostname)
    if let Ok(hostname) = query_snmp_get(ip, community, SYSTEM_NAME, timeout_ms).await {
        if !hostname.is_empty() && hostname != "Unknown" {
            info.insert("hostname".to_string(), hostname);
        }
    }
    
    // Query system description (often contains vendor info)
    if let Ok(description) = query_snmp_get(ip, community, SYSTEM_DESCR, timeout_ms).await {
        if !description.is_empty() && description != "Unknown" {
            info.insert("description".to_string(), description);
            
            // Try to extract vendor from description
            let vendor = extract_vendor_from_description(&description);
            if !vendor.is_empty() {
                info.insert("vendor".to_string(), vendor);
            }
        }
    }
    
    // Query system contact
    if let Ok(contact) = query_snmp_get(ip, community, SYSTEM_CONTACT, timeout_ms).await {
        if !contact.is_empty() && contact != "Unknown" {
            info.insert("contact".to_string(), contact);
        }
    }
    
    // Query system location
    if let Ok(location) = query_snmp_get(ip, community, SYSTEM_LOCATION, timeout_ms).await {
        if !location.is_empty() && location != "Unknown" {
            info.insert("location".to_string(), location);
        }
    }
    
    Ok(info)
}

pub async fn snmp_discovery(
    hosts: &[String], 
    config: &DiscoveryConfig,
    stop_signal: Arc<tokio::sync::RwLock<bool>>
) -> HashMap<String, HashMap<String, String>> {
    let mut results = HashMap::new();
    
    for host in hosts {
        if *stop_signal.read().await {
            break;
        }

        for community in &config.snmp_communities {
            // Test SNMP connectivity
            if let Ok(true) = test_snmp_connectivity(host, community, config.timeout_ms).await {
                // Query system information
                if let Ok(sys_info) = query_snmp_system_info(host, community, config.timeout_ms).await {
                    if !sys_info.is_empty() {
                        results.insert(host.clone(), sys_info);
                        break; // Found working community, no need to try others
                    }
                }
            }
        }
    }
    
    results
}

fn snmp_value_to_string(value: &SnmpValue) -> String {
    match value {
        SnmpValue::Integer(i) => i.to_string(),
        SnmpValue::OctetString(s) => String::from_utf8_lossy(&s).to_string(),
        SnmpValue::ObjectIdentifier(o) => String::from_utf8_lossy(&o).to_string(),
        SnmpValue::IpAddress(ip) => format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]),
        SnmpValue::Counter32(c) => c.to_string(),
        SnmpValue::Gauge32(g) => g.to_string(),
        SnmpValue::TimeTicks(t) => t.to_string(),
        SnmpValue::Counter64(c) => c.to_string(),
        _ => "Unknown".to_string(),
    }
}

fn extract_vendor_from_description(description: &str) -> String {
    let desc_lower = description.to_lowercase();
    
    // Common vendor patterns in SNMP system descriptions
    let vendors = vec![
        ("cisco", "Cisco"),
        ("juniper", "Juniper"),
        ("hp ", "HP"),
        ("hewlett", "HP"),
        ("dell", "Dell"),
        ("netgear", "Netgear"),
        ("d-link", "D-Link"),
        ("tp-link", "TP-Link"),
        ("ubiquiti", "Ubiquiti"),
        ("mikrotik", "MikroTik"),
        ("fortinet", "Fortinet"),
        ("palo alto", "Palo Alto"),
        ("aruba", "Aruba"),
        ("huawei", "Huawei"),
        ("zyxel", "ZyXEL"),
        ("linux", "Linux"),
        ("windows", "Microsoft"),
        ("freebsd", "FreeBSD"),
        ("openbsd", "OpenBSD"),
    ];
    
    for (pattern, vendor) in vendors {
        if desc_lower.contains(pattern) {
            return vendor.to_string();
        }
    }
    
    "Unknown".to_string()
}