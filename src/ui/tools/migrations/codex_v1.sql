-- Codex Tool Database Migration v1
-- Initial schema for codex tool

CREATE TABLE IF NOT EXISTS codex_entries (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT,
    entry_type TEXT NOT NULL,
    tags TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_codex_entries_entry_type ON codex_entries(entry_type);
CREATE INDEX IF NOT EXISTS idx_codex_entries_tags ON codex_entries(tags);

CREATE TABLE IF NOT EXISTS codex_relations (
    id TEXT PRIMARY KEY,
    source_entry_id TEXT NOT NULL,
    target_entry_id TEXT NOT NULL,
    relation_type TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_entry_id) REFERENCES codex_entries(id) ON DELETE CASCADE,
    FOREIGN KEY (target_entry_id) REFERENCES codex_entries(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_codex_relations_source ON codex_relations(source_entry_id);
CREATE INDEX IF NOT EXISTS idx_codex_relations_target ON codex_relations(target_entry_id);

CREATE TABLE IF NOT EXISTS codex_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert initial metadata
INSERT OR IGNORE INTO codex_metadata (key, value) VALUES 
    ('schema_version', '1'),
    ('created_at', CURRENT_TIMESTAMP);