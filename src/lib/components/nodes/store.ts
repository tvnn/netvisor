import { writable } from 'svelte/store';
import type { Node, NodeApi } from "$lib/components/nodes/types";
import { api } from '../../api/client';
import { createPoller, type Poller } from '../../utils/polling';

export const nodes = writable<Node[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

// Create node polling instance
let nodePoller: Poller | null = null;

export const nodeActions = {
  startNodePolling() {
    // Stop any existing polling
    this.stopNodePolling();
    
    // Create and start new poller
    nodePoller = createPoller({
      intervalMs: 30000, // 30 seconds
      onPoll: async () => {
        await this.loadNodes();
      },
      onError: (error) => {
        console.error('Failed to poll nodes:', error);
      },
      name: 'NodePoller'
    });
    
    nodePoller.start();
  },

  stopNodePolling() {
    if (nodePoller) {
      nodePoller.stop();
      nodePoller = null;
    }
  },

  getNodePollingStatus(): boolean {
    return nodePoller?.getIsRunning() ?? false;
  },

  async loadNodes() {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.getNodes();
      if (response.success && response.data) {
        nodes.set(response.data.nodes);
      } else {
        error.set(response.error || 'Failed to load nodes');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to load nodes:', err);
    } finally {
      loading.set(false);
    }
  },

  async createNode(data: NodeApi): Promise<Node | null> {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.createNode(data);
      if (response.success && response.data) {
        const newNode = response.data.node;
        nodes.update(current => [...current, newNode]);
        return newNode;
      } else {
        error.set(response.error || 'Failed to create node');
        return null;
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to create node:', err);
      return null;
    } finally {
      loading.set(false);
    }
  },

  async updateNode(id: string, data: NodeApi): Promise<Node | null> {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.updateNode(id, data);
      if (response.success && response.data) {
        const updatedNode = response.data.node;
        nodes.update(current => 
          current.map(node => node.id === id ? updatedNode : node)
        );
        return updatedNode;
      } else {
        error.set(response.error || 'Failed to update node');
        return null;
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to update node:', err);
      return null;
    } finally {
      loading.set(false);
    }
  },

  async deleteNode(id: string): Promise<boolean> {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.deleteNode(id);
      if (response.success) {
        nodes.update(current => current.filter(node => node.id !== id));
        return true;
      } else {
        error.set(response.error || 'Failed to delete node');
        return false;
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to delete node:', err);
      return false;
    } finally {
      loading.set(false);
    }
  },

  async refreshNode(nodeId: string) {
    try {
      const response = await api.getNode(nodeId);
      if (response.success && response.data) {
        const updatedNode = response.data.node;
        nodes.update(current => 
          current.map(node => node.id === nodeId ? updatedNode : node)
        );
      }
    } catch (err) {
      console.error('Failed to refresh node:', err);
    }
  },

  clearError() {
    error.set(null);
  }
};

// Export cleanup function for app-level cleanup
export function cleanupNodePolling() {
  nodeActions.stopNodePolling();
}