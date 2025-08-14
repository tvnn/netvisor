import { type TestType } from '$lib/config/tests/types'
import { type AssignedTest } from '$lib/types/nodes'

// Base configuration that all tests share
interface BaseTestConfig {
  timeout?: number;
  expected_result: 'Success' | 'Failure';
}

export interface TestResult {
  test_type: TestType;
  success: boolean;
  message: string;
  duration_ms: number;
  executed_at: string;
  details?: any;
}

// Specific configurations for each test type
export interface ConnectivityConfig extends BaseTestConfig {
  target: string;
  port?: number;
  protocol?: 'tcp' | 'udp';
}

export interface DirectIpConfig extends BaseTestConfig {
  target: string;
  port: number;
}

export interface PingConfig extends BaseTestConfig {
  target: string;
  attempts?: number;
}

export interface WellknownIpConfig extends BaseTestConfig {
  // No additional fields - uses predefined targets
}

export interface DnsResolutionConfig extends BaseTestConfig {
  domain: string;
}

export interface DnsOverHttpsConfig extends BaseTestConfig {
  target: string;
  domain: string;
  service_type?: string;
}

export interface VpnConnectivityConfig extends BaseTestConfig {
  target: string;
  port?: number;
}

export interface VpnTunnelConfig extends BaseTestConfig {
  expected_subnet: string;
}

export interface ServiceHealthConfig extends BaseTestConfig {
  target: string;
  port?: number;
  path?: string;
  expected_status?: number;
}

// FIXED: Use discriminated union with 'type' field for proper type narrowing
export type TestConfiguration = 
  | { type: 'Connectivity'; config: ConnectivityConfig }
  | { type: 'DirectIp'; config: DirectIpConfig }
  | { type: 'Ping'; config: PingConfig }
  | { type: 'WellknownIp'; config: WellknownIpConfig }
  | { type: 'DnsResolution'; config: DnsResolutionConfig }
  | { type: 'DnsOverHttps'; config: DnsOverHttpsConfig }
  | { type: 'VpnConnectivity'; config: VpnConnectivityConfig }
  | { type: 'VpnTunnel'; config: VpnTunnelConfig }
  | { type: 'ServiceHealth'; config: ServiceHealthConfig };

// =============================================================================
// TYPE GUARD FUNCTIONS FOR SAFE ACCESS
// =============================================================================

export function isConnectivityConfig(testConfig: TestConfiguration): testConfig is { type: 'Connectivity'; config: ConnectivityConfig } {
  return testConfig.type === 'Connectivity';
}

export function isDirectIpConfig(testConfig: TestConfiguration): testConfig is { type: 'DirectIp'; config: DirectIpConfig } {
  return testConfig.type === 'DirectIp';
}

export function isPingConfig(testConfig: TestConfiguration): testConfig is { type: 'Ping'; config: PingConfig } {
  return testConfig.type === 'Ping';
}

export function isWellknownIpConfig(testConfig: TestConfiguration): testConfig is { type: 'WellknownIp'; config: WellknownIpConfig } {
  return testConfig.type === 'WellknownIp';
}

export function isDnsResolutionConfig(testConfig: TestConfiguration): testConfig is { type: 'DnsResolution'; config: DnsResolutionConfig } {
  return testConfig.type === 'DnsResolution';
}

export function isDnsOverHttpsConfig(testConfig: TestConfiguration): testConfig is { type: 'DnsOverHttps'; config: DnsOverHttpsConfig } {
  return testConfig.type === 'DnsOverHttps';
}

export function isVpnConnectivityConfig(testConfig: TestConfiguration): testConfig is { type: 'VpnConnectivity'; config: VpnConnectivityConfig } {
  return testConfig.type === 'VpnConnectivity';
}

export function isVpnTunnelConfig(testConfig: TestConfiguration): testConfig is { type: 'VpnTunnel'; config: VpnTunnelConfig } {
  return testConfig.type === 'VpnTunnel';
}

