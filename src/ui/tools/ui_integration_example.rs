//! UI Integration Example
//!
//! Complete example showing how all real tool implementations integrate with the UI,
//! demonstrating real database operations, cross-tool communication, and user interactions.

use crate::ui::tools::{
    RealApplicationManager,
    RealCodexTool, RealHierarchyTool, RealAnalysisTool,
    database_integration::ToolDatabaseContext,
    threading_patterns::get_tool_registry,
    api_contracts::{ToolApiContract, get_api_contract, ToolLifecycleEvent},
};
use anyhow::Result;
use std::sync::Arc;
use std::time::{Instant, Duration};
use tokio::sync::RwLock;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::DatabaseAppState;

/// UI State Manager for coordinating tool interactions in the UI
pub struct UIStateManager {
    /// Application manager that coordinates all tools
    app_manager: Option<RealApplicationManager>,
    /// Current project state
    current_project: Option<ProjectUIState>,
    /// UI interaction history
    interaction_history: Vec<UIInteraction>,
    /// Tool visibility states
    tool_visibility: ToolVisibilityState,
    /// Performance monitoring
    ui_performance: UIPerformanceMetrics,
}

/// Project state as seen by the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUIState {
    /// Project ID
    pub project_id: String,
    /// Project name
    pub project_name: String,
    /// Last active timestamp
    pub last_active: Instant,
    /// Active tool tabs
    pub active_tabs: Vec<String>,
    /// Current selection state
    pub selection_state: SelectionState,
    /// UI preferences
    pub preferences: UIPreferences,
}

/// User interface interaction tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIInteraction {
    /// Interaction type
    pub interaction_type: InteractionType,
    /// Timestamp
    pub timestamp: Instant,
    /// Source tool
    pub source_tool: String,
    /// Target element
    pub target_element: String,
    /// Duration if applicable
    pub duration: Option<Duration>,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
}

/// Types of UI interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    /// Tool initialization
    ToolInitialization,
    /// Data loading
    DataLoading,
    /// Data saving
    DataSaving,
    /// User selection
    UserSelection,
    /// Cross-tool operation
    CrossToolOperation,
    /// Search operation
    SearchOperation,
    /// Navigation
    Navigation,
    /// Configuration change
    ConfigurationChange,
}

/// Tool visibility and layout state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolVisibilityState {
    /// Which tools are visible
    pub visible_tools: Vec<String>,
    /// Tool layout configuration
    pub layout_config: LayoutConfiguration,
    /// Active tool tab
    pub active_tool: Option<String>,
    /// Tool minimization state
    pub minimized_tools: Vec<String>,
}

/// UI layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfiguration {
    /// Window arrangement mode
    pub arrangement: LayoutArrangement,
    /// Panel sizes
    pub panel_sizes: std::collections::HashMap<String, f32>,
    /// Tool positions
    pub tool_positions: std::collections::HashMap<String, (f32, f32)>,
}

/// Layout arrangement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutArrangement {
    /// Single tool full screen
    SingleTool,
    /// Split view with multiple tools
    SplitView,
    /// Tabbed interface
    Tabbed,
    /// Floating panels
    Floating,
}

/// Current selection state in the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionState {
    /// Selected codex entry ID
    pub selected_codex_id: Option<String>,
    /// Selected hierarchy item ID
    pub selected_hierarchy_id: Option<String>,
    /// Selected analysis ID
    pub selected_analysis_id: Option<String>,
    /// Selection context
    pub selection_context: SelectionContext,
}

/// Context for current selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionContext {
    /// Source tool of selection
    pub source_tool: String,
    /// Selection mode
    pub selection_mode: SelectionMode,
    /// Related items
    pub related_items: Vec<String>,
}

/// Selection modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionMode {
    /// Single item selection
    Single,
    /// Multiple item selection
    Multiple,
    /// Range selection
    Range,
    /// Linked selection (cross-tool)
    Linked,
}

