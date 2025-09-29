interface Subnet {
	id: string;
	created_at: string;
	updated_at: string;
	cidr: string;
	name: string;
	dns_resolvers: string[];
	gateways: string[];
	reverse_proxies: string[];
	hosts: string[];
	description?: string;
	subnet_type: string;
	source: string;
}
