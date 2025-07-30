// src/lib/stores/checks.ts (Enhanced with category updates and detailed descriptions)
import type { CheckTypeConfig, CheckConfig } from '$lib/types';
import { 
  Wifi, Search, Shield, Router, Mail, Lock, 
  Server, Activity, Target, Globe, Zap 
} from 'lucide-svelte';

// Category icons mapping for UI display
export const CATEGORY_ICONS: Record<string, any> = {
  "Basic": Wifi,
  "DNS": Search,
  "VPN": Shield,
  "Local Network": Router,
  "Email": Mail,
  "Security": Lock,
  "Services": Server,
  "Performance": Activity,
  "Analysis": Target,
  "CDN": Globe
};

// Helper function to get default check configuration
export function getDefaultCheckConfig(checkType: string): CheckConfig {
  const checkTypeConfig = CHECK_TYPES[checkType];
  if (checkTypeConfig && checkTypeConfig.defaults) {
    return { ...checkTypeConfig.defaults };
  }
  return { timeout: 5000 };
}

// Check types available with improved categories and descriptions
export const CHECK_TYPES: Record<string, CheckTypeConfig> = {
  // ================== BASIC CONNECTIVITY ==================
  connectivityCheck: {
    name: 'Connectivity Check',
    description: 'Check HTTP/HTTPS connection to domain',
    details: 'Performs a full HTTP/HTTPS request to a domain name, checking both DNS resolution and web server connectivity. Validates that the target responds to standard web requests.',
    fields: ['target', 'port', 'protocol', 'timeout'],
    defaults: { target: '', port: 443, protocol: 'https', timeout: 5000 },
    category: "Basic"
  },
  directIpCheck: {
    name: 'Direct IP Connection',
    description: 'Check connection to IP address (no DNS)',
    details: 'Connects directly to an IP address, bypassing DNS resolution entirely. Useful for checking raw network connectivity or diagnosing DNS-related issues.',
    fields: ['target', 'port', 'timeout'],
    defaults: { target: '', port: 443, timeout: 5000 },
    category: "Basic"
  },
  serviceHealthCheck: {
    name: 'Service Health',
    description: 'Check web service returns expected status',
    details: 'Makes an HTTP request to a specific path and validates the response status code. Perfect for monitoring API endpoints, health checks, and service availability.',
    fields: ['target', 'port', 'path', 'expected_status', 'timeout'],
    defaults: { target: '', port: 80, path: '/', expected_status: 200, timeout: 5000 },
    category: "Basic"
  },
  responseTimeCheck: {
    name: 'Response Time',
    description: 'Measure connection latency vs threshold',
    details: 'Measures the time from connection initiation to first response, ensuring it meets performance requirements. Helps identify slow network paths or overloaded servers.',
    fields: ['target', 'port', 'timeout', 'max_response_time'],
    defaults: { target: '', port: 443, timeout: 5000, max_response_time: 1000 },
    category: "Basic"
  },
  pingCheck: {
    name: 'Ping',
    description: 'Check TCP connectivity with retry logic',
    details: 'Performs multiple TCP connection attempts to measure success rate and connection reliability. Unlike ICMP ping, this checks actual service connectivity.',
    fields: ['target', 'port', 'attempts', 'timeout'],
    defaults: { target: '', port: 443, attempts: 3, timeout: 5000 },
    category: "Basic"
  },
  wellknownIpCheck: {
    name: 'Internet Backbone Check',
    description: 'Check basic internet connectivity',
    details: 'Connects to well-known public DNS servers (Google 8.8.8.8, Cloudflare 1.1.1.1) to verify fundamental internet connectivity without relying on DNS resolution.',
    fields: ['timeout'],
    defaults: { timeout: 3000 },
    category: "Basic"
  },

  // ================== DNS RESOLUTION ==================
  dnsResolutionCheck: {
    name: 'DNS Resolution',
    description: 'Check domain name to IP resolution',
    details: 'Queries DNS servers to resolve domain names to IP addresses. Checks the fundamental DNS lookup process that underlies all internet communication.',
    fields: ['domain', 'timeout'],
    defaults: { domain: '', timeout: 5000 },
    category: "DNS"
  },
  dnsOverHttpsCheck: {
    name: 'DNS over HTTPS',
    description: 'Check secure DNS queries via HTTPS',
    details: 'Performs DNS lookups using DNS-over-HTTPS (DoH) for enhanced privacy and security. Checks connectivity to secure DNS providers like Cloudflare, Pi-hole, or custom DoH endpoints.',
    fields: ['target', 'domain', 'service_type', 'timeout'],
    defaults: { target: 'https://1.1.1.1/dns-query', domain: 'example.com', service_type: 'cloudflare', timeout: 5000 },
    category: "DNS"
  },

  // ================== VPN CONNECTIVITY ==================
  vpnConnectivityCheck: {
    name: 'VPN Connectivity',
    description: 'Check VPN connection is active',
    details: 'Verifies that VPN connection is properly established and routing traffic through the VPN tunnel. Detects VPN disconnections and routing issues.',
    fields: ['target', 'timeout'],
    defaults: { target: '', timeout: 10000 },
    category: "VPN"
  },
  vpnTunnelCheck: {
    name: 'VPN Tunnel Integrity',
    description: 'Check VPN tunnel stability',
    details: 'Performs comprehensive checks to ensure VPN tunnel maintains integrity under load, including data transmission verification and connection stability over time.',
    fields: ['target', 'timeout'],
    defaults: { target: '', timeout: 15000 },
    category: "VPN"
  },

  // ================== LOCAL NETWORK ==================
  localGatewayCheck: {
    name: 'Local Gateway Check',
    description: 'Check connection to network gateway',
    details: 'Checks connectivity to the local network gateway (router). Essential for diagnosing local network issues and verifying the first hop in network communication.',
    fields: ['timeout'],
    defaults: { timeout: 3000 },
    category: "Local Network"
  },
  dhcpDiscoveryCheck: {
    name: 'DHCP Discovery',
    description: 'Discover DHCP servers on network',
    details: 'Scans the local network segment to identify active DHCP servers. Useful for network troubleshooting and detecting rogue DHCP servers that could cause IP conflicts.',
    fields: ['interface', 'timeout'],
    defaults: { interface: 'auto', timeout: 10000 },
    category: "Local Network"
  },
  subnetScanCheck: {
    name: 'Subnet Scan',
    description: 'Scan local network for active devices',
    details: 'Performs a comprehensive scan of the local subnet to discover active devices and services. Maps the local network topology and identifies available resources.',
    fields: ['subnet', 'concurrent_scans', 'timeout'],
    defaults: { subnet: 'auto', concurrent_scans: 50, timeout: 30000 },
    category: "Local Network"
  },

  // ================== EMAIL SERVICES ==================
  smtpCheck: {
    name: 'SMTP Server Check',
    description: 'Test outbound email server connectivity',
    details: 'Tests connection to SMTP (Simple Mail Transfer Protocol) servers used for sending email. Verifies authentication, TLS encryption, and basic SMTP command responses.',
    fields: ['target', 'port', 'use_tls', 'timeout'],
    defaults: { target: '', port: 587, use_tls: true, timeout: 10000 },
    category: "Email"
  },
  imapCheck: {
    name: 'IMAP Server Check',
    description: 'Test inbound email server connectivity',
    details: 'Tests connection to IMAP (Internet Message Access Protocol) servers for retrieving email. Validates secure connections and server responsiveness for email clients.',
    fields: ['target', 'port', 'use_ssl', 'timeout'],
    defaults: { target: '', port: 993, use_ssl: true, timeout: 10000 },
    category: "Email"
  },
  pop3Check: {
    name: 'POP3 Server Check',
    description: 'Test POP3 email server connectivity',
    details: 'Tests connection to POP3 (Post Office Protocol 3) servers for downloading email. Verifies server availability and secure connection establishment.',
    fields: ['target', 'port', 'use_ssl', 'timeout'],
    defaults: { target: '', port: 995, use_ssl: true, timeout: 10000 },
    category: "Email"
  },

  // ================== SECURITY TESTING ==================
  sslCertificateCheck: {
    name: 'SSL Certificate Check',
    description: 'Validate SSL/TLS certificates',
    details: 'Examines SSL/TLS certificates for validity, expiration dates, and certificate chain integrity. Critical for ensuring secure connections and preventing certificate-related outages.',
    fields: ['target', 'port', 'min_days_until_expiry', 'check_chain', 'timeout'],
    defaults: { target: '', port: 443, min_days_until_expiry: 30, check_chain: true, timeout: 10000 },
    category: "Security"
  },
  portScanCheck: {
    name: 'Port Scan Check',
    description: 'Scan for open ports and services',
    details: 'Systematically tests specified ports to identify open services and potential security vulnerabilities. Essential for security audits and network reconnaissance.',
    fields: ['target', 'port_range', 'scan_type', 'timeout'],
    defaults: { target: '', port_range: '80,443,22,21,25,53,3389,5432,3306', scan_type: 'tcp', timeout: 2000 },
    category: "Security"
  },

  // ================== SERVICES ==================
  ftpCheck: {
    name: 'FTP Server Check',
    description: 'Check FTP/FTPS server connectivity',
    details: 'Checks connection to FTP (File Transfer Protocol) servers with support for both standard FTP and secure FTPS. Validates server availability and protocol handshake.',
    fields: ['target', 'port', 'use_ssl', 'passive_mode', 'timeout'],
    defaults: { target: '', port: 21, use_ssl: false, passive_mode: true, timeout: 10000 },
    category: "Services"
  },
  sshCheck: {
    name: 'SSH Server Check',
    description: 'Check SSH server connectivity',
    details: 'Checks connection to SSH (Secure Shell) servers for remote access. Validates server availability, protocol version, and secure connection establishment without authentication.',
    fields: ['target', 'port', 'check_banner', 'timeout'],
    defaults: { target: '', port: 22, check_banner: true, timeout: 10000 },
    category: "Services"
  },
  databaseCheck: {
    name: 'Database Server Check',
    description: 'Check database server connectivity',
    details: 'Checks connectivity to database servers including MySQL, PostgreSQL, MongoDB, and others. Validates that database services are accepting connections on their standard ports.',
    fields: ['target', 'port', 'db_type', 'timeout'],
    defaults: { target: '', port: 3306, db_type: 'mysql', timeout: 10000 },
    category: "Services"
  },
  ntpCheck: {
    name: 'NTP Time Server Check',
    description: 'Check network time synchronization',
    details: 'Checks connectivity to NTP (Network Time Protocol) servers and validates time synchronization accuracy. Critical for systems requiring precise time coordination.',
    fields: ['target', 'port', 'max_time_drift', 'timeout'],
    defaults: { target: 'pool.ntp.org', port: 123, max_time_drift: 1000, timeout: 5000 },
    category: "Services"
  },
  ldapCheck: {
    name: 'LDAP Server Check',
    description: 'Check directory service connectivity',
    details: 'Checks connection to LDAP (Lightweight Directory Access Protocol) servers used for directory services and authentication. Supports both standard LDAP and secure LDAPS.',
    fields: ['target', 'port', 'use_ssl', 'bind_dn', 'timeout'],
    defaults: { target: '', port: 389, use_ssl: false, bind_dn: '', timeout: 10000 },
    category: "Services"
  },
  sipCheck: {
    name: 'SIP Server Check',
    description: 'Check VoIP server connectivity',
    details: 'Checks connection to SIP (Session Initiation Protocol) servers used for VoIP communications. Validates server availability for voice and video calling services.',
    fields: ['target', 'port', 'transport', 'timeout'],
    defaults: { target: '', port: 5060, transport: 'udp', timeout: 5000 },
    category: "Services"
  },

  // ================== PERFORMANCE TESTING ==================
  bandwidthCheck: {
    name: 'Bandwidth Check',
    description: 'Measure network throughput',
    details: 'Performs download and upload tests to measure actual network bandwidth and throughput. Identifies network bottlenecks and validates connection speed against expectations.',
    fields: ['target', 'test_duration', 'test_type', 'timeout'],
    defaults: { target: 'https://speed.cloudflare.com/__down', test_duration: 10, test_type: 'download', timeout: 30000 },
    category: "Performance"
  },
  packetLossCheck: {
    name: 'Packet Loss Check',
    description: 'Measure packet loss percentage',
    details: 'Sends multiple connection attempts to measure packet loss rates over the network path. High packet loss indicates network congestion or hardware issues.',
    fields: ['target', 'port', 'packet_count', 'interval_ms', 'timeout'],
    defaults: { target: '', port: 443, packet_count: 20, interval_ms: 100, timeout: 1000 },
    category: "Performance"
  },
  jitterCheck: {
    name: 'Network Jitter Check',
    description: 'Measure connection time variance',
    details: 'Measures variance in connection times to assess network stability and consistency. High jitter indicates unstable network conditions that can affect real-time applications.',
    fields: ['target', 'port', 'sample_count', 'interval_ms', 'timeout'],
    defaults: { target: '', port: 443, sample_count: 10, interval_ms: 500, timeout: 5000 },
    category: "Performance"
  },

  // ================== NETWORK ANALYSIS ==================
  mtuDiscoveryCheck: {
    name: 'MTU Discovery',
    description: 'Find optimal packet size',
    details: 'Discovers the Maximum Transmission Unit (MTU) size along the network path by testing progressively larger packets. Optimizes network performance by avoiding packet fragmentation.',
    fields: ['target', 'start_size', 'max_size', 'timeout'],
    defaults: { target: '', start_size: 1500, max_size: 9000, timeout: 10000 },
    category: "Analysis"
  },
  tracerouteCheck: {
    name: 'Network Path Trace',
    description: 'Trace route to destination',
    details: 'Maps the complete network path from source to destination, showing each router hop along the way. Essential for diagnosing routing issues and network topology analysis.',
    fields: ['target', 'max_hops', 'timeout_per_hop', 'resolve_hostnames'],
    defaults: { target: '', max_hops: 30, timeout_per_hop: 5000, resolve_hostnames: true },
    category: "Analysis"
  },

  // ================== CDN PERFORMANCE ==================
  cdnCheck: {
    name: 'CDN Performance Check',
    description: 'Check CDN edge server performance',
    details: 'Evaluates Content Delivery Network performance including edge server response times, geographic routing efficiency, and content delivery optimization.',
    fields: ['target', 'expected_region', 'check_headers', 'timeout'],
    defaults: { target: '', expected_region: 'auto', check_headers: true, timeout: 10000 },
    category: "CDN"
  }
};