/// UI preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPreferences {
    /// Theme settings
    pub theme: ThemeSettings,
    /// Font settings
    pub fonts: FontSettings,
    /// Behavior settings
    pub behavior: BehaviorSettings,
    /// Performance settings
    pub performance: PerformanceSettings,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    /// Color scheme
    pub color_scheme: String,
    /// Font theme
    pub font_theme: String,
    /// Contrast level
    pub contrast: f32,
}

/// Font configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSettings {
    /// Base font size
    pub base_size: f32,
    /// Font family
    pub family: String,
    /// Line height
    pub line_height: f32,
}

/// UI behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSettings {
    /// Auto-save enabled
    pub auto_save: bool,
    /// Auto-refresh enabled
    pub auto_refresh: bool,
    /// Drag and drop enabled
    pub drag_drop_enabled: bool,
    /// Animation enabled
    pub animations_enabled: bool,
}

/// Performance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Refresh interval
    pub refresh_interval_ms: u64,
    /// Cache size limit
    pub max_cache_size: usize,
    /// Async loading enabled
    pub async_loading: bool,
    /// Virtual scrolling enabled
    pub virtual_scrolling: bool,
}

/// UI performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPerformanceMetrics {
    /// Average response time
    pub avg_response_time_ms: f64,
    /// UI frame rate
    pub frame_rate: f32,
    /// Memory usage
    pub memory_usage_mb: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Tool load times
    pub tool_load_times: std::collections::HashMap<String, Duration>,
}

impl Default for UIStateManager {
    fn default() -> Self {
        Self {
            app_manager: None,
            current_project: None,
            interaction_history: Vec::new(),
            tool_visibility: ToolVisibilityState {
                visible_tools: vec!["codex_tool".to_string(), "hierarchy_tool".to_string(), "analysis_tool".to_string()],
                layout_config: LayoutConfiguration {
                    arrangement: LayoutArrangement::SplitView,
                    panel_sizes: std::collections::HashMap::new(),
                    tool_positions: std::collections::HashMap::new(),
                },
                active_tool: Some("codex_tool".to_string()),
                minimized_tools: Vec::new(),
            },
            ui_performance: UIPerformanceMetrics {
                avg_response_time_ms: 0.0,
                frame_rate: 60.0,
                memory_usage_mb: 0.0,
                cache_hit_rate: 0.0,
                tool_load_times: std::collections::HashMap::new(),
            },
        }
    }
}

impl UIStateManager {
    /// Create a new UI state manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize the complete UI system with real tools
    pub async fn initialize_ui_system(&mut self, database_path: &str) -> Result<()> {
        println!("🎨 Initializing UI System with Real Tools...");
        
        // Initialize application manager
        let mut app_manager = RealApplicationManager::new();
        app_manager.initialize_application(database_path).await?;
        
        self.app_manager = Some(app_manager);
        
        // Initialize UI state
        self.ui_performance = UIPerformanceMetrics {
            avg_response_time_ms: 0.0,
            frame_rate: 60.0,
            memory_usage_mb: 50.0, // Base memory usage
            cache_hit_rate: 0.0,
            tool_load_times: std::collections::HashMap::new(),
        };
        
        println!("✅ UI System initialized successfully");
        Ok(())
    }

