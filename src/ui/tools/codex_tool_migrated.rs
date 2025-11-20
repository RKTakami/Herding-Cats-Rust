//! Migrated Codex Tool Implementation
//!
//! This module provides the migrated version of the codex tool using the new
//! unified architecture patterns with ToolDatabaseContext, ThreadSafeToolRegistry,
//! and ToolApiContract.

use crate::ui::tools::{
    database_integration::{ToolDatabaseContext, DatabaseOperationResult},
    threading_patterns::get_tool_registry,
    api_contracts::{ToolApiContract, get_api_contract, ToolLifecycleEvent},
    ToolIntegration, ToolType,
};
use anyhow::Result;
use std::sync::Arc;
use std::time::{Instant, Duration};
use tokio::sync::RwLock;
use async_trait::async_trait;

use super::codex_base::{CodexToolBase, CodexEntry, CodexEntryType};
use super::codex_database_service::CodexDatabaseService;

/// Migrated codex tool that implements the new architecture patterns
pub struct MigratedCodexTool {
    /// Database context for safe database operations
    database_context: Option<ToolDatabaseContext>,
    /// API contract for cross-tool communication
    api_contract: Arc<ToolApiContract>,
    /// Base codex functionality
    base: CodexToolBase,
    /// Tool registry reference for lifecycle management
    tool_registry: &'static crate::ui::tools::threading_patterns::ThreadSafeToolRegistry,
    /// Tool initialization timestamp
    initialized_at: Option<Instant>,
    /// Last operation duration tracking
    last_operation_duration: Option<Duration>,
    /// Performance metrics
    performance_stats: CodexPerformanceStats,
}

/// Performance statistics for codex operations
#[derive(Debug, Clone)]
pub struct CodexPerformanceStats {
    /// Total number of database operations
    pub total_operations: u64,
    /// Average operation latency
    pub average_latency_ms: f64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Total errors encountered
    pub total_errors: u64,
    /// Cache hit rate (if applicable)
    pub cache_hit_rate: f64,
}

impl Default for CodexPerformanceStats {
    fn default() -> Self {
        Self {
            total_operations: 0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            total_errors: 0,
            cache_hit_rate: 0.0,
        }
    }
}

impl MigratedCodexTool {
    /// Create a new migrated codex tool
    pub fn new() -> Self {
        Self {
            database_context: None,
            api_contract: get_api_contract().clone(),
            base: CodexToolBase::new(),
            tool_registry: get_tool_registry(),
            initialized_at: None,
            last_operation_duration: None,
            performance_stats: CodexPerformanceStats::default(),
        }
    }

