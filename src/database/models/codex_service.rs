//! Codex Database Service
//!
//! Provides database operations for codex entries including CRUD operations,
//! querying, and data management for the five codex categories:
//! Story Summary, Character Sheets, Objects, Time, and Place.

use async_trait::async_trait;
use chrono::Utc;
use sqlx::Row;
use uuid::Uuid;

// Import types from the correct module path
use crate::{
    database::models::codex::{
        CodexEntry, CodexEntryType, CodexQuery, CodexSortField, CodexStatistics, CodexStatus,
        EnhancedCodexEntry,
    },
    database::{DatabaseError, DatabaseResult},
};

/// Service interface for codex operations
#[async_trait]
pub trait CodexService {
    /// Initialize the codex database schema
    async fn initialize_schema(&self) -> DatabaseResult<()>;

    /// Create a new codex entry
    async fn create_entry(&self, entry: &CodexEntry) -> DatabaseResult<Uuid>;

    /// Get a codex entry by ID
    async fn get_entry(&self, entry_id: &Uuid) -> DatabaseResult<Option<CodexEntry>>;

    /// Update an existing codex entry
    async fn update_entry(&self, entry: &CodexEntry) -> DatabaseResult<()>;

    /// Delete a codex entry (soft delete)
    async fn delete_entry(&self, entry_id: &Uuid) -> DatabaseResult<()>;

    /// List codex entries with optional filtering
    async fn list_entries(&self, query: &CodexQuery) -> DatabaseResult<Vec<CodexEntry>>;

    /// Count codex entries matching query
    async fn count_entries(&self, query: &CodexQuery) -> DatabaseResult<i64>;

    /// Get statistics about codex data
    async fn get_statistics(&self, project_id: &Uuid) -> DatabaseResult<CodexStatistics>;

    /// Create an enhanced codex entry with type-specific data
    async fn create_enhanced_entry(&self, entry: &EnhancedCodexEntry) -> DatabaseResult<Uuid>;

    /// Get an enhanced codex entry by ID
    async fn get_enhanced_entry(
        &self,
        entry_id: &Uuid,
    ) -> DatabaseResult<Option<EnhancedCodexEntry>>;

    /// Search codex entries by text content
    async fn search_entries(
        &self,
        project_id: &Uuid,
        search_term: &str,
    ) -> DatabaseResult<Vec<CodexEntry>>;
}

/// Implementation of codex database service
pub struct CodexDatabaseService {
    // Using sqlx pool directly for better async support
    pool: sqlx::SqlitePool,
}

impl CodexDatabaseService {
    /// Create a new codex database service
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new codex database service with database path
    pub async fn from_path(db_path: &std::path::Path) -> DatabaseResult<Self> {
        let config = crate::database::enhanced_database_sqlx::DatabaseConfig::default();
        let db_service =
            crate::database::enhanced_database_sqlx::EnhancedDatabaseService::new(db_path, config)
                .await?;
        Ok(Self {
            pool: db_service.pool,
        })
    }
}

#[async_trait]
impl CodexService for CodexDatabaseService {
    async fn initialize_schema(&self) -> DatabaseResult<()> {
        // Note: With sqlx, schema initialization would be handled differently
        // For now, we'll assume the schema is already created
        log::info!("✅ Codex database schema initialization (using sqlx)");
        Ok(())
    }

