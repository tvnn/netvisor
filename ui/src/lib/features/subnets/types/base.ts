import type { EntitySource } from '$lib/shared/types';

export interface Subnet {
	id: string;
	created_at: string;
	updated_at: string;
	cidr: string;
	name: string;
	description?: string;
	subnet_type: string;
	source: EntitySource;
}
