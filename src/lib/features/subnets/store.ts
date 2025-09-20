import { writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';

export const subnets = writable<Subnet[]>([]);

export async function getSubnets() {
  return await api.request<Subnet[]>(
    '/subnets',
    subnets,
    (subnets) => subnets,
    { method: 'GET' },
    true
  )
}

export async function createSubnet(subnet: Subnet) {
  return await api.request<Subnet, Subnet[]>(
    '/subnets',
    subnets,
    (response, currentSubnets) => [...currentSubnets, response],
    {
      method: 'POST',
      body: JSON.stringify(subnet)
    }
  );
}

export async function updateSubnet(subnet: Subnet) {
  return await api.request<Subnet, Subnet[]>(
    `/subnets/${subnet.id}`,
    subnets,
    (response, currentSubnets) => 
      currentSubnets.map(s => s.id === subnet.id ? response : s),
    {
      method: 'PUT',
      body: JSON.stringify(subnet)
    }
  );
}

export async function deleteSubnet(subnetId: string) {
  return await api.request<void, Subnet[]>(
    `/subnets/${subnetId}`,
    subnets,
    (_, currentSubnets) => currentSubnets.filter(s => s.id !== subnetId),
    { method: 'DELETE' }
  );
}

export function createEmptySubnetFormData(): Subnet {
  return {
    id: uuidv4Sentinel,
    created_at: utcTimeZoneSentinel,
    updated_at: utcTimeZoneSentinel,
    name: '',
    cidr: '',
    description: '',
    subnet_type: 'Lan',
    dns_resolvers: [],
    gateways: [],
    hosts: [],
    reverse_proxies: [],
    source: 'Manual'
  };
}