import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { commands } from '../tauri-commands';
import type { Topology, TestTypeConfig, NetworkNode, TestConfig } from '../types';
import { getNodesReferencedInTopology } from './nodes';

// Store for all topologies
export const topologies: Writable<Topology[]> = writable([]);

// Test types available for topology creation with defaults
export const TEST_TYPES: Record<string, TestTypeConfig> = {
  // Basic connectivity tests
  connectivity_test: {
    name: 'Connectivity Test',
    description: 'Tests HTTP/HTTPS connection to a domain name (requires DNS resolution)',
    fields: ['target', 'port', 'protocol', 'timeout'],
    defaults: { target: '', port: 443, protocol: 'https', timeout: 5000 }
  },
  dns_resolution: {
    name: 'DNS Resolution',
    description: 'Tests if domain names can be resolved to IP addresses',
    fields: ['domain', 'timeout'],
    defaults: { domain: '', timeout: 5000 }
  },
  dns_over_https: {
    name: 'DNS over HTTPS',
    description: 'Tests secure DNS queries through HTTPS (Pi-hole, Cloudflare, etc.)',
    fields: ['target', 'test_domain', 'service_type', 'timeout'],
    defaults: { target: '', test_domain: 'google.com', service_type: 'auto', timeout: 5000 }
  },
  service_health: {
    name: 'Service Health',
    description: 'Tests if a web service returns expected HTTP status code',
    fields: ['target', 'port', 'path', 'expected_status', 'timeout'],
    defaults: { target: '', port: 80, path: '/', expected_status: 200, timeout: 5000 }
  },
  response_time: {
    name: 'Response Time',
    description: 'Measures connection latency and validates it meets threshold',
    fields: ['target', 'port', 'timeout', 'max_response_time'],
    defaults: { target: '', port: 443, timeout: 5000, max_response_time: 1000 }
  },
  ping_test: {
    name: 'Ping Test',
    description: 'Tests TCP connectivity with multiple attempts and success rate',
    fields: ['target', 'port', 'attempts', 'timeout'],
    defaults: { target: '', port: 443, attempts: 3, timeout: 5000 }
  },
  
  // VPN-specific tests
  vpn_connectivity: {
    name: 'VPN Endpoint Test',
    description: 'Tests if your VPN server endpoint is reachable (WireGuard port 51820)',
    fields: ['target', 'port', 'timeout'],
    defaults: { target: '', port: 51820, timeout: 5000 }
  },
  vpn_tunnel: {
    name: 'VPN Tunnel Validation',
    description: 'Checks if you\'re actually routing through VPN by validating public IP subnet',
    fields: ['target', 'timeout'],
    defaults: { target: '10.100.0.0/24', timeout: 5000 }
  },
  
  // Low-level network tests
  local_gateway: {
    name: 'Local Network Gateway',
    description: 'Tests connectivity to local router/gateway (works without internet)',
    fields: ['timeout'],
    defaults: { timeout: 3000 }
  },
  direct_ip: {
    name: 'Direct IP Connection',
    description: 'Tests connectivity to specific IP address (bypasses DNS completely)',
    fields: ['target', 'port', 'timeout'],
    defaults: { target: '', port: 443, timeout: 5000 }
  },
  wellknown_ip: {
    name: 'Internet Backbone Test',
    description: 'Tests internet connectivity using Google/Cloudflare DNS IPs (no DNS needed)',
    fields: ['timeout'],
    defaults: { timeout: 3000 }
  }
};

// Helper function to get default test configuration
export function getDefaultTestConfig(testType: string): TestConfig {
  const testTypeConfig = TEST_TYPES[testType];
  if (testTypeConfig && testTypeConfig.defaults) {
    return { ...testTypeConfig.defaults };
  }
  return { timeout: 5000 };
}

