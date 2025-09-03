import type { Capability } from "$lib/features/capabilities/types/base";
import type { NodeTarget } from "./targets";

export interface NodeContext {
  node_id?: string;
  node_type: string;
  capabilities: Capability[];
  target: any;
}
export interface Node {
  id: string;
  created_at: string;
  updated_at: string;
  last_seen: string;
  name: string;
  description: string;
  hostname: string;
  target: NodeTarget;
  node_type: string;
  capabilities: Capability[];

  subnets: NodeSubnetMembership[];
  monitoring_interval: number;
  node_groups: string[];
  discovery_status: string;
  status: string;
  dns_resolver_node_id: string;
}

export type NodeCapability = {
  source: Record<string, any>;
  daemon_id: string;
};
