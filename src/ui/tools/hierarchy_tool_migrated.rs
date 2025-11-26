//! Migrated Hierarchy Tool Implementation
//!
//! This module provides the migrated version of the hierarchy tool using the new
//! unified architecture patterns with ToolDatabaseContext, ThreadSafeToolRegistry,
//! and ToolApiContract.

use crate::ui::tools::{
    database_integration::{ToolDatabaseContext, DatabaseOperationResult},
    threading_patterns::get_tool_registry,
    api_contracts::{ToolApiContract, get_api_contract, ToolLifecycleEvent},
    base::ToolIntegration,
    base_types::ToolType,
};
use crate::database::DatabaseError;
use tracing::{info, warn, debug};
use anyhow::Result;
use std::sync::Arc;
use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};
use serde_json::json;
use tokio::sync::RwLock;
use async_trait::async_trait;

use super::hierarchy_base::{HierarchyItem, HierarchyLevel, HierarchyTree};
use super::hierarchy_ui::{HierarchyTool as LegacyHierarchyTool, HierarchyUiState};
use super::hierarchy_database::HierarchyDatabaseService;

/// Migrated hierarchy tool that implements the new architecture patterns
pub struct MigratedHierarchyTool {
    /// Database context for safe database operations
    database_context: Option<ToolDatabaseContext>,
    /// API contract for cross-tool communication
    api_contract: Arc<ToolApiContract>,
    /// Internal hierarchy tree data
    hierarchy_tree: HierarchyTree,
    /// UI state management
    ui_state: HierarchyUiState,
    /// Tool registry reference for lifecycle management
    tool_registry: &'static crate::ui::tools::threading_patterns::ThreadSafeToolRegistry,
    /// Tool initialization timestamp
    initialized_at: Option<Instant>,
    /// Last operation duration tracking
    last_operation_duration: Option<Duration>,
}

impl MigratedHierarchyTool {
    /// Create a new migrated hierarchy tool
    pub fn new() -> Self {
        Self {
            database_context: None,
            api_contract: Arc::new(get_api_contract().clone()),
            hierarchy_tree: HierarchyTree::new(),
            ui_state: HierarchyUiState::new(),
            tool_registry: get_tool_registry(),
            initialized_at: None,
            last_operation_duration: None,
        }
    }

