import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { HostGroup } from '$lib/features/host_groups/types/base'
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';

export const hostGroups = writable<HostGroup[]>([]);

export async function getHostGroups() {
  return await api.request<HostGroup[]>(
    '/host_groups',
    hostGroups,
    (groups) => groups,
    { method: 'GET', },
  )
}

export async function createHostGroup(data: HostGroup) {
  return await api.request<HostGroup, HostGroup[]>(
    '/host_groups',
    hostGroups,
    (group, current) => [...current, group],
    { method: 'POST', body: JSON.stringify(data)},
  )
}

export async function updateHostGroup(data: HostGroup) {
  return await api.request<HostGroup, HostGroup[]>(
    `/host_groups/${data.id}`,
    hostGroups,
    (updatedGroup, current) => current.map(g => g.id === data.id ? updatedGroup : g),
    { method: 'PUT', body: JSON.stringify(data)},
  )
}

export async function deleteHostGroup(id: string) {
  const result = await api.request<void, HostGroup[]>(
    `/host_groups/${id}`,
    hostGroups,
    (_, current) => current.filter(g => g.id !== id),
    { method: 'DELETE'},
  )
}

export function createEmptyHostGroupFormData(): HostGroup {
  return {
    id: uuidv4Sentinel,
    name: '',
    description: '',
    hosts: [],
    created_at: utcTimeZoneSentinel,
    updated_at: utcTimeZoneSentinel
  };
}