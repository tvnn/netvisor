// src/lib/features/services/store.ts
import { writable, derived, get } from 'svelte/store';
import { api } from '../../shared/utils/api';
import type { Service } from './types/base';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';
import { hosts } from '../hosts/store';
import type { Host } from '../hosts/types/base';

export const services = writable<Service[]>([]);

// Create a new service
export async function createService(service: Omit<Service, 'id' | 'created_at' | 'updated_at'>) {
	return await api.request<Service, Service[]>(
		'/services',
		services,
		(newService: Service, currentServices: Service[]) => [...currentServices, newService],
		{
			method: 'POST',
			body: JSON.stringify(service)
		}
	);
}

// Update an existing service
export async function updateService(serviceId: string, updates: Partial<Service>) {
	return await api.request<Service, Service[]>(
		`/services/${serviceId}`,
		services,
		(updatedService: Service, currentServices: Service[]) =>
			currentServices.map((s) => (s.id === serviceId ? updatedService : s)),
		{
			method: 'PUT',
			body: JSON.stringify(updates)
		}
	);
}

// Delete a service
export async function deleteService(serviceId: string) {
	return await api.request<void, Service[]>(
		`/services/${serviceId}`,
		services,
		(_, currentServices: Service[]) => currentServices.filter((s) => s.id !== serviceId),
		{ method: 'DELETE' }
	);
}

// Get all services
export async function getServices() {
	return await api.request<Service[]>(
		'/services',
		services,
		(services) => services,
		{ method: 'GET' },
		true
	);
}

// Bulk operations for host editing
export async function updateHostServices(
	hostId: string,
	servicesToUpdate: Service[],
	servicesToDelete: string[] = []
) {
	const promises = [];

	// Delete services first
	for (const serviceId of servicesToDelete) {
		promises.push(deleteService(serviceId));
	}

	// Update/create services
	for (const service of servicesToUpdate) {
		if (service.id) {
			promises.push(updateService(service.id, service));
		} else {
			promises.push(createService({ ...service, host_id: hostId }));
		}
	}

	await Promise.all(promises);
}

// Helper functions for working with services and the MetadataRegistry
export function createDefaultService(
	serviceType: string,
	host_id: string,
	serviceName?: string,
	defaultPorts?: string[]
): Service {
	return {
		id: uuidv4Sentinel,
		created_at: utcTimeZoneSentinel,
		updated_at: utcTimeZoneSentinel,
		host_id,
		service_definition: serviceType,
		name: serviceName || serviceType,
		port_bindings: defaultPorts ? [...defaultPorts] : [],
		interface_bindings: [],
		groups: []
	};
}

export function getServiceById(service_id: string): Service | null {
	return get(services).find((s) => s.id == service_id) || null;
}

export function getServiceHost(service_id: string): Host | null {
	let service = get(services).find((s) => s.id == service_id);
	if (service) {
		let host = get(hosts).find((h) => h.id == service.host_id) || null;
		return host;
	}
	return null;
}

export function getServicesForHost(host_id: string): Service[] {
	let host = get(hosts).find((h) => h.id == host_id);

	if (host) {
		const serviceMap = new Map(get(services).map((s) => [s.id, s]));
		return host.services.map((id) => serviceMap.get(id)).filter((s) => s != undefined);
	} else {
		return [];
	}
}

export function serviceHasInterfaceOnSubnet(service: Service, subnetId: string): boolean {
	const host = getServiceHost(service.id);
	if (!host) return false;

	return service.interface_bindings.some((bindingId) => {
		const iface = host.interfaces.find((iface) => iface.id === bindingId);
		return iface && iface.subnet_id === subnetId;
	});
}

export function getServiceName(service: Service): string {
	return service.name || service.service_definition;
}

export function getServicesForPort(port_id: string): Service[] {
	let host = get(hosts).find((h) => h.ports.some((p) => p.id === port_id));

	console.log(host);

	if (host) {
		let services = getServicesForHost(host.id);
		console.log(services);
		return services.filter((s) => s.port_bindings.some((p) => p === port_id));
	} else {
		return [];
	}
}
