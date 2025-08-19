import type { NODE_TYPE_CONFIG } from "$lib/config/nodes/types";
import { getNodeTargetTypeDefaultConfig, type NODE_TARGET_TYPE_CONFIG } from "$lib/config/nodes/targets";
import type { Test } from "./tests";
import type { CRITICALITY_CONFIG } from "$lib/config/nodes/criticality";
import type { NODE_STATUS_CONFIG } from "$lib/config/nodes/status";
import type { CAPABILITY_CONFIG } from "$lib/config/nodes/capabilities";

// Base form data - what the form actually handles
export interface NodeFormData {
  name: string;
  description: string;
  
  // Target configuration
  target: NodeTarget;
  
  node_type: NodeType;
  capabilities: NodeCapability[];
  
  // Discovery data (auto-populated)
  open_ports?: number[];
  detected_services?: DetectedService[];
  mac_address?: string;
  vlan_id?: number;
  
  // Monitoring configuration
  monitoring_interval: number; // 0 = disabled, >0 = interval
  assigned_tests: AssignedTest[];
  
  // Group membership
  node_groups: string[];
}

// API data model - what the backend expects/returns
export interface NodeApi {
  name: string;
  description?: string;
  
  // Target configuration matching Rust NodeTarget enum
  target: NodeTarget;
  
  node_type: NodeType;
  
  // Discovery & Capability Data
  open_ports: number[];
  detected_services: DetectedService[];
  mac_address?: string;
  capabilities: NodeCapability[];
  
  // Network Context
  subnet_membership: string[]; // CIDR blocks
  vlan_id?: number;
  
  // Monitoring Configuration
  monitoring_interval: number; // minutes, 0 = disabled, >0 = enabled with interval
  assigned_tests: AssignedTest[];
  
  // Standard Fields
  node_groups: string[]; // Group IDs this node belongs to
  position?: GraphPosition;
  current_status: NodeStatus;
}

export interface Node extends NodeApi {
  id: string;
  created_at: string;
  updated_at: string;
  last_seen?: string;
}

export interface Ipv4NodeTargetConfig {
  ip: string;
  port: number;
}

export interface Ipv6NodeTargetConfig {
  ip: string;
  port: number;
}

export interface HostnameTargetConfig {
  hostname: string;
  port: number;
}

export interface ServiceTargetConfig {
  hostname: string;
  port: number;
  path: string;
  protocol: ApplicationProtocol
}

export enum ApplicationProtocol {
  Http,
  Https,
  Ftp
}

export enum TransportProtocol {
  Udp,
  Tcp
}

export type NodeTarget =
  | { type: 'Ipv4Address', config: Ipv4NodeTargetConfig}
  | { type: 'Ipv6Address', config: Ipv6NodeTargetConfig}
  | { type: 'Hostname', config: HostnameTargetConfig}
  | { type: 'Service', config: ServiceTargetConfig}

export interface AssignedTest {
  test: Test;
  criticality: TestCriticality;
}

export interface DetectedService {
  port: number;
  protocol: string;        // "HTTP", "SSH", "MySQL", "Unknown"
  service_name?: string;   // "nginx", "OpenSSH", "MySQL"
  version?: string;        // "1.20.1", "8.9p1", "8.0.32"
  banner?: string;         // Raw service banner
  confidence: number;      // 0.0-1.0 detection confidence
}

export interface SubnetGroup {
  id: string;
  name: string;
  cidr: string;
  node_ids: string[];
  vlan_id?: number;
  created_at: string;
  updated_at: string;
}

export interface GraphPosition {
  x: number;
  y: number;
  z?: number;
}
export type NodeTargetType = keyof typeof NODE_TARGET_TYPE_CONFIG;
export type NodeType = keyof typeof NODE_TYPE_CONFIG;
export type TestCriticality = keyof typeof CRITICALITY_CONFIG;
export type NodeStatus = keyof typeof NODE_STATUS_CONFIG;
export type NodeCapability = keyof typeof CAPABILITY_CONFIG;

// Utility functions
export function createEmptyNodeFormData(): NodeFormData {
  return {
    name: '',
    description: '',
    target: {
      type: 'Ipv4Address',
      config: getNodeTargetTypeDefaultConfig('Ipv4Address'),
    },
    node_type: 'UnknownDevice',
    capabilities: [],
    open_ports: [],
    detected_services: [],
    monitoring_interval: 10,
    assigned_tests: [],
    node_groups: []
  };
}

export function nodeToFormData(node: Node): NodeFormData {
  return {
    name: node.name,
    description: node.description || '',
    target: node.target,
    node_type: node.node_type,
    capabilities: [...node.capabilities],
    open_ports: [...node.open_ports],
    detected_services: [...node.detected_services],
    mac_address: node.mac_address,
    vlan_id: node.vlan_id,
    monitoring_interval: node.monitoring_interval,
    assigned_tests: [...node.assigned_tests],
    node_groups: [...node.node_groups]
  };
}

export function formDataToNodeApi(formData: NodeFormData): NodeApi {
  return {
    name: formData.name.trim(),
    description: formData.description.trim() || undefined,
    target: formData.target,
    node_type: formData.node_type,
    open_ports: formData.open_ports || [],
    detected_services: formData.detected_services || [],
    mac_address: formData.mac_address,
    capabilities: formData.capabilities,
    subnet_membership: [],
    vlan_id: formData.vlan_id,
    monitoring_interval: formData.monitoring_interval,
    assigned_tests: formData.assigned_tests,
    node_groups: formData.node_groups,
    current_status: 'Unknown'
  };
}