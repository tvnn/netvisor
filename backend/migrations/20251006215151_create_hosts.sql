CREATE TABLE IF NOT EXISTS hosts (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    hostname TEXT,
    description TEXT,
    target TEXT NOT NULL,
    interfaces TEXT,
    services TEXT,
    ports TEXT,
    source TEXT NOT NULL,
    virtualization TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);