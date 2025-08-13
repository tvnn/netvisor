// API client for NetFrog backend
const API_BASE = 'http://localhost:3000/api';

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
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
    return this.request<{ nodes: any[]; total: number }>('/nodes');
  }

  async getNode(id: string) {
    return this.request<{ node: any }>(`/nodes/${id}`);
  }

  async createNode(data: any) {
    return this.request<{ node: any }>('/nodes', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateNode(id: string, data: any) {
    return this.request<{ node: any }>(`/nodes/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteNode(id: string) {
    return this.request<void>(`/nodes/${id}`, {
      method: 'DELETE',
    });
  }

  async assignTest(data: any) {
    return this.request<void>(`/nodes/${data.node_id}/assign-test`, {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async setMonitoring(nodeId: string, enabled: boolean) {
    return this.request<void>(`/nodes/${nodeId}/monitoring`, {
      method: 'PUT',
      body: JSON.stringify({ node_id: nodeId, enabled }),
    });
  }

  // Node group operations
  async getNodeGroups() {
    return this.request<{ groups: any[]; total: number }>('/groups');
  }

  async createNodeGroup(data: any) {
    return this.request<{ group: any }>('/groups', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateNodeGroup(id: string, data: any) {
    return this.request<{ group: any }>(`/groups/${id}`, {
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