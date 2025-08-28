import { writable } from 'svelte/store';
import type { Node, NodeApi } from "$lib/components/nodes/types";
import { api } from '../../api/client';

export const session_id = writable<string | null>(null);
export const discoveryStatus = writable<Record<string, any>>({});
export const loading = writable(false);
export const error = writable<string | null>(null);

export const discoveryActions = {
  async initiateDiscovery(daemon_id: string) {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.initiateDiscovery({daemon_id});
      if (response.success && response.data) {
        session_id.set(response.data.session_id);
      } else {
        error.set(response.error || 'Failed to initiate discovery');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to initiate discovery:', err);
    } finally {
      loading.set(false);
    }
  },

  async cancelDiscovery(id: string) {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.cancelDiscovery(id);
      if (response.success) {
        session_id.set(null);
      } else {
        error.set(response.error || 'Failed to cancel discovery');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to cancel discovery:', err);
    } finally {
      loading.set(false);
    }
  },

  async discoveryStatus(id: string) {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.discoveryStatus(id);
      if (response.success && response.data) {
        discoveryStatus.set(response.data.session);
      } else {
        error.set(response.error || 'Failed to get discovery status');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to get discovery status:', err);
    } finally {
      loading.set(false);
    }
  },

  clearError() {
    error.set(null);
  }
};