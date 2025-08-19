import type { TEST_TYPE_CONFIG } from "$lib/config/tests/types";

export interface TestResult {
  success: boolean;
  message: string;
  duration_ms: number;
  executed_at: string;
  criticality?: string;
  details?: any;
}

// Test configuration interfaces matching backend structs
export interface ConnectivityConfig {
  timeout_ms?: number;
}

export interface DirectIpConfig {
  timeout_ms?: number;
}

export interface PingConfig {
  packet_count?: number;
  timeout_ms?: number;
}

export interface ServiceHealthConfig {
  expected_status_code: number;
  timeout_ms?: number;
}

export interface DnsResolutionConfig {
  domain: string;
  expected_ip: string; // IP address as string
  timeout_ms?: number;
}

export interface DnsLookupConfig {
  expected_ip: string; // IP address as string  
  timeout_ms?: number;
}

export interface DnsOverHttpsConfig {
  domain: string;
  expected_ip: string; // IP address as string
  timeout_ms?: number;
}

export interface ReverseDnsConfig {
  expected_domain: string;
  timeout_ms?: number;
}

export interface VpnConnectivityConfig {
  timeout_ms?: number;
}

export interface VpnTunnelConfig {
  expected_subnet: string; // CIDR notation
  timeout_ms?: number;
}

export interface NtpSyncConfig {
  max_offset_ms: number;
  timeout_ms?: number;
}

// Test discriminated union matching backend Test enum
export type Test = 
  | { type: 'Connectivity'; config: ConnectivityConfig }
  | { type: 'DirectIp'; config: DirectIpConfig }
  | { type: 'Ping'; config: PingConfig }
  | { type: 'ServiceHealth'; config: ServiceHealthConfig }
  | { type: 'DnsResolution'; config: DnsResolutionConfig }
  | { type: 'DnsLookup'; config: DnsLookupConfig }
  | { type: 'DnsOverHttps'; config: DnsOverHttpsConfig }
  | { type: 'ReverseDns'; config: ReverseDnsConfig }
  | { type: 'VpnConnectivity'; config: VpnConnectivityConfig }
  | { type: 'VpnTunnel'; config: VpnTunnelConfig };

export type TestType = keyof typeof TEST_TYPE_CONFIG;