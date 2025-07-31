use serde_json::{json, Value};
use std::net::{ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::components::tests::checks::CheckConfig;

pub async fn ftp_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("FTP server target is required")?;
    let port = config.port.unwrap_or(21);
    let use_ssl = config.use_ssl.unwrap_or(false);
    let passive_mode = config.passive_mode.unwrap_or(true);
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    println!("Testing FTP connectivity to {}:{} (SSL: {}, Passive: {})", target, port, use_ssl, passive_mode);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve FTP server {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for FTP server {}", target))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(mut stream)) => {
            let duration = start.elapsed();
            
            // Read FTP welcome message
            let mut buffer = [0; 1024];
            let welcome = match timeout(Duration::from_millis(5000), stream.read(&mut buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&buffer[..n]).trim().to_string(),
                _ => "No welcome message".to_string(),
            };
            
            // Send FEAT command to check features
            let feat_cmd = "FEAT\r\n";
            let _ = stream.write_all(feat_cmd.as_bytes()).await;
            
            let mut response_buffer = [0; 2048];
            let feat_response = match timeout(Duration::from_millis(5000), stream.read(&mut response_buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&response_buffer[..n]).trim().to_string(),
                _ => "No FEAT response".to_string(),
            };
            
            let supports_mlsd = feat_response.contains("MLSD");
            let supports_utf8 = feat_response.contains("UTF8");
            let supports_tls = feat_response.contains("TLS") || feat_response.contains("SSL");
            
            Ok(json!({
                "ftp_server": format!("{}:{}", target, port),
                "response_time_ms": duration.as_millis(),
                "welcome_message": welcome,
                "supports_mlsd": supports_mlsd,
                "supports_utf8": supports_utf8,
                "supports_tls": supports_tls,
                "passive_mode": passive_mode,
                "ssl_enabled": use_ssl,
                "features": feat_response.lines().collect::<Vec<_>>(),
                "status": "reachable"
            }))
        },
        Ok(Err(e)) => Err(format!("FTP connection failed: {}", e)),
        Err(_) => Err(format!("FTP connection timeout after {}ms", timeout_ms)),
    }
}

pub async fn ssh_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("SSH server target is required")?;
    let port = config.port.unwrap_or(22);
    let check_banner = config.check_banner.unwrap_or(true);
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    println!("Testing SSH connectivity to {}:{}", target, port);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve SSH server {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for SSH server {}", target))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(mut stream)) => {
            let duration = start.elapsed();
            
            let mut banner = String::new();
            let mut version = String::new();
            let mut protocol_version = String::new();
            
            if check_banner {
                // Read SSH banner
                let mut buffer = [0; 1024];
                match timeout(Duration::from_millis(5000), stream.read(&mut buffer)).await {
                    Ok(Ok(n)) => {
                        banner = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                        
                        // Parse SSH banner (format: SSH-protocolversion-softwareversion comments)
                        if banner.starts_with("SSH-") {
                            let parts: Vec<&str> = banner.splitn(3, '-').collect();
                            if parts.len() >= 3 {
                                protocol_version = format!("SSH-{}", parts[1]);
                                version = parts[2].split_whitespace().next().unwrap_or("").to_string();
                            }
                        }
                    },
                    _ => banner = "No banner received".to_string(),
                }
            }
            
            Ok(json!({
                "ssh_server": format!("{}:{}", target, port),
                "response_time_ms": duration.as_millis(),
                "banner": banner,
                "protocol_version": protocol_version,
                "server_version": version,
                "banner_checked": check_banner,
                "status": "reachable"
            }))
        },
        Ok(Err(e)) => Err(format!("SSH connection failed: {}", e)),
        Err(_) => Err(format!("SSH connection timeout after {}ms", timeout_ms)),
    }
}

