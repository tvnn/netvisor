CREATE TABLE IF NOT EXISTS services (
    id BLOB PRIMARY KEY,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    name TEXT NOT NULL,
    host_id BLOB NOT NULL,
    groups TEXT,
    port_bindings TEXT NOT NULL,
    interface_bindings TEXT NOT NULL,
    service_definition TEXT NOT NULL,
    FOREIGN KEY (host_id) REFERENCES hosts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_services_host_id ON services(host_id);