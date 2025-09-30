export interface Service {
	id: string;
	created_at: string;
	updated_at: string;
	host_id: string;
	service_definition: string;
	name: string;
	port_bindings: string[];
	interface_bindings: string[];
}