pub async fn database_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("Database server target is required")?;
    let port = config.port.unwrap_or(3306);
    let db_type = config.db_type.as_deref().unwrap_or("mysql");
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    // Default ports for different database types
    let default_port = match db_type {
        "mysql" => 3306,
        "postgresql" | "postgres" => 5432,
        "mongodb" | "mongo" => 27017,
        "redis" => 6379,
        "mssql" | "sqlserver" => 1433,
        "oracle" => 1521,
        "cassandra" => 9042,
        "elasticsearch" => 9200,
        _ => port,
    };
    
    let actual_port = if port == 3306 { default_port } else { port };
    
    println!("Testing {} database connectivity to {}:{}", db_type, target, actual_port);
    
    let socket_addr = format!("{}:{}", target, actual_port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve database server {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for database server {}", target))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(mut stream)) => {
            let duration = start.elapsed();
            
            // Try to read initial handshake/greeting for some database types
            let mut greeting = String::new();
            let mut protocol_info = HashMap::new();
            
            match db_type {
                "mysql" => {
                    let mut buffer = [0; 1024];
                    if let Ok(Ok(n)) = timeout(Duration::from_millis(3000), stream.read(&mut buffer)).await {
                        let data = &buffer[..n];
                        if n > 5 {
                            let protocol_version = data[0];
                            greeting = format!("MySQL Protocol Version {}", protocol_version);
                            protocol_info.insert("protocol_version", json!(protocol_version));
                        }
                    }
                },
                "redis" => {
                    // Send PING command
                    let _ = stream.write_all(b"PING\r\n").await;
                    let mut buffer = [0; 1024];
                    if let Ok(Ok(n)) = timeout(Duration::from_millis(3000), stream.read(&mut buffer)).await {
                        greeting = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                    }
                },
                "postgresql" => {
                    greeting = "PostgreSQL connection established".to_string();
                },
                _ => {
                    greeting = format!("{} connection established", db_type);
                }
            }
            
            Ok(json!({
                "database_server": format!("{}:{}", target, actual_port),
                "database_type": db_type,
                "response_time_ms": duration.as_millis(),
                "greeting": greeting,
                "protocol_info": protocol_info,
                "default_port": default_port,
                "status": "reachable"
            }))
        },
        Ok(Err(e)) => Err(format!("{} database connection failed: {}", db_type, e)),
        Err(_) => Err(format!("{} database connection timeout after {}ms", db_type, timeout_ms)),
    }
}

pub async fn ntp_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("NTP server is required")?;
    let port = config.port.unwrap_or(123);
    let max_time_drift = config.max_time_drift.unwrap_or(1000); // milliseconds
    let timeout_ms = config.timeout.unwrap_or(5000);
    
    println!("Testing NTP server connectivity to {}:{}", target, port);
    
    // This is a simplified NTP test - a full implementation would use the NTP protocol
    let socket_addr = format!("{}:{}", target, port);
    
    match socket_addr.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                let start = Instant::now();
                
                // Try UDP connection (NTP uses UDP)
                match tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                    Ok(socket) => {
                        match timeout(Duration::from_millis(timeout_ms), socket.connect(addr)).await {
                            Ok(Ok(_)) => {
                                let duration = start.elapsed();
                                
                                // Get current system time for drift calculation
                                let system_time = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis();
                                
                                Ok(json!({
                                    "ntp_server": format!("{}:{}", target, port),
                                    "response_time_ms": duration.as_millis(),
                                    "system_time_ms": system_time,
                                    "max_drift_threshold_ms": max_time_drift,
                                    "connection_status": "reachable",
                                    "note": "Simplified NTP test - full protocol implementation needed for time sync",
                                    "status": "connected"
                                }))
                            },
                            _ => Err(format!("NTP server {}:{} is unreachable", target, port)),
                        }
                    },
                    Err(e) => Err(format!("UDP socket error: {}", e)),
                }
            } else {
                Err("No addresses found for NTP server".to_string())
            }
        },
        Err(e) => Err(format!("Failed to resolve NTP server {}: {}", target, e)),
    }
}

