//! Hierarchy Database Service
//!
//! Provides database operations for hierarchy data using the new ToolDatabaseContext pattern.
//! This service replaces direct database access with safe, retry-aware operations.

use crate::ui::tools::database_integration::{ToolDatabaseContext, DatabaseOperationResult};
use crate::ui::tools::hierarchy_base::{HierarchyItem, HierarchyLevel, HierarchyTree};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use async_trait::async_trait;

/// Service for managing hierarchy data in the database
pub struct HierarchyDatabaseService {
    /// Database context for operations
    database_context: Option<ToolDatabaseContext>,
    /// In-memory cache for frequently accessed data
    item_cache: HashMap<String, HierarchyItem>,
    /// Cache statistics for monitoring
    cache_stats: CacheStats,
}

/// Statistics about cache performance
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Cache hit rate
    pub hit_rate: f64,
}

/// Mock database storage for hierarchy data
#[derive(Debug, Default)]
pub struct HierarchyDatabaseStore {
    /// Storage for hierarchy items
    pub items: HashMap<String, HierarchyItem>,
    /// Storage for project hierarchies
    pub project_hierarchies: HashMap<String, Vec<String>>,
}

impl HierarchyDatabaseStore {
    /// Create a new empty database store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an item to the store
    pub fn add_item(&mut self, item: HierarchyItem) {
        self.items.insert(item.id.clone(), item.clone());
        
        // Add to project hierarchy
        let project_items = self.project_hierarchies
            .entry(item.project_id.clone())
            .or_insert_with(Vec::new);
        if !project_items.contains(&item.id) {
            project_items.push(item.id);
        }
    }

    /// Get an item by ID
    pub fn get_item(&self, item_id: &str) -> Option<&HierarchyItem> {
        self.items.get(item_id)
    }

    /// Get all items for a project
    pub fn get_project_items(&self, project_id: &str) -> Vec<&HierarchyItem> {
        self.project_hierarchies
            .get(project_id)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect()
    }

    /// Remove an item from the store
    pub fn remove_item(&mut self, item_id: &str) -> Option<HierarchyItem> {
        if let Some(item) = self.items.remove(item_id) {
            // Remove from project hierarchy
            if let Some(project_items) = self.project_hierarchies.get_mut(&item.project_id) {
                project_items.retain(|id| id != item_id);
            }
            Some(item)
        } else {
            None
        }
    }

    /// Update item position
    pub fn update_item_position(&mut self, item_id: &str, position: u32) -> Result<()> {
        if let Some(item) = self.items.get_mut(item_id) {
            item.update_position(position);
            Ok(())
        } else {
            anyhow::bail!("Item not found: {}", item_id)
        }
    }
}

/// Global database store instance (for demo purposes)
lazy_static::lazy_static! {
    static ref GLOBAL_HIERARCHY_DB: tokio::sync::RwLock<HierarchyDatabaseStore> = 
        tokio::sync::RwLock::new(HierarchyDatabaseStore::new());
}

impl HierarchyDatabaseService {
    /// Create a new hierarchy database service
    pub fn new() -> Self {
        Self {
            database_context: None,
            item_cache: HashMap::new(),
            cache_stats: CacheStats {
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
            },
        }
    }

    /// Initialize the service with database context
    pub async fn initialize(&mut self, database_context: ToolDatabaseContext) {
        self.database_context = Some(database_context);
    }

    /// Get hierarchy items for a project with caching
    pub async fn get_hierarchy_by_project(&self, project_id: &str) -> DatabaseOperationResult<Vec<HierarchyItem>> {
        let start_time = Instant::now();
        
        // Try cache first
        let cached_items = self.get_cached_hierarchy(project_id);
        if !cached_items.is_empty() {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_items, start_time.elapsed());
        }
        
