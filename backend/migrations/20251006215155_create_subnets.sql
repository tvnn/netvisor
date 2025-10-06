CREATE TABLE IF NOT EXISTS subnets (
    id BLOB PRIMARY KEY,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    cidr TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    hosts TEXT NOT NULL,
    dns_resolvers TEXT NOT NULL,
    gateways TEXT NOT NULL,
    reverse_proxies TEXT NOT NULL,
    subnet_type TEXT NOT NULL,
    source TEXT NOT NULL
);
