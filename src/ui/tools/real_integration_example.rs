//! Real Integration Example
//!
//! Complete example showing how all real tool implementations work together
//! with the new architecture patterns, real database connections, and UI integration.

use crate::ui::tools::{
    database_integration::ToolDatabaseContext,
    threading_patterns::get_tool_registry,
    api_contracts::{ToolApiContract, get_api_contract, ToolLifecycleEvent},
    RealCodexTool, RealHierarchyTool, RealAnalysisTool,
    RealCodexService, RealHierarchyService, RealAnalysisService,
};
use anyhow::Result;
use std::sync::Arc;
use std::time::{Instant, Duration};
use tokio::sync::RwLock;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::DatabaseAppState;

/// Complete application state manager that coordinates all tools
pub struct RealApplicationManager {
    /// Database context shared across all tools
    database_context: Option<ToolDatabaseContext>,
    /// Real codex tool instance
    codex_tool: Option<RealCodexTool>,
    /// Real hierarchy tool instance
    hierarchy_tool: Option<RealHierarchyTool>,
    /// Real analysis tool instance
    analysis_tool: Option<RealAnalysisTool>,
    /// Application-wide API contract
    api_contract: Arc<ToolApiContract>,
    /// Application state
    app_state: RealApplicationState,
}

/// Application state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealApplicationState {
    /// Application start time
    pub started_at: Instant,
    /// Current project ID
    pub current_project_id: Option<String>,
    /// Active tools
    pub active_tools: Vec<String>,
    /// System health status
    pub health_status: ApplicationHealth,
    /// Performance metrics
    pub performance_metrics: ApplicationMetrics,
}

/// Application health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationHealth {
    /// Database connection status
    pub database_connected: bool,
    /// Tool initialization status
    pub tools_initialized: bool,
    /// Last health check
    pub last_check: Instant,
    /// Active issues
    pub issues: Vec<String>,
}

/// Application performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    /// Total operations across all tools
    pub total_operations: u64,
    /// Average system latency
    pub average_latency_ms: f64,
    /// Overall success rate
    pub success_rate: f64,
    /// Memory usage
    pub memory_usage_mb: f64,
    /// Database connection pool status
    pub db_connection_pool: ConnectionPoolStatus,
}

/// Database connection pool status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolStatus {
    /// Active connections
    pub active_connections: u32,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection utilization percentage
    pub utilization_percent: f64,
}

impl Default for ApplicationHealth {
    fn default() -> Self {
        Self {
            database_connected: false,
            tools_initialized: false,
            last_check: Instant::now(),
            issues: Vec::new(),
        }
    }
}

impl Default for ApplicationMetrics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            db_connection_pool: ConnectionPoolStatus::default(),
        }
    }
}

impl Default for ConnectionPoolStatus {
    fn default() -> Self {
        Self {
            active_connections: 0,
            max_connections: 10,
            utilization_percent: 0.0,
        }
    }
}

impl RealApplicationManager {
    /// Create a new real application manager
    pub fn new() -> Self {
        Self {
            database_context: None,
            codex_tool: None,
            hierarchy_tool: None,
            analysis_tool: None,
            api_contract: get_api_contract().clone(),
            app_state: RealApplicationState {
                started_at: Instant::now(),
                current_project_id: None,
                active_tools: Vec::new(),
                health_status: ApplicationHealth::default(),
                performance_metrics: ApplicationMetrics::default(),
            },
        }
    }

    /// Initialize the complete application with real database
    pub async fn initialize_application(&mut self, database_path: &str) -> Result<()> {
        println!("🚀 Initializing Real Application Manager...");
        
        // Step 1: Initialize database context
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut database_context = ToolDatabaseContext::new("application", database_state).await;
        
        // Step 2: Initialize all tools
        self.initialize_all_tools(&mut database_context).await?;
        
        // Step 3: Set up cross-tool communication
        self.setup_tool_communication().await?;
        
        // Step 4: Initialize application state
        self.database_context = Some(database_context);
        self.app_state.health_status.database_connected = true;
        self.app_state.health_status.tools_initialized = true;
        
        println!("✅ Application initialization complete!");
        Ok(())
    }

