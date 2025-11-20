-- Hierarchy Tool Database Migration v1
-- Initial schema for hierarchy tool

CREATE TABLE IF NOT EXISTS hierarchy_nodes (
    id TEXT PRIMARY KEY,
    parent_id TEXT,
    name TEXT NOT NULL,
    node_type TEXT NOT NULL,
    content TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES hierarchy_nodes(id)
);

CREATE INDEX IF NOT EXISTS idx_hierarchy_nodes_parent_id ON hierarchy_nodes(parent_id);
CREATE INDEX IF NOT EXISTS idx_hierarchy_nodes_node_type ON hierarchy_nodes(node_type);

CREATE TABLE IF NOT EXISTS hierarchy_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert initial metadata
INSERT OR IGNORE INTO hierarchy_metadata (key, value) VALUES 
    ('schema_version', '1'),
    ('created_at', CURRENT_TIMESTAMP);