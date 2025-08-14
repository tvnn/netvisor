export interface NodeGroupFormData {
  name: string;
  description: string;
  node_sequence: string[];
  auto_diagnostic_enabled: boolean;
}

export interface NodeGroupApi {
  name: string;
  description: string;
  node_sequence: string[];
  auto_diagnostic_enabled: boolean;
}

export interface NodeGroup extends NodeGroupApi {
  id: string;
  created_at: string;
  updated_at: string;
}