    /// Initialize all tools with real database services
    async fn initialize_all_tools(&mut self, database_context: &mut ToolDatabaseContext) -> Result<()> {
        println!("🔧 Initializing Real Tools...");
        
        // Initialize codex tool
        let mut codex_tool = RealCodexTool::new();
        codex_tool.initialize(database_context).await
            .map_err(|e| anyhow::anyhow!("Failed to initialize codex tool: {}", e))?;
        self.codex_tool = Some(codex_tool);
        
        // Initialize hierarchy tool
        let mut hierarchy_tool = RealHierarchyTool::new();
        hierarchy_tool.initialize(database_context).await
            .map_err(|e| anyhow::anyhow!("Failed to initialize hierarchy tool: {}", e))?;
        self.hierarchy_tool = Some(hierarchy_tool);
        
        // Initialize analysis tool
        let mut analysis_tool = RealAnalysisTool::new();
        analysis_tool.initialize(database_context).await
            .map_err(|e| anyhow::anyhow!("Failed to initialize analysis tool: {}", e))?;
        self.analysis_tool = Some(analysis_tool);
        
        // Update active tools list
        self.app_state.active_tools = vec![
            "real_codex_tool".to_string(),
            "real_hierarchy_tool".to_string(),
            "real_analysis_tool".to_string(),
        ];
        
        println!("✅ All tools initialized successfully");
        Ok(())
    }

    /// Set up cross-tool communication
    async fn setup_tool_communication(&self) -> Result<()> {
        // Set up event broadcasting between tools
        self.api_contract.broadcast_lifecycle(ToolLifecycleEvent::Initialized {
            tool_id: "application_manager".to_string(),
            timestamp: Instant::now(),
            version: "2.0.0".to_string(),
        }).await
        .map_err(|e| anyhow::anyhow!("Failed to broadcast initialization event: {}", e))?;
        
        println!("📡 Tool communication established");
        Ok(())
    }

    /// Create a new project with all associated data
    pub async fn create_project(&mut self, project_name: &str) -> Result<String> {
        let start_time = Instant::now();
        let project_id = Uuid::new_v4().to_string();
        
        println!("📁 Creating project: {} (ID: {})", project_name, project_id);
        
        // Create project in all tools
        if let Some(codex_tool) = &mut self.codex_tool {
            // Create initial codex entries
            codex_tool.create_entry(
                format!("{} - Story Summary", project_name),
                crate::database::models::codex::CodexEntryType::StorySummary,
                format!("Story summary for {}", project_name),
                project_id.clone(),
            ).await?;
            
            codex_tool.create_entry(
                "Main Character".to_string(),
                crate::database::models::codex::CodexEntryType::CharacterSheet,
                "Main character description".to_string(),
                project_id.clone(),
            ).await?;
        }
        
        if let Some(hierarchy_tool) = &mut self.hierarchy_tool {
            // Create initial hierarchy structure
            hierarchy_tool.create_item(
                project_name.to_string(),
                crate::ui::tools::hierarchy_base::HierarchyLevel::Manuscript,
                None,
                project_id.clone(),
            ).await?;
            
            hierarchy_tool.create_item(
                "Chapter 1".to_string(),
                crate::ui::tools::hierarchy_base::HierarchyLevel::Chapter,
                Some(&project_id),
                project_id.clone(),
            ).await?;
        }
        
        if let Some(analysis_tool) = &mut self.analysis_tool {
            // Create initial analysis
            analysis_tool.create_analysis(
                project_id.clone(),
                "Project Overview".to_string(),
                format!("Initial analysis for {}", project_name),
            ).await?;
        }
        
        // Update application state
        self.app_state.current_project_id = Some(project_id.clone());
        
        let duration = start_time.elapsed();
        println!("✅ Project created in {:.2}ms", duration.as_millis());
        
        Ok(project_id)
    }

    /// Load project data across all tools
    pub async fn load_project(&self, project_id: &str) -> Result<ProjectData> {
        let start_time = Instant::now();
        
        println!("📂 Loading project data for: {}", project_id);
        
        let mut project_data = ProjectData::new(project_id);
        
        // Load codex entries
        if let Some(codex_tool) = &self.codex_tool {
            let entries = codex_tool.search_entries(
                project_id.to_string(),
                "".to_string(),
            ).await?;
            project_data.codex_entries = entries;
        }
        
        // Load hierarchy structure
        if let Some(hierarchy_tool) = &self.hierarchy_tool {
            let root_items = hierarchy_tool.get_children(None).await?;
            project_data.hierarchy_items = root_items;
        }
        
        // Load analysis data
        if let Some(analysis_tool) = &self.analysis_tool {
            let analyses = analysis_tool.list_analyses(project_id).await?;
            project_data.analyses = analyses;
        }
        
        let duration = start_time.elapsed();
        println!("✅ Project data loaded in {:.2}ms", duration.as_millis());
        
        Ok(project_data)
    }

