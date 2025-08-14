export type TestType = 'Connectivity' |
  'DirectIp' |
  'Ping' |
  'WellknownIp' |
  'DnsResolution' |
  'DnsOverHttps' |
  'VpnConnectivity' |
  'VpnTunnel' |
  'ServiceHealth' |
  'DaemonCommand' |
  'SshScript';

export interface ConnectivityConfig {
  timeout?: number;
  expected_result: string;
  target: string;
  port?: number;
  protocol?: string;
}

export interface DirectIpConfig {
  timeout?: number;
  expected_result: string;
  target: string;
  port: number;
}

export interface PingConfig {
  timeout?: number;
  expected_result: string;
  target: string;
  port?: number;
  attempts?: number;
}

export interface WellknownIpConfig {
  timeout?: number;
  expected_result: string;
}

export interface DnsResolutionConfig {
  timeout?: number;
  expected_result: string;
  domain: string;
}

export interface DnsOverHttpsConfig {
  timeout?: number;
  expected_result: string;
  target: string;
  domain: string;
  service_type?: string;
}

export interface VpnConnectivityConfig {
  timeout?: number;
  expected_result: string;
  target: string;
  port?: number;
}

export interface VpnTunnelConfig {
  timeout?: number;
  expected_result: string;
  expected_subnet: string;
}

export interface ServiceHealthConfig {
  timeout?: number;
  expected_result: string;
  target: string;
  port?: number;
  path?: string;
  expected_status?: number;
}

export interface DaemonCommandConfig {
  timeout?: number;
  expected_result: string;
  command: string;
  requires_confirmation?: boolean;
  rollback_command?: string;
  expected_output?: string;
}

export interface SshScriptConfig {
  timeout?: number;
  expected_result: string;
  command: string;
  ssh_target: string;
  requires_confirmation?: boolean;
  rollback_command?: string;
  expected_output?: string;
}

export type TestConfiguration = { Connectivity: ConnectivityConfig; } |
{ DirectIp: DirectIpConfig; } |
{ Ping: PingConfig; } |
{ WellknownIp: WellknownIpConfig; } |
{ DnsResolution: DnsResolutionConfig; } |
{ DnsOverHttps: DnsOverHttpsConfig; } |
{ VpnConnectivity: VpnConnectivityConfig; } |
{ VpnTunnel: VpnTunnelConfig; } |
{ ServiceHealth: ServiceHealthConfig; } |
{ DaemonCommand: DaemonCommandConfig; } |
{ SshScript: SshScriptConfig; };
export interface TestResult {
  test_type: TestType;
  success: boolean;
  message: string;
  duration_ms: number;
  executed_at: string;
  details?: any;
}

export interface TestDescription {
  name: string;
  shortDescription: string;
  detailedDescription: string;
}

export const TEST_DESCRIPTIONS: Record<TestType, TestDescription> = {
  Connectivity: {
    name: "Connectivity Test",
    shortDescription: "Tests basic TCP connectivity to a target host and port.",
    detailedDescription: "Establishes a TCP connection to verify that the target host is reachable and accepting connections on the specified port. This is the most fundamental network test for checking if a service is accessible."
  },

  DirectIp: {
    name: "Direct IP Test",
    shortDescription: "Tests connectivity directly to an IP address, bypassing DNS resolution.",
    detailedDescription: "Connects directly to an IP address without using DNS, useful for testing when you suspect DNS issues or want to verify connectivity to a specific server regardless of domain name resolution."
  },

  Ping: {
    name: "Ping Test",
    shortDescription: "Tests basic network reachability using ICMP ping packets.",
    detailedDescription: "Sends ICMP echo requests to test basic network connectivity and measure round-trip time. This is the most basic test to verify if a host is reachable over the network."
  },

  WellknownIp: {
    name: "Well-known IP Test",
    shortDescription: "Tests connectivity to reliable internet services like Google DNS and Cloudflare.",
    detailedDescription: "Connects to well-known, highly available internet services (8.8.8.8, 1.1.1.1, etc.) to verify that your internet connection and DNS infrastructure are working properly."
  },

  DnsResolution: {
    name: "DNS Resolution Test",
    shortDescription: "Tests the ability to resolve domain names to IP addresses.",
    detailedDescription: "Attempts to resolve a domain name to its IP address using the system's configured DNS servers. This verifies that DNS resolution is working properly for the testing device."
  },

  DnsOverHttps: {
    name: "DNS over HTTPS Test",
    shortDescription: "Tests DNS resolution using encrypted DNS over HTTPS (DoH) services.",
    detailedDescription: "Performs DNS queries using DNS over HTTPS, which encrypts DNS requests and sends them over HTTPS connections. This tests modern secure DNS resolution capabilities."
  },

  VpnConnectivity: {
    name: "VPN Connectivity Test",
    shortDescription: "Tests connectivity to a VPN server endpoint.",
    detailedDescription: "Verifies that a VPN server is reachable and accepting connections on the configured port. This tests the basic network path to the VPN service without establishing a full VPN connection."
  },

  VpnTunnel: {
    name: "VPN Tunnel Test",
    shortDescription: "Tests if a VPN tunnel is active and can reach the expected subnet.",
    detailedDescription: "Verifies that a VPN connection is established and working by testing connectivity to the VPN's internal network subnet. This confirms the VPN tunnel is fully functional."
  },

  ServiceHealth: {
    name: "Service Health Test",
    shortDescription: "Tests HTTP/HTTPS service health and response codes.",
    detailedDescription: "Makes HTTP requests to a web service and checks the response status code to verify the service is running correctly. This goes beyond basic connectivity to test actual service functionality."
  },

  DaemonCommand: {
    name: "Daemon Command Test",
    shortDescription: "Executes system commands via NetFrog daemon for remote system management.",
    detailedDescription: "Runs system commands on remote devices through the NetFrog daemon. This enables remote system administration, configuration checks, and automated maintenance tasks (Phase 5 feature)."
  },

  SshScript: {
    name: "SSH Script Test",
    shortDescription: "Executes commands via SSH connection for remote system access.",
    detailedDescription: "Connects to a remote system via SSH and executes commands, providing a fallback method for remote administration when the NetFrog daemon is not available (Phase 5 feature)."
  }
};

// Utility functions
export function getTestDescription(testType: TestType): TestDescription {
  return TEST_DESCRIPTIONS[testType];
}

export function getShortDescription(testType: TestType): string {
  return TEST_DESCRIPTIONS[testType].shortDescription;
}

export function getDetailedDescription(testType: TestType): string {
  return TEST_DESCRIPTIONS[testType].detailedDescription;
}

export function getTestTypeDisplayName(testType: TestType): string {
  switch (testType) {
    case 'Connectivity': return 'Connectivity';
    case 'DirectIp': return 'Direct IP';
    case 'Ping': return 'Ping';
    case 'WellknownIp': return 'Well-known IP';
    case 'DnsResolution': return 'DNS Resolution';
    case 'DnsOverHttps': return 'DNS over HTTPS';
    case 'VpnConnectivity': return 'VPN Connectivity';
    case 'VpnTunnel': return 'VPN Tunnel';
    case 'ServiceHealth': return 'Service Health';
    case 'DaemonCommand': return 'Daemon Command';
    case 'SshScript': return 'SSH Script';
    default: return testType;
  }
}

