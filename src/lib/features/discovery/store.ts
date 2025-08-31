import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { createPoller, Poller } from '../../shared/utils/polling';
import type { DaemonDiscoveryUpdate, InitiateDiscoveryRequest } from './types/api';
import { pushError } from '$lib/shared/stores/feedback';

// daemon_id to latest update
export const sessions = writable<Map<string, DaemonDiscoveryUpdate>>(new Map());

// Discovery status poller instance
let discoveryPoller: Poller | null = null;

export function startDiscoveryPolling() {  
  discoveryPoller = createPoller({
    intervalMs: 5000, // 5 seconds
    onPoll: async () => {
      await getActiveDiscoverySessions();
    },
    onError: (pollingError) => {
      pushError(`Failed to poll discovery status: ${pollingError}`);
      stopDiscoveryPolling();
    },
    name: 'DiscoveryPoller'
  });
  
  discoveryPoller.start();
}

export async function stopDiscoveryPolling() {
  if (discoveryPoller) {
    discoveryPoller.stop();
    discoveryPoller = null;
  }
}

export async function initiateDiscovery(data: InitiateDiscoveryRequest) {
  const result = await api.request<DaemonDiscoveryUpdate, Map<string, DaemonDiscoveryUpdate>>(
    '/discovery/initiate',
    sessions,
    (update, currentSessions) => {
      const map = new Map(currentSessions)
      map.set(update.daemon_id, update)
      return map
    },
    { method: 'POST', body: JSON.stringify(data) },
  )

  if (result?.success && !discoveryPoller?.getIsRunning) {
    startDiscoveryPolling();
  }
}

export async function cancelDiscovery(id: string) {
  const result = await api.request<void, Map<string, DaemonDiscoveryUpdate>>(
    `/discovery/cancel/${id}`,
    null,
    null,
    { method: 'POST' },
  )
}

export async function getActiveDiscoverySessions() {
  const result = await api.request<DaemonDiscoveryUpdate[], Map<string, DaemonDiscoveryUpdate>>(
    '/discovery/active',
    sessions,
    (actives, current) => actives.reduce((map,session) => {
      map.set(session.daemon_id, session);
      return map;
    }, new Map<string, DaemonDiscoveryUpdate>()),
    { method: 'GET' },
  )

  if (result?.success && result.data && result.data?.length > 0) {
    !discoveryPoller?.getIsRunning() ? startDiscoveryPolling() : null;
  } else {
    stopDiscoveryPolling();
  }
}