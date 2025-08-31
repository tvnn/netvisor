import type { NodeTarget } from "./targets";

export interface NodeContext {
  node_id?: string;
  node_type: string;
  capabilities: Record<string, any>[];
  target: any;
}
export interface Node {
  id: string;
  created_at: string;
  updated_at: string;
  last_seen?: string;
  name: string;
  description?: string;
  hostname?: string;
  target: NodeTarget;
  node_type: string;
  capabilities: Record<string, NodeCapability>[];
  mac_address?: string;
  subnets: string[]; // CIDR blocks
  monitoring_interval: number;
  node_groups?: string[];
  discovery_status?: string;
  status?: string;
}

export type NodeCapability = {
  source: Record<string, any>;
  daemon_id: string;
};
