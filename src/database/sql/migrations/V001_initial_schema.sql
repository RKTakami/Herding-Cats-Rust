-- Initial database schema migration
-- This migration creates the complete database schema for Herding Cats

-- Version: 001
-- Description: Initial schema with projects, documents, embeddings, search, and backup support
-- Date: 2025-11-11

BEGIN TRANSACTION;

-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

-- Create projects table for multi-project management
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    is_archived BOOLEAN NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT 0,
    settings TEXT
);

-- Create documents table for document storage
CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    document_type TEXT NOT NULL DEFAULT 'markdown',
    word_count INTEGER NOT NULL DEFAULT 0,
    checksum TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    version INTEGER NOT NULL DEFAULT 1,
    metadata TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Create document embeddings table for vector storage
CREATE TABLE document_embeddings (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    vector_data BLOB NOT NULL,
    model_name TEXT NOT NULL,
    chunk_index INTEGER NOT NULL DEFAULT 0,
    text_chunk TEXT NOT NULL,
    start_char INTEGER NOT NULL,
    end_char INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    metadata TEXT,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Create full-text search virtual table using FTS5
CREATE VIRTUAL TABLE document_fts USING fts5(
    title,
    content,
    id UNINDEXED,
    project_id UNINDEXED,
    content='documents',
    content_rowid='id'
);

-- Create document versions table for version control
CREATE TABLE document_versions (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    version INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    change_description TEXT,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    UNIQUE(document_id, version)
);

-- Create change log table for real-time tracking
CREATE TABLE change_log (
    id TEXT PRIMARY KEY,
    project_id TEXT,
    document_id TEXT,
    change_type TEXT NOT NULL,
    change_description TEXT,
    timestamp DATETIME NOT NULL,
    user_identifier TEXT,
    metadata TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Create backup metadata table for backup management
CREATE TABLE backup_metadata (
    id TEXT PRIMARY KEY,
    backup_name TEXT NOT NULL UNIQUE,
    backup_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    checksum TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    project_id TEXT,
    compression_type TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
);

-- Create project settings table for configurations
CREATE TABLE project_settings (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    setting_key TEXT NOT NULL,
    setting_value TEXT NOT NULL,
    setting_type TEXT NOT NULL DEFAULT 'string',
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, setting_key)
);

-- Create indexes for performance optimization
CREATE INDEX idx_projects_updated_at ON projects(updated_at);
CREATE INDEX idx_projects_active ON projects(is_active) WHERE is_active = 1;
CREATE INDEX idx_projects_archived ON projects(is_archived) WHERE is_archived = 0;

CREATE INDEX idx_documents_project_id ON documents(project_id);
CREATE INDEX idx_documents_updated_at ON documents(updated_at);
CREATE INDEX idx_documents_active ON documents(is_active) WHERE is_active = 1;
CREATE INDEX idx_documents_type ON documents(document_type);
CREATE INDEX idx_documents_word_count ON documents(word_count);

CREATE INDEX idx_embeddings_document_id ON document_embeddings(document_id);
CREATE INDEX idx_embeddings_model ON document_embeddings(model_name);
CREATE INDEX idx_embeddings_chunk ON document_embeddings(document_id, chunk_index);

CREATE INDEX idx_versions_document_id ON document_versions(document_id);
CREATE INDEX idx_versions_version ON document_versions(document_id, version);

CREATE INDEX idx_change_log_timestamp ON change_log(timestamp);
CREATE INDEX idx_change_log_project ON change_log(project_id);
CREATE INDEX idx_change_log_document ON change_log(document_id);

CREATE INDEX idx_backup_metadata_created ON backup_metadata(created_at);
CREATE INDEX idx_backup_metadata_project ON backup_metadata(project_id);

CREATE INDEX idx_project_settings_project ON project_settings(project_id);
CREATE INDEX idx_project_settings_key ON project_settings(project_id, setting_key);

COMMIT;

-- Record migration in schema version table
CREATE TABLE IF NOT EXISTS schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at DATETIME NOT NULL,
    description TEXT
);

INSERT OR REPLACE INTO schema_migrations (version, applied_at, description) 
VALUES ('001', CURRENT_TIMESTAMP, 'Initial database schema');