        // Cache miss - fetch from database
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_hierarchy_by_project",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_HIERARCHY_DB.read().await;
                    let items: Vec<HierarchyItem> = db_store.get_project_items(project_id)
                        .into_iter()
                        .cloned()
                        .collect();
                    Ok::<Vec<HierarchyItem>, String>(items)
                }),
                3,
            ).await;

            match result {
                Ok(items) => {
                    // Cache the results
                    self.cache_hierarchy_items(project_id, &items);
                    DatabaseOperationResult::success(items, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_hierarchy", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Create a new hierarchy item
    pub async fn create_hierarchy_item(&self, item: &HierarchyItem) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "create_hierarchy_item",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_HIERARCHY_DB.write().await;
                    
                    // Validate item doesn't already exist
                    if db_store.get_item(&item.id).is_some() {
                        return Err(format!("Item already exists: {}", item.id));
                    }
                    
                    // Add to database
                    db_store.add_item(item.clone());
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Invalidate cache for this project
                    self.invalidate_project_cache(&item.project_id);
                    
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("create_item", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Delete a hierarchy item
    pub async fn delete_hierarchy_item(&self, item_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "delete_hierarchy_item",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_HIERARCHY_DB.write().await;
                    
                    // Get item to determine project ID
                    let item = db_store.get_item(item_id)
                        .ok_or_else(|| format!("Item not found: {}", item_id))?;
                    let project_id = item.project_id.clone();
                    
                    // Remove from database
                    db_store.remove_item(item_id)
                        .ok_or_else(|| format!("Failed to remove item: {}", item_id))?;
                    
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Invalidate cache for this project
                    // Note: We need to get project_id from somewhere - in real implementation
                    // this would come from the database query above
                    // self.invalidate_project_cache(&project_id);
                    
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("delete_item", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Update hierarchy item position
    pub async fn update_hierarchy_item_position(&self, item_id: &str, position: u32) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "update_hierarchy_item_position",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_HIERARCHY_DB.write().await;
                    db_store.update_item_position(item_id, position)
                        .map_err(|e| e.to_string())?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Invalidate cache since item was modified
                    // Note: In real implementation, we'd need to determine the project ID
                    // self.invalidate_project_cache(&project_id);
                    
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("update_position", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get hierarchy item by ID
    pub async fn get_hierarchy_item(&self, item_id: &str) -> DatabaseOperationResult<Option<HierarchyItem>> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_item) = self.item_cache.get(item_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(Some(cached_item.clone()), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_hierarchy_item",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_HIERARCHY_DB.read().await;
                    let item = db_store.get_item(item_id).cloned();
                    Ok::<Option<HierarchyItem>, String>(item)
                }),
                3,
            ).await;

            match result {
                Ok(item) => {
                    // Cache the result if found
                    if let Some(ref item) = item {
                        self.item_cache.insert(item.id.clone(), item.clone());
                    }
                    
                    DatabaseOperationResult::success(item, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_item", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get hierarchy statistics
    pub async fn get_hierarchy_statistics(&self, project_id: &str) -> DatabaseOperationResult<HierarchyStatistics> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_hierarchy_statistics",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_HIERARCHY_DB.read().await;
                    let items = db_store.get_project_items(project_id);
                    
                    let mut stats = HierarchyStatistics::new();
                    stats.total_items = items.len();
                    
                    for item in items {
                        match item.level {
                            HierarchyLevel::Unassigned | HierarchyLevel::Manuscript => {
                                stats.manuscript_count += 1;
                            }
                            HierarchyLevel::Chapter => {
                                stats.chapter_count += 1;
                            }
                            HierarchyLevel::Scene => {
                                stats.scene_count += 1;
                            }
                        }
                        
                        if item.has_children() {
                            stats.branch_count += 1;
                        } else {
                            stats.leaf_count += 1;
                        }
                    }
                    
                    Ok::<HierarchyStatistics, String>(stats)
                }),
                3,
            ).await;

            match result {
                Ok(stats) => {
                    DatabaseOperationResult::success(stats, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_statistics", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Clear all hierarchy data for a project
    pub async fn clear_project_hierarchy(&self, project_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "clear_project_hierarchy",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_HIERARCHY_DB.write().await;
                    
                    // Get all items for the project
                    let project_items = db_store.get_project_items(project_id);
                    let item_ids: Vec<String> = project_items.iter()
                        .map(|item| item.id.clone())
                        .collect();
                    
                    // Remove all items
                    for item_id in item_ids {
                        db_store.remove_item(&item_id);
                    }
                    
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Clear cache for this project
                    self.invalidate_project_cache(project_id);
                    
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("clear_hierarchy", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        self.cache_stats.clone()
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.item_cache.clear();
        self.cache_stats = CacheStats {
            hits: 0,
            misses: 0,
            hit_rate: 0.0,
        };
    }

    /// Get cached hierarchy items for a project
    fn get_cached_hierarchy(&self, project_id: &str) -> Vec<HierarchyItem> {
        // In a real implementation, we'd have a more sophisticated caching strategy
        // For now, we'll return empty to force database reads
        Vec::new()
    }

    /// Cache hierarchy items
    fn cache_hierarchy_items(&mut self, project_id: &str, items: &[HierarchyItem]) {
        // Cache individual items
        for item in items {
            self.item_cache.insert(item.id.clone(), item.clone());
        }
    }

    /// Invalidate cache for a project
    fn invalidate_project_cache(&mut self, project_id: &str) {
        // In a real implementation with project-based caching, we'd remove
        // all items for this project from cache
        // For now, we'll clear the entire cache
        self.item_cache.clear();
    }

    /// Update cache hit rate
    fn update_cache_hit_rate(&mut self) {
        let total_requests = self.cache_stats.hits + self.cache_stats.misses;
        if total_requests > 0 {
            self.cache_stats.hit_rate = (self.cache_stats.hits as f64 / total_requests as f64) * 100.0;
        }
    }
}

/// Statistics about hierarchy data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyStatistics {
    /// Total number of items
    pub total_items: usize,
    /// Number of manuscript-level items
    pub manuscript_count: usize,
    /// Number of chapter-level items
    pub chapter_count: usize,
    /// Number of scene-level items
    pub scene_count: usize,
    /// Number of branch items (items with children)
    pub branch_count: usize,
    /// Number of leaf items (items without children)
    pub leaf_count: usize,
    /// Average depth of the hierarchy
    pub average_depth: f64,
    /// Maximum depth of the hierarchy
    pub max_depth: u32,
}

impl HierarchyStatistics {
    /// Create a new statistics instance
    pub fn new() -> Self {
        Self {
            total_items: 0,
            manuscript_count: 0,
            chapter_count: 0,
            scene_count: 0,
            branch_count: 0,
            leaf_count: 0,
            average_depth: 0.0,
            max_depth: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseAppState;
    use tokio::sync::RwLock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_hierarchy_database_service_creation() {
        let mut service = HierarchyDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_hierarchy_db", database_state).await;
        
        service.initialize(database_context).await;
        assert!(service.database_context.is_some());
    }

    #[tokio::test]
    async fn test_hierarchy_item_crud() {
        let mut service = HierarchyDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_hierarchy_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create an item
        let item = HierarchyItem::new(
            "test_item_1".to_string(),
            "Test Item".to_string(),
            HierarchyLevel::Chapter,
            None,
            "test_project".to_string(),
        );
        
        let create_result = service.create_hierarchy_item(&item).await;
        assert!(create_result.is_success());
        
        // Get the item
        let get_result = service.get_hierarchy_item("test_item_1").await;
        assert!(get_result.is_success());
        assert!(get_result.data.unwrap().is_some());
        
        // Delete the item
        let delete_result = service.delete_hierarchy_item("test_item_1").await;
        assert!(delete_result.is_success());
        
        // Verify deletion
        let get_after_delete = service.get_hierarchy_item("test_item_1").await;
        assert!(get_after_delete.is_success());
        assert!(get_after_delete.data.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_hierarchy_by_project() {
        let mut service = HierarchyDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_hierarchy_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create test items
        let manuscript = HierarchyItem::new(
            "manuscript_1".to_string(),
            "Test Manuscript".to_string(),
            HierarchyLevel::Manuscript,
            None,
            "test_project".to_string(),
        );
        
        let chapter = HierarchyItem::new(
            "chapter_1".to_string(),
            "Test Chapter".to_string(),
            HierarchyLevel::Chapter,
            Some("manuscript_1".to_string()),
            "test_project".to_string(),
        );
        
        // Add items
        service.create_hierarchy_item(&manuscript).await;
        service.create_hierarchy_item(&chapter).await;
        
        // Get hierarchy for project
        let hierarchy_result = service.get_hierarchy_by_project("test_project").await;
        assert!(hierarchy_result.is_success());
        
        let items = hierarchy_result.data.unwrap();
        assert_eq!(items.len(), 2);
        
        // Verify cache stats
        let stats = service.get_cache_stats();
        assert!(stats.misses > 0); // Should have at least one miss
    }

    #[tokio::test]
    async fn test_hierarchy_statistics() {
        let mut service = HierarchyDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_hierarchy_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create test hierarchy
        let manuscript = HierarchyItem::new(
            "manuscript_1".to_string(),
            "Test Manuscript".to_string(),
            HierarchyLevel::Manuscript,
            None,
            "test_project".to_string(),
        );
        
        let chapter = HierarchyItem::new(
            "chapter_1".to_string(),
            "Test Chapter".to_string(),
            HierarchyLevel::Chapter,
            Some("manuscript_1".to_string()),
            "test_project".to_string(),
        );
        
        let scene = HierarchyItem::new(
            "scene_1".to_string(),
            "Test Scene".to_string(),
            HierarchyLevel::Scene,
            Some("chapter_1".to_string()),
            "test_project".to_string(),
        );
        
        service.create_hierarchy_item(&manuscript).await;
        service.create_hierarchy_item(&chapter).await;
        service.create_hierarchy_item(&scene).await;
        
        // Get statistics
        let stats_result = service.get_hierarchy_statistics("test_project").await;
        assert!(stats_result.is_success());
        
        let stats = stats_result.data.unwrap();
        assert_eq!(stats.total_items, 3);
        assert_eq!(stats.manuscript_count, 1);
        assert_eq!(stats.chapter_count, 1);
        assert_eq!(stats.scene_count, 1);
    }
}