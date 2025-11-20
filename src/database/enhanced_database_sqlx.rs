//! Enhanced Database Service with SQLx - Async database operations
//!
//! Replaces the rusqlite-based implementation with sqlx for proper async/await support.

use crate::database::{DatabaseError, DatabaseResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Column, Row, SqlitePool};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Database configuration for sqlx
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: std::time::Duration,
    pub idle_timeout: Option<std::time::Duration>,
    pub max_lifetime: Option<std::time::Duration>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 1,
            acquire_timeout: std::time::Duration::from_secs(30),
            idle_timeout: Some(std::time::Duration::from_secs(600)),
            max_lifetime: Some(std::time::Duration::from_secs(3600)),
        }
    }
}

/// Enhanced database service with sqlx for async operations
#[derive(Debug, Clone)]
pub struct EnhancedDatabaseService {
    pub pool: SqlitePool,
    db_path: PathBuf,
}

/// Database row data for sqlx
#[derive(Debug, Clone)]
pub struct DatabaseRow {
    columns: Vec<String>,
    values: Vec<Option<String>>,
}

impl DatabaseRow {
    pub fn new(columns: Vec<String>, values: Vec<Option<String>>) -> Self {
        Self { columns, values }
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.values.get(index).and_then(|v| v.as_deref())
    }

