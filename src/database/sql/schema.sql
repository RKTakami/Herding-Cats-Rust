-- Herding Cats Database Schema
-- Complete database schema with tables, indexes, and constraints

-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

-- Projects table for multi-project management
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,                    -- UUID for project identification
    name TEXT NOT NULL,                     -- Project name
    description TEXT,                       -- Optional project description
    created_at DATETIME NOT NULL,          -- Creation timestamp
    updated_at DATETIME NOT NULL,          -- Last modification timestamp
    is_archived BOOLEAN NOT NULL DEFAULT 0, -- Whether project is archived
    is_active BOOLEAN NOT NULL DEFAULT 0,   -- Whether project is active (single active project)
    settings TEXT                           -- Optional settings as JSON string
);

-- Documents table for document storage
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,                    -- UUID for document identification
    project_id TEXT NOT NULL,               -- Foreign key to projects table
    title TEXT NOT NULL,                    -- Document title
    content TEXT,                           -- Document content (may be large)
    document_type TEXT NOT NULL DEFAULT 'markdown', -- Document type (markdown, plain_text, etc.)
    word_count INTEGER NOT NULL DEFAULT 0,  -- Number of words in document
    checksum TEXT NOT NULL,                 -- SHA-256 checksum for integrity verification
    created_at DATETIME NOT NULL,          -- Creation timestamp
    updated_at DATETIME NOT NULL,          -- Last modification timestamp
    is_active BOOLEAN NOT NULL DEFAULT 1,   -- Whether document is active/not deleted
    version INTEGER NOT NULL DEFAULT 1,     -- Document version number
    metadata TEXT,                          -- Optional metadata as JSON string
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Document embeddings table for vector storage
CREATE TABLE IF NOT EXISTS document_embeddings (
    id TEXT PRIMARY KEY,                    -- UUID for embedding identification
    document_id TEXT NOT NULL,              -- Foreign key to documents table
    vector_data BLOB NOT NULL,              -- Vector data stored as binary blob
    model_name TEXT NOT NULL,               -- Model used to generate embedding
    chunk_index INTEGER NOT NULL DEFAULT 0, -- Chunk index within document
    text_chunk TEXT NOT NULL,               -- Original text chunk
    start_char INTEGER NOT NULL,            -- Starting character position
    end_char INTEGER NOT NULL,              -- Ending character position
    created_at DATETIME NOT NULL,          -- Creation timestamp
    metadata TEXT,                          -- Optional metadata as JSON string
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Full-text search virtual table using FTS5
CREATE VIRTUAL TABLE IF NOT EXISTS document_fts USING fts5(
    title,                                  -- Document title
    content                                 -- Document content
);

-- Document versions table for version control
CREATE TABLE IF NOT EXISTS document_versions (
    id TEXT PRIMARY KEY,                    -- UUID for version identification
    document_id TEXT NOT NULL,              -- Foreign key to documents table
    version INTEGER NOT NULL,               -- Version number
    title TEXT NOT NULL,                    -- Title at this version
    content TEXT NOT NULL,                  -- Content at this version
    created_at DATETIME NOT NULL,          -- Version creation timestamp
    change_description TEXT,                -- Optional change description
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    UNIQUE(document_id, version)
);

-- Change log table for real-time tracking
CREATE TABLE IF NOT EXISTS change_log (
    id TEXT PRIMARY KEY,                    -- UUID for change log entry
    project_id TEXT,                        -- Optional project ID (NULL for system changes)
    document_id TEXT,                       -- Optional document ID (NULL for project-level changes)
    change_type TEXT NOT NULL,              -- Type of change (create, update, delete, etc.)
    change_description TEXT,                -- Human-readable change description
    timestamp DATETIME NOT NULL,           -- Change timestamp
    user_identifier TEXT,                   -- Optional user identifier
    metadata TEXT,                          -- Optional metadata as JSON string
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Backup metadata table for backup management
CREATE TABLE IF NOT EXISTS backup_metadata (
    id TEXT PRIMARY KEY,                    -- UUID for backup identification
    backup_type TEXT NOT NULL,              -- Backup type (manual, automatic, emergency) - JSON serialized
    file_path TEXT NOT NULL,                -- Path to backup file
    file_size INTEGER NOT NULL,             -- Backup file size in bytes
    checksum TEXT NOT NULL,                 -- SHA-256 checksum of backup file
    created_at INTEGER NOT NULL,           -- Backup creation timestamp (i64)
    project_id TEXT,                        -- Optional project-specific backup
    description TEXT,                       -- Optional description
    success BOOLEAN NOT NULL DEFAULT 0,     -- Whether backup was successful
    error_message TEXT,                     -- Optional error message if backup failed
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Project settings table for configurations
CREATE TABLE IF NOT EXISTS project_settings (
    id TEXT PRIMARY KEY,                    -- UUID for settings identification
    project_id TEXT NOT NULL,               -- Foreign key to projects table
    setting_key TEXT NOT NULL,              -- Setting identifier
    setting_value TEXT NOT NULL,            -- Setting value
    setting_type TEXT NOT NULL DEFAULT 'string', -- Setting type (string, number, boolean, json)
    created_at DATETIME NOT NULL,          -- Setting creation timestamp
    updated_at DATETIME NOT NULL,          -- Last update timestamp
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, setting_key)
);

-- Indexes for performance optimization
CREATE INDEX IF NOT EXISTS idx_projects_updated_at ON projects(updated_at);
CREATE INDEX IF NOT EXISTS idx_projects_active ON projects(is_active) WHERE is_active = 1;
CREATE INDEX IF NOT EXISTS idx_projects_archived ON projects(is_archived) WHERE is_archived = 0;

CREATE INDEX IF NOT EXISTS idx_documents_project_id ON documents(project_id);
CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at);
CREATE INDEX IF NOT EXISTS idx_documents_active ON documents(is_active) WHERE is_active = 1;
CREATE INDEX IF NOT EXISTS idx_documents_type ON documents(document_type);
CREATE INDEX IF NOT EXISTS idx_documents_word_count ON documents(word_count);

CREATE INDEX IF NOT EXISTS idx_embeddings_document_id ON document_embeddings(document_id);
CREATE INDEX IF NOT EXISTS idx_embeddings_model ON document_embeddings(model_name);
CREATE INDEX IF NOT EXISTS idx_embeddings_chunk ON document_embeddings(document_id, chunk_index);

CREATE INDEX IF NOT EXISTS idx_versions_document_id ON document_versions(document_id);
CREATE INDEX IF NOT EXISTS idx_versions_version ON document_versions(document_id, version);

CREATE INDEX IF NOT EXISTS idx_change_log_timestamp ON change_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_change_log_project ON change_log(project_id);
CREATE INDEX IF NOT EXISTS idx_change_log_document ON change_log(document_id);

CREATE INDEX IF NOT EXISTS idx_backup_metadata_created ON backup_metadata(created_at);
CREATE INDEX IF NOT EXISTS idx_backup_metadata_project ON backup_metadata(project_id);

CREATE INDEX IF NOT EXISTS idx_project_settings_project ON project_settings(project_id);
CREATE INDEX IF NOT EXISTS idx_project_settings_key ON project_settings(project_id, setting_key);

-- Triggers for maintaining data integrity

-- Update updated_at timestamp for projects
CREATE TRIGGER IF NOT EXISTS update_projects_updated_at 
    AFTER UPDATE ON projects
    FOR EACH ROW
BEGIN
    UPDATE projects SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- Update updated_at timestamp for documents
CREATE TRIGGER IF NOT EXISTS update_documents_updated_at 
    AFTER UPDATE ON documents
    FOR EACH ROW
BEGIN
    UPDATE documents SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- FTS triggers removed - using simple FTS5 table without automatic indexing
-- Search indexing will be handled manually by the application

-- Maintain word count when documents are updated
CREATE TRIGGER IF NOT EXISTS update_document_word_count
    AFTER UPDATE ON documents
    FOR EACH ROW
    WHEN NEW.content != OLD.content
BEGIN
    UPDATE documents
    SET word_count = (
        CASE
            WHEN NEW.content IS NOT NULL AND NEW.content != ''
            THEN LENGTH(TRIM(NEW.content)) - LENGTH(REPLACE(TRIM(NEW.content), ' ', '')) + 1
            ELSE 0
        END
    )
    WHERE id = NEW.id;
END;

-- Automatic version creation when documents are updated
CREATE TRIGGER IF NOT EXISTS auto_document_version
    AFTER UPDATE ON documents
    FOR EACH ROW
    WHEN NEW.content != OLD.content OR NEW.title != OLD.title
BEGIN
    INSERT INTO document_versions (id, document_id, version, title, content, created_at, change_description)
    VALUES (
        lower(hex(randomblob(16))),
        NEW.id,
        NEW.version,
        NEW.title,
        NEW.content,
        CURRENT_TIMESTAMP,
        'Automatic version created on update'
    );
END;

-- Insert initial document version when document is created
CREATE TRIGGER IF NOT EXISTS initial_document_version
    AFTER INSERT ON documents
    FOR EACH ROW
BEGIN
    INSERT INTO document_versions (id, document_id, version, title, content, created_at, change_description)
    VALUES (
        lower(hex(randomblob(16))),
        NEW.id,
        1,
        NEW.title,
        NEW.content,
        CURRENT_TIMESTAMP,
        'Initial version'
    );
END;

-- Update project_settings updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_project_settings_updated_at 
    AFTER UPDATE ON project_settings
    FOR EACH ROW
BEGIN
    UPDATE project_settings SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- Initialize FTS5 search index for existing documents
-- This will be run when needed to backfill existing data