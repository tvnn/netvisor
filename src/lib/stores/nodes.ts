import { writable } from 'svelte/store';
import type { Node, NodeApi, NodeFormData, NodeType } from "$lib/types/nodes";
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

  async getCapabilityCompatibility(nodeType: NodeType) {
    try {
      const response = await api.getCapabilityCompatibility(nodeType);
      if (response.success && response.data) {
        return response.data;
      } else {
        return false;
      }
    } catch (err) {
      console.error('Failed to get capability recommendations')
      return false;
    }
  },

  clearError() {
    error.set(null);
  }
};