    /// Create a new project through the UI
    pub async fn create_project_ui(&mut self, project_name: &str) -> Result<String> {
        let start_time = Instant::now();
        
        let project_id = if let Some(app_manager) = &mut self.app_manager {
            app_manager.create_project(project_name).await?
        } else {
            return Err(anyhow::anyhow!("UI system not initialized"));
        };
        
        // Update UI state
        self.current_project = Some(ProjectUIState {
            project_id: project_id.clone(),
            project_name: project_name.to_string(),
            last_active: Instant::now(),
            active_tabs: vec!["codex".to_string(), "hierarchy".to_string(), "analysis".to_string()],
            selection_state: SelectionState {
                selected_codex_id: None,
                selected_hierarchy_id: None,
                selected_analysis_id: None,
                selection_context: SelectionContext {
                    source_tool: "project_manager".to_string(),
                    selection_mode: SelectionMode::Single,
                    related_items: Vec::new(),
                },
            },
            preferences: UIPreferences {
                theme: ThemeSettings {
                    color_scheme: "default".to_string(),
                    font_theme: "system".to_string(),
                    contrast: 1.0,
                },
                fonts: FontSettings {
                    base_size: 14.0,
                    family: "System".to_string(),
                    line_height: 1.4,
                },
                behavior: BehaviorSettings {
                    auto_save: true,
                    auto_refresh: true,
                    drag_drop_enabled: true,
                    animations_enabled: true,
                },
                performance: PerformanceSettings {
                    refresh_interval_ms: 5000,
                    max_cache_size: 1000,
                    async_loading: true,
                    virtual_scrolling: true,
                },
            },
        });
        
        // Record interaction
        self.record_interaction(InteractionType::DataSaving, "project_manager", &format!("Create project: {}", project_name), true, None);
        
        let duration = start_time.elapsed();
        println!("✅ Project created in UI: {} (ID: {}) in {:.2}ms", project_name, project_id, duration.as_millis());
        
        Ok(project_id)
    }

