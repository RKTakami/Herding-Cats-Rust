//! Analysis Service
//!
//! Database service for managing analysis data, providing CRUD operations
//! and integration with other writing tools through drag-and-drop functionality.

use sqlx::{self};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::database::{
    models::analysis::{AnalysisWithFields, *},
    DatabaseError, DatabaseResult, EnhancedDatabaseService,
};

/// Analysis service for managing analysis data
#[derive(Debug)]
pub struct AnalysisService {
    /// Enhanced database service
    pub db_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
}

impl Default for AnalysisService {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisService {
    /// Create a new analysis service
    pub fn new() -> Self {
        Self { db_service: None }
    }

    /// Initialize with database service
    pub fn with_database_service(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Self {
        Self {
            db_service: Some(db_service),
        }
    }

    /// Initialize analysis tables and types
    pub async fn initialize(&self) -> DatabaseResult<()> {
        Ok(())
    }

    /// Create a new analysis
    pub async fn create_analysis(
        &self,
        project_id: Uuid,
        title: String,
        description: String,
        writing_type: WritingType,
    ) -> DatabaseResult<Analysis> {
        let analysis = Analysis {
            id: Uuid::new_v4(),
            project_id,
            title,
            description,
            writing_type,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let db_service = self.db_service.as_ref().ok_or_else(|| {
            DatabaseError::Connection("Database service not initialized".to_string())
        })?;
        let db = db_service.read().await;

        sqlx::query(INSERT_ANALYSIS_SQL)
            .bind(analysis.id.to_string())
            .bind(analysis.project_id.to_string())
            .bind(&analysis.title)
            .bind(&analysis.description)
            .bind(analysis.writing_type as i32)
            .bind(analysis.created_at.to_rfc3339())
            .bind(analysis.updated_at.to_rfc3339())
            .execute(&db.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to create analysis: {}", e)))?;

        Ok(analysis)
    }

    /// Update an existing analysis
    pub async fn update_analysis(
        &self,
        analysis_id: Uuid,
        project_id: Uuid,
        title: String,
        description: String,
        writing_type: WritingType,
    ) -> DatabaseResult<Analysis> {
        let updated_at = chrono::Utc::now();

        let db_service = self.db_service.as_ref().ok_or_else(|| {
            DatabaseError::Connection("Database service not initialized".to_string())
        })?;
        let db = db_service.read().await;

        sqlx::query(UPDATE_ANALYSIS_SQL)
            .bind(analysis_id.to_string())
            .bind(&title)
            .bind(&description)
            .bind(writing_type as i32)
            .bind(updated_at.to_rfc3339())
            .bind(project_id.to_string())
            .execute(&db.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to update analysis: {}", e)))?;

        let updated_analysis = Analysis {
            id: analysis_id,
            project_id,
            title,
            description,
            writing_type,
            created_at: chrono::Utc::now(), // This would need to be fetched from DB
            updated_at,
        };

        Ok(updated_analysis)
    }

    /// Get analysis by ID
    pub async fn get_analysis(
        &self,
        _analysis_id: Uuid,
        _project_id: Uuid,
    ) -> DatabaseResult<Option<Analysis>> {
        // For now, return None - this would need proper implementation
        Ok(None)
    }

    /// Get all analyses for a project
    pub async fn get_analyses_by_project(
        &self,
        _project_id: Uuid,
        _limit: i32,
        _offset: i32,
    ) -> DatabaseResult<Vec<Analysis>> {
        // For now, return empty vector - this would need proper implementation
        Ok(Vec::new())
    }

    /// Delete an analysis
    pub async fn delete_analysis(
        &self,
        analysis_id: Uuid,
        project_id: Uuid,
    ) -> DatabaseResult<bool> {
        let db_service = self.db_service.as_ref().ok_or_else(|| {
            DatabaseError::Connection("Database service not initialized".to_string())
        })?;
        let db = db_service.read().await;

        let result = sqlx::query(DELETE_ANALYSIS_SQL)
            .bind(analysis_id.to_string())
            .bind(project_id.to_string())
            .execute(&db.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to delete analysis: {}", e)))?;

        Ok(result.rows_affected() > 0)
    }

    /// Create a new analysis field
    pub async fn create_analysis_field(
        &self,
        analysis_id: Uuid,
        field_type: AnalysisFieldType,
        content: String,
    ) -> DatabaseResult<AnalysisField> {
        let field = AnalysisField {
            id: Uuid::new_v4(),
            analysis_id,
            field_type,
            content,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let db_service = self.db_service.as_ref().ok_or_else(|| {
            DatabaseError::Connection("Database service not initialized".to_string())
        })?;
        let db = db_service.read().await;

        let field_type_int = field.field_type.clone() as i32;
        let field_id = field.id.to_string();
        let analysis_id = field.analysis_id.to_string();
        let content = field.content.clone();
        let created_at = field.created_at.to_rfc3339();
        let updated_at = field.updated_at.to_rfc3339();

        sqlx::query(INSERT_ANALYSIS_FIELD_SQL)
            .bind(&field_id)
            .bind(&analysis_id)
            .bind(field_type_int)
            .bind(&content)
            .bind(&created_at)
            .bind(&updated_at)
            .execute(&db.pool)
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to create analysis field: {}", e))
            })?;

        Ok(field)
    }

    /// Update an analysis field
    pub async fn update_analysis_field(
        &self,
        field_id: Uuid,
        analysis_id: Uuid,
        field_type: AnalysisFieldType,
        content: String,
    ) -> DatabaseResult<AnalysisField> {
        let updated_at = chrono::Utc::now();

        let db_service = self.db_service.as_ref().ok_or_else(|| {
            DatabaseError::Connection("Database service not initialized".to_string())
        })?;
        let db = db_service.read().await;

        let field_type_int = field_type.clone() as i32;
        let field_id_str = field_id.to_string();
        let analysis_id_str = analysis_id.to_string();
        let content_clone = content.clone();

        sqlx::query(UPDATE_ANALYSIS_FIELD_SQL)
            .bind(&field_id_str)
            .bind(field_type_int)
            .bind(&content_clone)
            .bind(updated_at.to_rfc3339())
            .bind(&analysis_id_str)
            .execute(&db.pool)
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to update analysis field: {}", e))
            })?;

        let updated_field = AnalysisField {
            id: field_id,
            analysis_id,
            field_type,
            content,
            created_at: chrono::Utc::now(),
            updated_at,
        };

        Ok(updated_field)
    }

    /// Get all fields for an analysis
    pub async fn get_analysis_fields(
        &self,
        _analysis_id: Uuid,
    ) -> DatabaseResult<Vec<AnalysisField>> {
        Ok(Vec::new())
    }

    /// Get analysis with its fields
    pub async fn get_analysis_with_fields(
        &self,
        _analysis_id: Uuid,
        _project_id: Uuid,
    ) -> DatabaseResult<Option<AnalysisWithFields>> {
        Ok(None)
    }

    /// Delete an analysis field
    pub async fn delete_analysis_field(
        &self,
        field_id: Uuid,
        analysis_id: Uuid,
    ) -> DatabaseResult<bool> {
        let db_service = self.db_service.as_ref().ok_or_else(|| {
            DatabaseError::Connection("Database service not initialized".to_string())
        })?;
        let db = db_service.read().await;

        let result = sqlx::query(DELETE_ANALYSIS_FIELD_SQL)
            .bind(field_id.to_string())
            .bind(analysis_id.to_string())
            .execute(&db.pool)
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to delete analysis field: {}", e))
            })?;

        Ok(result.rows_affected() > 0)
    }