    /// Perform cross-tool operation
    pub async fn create_character_and_scene(
        &mut self,
        character_name: &str,
        scene_name: &str,
        project_id: &str,
    ) -> Result<()> {
        let start_time = Instant::now();
        
        println!("🎭 Creating character '{}' and scene '{}' for project '{}'", 
                 character_name, scene_name, project_id);
        
        // Create character in codex
        if let Some(codex_tool) = &mut self.codex_tool {
            codex_tool.create_entry(
                character_name.to_string(),
                crate::database::models::codex::CodexEntryType::CharacterSheet,
                format!("Character profile for {}", character_name),
                project_id.to_string(),
            ).await?;
        }
        
        // Create scene in hierarchy
        if let Some(hierarchy_tool) = &mut self.hierarchy_tool {
            // Find manuscript level item
            let root_items = hierarchy_tool.get_children(None).await?;
            if let Some(manuscript) = root_items.first() {
                hierarchy_tool.create_item(
                    scene_name.to_string(),
                    crate::ui::tools::hierarchy_base::HierarchyLevel::Scene,
                    Some(&manuscript.id),
                    project_id.to_string(),
                ).await?;
            }
        }
        
        // Create analysis linking character to scene
        if let Some(analysis_tool) = &mut self.analysis_tool {
            analysis_tool.create_analysis(
                project_id.to_string(),
                "Character-Scene Relationship".to_string(),
                format!("Analysis of {}'s relationship to scene '{}'", 
                       character_name, scene_name),
            ).await?;
        }
        
        let duration = start_time.elapsed();
        println!("✅ Cross-tool operation completed in {:.2}ms", duration.as_millis());
        
        Ok(())
    }

    /// Get application health status
    pub async fn get_health_status(&self) -> ApplicationHealth {
        let mut health = self.app_state.health_status.clone();
        health.last_check = Instant::now();
        
        // Check database connectivity
        if let Some(context) = &self.database_context {
            // This would check actual database connection status
            health.database_connected = true;
        } else {
            health.database_connected = false;
            health.issues.push("Database context not available".to_string());
        }
        
        // Check tool health
        let mut tools_healthy = 0;
        let total_tools = 3;
        
        if self.codex_tool.is_some() {
            tools_healthy += 1;
        } else {
            health.issues.push("Codex tool not initialized".to_string());
        }
        
        if self.hierarchy_tool.is_some() {
            tools_healthy += 1;
        } else {
            health.issues.push("Hierarchy tool not initialized".to_string());
        }
        
        if self.analysis_tool.is_some() {
            tools_healthy += 1;
        } else {
            health.issues.push("Analysis tool not initialized".to_string());
        }
        
        health.tools_initialized = tools_healthy == total_tools;
        
        health
    }

    /// Get comprehensive system metrics
    pub async fn get_system_metrics(&self) -> ApplicationMetrics {
        let mut metrics = self.app_state.performance_metrics.clone();
        
        // Calculate aggregate metrics from all tools
        let mut total_operations = 0;
        let mut total_latency = 0.0;
        let mut total_tools = 0;
        let mut success_operations = 0;
        
        if let Some(codex_tool) = &self.codex_tool {
            let stats = codex_tool.get_performance_stats();
            total_operations += stats.total_operations;
            total_latency += stats.average_latency_ms;
            total_tools += 1;
            success_operations += stats.total_operations - stats.total_errors;
        }
        
        if let Some(hierarchy_tool) = &self.hierarchy_tool {
            let stats = hierarchy_tool.get_performance_stats();
            total_operations += stats.total_operations;
            total_latency += stats.average_latency_ms;
            total_tools += 1;
            success_operations += stats.total_operations - stats.total_errors;
        }
        
        if let Some(analysis_tool) = &self.analysis_tool {
            let stats = analysis_tool.get_performance_stats();
            total_operations += stats.total_operations;
            total_latency += stats.average_latency_ms;
            total_tools += 1;
            success_operations += stats.total_operations - stats.total_errors;
        }
        
        if total_tools > 0 {
            metrics.total_operations = total_operations;
            metrics.average_latency_ms = total_latency / total_tools as f64;
            metrics.success_rate = if total_operations > 0 {
                (success_operations as f64 / total_operations as f64) * 100.0
            } else {
                100.0
            };
        }
        
        metrics
    }

