import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { NodeGroup } from '$lib/features/node_groups/types/base'

export const nodeGroups = writable<NodeGroup[]>([]);
export const loading = writable(false);
export const error = writable<string | null>(null);

export async function getNodeGroups() {
  return await api.request<NodeGroup[]>(
    '/groups',
    nodeGroups,
    (groups) => groups,
    error,
    loading,
    { method: 'GET', },
    "Failed to get node groups"
  )
}

export async function createNodeGroup(data: NodeGroup) {
  return await api.request<NodeGroup, NodeGroup[]>(
    '/groups',
    nodeGroups,
    (group, current) => [...current, group],
    error,
    loading,
    { method: 'POST', body: JSON.stringify(data)},
    "Failed to create node group"
  )
}

export async function updateNodeGroup(data: NodeGroup) {
  return await api.request<NodeGroup, NodeGroup[]>(
    `/groups/${data.id}`,
    nodeGroups,
    (updatedGroup, current) => current.map(g => g.id === data.id ? updatedGroup : g),
    error,
    loading,
    { method: 'POST', body: JSON.stringify(data)},
    "Failed to update node group"
  )
}

export async function deleteNodeGroup(id: string) {
  const result = await api.request<void, NodeGroup[]>(
    `/groups/${id}`,
    nodeGroups,
    (_, current) => current.filter(g => g.id !== id),
    error,
    loading,
    { method: 'DELETE'},
    "Failed to delete node group"
  )
}

export function clearError() {
  error.set(null)
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