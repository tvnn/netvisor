import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { NodeGroup } from '$lib/features/node_groups/types/base'

export const nodeGroups = writable<NodeGroup[]>([]);

export async function getNodeGroups() {
  return await api.request<NodeGroup[]>(
    '/groups',
    nodeGroups,
    (groups) => groups,
    { method: 'GET', },
  )
}

export async function createNodeGroup(data: NodeGroup) {
  return await api.request<NodeGroup, NodeGroup[]>(
    '/groups',
    nodeGroups,
    (group, current) => [...current, group],
    { method: 'POST', body: JSON.stringify(data)},
  )
}

export async function updateNodeGroup(data: NodeGroup) {
  return await api.request<NodeGroup, NodeGroup[]>(
    `/groups/${data.id}`,
    nodeGroups,
    (updatedGroup, current) => current.map(g => g.id === data.id ? updatedGroup : g),
    { method: 'POST', body: JSON.stringify(data)},
  )
}

export async function deleteNodeGroup(id: string) {
  const result = await api.request<void, NodeGroup[]>(
    `/groups/${id}`,
    nodeGroups,
    (_, current) => current.filter(g => g.id !== id),
    { method: 'DELETE'},
  )
}

export function createEmptyFormData(): NodeGroup {
  return {
    name: '',
    description: '',
    node_sequence: [] as string[],
    auto_diagnostic_enabled: true,
    id: '',
    created_at: '',
    updated_at: ''
  };
}