pub async fn ldap_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("LDAP server target is required")?;
    let port = config.port.unwrap_or(389);
    let use_ssl = config.use_ssl.unwrap_or(false);
    let bind_dn = config.bind_dn.as_deref().unwrap_or("");
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    let actual_port = if use_ssl && port == 389 { 636 } else { port };
    
    println!("Testing LDAP connectivity to {}:{} (SSL: {})", target, actual_port, use_ssl);
    
    let socket_addr = format!("{}:{}", target, actual_port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve LDAP server {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for LDAP server {}", target))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(_stream)) => {
            let duration = start.elapsed();
            
            // LDAP uses a binary protocol, so we can't easily test without a full LDAP client
            // This tests basic TCP connectivity
            
            Ok(json!({
                "ldap_server": format!("{}:{}", target, actual_port),
                "response_time_ms": duration.as_millis(),
                "ssl_enabled": use_ssl,
                "bind_dn": bind_dn,
                "connection_status": "tcp_connected",
                "note": "Basic TCP connectivity test - full LDAP protocol implementation needed",
                "status": "reachable"
            }))
        },
        Ok(Err(e)) => Err(format!("LDAP connection failed: {}", e)),
        Err(_) => Err(format!("LDAP connection timeout after {}ms", timeout_ms)),
    }
}

pub async fn sip_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("SIP server target is required")?;
    let port = config.port.unwrap_or(5060);
    let transport = config.transport.as_deref().unwrap_or("udp");
    let timeout_ms = config.timeout.unwrap_or(5000);
    
    println!("Testing SIP connectivity to {}:{} (transport: {})", target, port, transport);
    
    let socket_addr = format!("{}:{}", target, port);
    
    match socket_addr.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                let start = Instant::now();
                
                match transport {
                    "udp" => {
                        match tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                            Ok(socket) => {
                                match timeout(Duration::from_millis(timeout_ms), socket.connect(addr)).await {
                                    Ok(Ok(_)) => {
                                        let duration = start.elapsed();
                                        
                                        // Send a basic SIP OPTIONS request
                                        let sip_options = format!(
                                            "OPTIONS sip:{}:{} SIP/2.0\r\n\
                                             Via: SIP/2.0/UDP netzoot.test:5060\r\n\
                                             From: <sip:test@netzoot.test>\r\n\
                                             To: <sip:{}:{}>\r\n\
                                             Call-ID: test-{}\r\n\
                                             CSeq: 1 OPTIONS\r\n\
                                             Contact: <sip:test@netzoot.test:5060>\r\n\
                                             Content-Length: 0\r\n\r\n",
                                            target, port, target, port, 
                                            std::time::SystemTime::now()
                                                .duration_since(std::time::UNIX_EPOCH)
                                                .unwrap()
                                                .as_secs()
                                        );
                                        
                                        let _ = socket.send(sip_options.as_bytes()).await;
                                        
                                        // Try to read response
                                        let mut buffer = [0; 1024];
                                        let response = match timeout(Duration::from_millis(2000), socket.recv(&mut buffer)).await {
                                            Ok(Ok(n)) => String::from_utf8_lossy(&buffer[..n]).to_string(),
                                            _ => "No response".to_string(),
                                        };
                                        
                                        Ok(json!({
                                            "sip_server": format!("{}:{}", target, port),
                                            "transport": transport,
                                            "response_time_ms": duration.as_millis(),
                                            "sip_response": response,
                                            "options_sent": true,
                                            "status": "reachable"
                                        }))
                                    },
                                    _ => Err(format!("SIP UDP connection failed to {}:{}", target, port)),
                                }
                            },
                            Err(e) => Err(format!("UDP socket error: {}", e)),
                        }
                    },
                    "tcp" => {
                        match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await {
                            Ok(Ok(_)) => {
                                let duration = start.elapsed();
                                
                                Ok(json!({
                                    "sip_server": format!("{}:{}", target, port),
                                    "transport": transport,
                                    "response_time_ms": duration.as_millis(),
                                    "tcp_connection": "successful",
                                    "status": "reachable"
                                }))
                            },
                            _ => Err(format!("SIP TCP connection failed to {}:{}", target, port)),
                        }
                    },
                    _ => Err("Invalid transport. Use 'udp' or 'tcp'".to_string()),
                }
            } else {
                Err("No addresses found for SIP server".to_string())
            }
        },
        Err(e) => Err(format!("Failed to resolve SIP server {}: {}", target, e)),
    }
}