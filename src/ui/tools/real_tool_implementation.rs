//! Real Tool Implementation
//!
//! Production-ready tool implementations that connect to real database services
//! and provide actual functionality for codex, hierarchy, and analysis tools.

use crate::ui::tools::{
    database_integration::ToolDatabaseContext,
    threading_patterns::get_tool_registry,
    api_contracts::{ToolApiContract, get_api_contract, ToolLifecycleEvent},
    ToolIntegration, ToolType, RealCodexService, RealHierarchyService, RealAnalysisService,
};
use anyhow::Result;
use std::sync::Arc;
use std::time::{Instant, Duration};
use tokio::sync::RwLock;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Real codex tool implementation with actual database integration
pub struct RealCodexTool {
    /// Database context for operations
    database_context: Option<ToolDatabaseContext>,
    /// API contract for cross-tool communication
    api_contract: Arc<ToolApiContract>,
    /// Real codex service for database operations
    codex_service: Option<RealCodexService>,
    /// Tool registry for lifecycle management
    tool_registry: &'static crate::ui::tools::threading_patterns::ThreadSafeToolRegistry,
    /// Tool metadata
    tool_id: String,
    initialized_at: Option<Instant>,
    last_operation_duration: Option<Duration>,
    performance_stats: ToolPerformanceStats,
}

/// Real hierarchy tool implementation
pub struct RealHierarchyTool {
    /// Database context for operations
    database_context: Option<ToolDatabaseContext>,
    /// API contract for cross-tool communication
    api_contract: Arc<ToolApiContract>,
    /// Real hierarchy service for database operations
    hierarchy_service: Option<RealHierarchyService>,
    /// Tool registry for lifecycle management
    tool_registry: &'static crate::ui::tools::threading_patterns::ThreadSafeToolRegistry,
    /// Tool metadata
    tool_id: String,
    initialized_at: Option<Instant>,
    last_operation_duration: Option<Duration>,
    performance_stats: ToolPerformanceStats,
}

/// Real analysis tool implementation
pub struct RealAnalysisTool {
    /// Database context for operations
    database_context: Option<ToolDatabaseContext>,
    /// API contract for cross-tool communication
    api_contract: Arc<ToolApiContract>,
    /// Real analysis service for database operations
    analysis_service: Option<RealAnalysisService>,
    /// Tool registry for lifecycle management
    tool_registry: &'static crate::ui::tools::threading_patterns::ThreadSafeToolRegistry,
    /// Tool metadata
    tool_id: String,
    initialized_at: Option<Instant>,
    last_operation_duration: Option<Duration>,
    performance_stats: ToolPerformanceStats,
}

/// Common performance statistics for all tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPerformanceStats {
    pub total_operations: u64,
    pub average_latency_ms: f64,
    pub success_rate: f64,
    pub total_errors: u64,
    pub cache_hit_rate: f64,
}

impl Default for ToolPerformanceStats {
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

impl RealCodexTool {
    /// Create a new real codex tool
    pub fn new() -> Self {
        Self {
            database_context: None,
            api_contract: get_api_contract().clone(),
            codex_service: None,
            tool_registry: get_tool_registry(),
            tool_id: format!("real_codex_{}", Uuid::new_v4()),
            initialized_at: None,
            last_operation_duration: None,
            performance_stats: ToolPerformanceStats::default(),
        }
    }

    /// Initialize the real codex service
    pub async fn initialize_service(&mut self) -> Result<()> {
        if let Some(database_context) = &self.database_context {
            let mut service = RealCodexService::new();
            service.initialize(database_context.clone()).await;
            
            // Initialize the database schema
            service.initialize_schema().await
                .map_err(|e| anyhow::anyhow!("Failed to initialize codex schema: {}", e))?;
            
            self.codex_service = Some(service);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Database context not available"))
        }
    }

    /// Create a new codex entry
    pub async fn create_entry(
        &mut self,
        title: String,
        entry_type: crate::database::models::codex::CodexEntryType,
        content: String,
        project_id: String,
    ) -> Result<crate::database::models::codex::CodexEntry> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.codex_service {
            let entry = crate::database::models::codex::CodexEntry::new(
                Uuid::new_v4(),
                title,
                entry_type,
                content,
                project_id,
            );
            
            let entry_id = service.create_entry(&entry).await
                .map_err(|e| anyhow::anyhow!("Failed to create codex entry: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            // Broadcast creation event
            self.broadcast_event("entry_created").await;
            
            Ok(entry)
        } else {
            Err(anyhow::anyhow!("Codex service not initialized"))
        }
    }

    /// Get codex entry by ID
    pub async fn get_entry(
        &self,
        entry_id: Uuid,
    ) -> Result<Option<crate::database::models::codex::CodexEntry>> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.codex_service {
            let result = service.get_entry(&entry_id).await
                .map_err(|e| anyhow::anyhow!("Failed to get codex entry: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Codex service not initialized"))
        }
    }

