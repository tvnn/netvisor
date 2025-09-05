// import type { TestSection } from "./forms";

export interface CapabilityTest {
  test: {
    type: string,
    config: Record<string, any>
  };  // Test enum as string
  criticality: string;  // TestCriticality enum as string
  enabled: boolean;
}

export interface Capability {
  [capabilityType: string]: CapabilityConfig
}

export interface CapabilityConfig extends CapabilityConfigBase {
  [key: string]: any;
}

export interface CapabilityConfigBase {
  name: string;  // "API Server", "Node-Level Tests", "Daemon Service"
  // tests: CapabilityTest[];
  port?: number;
  process?: string;
  discovery_ports?: number[];
}

// Helper functions to work with the generic structure
export function getCapabilityType(capability: Capability): string {
  return Object.keys(capability)[0];
}

export function getCapabilityConfig(capability: Capability): CapabilityConfig {
  const type = getCapabilityType(capability);
  return capability[type];
}

export function createCapability(type: string, config: CapabilityConfig): Capability {
  return {
    [type]: {
      ...config
    }
  };
}

export function updateCapabilityConfig(capability: Capability, updates: Partial<CapabilityConfig>): Capability {
  const type = getCapabilityType(capability);
  const config = getCapabilityConfig(capability);
  
  return {
    [type]: {
      ...config,
      ...updates
    }
  };
}

// export function getTestConfigFromSchema(section: TestSection): Record<string, any> {
//   const config: Record<string, any> = {};
  
//   section.test_fields.forEach(field => {
//     if (field.default_value !== undefined && field.id !== 'criticality') {
//       config[field.id] = field.default_value;
//     }
//   });
  
//   return config;
// }
