CREATE TABLE IF NOT EXISTS daemons (
    id BLOB PRIMARY KEY,
    host_id BLOB NOT NULL,
    ip TEXT NOT NULL,
    port INTEGER NOT NULL,
    registered_at TEXT NOT NULL,
    last_seen TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_daemon_host_id ON daemons(host_id);