export function isServiceHealthConfig(testConfig: TestConfiguration): testConfig is { type: 'ServiceHealth'; config: ServiceHealthConfig } {
  return testConfig.type === 'ServiceHealth';
}

// =============================================================================
// UTILITY FUNCTIONS FOR EXTRACTING CONFIG DATA
// =============================================================================

// Extract target information for display purposes
export function getTestTarget(testConfig: TestConfiguration): string {
  if (isConnectivityConfig(testConfig)) {
    return testConfig.config.target;
  }
  if (isDirectIpConfig(testConfig)) {
    return testConfig.config.target;
  }
  if (isPingConfig(testConfig)) {
    return testConfig.config.target;
  }
  if (isDnsResolutionConfig(testConfig)) {
    return testConfig.config.domain;
  }
  if (isDnsOverHttpsConfig(testConfig)) {
    return testConfig.config.domain;
  }
  if (isVpnConnectivityConfig(testConfig)) {
    return testConfig.config.target;
  }
  if (isVpnTunnelConfig(testConfig)) {
    return testConfig.config.expected_subnet;
  }
  if (isServiceHealthConfig(testConfig)) {
    return testConfig.config.target;
  }
  if (isWellknownIpConfig(testConfig)) {
    return 'Well-known IPs';
  }
  return 'Unknown';
}

// Extract port information if available
export function getTestPort(testConfig: TestConfiguration): number | undefined {
  if (isConnectivityConfig(testConfig)) {
    return testConfig.config.port;
  }
  if (isDirectIpConfig(testConfig)) {
    return testConfig.config.port;
  }
  if (isVpnConnectivityConfig(testConfig)) {
    return testConfig.config.port;
  }
  if (isServiceHealthConfig(testConfig)) {
    return testConfig.config.port;
  }
  return undefined;
}

export function extractConfigFromTest(assignedTest: AssignedTest): any {
  const testConfig = assignedTest.test_config;
  
  // Use type guards for safe access
  if (isConnectivityConfig(testConfig)) {
    return {
      target: testConfig.config.target,
      port: testConfig.config.port?.toString() || '',
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result,
      protocol: testConfig.config.protocol || 'tcp'
    };
  }
  
  if (isDirectIpConfig(testConfig)) {
    return {
      target: testConfig.config.target,
      port: testConfig.config.port.toString(),
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result
    };
  }
  
  if (isPingConfig(testConfig)) {
    return {
      target: testConfig.config.target,
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result,
      attempts: testConfig.config.attempts?.toString() || '4'
    };
  }
  
  if (isWellknownIpConfig(testConfig)) {
    return {
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result
    };
  }
  
  if (isDnsResolutionConfig(testConfig)) {
    return {
      domain: testConfig.config.domain,
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result
    };
  }
  
  if (isDnsOverHttpsConfig(testConfig)) {
    return {
      target: testConfig.config.target,
      domain: testConfig.config.domain,
      service_type: testConfig.config.service_type || '',
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result
    };
  }
  
  if (isVpnConnectivityConfig(testConfig)) {
    return {
      target: testConfig.config.target,
      port: testConfig.config.port?.toString() || '',
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result
    };
  }
  
  if (isVpnTunnelConfig(testConfig)) {
    return {
      expected_subnet: testConfig.config.expected_subnet,
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result
    };
  }
  
  if (isServiceHealthConfig(testConfig)) {
    return {
      target: testConfig.config.target,
      port: testConfig.config.port?.toString() || '',
      path: testConfig.config.path || '/',
      expected_status: testConfig.config.expected_status?.toString() || '200',
      timeout: testConfig.config.timeout?.toString() || '30000',
      expected_result: testConfig.config.expected_result
    };
  }
  
  // Fallback
  return {
    timeout: '30000',
    expected_result: 'Success'
  };
}