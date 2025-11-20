//! Project Management Service - Complete implementation
//!
//! Provides comprehensive project management functionality including
//! project lifecycle management, data isolation, statistics tracking,
//! and settings management.

use crate::error::DatabaseError;
use crate::error::DatabaseResult;
use crate::EnhancedDatabaseService;
use crate::Project;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Project settings structure
#[derive(Debug, Clone)]
pub struct ProjectSettings {
    pub auto_save_enabled: bool,
    pub auto_save_interval: u32,
    pub backup_enabled: bool,
    pub search_enabled: bool,
    pub theme: String,
    pub font_size: u32,
}

/// Project statistics
#[derive(Debug, Clone, Default)]
pub struct ProjectStatistics {
    pub document_count: usize,
    pub total_words: usize,
    pub storage_size: usize,
    pub last_document_update: Option<chrono::DateTime<chrono::Utc>>,
    pub active_embeddings: usize,
    pub total_backups: usize,
}

/// Project management service with database integration
#[derive(Debug)]
pub struct ProjectManagementService {
    db_service: Arc<RwLock<EnhancedDatabaseService>>,
}

impl ProjectManagementService {
    /// Create a new project management service
    pub fn new(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Self {
        Self { db_service }
    }

    /// Create a new project
    pub async fn create_project(
        &self,
        name: String,
        description: Option<String>,
    ) -> DatabaseResult<String> {
        let db_service = self.db_service.read().await;
        let project_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let description_str = description.as_deref().unwrap_or("");

        sqlx::query(
            "INSERT INTO projects (id, name, description, created_at, updated_at, is_archived, is_active, settings)
             VALUES (?1, ?2, ?3, ?4, ?5, 0, 0, NULL)"
        )
        .bind(&project_id)
        .bind(&name)
        .bind(description_str)
        .bind(&now)
        .bind(&now)
        .execute(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to create project: {}", e)))?;

        Ok(project_id)
    }

    /// Get project by ID
    pub async fn get_project(&self, project_id: &Uuid) -> DatabaseResult<Option<Project>> {
        let db_service = self.db_service.read().await;

        let result: Option<(
            String,
            String,
            Option<String>,
            String,
            String,
            bool,
            bool,
            Option<String>,
        )> = sqlx::query_as(
            "SELECT id, name, description, created_at, updated_at, is_archived, is_active, settings
             FROM projects WHERE id = ?1",
        )
        .bind(project_id.to_string())
        .fetch_optional(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get project: {}", e)))?;

        match result {
            Some((
                id,
                name,
                description,
                created_at_str,
                updated_at_str,
                is_archived,
                is_active,
                settings,
            )) => {
                // Handle NULL or empty datetime values
                let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now);

                let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now);

                Ok(Some(Project {
                    id: Uuid::parse_str(&id)
                        .map_err(|e| DatabaseError::Service(format!("Invalid UUID: {}", e)))?,
                    name,
                    description,
                    created_at,
                    updated_at,
                    is_archived,
                    is_active,
                    settings,
                }))
            }
            None => Ok(None),
        }
    }

    /// Get all projects
    pub async fn get_all_projects(&self) -> DatabaseResult<Vec<Project>> {
        let db_service = self.db_service.read().await;

        let rows: Vec<(
            String,
            String,
            Option<String>,
            String,
            String,
            bool,
            bool,
            Option<String>,
        )> = sqlx::query_as(
            "SELECT id, name, description, created_at, updated_at, is_archived, is_active, settings
             FROM projects ORDER BY updated_at DESC",
        )
        .fetch_all(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get projects: {}", e)))?;

        let mut projects = Vec::new();
        for (
            id,
            name,
            description,
            created_at_str,
            updated_at_str,
            is_archived,
            is_active,
            settings,
        ) in rows
        {
            // Handle NULL or empty datetime values
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(chrono::Utc::now);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(chrono::Utc::now);

            projects.push(Project {
                id: Uuid::parse_str(&id)
                    .map_err(|e| DatabaseError::Service(format!("Invalid UUID: {}", e)))?,
                name,
                description,
                created_at,
                updated_at,
                is_archived,
                is_active,
                settings,
            });
        }

        Ok(projects)
    }

    /// Set active project (only one active project at a time)
    pub async fn set_active_project(&self, project_id: &Uuid) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;

        // Deactivate all projects
        sqlx::query("UPDATE projects SET is_active = 0")
            .execute(&db_service.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to deactivate projects: {}", e)))?;

        // Activate the specified project
        sqlx::query("UPDATE projects SET is_active = 1 WHERE id = ?1")
            .bind(project_id.to_string())
            .execute(&db_service.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to activate project: {}", e)))?;

        Ok(())
    }

    /// Archive a project
    pub async fn archive_project(&self, project_id: &Uuid) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE projects SET is_archived = 1, is_active = 0, updated_at = ?1 WHERE id = ?2",
        )
        .bind(&now)
        .bind(project_id.to_string())
        .execute(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to archive project: {}", e)))?;

        Ok(())
    }

    /// Delete a project with data cleanup
    pub async fn delete_project(&self, project_id: &Uuid) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;

        // The CASCADE foreign key constraints will handle cleanup of:
        // - documents (with their embeddings, versions, etc.)
        // - document_embeddings
        // - document_versions
        // - change_log entries
        // - project_settings
        // - backup_metadata

        sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(project_id.to_string())
            .execute(&db_service.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to delete project: {}", e)))?;

        Ok(())
    }

    /// Get project settings as structured data
    pub async fn get_project_settings(
        &self,
        project_id: &Uuid,
    ) -> DatabaseResult<Option<ProjectSettings>> {
        let db_service = self.db_service.read().await;

        let rows: Vec<(String, String, String)> = sqlx::query_as(
            "SELECT setting_key, setting_value, setting_type FROM project_settings WHERE project_id = ?1"
        )
        .bind(project_id.to_string())
        .fetch_all(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get project settings: {}", e)))?;

        let mut settings_map = std::collections::HashMap::new();
        for (key, value, _type) in rows {
            settings_map.insert(key, value);
        }

        if settings_map.is_empty() {
            return Ok(None);
        }

        // Convert to ProjectSettings structure
        let project_settings = ProjectSettings {
            auto_save_enabled: settings_map
                .get("auto_save_enabled")
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(true),
            auto_save_interval: settings_map
                .get("auto_save_interval")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(300), // 5 minutes default
            backup_enabled: settings_map
                .get("backup_enabled")
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(true),
            search_enabled: settings_map
                .get("search_enabled")
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(true),
            theme: settings_map
                .get("theme")
                .cloned()
                .unwrap_or_else(|| "default".to_string()),
            font_size: settings_map
                .get("font_size")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(14),
        };

        Ok(Some(project_settings))
    }

    /// Update project settings
    pub async fn update_project_settings(
        &self,
        project_id: &Uuid,
        settings: &ProjectSettings,
    ) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;
        let now = chrono::Utc::now().to_rfc3339();

        // Define settings to update
        let settings_to_update = vec![
            ("auto_save_enabled", settings.auto_save_enabled.to_string()),
            (
                "auto_save_interval",
                settings.auto_save_interval.to_string(),
            ),
            ("backup_enabled", settings.backup_enabled.to_string()),
            ("search_enabled", settings.search_enabled.to_string()),
            ("theme", settings.theme.clone()),
            ("font_size", settings.font_size.to_string()),
        ];

        for (key, value) in settings_to_update {
            // Use INSERT OR REPLACE to handle both insert and update
            sqlx::query(
                "INSERT OR REPLACE INTO project_settings (id, project_id, setting_key, setting_value, setting_type, created_at, updated_at)
                 VALUES (COALESCE((SELECT id FROM project_settings WHERE project_id = ?1 AND setting_key = ?2), lower(hex(randomblob(16)))), ?1, ?2, ?3, 'string', ?4, ?4)"
            )
            .bind(project_id.to_string())
            .bind(key)
            .bind(&value)
            .bind(&now)
            .execute(&db_service.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to update project setting {}: {}", key, e)))?;
        }

        Ok(())
    }

    /// Reset project settings to defaults
    pub async fn reset_project_settings(&self, project_id: &Uuid) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;

        sqlx::query("DELETE FROM project_settings WHERE project_id = ?1")
            .bind(project_id.to_string())
            .execute(&db_service.pool)
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to reset project settings: {}", e))
            })?;

        Ok(())
    }