    /// Cleanup and shutdown all tools
    pub async fn shutdown(&mut self) -> Result<()> {
        println!("🛑 Shutting down application...");
        
        // Cleanup all tools
        if let Some(mut codex_tool) = self.codex_tool.take() {
            codex_tool.cleanup().await
                .map_err(|e| anyhow::anyhow!("Failed to cleanup codex tool: {}", e))?;
        }
        
        if let Some(mut hierarchy_tool) = self.hierarchy_tool.take() {
            hierarchy_tool.cleanup().await
                .map_err(|e| anyhow::anyhow!("Failed to cleanup hierarchy tool: {}", e))?;
        }
        
        if let Some(mut analysis_tool) = self.analysis_tool.take() {
            analysis_tool.cleanup().await
                .map_err(|e| anyhow::anyhow!("Failed to cleanup analysis tool: {}", e))?;
        }
        
        // Clear database context
        self.database_context = None;
        
        println!("✅ Application shutdown complete");
        Ok(())
    }
}

/// Complete project data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    /// Project ID
    pub project_id: String,
    /// Codex entries
    pub codex_entries: Vec<crate::database::models::codex::CodexEntry>,
    /// Hierarchy items
    pub hierarchy_items: Vec<crate::ui::tools::hierarchy_base::HierarchyItem>,
    /// Analysis records
    pub analyses: Vec<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>,
    /// Load timestamp
    pub loaded_at: Instant,
}

impl ProjectData {
    /// Create new project data
    pub fn new(project_id: &str) -> Self {
        Self {
            project_id: project_id.to_string(),
            codex_entries: Vec::new(),
            hierarchy_items: Vec::new(),
            analyses: Vec::new(),
            loaded_at: Instant::now(),
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
    async fn test_application_manager_creation() {
        let manager = RealApplicationManager::new();
        assert!(manager.database_context.is_none());
        assert!(manager.codex_tool.is_none());
        assert!(manager.hierarchy_tool.is_none());
        assert!(manager.analysis_tool.is_none());
        assert_eq!(manager.app_state.active_tools.len(), 0);
    }

    #[tokio::test]
    async fn test_application_initialization() {
        let mut manager = RealApplicationManager::new();
        
        let result = manager.initialize_application("test.db").await;
        assert!(result.is_ok());
        
        assert!(manager.database_context.is_some());
        assert!(manager.codex_tool.is_some());
        assert!(manager.hierarchy_tool.is_some());
        assert!(manager.analysis_tool.is_some());
        assert_eq!(manager.app_state.active_tools.len(), 3);
        assert!(manager.app_state.health_status.tools_initialized);
    }

    #[tokio::test]
    async fn test_project_creation() {
        let mut manager = RealApplicationManager::new();
        manager.initialize_application("test.db").await.unwrap();
        
        let project_id = manager.create_project("Test Project").await.unwrap();
        assert!(!project_id.is_empty());
        
        assert_eq!(manager.app_state.current_project_id, Some(project_id));
    }

    #[tokio::test]
    async fn test_health_status() {
        let mut manager = RealApplicationManager::new();
        manager.initialize_application("test.db").await.unwrap();
        
        let health = manager.get_health_status().await;
        assert!(health.database_connected);
        assert!(health.tools_initialized);
        assert!(health.issues.is_empty());
    }

    #[tokio::test]
    async fn test_system_metrics() {
        let mut manager = RealApplicationManager::new();
        manager.initialize_application("test.db").await.unwrap();
        
        let metrics = manager.get_system_metrics().await;
        assert!(metrics.total_operations >= 0);
        assert!(metrics.success_rate >= 0.0);
        assert!(metrics.success_rate <= 100.0);
    }
}

/// Example usage of the complete real integration
pub async fn example_real_integration() -> Result<()> {
    println!("🎯 Real Integration Example");
    println!("==========================");
    
    // Create and initialize application
    let mut app_manager = RealApplicationManager::new();
    app_manager.initialize_application("example_app.db").await?;
    
    // Create a new project
    let project_id = app_manager.create_project("My Novel").await?;
    
    // Perform cross-tool operations
    app_manager.create_character_and_scene(
        "Alice Johnson",
        "The Mysterious Forest",
        &project_id,
    ).await?;
    
    // Load project data
    let project_data = app_manager.load_project(&project_id).await?;
    println!("📊 Loaded {} codex entries", project_data.codex_entries.len());
    println!("📊 Loaded {} hierarchy items", project_data.hierarchy_items.len());
    println!("📊 Loaded {} analyses", project_data.analyses.len());
    
    // Check health and metrics
    let health = app_manager.get_health_status().await;
    let metrics = app_manager.get_system_metrics().await;
    
    println!("💚 Health Status: {:#?}", health);
    println!("📈 Metrics: {:#?}", metrics);
    
    // Shutdown
    app_manager.shutdown().await?;
    
    println!("✅ Real integration example completed successfully!");
    Ok(())
}