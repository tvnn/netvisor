CREATE TABLE IF NOT EXISTS hosts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    hostname TEXT,
    description TEXT,
    target TEXT NOT NULL,
    subnets TEXT,
    services TEXT,
    open_ports TEXT,
    groups TEXT,
    last_seen TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS host_groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    hosts TEXT NOT NULL,
    auto_diagnostic_enabled BOOLEAN DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS daemons (
    id TEXT PRIMARY KEY,
    host_id TEXT NOT NULL,
    registered_at TEXT NOT NULL,
    last_seen TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subnets (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    cidr TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    hosts TEXT NOT NULL,
    dns_resolvers TEXT NOT NULL,
    gateways TEXT NOT NULL,
    subnet_type TEXT NOT NULL,
    source TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_subnets_unique_cidr_gateways ON subnets(cidr, gateways);
