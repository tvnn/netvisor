import { writable, get, derived } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { PortInterfaceBinding, Service } from './types/base';
import { formatPort, utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import { formatInterface, getInterfaceFromId, getPortFromId, hosts } from '../hosts/store';
import type { Host, ServiceBinding } from '../hosts/types/base';

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
		bindings: []
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
		return services.filter((s) => s.bindings.some((b) => b.port_id === port_id));
	}
	return [];
}

export function getServiceBindingsFromService(service: Service): ServiceBinding[] {
	return service.bindings.map((binding) => {
		return {
			service_id: service.id,
			binding_id: binding.id
		} as ServiceBinding;
	});
}

export function getServiceForBinding(binding: PortInterfaceBinding): Service | null {
	return get(services).find((s) => s.bindings.map((b) => b.id).includes(binding.id)) || null;
}

export function getBindingFromId(id: string): PortInterfaceBinding | null {
	return (
		get(services)
			.flatMap((s) => s.bindings)
			.find((b) => b.id == id) || null
	);
}

export function getPortInterfaceBindingDisplayName(binding: PortInterfaceBinding): string {
	const service = getServiceForBinding(binding);
	if (service) {
		const host = getServiceHost(service.id);
		if (host) {
			const port = getPortFromId(binding.port_id);
			const iface = getInterfaceFromId(binding.interface_id);
			if (port && iface) return formatInterface(iface) + formatPort(port);
		}
	}
	return 'Unknown Binding';
}
