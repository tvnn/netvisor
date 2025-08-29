import { getNodeTargetMetadata } from "$lib/api/registry";
import { get } from 'svelte/store';
import type { TestConfigSchema, TestResult } from "../tests/types";

// Base form data - what the form actually handles
export interface NodeFormData {
  name: string;
  description: string;
  hostname?: string;                    
  target: NodeTarget;
  node_type: string;
  capabilities: Record<string, any>[];
  mac_address?: string;                 
  subnets: string[];                   
  monitoring_interval: number;
  assigned_tests: AssignedTest[];                   
}
// API data model - what the backend expects/returns
export interface NodeApi {
  name: string;
  description?: string;
  hostname?: string;
  target: NodeTarget;
  node_type: string;
  capabilities: Record<string, NodeCapability>[];
  mac_address?: string;
  subnets: string[]; // CIDR blocks
  monitoring_interval: number;
  assigned_tests: AssignedTest[];
  node_groups?: string[];
  discovery_status?: string;              
  status?: string;
}

export interface Node extends NodeApi {
  id: string;
  created_at: string;
  updated_at: string;
  last_seen?: string;
}

export interface IpNodeTargetConfig {
  ip: string;
}

export interface UrlTargetConfig {
  hostname: string;
  path: string;
  protocol: ApplicationProtocol
}

export enum ApplicationProtocol {
  Http = "Http",
  Https = "Https",
  Ftp = "Ftp"
}

export type NodeCapability = {
  source: Record<string, any>,
  daemon_id: string
}

export type NodeTarget =
  | { type: 'IpAddress', config: IpNodeTargetConfig}
  | { type: 'Url', config: UrlTargetConfig}

export interface AssignedTest {
  test: {
    type: string,
    config: Record<string, any>
  };
  criticality: string;
}

export interface NodeResult {
  test_results: TestResult[];
  executed_at: string;
  node_status: string;
  duration_ms: number;
  node_id: string;
}
// Utility functions
export function createDefaultTestsFromSchemas(schemas: Record<string, TestConfigSchema>): AssignedTest[] {
  const defaultTests: AssignedTest[] = [];
  
  // Create Connectivity test from schema if available
  if (schemas['Connectivity']) {
    const connectivitySchema = schemas['Connectivity'];
    const defaultConfig: Record<string, any> = {};
    
    // Extract default values from schema fields
    connectivitySchema.fields?.forEach(field => {
      if (field.default_value !== null && field.default_value !== undefined) {
        defaultConfig[field.id] = field.default_value;
      }
    });
    
    defaultTests.push({
      test: {
        type: 'Connectivity',
        config: defaultConfig
      },
      criticality: 'Critical'
    });
  }
  
  return defaultTests;
}

// In src/lib/components/nodes/types.ts
export function nodeToFormData(node: Node): NodeFormData {
  return {
    name: node.name,
    description: node.description || '',
    hostname: node.hostname || '',                    
    target: node.target,
    node_type: node.node_type,
    capabilities: [...node.capabilities],
    mac_address: node.mac_address || '',             
    subnets: node.subnets || [],                     
    monitoring_interval: node.monitoring_interval,
    assigned_tests: [...node.assigned_tests],
  };
}

export function formDataToNodeApi(formData: NodeFormData): NodeApi {
  return {
    name: formData.name.trim(),
    description: formData.description.trim() || undefined,
    hostname: formData.hostname?.trim() || undefined,  
    target: formData.target,
    node_type: formData.node_type,
    mac_address: formData.mac_address?.trim() || undefined, 
    capabilities: formData.capabilities,
    subnets: formData.subnets || [],                
    monitoring_interval: formData.monitoring_interval,
    assigned_tests: formData.assigned_tests, 
  };
}

export function createEmptyNodeFormData(): NodeFormData {
  return {
    name: '',
    description: '',
    hostname: '',              
    target: {
      type: 'IpAddress',
      config: {
        ip: '',
      },
    },
    node_type: 'UnknownDevice',
    capabilities: [],
    mac_address: '',           
    subnets: [],              
    monitoring_interval: 10,
    assigned_tests: [],
  };
}

export function getNodeTargetString(target: NodeTarget): string {
  switch (target.type) {
    case 'IpAddress':
      return target.config.ip;
    case 'Url':
      const base = `${target.config.protocol.toLowerCase()}://${target.config.hostname}`;
      const path = target.config.path || '';
      return base + path;
    default:
      return 'Unknown target';
  }
}

export function validateTarget(target: NodeTarget): string[] {
  const errors: string[] = [];
  
  switch (target.type) {
    case 'IpAddress':
      if (!target.config.ip) {
        errors.push('IP address is required');
      }
      break;
    case 'Url':
      if (!target.config.protocol) {
        errors.push('Protocol is required');
      }
      if (!target.config.hostname) {
        errors.push('Hostname is required');
      }
      break;
  }
  
  return errors;
}
export interface NodeListResponse {
  nodes: Node[];
  total: number;
}
export interface NodeResponse {
  node: Node;
}
