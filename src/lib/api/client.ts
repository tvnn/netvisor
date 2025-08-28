import type { DaemonListResponse } from "$lib/components/daemons/types";
import type { DiagnosticExecutionApi, DiagnosticExecutionResponse } from "$lib/components/diagnostics/types";
import type { CancelDiscoveryResponse, DiscoverySessionRequest, DaemonDiscoveryUpdate, InitiateDiscoveryRequest, InitiateDiscoveryResponse } from "$lib/components/discovery/types";
import type { NodeGroupApi, NodeGroupListResponse, NodeGroupResponse } from "../components/node_groups/types";
import type { NodeApi, NodeListResponse, NodeResponse } from "../components/nodes/types";

// API client for NetVisor backend
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

  async executeNodeGroupDiagnostics(id: string, data: DiagnosticExecutionApi) {
    return this.request<void>(`/diagnostics/execute/${id}`, {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async getDiagnostics() {
    return this.request<DiagnosticExecutionResponse>('/diagnostics', {
      method:  'GET'
    })
  }

  async getDiagnostic(id: string) {
    return this.request<DiagnosticExecutionResponse>(`/diagnostics/${id}`, {
      method:  'GET'
    })
  }

  async deleteDiagnostic(id: string) {
    return this.request<DiagnosticExecutionResponse>(`/diagnostics/${id}`, {
      method:  'DELETE'
    })
  }

  async getDaemons() {
    return this.request<DaemonListResponse>('/daemons', {
      method:  'GET'
    })
  }

  async initiateDiscovery(data: InitiateDiscoveryRequest) {
    return this.request<InitiateDiscoveryResponse>('/discovery/initiate', {
      method:  'POST',
      body: JSON.stringify(data)
    })
  }

  async cancelDiscovery(session_id: string) {
    return this.request<CancelDiscoveryResponse>(`/discovery/${session_id}/cancel`, {
      method:  'POST',
    })
  }

  async discoveryStatus(session_id: string) {
    return this.request<DaemonDiscoveryUpdate>(`/discovery/${session_id}/status`, {
      method:  'GET',
    })
  }


}

export const api = new ApiClient();