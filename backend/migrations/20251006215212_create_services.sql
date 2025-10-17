CREATE TABLE IF NOT EXISTS services (
    id BLOB PRIMARY KEY,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    name TEXT NOT NULL,
    host_id BLOB NOT NULL,
    bindings TEXT,
    service_definition TEXT NOT NULL,
    virtualization TEXT,
    vms TEXT,
    containers TEXT,
    source TEXT NOT NULL,
    FOREIGN KEY (host_id) REFERENCES hosts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_services_host_id ON services(host_id);