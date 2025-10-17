import { get, writable } from 'svelte/store';
import { api } from '../../shared/utils/api';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import type { Subnet } from './types/base';
import { services } from '../services/store';
import { getInterfaceFromId, hosts } from '../hosts/store';
import { serviceDefinitions } from '$lib/shared/stores/metadata';
import type { Service } from '../services/types/base';

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
		source: {
			type: 'Manual'
		}
	};
}

export function getSubnetFromId(id: string) {
	return get(subnets).find((s) => s.id == id);
}

export function isContainerSubnet(id: string): Subnet | boolean {
	const subnet = get(subnets).find((s) => s.id == id);
	if (subnet) {
		return subnet.cidr == '0.0.0.0/0' && subnet.source.type == 'System';
	}
	return false;
}

export function getSubnetServices(
	subnet: Subnet,
	infra_type_flag?: 'is_dns_resolver' | 'is_gateway' | 'is_reverse_proxy'
): Service[] {
	const host_ids = get(hosts)
		.filter((h) => h.interfaces.some((i) => i.subnet_id == subnet.id))
		.map((h) => h.id);
	const interface_ids = get(hosts)
		.flatMap((h) => h.interfaces)
		.filter((i) => i.subnet_id == subnet.id)
		.map((i) => i.id);

	const subnetServices = get(services).filter((s) => {
		return s.bindings.some(
			(b) =>
				(b.interface_id && interface_ids.includes(b.interface_id)) ||
				(host_ids.includes(s.host_id) && b.interface_id == null)
		);
	});

	return subnetServices.filter((s) => {
		if (!infra_type_flag) return subnetServices;
		else {
			return (
				serviceDefinitions.getMetadata(s.service_definition)[infra_type_flag] &&
				s.bindings.some(
					(b) => b.interface_id && getInterfaceFromId(b.interface_id)?.subnet_id == subnet.id
				)
			);
		}
	});
}
