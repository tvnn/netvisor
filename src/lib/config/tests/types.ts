import type { ConnectivityConfig, DirectIpConfig, PingConfig, DnsResolutionConfig, DnsOverHttpsConfig, VpnConnectivityConfig, VpnTunnelConfig, ServiceHealthConfig, Test, DnsLookupConfig, ReverseDnsConfig, TestType } from "$lib/types/tests";

interface TestTypeConfig {
  display: string;
  category: string;
  description: Record<string, string>,
  defaultConfig: Record<string, any>,
  requiredFields: string[],
  optionalFields: string[]
  icon: string,
  color: string,
}

export const TEST_TYPE_CONFIG: Record<string, TestTypeConfig> = {
  Connectivity: {
    display: 'Connectivity',
    category: 'Basic',
    description: {
      short: 'Tests basic TCP connectivity to the node\'s target.',
      detailed: 'Establishes a TCP connection to verify that the node is reachable and accepting connections. Uses the node\'s configured target (IP, hostname, or service).'
    },
    defaultConfig: {
      timeout_ms: 30000
    },
    requiredFields: [],
    optionalFields: ['timeout_ms'],
    icon: 'Network',
    color: 'text-blue-400'
  },

  DirectIp: {
    display: 'Direct IP',
    category: 'Basic',
    description: {
      short: 'Tests connectivity directly to the node\'s IP address, bypassing DNS.',
      detailed: 'Connects directly to an IP address without using DNS. Requires the node to have an IP address target configured.'
    },
    defaultConfig: {
      timeout_ms: 30000
    },
    requiredFields: [],
    optionalFields: ['timeout_ms'],
    icon: 'Route',
    color: 'text-blue-400'
  },

  Ping: {
    display: 'Ping',
    category: 'Basic',
    description: {
      short: 'Tests basic network reachability using ICMP ping packets.',
      detailed: 'Sends ICMP echo requests to test basic network connectivity and measure round-trip time to the node\'s IP address.'
    },
    defaultConfig: {
      packet_count: 4,
      timeout_ms: 30000
    },
    requiredFields: [],
    optionalFields: ['packet_count', 'timeout_ms'],
    icon: 'Target',
    color: 'text-green-400'
  },

  ServiceHealth: {
    display: 'Service Health',
    category: 'Application',
    description: {
      short: 'Tests HTTP/HTTPS service health using the node\'s service configuration.',
      detailed: 'Makes HTTP requests to the node\'s configured service endpoint and checks response status codes to verify the service is running correctly.'
    },
    defaultConfig: {
      expected_status_code: 200,
      timeout_ms: 30000
    },
    requiredFields: ['expected_status_code'],
    optionalFields: ['timeout_ms'],
    icon: 'Activity',
    color: 'text-green-400'
  },

  DnsResolution: {
    display: 'DNS Resolution',
    category: 'DNS',
    description: {
      short: 'Tests the node\'s ability to resolve domain names to IP addresses.',
      detailed: 'Tests whether this DNS server node can resolve a specific domain to an expected IP address. Validates DNS server functionality.'
    },
    defaultConfig: {
      domain: 'example.com',
      expected_ip: '127.0.0.1',
      timeout_ms: 30000
    },
    requiredFields: ['domain', 'expected_ip'],
    optionalFields: ['timeout_ms'],
    icon: 'Search',
    color: 'text-purple-400'
  },

  DnsLookup: {
    display: 'DNS Lookup',
    category: 'DNS',
    description: {
      short: 'Tests whether this node\'s domain resolves to the expected IP address.',
      detailed: 'Validates that a domain name resolves to this node\'s IP address using external DNS servers. Verifies DNS propagation and accessibility.'
    },
    defaultConfig: {
      expected_ip: '127.0.0.1',
      timeout_ms: 30000
    },
    requiredFields: ['expected_ip'],
    optionalFields: ['timeout_ms'],
    icon: 'Search',
    color: 'text-purple-400'
  },

  DnsOverHttps: {
    display: 'DNS over HTTPS',
    category: 'DNS',
    description: {
      short: 'Tests DNS resolution using this node\'s DNS over HTTPS service.',
      detailed: 'Performs DNS queries using DNS over HTTPS on this node, which encrypts DNS requests and sends them over HTTPS connections.'
    },
    defaultConfig: {
      domain: 'example.com',
      expected_ip: '127.0.0.1',
      timeout_ms: 30000
    },
    requiredFields: ['domain', 'expected_ip'],
    optionalFields: ['timeout_ms'],
    icon: 'Shield',
    color: 'text-purple-400'
  },

  ReverseDns: {
    display: 'Reverse DNS',
    category: 'DNS',
    description: {
      short: 'Tests reverse DNS lookup for this node\'s IP address.',
      detailed: 'Performs reverse DNS lookup to verify that this node\'s IP address resolves to the expected domain name.'
    },
    defaultConfig: {
      expected_domain: 'example.com',
      timeout_ms: 30000
    },
    requiredFields: ['expected_domain'],
    optionalFields: ['timeout_ms'],
    icon: 'RotateCcw',
    color: 'text-purple-400'
  },

  VpnConnectivity: {
    display: 'VPN Connectivity',
    category: 'VPN',
    description: {
      short: 'Tests connectivity to this VPN server node.',
      detailed: 'Verifies that this VPN server is reachable and accepting connections. Tests the basic network path to the VPN service.'
    },
    defaultConfig: {
      timeout_ms: 30000
    },
    requiredFields: [],
    optionalFields: ['timeout_ms'],
    icon: 'Shield',
    color: 'text-orange-400'
  },

  VpnTunnel: {
    display: 'VPN Tunnel',
    category: 'VPN',
    description: {
      short: 'Tests if this VPN node\'s tunnel is active and accessible.',
      detailed: 'Verifies that a VPN connection through this node is established and working by testing connectivity to the expected VPN subnet.'
    },
    defaultConfig: {
      expected_subnet: '10.100.0.0/24',
      timeout_ms: 30000
    },
    requiredFields: ['expected_subnet'],
    optionalFields: ['timeout_ms'],
    icon: 'Tunnel',
    color: 'text-orange-400'
  }
} as const;

