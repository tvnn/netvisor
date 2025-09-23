CREATE TABLE IF NOT EXISTS hosts (
    id BLOB PRIMARY KEY,
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
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    service_bindings TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS daemons (
    id BLOB PRIMARY KEY,
    host_id BLOB NOT NULL,
    ip TEXT NOT NULL,
    port INTEGER NOT NULL,
    registered_at TEXT NOT NULL,
    last_seen TEXT NOT NULL
);

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

CREATE TABLE IF NOT EXISTS services (
    id BLOB PRIMARY KEY,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    name TEXT NOT NULL,
    host_id BLOB NOT NULL,
    groups TEXT,
    ports TEXT NOT NULL,
    interface_bindings TEXT NOT NULL,
    service_definition TEXT NOT NULL,
    FOREIGN KEY (host_id) REFERENCES hosts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_services_host_id ON services(host_id)