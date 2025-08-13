CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT,
    ip TEXT,
    port INTEGER,
    path TEXT,
    description TEXT,
    node_type TEXT,
    capabilities TEXT,
    assigned_tests TEXT,
    monitoring_enabled BOOLEAN DEFAULT false,
    node_groups TEXT,
    position TEXT,
    current_status TEXT DEFAULT 'Unknown',
    subnet_membership TEXT,
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

CREATE TABLE IF NOT EXISTS subnet_groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    cidr TEXT NOT NULL,
    node_ids TEXT NOT NULL,
    vlan_id INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS diagnostic_executions (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL,
    group_name TEXT NOT NULL,
    trigger_reason TEXT NOT NULL,
    node_results TEXT NOT NULL,
    overall_status TEXT NOT NULL,
    generated_remediation_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS remediations (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    steps TEXT NOT NULL,
    generated_from_diagnostic TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nodes_type ON nodes(node_type);
CREATE INDEX IF NOT EXISTS idx_nodes_status ON nodes(current_status);
CREATE INDEX IF NOT EXISTS idx_nodes_monitoring ON nodes(monitoring_enabled);
CREATE INDEX IF NOT EXISTS idx_diagnostic_executions_group ON diagnostic_executions(group_id);
CREATE INDEX IF NOT EXISTS idx_diagnostic_executions_status ON diagnostic_executions(overall_status);
CREATE INDEX IF NOT EXISTS idx_diagnostic_executions_created ON diagnostic_executions(created_at);