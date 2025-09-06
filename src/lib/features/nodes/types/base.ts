import type { Service } from "$lib/features/services/types/base";
import type { NodeTarget } from "./targets";

export interface Node {
  id: string;
  created_at: string;
  updated_at: string;
  last_seen: string;
  name: string;
  description: string;
  hostname: string;
  target: NodeTarget;
  services: Service[];

  subnets: NodeSubnetMembership[];
  node_groups: string[];
  discovery_status: string;
}

export type NodeCapability = {
  source: Record<string, any>;
  daemon_id: string;
};