// Get all available check types
export function getAllCheckTypes(): string[] {
  return Object.keys(CHECK_TYPES);
}

// Get check types by category
export function getCheckTypesByCategory(category: string): string[] {
  return Object.keys(CHECK_TYPES).filter(key => CHECK_TYPES[key].category === category);
}

// Get all available categories
export function getAllCategories(): string[] {
  const categories = new Set(Object.values(CHECK_TYPES).map(config => config.category));
  return Array.from(categories).sort();
}

// Validate check configuration based on check type
export function validateCheckConfig(checkType: string, config: CheckConfig): string[] {
  const checkTypeConfig = CHECK_TYPES[checkType];
  if (!checkTypeConfig) {
    return [`Unknown check type: ${checkType}`];
  }

  const errors: string[] = [];
  
  // Check required fields
  checkTypeConfig.fields.forEach(field => {
    const value = config[field as keyof CheckConfig];
    
    // Check for required target/domain fields
    if ((field === 'target' || field === 'domain') && (!value || (typeof value === 'string' && !value.trim()))) {
      errors.push(`${field} is required for ${checkTypeConfig.name}`);
    }
    
    // Validate port numbers
    if (field === 'port' && value !== undefined) {
      const port = typeof value === 'number' ? value : parseInt(value as string);
      if (isNaN(port) || port < 1 || port > 65535) {
        errors.push(`Port must be a valid number between 1 and 65535`);
      }
    }
    
    // Validate timeout values
    if (field === 'timeout' && value !== undefined) {
      const timeout = typeof value === 'number' ? value : parseInt(value as string);
      if (isNaN(timeout) || timeout < 100 || timeout > 300000) {
        errors.push(`Timeout must be between 100ms and 5 minutes`);
      }
    }
    
    // Validate other numeric fields
    if (['attempts', 'packet_count', 'sample_count', 'max_hops'].includes(field) && value !== undefined) {
      const numValue = typeof value === 'number' ? value : parseInt(value as string);
      if (isNaN(numValue) || numValue < 1) {
        errors.push(`${field} must be a positive number`);
      }
    }
  });
  
  return errors;
}