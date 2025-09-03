
import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';

export const subnets = writable<Subnet[]>([]);

export async function getSubnets() {
  return await api.request<Subnet[]>(
    '/subnets',
    subnets,
    (subnets) => subnets,
    { method: 'GET' },
  )
}