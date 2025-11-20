//! Codex Database Service
//!
//! Provides database operations for codex data using the new ToolDatabaseContext pattern.
//! This service replaces direct database access with safe, retry-aware operations.

use crate::ui::tools::database_integration::{ToolDatabaseContext, DatabaseOperationResult};
use crate::ui::tools::codex_base::{CodexEntry, CodexEntryType};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use async_trait::async_trait;

/// Service for managing codex data in the database
pub struct CodexDatabaseService {
    /// Database context for operations
    database_context: Option<ToolDatabaseContext>,
    /// In-memory cache for frequently accessed data
    entry_cache: HashMap<String, CodexEntry>,
    /// Search index for efficient querying
    search_index: HashMap<String, Vec<String>>,
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

/// Mock database storage for codex data
#[derive(Debug, Default)]
pub struct CodexDatabaseStore {
    /// Storage for codex entries
    pub entries: HashMap<String, CodexEntry>,
    /// Storage for project codex data
    pub project_entries: HashMap<String, Vec<String>>,
}

impl CodexDatabaseStore {
    /// Create a new empty database store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entry to the store
    pub fn add_entry(&mut self, entry: CodexEntry) {
        self.entries.insert(entry.id.clone(), entry.clone());
        
        // Add to project entries
        let project_entries = self.project_entries
            .entry(entry.project_id.clone())
            .or_insert_with(Vec::new);
        if !project_entries.contains(&entry.id) {
            project_entries.push(entry.id);
        }
    }

    /// Get an entry by ID
    pub fn get_entry(&self, entry_id: &str) -> Option<&CodexEntry> {
        self.entries.get(entry_id)
    }

    /// Get all entries for a project
    pub fn get_project_entries(&self, project_id: &str) -> Vec<&CodexEntry> {
        self.project_entries
            .get(project_id)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|id| self.entries.get(id))
            .collect()
    }

    /// Get entries by type for a project
    pub fn get_entries_by_type(&self, project_id: &str, entry_type: CodexEntryType) -> Vec<&CodexEntry> {
        self.get_project_entries(project_id)
            .into_iter()
            .filter(|entry| entry.entry_type == entry_type)
            .collect()
    }

