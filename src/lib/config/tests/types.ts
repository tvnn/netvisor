import type { TestConfiguration, ConnectivityConfig, DirectIpConfig, PingConfig, WellknownIpConfig, DnsResolutionConfig, DnsOverHttpsConfig, VpnConnectivityConfig, VpnTunnelConfig, ServiceHealthConfig } from "$lib/types/tests";

export const TEST_TYPE_CONFIG = {
  Connectivity: {
    display: 'Connectivity',
    category: 'Basic',
    description: {
      short: 'Tests basic TCP connectivity to a target host and port.',
      detailed: 'Establishes a TCP connection to verify that the target host is reachable and accepting connections on the specified port. This is the most fundamental network test.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success'
    },
    requiredFields: ['target'],
    optionalFields: ['port', 'protocol'],
    icon: 'Network',
    color: 'text-blue-400'
  },

  DirectIp: {
    display: 'Direct IP',
    category: 'Basic',
    description: {
      short: 'Tests connectivity directly to an IP address, bypassing DNS.',
      detailed: 'Connects directly to an IP address without using DNS, useful for testing when you suspect DNS issues or want to verify connectivity regardless of domain resolution.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success'
    },
    requiredFields: ['target', 'port'],
    optionalFields: [],
    icon: 'Route',
    color: 'text-blue-400'
  },

  Ping: {
    display: 'Ping',
    category: 'Basic',
    description: {
      short: 'Tests basic network reachability using ICMP ping packets.',
      detailed: 'Sends ICMP echo requests to test basic network connectivity and measure round-trip time. The most basic test to verify if a host is reachable.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success',
      attempts: 4
    },
    requiredFields: ['target'],
    optionalFields: ['attempts'],
    icon: 'Target',
    color: 'text-green-400'
  },

  WellknownIp: {
    display: 'Well-known IP',
    category: 'Internet',
    description: {
      short: 'Tests connectivity to reliable internet services like Google DNS.',
      detailed: 'Connects to well-known, highly available internet services (8.8.8.8, 1.1.1.1, etc.) to verify internet connectivity and DNS infrastructure.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success'
    },
    requiredFields: [],
    optionalFields: [],
    icon: 'Globe',
    color: 'text-green-400'
  },

  DnsResolution: {
    display: 'DNS Resolution',
    category: 'DNS',
    description: {
      short: 'Tests the ability to resolve domain names to IP addresses.',
      detailed: 'Attempts to resolve a domain name to its IP address using the system configured DNS servers. Verifies DNS resolution is working properly.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success'
    },
    requiredFields: ['domain'],
    optionalFields: [],
    icon: 'Search',
    color: 'text-purple-400'
  },

  DnsOverHttps: {
    display: 'DNS over HTTPS',
    category: 'DNS',
    description: {
      short: 'Tests DNS resolution using encrypted DNS over HTTPS services.',
      detailed: 'Performs DNS queries using DNS over HTTPS, which encrypts DNS requests and sends them over HTTPS connections for privacy and security.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success'
    },
    requiredFields: ['target', 'domain'],
    optionalFields: ['service_type'],
    icon: 'Shield',
    color: 'text-purple-400'
  },

  VpnConnectivity: {
    display: 'VPN Connectivity',
    category: 'VPN',
    description: {
      short: 'Tests connectivity to a VPN server endpoint.',
      detailed: 'Verifies that a VPN server is reachable and accepting connections on the configured port. Tests the basic network path to the VPN service.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success'
    },
    requiredFields: ['target'],
    optionalFields: ['port'],
    icon: 'Shield',
    color: 'text-orange-400'
  },

  VpnTunnel: {
    display: 'VPN Tunnel',
    category: 'VPN',
    description: {
      short: 'Tests if a VPN tunnel is active and can reach the expected subnet.',
      detailed: 'Verifies that a VPN connection is established and working by testing connectivity to the VPN internal network subnet. Confirms the tunnel is functional.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success',
      expected_subnet: '10.100.0.0/24'
    },
    requiredFields: ['expected_subnet'],
    optionalFields: [],
    icon: 'Tunnel',
    color: 'text-orange-400'
  },

  ServiceHealth: {
    display: 'Service Health',
    category: 'Application',
    description: {
      short: 'Tests HTTP/HTTPS service health and response codes.',
      detailed: 'Makes HTTP requests to a web service and checks the response status code to verify the service is running correctly beyond basic connectivity.'
    },
    defaultConfig: {
      timeout: 30000,
      expected_result: 'Success',
      path: '/',
      expected_status: 200
    },
    requiredFields: ['target'],
    optionalFields: ['port', 'path', 'expected_status'],
    icon: 'Activity',
    color: 'text-green-400'
  }
} as const;