    /// Load codex entries from database with retry logic
    pub async fn load_entries(&mut self, project_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let result = context.execute_with_retry(
                "load_entries",
                |service| Box::pin(async move {
                    let entries = service.get_entries_by_project(project_id).await?;
                    Ok::<Vec<CodexEntry>, String>(entries)
                }),
                3,
            ).await;

            match result {
                Ok(entries) => {
                    // Update base with loaded entries
                    self.base.entries = entries;
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_performance_stats(true, duration);
                    
                    // Broadcast successful load
                    self.broadcast_codex_event("entries_loaded").await;
                    
                    DatabaseOperationResult::success((), duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.update_performance_stats(false, duration);
                    DatabaseOperationResult::validation_error("load_entries", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Create a new codex entry with database persistence
    pub async fn create_entry(
        &mut self,
        title: String,
        entry_type: CodexEntryType,
        content: String,
        project_id: String,
    ) -> DatabaseOperationResult<String> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            // Generate entry ID
            let entry_id = format!("codex_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis());

            // Create codex entry
            let entry = CodexEntry::new(
                entry_id.clone(),
                title,
                entry_type,
                content,
                project_id.clone(),
            );

            // Save to database
            let result = context.execute_with_retry(
                "create_entry",
                |service| Box::pin(async move {
                    service.create_entry(&entry).await?;
                    Ok::<String, String>(entry_id.clone())
                }),
                3,
            ).await;

            match result {
                Ok(saved_id) => {
                    // Update in-memory entries
                    self.base.entries.push(entry);
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_performance_stats(true, duration);
                    
                    // Broadcast creation event
                    self.broadcast_codex_event("entry_created").await;
                    
                    DatabaseOperationResult::success(saved_id, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.update_performance_stats(false, duration);
                    DatabaseOperationResult::validation_error("create_entry", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Update a codex entry with database persistence
    pub async fn update_entry(&mut self, entry: &CodexEntry) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            // Update in database
            let result = context.execute_with_retry(
                "update_entry",
                |service| Box::pin(async move {
                    service.update_entry(entry).await?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Update in-memory entries
                    if let Some(index) = self.base.entries.iter().position(|e| e.id == entry.id) {
                        self.base.entries[index] = entry.clone();
                    }
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_performance_stats(true, duration);
                    
                    DatabaseOperationResult::success((), duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.update_performance_stats(false, duration);
                    DatabaseOperationResult::validation_error("update_entry", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Delete a codex entry with database persistence
    pub async fn delete_entry(&mut self, entry_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            // Delete from database
            let result = context.execute_with_retry(
                "delete_entry",
                |service| Box::pin(async move {
                    service.delete_entry(entry_id).await?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Remove from in-memory entries
                    self.base.entries.retain(|e| e.id != entry_id);
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_performance_stats(true, duration);
                    
                    // Broadcast deletion event
                    self.broadcast_codex_event("entry_deleted").await;
                    
                    DatabaseOperationResult::success((), duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.update_performance_stats(false, duration);
                    DatabaseOperationResult::validation_error("delete_entry", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Search codex entries with database integration
    pub async fn search_entries(&self, query: &str) -> DatabaseOperationResult<Vec<CodexEntry>> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "search_entries",
                |service| Box::pin(async move {
                    let results = service.search_entries(query).await?;
                    Ok::<Vec<CodexEntry>, String>(results)
                }),
                3,
            ).await;

            match result {
                Ok(entries) => {
                    let duration = start_time.elapsed();
                    self.update_performance_stats(true, duration);
                    
                    DatabaseOperationResult::success(entries, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.update_performance_stats(false, duration);
                    DatabaseOperationResult::validation_error("search_entries", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get entries by type
    pub fn get_entries_by_type(&self, entry_type: CodexEntryType) -> Vec<&CodexEntry> {
        self.base.entries.iter()
            .filter(|entry| entry.entry_type == entry_type)
            .collect()
    }

    /// Get codex statistics
    pub fn get_codex_stats(&self) -> CodexStats {
        let total_entries = self.base.entries.len();
        let character_count = self.base.entries.iter()
            .map(|entry| entry.content.len())
            .sum::<usize>();
        
        let mut type_counts = std::collections::HashMap::new();
        for entry in &self.base.entries {
            *type_counts.entry(entry.entry_type.clone()).or_insert(0) += 1;
        }
        
        CodexStats {
            total_entries,
            character_count,
            type_counts,
            last_operation_duration: self.last_operation_duration,
            performance_stats: self.performance_stats.clone(),
        }
    }

    /// Export codex data
    pub fn export_codex(&self, format: &str) -> DatabaseOperationResult<String> {
        let start_time = Instant::now();
        
        // Generate export data
        let export_data = match format {
            "json" => {
                serde_json::to_string_pretty(&self.base.entries)
                    .map_err(|e| e.to_string())?
            }
            "markdown" => {
                self.generate_markdown_export()
            }
            _ => {
                return DatabaseOperationResult::validation_error(
                    "export_format", 
                    format!("Unsupported export format: {}", format)
                );
            }
        };
        
        let duration = start_time.elapsed();
        self.update_performance_stats(true, duration);
        
        DatabaseOperationResult::success(export_data, duration)
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> &CodexPerformanceStats {
        &self.performance_stats
    }

    /// Migrate from legacy codex tool
    pub async fn migrate_from_legacy(
        legacy_tool: super::codex_ui::CodexTool,
        database_context: ToolDatabaseContext,
    ) -> Result<Self> {
        let mut migrated_tool = Self::new();
        
        // Initialize with database context
        let mut db_context = database_context;
        migrated_tool.initialize(&mut db_context).await?;
        
        // Copy base data
        migrated_tool.base = legacy_tool.base;
        
        Ok(migrated_tool)
    }

    /// Generate markdown export
    fn generate_markdown_export(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Codex Export\n\n");
        
        for entry in &self.base.entries {
            markdown.push_str(&format!("## {} ({:?})\n\n", entry.title, entry.entry_type));
            markdown.push_str(&entry.content);
            markdown.push_str("\n\n");
            markdown.push_str("---\n\n");
        }
        
        markdown
    }

    /// Update performance statistics
    fn update_performance_stats(&mut self, success: bool, duration: Duration) {
        self.performance_stats.total_operations += 1;
        
        if success {
            // Update moving average for latency
            let current_avg = self.performance_stats.average_latency_ms;
            let operation_ms = duration.as_millis_f64();
            let count = self.performance_stats.total_operations as f64;
            self.performance_stats.average_latency_ms = 
                (current_avg * (count - 1.0) + operation_ms) / count;
        } else {
            self.performance_stats.total_errors += 1;
        }
        
        // Update success rate
        let success_count = self.performance_stats.total_operations - self.performance_stats.total_errors;
        self.performance_stats.success_rate = 
            (success_count as f64 / self.performance_stats.total_operations as f64) * 100.0;
    }

    /// Broadcast codex-related events
    async fn broadcast_codex_event(&self, event_type: &str) {
        if let Err(e) = self.api_contract.broadcast_lifecycle(ToolLifecycleEvent::CustomEvent {
            tool_id: self.tool_type().display_name().to_string(),
            event_type: event_type.to_string(),
            timestamp: Instant::now(),
            data: Some(format!("Codex event: {}", event_type)),
        }).await {
            warn!("Failed to broadcast codex event: {}", e);
        }
    }
}

/// Statistics about codex data
#[derive(Debug, Clone)]
pub struct CodexStats {
    /// Total number of entries
    pub total_entries: usize,
    /// Total character count
    pub character_count: usize,
    /// Entry count by type
    pub type_counts: std::collections::HashMap<CodexEntryType, usize>,
    /// Duration of last operation
    pub last_operation_duration: Option<Duration>,
    /// Performance statistics
    pub performance_stats: CodexPerformanceStats,
}

#[async_trait]
impl ToolIntegration for MigratedCodexTool {
    /// Initialize the codex tool with database context
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        // Store database context
        self.database_context = Some(database_context.clone());
        
        // Register with tool registry
        let tool_id = format!("{}_{}", self.tool_type().display_name(), uuid::Uuid::new_v4());
        self.tool_registry.register_tool(tool_id.clone(), Arc::new(()) as Arc<dyn Send + Sync + 'static>).await?;
        
        // Initialize API contract
        self.api_contract = get_api_contract().clone();
        
        // Mark as initialized
        self.initialized_at = Some(Instant::now());
        
        // Broadcast initialization event
        self.broadcast_codex_event("initialized").await;
        
        info!("Codex tool initialized successfully");
        Ok(())
    }

    /// Update tool state
    fn update(&mut self) -> Result<(), String> {
        // Perform any periodic updates
        // For codex tool, this might include:
        // - Syncing with database changes
        // - Updating search indexes
        // - Performing cleanup operations
        
        // Validate entry integrity
        self.validate_entries_integrity()?;
        
        Ok(())
    }

    /// Render the tool UI
    fn render(&mut self) -> Result<(), String> {
        // This would typically render the codex UI
        // For now, we'll just validate that the tool is in a renderable state
        
        if self.database_context.is_none() {
            return Err("Database context not initialized".to_string());
        }
        
        Ok(())
    }

    /// Cleanup tool resources
    async fn cleanup(&mut self) -> Result<(), String> {
        // Broadcast cleanup event
        self.broadcast_codex_event("cleanup_started").await;
        
        // Clear in-memory data
        self.base.entries.clear();
        
        // Unregister from tool registry
        let tool_id = format!("{}_{}", self.tool_type().display_name(), uuid::Uuid::new_v4());
        self.tool_registry.unregister_tool(&tool_id).await?;
        
        // Clear database context
        self.database_context = None;
        
        info!("Codex tool cleanup completed");
        Ok(())
    }
}

impl MigratedCodexTool {
    /// Validate entries integrity
    fn validate_entries_integrity(&self) -> Result<(), String> {
        // Check for duplicate IDs
        let mut ids = std::collections::HashSet::new();
        for entry in &self.base.entries {
            if !ids.insert(&entry.id) {
                return Err(format!("Duplicate entry ID detected: {}", entry.id));
            }
        }
        
        // Check for empty titles
        for entry in &self.base.entries {
            if entry.title.trim().is_empty() {
                return Err(format!("Entry with empty title found: {}", entry.id));
            }
        }
        
        Ok(())
    }
}

impl Default for MigratedCodexTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolType for MigratedCodexTool {
    fn display_name(&self) -> &'static str {
        "Codex Tool"
    }
    
    fn tool_id(&self) -> &'static str {
        "codex_tool"
    }
    
    fn version(&self) -> &'static str {
        "2.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Worldbuilding and reference management tool with database integration"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseAppState;
    use tokio::sync::RwLock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_codex_tool_initialization() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedCodexTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_codex", database_state).await;
        let result = tool.initialize(&mut database_context).await;
        
        assert!(result.is_ok());
        assert!(tool.database_context.is_some());
        assert!(tool.initialized_at.is_some());
    }

    #[tokio::test]
    async fn test_codex_entry_creation() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedCodexTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_codex", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create a new entry
        let result = tool.create_entry(
            "Test Character".to_string(),
            CodexEntryType::Character,
            "This is a test character.".to_string(),
            "test_project".to_string(),
        ).await;
        
        assert!(result.is_success());
        assert_eq!(tool.base.entries.len(), 1);
        assert_eq!(tool.base.entries[0].title, "Test Character");
    }

    #[tokio::test]
    async fn test_codex_entry_deletion() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedCodexTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_codex", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create and then delete an entry
        let entry_id = tool.create_entry(
            "Test Location".to_string(),
            CodexEntryType::Location,
            "This is a test location.".to_string(),
            "test_project".to_string(),
        ).await;
        
        assert!(entry_id.is_success());
        
        let delete_result = tool.delete_entry(&entry_id.data.unwrap()).await;
        assert!(delete_result.is_success());
        assert_eq!(tool.base.entries.len(), 0);
    }

    #[tokio::test]
    async fn test_codex_stats() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedCodexTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_codex", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create test entries
        tool.create_entry(
            "Character 1".to_string(),
            CodexEntryType::Character,
            "Character content".to_string(),
            "test_project".to_string(),
        ).await;
        
        tool.create_entry(
            "Location 1".to_string(),
            CodexEntryType::Location,
            "Location content".to_string(),
            "test_project".to_string(),
        ).await;
        
        let stats = tool.get_codex_stats();
        assert_eq!(stats.total_entries, 2);
        assert!(stats.character_count > 0);
        assert_eq!(stats.type_counts.len(), 2);
    }

    #[tokio::test]
    async fn test_codex_export() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedCodexTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_codex", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create test entry
        tool.create_entry(
            "Test Export".to_string(),
            CodexEntryType::Character,
            "Export test content.".to_string(),
            "test_project".to_string(),
        ).await;
        
        // Test JSON export
        let json_result = tool.export_codex("json").await;
        assert!(json_result.is_success());
        assert!(json_result.data.unwrap().contains("Test Export"));
        
        // Test Markdown export
        let md_result = tool.export_codex("markdown").await;
        assert!(md_result.is_success());
        let markdown = md_result.data.unwrap();
        assert!(markdown.contains("# Codex Export"));
        assert!(markdown.contains("## Test Export"));
    }
}