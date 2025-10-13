CREATE TABLE IF NOT EXISTS groups (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    group_type TEXT NOT NULL,
    service_bindings TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    source TEXT NOT NULL
);