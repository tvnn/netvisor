CREATE TABLE IF NOT EXISTS hosts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    hostname TEXT,
    description TEXT,
    target TEXT NOT NULL,
    interfaces TEXT,
    services TEXT,
    open_ports TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    services TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS daemons (
    id TEXT PRIMARY KEY,
    host_id TEXT NOT NULL,
    ip TEXT NOT NULL,
    port INTEGER NOT NULL,
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
    reverse_proxies TEXT NOT NULL,
    subnet_type TEXT NOT NULL,
    source TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS services (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    name TEXT NOT NULL,
    host_id TEXT NOT NULL,
    groups TEXT,
    ports TEXT NOT NULL,
    interface_bindings TEXT NOT NULL,
    service_type TEXT NOT NULL,
    FOREIGN KEY (host_id) REFERENCES hosts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_services_host_id ON services(host_id)