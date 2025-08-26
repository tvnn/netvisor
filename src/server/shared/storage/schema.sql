CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    target TEXT NOT NULL,
    description TEXT,
    node_type TEXT,
    capabilities TEXT,
    assigned_tests TEXT,
    monitoring_interval INTEGER,
    node_groups TEXT,
    position TEXT,
    current_status TEXT DEFAULT 'Unknown',
    subnet_membership TEXT,
    open_ports TEXT,
    detected_services TEXT,
    mac_address TEXT,
    last_seen TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS node_groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    node_sequence TEXT NOT NULL,
    auto_diagnostic_enabled BOOLEAN DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS diagnostic_executions (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL,
    trigger_reason TEXT NOT NULL,
    node_results TEXT NOT NULL,
    status TEXT NOT NULL,
    generated_remediation_id TEXT,
    started_at TEXT NOT NULL,
    updated_at TEXT,
    completed_at TEXT
);

CREATE TABLE IF NOT EXISTS daemons {
    id TEXT PRIMARY KEY,
    ip TEXT NOT NULL,
    port INTEGER NOT NULL,
    hostname TEXT NOT NULL,
    status TEXT NOT NULL,
    registered_at TEXT NOT NULL,
    last_seen TEXT NOT NULL
}

CREATE INDEX IF NOT EXISTS idx_nodes_type ON nodes(node_type);
CREATE INDEX IF NOT EXISTS idx_diagnostic_executions_group ON diagnostic_executions(group_id);
CREATE INDEX IF NOT EXISTS idx_diagnostic_executions_status ON diagnostic_executions(status);
CREATE INDEX IF NOT EXISTS idx_diagnostic_executions_created ON diagnostic_executions(started_at);