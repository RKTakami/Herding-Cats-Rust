//! Real Database Integration Implementation
//!
//! Complete SQLx-based database integration for the new architecture patterns.
//! This module provides the actual database operations that connect to SQLite
//! and implement the ToolDatabaseContext interface.

use crate::{
    database::models::codex::{CodexEntry, CodexEntryType, CodexQuery, DatabaseResult},
    ui::tools::database_integration::{ToolDatabaseContext, DatabaseOperationResult},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool, Row};
use std::sync::Arc;
use std::time::{Instant, Duration};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Real database implementation using SQLx
pub struct RealDatabaseIntegration {
    /// SQLx connection pool
    pool: Option<SqlitePool>,
    /// Database path
    database_path: String,
    /// Connection timeout
    connection_timeout: Duration,
    /// Operation timeout
    operation_timeout: Duration,
    /// Connection pool size
    pool_size: u32,
}

impl RealDatabaseIntegration {
    /// Create a new real database integration
    pub fn new(database_path: &str) -> Self {
        Self {
            pool: None,
            database_path: database_path.to_string(),
            connection_timeout: Duration::from_secs(30),
            operation_timeout: Duration::from_secs(60),
            pool_size: 10,
        }
    }

    /// Initialize the database connection
    pub async fn initialize(&mut self) -> Result<()> {
        // Create database URL
        let database_url = if self.database_path == ":memory:" {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite://{}", self.database_path)
        };

        // Create connection pool
        let pool = SqlitePool::connect(&database_url).await?;

        // Configure pool
        sqlx::pool::PoolOptions::<Sqlite>
            .new()
            .max_connections(self.pool_size)
            .acquire_timeout(self.connection_timeout)
            .idle_timeout(Some(Duration::from_secs(300)))
            .max_lifetime(Some(Duration::from_secs(3600)))
            .connect(&database_url)
            .await?;

        self.pool = Some(pool);

        // Initialize database schema
        self.initialize_schema().await?;

        Ok(())
    }

