import { writable, get, derived } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Binding, Service } from './types/base';
import { formatPort, utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import { formatInterface, getInterfaceFromId, getPortFromId, hosts } from '../hosts/store';
import { ALL_INTERFACES, type Host } from '../hosts/types/base';
import { groups } from '../groups/store';

export const services = writable<Service[]>([]);

// Get all services
export async function getServices() {
	return await api.request<Service[]>('/services', services, (services) => services, {
		method: 'GET'
	});
}

// Helper functions for working with services and the MetadataRegistry
export function createDefaultService(
	serviceType: string,
	host_id: string,
	serviceName?: string
): Service {
	return {
		id: uuidv4Sentinel,
		created_at: utcTimeZoneSentinel,
		updated_at: utcTimeZoneSentinel,
		host_id,
		service_definition: serviceType,
		name: serviceName || serviceType,
		bindings: [],
		virtualization: null
	};
}

export function formatServiceAsHost(service_id: string): string {
	const service = getServiceById(service_id);
	const host = getServiceHost(service_id);

	if (host && service) {
		if (host.name == service.name) return host.name;
		else return host.name + ': ' + service.name;
	} else if (host && !service) return host.name + ': ' + 'Unknown Service';
	else if (!host && service) return service.name + '(Unknown Host)';
	else return 'Unknown Service';
}

export function getServiceById(service_id: string): Service | null {
	return get(services).find((s) => s.id == service_id) || null;
}

export function getServiceHost(service_id: string): Host | null {
	const service = get(services).find((s) => s.id == service_id);
	if (service) {
		const host = get(hosts).find((h) => h.id == service.host_id) || null;
		return host;
	}
	return null;
}

export function getServicesForHost(host_id: string): Service[] {
	const host = get(hosts).find((h) => h.id == host_id);

	if (host) {
		const serviceMap = new Map(get(services).map((s) => [s.id, s]));
		return host.services.map((id) => serviceMap.get(id)).filter((s) => s != undefined);
	} else {
		return [];
	}
}

export function getServicesForHostReactive(host_id: string) {
	return derived([hosts, services], ([$hosts, $services]) => {
		const host = $hosts.find((h) => h.id === host_id);

		if (host) {
			const serviceMap = new Map($services.map((s) => [s.id, s]));
			return host.services.map((id) => serviceMap.get(id)).filter((s) => s !== undefined);
		} else {
			return [];
		}
	});
}

export function getServicesForGroupReactive(group_id: string) {
	return derived([groups, services], ([$groups, $services]) => {
		const group = $groups.find((g) => g.id == group_id);

		if (group) {
			const serviceMap = new Map($services.flatMap((s) => s.bindings.map((b) => [b.id, s])));
			return group.service_bindings.map((sb) => serviceMap.get(sb)).filter((s) => s !== undefined);
		} else {
			return [];
		}
	});
}

export function serviceHasInterfaceOnSubnet(service: Service, subnetId: string): boolean {
	const host = getServiceHost(service.id);
	if (!host) return false;

	return service.bindings.some((binding) => {
		const iface = host.interfaces.find((iface) => iface.id === binding.interface_id);
		return iface && iface.subnet_id === subnetId;
	});
}

export function getServiceName(service: Service): string {
	return service.name || service.service_definition;
}

export function getServicesForPort(port_id: string): Service[] {
	const host = get(hosts).find((h) => h.ports.some((p) => p.id === port_id));

	if (host) {
		const services = getServicesForHost(host.id);
		return services.filter((s) =>
			s.bindings.some((b) => b.type == 'Layer4' && b.port_id === port_id)
		);
	}
	return [];
}

export function getServicesForInterface(interface_id: string): Service[] {
	const host = get(hosts).find((h) => h.interfaces.some((i) => i.id === interface_id));

	if (host) {
		const services = getServicesForHost(host.id);
		return services.filter((s) => s.bindings.some((b) => b.interface_id === interface_id));
	}
	return [];
}

export function getServiceForBinding(binding_id: string): Service | null {
	return get(services).find((s) => s.bindings.map((b) => b.id).includes(binding_id)) || null;
}

export function getBindingFromId(id: string): Binding | null {
	return (
		get(services)
			.flatMap((s) => s.bindings)
			.find((b) => b.id == id) || null
	);
}

export function getLayerBindingDisplayName(binding: Binding): string {
	const service = getServiceForBinding(binding.id);
	if (service) {
		const iface = binding.interface_id ? getInterfaceFromId(binding.interface_id) : ALL_INTERFACES;
		const host = getServiceHost(service.id);
		if (host) {
			switch (binding.type) {
				case 'Layer3':
					if (iface) return formatInterface(iface);
					break;
				case 'Layer4': {
					const port = getPortFromId(binding.port_id);
					if (port && iface) return formatInterface(iface) + ' · ' + formatPort(port);
					break;
				}
			}
		}
	}
	return 'Unknown Binding';
}