    /// Search codex entries
    pub async fn search_entries(
        &self,
        project_id: String,
        search_term: String,
    ) -> Result<Vec<crate::database::models::codex::CodexEntry>> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.codex_service {
            let query = crate::database::models::codex::CodexQuery {
                project_id: Some(Uuid::parse_str(&project_id).unwrap_or_default()),
                search_term: Some(search_term),
                ..Default::default()
            };
            
            let results = service.list_entries(&query).await
                .map_err(|e| anyhow::anyhow!("Failed to search codex entries: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(results)
        } else {
            Err(anyhow::anyhow!("Codex service not initialized"))
        }
    }

    /// Get codex statistics
    pub async fn get_statistics(
        &self,
        project_id: String,
    ) -> Result<crate::database::models::codex::CodexStatistics> {
        if let Some(service) = &self.codex_service {
            service.get_statistics(&Uuid::parse_str(&project_id).unwrap_or_default()).await
                .map_err(|e| anyhow::anyhow!("Failed to get codex statistics: {}", e))
        } else {
            Err(anyhow::anyhow!("Codex service not initialized"))
        }
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> Option<crate::ui::tools::real_codex_service::CacheStats> {
        if let Some(service) = &self.codex_service {
            Some(service.get_cache_stats().await)
        } else {
            None
        }
    }

    /// Clear cache
    pub async fn clear_cache(&self) -> Result<()> {
        if let Some(service) = &self.codex_service {
            service.clear_cache().await;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Codex service not initialized"))
        }
    }
}

impl RealHierarchyTool {
    /// Create a new real hierarchy tool
    pub fn new() -> Self {
        Self {
            database_context: None,
            api_contract: get_api_contract().clone(),
            hierarchy_service: None,
            tool_registry: get_tool_registry(),
            tool_id: format!("real_hierarchy_{}", Uuid::new_v4()),
            initialized_at: None,
            last_operation_duration: None,
            performance_stats: ToolPerformanceStats::default(),
        }
    }

    /// Initialize the real hierarchy service
    pub async fn initialize_service(&mut self) -> Result<()> {
        if let Some(database_context) = &self.database_context {
            let mut service = RealHierarchyService::new();
            service.initialize(database_context.clone()).await;
            
            // Initialize the database schema
            service.initialize_schema().await
                .map_err(|e| anyhow::anyhow!("Failed to initialize hierarchy schema: {}", e))?;
            
            self.hierarchy_service = Some(service);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Database context not available"))
        }
    }

    /// Create a new hierarchy item
    pub async fn create_item(
        &mut self,
        title: String,
        level: crate::ui::tools::hierarchy_base::HierarchyLevel,
        parent_id: Option<String>,
        project_id: String,
    ) -> Result<String> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.hierarchy_service {
            let item = crate::ui::tools::hierarchy_base::HierarchyItem::new(
                format!("hierarchy_{}", Uuid::new_v4()),
                title,
                level,
                parent_id,
                project_id,
            );
            
            service.create_item(&item).await
                .map_err(|e| anyhow::anyhow!("Failed to create hierarchy item: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(item.id)
        } else {
            Err(anyhow::anyhow!("Hierarchy service not initialized"))
        }
    }

    /// Get hierarchy item by ID
    pub async fn get_item(
        &self,
        item_id: &str,
    ) -> Result<Option<crate::ui::tools::hierarchy_base::HierarchyItem>> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.hierarchy_service {
            let result = service.get_item(item_id).await
                .map_err(|e| anyhow::anyhow!("Failed to get hierarchy item: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Hierarchy service not initialized"))
        }
    }

    /// Get children of a hierarchy item
    pub async fn get_children(
        &self,
        parent_id: Option<&str>,
    ) -> Result<Vec<crate::ui::tools::hierarchy_base::HierarchyItem>> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.hierarchy_service {
            let results = service.get_children(parent_id).await
                .map_err(|e| anyhow::anyhow!("Failed to get hierarchy children: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(results)
        } else {
            Err(anyhow::anyhow!("Hierarchy service not initialized"))
        }
    }

    /// Move hierarchy item to new parent
    pub async fn move_item(
        &mut self,
        item_id: &str,
        new_parent_id: Option<&str>,
    ) -> Result<()> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.hierarchy_service {
            service.move_item(item_id, new_parent_id).await
                .map_err(|e| anyhow::anyhow!("Failed to move hierarchy item: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Hierarchy service not initialized"))
        }
    }
}

impl RealAnalysisTool {
    /// Create a new real analysis tool
    pub fn new() -> Self {
        Self {
            database_context: None,
            api_contract: get_api_contract().clone(),
            analysis_service: None,
            tool_registry: get_tool_registry(),
            tool_id: format!("real_analysis_{}", Uuid::new_v4()),
            initialized_at: None,
            last_operation_duration: None,
            performance_stats: ToolPerformanceStats::default(),
        }
    }

    /// Initialize the real analysis service
    pub async fn initialize_service(&mut self) -> Result<()> {
        if let Some(database_context) = &self.database_context {
            let mut service = RealAnalysisService::new();
            service.initialize(database_context.clone()).await;
            
            // Initialize the database schema
            service.initialize_schema().await
                .map_err(|e| anyhow::anyhow!("Failed to initialize analysis schema: {}", e))?;
            
            self.analysis_service = Some(service);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Database context not available"))
        }
    }

    /// Create analysis record
    pub async fn create_analysis(
        &mut self,
        project_id: String,
        analysis_type: String,
        content: String,
    ) -> Result<String> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.analysis_service {
            let analysis = crate::ui::tools::analysis_tool_migrated::AnalysisRecord {
                id: format!("analysis_{}", Uuid::new_v4()),
                project_id,
                analysis_type,
                content,
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                updated_at: Some(chrono::Utc::now().to_rfc3339()),
                is_active: true,
            };
            
            service.create_analysis(&analysis).await
                .map_err(|e| anyhow::anyhow!("Failed to create analysis: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(analysis.id)
        } else {
            Err(anyhow::anyhow!("Analysis service not initialized"))
        }
    }

    /// Get analysis by ID
    pub async fn get_analysis(
        &self,
        analysis_id: &str,
    ) -> Result<Option<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.analysis_service {
            let result = service.get_analysis(analysis_id).await
                .map_err(|e| anyhow::anyhow!("Failed to get analysis: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Analysis service not initialized"))
        }
    }

    /// List analyses for a project
    pub async fn list_analyses(
        &self,
        project_id: &str,
    ) -> Result<Vec<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>> {
        let start_time = Instant::now();
        
        if let Some(service) = &self.analysis_service {
            let results = service.list_analyses(project_id).await
                .map_err(|e| anyhow::anyhow!("Failed to list analyses: {}", e))?;
            
            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_performance_stats(true, duration);
            
            Ok(results)
        } else {
            Err(anyhow::anyhow!("Analysis service not initialized"))
        }
    }
}

// Common implementation for all real tools
impl RealCodexTool {
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

    /// Broadcast tool event
    async fn broadcast_event(&self, event_type: &str) {
        if let Err(e) = self.api_contract.broadcast_lifecycle(ToolLifecycleEvent::CustomEvent {
            tool_id: self.tool_id.clone(),
            event_type: event_type.to_string(),
            timestamp: Instant::now(),
            data: Some(format!("RealCodexTool event: {}", event_type)),
        }).await {
            eprintln!("Failed to broadcast codex event: {}", e);
        }
    }
}

// Implement ToolIntegration for RealCodexTool
#[async_trait]
impl ToolIntegration for RealCodexTool {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        self.database_context = Some(database_context.clone());
        
        // Register with tool registry
        self.tool_registry.register_tool(
            self.tool_id.clone(), 
            Arc::new(()) as Arc<dyn Send + Sync + 'static>
        ).await.map_err(|e| e.to_string())?;
        
        // Initialize the real service
        self.initialize_service().await
            .map_err(|e| format!("Failed to initialize codex service: {}", e))?;
        
        self.initialized_at = Some(Instant::now());
        self.broadcast_event("initialized").await;
        
        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        // Perform periodic updates
        if let Some(service) = &self.codex_service {
            // Check cache statistics and potentially clear if needed
            // Update any internal state
        }
        
        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
        // Validate tool is in renderable state
        if self.database_context.is_none() {
            return Err("Database context not initialized".to_string());
        }
        
        if self.codex_service.is_none() {
            return Err("Codex service not initialized".to_string());
        }
        
        Ok(())
    }

    async fn cleanup(&mut self) -> Result<(), String> {
        self.broadcast_event("cleanup_started").await;
        
        // Clear service
        self.codex_service = None;
        
        // Unregister from tool registry
        self.tool_registry.unregister_tool(&self.tool_id).await
            .map_err(|e| e.to_string())?;
        
        // Clear database context
        self.database_context = None;
        
        Ok(())
    }
}

// Implement ToolType for RealCodexTool
impl ToolType for RealCodexTool {
    fn display_name(&self) -> &'static str {
        "Real Codex Tool"
    }
    
