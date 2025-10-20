CREATE TABLE IF NOT EXISTS groups (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    group_type TEXT NOT NULL,
    service_bindings JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    source JSONB NOT NULL
);