// Topology management functions
export const topologyActions = {
  async add(topology: Omit<Topology, 'id' | 'createdAt' | 'updatedAt'>): Promise<Topology> {
    try {
      const newTopology: Topology = {
        ...topology,
        id: crypto.randomUUID(),
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      };
      
      topologies.update(current => [...current, newTopology]);
      await commands.saveTopology(newTopology);
      
      return newTopology;
    } catch (error) {
      console.error('Failed to add topology:', error);
      throw error;
    }
  },

  async update(id: string, updates: Partial<Topology>): Promise<Topology> {
    try {
      const updatedTopology: Topology = {
        ...updates as Topology,
        id,
        updatedAt: new Date().toISOString()
      };
      
      topologies.update(current =>
        current.map(t => t.id === id ? updatedTopology : t)
      );
      
      await commands.updateTopology(id, updatedTopology);
      
      return updatedTopology;
    } catch (error) {
      console.error('Failed to update topology:', error);
      throw error;
    }
  },

  async delete(id: string): Promise<void> {
    try {
      topologies.update(current => current.filter(t => t.id !== id));
      await commands.deleteTopology(id);
    } catch (error) {
      console.error('Failed to delete topology:', error);
      throw error;
    }
  },

  async duplicate(id: string): Promise<Topology> {
    try {
      // Get current topologies synchronously
      let currentTopologies: Topology[] = [];
      const unsubscribe = topologies.subscribe(value => {
        currentTopologies = value;
      });
      unsubscribe(); // Immediately unsubscribe after getting the value
      
      const original = currentTopologies.find(t => t.id === id);
      if (!original) throw new Error('Topology not found');
      
      const duplicate: Omit<Topology, 'id' | 'createdAt' | 'updatedAt'> = {
        ...original,
        name: `${original.name} (Copy)`
      };
      
      return await this.add(duplicate);
    } catch (error) {
      console.error('Failed to duplicate topology:', error);
      throw error;
    }
  }
};

// Validation functions
export function validateTopology(topology: Topology, availableNodes: NetworkNode[] = []): string[] {
  const errors: string[] = [];
  
  if (!topology.name?.trim()) {
    errors.push('Name is required');
  }
  
  if (!topology.layers || !Array.isArray(topology.layers) || topology.layers.length === 0) {
    errors.push('At least one layer is required');
  }
  
  // Validate layers
  topology.layers?.forEach((layer, layerIndex) => {
    if (!layer.name?.trim()) {
      errors.push(`Layer ${layerIndex + 1}: Name is required`);
    }
    
    if (!layer.tests || !Array.isArray(layer.tests) || layer.tests.length === 0) {
      errors.push(`Layer ${layerIndex + 1}: At least one test is required`);
    }
    
    // Validate tests
    layer.tests?.forEach((test, testIndex) => {
      if (!test.type || !TEST_TYPES[test.type]) {
        errors.push(`Layer ${layerIndex + 1}, Test ${testIndex + 1}: Invalid test type`);
      }
      
      // Validate test configuration based on test type
      const testType = TEST_TYPES[test.type];
      if (testType) {
        testType.fields.forEach(field => {
          const value = test.config[field as keyof TestConfig];
          if (field === 'target' || field === 'domain') {
            if (!value || (typeof value === 'string' && !value.trim())) {
              errors.push(`Layer ${layerIndex + 1}, Test ${testIndex + 1}: ${field} is required`);
            }
          }
        });
      }
    });
  });
  
  return errors;
}

// Helper to create a blank topology
export function createBlankTopology(): Omit<Topology, 'id' | 'createdAt' | 'updatedAt'> {
  return {
    name: '',
    description: '',
    version: '1.0',
    layers: []
  };
}

// Load topologies on app start
export async function loadTopologies(): Promise<void> {
  try {
    const loadedTopologies = await commands.getTopologies();
    topologies.set(loadedTopologies);
  } catch (error) {
    console.error('Failed to load topologies:', error);
    // Set empty array on error
    topologies.set([]);
  }
}