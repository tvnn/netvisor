import type { NODE_TYPE_CONFIG } from "$lib/config/nodes/types";
import type { TestConfiguration } from "./tests";
import type { TestType } from "$lib/config/tests/types";
import type { CRITICALITY_CONFIG } from "$lib/config/nodes/criticality";
import type { NODE_STATUS_CONFIG } from "$lib/config/nodes/status";
import type { CAPABILITY_CONFIG } from "$lib/config/nodes/capabilities";

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
  node_type: NodeType;
  domain?: string;       
  ip?: string;          
  port?: number;        
  path?: string;        
  description?: string; 
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

export interface GraphPosition {
  x: number;
  y: number;
  z?: number;
}
export type NodeType = keyof typeof NODE_TYPE_CONFIG;
export type TestCriticality = keyof typeof CRITICALITY_CONFIG;
export type NodeStatus = keyof typeof NODE_STATUS_CONFIG;
export type NodeCapability = keyof typeof CAPABILITY_CONFIG;