    /// Load hierarchy data from database with retry logic
    pub async fn load_hierarchy(&mut self, project_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let project_id = project_id.to_string();
            let result = context.execute_with_retry(
                "load_hierarchy",
                |service| {
                    let project_id = project_id.clone();
                    Box::pin(async move {
                        let pool = service.read().await.pool.clone();
                        let db_service = HierarchyDatabaseService::new(pool);
                        // Ensure schema exists (should be done in initialize, but safe here too)
                        db_service.initialize_schema().await.map_err(|e| DatabaseError::Service(e.to_string()))?;
                        
                        let hierarchy_data = db_service.get_items_by_project(&project_id).await
                            .map_err(|e| DatabaseError::Service(e.to_string()))?;
                        Ok::<Vec<HierarchyItem>, DatabaseError>(hierarchy_data)
                    })
                },
                3,
            ).await;

            match result {
                Ok(hierarchy_items) => {
                    // Build hierarchy tree from loaded data
                    self.hierarchy_tree.clear();
                    for item in hierarchy_items {
                        if let Err(e) = self.hierarchy_tree.add_item(item) {
                            warn!("Failed to add item to tree: {}", e);
                            return DatabaseOperationResult::validation_error("load_hierarchy".to_string(), e);
                        }
                    }
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    
                    // Broadcast successful load
                    self.broadcast_hierarchy_event("loaded").await;
                    
                    DatabaseOperationResult::success((), duration.as_millis() as u64)
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("load_hierarchy".to_string(), e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable".to_string())
        }
    }

    /// Create a new hierarchy item with database persistence
    pub async fn create_hierarchy_item(
        &mut self,
        title: String,
        level: HierarchyLevel,
        parent_id: Option<String>,
    ) -> DatabaseOperationResult<String> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            // Generate item ID
            let item_id = format!("hierarchy_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis());

            // Create hierarchy item
            let item = HierarchyItem::new(
                item_id.clone(),
                title.clone(),
                level,
                parent_id.clone(),
                "default_project".to_string(), // This should come from app state
            );

            // Save to database
            let item_clone = item.clone();
            let result = context.execute_with_retry(
                "create_hierarchy_item",
                |service| {
                    let item_clone = item_clone.clone();
                    let item_id = item_id.clone();
                    Box::pin(async move {
                        let pool = service.read().await.pool.clone();
                        let db_service = HierarchyDatabaseService::new(pool);
                        db_service.create_item(&item_clone).await.map_err(|e| DatabaseError::Service(e.to_string()))?;
                        Ok::<String, DatabaseError>(item_id)
                    })
                },
                3,
            ).await;

            match result {
                Ok(saved_id) => {
                    // Update in-memory tree
                    if let Err(e) = self.hierarchy_tree.add_item(item) {
                        warn!("Failed to add item to in-memory tree: {}", e);
                    }
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    
                    // Broadcast creation event
                    self.broadcast_hierarchy_event("item_created").await;
                    
                    DatabaseOperationResult::success(saved_id, duration.as_millis() as u64)
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("create_item".to_string(), e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable".to_string())
        }
    }

    /// Delete a hierarchy item with database persistence
    pub async fn delete_hierarchy_item(&mut self, item_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            // Get item for validation
            let _item_to_delete = match self.hierarchy_tree.get_item(item_id).cloned() {
                Some(item) => item,
                None => return DatabaseOperationResult::not_found("Hierarchy Item", item_id.to_string()),
            };

            // Delete from database
            let item_id_clone = item_id.to_string();
            let result = context.execute_with_retry(
                "delete_hierarchy_item",
                |service| {
                    let item_id_clone = item_id_clone.clone();
                    Box::pin(async move {
                        let pool = service.read().await.pool.clone();
                        let db_service = HierarchyDatabaseService::new(pool);
                        db_service.delete_item(&item_id_clone).await.map_err(|e| DatabaseError::Service(e.to_string()))?;
                        Ok::<(), DatabaseError>(())
                    })
                },
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Remove from in-memory tree
                    if let Err(e) = self.hierarchy_tree.remove_item(item_id) {
                        warn!("Failed to remove item from in-memory tree: {}", e);
                    }
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    
                    // Broadcast deletion event
                    self.broadcast_hierarchy_event("item_deleted").await;
                    
                    DatabaseOperationResult::success((), duration.as_millis() as u64)
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("delete_item".to_string(), e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable".to_string())
        }
    }

    /// Update hierarchy item position with database persistence
    pub async fn update_item_position(
        &mut self,
        item_id: &str,
        new_position: u32,
    ) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            // Update in database
            let item_id_clone = item_id.to_string();
            let result = context.execute_with_retry(
                "update_item_position",
                |service| {
                    let item_id_clone = item_id_clone.clone();
                    Box::pin(async move {
                        let pool = service.read().await.pool.clone();
                        let db_service = HierarchyDatabaseService::new(pool);
                        db_service.update_item_position(&item_id_clone, new_position).await.map_err(|e| DatabaseError::Service(e.to_string()))?;
                        Ok::<(), DatabaseError>(())
                    })
                },
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Update in-memory tree
                    if let Some(item) = self.hierarchy_tree.get_item_mut(item_id) {
                        item.update_position(new_position);
                    }
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    
                    DatabaseOperationResult::success((), duration.as_millis() as u64)
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("update_position".to_string(), e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable".to_string())
        }
    }

    /// Get hierarchy tree statistics
    pub fn get_hierarchy_stats(&self) -> HierarchyStats {
        let total_items = self.hierarchy_tree.len();
        let root_items = self.hierarchy_tree.get_root_items().len();
        let leaf_items = self.hierarchy_tree.get_leaf_items().len();
        let branch_items = self.hierarchy_tree.get_branch_items().len();
        
        HierarchyStats {
            total_items,
            root_items,
            leaf_items,
            branch_items,
            depth: self.calculate_max_depth(),
            last_operation_duration: self.last_operation_duration,
        }
    }

    /// Get selected item ID
    pub fn get_selected_item(&self) -> Option<String> {
        self.ui_state.selected_item.clone()
    }

    /// Select an item
    pub fn select_item(&mut self, item_id: Option<String>) {
        self.ui_state.select_item(item_id);
    }

    /// Get current hierarchy tree
    pub fn get_hierarchy_tree(&self) -> &HierarchyTree {
        &self.hierarchy_tree
    }

    /// Calculate maximum hierarchy depth
    fn calculate_max_depth(&self) -> u32 {
        let mut max_depth = 0;
        for root_item in self.hierarchy_tree.get_root_items() {
            max_depth = max_depth.max(self.calculate_item_depth(&root_item.id, 0));
        }
        max_depth
    }

    /// Calculate depth for a specific item
    fn calculate_item_depth(&self, item_id: &str, current_depth: u32) -> u32 {
        let mut max_depth = current_depth;
        let children = self.hierarchy_tree.get_children(item_id);
        
        for child in children {
            max_depth = max_depth.max(self.calculate_item_depth(&child.id, current_depth + 1));
        }
        
        max_depth
    }

    /// Broadcast hierarchy-related events
    async fn broadcast_hierarchy_event(&self, event_type: &str) {
        use std::time::{SystemTime, UNIX_EPOCH};
        use serde_json::json;
        if let Err(e) = self.api_contract.broadcast_lifecycle(ToolLifecycleEvent::CustomEvent {
            tool_id: "hierarchy".to_string(),
            event_type: event_type.to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64,
            payload: json!({ "message": format!("Hierarchy event: {}", event_type) }),
        }).await {
            warn!("Failed to broadcast hierarchy event: {}", e);
        }
    }

    /// Migrate from legacy hierarchy tool
    pub async fn migrate_from_legacy(
        legacy_tool: LegacyHierarchyTool,
        database_context: ToolDatabaseContext,
    ) -> Result<Self> {
        let mut migrated_tool = Self::new();
        
        // Initialize with database context
        let mut db_context = database_context;
        migrated_tool.initialize(&mut db_context).await.map_err(|e| anyhow::anyhow!(e))?;
        
        // Copy hierarchy data
        let legacy_tree = legacy_tool.get_hierarchy();
        for item in legacy_tree.get_all_items() {
            if let Err(e) = migrated_tool.hierarchy_tree.add_item(item.clone()) {
                warn!("Failed to migrate item {}: {}", item.id, e);
            }
        }
        
        // Copy UI state
        // Note: This would need to be adapted based on actual legacy tool structure
        
        Ok(migrated_tool)
    }
}

/// Statistics about the hierarchy structure
#[derive(Debug, Clone)]
pub struct HierarchyStats {
    /// Total number of items in hierarchy
    pub total_items: usize,
    /// Number of root items (top-level)
    pub root_items: usize,
    /// Number of leaf items (items with no children)
    pub leaf_items: usize,
    /// Number of branch items (items with children)
    pub branch_items: usize,
    /// Maximum depth of the hierarchy
    pub depth: u32,
    /// Duration of last operation
    pub last_operation_duration: Option<Duration>,
}

#[async_trait]
impl ToolIntegration for MigratedHierarchyTool {
    /// Initialize the hierarchy tool with database context
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        // Store database context
        self.database_context = Some(database_context.clone());
        
        // Register with tool registry
        let tool_id = format!("{}_{}", self.tool_type().display_name(), uuid::Uuid::new_v4());
        self.tool_registry.register_tool(tool_id.clone(), Arc::new(()) as Arc<dyn Send + Sync + 'static>).await?;
        
        // Initialize API contract
        self.api_contract = Arc::new(get_api_contract().clone());
        
        // Initialize database service and schema
        if let Ok(db_service) = database_context.get_database_service().await {
            let pool = db_service.read().await.pool.clone();
            let hierarchy_db = HierarchyDatabaseService::new(pool);
            if let Err(e) = hierarchy_db.initialize_schema().await {
                warn!("Failed to initialize hierarchy database schema: {}", e);
                return Err(e.to_string());
            }
            self.db_service = Some(hierarchy_db);
        } else {
            warn!("Database service not available during initialization");
        }

        // Mark as initialized
        self.initialized_at = Some(Instant::now());
        
        // Broadcast initialization event
        self.broadcast_hierarchy_event("initialized").await;
        
        info!("Hierarchy tool initialized successfully");
        Ok(())
    }

    /// Update tool state
    fn update(&mut self) -> Result<(), String> {
        // Perform any periodic updates
        // For hierarchy tool, this might include:
        // - Syncing with database changes
        // - Updating UI state
        // - Performing cleanup operations
        
        // Validate hierarchy integrity
        self.validate_hierarchy_integrity()?;
        
        Ok(())
    }

    /// Render the tool UI
    fn render(&mut self) -> Result<(), String> {
        // This would typically render the hierarchy UI
        // For now, we'll just validate that the tool is in a renderable state
        
        if self.database_context.is_none() {
            return Err("Database context not initialized".to_string());
        }
        
        if self.hierarchy_tree.is_empty() {
            debug!("Hierarchy tool: No data to render");
        }
        
        Ok(())
    }

    /// Cleanup tool resources
    async fn cleanup(&mut self) -> Result<(), String> {
        // Broadcast cleanup event
        self.broadcast_hierarchy_event("cleanup_started").await;
        
        // Clear in-memory data
        self.hierarchy_tree.clear();
        self.ui_state = HierarchyUiState::new();
        
        // Unregister from tool registry
        let tool_id = format!("{}_{}", self.tool_type().display_name(), uuid::Uuid::new_v4());
        self.tool_registry.unregister_tool(&tool_id).await?;
        
        // Clear database context
        self.database_context = None;
        
        info!("Hierarchy tool cleanup completed");
        Ok(())
    }

    fn tool_type(&self) -> ToolType {
        ToolType::Hierarchy
    }

    fn display_name(&self) -> &str {
        "Hierarchy Tool"
    }

    fn is_initialized(&self) -> bool {
        self.initialized_at.is_some()
    }

    fn initialized_at(&self) -> Option<Instant> {
        self.initialized_at
    }
}

impl MigratedHierarchyTool {
    /// Validate hierarchy integrity
    fn validate_hierarchy_integrity(&self) -> Result<(), String> {
        // Check for orphaned items
        for item in self.hierarchy_tree.get_all_items() {
            if let Some(parent_id) = &item.parent_id {
                if !self.hierarchy_tree.get_item(parent_id).is_some() {
                    return Err(format!("Orphaned item detected: {} (parent: {})", item.id, parent_id));
                }
            }
        }
        
        // Check for circular references
        for root_item in self.hierarchy_tree.get_root_items() {
            if self.has_circular_reference(&root_item.id, &root_item.id) {
                return Err(format!("Circular reference detected starting from: {}", root_item.id));
            }
        }
        
        Ok(())
    }

    /// Check for circular references in hierarchy
    fn has_circular_reference(&self, start_id: &str, current_id: &str) -> bool {
        let children = self.hierarchy_tree.get_children(current_id);
        for child in children {
            if child.id == start_id {
                return true;
            }
            if self.has_circular_reference(start_id, &child.id) {
                return true;
            }
        }
        false
    }
}

impl Default for MigratedHierarchyTool {
    fn default() -> Self {
        Self::new()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseAppState;
    use tokio::sync::RwLock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_hierarchy_tool_initialization() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedHierarchyTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_hierarchy", database_state).await;
        let result = tool.initialize(&mut database_context).await;
        
        assert!(result.is_ok());
        assert!(tool.database_context.is_some());
        assert!(tool.initialized_at.is_some());
    }

    #[tokio::test]
    async fn test_hierarchy_item_creation() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedHierarchyTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_hierarchy", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create a root manuscript item
        let result = tool.create_hierarchy_item(
            "Test Manuscript".to_string(),
            HierarchyLevel::Manuscript,
            None,
        ).await;
        
        assert!(result.is_success());
        assert_eq!(tool.hierarchy_tree.len(), 1);
    }

    #[tokio::test]
    async fn test_hierarchy_item_deletion() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedHierarchyTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_hierarchy", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create and then delete an item
        let item_id = tool.create_hierarchy_item(
            "Test Item".to_string(),
            HierarchyLevel::Chapter,
            None,
        ).await;
        
        assert!(item_id.is_success());
        
        let delete_result = tool.delete_hierarchy_item(&item_id.into_result().unwrap()).await;
        assert!(delete_result.is_success());
        assert_eq!(tool.hierarchy_tree.len(), 0);
    }

    #[tokio::test]
    async fn test_hierarchy_stats() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedHierarchyTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_hierarchy", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create a hierarchy structure
        let manuscript_id = tool.create_hierarchy_item(
            "Test Manuscript".to_string(),
            HierarchyLevel::Manuscript,
            None,
        ).await.into_result().unwrap();
        
        let chapter_id = tool.create_hierarchy_item(
            "Test Chapter".to_string(),
            HierarchyLevel::Chapter,
            Some(manuscript_id),
        ).await.into_result().unwrap();
        
        tool.create_hierarchy_item(
            "Test Scene".to_string(),
            HierarchyLevel::Scene,
            Some(chapter_id),
        ).await;
        
        let stats = tool.get_hierarchy_stats();
        assert_eq!(stats.total_items, 3);
        assert_eq!(stats.root_items, 1);
        assert_eq!(stats.depth, 2);
    }
}