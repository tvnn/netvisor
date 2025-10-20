CREATE TABLE IF NOT EXISTS subnets (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    cidr TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    subnet_type TEXT NOT NULL,
    source JSONB NOT NULL
);
