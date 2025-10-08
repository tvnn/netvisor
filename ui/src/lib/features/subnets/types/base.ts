import type { ServiceBinding } from '$lib/features/hosts/types/base';

export interface Subnet {
	id: string;
	created_at: string;
	updated_at: string;
	cidr: string;
	name: string;
	dns_resolvers: ServiceBinding[];
	gateways: ServiceBinding[];
	reverse_proxies: ServiceBinding[];
	hosts: string[];
	description?: string;
	subnet_type: string;
	source: string;
}
