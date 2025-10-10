import { get, writable } from 'svelte/store';
import type {
	AllInterfaces,
	Host,
	HostWithServicesRequest,
	Interface,
	Port,
	ServiceBinding
} from './types/base';
import { api } from '../../shared/utils/api';
import { pushSuccess } from '$lib/shared/stores/feedback';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import { isContainerSubnet } from '../subnets/store';
import { getBindingFromId, getLayerBindingDisplayName } from '../services/store';

export const hosts = writable<Host[]>([]);
export const polling = writable(false);

export async function getHosts() {
	return await api.request<Host[]>('/hosts', hosts, (hosts) => hosts, { method: 'GET' });
}

export async function createHost(data: HostWithServicesRequest) {
	const result = await api.request<Host, Host[]>(
		'/hosts',
		hosts,
		(host, current) => [...current, host],
		{ method: 'POST', body: JSON.stringify(data) }
	);

	return result;
}

export async function updateHost(data: HostWithServicesRequest) {
	return await api.request<Host, Host[]>(
		`/hosts`,
		hosts,
		(updatedHost, current) => {
			return current.map((n) => (n.id === data.host.id ? updatedHost : n));
		},
		{ method: 'PUT', body: JSON.stringify(data) }
	);
}

export async function deleteHost(id: string) {
	return await api.request<void, Host[]>(
		`/hosts/${id}`,
		hosts,
		(_, current) => current.filter((g) => g.id !== id),
		{ method: 'DELETE' }
	);
}

export async function consolidateHosts(destination_host_id: string, other_host_id: string) {
	const other_host_name = getHostFromId(other_host_id)?.name;

	return await api.request<Host, Host[]>(
		`/hosts/${destination_host_id}/consolidate/${other_host_id}`,
		hosts,
		(updatedHost, current) => {
			current = current.filter((g) => g.id !== other_host_id);
			pushSuccess(`Consolidated host "${other_host_name}" into host "${updatedHost.name}"`);
			return current.map((h) => (h.id == destination_host_id ? updatedHost : h));
		},
		{ method: 'PUT' }
	);
}

export function createEmptyHostFormData(): Host {
	return {
		id: uuidv4Sentinel,
		created_at: utcTimeZoneSentinel,
		updated_at: utcTimeZoneSentinel,
		name: '',
		description: '',
		hostname: '',
		target: {
			type: 'None'
		},
		services: [],
		interfaces: [],
		ports: [],
		source: 'Manual'
	};
}

export function getHostTargetString(host: Host): string | null {
	switch (host.target.type) {
		case 'ServiceBinding': {
			const binding = getBindingFromId(host.target.config.binding_id);
			if (binding) {
				return getLayerBindingDisplayName(binding);
			}
			return 'Unknown Binding';
		}
		case 'None': {
			return 'None';
		}
		case 'Hostname': {
			return host.hostname;
		}
	}
}

export function formatInterface(i: Interface | AllInterfaces): string {
	if (i.id == null) return i.name;
	return isContainerSubnet(i.subnet_id) ? i.name : (i.name ? i.name + ': ' : '') + i.ip_address;
}

export function getHostFromId(id: string): Host | undefined {
	return get(hosts).find((h) => h.id == id);
}

export function getInterfaceFromId(id: string): Interface | undefined {
	for (const host of get(hosts)) {
		const iface = host.interfaces.find((i) => i.id == id);
		if (iface != undefined) {
			return iface;
		}
	}
	return undefined;
}

export function getPortFromId(id: string): Port | undefined {
	for (const host of get(hosts)) {
		const port = host.ports.find((p) => p.id == id);
		if (port != undefined) {
			return port;
		}
	}
	return undefined;
}

export function serviceBindingToId(binding: ServiceBinding): string {
	return JSON.stringify(Object.fromEntries(Object.entries(binding).sort()));
}

export function serviceBindingIdToObj(binding_string: string): ServiceBinding | null {
	const parsed = JSON.parse(binding_string);
	if (parsed?.binding_id && parsed?.service_id) {
		return parsed as ServiceBinding;
	} else {
		return null;
	}
}
