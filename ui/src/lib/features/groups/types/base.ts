import type { EntitySource } from '$lib/shared/types';

export interface Group {
	id: string;
	created_at: string;
	updated_at: string;
	name: string;
	description: string;
	// Binding IDs
	service_bindings: string[];
	group_type: string;
	source: EntitySource;
	network_id: string;
}