    fn tool_id(&self) -> &'static str {
        "real_codex_tool"
    }
    
    fn version(&self) -> &'static str {
        "2.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Production-ready codex tool with real database integration"
    }
}

// Implement ToolIntegration for RealHierarchyTool
#[async_trait]
impl ToolIntegration for RealHierarchyTool {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        self.database_context = Some(database_context.clone());
        
        // Register with tool registry
        self.tool_registry.register_tool(
            self.tool_id.clone(), 
            Arc::new(()) as Arc<dyn Send + Sync + 'static>
        ).await.map_err(|e| e.to_string())?;
        
        // Initialize the real service
        self.initialize_service().await
            .map_err(|e| format!("Failed to initialize hierarchy service: {}", e))?;
        
        self.initialized_at = Some(Instant::now());
        
        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        // Perform periodic updates
        if let Some(service) = &self.hierarchy_service {
            // Update any internal state
        }
        
        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
        // Validate tool is in renderable state
        if self.database_context.is_none() {
            return Err("Database context not initialized".to_string());
        }
        
        if self.hierarchy_service.is_none() {
            return Err("Hierarchy service not initialized".to_string());
        }
        
        Ok(())
    }

    async fn cleanup(&mut self) -> Result<(), String> {
        // Clear service
        self.hierarchy_service = None;
        
        // Unregister from tool registry
        self.tool_registry.unregister_tool(&self.tool_id).await
            .map_err(|e| e.to_string())?;
        
        // Clear database context
        self.database_context = None;
        
        Ok(())
    }
}