export const getTestDisplay = (type: TestType) => TEST_TYPE_CONFIG[type].display;
export const getTestDescription = (type: TestType) => TEST_TYPE_CONFIG[type].description;
export const getTestCategory = (type: TestType) => TEST_TYPE_CONFIG[type].category;
export const getTestIcon = (type: TestType) => TEST_TYPE_CONFIG[type].icon;
export const getTestColor = (type: TestType) => TEST_TYPE_CONFIG[type].color;
export const getTestDefaultConfig = (type: TestType) => TEST_TYPE_CONFIG[type].defaultConfig;

export const getTestTypes = () => Object.keys(TEST_TYPE_CONFIG) as TestType[];

export const getTestsByCategory = (category: string) => Object.entries(TEST_TYPE_CONFIG)
  .filter(([_, config]) => config.category === category)
  .map(([type, _]) => type as TestType);// Helper to create default configuration for a test type

  // Type guards for safe access
export function isConnectivityTest(test: Test): test is { type: 'Connectivity'; config: ConnectivityConfig } {
  return test.type === 'Connectivity';
}

export function isDirectIpTest(test: Test): test is { type: 'DirectIp'; config: DirectIpConfig } {
  return test.type === 'DirectIp';
}

export function isPingTest(test: Test): test is { type: 'Ping'; config: PingConfig } {
  return test.type === 'Ping';
}

export function isServiceHealthTest(test: Test): test is { type: 'ServiceHealth'; config: ServiceHealthConfig } {
  return test.type === 'ServiceHealth';
}

export function isDnsResolutionTest(test: Test): test is { type: 'DnsResolution'; config: DnsResolutionConfig } {
  return test.type === 'DnsResolution';
}

export function isDnsLookupTest(test: Test): test is { type: 'DnsLookup'; config: DnsLookupConfig } {
  return test.type === 'DnsLookup';
}

export function isDnsOverHttpsTest(test: Test): test is { type: 'DnsOverHttps'; config: DnsOverHttpsConfig } {
  return test.type === 'DnsOverHttps';
}

export function isReverseDnsTest(test: Test): test is { type: 'ReverseDns'; config: ReverseDnsConfig } {
  return test.type === 'ReverseDns';
}

export function isVpnConnectivityTest(test: Test): test is { type: 'VpnConnectivity'; config: VpnConnectivityConfig } {
  return test.type === 'VpnConnectivity';
}

export function isVpnTunnelTest(test: Test): test is { type: 'VpnTunnel'; config: VpnTunnelConfig } {
  return test.type === 'VpnTunnel';
}

// Utility functions
export function createDefaultTest(testType: TestType): Test {
  let defaultConfig = getTestDefaultConfig(testType);
  switch (testType) {
    case 'Connectivity':
      return { type: 'Connectivity', config: defaultConfig as ConnectivityConfig}
    case 'DirectIp':
      return { type: 'DirectIp', config: defaultConfig as DirectIpConfig}
    case 'Ping':
      return { type: 'Ping', config: defaultConfig as PingConfig}
    case 'ServiceHealth':
      return { type: 'ServiceHealth', config: defaultConfig as ServiceHealthConfig}
    case 'DnsResolution':
      return { type: 'DnsResolution', config: defaultConfig as DnsResolutionConfig}
    case 'DnsLookup':
      return { type: 'DnsLookup', config: defaultConfig as DnsLookupConfig}
    case 'DnsOverHttps':
      return { type: 'DnsOverHttps', config: defaultConfig as DnsOverHttpsConfig}
    case 'ReverseDns':
      return { type: 'ReverseDns', config: defaultConfig as ReverseDnsConfig}
    case 'VpnConnectivity':
      return { type: 'VpnConnectivity', config: defaultConfig as VpnConnectivityConfig}
    case 'VpnTunnel':
      return { type: 'VpnTunnel', config: defaultConfig as VpnTunnelConfig}
    default:
      throw new Error(`Unknown test type: ${testType}`);
  }
}