    /// Load a project in the UI
    pub async fn load_project_ui(&mut self, project_id: &str) -> Result<()> {
        let start_time = Instant::now();
        
        if let Some(app_manager) = &self.app_manager {
            let project_data = app_manager.load_project(project_id).await?;
            
            // Update selection state based on loaded data
            let mut selection_state = self.current_project.as_ref().map(|p| p.selection_state.clone())
                .unwrap_or_else(|| SelectionState {
                    selected_codex_id: None,
                    selected_hierarchy_id: None,
                    selected_analysis_id: None,
                    selection_context: SelectionContext {
                        source_tool: "project_loader".to_string(),
                        selection_mode: SelectionMode::Single,
                        related_items: Vec::new(),
                    },
                });
            
            // Auto-select first items if available
            if selection_state.selected_codex_id.is_none() && !project_data.codex_entries.is_empty() {
                selection_state.selected_codex_id = Some(project_data.codex_entries[0].id.to_string());
            }
            
            if selection_state.selected_hierarchy_id.is_none() && !project_data.hierarchy_items.is_empty() {
                selection_state.selected_hierarchy_id = Some(project_data.hierarchy_items[0].id.clone());
            }
            
            if selection_state.selected_analysis_id.is_none() && !project_data.analyses.is_empty() {
                selection_state.selected_analysis_id = Some(project_data.analyses[0].id.clone());
            }
            
            // Update current project state
            self.current_project = Some(ProjectUIState {
                project_id: project_id.to_string(),
                project_name: project_data.codex_entries.iter()
                    .find(|entry| entry.entry_type == crate::database::models::codex::CodexEntryType::StorySummary)
                    .map(|entry| entry.title.clone())
                    .unwrap_or_else(|| format!("Project {}", project_id)),
                last_active: Instant::now(),
                active_tabs: vec!["codex".to_string(), "hierarchy".to_string(), "analysis".to_string()],
                selection_state,
                preferences: self.current_project.as_ref().map(|p| p.preferences.clone())
                    .unwrap_or_default(),
            });
            
            // Record interaction
            self.record_interaction(InteractionType::DataLoading, "project_loader", &format!("Load project: {}", project_id), true, None);
            
            let duration = start_time.elapsed();
            println!("✅ Project loaded in UI: {} in {:.2}ms", project_id, duration.as_millis());
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("UI system not initialized"))
        }
    }

    /// Perform cross-tool operation through UI
    pub async fn create_character_and_scene_ui(
        &mut self,
        character_name: &str,
        scene_name: &str,
    ) -> Result<()> {
        let start_time = Instant::now();
        
        if let Some(app_manager) = &mut self.app_manager {
            if let Some(current_project) = &self.current_project {
                app_manager.create_character_and_scene(
                    character_name,
                    scene_name,
                    &current_project.project_id,
                ).await?;
                
                // Update selection state to reflect new items
                if let Some(project_state) = &mut self.current_project {
                    // In a real UI, these would be the actual IDs of the created items
                    project_state.selection_state.selected_codex_id = Some(format!("character_{}", character_name.replace(" ", "_")));
                    project_state.selection_state.selected_hierarchy_id = Some(format!("scene_{}", scene_name.replace(" ", "_")));
                    project_state.selection_state.selection_context.related_items = vec![
                        format!("character_{}", character_name.replace(" ", "_")),
                        format!("scene_{}", scene_name.replace(" ", "_")),
                    ];
                }
                
                // Record interaction
                self.record_interaction(
                    InteractionType::CrossToolOperation,
                    "ui_manager",
                    &format!("Create character '{}' and scene '{}'", character_name, scene_name),
                    true,
                    None,
                );
                
                let duration = start_time.elapsed();
                println!("✅ Cross-tool operation completed in {:.2}ms", duration.as_millis());
                
                Ok(())
            } else {
                Err(anyhow::anyhow!("No active project"))
            }
        } else {
            Err(anyhow::anyhow!("UI system not initialized"))
        }
    }

    /// Handle tool tab selection in UI
    pub fn select_tool_tab(&mut self, tool_name: &str) -> Result<()> {
        self.tool_visibility.active_tool = Some(tool_name.to_string());
        
        self.record_interaction(
            InteractionType::Navigation,
            "tab_manager",
            &format!("Select tool tab: {}", tool_name),
            true,
            None,
        );
        
        Ok(())
    }

    /// Handle item selection across tools
    pub fn select_item_in_tool(&mut self, tool_name: &str, item_id: &str) -> Result<()> {
        if let Some(project_state) = &mut self.current_project {
            match tool_name {
                "codex" => {
                    project_state.selection_state.selected_codex_id = Some(item_id.to_string());
                    project_state.selection_state.selection_context.source_tool = "codex_tool".to_string();
                }
                "hierarchy" => {
                    project_state.selection_state.selected_hierarchy_id = Some(item_id.to_string());
                    project_state.selection_state.selection_context.source_tool = "hierarchy_tool".to_string();
                }
                "analysis" => {
                    project_state.selection_state.selected_analysis_id = Some(item_id.to_string());
                    project_state.selection_state.selection_context.source_tool = "analysis_tool".to_string();
                }
                _ => return Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
            }
            
            // Record interaction
            self.record_interaction(
                InteractionType::UserSelection,
                tool_name,
                &format!("Select item: {}", item_id),
                true,
                None,
            );
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No active project"))
        }
    }

    /// Search across all tools
    pub async fn search_across_tools(&self, search_term: &str) -> Result<SearchResults> {
        if let Some(app_manager) = &self.app_manager {
            let mut results = SearchResults::new();
            
            // Search in codex entries
            if let Some(project_id) = &self.current_project.as_ref().map(|p| p.project_id.clone()) {
                if let Ok(entries) = app_manager.codex_tool.as_ref().unwrap()
                    .search_entries(project_id.to_string(), search_term.to_string()).await
                {
                    results.codex_results = entries;
                }
            }
            
            // Record interaction
            self.record_interaction(
                InteractionType::SearchOperation,
                "search_manager",
                &format!("Search term: '{}'", search_term),
                true,
                None,
            );
            
            Ok(results)
        } else {
            Err(anyhow::anyhow!("UI system not initialized"))
        }
    }

    /// Update UI performance metrics
    pub fn update_performance_metrics(&mut self, tool_name: &str, load_time: Duration) {
        self.ui_performance.tool_load_times.insert(tool_name.to_string(), load_time);
        
        // Calculate average response time
        let total_time: Duration = self.ui_performance.tool_load_times.values().sum();
        let count = self.ui_performance.tool_load_times.len();
        self.ui_performance.avg_response_time_ms = total_time.as_millis_f64() / count as f64;
        
        // Update memory usage (simulate)
        self.ui_performance.memory_usage_mb = 50.0 + (count as f64 * 10.0);
    }

    /// Record UI interaction for analytics
    fn record_interaction(
        &mut self,
        interaction_type: InteractionType,
        source_tool: &str,
        target_element: &str,
        success: bool,
        error_message: Option<String>,
    ) {
        let interaction = UIInteraction {
            interaction_type,
            timestamp: Instant::now(),
            source_tool: source_tool.to_string(),
            target_element: target_element.to_string(),
            duration: None,
            success,
            error_message,
        };
        
        self.interaction_history.push(interaction);
        
        // Keep only last 1000 interactions
        if self.interaction_history.len() > 1000 {
            self.interaction_history.remove(0);
        }
    }

    /// Get UI analytics summary
    pub fn get_ui_analytics(&self) -> UIAnalytics {
        let total_interactions = self.interaction_history.len();
        let successful_interactions = self.interaction_history.iter().filter(|i| i.success).count();
        let success_rate = if total_interactions > 0 {
            (successful_interactions as f64 / total_interactions as f64) * 100.0
        } else {
            0.0
        };
        
        // Count interaction types
        let mut interaction_counts = std::collections::HashMap::new();
        for interaction in &self.interaction_history {
            *interaction_counts.entry(format!("{:?}", interaction.interaction_type)).or_insert(0) += 1;
        }
        
        UIAnalytics {
            total_interactions,
            success_rate,
            interaction_counts,
            avg_response_time_ms: self.ui_performance.avg_response_time_ms,
            memory_usage_mb: self.ui_performance.memory_usage_mb,
            current_project_name: self.current_project.as_ref().map(|p| p.project_name.clone()),
            active_tools: self.tool_visibility.visible_tools.clone(),
        }
    }

    /// Cleanup and shutdown UI system
    pub async fn shutdown_ui(&mut self) -> Result<()> {
        println!("🛑 Shutting down UI system...");
        
        // Shutdown application manager
        if let Some(mut app_manager) = self.app_manager.take() {
            app_manager.shutdown().await?;
        }
        
        // Record shutdown interaction
        self.record_interaction(
            InteractionType::ConfigurationChange,
            "ui_manager",
            "Shutdown UI system",
            true,
            None,
        );
        
        println!("✅ UI system shutdown complete");
        Ok(())
    }
}

