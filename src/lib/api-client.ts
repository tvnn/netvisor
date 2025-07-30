// API client for HTTP backend (replaces tauri-commands.ts)
import type { NetworkNode, Test, DiagnosticResults, CheckResult, CheckConfig } from './types';

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

class ApiClient {
  private baseUrl: string;

  constructor() {
    // Auto-detect API endpoint
    if (typeof window !== 'undefined') {
      // In browser - use current origin with /api prefix
      this.baseUrl = `${window.location.origin}/api`;
    } else {
      // SSR/build time - use default
      this.baseUrl = 'http://localhost:3000/api';
    }
  }

  private async request<T>(
    endpoint: string, 
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    
    const response = await fetch(url, {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      ...options,
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const result: ApiResponse<T> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'API request failed');
    }

    return result.data!;
  }

  // Health check
  async health(): Promise<Record<string, string>> {
    return this.request('/health');
  }

  // Node operations
  async getNodes(): Promise<NetworkNode[]> {
    return this.request('/nodes');
  }

  async saveNode(node: Omit<NetworkNode, 'id' | 'created_at' | 'updated_at'>): Promise<NetworkNode> {
    return this.request('/nodes', {
      method: 'POST',
      body: JSON.stringify(node),
    });
  }

  async updateNode(id: string, node: Omit<NetworkNode, 'id' | 'created_at' | 'updated_at'>): Promise<NetworkNode> {
    return this.request(`/nodes/${id}`, {
      method: 'PUT',
      body: JSON.stringify(node),
    });
  }

  async deleteNode(id: string): Promise<void> {
    return this.request(`/nodes/${id}`, {
      method: 'DELETE',
    });
  }

  // Test operations
  async getTests(): Promise<Test[]> {
    return this.request('/tests');
  }

  async saveTest(test: Omit<Test, 'id' | 'created_at' | 'updated_at'>): Promise<Test> {
    return this.request('/tests', {
      method: 'POST',
      body: JSON.stringify({
        name: test.name,
        description: test.description,
        version: test.version,
        layers: test.layers,
      }),
    });
  }

  async updateTest(id: string, test: Omit<Test, 'id' | 'created_at' | 'updated_at'>): Promise<Test> {
    return this.request(`/tests/${id}`, {
      method: 'PUT',
      body: JSON.stringify({
        name: test.name,
        description: test.description,
        version: test.version,
        layers: test.layers,
      }),
    });
  }

  async deleteTest(id: string): Promise<void> {
    return this.request(`/tests/${id}`, {
      method: 'DELETE',
    });
  }

  // Diagnostics
  async runDiagnostics(testId: string): Promise<DiagnosticResults> {
    return this.request(`/diagnostics/run/${testId}`, {
      method: 'POST',
    });
  }

  async getDiagnosticResults(testId?: string, limit?: number): Promise<DiagnosticResults[]> {
    const params = new URLSearchParams();
    if (testId) params.set('test_id', testId);
    if (limit) params.set('limit', limit.toString());
    
    const query = params.toString();
    return this.request(`/diagnostics/results${query ? `?${query}` : ''}`);
  }

  // Individual checks
  async executeCheck(checkType: string, config: CheckConfig): Promise<CheckResult> {
    return this.request(`/checks/${checkType}`, {
      method: 'POST',
      body: JSON.stringify({ config }),
    });
  }

  // Configuration
  async getConfig(): Promise<any> {
    return this.request('/config');
  }
}

// Export singleton instance
export const apiClient = new ApiClient();

// Export commands interface that matches the existing tauri-commands structure
export const commands = {
  // Health
  health: () => apiClient.health(),

  // Nodes
  getNodes: () => apiClient.getNodes(),
  saveNode: (node: Omit<NetworkNode, 'id' | 'created_at' | 'updated_at'>) => apiClient.saveNode(node),
  updateNode: (id: string, node: Omit<NetworkNode, 'id' | 'created_at' | 'updated_at'>) => apiClient.updateNode(id, node),
  deleteNode: (id: string) => apiClient.deleteNode(id),

  // Tests  
  getTests: () => apiClient.getTests(),
  saveTest: (test: Omit<Test, 'id' | 'created_at' | 'updated_at'>) => apiClient.saveTest(test),
  updateTest: (id: string, test: Omit<Test, 'id' | 'created_at' | 'updated_at'>) => apiClient.updateTest(id, test),
  deleteTest: (id: string) => apiClient.deleteTest(id),

  // Diagnostics
  runDiagnostics: (test: Test) => apiClient.runDiagnostics(test.id),
  getDiagnosticResults: (testId?: string, limit?: number) => apiClient.getDiagnosticResults(testId, limit),

  // Checks - maintaining the same interface as the original tauri commands
  executeCheck: (checkType: string, config: CheckConfig) => apiClient.executeCheck(checkType, config),
};