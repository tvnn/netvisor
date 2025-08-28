import { writable } from 'svelte/store';
import type { Node, NodeApi } from "$lib/components/nodes/types";
import { api } from '../../api/client';
import type { Daemon } from './types';

export const daemons = writable<Daemon[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export const daemonActions = {
  async loadDaemons() {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.getDaemons();
      if (response.success && response.data) {
        daemons.set(response.data.daemons);
      } else {
        error.set(response.error || 'Failed to load daemons');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to load daemons:', err);
    } finally {
      loading.set(false);
    }
  },
};