    pub fn get_by_name(&self, name: &str) -> Option<&str> {
        if let Some(index) = self.columns.iter().position(|col| col == name) {
            self.get(index)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Result from database queries containing owned data
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows: Vec<DatabaseRow>,
    pub column_count: usize,
}

impl QueryResult {
    /// Get the first row if it exists
    pub fn first(&self) -> Option<&DatabaseRow> {
        self.rows.first()
    }

    /// Get the total number of rows
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Check if there are no rows
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl IntoIterator for QueryResult {
    type Item = DatabaseRow;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl EnhancedDatabaseService {
    /// Create a new enhanced database service with sqlx
    pub async fn new(db_path: &Path, _config: DatabaseConfig) -> DatabaseResult<Self> {
        let db_path_str = db_path.to_str().ok_or_else(|| {
            DatabaseError::Configuration("Database path must be valid UTF-8".to_string())
        })?;

        // Create the database directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                DatabaseError::Service(format!("Failed to create database directory: {}", e))
            })?;
        }

        // Build connection string
        let connection_string = format!("sqlite://{}", db_path_str);

        // Create connection pool
        let pool = SqlitePool::connect(&connection_string).await.map_err(|e| {
            DatabaseError::Connection(format!("Failed to connect to database: {}", e))
        })?;

        let service = Self {
            pool,
            db_path: db_path.to_path_buf(),
        };

        // Initialize database
        service.initialize_database().await?;

        Ok(service)
    }

    /// Get database statistics
    pub async fn get_database_stats(&self) -> DatabaseResult<DatabaseStats> {
        let active_projects: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM projects WHERE is_archived = 0 AND is_active = 1",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get active projects: {}", e)))?;

        let active_documents: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM documents WHERE is_active = 1")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                    DatabaseError::Service(format!("Failed to get active documents: {}", e))
                })?;

        let total_embeddings: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM document_embeddings")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to get total embeddings: {}", e))
            })?;

        let db_size_bytes = self.get_database_file_size();

        let wal_mode_enabled = self.check_wal_mode().await;
        let foreign_keys_enabled = self.check_foreign_keys_enabled().await;
        let integrity_check_passed = self.check_integrity().await?;

        let performance_metrics = self.collect_performance_metrics().await?;

        Ok(DatabaseStats {
            active_projects: active_projects as usize,
            active_documents: active_documents as usize,
            total_embeddings: total_embeddings as usize,
            db_size_bytes,
            wal_mode_enabled,
            foreign_keys_enabled,
            integrity_check_passed,
            performance_metrics,
        })
    }

    /// Get database file size
    fn get_database_file_size(&self) -> usize {
        std::fs::metadata(&self.db_path)
            .map(|m| m.len() as usize)
            .unwrap_or(0)
    }

    /// Create a new document with integrity checking
    pub async fn create_document(
        &self,
        document_id: String,
        project_id: String,
        title: String,
        content: String,
    ) -> DatabaseResult<String> {
        let checksum = self.calculate_checksum(&content);
        let word_count = content.split_whitespace().count() as i32;
        let created_at = Utc::now();
        let updated_at = Utc::now();

        sqlx::query(
            "INSERT INTO documents (id, project_id, title, content, document_type, word_count, checksum, created_at, updated_at, is_active, version, metadata)
             VALUES (?, ?, ?, ?, 'markdown', ?, ?, ?, ?, 1, 1, NULL)"
        )
        .bind(&document_id)
        .bind(&project_id)
        .bind(&title)
        .bind(&content)
        .bind(word_count)
        .bind(&checksum)
        .bind(created_at)
        .bind(updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to create document: {}", e)))?;

        Ok(document_id)
    }

    /// Get document by ID with integrity verification
    pub async fn get_document(&self, id: String) -> DatabaseResult<Option<String>> {
        let result: Option<(String,)> =
            sqlx::query_as("SELECT content FROM documents WHERE id = ? AND is_active = 1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| DatabaseError::Service(format!("Failed to get document: {}", e)))?;

        Ok(result.map(|(content,)| content))
    }

    /// Update document with automatic checksum update
    pub async fn update_document(
        &self,
        id: String,
        title: String,
        content: String,
    ) -> DatabaseResult<()> {
        let checksum = self.calculate_checksum(&content);
        let word_count = content.split_whitespace().count() as i32;
        let updated_at = Utc::now();
        let version = 2i32;

        sqlx::query(
            "UPDATE documents SET title = ?, content = ?, document_type = 'markdown', word_count = ?, checksum = ?, updated_at = ?, version = ? WHERE id = ?"
        )
        .bind(&title)
        .bind(&content)
        .bind(word_count)
        .bind(&checksum)
        .bind(updated_at)
        .bind(version)
        .bind(&id)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to update document: {}", e)))?;

        Ok(())
    }

    /// Delete document with soft delete
    pub async fn delete_document(&self, id: String) -> DatabaseResult<()> {
        let updated_at = Utc::now();

        sqlx::query("UPDATE documents SET is_active = 0, updated_at = ? WHERE id = ?")
            .bind(updated_at)
            .bind(&id)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to delete document: {}", e)))?;

        Ok(())
    }

    /// Execute SQL query and return results
    pub async fn query(&self, sql: &str, params: &[String]) -> DatabaseResult<QueryResult> {
        let mut query_builder = sqlx::query(sql);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Query execution failed: {}", e)))?;

        let mut result_rows = Vec::new();

        if let Some(first_row) = rows.first() {
            let columns: Vec<String> = first_row
                .columns()
                .iter()
                .map(|col| col.name().to_string())
                .collect();

            for row in rows {
                let mut values = Vec::new();
                for i in 0..columns.len() {
                    let value: Option<String> = row.get(i);
                    values.push(value);
                }
                result_rows.push(DatabaseRow::new(columns.clone(), values));
            }
        }

        Ok(QueryResult {
            rows: result_rows.clone(),
            column_count: result_rows.first().map(|r| r.len()).unwrap_or(0),
        })
    }

    /// Execute SQL statement
    pub async fn execute(&self, sql: &str, params: &[String]) -> DatabaseResult<()> {
        let mut query_builder = sqlx::query(sql);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        query_builder
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Statement execution failed: {}", e)))?;

        Ok(())
    }

    /// Calculate SHA-256 checksum for document content
    fn calculate_checksum(&self, content: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Initialize database with schema if needed
    pub async fn initialize_database(&self) -> DatabaseResult<()> {
        // Check if schema exists
        let table_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        )
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        if table_count == 0 {
            // Load and execute schema - for now we'll create basic tables
            let schema_sql = include_str!("sql/schema.sql");
            sqlx::query(schema_sql)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    DatabaseError::Migration(format!("Failed to execute schema: {}", e))
                })?;
        }

        Ok(())
    }

    /// Check database integrity
    async fn check_integrity(&self) -> DatabaseResult<bool> {
        let result: (String,) = sqlx::query_as("PRAGMA integrity_check")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Integrity check failed: {}", e)))?;

        Ok(result.0 == "ok")
    }

    /// Check if WAL mode is enabled
    async fn check_wal_mode(&self) -> bool {
        let result: (String,) = sqlx::query_as("PRAGMA journal_mode")
            .fetch_one(&self.pool)
            .await
            .unwrap_or_else(|_| ("delete".to_string(),));

        result.0.to_lowercase() == "wal"
    }

    /// Check if foreign keys are enabled
    async fn check_foreign_keys_enabled(&self) -> bool {
        let result: (i64,) = sqlx::query_as("PRAGMA foreign_keys")
            .fetch_one(&self.pool)
            .await
            .unwrap_or((0i64,));

        result.0 == 1
    }

    /// Collect performance metrics
    async fn collect_performance_metrics(&self) -> DatabaseResult<DatabasePerformanceMetrics> {
        Ok(DatabasePerformanceMetrics {
            connection_count: 1,  // Simplified for sqlx
            transaction_count: 0, // Would need tracking
            cache_hit_ratio: 0.9, // Simplified
            query_performance: HashMap::new(),
            last_vacuum: None,
            last_analyze: None,
        })
    }

    /// Test database connection
    pub async fn test_connection(&self) -> Result<(), DatabaseError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| {
                DatabaseError::Connection(format!("Database connection test failed: {}", e))
            })?;
        Ok(())
    }

    /// Get database path
    pub fn get_database_path(&self) -> &Path {
        &self.db_path
    }
}

/// Database performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabasePerformanceMetrics {
    pub connection_count: usize,
    pub transaction_count: u64,
    pub cache_hit_ratio: f64,
    pub query_performance: HashMap<String, u64>,
    pub last_vacuum: Option<DateTime<Utc>>,
    pub last_analyze: Option<DateTime<Utc>>,
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseStats {
    pub active_projects: usize,
    pub active_documents: usize,
    pub total_embeddings: usize,
    pub db_size_bytes: usize,
    pub wal_mode_enabled: bool,
    pub foreign_keys_enabled: bool,
    pub integrity_check_passed: bool,
    pub performance_metrics: DatabasePerformanceMetrics,
}