/// Search results across all tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    /// Codex search results
    pub codex_results: Vec<crate::database::models::codex::CodexEntry>,
    /// Hierarchy search results
    pub hierarchy_results: Vec<crate::ui::tools::hierarchy_base::HierarchyItem>,
    /// Analysis search results
    pub analysis_results: Vec<crate::ui::tools::analysis_tool_migrated::AnalysisRecord>,
    /// Search timestamp
    pub search_time: Instant,
}

impl SearchResults {
    pub fn new() -> Self {
        Self {
            codex_results: Vec::new(),
            hierarchy_results: Vec::new(),
            analysis_results: Vec::new(),
            search_time: Instant::now(),
        }
    }
}

/// UI Analytics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAnalytics {
    /// Total number of interactions
    pub total_interactions: usize,
    /// Success rate percentage
    pub success_rate: f64,
    /// Count of each interaction type
    pub interaction_counts: std::collections::HashMap<String, usize>,
    /// Average response time
    pub avg_response_time_ms: f64,
    /// Current memory usage
    pub memory_usage_mb: f64,
    /// Current project name
    pub current_project_name: Option<String>,
    /// Active tools
    pub active_tools: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_ui_state_manager_creation() {
        let ui_manager = UIStateManager::new();
        assert!(ui_manager.app_manager.is_none());
        assert!(ui_manager.current_project.is_none());
        assert_eq!(ui_manager.interaction_history.len(), 0);
        assert_eq!(ui_manager.tool_visibility.visible_tools.len(), 3);
    }

    #[tokio::test]
    async fn test_ui_system_initialization() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        
        let mut ui_manager = UIStateManager::new();
        assert!(ui_manager.initialize_ui_system(db_path).await.is_ok());
        
        assert!(ui_manager.app_manager.is_some());
        assert_eq!(ui_manager.ui_performance.memory_usage_mb, 50.0);
        assert_eq!(ui_manager.ui_performance.frame_rate, 60.0);
    }

    #[tokio::test]
    async fn test_project_creation_ui() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        
        let mut ui_manager = UIStateManager::new();
        ui_manager.initialize_ui_system(db_path).await.unwrap();
        
        let project_id = ui_manager.create_project_ui("Test Project").await.unwrap();
        assert!(!project_id.is_empty());
        
        assert!(ui_manager.current_project.is_some());
        let project = ui_manager.current_project.as_ref().unwrap();
        assert_eq!(project.project_name, "Test Project");
        assert_eq!(project.project_id, project_id);
        assert_eq!(project.active_tabs.len(), 3);
    }

    #[tokio::test]
    async fn test_ui_analytics() {
        let mut ui_manager = UIStateManager::new();
        
        // Record some interactions
        ui_manager.record_interaction(
            InteractionType::ToolInitialization,
            "test_tool",
            "Initialize tool",
            true,
            None,
        );
        
        ui_manager.record_interaction(
            InteractionType::DataLoading,
            "test_tool",
            "Load data",
            false,
            Some("Connection failed".to_string()),
        );
        
        let analytics = ui_manager.get_ui_analytics();
        assert_eq!(analytics.total_interactions, 2);
        assert_eq!(analytics.success_rate, 50.0);
        assert!(analytics.interaction_counts.contains_key("ToolInitialization"));
        assert!(analytics.interaction_counts.contains_key("DataLoading"));
    }
}

