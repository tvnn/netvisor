CREATE TABLE IF NOT EXISTS services (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    name TEXT NOT NULL,
    host_id UUID NOT NULL,
    bindings JSONB,
    service_definition TEXT NOT NULL,
    virtualization JSONB,
    vms JSONB,
    containers JSONB,
    source JSONB NOT NULL,
    FOREIGN KEY (host_id) REFERENCES hosts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_services_host_id ON services(host_id);