    async fn create_entry(&self, entry: &CodexEntry) -> DatabaseResult<Uuid> {
        let entry_type_str = match entry.entry_type {
            CodexEntryType::StorySummary => "story_summary",
            CodexEntryType::CharacterSheet => "character_sheet",
            CodexEntryType::Object => "object",
            CodexEntryType::Time => "time",
            CodexEntryType::Place => "place",
        };

        let status_str = match entry.status {
            CodexStatus::Draft => "draft",
            CodexStatus::InReview => "in_review",
            CodexStatus::Final => "final",
            CodexStatus::Archived => "archived",
        };

        let metadata_str = entry.metadata.as_deref().unwrap_or("");

        sqlx::query(
            r#"
            INSERT INTO codex_entries (
                id, project_id, entry_type, title, content, status,
                created_at, updated_at, is_active, metadata, sort_order
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(entry.id.to_string())
        .bind(entry.project_id.to_string())
        .bind(entry_type_str)
        .bind(&entry.title)
        .bind(&entry.content)
        .bind(status_str)
        .bind(entry.created_at.to_rfc3339())
        .bind(entry.updated_at.to_rfc3339())
        .bind(entry.is_active)
        .bind(metadata_str)
        .bind(entry.sort_order)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to create codex entry: {}", e)))?;

        log::info!(
            "✅ Created codex entry: {} ({})",
            entry.title,
            entry.entry_type.display_name()
        );
        Ok(entry.id)
    }

    async fn get_entry(&self, entry_id: &Uuid) -> DatabaseResult<Option<CodexEntry>> {
        let row = sqlx::query(
            r#"
            SELECT id, project_id, entry_type, title, content, status,
                   created_at, updated_at, is_active, metadata, sort_order
            FROM codex_entries
            WHERE id = ?
            AND is_active = 1
            "#,
        )
        .bind(entry_id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get codex entry: {}", e)))?;

        if let Some(row) = row {
            self.row_to_entry(&row).map(Some)
        } else {
            Ok(None)
        }
    }

    async fn update_entry(&self, entry: &CodexEntry) -> DatabaseResult<()> {
        let entry_type_str = match entry.entry_type {
            CodexEntryType::StorySummary => "story_summary",
            CodexEntryType::CharacterSheet => "character_sheet",
            CodexEntryType::Object => "object",
            CodexEntryType::Time => "time",
            CodexEntryType::Place => "place",
        };

        let status_str = match entry.status {
            CodexStatus::Draft => "draft",
            CodexStatus::InReview => "in_review",
            CodexStatus::Final => "final",
            CodexStatus::Archived => "archived",
        };

        let metadata_str = entry.metadata.as_deref().unwrap_or("");

        sqlx::query(
            r#"
            UPDATE codex_entries
            SET title = ?, content = ?, entry_type = ?, status = ?,
                updated_at = ?, metadata = ?, sort_order = ?
            WHERE id = ?
            AND is_active = 1
            "#,
        )
        .bind(&entry.title)
        .bind(&entry.content)
        .bind(entry_type_str)
        .bind(status_str)
        .bind(entry.updated_at.to_rfc3339())
        .bind(metadata_str)
        .bind(entry.sort_order)
        .bind(entry.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to update codex entry: {}", e)))?;

        log::info!(
            "✅ Updated codex entry: {} ({})",
            entry.title,
            entry.entry_type.display_name()
        );
        Ok(())
    }

    async fn delete_entry(&self, entry_id: &Uuid) -> DatabaseResult<()> {
        sqlx::query("UPDATE codex_entries SET is_active = 0 WHERE id = ?")
            .bind(entry_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to delete codex entry: {}", e)))?;

        log::info!("✅ Deleted codex entry: {}", entry_id);
        Ok(())
    }

    async fn list_entries(&self, query: &CodexQuery) -> DatabaseResult<Vec<CodexEntry>> {
        // For now, use a simpler approach that matches other working code in the project
        let mut sql = r#"
        SELECT id, project_id, entry_type, title, content, status,
               created_at, updated_at, is_active, metadata, sort_order
        FROM codex_entries
        WHERE is_active = 1
        "#
        .to_string();

        let mut params: Vec<String> = Vec::new();

        // Apply filters
        if let Some(project_id) = &query.project_id {
            sql.push_str(" AND project_id = ?");
            params.push(project_id.to_string());
        }

        if let Some(entry_type) = query.entry_type {
            sql.push_str(" AND entry_type = ?");
            let entry_type_str = match entry_type {
                CodexEntryType::StorySummary => "story_summary",
                CodexEntryType::CharacterSheet => "character_sheet",
                CodexEntryType::Object => "object",
                CodexEntryType::Time => "time",
                CodexEntryType::Place => "place",
            };
            params.push(entry_type_str.to_string());
        }

        if let Some(status) = query.status {
            sql.push_str(" AND status = ?");
            let status_str = match status {
                CodexStatus::Draft => "draft",
                CodexStatus::InReview => "in_review",
                CodexStatus::Final => "final",
                CodexStatus::Archived => "archived",
            };
            params.push(status_str.to_string());
        }

        if let Some(search_term) = &query.search_term {
            sql.push_str(" AND (title LIKE ? OR content LIKE ?)");
            let search_pattern = format!("%{}%", search_term);
            params.push(search_pattern.clone());
            params.push(search_pattern);
        }

        // Apply sorting
        if let Some(sort_field) = query.sort_by {
            sql.push_str(" ORDER BY ");
            match sort_field {
                CodexSortField::Title => sql.push_str("title"),
                CodexSortField::CreatedAt => sql.push_str("created_at"),
                CodexSortField::UpdatedAt => sql.push_str("updated_at"),
                CodexSortField::SortOrder => sql.push_str("sort_order"),
                CodexSortField::Status => sql.push_str("status"),
                CodexSortField::EntryType => sql.push_str("entry_type"),
            }

            if query.sort_desc {
                sql.push_str(" DESC");
            }
        }

        // Apply pagination
        if let Some(limit) = query.limit {
            sql.push_str(" LIMIT ?");
            params.push(limit.to_string());

            sql.push_str(" OFFSET ?");
            params.push(query.offset.to_string());
        }

        let mut query_builder = sqlx::query(&sql);
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to list codex entries: {}", e)))?;

        let mut entries = Vec::new();
        for row in rows {
            if let Ok(entry) = self.row_to_entry(&row) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    async fn count_entries(&self, query: &CodexQuery) -> DatabaseResult<i64> {
        let mut sql = "SELECT COUNT(*) FROM codex_entries WHERE is_active = 1".to_string();
        let mut params: Vec<String> = Vec::new();

        // Apply same filters as list_entries
        if let Some(project_id) = &query.project_id {
            sql.push_str(" AND project_id = ?");
            params.push(project_id.to_string());
        }

        if let Some(entry_type) = query.entry_type {
            sql.push_str(" AND entry_type = ?");
            let entry_type_str = match entry_type {
                CodexEntryType::StorySummary => "story_summary",
                CodexEntryType::CharacterSheet => "character_sheet",
                CodexEntryType::Object => "object",
                CodexEntryType::Time => "time",
                CodexEntryType::Place => "place",
            };
            params.push(entry_type_str.to_string());
        }

        if let Some(status) = query.status {
            sql.push_str(" AND status = ?");
            let status_str = match status {
                CodexStatus::Draft => "draft",
                CodexStatus::InReview => "in_review",
                CodexStatus::Final => "final",
                CodexStatus::Archived => "archived",
            };
            params.push(status_str.to_string());
        }

        if let Some(search_term) = &query.search_term {
            sql.push_str(" AND (title LIKE ? OR content LIKE ?)");
            let search_pattern = format!("%{}%", search_term);
            params.push(search_pattern.clone());
            params.push(search_pattern);
        }

        let mut query_builder = sqlx::query(&sql);
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        let row = query_builder
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to count entries: {}", e)))?;

        let count: i64 = row.get(0);
        Ok(count)
    }

    async fn get_statistics(&self, project_id: &Uuid) -> DatabaseResult<CodexStatistics> {
        // Count total entries
        let row = sqlx::query(
            "SELECT COUNT(*) FROM codex_entries WHERE project_id = ? AND is_active = 1",
        )
        .bind(project_id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to count total entries: {}", e)))?;
        let total_entries: i64 = row.get(0);

        // Count by type
        let type_rows = sqlx::query(
            r#"
            SELECT entry_type, COUNT(*)
            FROM codex_entries
            WHERE project_id = ? AND is_active = 1
            GROUP BY entry_type
            "#,
        )
        .bind(project_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to count by type: {}", e)))?;

        let mut entries_by_type = std::collections::HashMap::new();
        for row in type_rows {
            let entry_type: String = row.get(0);
            let count: i64 = row.get(1);
            entries_by_type.insert(entry_type, count);
        }

        // Count by status
        let status_rows = sqlx::query(
            r#"
            SELECT status, COUNT(*)
            FROM codex_entries
            WHERE project_id = ? AND is_active = 1
            GROUP BY status
            "#,
        )
        .bind(project_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to count by status: {}", e)))?;

        let mut entries_by_status = std::collections::HashMap::new();
        for row in status_rows {
            let status: String = row.get(0);
            let count: i64 = row.get(1);
            entries_by_status.insert(status, count);
        }

        // Calculate word count
        let row = sqlx::query(
            r#"
            SELECT SUM(LENGTH(content) - LENGTH(REPLACE(content, ' ', '')) + 1)
            FROM codex_entries
            WHERE project_id = ? AND is_active = 1
            "#,
        )
        .bind(project_id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to calculate word count: {}", e)))?;
        let total_word_count: Option<i64> = row.get(0);
        let total_word_count = total_word_count.unwrap_or(0);

        // Get last updated
        let row = sqlx::query(
            r#"
            SELECT MAX(updated_at)
            FROM codex_entries
            WHERE project_id = ? AND is_active = 1
            "#,
        )
        .bind(project_id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get last updated: {}", e)))?;
        let last_updated: Option<String> = row.get(0);
        let last_updated = last_updated.and_then(|s| s.parse().ok());

        Ok(CodexStatistics {
            total_entries,
            entries_by_type,
            entries_by_status,
            total_word_count,
            last_updated,
            projects_breakdown: std::collections::HashMap::new(), // Would need more complex query
        })
    }

    async fn create_enhanced_entry(&self, entry: &EnhancedCodexEntry) -> DatabaseResult<Uuid> {
        // Create the base entry first
        let entry_id = self.create_entry(&entry.base).await;

        // TODO: Store enhanced data in separate tables
        // This would require additional tables for character_data, place_data, etc.

        Ok(entry_id?)
    }

    async fn get_enhanced_entry(
        &self,
        entry_id: &Uuid,
    ) -> DatabaseResult<Option<EnhancedCodexEntry>> {
        if let Some(base_entry) = self.get_entry(entry_id).await? {
            // TODO: Load enhanced data from separate tables
            let mut enhanced_entry = EnhancedCodexEntry::new(
                base_entry.project_id,
                base_entry.entry_type,
                base_entry.title.clone(),
                base_entry.content.clone(),
            );

            // Copy base data - clone to avoid partial move
            let new_entry = base_entry;
            enhanced_entry.base = new_entry;

            Ok(Some(enhanced_entry))
        } else {
            Ok(None)
        }
    }

    async fn search_entries(
        &self,
        project_id: &Uuid,
        search_term: &str,
    ) -> DatabaseResult<Vec<CodexEntry>> {
        let mut query = CodexQuery::default();
        query.project_id = Some(*project_id);
        query.search_term = Some(search_term.to_string());
        query.sort_by = Some(CodexSortField::Title);
        query.sort_desc = false;

        self.list_entries(&query).await
    }
}

impl CodexDatabaseService {
    /// Convert a sqlx::Row to a CodexEntry
    fn row_to_entry(&self, row: &sqlx::sqlite::SqliteRow) -> DatabaseResult<CodexEntry> {
        use chrono::DateTime;

        let id_str: String = row.get(0);
        let id = id_str
            .parse::<Uuid>()
            .map_err(|e| DatabaseError::ValidationError(format!("Invalid UUID: {}", e)))?;

        let project_id_str: String = row.get(1);
        let project_id = project_id_str
            .parse::<Uuid>()
            .map_err(|e| DatabaseError::ValidationError(format!("Invalid project UUID: {}", e)))?;

        let entry_type_str: String = row.get(2);
        let entry_type = match entry_type_str.as_str() {
            "story_summary" => CodexEntryType::StorySummary,
            "character_sheet" => CodexEntryType::CharacterSheet,
            "object" => CodexEntryType::Object,
            "time" => CodexEntryType::Time,
            "place" => CodexEntryType::Place,
            _ => {
                return Err(DatabaseError::ValidationError(format!(
                    "Unknown entry type: {}",
                    entry_type_str
                )))
            }
        };

        let title: String = row.get(3);
        let content: String = row.get(4);

        let status_str: String = row.get(5);
        let status = match status_str.as_str() {
            "draft" => CodexStatus::Draft,
            "in_review" => CodexStatus::InReview,
            "final" => CodexStatus::Final,
            "archived" => CodexStatus::Archived,
            _ => {
                return Err(DatabaseError::ValidationError(format!(
                    "Unknown status: {}",
                    status_str
                )))
            }
        };

        let created_at_str: String = row.get(6);
        let created_at: DateTime<Utc> = created_at_str
            .parse()
            .map_err(|e| DatabaseError::ValidationError(format!("Invalid datetime: {}", e)))?;

        let updated_at_str: String = row.get(7);
        let updated_at: DateTime<Utc> = updated_at_str
            .parse()
            .map_err(|e| DatabaseError::ValidationError(format!("Invalid datetime: {}", e)))?;

        let is_active: bool = row.get(8);

        let metadata_str: String = row.get(9);
        let metadata = if metadata_str.is_empty() {
            None
        } else {
            Some(metadata_str)
        };

        let sort_order: i32 = row.get(10);

        Ok(CodexEntry {
            id,
            project_id,
            entry_type,
            title,
            content,
            status,
            created_at,
            updated_at,
            is_active,
            metadata,
            sort_order,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_type_display_names() {
        assert_eq!(CodexEntryType::StorySummary.display_name(), "Story Summary");
        assert_eq!(
            CodexEntryType::CharacterSheet.display_name(),
            "Character Sheet"
        );
        assert_eq!(CodexEntryType::Object.display_name(), "Object");
        assert_eq!(CodexEntryType::Time.display_name(), "Time");
        assert_eq!(CodexEntryType::Place.display_name(), "Place");
    }

    #[test]
    fn test_status_display_names() {
        assert_eq!(CodexStatus::Draft.display_name(), "Draft");
        assert_eq!(CodexStatus::InReview.display_name(), "In Review");
        assert_eq!(CodexStatus::Final.display_name(), "Final");
        assert_eq!(CodexStatus::Archived.display_name(), "Archived");
    }
}
