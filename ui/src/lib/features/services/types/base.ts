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
	// Host UUIDs
	vms: string[];
	// Service UUIDs
	containers: string[];
}

export type ServiceWithVMs = Omit<Service, 'vms'> & {
	vms: string[];
};

export interface DockerVirtualization {
	container_id: string;
	container_name: string;
}

export type Binding =
	| { type: 'Layer3'; id: string; interface_id: string }
	| { type: 'Layer4'; id: string; interface_id: string | null; port_id: string };

export type Layer4Binding = Extract<Binding, { type: 'Layer4' }>;
export type Layer3Binding = Extract<Binding, { type: 'Layer3' }>;