    /// Search analysis fields across a project
    pub async fn search_analysis_fields(
        &self,
        _project_id: Uuid,
        _query: String,
        _limit: i32,
        _offset: i32,
    ) -> DatabaseResult<Vec<AnalysisField>> {
        Ok(Vec::new())
    }

    /// Get analysis statistics for a project
    pub async fn get_analysis_stats(&self, _project_id: Uuid) -> DatabaseResult<AnalysisStats> {
        Ok(AnalysisStats {
            total_analyses: 0,
            by_writing_type: std::collections::HashMap::new(),
            total_fields: 0,
            avg_fields_per_analysis: 0.0,
        })
    }

    /// Generate insights based on analysis fields
    fn _generate_insights(&self, fields: &[AnalysisField]) -> Vec<String> {
        let mut insights = Vec::new();

        // Count fields by type
        let mut field_counts = std::collections::HashMap::new();
        for field in fields {
            field_counts
                .entry(field.field_type.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        // Character insights
        let character_count = field_counts
            .get(&AnalysisFieldType::Character)
            .unwrap_or(&0);
        if *character_count < 2 {
            insights
                .push("Consider adding more characters to create richer interactions".to_string());
        } else if *character_count > 10 {
            insights.push(
                "You have many characters - consider consolidating to avoid confusion".to_string(),
            );
        }

        // Setting insights
        let setting_count = field_counts.get(&AnalysisFieldType::Setting).unwrap_or(&0);
        if *setting_count == 0 {
            insights.push("Consider adding setting details to ground your story".to_string());
        }

        // Plot insights
        let plot_count = field_counts
            .get(&AnalysisFieldType::PlotPoint)
            .unwrap_or(&0);
        if *plot_count < 3 {
            insights
                .push("Consider adding more plot points for better story structure".to_string());
        }

        // Theme insights
        let theme_count = field_counts.get(&AnalysisFieldType::Theme).unwrap_or(&0);
        if *theme_count == 0 {
            insights
                .push("Consider identifying central themes to give your work focus".to_string());
        }

        // Research-specific insights
        let thesis_count = field_counts
            .get(&AnalysisFieldType::ThesisStatement)
            .unwrap_or(&0);
        let evidence_count = field_counts.get(&AnalysisFieldType::Evidence).unwrap_or(&0);

        if *thesis_count == 0 {
            insights.push("Make sure to clearly state your thesis statement".to_string());
        }

        if *evidence_count < 3 {
            insights.push("Consider adding more evidence to support your argument".to_string());
        }

        insights
    }

    /// Import analysis from another tool (e.g., drag from Codex)
    pub async fn import_from_codex(
        &self,
        analysis_id: Uuid,
        codex_entries: Vec<(String, String)>,
    ) -> DatabaseResult<()> {
        for (entry_type, content) in codex_entries {
            let field_type = match entry_type.as_str() {
                "CharacterSheet" => AnalysisFieldType::Character,
                "Place" => AnalysisFieldType::Setting,
                "StorySummary" => AnalysisFieldType::Theme,
                _ => AnalysisFieldType::Body,
            };

            self.create_analysis_field(analysis_id, field_type, content)
                .await?;
        }

        Ok(())
    }

    /// Export analysis summary
    pub async fn export_analysis_summary(
        &self,
        analysis_id: Uuid,
        project_id: Uuid,
    ) -> DatabaseResult<String> {
        let analysis_with_fields = match self
            .get_analysis_with_fields(analysis_id, project_id)
            .await?
        {
            Some(awf) => awf,
            None => return Err(DatabaseError::NotFound("Analysis not found".to_string())),
        };

        let mut summary = format!(
            "# Analysis Summary: {}\n\n",
            analysis_with_fields.analysis.title
        );
        summary.push_str(&format!(
            "**Description:** {}\n",
            analysis_with_fields.analysis.description
        ));
        summary.push_str(&format!(
            "**Writing Type:** {}\n",
            analysis_with_fields.analysis.writing_type.display_name()
        ));
        summary.push_str(&format!(
            "**Created:** {}\n\n",
            analysis_with_fields
                .analysis
                .created_at
                .format("%Y-%m-%d %H:%M:%S")
        ));

        summary.push_str("## Analysis Fields\n\n");
        for field in &analysis_with_fields.fields {
            summary.push_str(&format!(
                "### {} - {}\n",
                field.field_type.display_name(),
                field.content
            ));
        }

        if !analysis_with_fields.insights.is_empty() {
            summary.push_str("\n## Generated Insights\n\n");
            for insight in &analysis_with_fields.insights {
                summary.push_str(&format!("- {}\n", insight));
            }
        }

        Ok(summary)
    }
}
