import type { ServiceBinding } from '$lib/features/hosts/types/base';

export interface Group {
	id: string;
	created_at: string;
	updated_at: string;
	name: string;
	description: string;
	service_bindings: ServiceBinding[];
	group_type: string;
}
