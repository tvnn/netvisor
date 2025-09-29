import { get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import type { Subnet } from './types/base';

export const subnets = writable<Subnet[]>([]);

export async function getSubnets() {
	return await api.request<Subnet[]>('/subnets', subnets, (subnets) => subnets, { method: 'GET' });
}

export async function createSubnet(subnet: Subnet) {
	const result = await api.request<Subnet, Subnet[]>(
		'/subnets',
		subnets,
		(response, currentSubnets) => [...currentSubnets, response],
		{
			method: 'POST',
			body: JSON.stringify(subnet)
		}
	);

	return result;
}

export async function updateSubnet(subnet: Subnet) {
	const result = await api.request<Subnet, Subnet[]>(
		`/subnets/${subnet.id}`,
		subnets,
		(response, currentSubnets) => currentSubnets.map((s) => (s.id === subnet.id ? response : s)),
		{
			method: 'PUT',
			body: JSON.stringify(subnet)
		}
	);

	return result;
}

export async function deleteSubnet(subnetId: string) {
	const result = await api.request<void, Subnet[]>(
		`/subnets/${subnetId}`,
		subnets,
		(_, currentSubnets) => currentSubnets.filter((s) => s.id !== subnetId),
		{ method: 'DELETE' }
	);

	return result;
}

export function createEmptySubnetFormData(): Subnet {
	return {
		id: uuidv4Sentinel,
		created_at: utcTimeZoneSentinel,
		updated_at: utcTimeZoneSentinel,
		name: '',
		cidr: '',
		description: '',
		subnet_type: 'Unknown',
		dns_resolvers: [],
		gateways: [],
		hosts: [],
		reverse_proxies: [],
		source: 'Manual'
	};
}

export function getSubnetFromId(id: string) {
	return get(subnets).find((s) => s.id == id);
}

export function isContainerSubnet(id: string): Subnet | boolean {
	const subnet = get(subnets).find((s) => s.id == id);
	if (subnet) {
		return subnet.cidr == '0.0.0.0/0' && subnet.source == 'System';
	}
	return false;
}
