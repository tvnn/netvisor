export interface NodeGroup {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  description: string;
  node_sequence: string[];
  auto_diagnostic_enabled: boolean;
}

export interface NodeGroupListResponse {
  groups: NodeGroup[];
  total: number;
}

export interface NodeGroupResponse {
  group: NodeGroup;
}