// Implement ToolType for RealHierarchyTool
impl ToolType for RealHierarchyTool {
    fn display_name(&self) -> &'static str {
        "Real Hierarchy Tool"
    }
    
    fn tool_id(&self) -> &'static str {
        "real_hierarchy_tool"
    }
    
    fn version(&self) -> &'static str {
        "2.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Production-ready hierarchy tool with real database integration"
    }
}

// Implement ToolIntegration for RealAnalysisTool
#[async_trait]
impl ToolIntegration for RealAnalysisTool {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        self.database_context = Some(database_context.clone());
        
        // Register with tool registry
        self.tool_registry.register_tool(
            self.tool_id.clone(), 
            Arc::new(()) as Arc<dyn Send + Sync + 'static>
        ).await.map_err(|e| e.to_string())?;
        
        // Initialize the real service
        self.initialize_service().await
            .map_err(|e| format!("Failed to initialize analysis service: {}", e))?;
        
        self.initialized_at = Some(Instant::now());
        
        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        // Perform periodic updates
        if let Some(service) = &self.analysis_service {
            // Update any internal state
        }
        
        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
        // Validate tool is in renderable state
        if self.database_context.is_none() {
            return Err("Database context not initialized".to_string());
        }
        
        if self.analysis_service.is_none() {
            return Err("Analysis service not initialized".to_string());
        }
        
        Ok(())
    }

    async fn cleanup(&mut self) -> Result<(), String> {
        // Clear service
        self.analysis_service = None;
        
        // Unregister from tool registry
        self.tool_registry.unregister_tool(&self.tool_id).await
            .map_err(|e| e.to_string())?;
        
        // Clear database context
        self.database_context = None;
        
        Ok(())
    }
}

// Implement ToolType for RealAnalysisTool
impl ToolType for RealAnalysisTool {
    fn display_name(&self) -> &'static str {
        "Real Analysis Tool"
    }
    
    fn tool_id(&self) -> &'static str {
        "real_analysis_tool"
    }
    
    fn version(&self) -> &'static str {
        "2.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Production-ready analysis tool with real database integration"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseAppState;
    use tokio::sync::RwLock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_real_codex_tool_creation() {
        let tool = RealCodexTool::new();
        assert!(tool.database_context.is_none());
        assert!(tool.codex_service.is_none());
        assert!(tool.initialized_at.is_none());
    }

    #[tokio::test]
    async fn test_real_hierarchy_tool_creation() {
        let tool = RealHierarchyTool::new();
        assert!(tool.database_context.is_none());
        assert!(tool.hierarchy_service.is_none());
        assert!(tool.initialized_at.is_none());
    }

    #[tokio::test]
    async fn test_real_analysis_tool_creation() {
        let tool = RealAnalysisTool::new();
        assert!(tool.database_context.is_none());
        assert!(tool.analysis_service.is_none());
        assert!(tool.initialized_at.is_none());
    }

    #[tokio::test]
    async fn test_tool_initialization() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = RealCodexTool::new();
        let mut database_context = ToolDatabaseContext::new("test_tool", database_state).await;
        
        let result = tool.initialize(&mut database_context).await;
        assert!(result.is_ok());
        assert!(tool.database_context.is_some());
        assert!(tool.codex_service.is_some());
        assert!(tool.initialized_at.is_some());
    }
}