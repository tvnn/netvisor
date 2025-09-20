import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';

export const topology = writable<TopologyResponse>();

export async function getTopology() {
  return await api.request<TopologyResponse>(
    '/topology',
    topology,
    (topology) => topology,
    { method: 'GET' },
    true
  )
}