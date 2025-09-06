import { get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Daemon } from "./types/base";
import type { DaemonDiscoveryUpdate } from '../discovery/types/api';
import { nodes } from '../nodes/store';
import type { Node } from '../nodes/types/base';

export const daemons = writable<Daemon[]>([]);

export async function getDaemons() {
  return await api.request<Daemon[]>(
    '/daemons',
    daemons,
    (daemons) => daemons,
    { method: 'GET' },
  )
}

export function getDaemonDiscoveryState(daemon_id: string | null, sessionsMap: Map<string, DaemonDiscoveryUpdate>): DaemonDiscoveryUpdate | null {
  if (!daemon_id) return null;
  return sessionsMap.get(daemon_id) || null;
}

export function getDaemonNode(daemon_id: string): Node | null {
  const daemon = get(daemons).find(d => d.id === daemon_id);
  if (!daemon) return null;

  const node = get(nodes).find(n => n.id === daemon.node_id) || null;
  return node ? node : null;
}

export function getNodeDaemon(node_id: string): Daemon | null {
  const daemon = get(daemons).find(d => d.node_id === node_id);
  return daemon ? daemon :null
} 