    /// Get comprehensive project statistics
    pub async fn get_project_statistics(
        &self,
        project_id: &Uuid,
    ) -> DatabaseResult<ProjectStatistics> {
        let db_service = self.db_service.read().await;

        // Document count
        let document_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM documents WHERE project_id = ?1 AND is_active = 1",
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get document count: {}", e)))?;

        // Total words
        let total_words: Option<i64> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(word_count), 0) FROM documents WHERE project_id = ?1 AND is_active = 1"
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get total words: {}", e)))?;

        // Storage size (content length)
        let storage_size: Option<i64> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(LENGTH(content)), 0) FROM documents WHERE project_id = ?1 AND is_active = 1"
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get storage size: {}", e)))?;

        // Last document update
        let last_document_update_str: Option<String> = sqlx::query_scalar(
            "SELECT MAX(updated_at) FROM documents WHERE project_id = ?1 AND is_active = 1",
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get last update: {}", e)))?;

        let last_document_update = if let Some(dt_str) = last_document_update_str {
            chrono::DateTime::parse_from_rfc3339(&dt_str)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        } else {
            None
        };

        // Active embeddings count
        let active_embeddings: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM document_embeddings
             WHERE document_id IN (SELECT id FROM documents WHERE project_id = ?1 AND is_active = 1)"
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get embeddings count: {}", e)))?;

        // Total backups count
        let total_backups: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM backup_metadata WHERE project_id = ?1")
                .bind(project_id.to_string())
                .fetch_one(&db_service.pool)
                .await
                .map_err(|e| {
                    DatabaseError::Service(format!("Failed to get backups count: {}", e))
                })?;

        Ok(ProjectStatistics {
            document_count: document_count as usize,
            total_words: total_words.unwrap_or(0) as usize,
            storage_size: storage_size.unwrap_or(0) as usize,
            last_document_update,
            active_embeddings: active_embeddings as usize,
            total_backups: total_backups as usize,
        })
    }

    /// Calculate advanced project metrics
    pub async fn calculate_project_metrics(
        &self,
        project_id: &Uuid,
    ) -> DatabaseResult<std::collections::HashMap<String, Value>> {
        let db_service = self.db_service.read().await;

        let mut metrics = std::collections::HashMap::new();

        // Average document size
        let avg_doc_size: Option<f64> = sqlx::query_scalar(
            "SELECT COALESCE(AVG(LENGTH(content)), 0) FROM documents WHERE project_id = ?1 AND is_active = 1"
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get average document size: {}", e)))?;

        // Most active time (by hour)
        let most_active_hour: Option<i64> = sqlx::query_scalar(
            "SELECT COALESCE(CAST(strftime('%H', MAX(updated_at)) AS INTEGER), 0) FROM documents WHERE project_id = ?1 AND is_active = 1"
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get most active hour: {}", e)))?;

        // Document type distribution
        let type_rows: Vec<(String, i64)> = sqlx::query_as(
            "SELECT document_type, COUNT(*) FROM documents WHERE project_id = ?1 AND is_active = 1 GROUP BY document_type"
        )
        .bind(project_id.to_string())
        .fetch_all(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get document types: {}", e)))?;

        let mut doc_type_counts = std::collections::HashMap::new();
        for (doc_type, count) in type_rows {
            doc_type_counts.insert(doc_type, Value::from(count as u32));
        }

        // Convert HashMap to serde_json::Map
        let doc_type_distribution = serde_json::Map::from_iter(doc_type_counts);

        // Weekly activity (documents created per week for last 4 weeks)
        let weekly_activity: Option<i64> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM documents
             WHERE project_id = ?1 AND is_active = 1 AND created_at >= date('now', '-4 weeks')",
        )
        .bind(project_id.to_string())
        .fetch_one(&db_service.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get weekly activity: {}", e)))?;

        // Build metrics object
        metrics.insert(
            "average_document_size".to_string(),
            Value::from(avg_doc_size.unwrap_or(0.0)),
        );
        metrics.insert(
            "most_active_hour".to_string(),
            Value::from(most_active_hour.unwrap_or(0) as u32),
        );
        metrics.insert(
            "document_type_distribution".to_string(),
            Value::Object(doc_type_distribution),
        );
        metrics.insert(
            "weekly_activity".to_string(),
            Value::from(weekly_activity.unwrap_or(0) as u32),
        );
        metrics.insert(
            "data_collected_at".to_string(),
            Value::from(chrono::Utc::now().to_rfc3339()),
        );

        Ok(metrics)
    }
}
