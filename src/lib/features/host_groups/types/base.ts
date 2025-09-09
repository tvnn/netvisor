export interface HostGroup {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  description: string;
  hosts: string[];
}

export interface HostGroupListResponse {
  groups: HostGroup[];
  total: number;
}

export interface HostGroupResponse {
  group: HostGroup;
}
