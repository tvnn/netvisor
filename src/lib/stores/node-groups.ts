import { writable } from 'svelte/store';
import { api } from '../api-client';
import type { NodeGroup } from '$lib/types/node-groups'

export const nodeGroups = writable<NodeGroup[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export const nodeGroupActions = {
  async loadGroups() {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.getNodeGroups();
      if (response.success && response.data) {
        nodeGroups.set(response.data.groups);
      } else {
        error.set(response.error || 'Failed to load node groups');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to load node groups:', err);
    } finally {
      loading.set(false);
    }
  },

  async createGroup(data: any): Promise<NodeGroup | null> {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.createNodeGroup(data);
      if (response.success && response.data) {
        const newGroup = response.data.group;
        nodeGroups.update(current => [...current, newGroup]);
        return newGroup;
      } else {
        error.set(response.error || 'Failed to create node group');
        return null;
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to create node group:', err);
      return null;
    } finally {
      loading.set(false);
    }
  },

  async updateGroup(id: string, data: any): Promise<NodeGroup | null> {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.updateNodeGroup(id, data);
      if (response.success && response.data) {
        const updatedGroup = response.data.group;
        nodeGroups.update(current => 
          current.map(group => group.id === id ? updatedGroup : group)
        );
        return updatedGroup;
      } else {
        error.set(response.error || 'Failed to update node group');
        return null;
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to update node group:', err);
      return null;
    } finally {
      loading.set(false);
    }
  },

  async deleteGroup(id: string): Promise<boolean> {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.deleteNodeGroup(id);
      if (response.success) {
        nodeGroups.update(current => current.filter(group => group.id !== id));
        return true;
      } else {
        error.set(response.error || 'Failed to delete node group');
        return false;
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to delete node group:', err);
      return false;
    } finally {
      loading.set(false);
    }
  },

  clearError() {
    error.set(null);
  }
};