export type TestType = keyof typeof TEST_TYPE_CONFIG;
// Utility functions

export const getTestDisplay = (type: TestType) => TEST_TYPE_CONFIG[type].display;
export const getTestDescription = (type: TestType) => TEST_TYPE_CONFIG[type].description;
export const getTestCategory = (type: TestType) => TEST_TYPE_CONFIG[type].category;
export const getTestIcon = (type: TestType) => TEST_TYPE_CONFIG[type].icon;
export const getTestColor = (type: TestType) => TEST_TYPE_CONFIG[type].color;
// Get tests by category

export const getTestsByCategory = (category: string) => Object.entries(TEST_TYPE_CONFIG)
  .filter(([_, config]) => config.category === category)
  .map(([type, _]) => type as TestType);// Helper to create default configuration for a test type

export const createDefaultTestConfig = (testType: TestType): TestConfiguration => {
  const defaults = TEST_TYPE_CONFIG[testType].defaultConfig;

  switch (testType) {
    case 'Connectivity':
      return {
        type: 'Connectivity',
        config: { target: '', ...defaults } as ConnectivityConfig
      };
    case 'DirectIp':
      return {
        type: 'DirectIp',
        config: { target: '', port: 80, ...defaults } as DirectIpConfig
      };
    case 'Ping':
      return {
        type: 'Ping',
        config: { target: '', ...defaults } as PingConfig
      };
    case 'WellknownIp':
      return {
        type: 'WellknownIp',
        config: { ...defaults } as WellknownIpConfig
      };
    case 'DnsResolution':
      return {
        type: 'DnsResolution',
        config: { domain: '', ...defaults } as DnsResolutionConfig
      };
    case 'DnsOverHttps':
      return {
        type: 'DnsOverHttps',
        config: { target: '', domain: '', ...defaults } as DnsOverHttpsConfig
      };
    case 'VpnConnectivity':
      return {
        type: 'VpnConnectivity',
        config: { target: '', ...defaults } as VpnConnectivityConfig
      };
    case 'VpnTunnel':
      return {
        type: 'VpnTunnel',
        config: { expected_subnet: '10.100.0.0/24', ...defaults } as VpnTunnelConfig
      };
    case 'ServiceHealth':
      return {
        type: 'ServiceHealth',
        config: { target: '', ...defaults } as ServiceHealthConfig
      };
    default:
      throw new Error(`Unknown test type: ${testType}`);
  }
};
// Validation helpers

export const validateTestConfig = (testConfig: TestConfiguration): string[] => {
  const errors: string[] = [];
  const { type, config } = testConfig;
  const requiredFields = TEST_TYPE_CONFIG[type].requiredFields;

  // Check required fields
  for (const field of requiredFields) {
    if (!(field in config) || !config[field as keyof typeof config]) {
      errors.push(`${field} is required for ${TEST_TYPE_CONFIG[type].display} test`);
    }
  }

  // Type-specific validation
  switch (type) {
    case 'DirectIp':
    case 'ServiceHealth':
      if ('port' in config && config.port && (config.port < 1 || config.port > 65535)) {
        errors.push('Port must be between 1 and 65535');
      }
      break;
    case 'Ping':
      if ('attempts' in config && config.attempts && config.attempts < 1) {
        errors.push('Attempts must be greater than 0');
      }
      break;
    case 'VpnTunnel':
      if ('expected_subnet' in config && config.expected_subnet) {
        // Basic CIDR validation
        const cidrPattern = /^(\d{1,3}\.){3}\d{1,3}\/\d{1,2}$/;
        if (!cidrPattern.test(config.expected_subnet)) {
          errors.push('Expected subnet must be in CIDR format (e.g., 10.100.0.0/24)');
        }
      }
      break;
  }

  return errors;
};

