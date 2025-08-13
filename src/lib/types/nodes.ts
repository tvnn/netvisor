import { writable } from 'svelte/store';
import type { Node } from '../types';
import { api } from '../api-client';

export const nodes = writable<Node[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export const nodeActions = {
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

  async createNode(data: any) {
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

  async updateNode(id: string, data: any) {
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

  async deleteNode(id: string) {
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

  async assignTest(data: any) {
    try {
      const response = await api.assignTest(data);
      if (response.success) {
        // Refresh the node to get updated assigned_tests
        await this.refreshNode(data.node_id);
        return true;
      } else {
        error.set(response.error || 'Failed to assign test');
        return false;
      }
    } catch (err) {
      error.set('Network error');
      return false;
    }
  },

  async setMonitoring(nodeId: string, enabled: boolean) {
    try {
      const response = await api.setMonitoring(nodeId, enabled);
      if (response.success) {
        nodes.update(current => 
          current.map(node => 
            node.id === nodeId 
              ? { ...node, monitoring_enabled: enabled }
              : node
          )
        );
        return true;
      } else {
        error.set(response.error || 'Failed to set monitoring');
        return false;
      }
    } catch (err) {
      error.set('Network error');
      return false;
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