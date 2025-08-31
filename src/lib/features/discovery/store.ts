import { writable, get } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { daemons } from '../daemons/store';
import { nodes } from '../nodes/store';
import { createPoller, Poller } from '../../shared/utils/polling';
import type { DaemonDiscoveryUpdate, InitiateDiscoveryRequest } from './types/api';
import type { Node } from "../nodes/types/base";

// daemon_id to latest update
export const sessions = writable<Map<string, DaemonDiscoveryUpdate>>(new Map());
export const loading = writable(false);
export const error = writable<string | null>(null);

// Discovery status poller instance
let discoveryPoller: Poller | null = null;

export function startDiscoveryPolling() {  
  discoveryPoller = createPoller({
    intervalMs: 5000, // 5 seconds
    onPoll: async () => {
      await getActiveDiscoverySessions();
    },
    onError: (pollingError) => {
      error.set('Network error while polling');
      console.error('Failed to poll discovery status:', pollingError);
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
    (update, sessions) => sessions.set(data.daemon_id, update),
    error,
    loading,
    { method: 'POST', body: JSON.stringify(data) },
    "Failed to initiate discovery"
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
    error,
    loading,
    { method: 'POST' },
    "Failed to cancel discovery"
  )
}

export async function getActiveDiscoverySessions() {
  const result = await api.request<DaemonDiscoveryUpdate[], Map<string, DaemonDiscoveryUpdate>>(
    '/discovery/active',
    sessions,
    (actives, current) => actives.reduce((map,session) => {
      map.set(session.daemon_id, session);
      return map;
    }, {} as Map<string, DaemonDiscoveryUpdate>),
    error,
    loading,
    { method: 'GET' },
    "Failed to get active discovery sessions"
  )

  if (result?.success && result.data && result.data?.length > 0) {
    !discoveryPoller?.getIsRunning() ? startDiscoveryPolling() : null;
  } else {
    stopDiscoveryPolling();
  }
}

export function getDaemonDiscoveryState(daemon_id: string | null): DaemonDiscoveryUpdate | null {
  return get(sessions).values().find(session => session.daemon_id == daemon_id) || null
}

// Helper function to get node name for a daemon
export function getDaemonNode(daemon_id: string): Node | null {
  const daemon = get(daemons).find(d => d.id === daemon_id);
  if (!daemon) return null;
  
  const node = get(nodes).find(n => n.id === daemon.node_id) || null;
  return node ? node : null;
}