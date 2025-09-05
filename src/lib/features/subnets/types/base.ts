interface Subnet {
    id: string,
    created_at: string,
    updated_at: string,
    cidr: string,
    name: string,
    dns_resolvers: string[],
    gateways: string[]
    description?: string
}

interface NodeSubnetMembership {
    subnet_id: string,
    ip_address?: string,
    mac_address?: string
}