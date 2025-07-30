use std::time::Duration;
use reqwest::Client;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_USER_AGENT: &str = "Netzoot/1.0";

// Create HTTP client with reasonable defaults
pub fn create_http_client(timeout_ms: Option<u64>) -> Result<Client, String> {
    let timeout = timeout_ms
        .map(|ms| Duration::from_millis(ms))
        .unwrap_or(DEFAULT_TIMEOUT);
    
    Client::builder()
        .timeout(timeout)
        .user_agent(DEFAULT_USER_AGENT)
        .danger_accept_invalid_certs(false)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))
}

// Helper function for common service names
pub fn get_common_service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        143 => "IMAP",
        443 => "HTTPS",
        587 => "SMTP-Submission",
        993 => "IMAP-SSL",
        995 => "POP3-SSL",
        3306 => "MySQL",
        3389 => "RDP",
        5432 => "PostgreSQL",
        6379 => "Redis",
        27017 => "MongoDB",
        _ => "Unknown",
    }
}