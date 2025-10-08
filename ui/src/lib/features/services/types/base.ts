export interface Service<T extends Binding = Binding> {
	id: string;
	created_at: string;
	updated_at: string;
	host_id: string;
	service_definition: string;
	name: string;
	bindings: T[];
}

export type Binding =
	| { type: 'Layer3'; id: string; interface_id: string }
	| { type: 'Layer4'; id: string; interface_id: string; port_id: string };

export type Layer4Binding = Extract<Binding, { type: 'Layer4' }>;
export type Layer3Binding = Extract<Binding, { type: 'Layer3' }>;
