use serde_json::{json, Value};
use std::net::{ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::components::tests::checks::CheckConfig;

// SMTP server connectivity test
pub async fn smtp_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("SMTP server target is required")?;
    let port = config.port.unwrap_or(587);
    let use_tls = config.use_tls.unwrap_or(true);
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    println!("Testing SMTP connectivity to {}:{} (TLS: {})", target, port, use_tls);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve SMTP server {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for SMTP server {}", target))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(mut stream)) => {
            let duration = start.elapsed();
            
            // Read SMTP banner
            let mut buffer = [0; 1024];
            let banner = match timeout(Duration::from_millis(5000), stream.read(&mut buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&buffer[..n]).trim().to_string(),
                _ => "No banner received".to_string(),
            };
            
            // Send EHLO command
            let ehlo_cmd = "EHLO netzoot.test\r\n";
            let _ = stream.write_all(ehlo_cmd.as_bytes()).await;
            
            let mut response_buffer = [0; 2048];
            let ehlo_response = match timeout(Duration::from_millis(5000), stream.read(&mut response_buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&response_buffer[..n]).trim().to_string(),
                _ => "No EHLO response".to_string(),
            };
            
            let supports_starttls = ehlo_response.contains("STARTTLS");
            let supports_auth = ehlo_response.contains("AUTH");
            
            Ok(json!({
                "smtp_server": format!("{}:{}", target, port),
                "response_time_ms": duration.as_millis(),
                "banner": banner,
                "supports_starttls": supports_starttls,
                "supports_auth": supports_auth,
                "ehlo_response": ehlo_response.lines().collect::<Vec<_>>(),
                "tls_required": use_tls,
                "status": "reachable"
            }))
        },
        Ok(Err(e)) => Err(format!("SMTP connection failed: {}", e)),
        Err(_) => Err(format!("SMTP connection timeout after {}ms", timeout_ms)),
    }
}

// IMAP server connectivity test
pub async fn imap_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("IMAP server target is required")?;
    let port = config.port.unwrap_or(993);
    let use_ssl = config.use_ssl.unwrap_or(true);
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    println!("Testing IMAP connectivity to {}:{} (SSL: {})", target, port, use_ssl);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve IMAP server {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for IMAP server {}", target))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(mut stream)) => {
            let duration = start.elapsed();
            
            // Read IMAP greeting
            let mut buffer = [0; 1024];
            let greeting = match timeout(Duration::from_millis(5000), stream.read(&mut buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&buffer[..n]).trim().to_string(),
                _ => "No greeting received".to_string(),
            };
            
            // Send CAPABILITY command
            let capability_cmd = "A001 CAPABILITY\r\n";
            let _ = stream.write_all(capability_cmd.as_bytes()).await;
            
            let mut response_buffer = [0; 2048];
            let capability_response = match timeout(Duration::from_millis(5000), stream.read(&mut response_buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&response_buffer[..n]).trim().to_string(),
                _ => "No CAPABILITY response".to_string(),
            };
            
            let supports_starttls = capability_response.contains("STARTTLS");
            let supports_idle = capability_response.contains("IDLE");
            let imap_version = if capability_response.contains("IMAP4rev1") { "IMAP4rev1" } else { "Unknown" };
            
            Ok(json!({
                "imap_server": format!("{}:{}", target, port),
                "response_time_ms": duration.as_millis(),
                "greeting": greeting,
                "imap_version": imap_version,
                "supports_starttls": supports_starttls,
                "supports_idle": supports_idle,
                "capabilities": capability_response.lines().collect::<Vec<_>>(),
                "ssl_enabled": use_ssl,
                "status": "reachable"
            }))
        },
        Ok(Err(e)) => Err(format!("IMAP connection failed: {}", e)),
        Err(_) => Err(format!("IMAP connection timeout after {}ms", timeout_ms)),
    }
}

// POP3 server connectivity test
pub async fn pop3_check(config: &CheckConfig) -> Result<Value, String> {
    let target = config.target.as_ref().ok_or("POP3 server target is required")?;
    let port = config.port.unwrap_or(995);
    let use_ssl = config.use_ssl.unwrap_or(true);
    let timeout_ms = config.timeout.unwrap_or(10000);
    
    println!("Testing POP3 connectivity to {}:{} (SSL: {})", target, port, use_ssl);
    
    let socket_addr = format!("{}:{}", target, port);
    let addr = socket_addr.to_socket_addrs()
        .map_err(|e| format!("Failed to resolve POP3 server {}: {}", target, e))?
        .next()
        .ok_or_else(|| format!("No addresses found for POP3 server {}", target))?;
    
    let start = Instant::now();
    let connect_future = TcpStream::connect(&addr);
    
    match timeout(Duration::from_millis(timeout_ms), connect_future).await {
        Ok(Ok(mut stream)) => {
            let duration = start.elapsed();
            
            // Read POP3 greeting
            let mut buffer = [0; 1024];
            let greeting = match timeout(Duration::from_millis(5000), stream.read(&mut buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&buffer[..n]).trim().to_string(),
                _ => "No greeting received".to_string(),
            };
            
            // Send CAPA command to check capabilities
            let capa_cmd = "CAPA\r\n";
            let _ = stream.write_all(capa_cmd.as_bytes()).await;
            
            let mut response_buffer = [0; 2048];
            let capa_response = match timeout(Duration::from_millis(5000), stream.read(&mut response_buffer)).await {
                Ok(Ok(n)) => String::from_utf8_lossy(&response_buffer[..n]).trim().to_string(),
                _ => "No CAPA response".to_string(),
            };
            
            let supports_stls = capa_response.contains("STLS");
            let supports_uidl = capa_response.contains("UIDL");
            let supports_top = capa_response.contains("TOP");
            
            Ok(json!({
                "pop3_server": format!("{}:{}", target, port),
                "response_time_ms": duration.as_millis(),
                "greeting": greeting,
                "supports_stls": supports_stls,
                "supports_uidl": supports_uidl,
                "supports_top": supports_top,
                "capabilities": capa_response.lines().collect::<Vec<_>>(),
                "ssl_enabled": use_ssl,
                "status": "reachable"
            }))
        },
        Ok(Err(e)) => Err(format!("POP3 connection failed: {}", e)),
        Err(_) => Err(format!("POP3 connection timeout after {}ms", timeout_ms)),
    }
}