import type { EntitySource } from '$lib/shared/types';

export type ServiceVirtualization = { type: 'Docker'; details: DockerVirtualization };

export interface Service<T extends Binding = Binding> {
	id: string;
	created_at: string;
	updated_at: string;
	host_id: string;
	service_definition: string;
	name: string;
	bindings: T[];
	virtualization: ServiceVirtualization | null;
	source: EntitySource;
	network_id: string;
}

export type ServiceWithVMs = Omit<Service, 'vms'> & {
	vms: string[];
};

export interface DockerVirtualization {
	container_id: string | null;
	container_name: string | null;
	service_id: string;
}

export type Binding =
	| { type: 'Layer3'; id: string; interface_id: string }
	| { type: 'Layer4'; id: string; interface_id: string | null; port_id: string };

export type Layer4Binding = Extract<Binding, { type: 'Layer4' }>;
export type Layer3Binding = Extract<Binding, { type: 'Layer3' }>;
