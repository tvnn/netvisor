import type { TestConfiguration, TestType } from "./tests";

// Base form data - what the form actually handles
export interface NodeFormData {
  name: string;
  domain: string;
  ip: string;
  port: number;
  path: string;
  description: string;
  node_type: NodeType;
  capabilities: NodeCapability[];
  monitoring_enabled: boolean;
  assigned_tests: AssignedTest[];
}

// API data model - what the backend expects/returns
export interface NodeApi {
  name: string;
  domain?: string;       
  ip?: string;          
  port?: number;        
  path?: string;        
  description?: string; 
  node_type?: NodeType;
  capabilities: NodeCapability[];
  monitoring_enabled: boolean;
  assigned_tests: AssignedTest[];
  node_groups: string[];
  position?: GraphPosition;
  current_status: NodeStatus;
  subnet_membership: string[];
}

export interface Node extends NodeApi {
  id: string;
  created_at: string;
  updated_at: string;
  last_seen?: string;
}

export type NodeType = 'Router' | 'Switch' | 'AccessPoint' | 'Firewall' |
  'WebServer' | 'DatabaseServer' | 'MediaServer' | 'DnsServer' | 'VpnServer' | 'NasDevice' |
  'Workstation' | 'IotDevice' | 'Printer' | 'Camera' |
  'UnknownDevice';

export type NodeCapability = 'SshAccess' | 'RdpAccess' | 'VncAccess' |
  'HttpService' | 'HttpsService' |
  'DatabaseService' |
  'DnsService' | 'EmailService' | 'FtpService';

export type NodeStatus = 'Healthy' | 'Degraded' | 'Failed' | 'Unknown';

export interface AssignedTest {
  test_type: TestType;
  test_config: TestConfiguration;
  monitor_interval_minutes?: number;
  enabled: boolean;
  criticality: TestCriticality;
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

export type TestCriticality = 'Critical' | 'Important' | 'Informational';

export interface GraphPosition {
  x: number;
  y: number;
  z?: number;
}

// Utility functions
export function getNodeTypeDisplayName(nodeType: NodeType): string {
  switch (nodeType) {
    case 'Router': return 'Router';
    case 'Switch': return 'Switch';
    case 'AccessPoint': return 'Access Point';
    case 'Firewall': return 'Firewall';
    case 'WebServer': return 'Web Server';
    case 'DatabaseServer': return 'Database Server';
    case 'MediaServer': return 'Media Server';
    case 'DnsServer': return 'DNS Server';
    case 'VpnServer': return 'VPN Server';
    case 'NasDevice': return 'NAS Device';
    case 'Workstation': return 'Workstation';
    case 'IotDevice': return 'IoT Device';
    case 'Printer': return 'Printer';
    case 'Camera': return 'Camera';
    case 'UnknownDevice': return 'Unknown Device';
    default: return nodeType;
  }
}
export function getCriticalityDisplayName(criticality: TestCriticality): string {
  switch (criticality) {
    case 'Critical': return 'Critical';
    case 'Important': return 'Important';
    case 'Informational': return 'Informational';
    default: return criticality;
  }
}

export function getNodeStatusDisplayName(status: NodeStatus): string {
  switch (status) {
    case 'Healthy': return 'Healthy';
    case 'Degraded': return 'Degraded';
    case 'Failed': return 'Failed';
    case 'Unknown': return 'Unknown';
    default: return status;
  }
}

export function getNodeStatusColor(status: NodeStatus): string {
  switch (status) {
    case 'Healthy': return 'text-green-400';
    case 'Degraded': return 'text-yellow-400';
    case 'Failed': return 'text-red-400';
    case 'Unknown': return 'text-gray-400';
    default: return 'text-gray-400';
  }
}

export function getCriticalityColor(criticality: TestCriticality): string {
  switch (criticality) {
    case 'Critical': return 'text-red-400';
    case 'Important': return 'text-yellow-400';
    case 'Informational': return 'text-blue-400';
    default: return 'text-gray-400';
  }
}