    /// Initialize database schema using main comprehensive schema
    async fn initialize_schema(&self) -> Result<()> {
        if let Some(pool) = &self.pool {
            // Load and execute the main comprehensive schema
            let schema_sql = include_str!("../../../database/sql/schema.sql");

            // Execute the comprehensive schema
            sqlx::query(schema_sql)
                .execute(pool)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to execute main schema: {}", e))?;

            println!("✅ Main comprehensive schema loaded successfully");

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Create a codex entry in the database
    pub async fn create_codex_entry(&self, entry: &CodexEntry) -> Result<()> {
        if let Some(pool) = &self.pool {
            let created_at = Utc::now().to_rfc3339();
            let updated_at = created_at.clone();

            sqlx::query(r#"
                INSERT INTO codex_entries (id, title, content, entry_type, project_id, created_at, updated_at, is_active)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#)
            .bind(&entry.id.to_string())
            .bind(&entry.title)
            .bind(&entry.content)
            .bind(entry.entry_type.to_string())
            .bind(&entry.project_id.to_string())
            .bind(created_at)
            .bind(updated_at)
            .bind(true)
            .execute(pool)
            .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Get a codex entry by ID
    pub async fn get_codex_entry(&self, entry_id: &str) -> Result<Option<CodexEntry>> {
        if let Some(pool) = &self.pool {
            let row = sqlx::query(r#"
                SELECT id, title, content, entry_type, project_id, created_at, updated_at
                FROM codex_entries
                WHERE id = ? AND is_active = 1
            "#)
            .bind(entry_id)
            .fetch_optional(pool)
            .await?;

            if let Some(row) = row {
                let entry = CodexEntry {
                    id: Uuid::parse_str(row.get::<&str, _>("id"))?,
                    title: row.get::<&str, _>("title").to_string(),
                    content: row.get::<&str, _>("content").to_string(),
                    entry_type: CodexEntryType::from_string(row.get::<&str, _>("entry_type"))?,
                    project_id: Uuid::parse_str(row.get::<&str, _>("project_id"))?,
                    created_at: Some(row.get::<&str, _>("created_at").to_string()),
                    updated_at: Some(row.get::<&str, _>("updated_at").to_string()),
                };
                Ok(Some(entry))
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Update a codex entry
    pub async fn update_codex_entry(&self, entry: &CodexEntry) -> Result<()> {
        if let Some(pool) = &self.pool {
            let updated_at = Utc::now().to_rfc3339();

            sqlx::query(r#"
                UPDATE codex_entries
                SET title = ?, content = ?, entry_type = ?, updated_at = ?
                WHERE id = ? AND is_active = 1
            "#)
            .bind(&entry.title)
            .bind(&entry.content)
            .bind(entry.entry_type.to_string())
            .bind(updated_at)
            .bind(&entry.id.to_string())
            .execute(pool)
            .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Delete a codex entry (soft delete)
    pub async fn delete_codex_entry(&self, entry_id: &str) -> Result<()> {
        if let Some(pool) = &self.pool {
            sqlx::query(r#"
                UPDATE codex_entries
                SET is_active = 0
                WHERE id = ?
            "#)
            .bind(entry_id)
            .execute(pool)
            .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// List codex entries with query parameters
    pub async fn list_codex_entries(&self, query: &CodexQuery) -> Result<Vec<CodexEntry>> {
        if let Some(pool) = &self.pool {
            let mut conditions = vec!["is_active = 1".to_string()];
            let mut params: Vec<Box<dyn sqlx::Encode<'_, Sqlite>>> = Vec::new();

            if let Some(project_id) = query.project_id {
                conditions.push("project_id = ?".to_string());
                params.push(Box::new(project_id.to_string()));
            }

            if let Some(entry_type) = query.entry_type {
                conditions.push("entry_type = ?".to_string());
                params.push(Box::new(entry_type.to_string()));
            }

            if let Some(search_term) = &query.search_term {
                conditions.push("(title LIKE ? OR content LIKE ?)".to_string());
                let search_pattern = format!("%{}%", search_term);
                params.push(Box::new(search_pattern.clone()));
                params.push(Box::new(search_pattern));
            }

            let where_clause = if conditions.is_empty() {
                String::new()
            } else {
                format!("WHERE {}", conditions.join(" AND "))
            };

            let order_by = match query.sort_by {
                Some(field) => format!("ORDER BY {} {}", field.to_string(), if query.sort_desc { "DESC" } else { "ASC" }),
                None => "ORDER BY updated_at DESC".to_string(),
            };

            let limit_clause = if let Some(limit) = query.limit {
                format!("LIMIT {}", limit)
            } else {
                String::new()
            };

            let sql = format!(
                "SELECT id, title, content, entry_type, project_id, created_at, updated_at FROM codex_entries {} {} {}",
                where_clause, order_by, limit_clause
            );

            let mut query_builder = sqlx::query(&sql);
            for param in params {
                query_builder = query_builder.bind(param);
            }

            let rows = query_builder.fetch_all(pool).await?;
            let mut entries = Vec::new();

            for row in rows {
                let entry = CodexEntry {
                    id: Uuid::parse_str(row.get::<&str, _>("id"))?,
                    title: row.get::<&str, _>("title").to_string(),
                    content: row.get::<&str, _>("content").to_string(),
                    entry_type: CodexEntryType::from_string(row.get::<&str, _>("entry_type"))?,
                    project_id: Uuid::parse_str(row.get::<&str, _>("project_id"))?,
                    created_at: Some(row.get::<&str, _>("created_at").to_string()),
                    updated_at: Some(row.get::<&str, _>("updated_at").to_string()),
                };
                entries.push(entry);
            }

            Ok(entries)
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Create a hierarchy item
    pub async fn create_hierarchy_item(&self, item: &crate::ui::tools::hierarchy_base::HierarchyItem) -> Result<()> {
        if let Some(pool) = &self.pool {
            let created_at = Utc::now().to_rfc3339();
            let updated_at = created_at.clone();

            sqlx::query(r#"
                INSERT INTO hierarchy_items (id, title, level, parent_id, position, project_id, created_at, updated_at, is_active)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#)
            .bind(&item.id)
            .bind(&item.title)
            .bind(item.level.to_string())
            .bind(item.parent_id.as_deref())
            .bind(item.position as i64)
            .bind(&item.project_id)
            .bind(created_at)
            .bind(updated_at)
            .bind(true)
            .execute(pool)
            .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Get hierarchy item by ID
    pub async fn get_hierarchy_item(&self, item_id: &str) -> Result<Option<crate::ui::tools::hierarchy_base::HierarchyItem>> {
        if let Some(pool) = &self.pool {
            let row = sqlx::query(r#"
                SELECT id, title, level, parent_id, position, project_id, created_at, updated_at
                FROM hierarchy_items
                WHERE id = ? AND is_active = 1
            "#)
            .bind(item_id)
            .fetch_optional(pool)
            .await?;

            if let Some(row) = row {
                let item = crate::ui::tools::hierarchy_base::HierarchyItem {
                    id: row.get::<&str, _>("id").to_string(),
                    title: row.get::<&str, _>("title").to_string(),
                    level: crate::ui::tools::hierarchy_base::HierarchyLevel::from_string(row.get::<&str, _>("level"))?,
                    parent_id: row.get::<Option<&str>, _>("parent_id").map(|s| s.to_string()),
                    position: row.get::<i64, _>("position") as u32,
                    project_id: row.get::<&str, _>("project_id").to_string(),
                    created_at: Some(row.get::<&str, _>("created_at").to_string()),
                    updated_at: Some(row.get::<&str, _>("updated_at").to_string()),
                    metadata: None,
                };
                Ok(Some(item))
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Update hierarchy item
    pub async fn update_hierarchy_item(&self, item: &crate::ui::tools::hierarchy_base::HierarchyItem) -> Result<()> {
        if let Some(pool) = &self.pool {
            let updated_at = Utc::now().to_rfc3339();

            sqlx::query(r#"
                UPDATE hierarchy_items
                SET title = ?, level = ?, parent_id = ?, position = ?, updated_at = ?
                WHERE id = ? AND is_active = 1
            "#)
            .bind(&item.title)
            .bind(item.level.to_string())
            .bind(item.parent_id.as_deref())
            .bind(item.position as i64)
            .bind(updated_at)
            .bind(&item.id)
            .execute(pool)
            .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Get children of a hierarchy item
    pub async fn get_hierarchy_children(&self, parent_id: Option<&str>) -> Result<Vec<crate::ui::tools::hierarchy_base::HierarchyItem>> {
        if let Some(pool) = &self.pool {
            let mut items = Vec::new();

            if let Some(parent_id) = parent_id {
                let rows = sqlx::query(r#"
                    SELECT id, title, level, parent_id, position, project_id, created_at, updated_at
                    FROM hierarchy_items
                    WHERE parent_id = ? AND is_active = 1
                    ORDER BY position, title
                "#)
                .bind(parent_id)
                .fetch_all(pool)
                .await?;

                for row in rows {
                    let item = crate::ui::tools::hierarchy_base::HierarchyItem {
                        id: row.get::<&str, _>("id").to_string(),
                        title: row.get::<&str, _>("title").to_string(),
                        level: crate::ui::tools::hierarchy_base::HierarchyLevel::from_string(row.get::<&str, _>("level"))?,
                        parent_id: Some(row.get::<&str, _>("parent_id").to_string()),
                        position: row.get::<i64, _>("position") as u32,
                        project_id: row.get::<&str, _>("project_id").to_string(),
                        created_at: Some(row.get::<&str, _>("created_at").to_string()),
                        updated_at: Some(row.get::<&str, _>("updated_at").to_string()),
                        metadata: None,
                    };
                    items.push(item);
                }
            } else {
                // Get root items (items with no parent)
                let rows = sqlx::query(r#"
                    SELECT id, title, level, parent_id, position, project_id, created_at, updated_at
                    FROM hierarchy_items
                    WHERE parent_id IS NULL AND is_active = 1
                    ORDER BY position, title
                "#)
                .fetch_all(pool)
                .await?;

                for row in rows {
                    let item = crate::ui::tools::hierarchy_base::HierarchyItem {
                        id: row.get::<&str, _>("id").to_string(),
                        title: row.get::<&str, _>("title").to_string(),
                        level: crate::ui::tools::hierarchy_base::HierarchyLevel::from_string(row.get::<&str, _>("level"))?,
                        parent_id: None,
                        position: row.get::<i64, _>("position") as u32,
                        project_id: row.get::<&str, _>("project_id").to_string(),
                        created_at: Some(row.get::<&str, _>("created_at").to_string()),
                        updated_at: Some(row.get::<&str, _>("updated_at").to_string()),
                        metadata: None,
                    };
                    items.push(item);
                }
            }

            Ok(items)
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Move hierarchy item to new parent
    pub async fn move_hierarchy_item(&self, item_id: &str, new_parent_id: Option<&str>) -> Result<()> {
        if let Some(pool) = &self.pool {
            let updated_at = Utc::now().to_rfc3339();

            sqlx::query(r#"
                UPDATE hierarchy_items
                SET parent_id = ?, updated_at = ?
                WHERE id = ? AND is_active = 1
            "#)
            .bind(new_parent_id)
            .bind(updated_at)
            .bind(item_id)
            .execute(pool)
            .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Create an analysis record
    pub async fn create_analysis_record(&self, analysis: &crate::ui::tools::analysis_tool_migrated::AnalysisRecord) -> Result<()> {
        if let Some(pool) = &self.pool {
            let created_at = analysis.created_at.as_deref().unwrap_or_else(|| Utc::now().to_rfc3339());
            let updated_at = analysis.updated_at.as_deref().unwrap_or_else(|| Utc::now().to_rfc3339());

            sqlx::query(r#"
                INSERT INTO analysis_records (id, project_id, analysis_type, content, created_at, updated_at, is_active)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            "#)
            .bind(&analysis.id)
            .bind(&analysis.project_id)
            .bind(&analysis.analysis_type)
            .bind(&analysis.content)
            .bind(created_at)
            .bind(updated_at)
            .bind(analysis.is_active)
            .execute(pool)
            .await?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Get analysis record by ID
    pub async fn get_analysis_record(&self, analysis_id: &str) -> Result<Option<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>> {
        if let Some(pool) = &self.pool {
            let row = sqlx::query(r#"
                SELECT id, project_id, analysis_type, content, created_at, updated_at, is_active
                FROM analysis_records
                WHERE id = ? AND is_active = 1
            "#)
            .bind(analysis_id)
            .fetch_optional(pool)
            .await?;

            if let Some(row) = row {
                let analysis = crate::ui::tools::analysis_tool_migrated::AnalysisRecord {
                    id: row.get::<&str, _>("id").to_string(),
                    project_id: row.get::<&str, _>("project_id").to_string(),
                    analysis_type: row.get::<&str, _>("analysis_type").to_string(),
                    content: row.get::<&str, _>("content").to_string(),
                    created_at: Some(row.get::<&str, _>("created_at").to_string()),
                    updated_at: Some(row.get::<&str, _>("updated_at").to_string()),
                    is_active: row.get::<bool, _>("is_active"),
                };
                Ok(Some(analysis))
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// List analysis records for a project
    pub async fn list_analysis_records(&self, project_id: &str) -> Result<Vec<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>> {
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(r#"
                SELECT id, project_id, analysis_type, content, created_at, updated_at, is_active
                FROM analysis_records
                WHERE project_id = ? AND is_active = 1
                ORDER BY created_at DESC
            "#)
            .bind(project_id)
            .fetch_all(pool)
            .await?;

            let mut analyses = Vec::new();

            for row in rows {
                let analysis = crate::ui::tools::analysis_tool_migrated::AnalysisRecord {
                    id: row.get::<&str, _>("id").to_string(),
                    project_id: row.get::<&str, _>("project_id").to_string(),
                    analysis_type: row.get::<&str, _>("analysis_type").to_string(),
                    content: row.get::<&str, _>("content").to_string(),
                    created_at: Some(row.get::<&str, _>("created_at").to_string()),
                    updated_at: Some(row.get::<&str, _>("updated_at").to_string()),
                    is_active: row.get::<bool, _>("is_active"),
                };
                analyses.push(analysis);
            }

            Ok(analyses)
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Get database connection pool
    pub fn get_pool(&self) -> Option<&SqlitePool> {
        self.pool.as_ref()
    }

    /// Get database statistics
    pub async fn get_database_stats(&self) -> Result<DatabaseStats> {
        if let Some(pool) = &self.pool {
            let codex_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM codex_entries WHERE is_active = 1")
                .fetch_one(pool)
                .await?;

            let hierarchy_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM hierarchy_items WHERE is_active = 1")
                .fetch_one(pool)
                .await?;

            let analysis_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM analysis_records WHERE is_active = 1")
                .fetch_one(pool)
                .await?;

            Ok(DatabaseStats {
                codex_entries: codex_count.0 as u64,
                hierarchy_items: hierarchy_count.0 as u64,
                analysis_records: analysis_count.0 as u64,
                total_size_mb: 0.0, // Would need filesystem access to calculate
                connection_count: pool.size() as u64,
            })
        } else {
            Err(anyhow::anyhow!("Database pool not initialized"))
        }
    }

    /// Close database connection
    pub async fn close(&self) -> Result<()> {
        if let Some(pool) = &self.pool {
            pool.close().await;
            Ok(())
        } else {
            Ok(())
        }
    }
}

/// Database statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub codex_entries: u64,
    pub hierarchy_items: u64,
    pub analysis_records: u64,
    pub total_size_mb: f64,
    pub connection_count: u64,
}

/// Extension trait to add real database operations to ToolDatabaseContext
#[async_trait]
pub trait RealDatabaseOperations {
    /// Create a codex entry
    async fn create_codex_entry(&mut self, entry: &CodexEntry) -> DatabaseResult<()>;

    /// Get a codex entry by ID
    async fn get_codex_entry(&self, entry_id: &str) -> DatabaseResult<Option<CodexEntry>>;

    /// Update a codex entry
    async fn update_codex_entry(&mut self, entry: &CodexEntry) -> DatabaseResult<()>;

    /// Delete a codex entry (soft delete)
    async fn delete_codex_entry(&mut self, entry_id: &str) -> DatabaseResult<()>;

    /// List codex entries with query parameters
    async fn list_codex_entries(&self, query: &CodexQuery) -> DatabaseResult<Vec<CodexEntry>>;

    /// Create a hierarchy item
    async fn create_hierarchy_item(&mut self, item: &crate::ui::tools::hierarchy_base::HierarchyItem) -> DatabaseResult<()>;

    /// Get a hierarchy item by ID
    async fn get_hierarchy_item(&self, item_id: &str) -> DatabaseResult<Option<crate::ui::tools::hierarchy_base::HierarchyItem>>;

    /// Update a hierarchy item
    async fn update_hierarchy_item(&mut self, item: &crate::ui::tools::hierarchy_base::HierarchyItem) -> DatabaseResult<()>;

    /// Get children of a hierarchy item
    async fn get_hierarchy_children(&self, parent_id: Option<&str>) -> DatabaseResult<Vec<crate::ui::tools::hierarchy_base::HierarchyItem>>;

    /// Move a hierarchy item to a new parent
    async fn move_hierarchy_item(&mut self, item_id: &str, new_parent_id: Option<&str>) -> DatabaseResult<()>;

    /// Create an analysis record
    async fn create_analysis_record(&mut self, analysis: &crate::ui::tools::analysis_tool_migrated::AnalysisRecord) -> DatabaseResult<()>;

    /// Get an analysis record by ID
    async fn get_analysis_record(&self, analysis_id: &str) -> DatabaseResult<Option<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>>;

    /// List analysis records for a project
    async fn list_analysis_records(&self, project_id: &str) -> DatabaseResult<Vec<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>>;

    /// Get database statistics
    async fn get_database_stats(&self) -> DatabaseResult<DatabaseStats>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_real_database_integration() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();

        let mut db = RealDatabaseIntegration::new(db_path);
        assert!(db.initialize().await.is_ok());

        assert!(db.get_pool().is_some());

        let stats = db.get_database_stats().await.unwrap();
        assert_eq!(stats.codex_entries, 0);
        assert_eq!(stats.hierarchy_items, 0);
        assert_eq!(stats.analysis_records, 0);

        assert!(db.close().await.is_ok());
    }

    #[tokio::test]
    async fn test_database_schema_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();

        let mut db = RealDatabaseIntegration::new(db_path);
        db.initialize().await.unwrap();

        // Test that tables were created by trying to query them
        let pool = db.get_pool().unwrap();

        // Test codex_entries table
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM codex_entries")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);

        // Test hierarchy_items table
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM hierarchy_items")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);

        // Test analysis_records table
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM analysis_records")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);

        db.close().await.unwrap();
    }
}