    /// Search entries by content
    pub fn search_entries(&self, query: &str) -> Vec<&CodexEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .values()
            .filter(|entry| {
                entry.title.to_lowercase().contains(&query_lower) ||
                entry.content.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Remove an entry from the store
    pub fn remove_entry(&mut self, entry_id: &str) -> Option<CodexEntry> {
        if let Some(entry) = self.entries.remove(entry_id) {
            // Remove from project entries
            if let Some(project_entries) = self.project_entries.get_mut(&entry.project_id) {
                project_entries.retain(|id| id != entry_id);
            }
            Some(entry)
        } else {
            None
        }
    }

    /// Update entry content
    pub fn update_entry(&mut self, entry: &CodexEntry) -> Result<()> {
        if self.entries.contains_key(&entry.id) {
            self.entries.insert(entry.id.clone(), entry.clone());
            Ok(())
        } else {
            anyhow::bail!("Entry not found: {}", entry.id)
        }
    }
}

/// Global database store instance (for demo purposes)
lazy_static::lazy_static! {
    static ref GLOBAL_CODEX_DB: tokio::sync::RwLock<CodexDatabaseStore> = 
        tokio::sync::RwLock::new(CodexDatabaseStore::new());
}

impl CodexDatabaseService {
    /// Create a new codex database service
    pub fn new() -> Self {
        Self {
            database_context: None,
            entry_cache: HashMap::new(),
            search_index: HashMap::new(),
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

    /// Get codex entries for a project with caching
    pub async fn get_entries_by_project(&self, project_id: &str) -> DatabaseOperationResult<Vec<CodexEntry>> {
        let start_time = Instant::now();
        
        // Try cache first
        let cached_entries = self.get_cached_entries(project_id);
        if !cached_entries.is_empty() {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_entries, start_time.elapsed());
        }
        
        // Cache miss - fetch from database
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_entries_by_project",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_CODEX_DB.read().await;
                    let entries: Vec<CodexEntry> = db_store.get_project_entries(project_id)
                        .into_iter()
                        .cloned()
                        .collect();
                    Ok::<Vec<CodexEntry>, String>(entries)
                }),
                3,
            ).await;

            match result {
                Ok(entries) => {
                    // Cache the results
                    self.cache_entries(project_id, &entries);
                    DatabaseOperationResult::success(entries, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_entries", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get codex entries by type
    pub async fn get_entries_by_type(&self, project_id: &str, entry_type: CodexEntryType) -> DatabaseOperationResult<Vec<CodexEntry>> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_entries_by_type",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_CODEX_DB.read().await;
                    let entries: Vec<CodexEntry> = db_store.get_entries_by_type(project_id, entry_type)
                        .into_iter()
                        .cloned()
                        .collect();
                    Ok::<Vec<CodexEntry>, String>(entries)
                }),
                3,
            ).await;

            match result {
                Ok(entries) => {
                    DatabaseOperationResult::success(entries, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_entries_by_type", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Create a new codex entry
    pub async fn create_entry(&self, entry: &CodexEntry) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "create_entry",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_CODEX_DB.write().await;
                    
                    // Validate entry doesn't already exist
                    if db_store.get_entry(&entry.id).is_some() {
                        return Err(format!("Entry already exists: {}", entry.id));
                    }
                    
                    // Add to database
                    db_store.add_entry(entry.clone());
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Invalidate cache for this project
                    self.invalidate_project_cache(&entry.project_id);
                    
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("create_entry", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Update a codex entry
    pub async fn update_entry(&self, entry: &CodexEntry) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "update_entry",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_CODEX_DB.write().await;
                    db_store.update_entry(entry)
                        .map_err(|e| e.to_string())?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Invalidate cache since entry was modified
                    self.invalidate_project_cache(&entry.project_id);
                    
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("update_entry", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Delete a codex entry
    pub async fn delete_entry(&self, entry_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "delete_entry",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_CODEX_DB.write().await;
                    
                    // Get entry to determine project ID
                    let entry = db_store.get_entry(entry_id)
                        .ok_or_else(|| format!("Entry not found: {}", entry_id))?;
                    let project_id = entry.project_id.clone();
                    
                    // Remove from database
                    db_store.remove_entry(entry_id)
                        .ok_or_else(|| format!("Failed to remove entry: {}", entry_id))?;
                    
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Invalidate cache for this project
                    self.invalidate_project_cache(&project_id);
                    
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("delete_entry", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Search codex entries
    pub async fn search_entries(&self, query: &str) -> DatabaseOperationResult<Vec<CodexEntry>> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "search_entries",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_CODEX_DB.read().await;
                    let entries: Vec<CodexEntry> = db_store.search_entries(query)
                        .into_iter()
                        .cloned()
                        .collect();
                    Ok::<Vec<CodexEntry>, String>(entries)
                }),
                3,
            ).await;

            match result {
                Ok(entries) => {
                    DatabaseOperationResult::success(entries, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("search_entries", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get entry by ID
    pub async fn get_entry(&self, entry_id: &str) -> DatabaseOperationResult<Option<CodexEntry>> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_entry) = self.entry_cache.get(entry_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(Some(cached_entry.clone()), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_entry",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_CODEX_DB.read().await;
                    let entry = db_store.get_entry(entry_id).cloned();
                    Ok::<Option<CodexEntry>, String>(entry)
                }),
                3,
            ).await;

            match result {
                Ok(entry) => {
                    // Cache the result if found
                    if let Some(ref entry) = entry {
                        self.entry_cache.insert(entry.id.clone(), entry.clone());
                    }
                    
                    DatabaseOperationResult::success(entry, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_entry", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get codex statistics
    pub async fn get_codex_statistics(&self, project_id: &str) -> DatabaseOperationResult<CodexStatistics> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_codex_statistics",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_CODEX_DB.read().await;
                    let entries = db_store.get_project_entries(project_id);
                    
                    let mut stats = CodexStatistics::new();
                    stats.total_entries = entries.len();
                    stats.total_characters = entries.iter()
                        .map(|entry| entry.content.len())
                        .sum();
                    
                    for entry in entries {
                        match entry.entry_type {
                            CodexEntryType::Character => stats.character_count += 1,
                            CodexEntryType::Location => stats.location_count += 1,
                            CodexEntryType::Item => stats.item_count += 1,
                            CodexEntryType::Organization => stats.organization_count += 1,
                            CodexEntryType::Concept => stats.concept_count += 1,
                            CodexEntryType::Other => stats.other_count += 1,
                        }
                    }
                    
                    Ok::<CodexStatistics, String>(stats)
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

    /// Clear all codex data for a project
    pub async fn clear_project_codex(&self, project_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "clear_project_codex",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_CODEX_DB.write().await;
                    
                    // Get all entries for the project
                    let project_entries = db_store.get_project_entries(project_id);
                    let entry_ids: Vec<String> = project_entries.iter()
                        .map(|entry| entry.id.clone())
                        .collect();
                    
                    // Remove all entries
                    for entry_id in entry_ids {
                        db_store.remove_entry(&entry_id);
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
                    DatabaseOperationResult::validation_error("clear_codex", e.to_string())
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
        self.entry_cache.clear();
        self.search_index.clear();
        self.cache_stats = CacheStats {
            hits: 0,
            misses: 0,
            hit_rate: 0.0,
        };
    }

    /// Get cached entries for a project
    fn get_cached_entries(&self, project_id: &str) -> Vec<CodexEntry> {
        // In a real implementation, we'd have a more sophisticated caching strategy
        // For now, we'll return empty to force database reads
        Vec::new()
    }

    /// Cache entries for a project
    fn cache_entries(&mut self, project_id: &str, entries: &[CodexEntry]) {
        // Cache individual entries
        for entry in entries {
            self.entry_cache.insert(entry.id.clone(), entry.clone());
        }
    }

    /// Invalidate cache for a project
    fn invalidate_project_cache(&mut self, project_id: &str) {
        // In a real implementation with project-based caching, we'd remove
        // all entries for this project from cache
        // For now, we'll clear the entire cache
        self.entry_cache.clear();
    }

    /// Update cache hit rate
    fn update_cache_hit_rate(&mut self) {
        let total_requests = self.cache_stats.hits + self.cache_stats.misses;
        if total_requests > 0 {
            self.cache_stats.hit_rate = (self.cache_stats.hits as f64 / total_requests as f64) * 100.0;
        }
    }
}

/// Statistics about codex data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexStatistics {
    /// Total number of entries
    pub total_entries: usize,
    /// Total character count
    pub total_characters: usize,
    /// Number of character entries
    pub character_count: usize,
    /// Number of location entries
    pub location_count: usize,
    /// Number of item entries
    pub item_count: usize,
    /// Number of organization entries
    pub organization_count: usize,
    /// Number of concept entries
    pub concept_count: usize,
    /// Number of other entries
    pub other_count: usize,
}

impl CodexStatistics {
    /// Create a new statistics instance
    pub fn new() -> Self {
        Self {
            total_entries: 0,
            total_characters: 0,
            character_count: 0,
            location_count: 0,
            item_count: 0,
            organization_count: 0,
            concept_count: 0,
            other_count: 0,
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
    async fn test_codex_database_service_creation() {
        let mut service = CodexDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_codex_db", database_state).await;
        
        service.initialize(database_context).await;
        assert!(service.database_context.is_some());
    }

    #[tokio::test]
    async fn test_codex_entry_crud() {
        let mut service = CodexDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_codex_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create an entry
        let entry = CodexEntry::new(
            "test_entry_1".to_string(),
            "Test Character".to_string(),
            CodexEntryType::Character,
            "This is a test character.".to_string(),
            "test_project".to_string(),
        );
        
        let create_result = service.create_entry(&entry).await;
        assert!(create_result.is_success());
        
        // Get the entry
        let get_result = service.get_entry("test_entry_1").await;
        assert!(get_result.is_success());
        assert!(get_result.data.unwrap().is_some());
        
        // Delete the entry
        let delete_result = service.delete_entry("test_entry_1").await;
        assert!(delete_result.is_success());
        
        // Verify deletion
        let get_after_delete = service.get_entry("test_entry_1").await;
        assert!(get_after_delete.is_success());
        assert!(get_after_delete.data.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_entries_by_project() {
        let mut service = CodexDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_codex_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create test entries
        let character = CodexEntry::new(
            "character_1".to_string(),
            "Test Character".to_string(),
            CodexEntryType::Character,
            "Character description.".to_string(),
            "test_project".to_string(),
        );
        
        let location = CodexEntry::new(
            "location_1".to_string(),
            "Test Location".to_string(),
            CodexEntryType::Location,
            "Location description.".to_string(),
            "test_project".to_string(),
        );
        
        // Add entries
        service.create_entry(&character).await;
        service.create_entry(&location).await;
        
        // Get entries for project
        let entries_result = service.get_entries_by_project("test_project").await;
        assert!(entries_result.is_success());
        
        let entries = entries_result.data.unwrap();
        assert_eq!(entries.len(), 2);
        
        // Verify cache stats
        let stats = service.get_cache_stats();
        assert!(stats.misses > 0); // Should have at least one miss
    }

    #[tokio::test]
    async fn test_entries_by_type() {
        let mut service = CodexDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_codex_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create test entries
        let character1 = CodexEntry::new(
            "character_1".to_string(),
            "Character 1".to_string(),
            CodexEntryType::Character,
            "Character content.".to_string(),
            "test_project".to_string(),
        );
        
        let character2 = CodexEntry::new(
            "character_2".to_string(),
            "Character 2".to_string(),
            CodexEntryType::Character,
            "More character content.".to_string(),
            "test_project".to_string(),
        );
        
        let location = CodexEntry::new(
            "location_1".to_string(),
            "Location 1".to_string(),
            CodexEntryType::Location,
            "Location content.".to_string(),
            "test_project".to_string(),
        );
        
        service.create_entry(&character1).await;
        service.create_entry(&character2).await;
        service.create_entry(&location).await;
        
        // Get characters
        let characters_result = service.get_entries_by_type("test_project", CodexEntryType::Character).await;
        assert!(characters_result.is_success());
        
        let characters = characters_result.data.unwrap();
        assert_eq!(characters.len(), 2);
        
        // Get locations
        let locations_result = service.get_entries_by_type("test_project", CodexEntryType::Location).await;
        assert!(locations_result.is_success());
        
        let locations = locations_result.data.unwrap();
        assert_eq!(locations.len(), 1);
    }

    #[tokio::test]
    async fn test_search_entries() {
        let mut service = CodexDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_codex_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create test entries
        let character = CodexEntry::new(
            "search_test_1".to_string(),
            "Hero Character".to_string(),
            CodexEntryType::Character,
            "This hero has a brave personality.".to_string(),
            "test_project".to_string(),
        );
        
        let location = CodexEntry::new(
            "search_test_2".to_string(),
            "Mystic Forest".to_string(),
            CodexEntryType::Location,
            "A mystical forest with ancient trees.".to_string(),
            "test_project".to_string(),
        );
        
        service.create_entry(&character).await;
        service.create_entry(&location).await;
        
        // Search for "hero"
        let search_result = service.search_entries("hero").await;
        assert!(search_result.is_success());
        
        let results = search_result.data.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "search_test_1");
        
        // Search for "forest"
        let search_result2 = service.search_entries("forest").await;
        assert!(search_result2.is_success());
        
        let results2 = search_result2.data.unwrap();
        assert_eq!(results2.len(), 1);
        assert_eq!(results2[0].id, "search_test_2");
    }

    #[tokio::test]
    async fn test_codex_statistics() {
        let mut service = CodexDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_codex_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Create test entries
        let character = CodexEntry::new(
            "stats_char".to_string(),
            "Test Character".to_string(),
            CodexEntryType::Character,
            "Character with 20 characters.".to_string(),
            "test_project".to_string(),
        );
        
        let location = CodexEntry::new(
            "stats_loc".to_string(),
            "Test Location".to_string(),
            CodexEntryType::Location,
            "Location with 20 characters.".to_string(),
            "test_project".to_string(),
        );
        
        service.create_entry(&character).await;
        service.create_entry(&location).await;
        
        // Get statistics
        let stats_result = service.get_codex_statistics("test_project").await;
        assert!(stats_result.is_success());
        
        let stats = stats_result.data.unwrap();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.total_characters, 40); // 20 + 20
        assert_eq!(stats.character_count, 1);
        assert_eq!(stats.location_count, 1);
    }
}