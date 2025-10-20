CREATE TABLE IF NOT EXISTS hosts (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    hostname TEXT,
    description TEXT,
    target JSONB NOT NULL,
    interfaces JSONB,
    services JSONB,
    ports JSONB,
    source JSONB NOT NULL,
    virtualization JSONB,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);