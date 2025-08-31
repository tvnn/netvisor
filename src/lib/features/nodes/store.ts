import { writable } from 'svelte/store';
import type { Node } from "./types/base";
import { api } from '../../shared/utils/api';
import { createPoller, type Poller } from '../../shared/utils/polling';
import type { NodeTarget } from './types/targets';
import { pushError } from '$lib/shared/stores/feedback';

export const nodes = writable<Node[]>([]);
export const polling = writable(false);

// Create node polling instance
let nodePoller: Poller | null = null;

export function startNodePolling() {  
  nodePoller = createPoller({
    intervalMs: 5000, // 5 seconds
    onPoll: async () => {
      await getNodes();
    },
    onError: (pollingError) => {
      pushError(`Failed to poll node status: ${pollingError}`);
      stopNodePolling();
    },
    name: 'NodePoller'
  });
  
  nodePoller.start();
}

export async function stopNodePolling() {
  if (nodePoller) {
    nodePoller.stop();
    nodePoller = null;
  }
}

export async function getNodes() {
  return await api.request<Node[]>(
    '/nodes',
    nodes,
    (nodes) => nodes,
    { method: 'GET', },
  )
}

export async function createNode(data: Node) {
  return await api.request<Node, Node[]>(
    '/nodes',
    nodes,
    (node, current) => [...current, node],
    { method: 'POST', body: JSON.stringify(data)},
  )
}

export async function updateNode(data: Node) {
  return await api.request<Node, Node[]>(
    `/nodes/${data.id}`,
    nodes,
    (updatedNode, current) => current.map(n => n.id === data.id ? updatedNode : n),
    { method: 'POST', body: JSON.stringify(data)},
  )
}

export async function deleteNode(id: string) {
  return await api.request<void, Node[]>(
    `/nodes/${id}`,
    nodes,
    (_, current) => current.filter(g => g.id !== id),
    { method: 'DELETE'},
  )
}


export function createEmptyNodeFormData(): Node {
  return {
    id: '',
    created_at: '',
    updated_at: '',
    name: '',
    status: 'Unknown',
    description: '',
    hostname: '',
    target: {
      type: 'IpAddress',
      config: {
        ip: '',
      },
    },
    node_type: 'UnknownDevice',
    capabilities: [],
    mac_address: '',
    subnets: [],
    monitoring_interval: 10,
    last_seen: '',
    node_groups: [],
    discovery_status: ''
  };
}

export function getNodeTargetString(target: NodeTarget): string {
  switch (target.type) {
    case 'IpAddress':
      return target.config.ip;
    case 'Hostname':
      return target.config.hostname;
    default:
      return 'Unknown target';
  }
}
