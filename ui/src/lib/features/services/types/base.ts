export interface Service {
	id: string;
	created_at: string;
	updated_at: string;
	host_id: string;
	service_definition: string;
	name: string;
	bindings: PortInterfaceBinding[];
}

export interface PortInterfaceBinding {
	id: string;
	port_id: string;
	interface_id: string;
}
