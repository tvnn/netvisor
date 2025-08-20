import type { NodeGroupApi, NodeGroup } from "../types/node-groups";
import type { NodeApi, Node } from "../types/nodes";

// API client for NetFrog backend
const API_BASE = 'http://localhost:3000/api';

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

interface NodeListResponse {
  nodes: Node[];
  total: number;
}

interface NodeResponse {
  node: Node;
}

interface NodeGroupListResponse {
  groups: NodeGroup[];
  total: number;
}

interface NodeGroupResponse {
  group: NodeGroup;
}

interface CapabilityCompatibilityResponse {
  recommendations: string[];
  warnings: string[];
}

class ApiClient {
  private async request<T>(endpoint: string, options: RequestInit = {}): Promise<ApiResponse<T>> {
    const url = `${API_BASE}${endpoint}`;
    
    try {
      const response = await fetch(url, {
        headers: {
          'Content-Type': 'application/json',
          ...options.headers,
        },
        ...options,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({ 
          success: false, 
          error: `HTTP ${response.status}: ${response.statusText}` 
        }));
        return { success: false, error: errorData.error || `HTTP ${response.status}` };
      }

      return await response.json();
    } catch (error) {
      return { 
        success: false, 
        error: error instanceof Error ? error.message : 'Network error' 
      };
    }
  }

  // Health check
  async getHealth() {
    return this.request<string>('/health');
  }

  // Node operations
  async getNodes() {
    return this.request<NodeListResponse>('/nodes');
  }

  async getNode(id: string) {
    return this.request<NodeResponse>(`/nodes/${id}`);
  }

  async createNode(data: NodeApi) {
    return this.request<NodeResponse>('/nodes', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateNode(id: string, data: Partial<NodeApi>) {
    return this.request<NodeResponse>(`/nodes/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteNode(id: string) {
    return this.request<void>(`/nodes/${id}`, {
      method: 'DELETE',
    });
  }

  async getCapabilityCompatibility(nodeType: string) {
    return this.request<CapabilityCompatibilityResponse>(`/nodes/capability-compatibility?node_type=${nodeType.toString()}`);
  }

  // Node group operations
  async getNodeGroups() {
    return this.request<NodeGroupListResponse>('/groups');
  }

  async createNodeGroup(data: NodeGroupApi) {
    return this.request<NodeGroupResponse>('/groups', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateNodeGroup(id: string, data: Partial<NodeGroupApi>) {
    return this.request<NodeGroupResponse>(`/groups/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteNodeGroup(id: string) {
    return this.request<void>(`/groups/${id}`, {
      method: 'DELETE',
    });
  }
}

export const api = new ApiClient();