/// Example of complete UI workflow
pub async fn example_complete_ui_workflow() -> Result<()> {
    println!("🎨 Complete UI Workflow Example");
    println!("===============================");
    
    // Create UI state manager
    let mut ui_manager = UIStateManager::new();
    ui_manager.initialize_ui_system("ui_example.db").await?;
    
    // Create a project
    let project_id = ui_manager.create_project_ui("My Novel Project").await?;
    
    // Load the project
    ui_manager.load_project_ui(&project_id).await?;
    
    // Perform cross-tool operations
    ui_manager.create_character_and_scene_ui("Alice Johnson", "The Mysterious Forest").await?;
    
    // Simulate tool interactions
    ui_manager.select_tool_tab("codex")?;
    ui_manager.select_item_in_tool("codex", "character_alice_johnson")?;
    
    ui_manager.select_tool_tab("hierarchy")?;
    ui_manager.select_item_in_tool("hierarchy", "scene_the_mysterious_forest")?;
    
    // Search across tools
    let search_results = ui_manager.search_across_tools("Alice").await?;
    println!("📊 Found {} codex results for 'Alice'", search_results.codex_results.len());
    
    // Update performance metrics
    ui_manager.update_performance_metrics("codex_tool", Duration::from_millis(150));
    ui_manager.update_performance_metrics("hierarchy_tool", Duration::from_millis(120));
    
    // Get analytics
    let analytics = ui_manager.get_ui_analytics();
    println!("📈 UI Analytics:");
    println!("  - Total interactions: {}", analytics.total_interactions);
    println!("  - Success rate: {:.1}%", analytics.success_rate);
    println!("  - Average response time: {:.2}ms", analytics.avg_response_time_ms);
    println!("  - Memory usage: {:.1}MB", analytics.memory_usage_mb);
    
    // Shutdown
    ui_manager.shutdown_ui().await?;
    
    println!("✅ Complete UI workflow example finished!");
    Ok(())
}