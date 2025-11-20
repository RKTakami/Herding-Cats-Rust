//! Migrated Structure Tool Implementation
//!
//! This module provides the migrated version of the structure tool using the new
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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::structure_data::{StructureData, PlotStage, PlotType, CharacterArc, WorldStructure};

/// Migrated structure tool that implements the new architecture patterns
pub struct MigratedStructureTool {
    /// Database context for safe database operations
    database_context: Option<ToolDatabaseContext>,
    /// API contract for cross-tool communication
    api_contract: Arc<ToolApiContract>,
    /// Structure data and configuration
    structure_data: StructureData,
    /// Tool registry reference for lifecycle management
    tool_registry: &'static crate::ui::tools::threading_patterns::ThreadSafeToolRegistry,
    /// Tool initialization timestamp
    initialized_at: Option<Instant>,
    /// Last operation duration tracking
    last_operation_duration: Option<Duration>,
    /// Structure analysis and planning metrics
    planning_metrics: PlanningMetrics,
    /// Cached structure templates
    structure_templates: HashMap<String, StructureTemplate>,
}

/// Structure template for common story patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Plot type this template applies to
    pub plot_type: PlotType,
    /// Template stages
    pub stages: Vec<TemplateStage>,
    /// Template complexity level
    pub complexity: TemplateComplexity,
    /// Estimated completion time
    pub estimated_time: Option<Duration>,
}

/// Template stage definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStage {
    /// Stage name
    pub name: String,
    /// Stage description
    pub description: String,
    /// Stage order
    pub order: u32,
    /// Suggested word count range
    pub word_count_range: (u32, u32),
    /// Key elements to include
    pub key_elements: Vec<String>,
    /// Common pitfalls to avoid
    pub pitfalls: Vec<String>,
}

/// Template complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateComplexity {
    /// Simple, straightforward structure
    Simple,
    /// Moderate complexity with subplots
    Moderate,
    /// Complex with multiple threads
    Complex,
    /// Epic scale with extensive worldbuilding
    Epic,
}

/// Planning metrics and statistics
#[derive(Debug, Clone)]
pub struct PlanningMetrics {
    /// Total planning sessions
    pub total_sessions: u64,
    /// Average planning time
    pub avg_planning_time_ms: f64,
    /// Structure completion rate
    pub completion_rate: f64,
    /// Template usage statistics
    pub template_usage: HashMap<String, u64>,
    /// Last planning session
    pub last_planning_session: Option<Instant>,
    /// Structure coherence score
    pub coherence_score: f64,
}

impl Default for PlanningMetrics {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            avg_planning_time_ms: 0.0,
            completion_rate: 0.0,
            template_usage: HashMap::new(),
            last_planning_session: None,
            coherence_score: 0.0,
        }
    }
}

impl MigratedStructureTool {
    /// Create a new migrated structure tool
    pub fn new() -> Self {
        Self {
            database_context: None,
            api_contract: get_api_contract().clone(),
            structure_data: StructureData::new(),
            tool_registry: get_tool_registry(),
            initialized_at: None,
            last_operation_duration: None,
            planning_metrics: PlanningMetrics::default(),
            structure_templates: HashMap::new(),
        }
    }

    /// Load structure data from database with retry logic
    pub async fn load_structure_data(&mut self, project_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let result = context.execute_with_retry(
                "load_structure_data",
                |service| Box::pin(async move {
                    let structure_data = service.get_structure_data_by_project(project_id).await?;
                    Ok::<StructureData, String>(structure_data)
                }),
                3,
            ).await;

            match result {
                Ok(data) => {
                    self.structure_data = data;
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_planning_metrics(duration);
                    
                    // Broadcast successful load
                    self.broadcast_structure_event("data_loaded").await;
                    
                    DatabaseOperationResult::success((), duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("load_structure", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Create a new plot structure
    pub async fn create_plot_structure(
        &mut self,
        project_id: String,
        plot_type: PlotType,
        title: String,
        description: String,
    ) -> DatabaseOperationResult<String> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            // Generate structure ID
            let structure_id = format!("structure_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis());

            let result = context.execute_with_retry(
                "create_plot_structure",
                |service| Box::pin(async move {
                    service.create_plot_structure(&structure_id, &project_id, plot_type, &title, &description).await?;
                    Ok::<String, String>(structure_id.clone())
                }),
                3,
            ).await;

            match result {
                Ok(created_id) => {
                    // Update local structure data
                    self.structure_data.plots.push(created_id.clone());
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_planning_metrics(duration);
                    
                    DatabaseOperationResult::success(created_id, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("create_structure", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Add plot stage to structure
    pub async fn add_plot_stage(
        &mut self,
        structure_id: &str,
        stage_name: String,
        stage_description: String,
        order: u32,
        word_target: Option<u32>,
    ) -> DatabaseOperationResult<String> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let stage_id = format!("stage_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis());

            let result = context.execute_with_retry(
                "add_plot_stage",
                |service| Box::pin(async move {
                    service.add_plot_stage(&stage_id, structure_id, &stage_name, &stage_description, order, word_target).await?;
                    Ok::<String, String>(stage_id)
                }),
                3,
            ).await;

            match result {
                Ok(stage_id) => {
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_planning_metrics(duration);
                    
                    DatabaseOperationResult::success(stage_id, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("add_stage", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Create character arc
    pub async fn create_character_arc(
        &mut self,
        character_name: String,
        arc_type: String,
        stages: Vec<CharacterArcStage>,
    ) -> DatabaseOperationResult<String> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let arc_id = format!("arc_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis());

            let result = context.execute_with_retry(
                "create_character_arc",
                |service| Box::pin(async move {
                    service.create_character_arc(&arc_id, &character_name, &arc_type, &stages).await?;
                    Ok::<String, String>(arc_id)
                }),
                3,
            ).await;

            match result {
                Ok(arc_id) => {
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_planning_metrics(duration);
                    
                    DatabaseOperationResult::success(arc_id, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("create_arc", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Apply structure template
    pub async fn apply_structure_template(
        &mut self,
        structure_id: &str,
        template_name: &str,
    ) -> DatabaseOperationResult<Vec<String>> {
        let start_time = Instant::now();
        
        // Get template
        let template = match self.structure_templates.get(template_name) {
            Some(template) => template,
            None => {
                return DatabaseOperationResult::not_found("Template", template_name.to_string());
            }
        };

        if let Some(context) = &mut self.database_context {
            let mut created_stages = Vec::new();

            for stage_template in &template.stages {
                let stage_id = format!("stage_{}_{}", structure_id, stage_template.order);
                
                let result = context.execute_with_retry(
                    "apply_template_stage",
                    |service| Box::pin(async move {
                        service.add_plot_stage(&stage_id, structure_id, &stage_template.name, &stage_template.description, stage_template.order, Some(stage_template.word_count_range.1)).await?;
                        Ok::<String, String>(stage_id.clone())
                    }),
                    3,
                ).await;

                match result {
                    Ok(stage_id) => {
                        created_stages.push(stage_id);
                    }
                    Err(_) => {
                        return DatabaseOperationResult::validation_error("apply_template", format!("Failed to create stage: {}", stage_template.name));
                    }
                }
            }

            // Track template usage
            *self.planning_metrics.template_usage.entry(template_name.to_string()).or_insert(0) += 1;

            let duration = start_time.elapsed();
            self.last_operation_duration = Some(duration);
            self.update_planning_metrics(duration);
            
            DatabaseOperationResult::success(created_stages, duration)
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Analyze structure coherence
    pub async fn analyze_structure_coherence(&self, structure_id: &str) -> DatabaseOperationResult<StructureAnalysis> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "analyze_structure_coherence",
                |service| Box::pin(async move {
                    let analysis = service.analyze_structure(structure_id).await?;
                    Ok::<StructureAnalysis, String>(analysis)
                }),
                3,
            ).await;

            match result {
                Ok(analysis) => {
                    DatabaseOperationResult::success(analysis, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("analyze_structure", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Generate structure recommendations
    pub async fn generate_structure_recommendations(&self, project_id: &str) -> DatabaseOperationResult<Vec<StructureRecommendation>> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "generate_structure_recommendations",
                |service| Box::pin(async move {
                    let recommendations = service.generate_recommendations(project_id).await?;
                    Ok::<Vec<StructureRecommendation>, String>(recommendations)
                }),
                3,
            ).await;

            match result {
                Ok(recommendations) => {
                    DatabaseOperationResult::success(recommendations, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("generate_recommendations", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get structure statistics
    pub fn get_structure_stats(&self) -> StructureStats {
        let total_structures = self.structure_data.plots.len();
        let total_stages = self.structure_data.plot_stages.len();
        let total_arcs = self.structure_data.character_arcs.len();
        
        StructureStats {
            total_structures,
            total_stages,
            total_arcs,
            avg_stages_per_structure: if total_structures > 0 {
                total_stages as f64 / total_structures as f64
            } else {
                0.0
            },
            most_used_template: self.get_most_used_template(),
            planning_metrics: self.planning_metrics.clone(),
            last_operation_duration: self.last_operation_duration,
        }
    }

    /// Add structure template
    pub fn add_structure_template(&mut self, template: StructureTemplate) {
        self.structure_templates.insert(template.name.clone(), template);
    }

    /// Get available templates
    pub fn get_available_templates(&self) -> Vec<&StructureTemplate> {
        self.structure_templates.values().collect()
    }

    /// Migrate from legacy structure tool
    pub async fn migrate_from_legacy(
        legacy_tool: super::structure_data::StructureTool,
        database_context: ToolDatabaseContext,
    ) -> Result<Self> {
        let mut migrated_tool = Self::new();
        
        // Initialize with database context
        let mut db_context = database_context;
        migrated_tool.initialize(&mut db_context).await?;
        
        // Copy structure data
        migrated_tool.structure_data = legacy_tool.structure_data;
        
        Ok(migrated_tool)
    }

    /// Update planning metrics
    fn update_planning_metrics(&mut self, duration: Duration) {
        self.planning_metrics.total_sessions += 1;
        
        // Update moving average for planning time
        let current_avg = self.planning_metrics.avg_planning_time_ms;
        let operation_ms = duration.as_millis_f64();
        let count = self.planning_metrics.total_sessions as f64;
        self.planning_metrics.avg_planning_time_ms = 
            (current_avg * (count - 1.0) + operation_ms) / count;
        
        self.planning_metrics.last_planning_session = Some(Instant::now());
    }

    /// Get most used template
    fn get_most_used_template(&self) -> Option<String> {
        self.planning_metrics.template_usage
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(name, _)| name.clone())
    }

    /// Broadcast structure-related events
    async fn broadcast_structure_event(&self, event_type: &str) {
        if let Err(e) = self.api_contract.broadcast_lifecycle(ToolLifecycleEvent::CustomEvent {
            tool_id: self.tool_type().display_name().to_string(),
            event_type: event_type.to_string(),
            timestamp: Instant::now(),
            data: Some(format!("Structure event: {}", event_type)),
        }).await {
            warn!("Failed to broadcast structure event: {}", e);
        }
    }
}

/// Character arc stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArcStage {
    /// Stage name
    pub name: String,
    /// Stage description
    pub description: String,
    /// Character state before stage
    pub before_state: String,
    /// Character state after stage
    pub after_state: String,
    /// Key events or decisions
    pub key_events: Vec<String>,
    /// Character growth or change
    pub growth: String,
}

/// Structure analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureAnalysis {
    /// Overall coherence score (0.0 to 1.0)
    pub coherence_score: f64,
    /// Structure completeness percentage
    pub completeness: f64,
    /// Plot hole detection results
    pub plot_holes: Vec<PlotHole>,
    /// Pacing analysis
    pub pacing_analysis: PacingAnalysis,
    /// Character arc integration score
    pub arc_integration_score: f64,
    /// Worldbuilding consistency score
    pub worldbuilding_consistency: f64,
    /// Generated at timestamp
    pub generated_at: Instant,
}

/// Plot hole detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotHole {
    /// Hole description
    pub description: String,
    /// Severity level (1-10)
    pub severity: u8,
    /// Location in structure
    pub location: String,
    /// Suggested fix
    pub suggested_fix: String,
    /// Impact on story
    pub impact: String,
}

/// Pacing analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingAnalysis {
    /// Overall pacing score
    pub pacing_score: f64,
    /// Fast sections
    pub fast_sections: Vec<String>,
    /// Slow sections
    pub slow_sections: Vec<String>,
    /// Pacing recommendations
    pub recommendations: Vec<String>,
    /// Word count distribution analysis
    pub word_count_analysis: WordCountAnalysis,
}

/// Word count analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordCountAnalysis {
    /// Actual vs planned word counts
    pub actual_vs_planned: Vec<(String, u32, u32)>,
    /// Distribution by section
    pub distribution_by_section: HashMap<String, (u32, f64)>,
    /// Pacing indicators
    pub pacing_indicators: Vec<PacingIndicator>,
}

/// Pacing indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingIndicator {
    /// Indicator type
    pub indicator_type: String,
    /// Value
    pub value: f64,
    /// Threshold
    pub threshold: f64,
    /// Status (good/warning/bad)
    pub status: String,
}

/// Structure recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Recommendation description
    pub description: String,
    /// Priority level
    pub priority: u8,
    /// Expected impact
    pub expected_impact: String,
    /// Implementation complexity
    pub complexity: String,
    /// Related elements
    pub related_elements: Vec<String>,
}

/// Types of structure recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    /// Add missing plot element
    AddPlotElement,
    /// Remove redundant element
    RemoveRedundant,
    /// Improve character development
    ImproveCharacter,
    /// Enhance worldbuilding
    EnhanceWorldbuilding,
    /// Fix pacing issues
    FixPacing,
    /// Strengthen themes
    StrengthenThemes,
}

/// Structure statistics summary
#[derive(Debug, Clone)]
pub struct StructureStats {
    /// Total number of structures
    pub total_structures: usize,
    /// Total number of plot stages
    pub total_stages: usize,
    /// Total number of character arcs
    pub total_arcs: usize,
    /// Average stages per structure
    pub avg_stages_per_structure: f64,
    /// Most used template name
    pub most_used_template: Option<String>,
    /// Planning metrics
    pub planning_metrics: PlanningMetrics,
    /// Last operation duration
    pub last_operation_duration: Option<Duration>,
}

#[async_trait]
impl ToolIntegration for MigratedStructureTool {
    /// Initialize the structure tool with database context
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        // Store database context
        self.database_context = Some(database_context.clone());
        
        // Register with tool registry
        let tool_id = format!("{}_{}", self.tool_type().display_name(), uuid::Uuid::new_v4());
        self.tool_registry.register_tool(tool_id.clone(), Arc::new(()) as Arc<dyn Send + Sync + 'static>).await?;
        
        // Initialize API contract
        self.api_contract = get_api_contract().clone();
        
        // Load default templates
        self.load_default_templates();
        
        // Mark as initialized
        self.initialized_at = Some(Instant::now());
        
        // Broadcast initialization event
        self.broadcast_structure_event("initialized").await;
        
        info!("Structure tool initialized successfully");
        Ok(())
    }

    /// Update tool state
    fn update(&mut self) -> Result<(), String> {
        // Perform any periodic updates
        // For structure tool, this might include:
        // - Analyzing structure completeness
        // - Generating recommendations
        // - Validating template consistency
        
        // Validate structure data integrity
        self.validate_structure_data_integrity()?;
        
        Ok(())
    }

    /// Render the tool UI
    fn render(&mut self) -> Result<(), String> {
        // This would typically render the structure UI
        // For now, we'll just validate that the tool is in a renderable state
        
        if self.database_context.is_none() {
            return Err("Database context not initialized".to_string());
        }
        
        Ok(())
    }

    /// Cleanup tool resources
    async fn cleanup(&mut self) -> Result<(), String> {
        // Broadcast cleanup event
        self.broadcast_structure_event("cleanup_started").await;
        
        // Clear cached templates
        self.structure_templates.clear();
        
        // Unregister from tool registry
        let tool_id = format!("{}_{}", self.tool_type().display_name(), uuid::Uuid::new_v4());
        self.tool_registry.unregister_tool(&tool_id).await?;
        
        // Clear database context
        self.database_context = None;
        
        info!("Structure tool cleanup completed");
        Ok(())
    }
}

impl MigratedStructureTool {
    /// Load default structure templates
    fn load_default_templates(&mut self) {
        // Three-Act Structure
        let three_act = StructureTemplate {
            name: "Three-Act Structure".to_string(),
            description: "Classic three-act narrative structure".to_string(),
            plot_type: PlotType::Traditional,
            stages: vec![
                TemplateStage {
                    name: "Act 1: Setup".to_string(),
                    description: "Introduce characters, setting, and inciting incident".to_string(),
                    order: 1,
                    word_count_range: (5000, 10000),
                    key_elements: vec![
                        "Protagonist introduction".to_string(),
                        "Inciting incident".to_string(),
                        "Stake establishment".to_string(),
                    ],
                    pitfalls: vec![
                        "Info dumping".to_string(),
                        "Weak inciting incident".to_string(),
                    ],
                },
                TemplateStage {
                    name: "Act 2: Confrontation".to_string(),
                    description: "Rising action and obstacles".to_string(),
                    order: 2,
                    word_count_range: (20000, 30000),
                    key_elements: vec![
                        "Rising conflict".to_string(),
                        "Character development".to_string(),
                        "Midpoint reversal".to_string(),
                    ],
                    pitfalls: vec![
                        "Sagging middle".to_string(),
                        "Passive protagonist".to_string(),
                    ],
                },
                TemplateStage {
                    name: "Act 3: Resolution".to_string(),
                    description: "Climax and resolution".to_string(),
                    order: 3,
                    word_count_range: (10000, 15000),
                    key_elements: vec![
                        "Final confrontation".to_string(),
                        "Climax".to_string(),
                        "Resolution".to_string(),
                    ],
                    pitfalls: vec![
                        "Rushed ending".to_string(),
                        "Deus ex machina".to_string(),
                    ],
                },
            ],
            complexity: TemplateComplexity::Simple,
            estimated_time: Some(Duration::from_days(30)),
        };

        self.structure_templates.insert(three_act.name.clone(), three_act);

        // Hero's Journey
        let heroes_journey = StructureTemplate {
            name: "Hero's Journey".to_string(),
            description: "Joseph Campbell's monomyth structure".to_string(),
            plot_type: PlotType::Epic,
            stages: vec![
                TemplateStage {
                    name: "Ordinary World".to_string(),
                    description: "Hero's normal life before adventure".to_string(),
                    order: 1,
                    word_count_range: (3000, 7000),
                    key_elements: vec![
                        "Hero introduction".to_string(),
                        "Establishing normalcy".to_string(),
                        "Foreshadowing".to_string(),
                    ],
                    pitfalls: vec![
                        "Boring setup".to_string(),
                        "Over-explanation".to_string(),
                    ],
                },
                TemplateStage {
                    name: "Call to Adventure".to_string(),
                    description: "Inciting incident that starts the journey".to_string(),
                    order: 2,
                    word_count_range: (1000, 3000),
                    key_elements: vec![
                        "Call to adventure".to_string(),
                        "Refusal of call".to_string(),
                        "Meeting mentor".to_string(),
                    ],
                    pitfalls: vec![
                        "Weak call".to_string(),
                        "Predictable refusal".to_string(),
                    ],
                },
                TemplateStage {
                    name: "Tests, Allies, Enemies".to_string(),
                    description: "Trials and companions on the journey".to_string(),
                    order: 3,
                    word_count_range: (15000, 25000),
                    key_elements: vec![
                        "Trials and challenges".to_string(),
                        "Allies and enemies".to_string(),
                        "Approach to inmost cave".to_string(),
                    ],
                    pitfalls: vec![
                        "Too many subplots".to_string(),
                        "Underdeveloped allies".to_string(),
                    ],
                },
            ],
            complexity: TemplateComplexity::Moderate,
            estimated_time: Some(Duration::from_days(60)),
        };

        self.structure_templates.insert(heroes_journey.name.clone(), heroes_journey);
    }

    /// Validate structure data integrity
    fn validate_structure_data_integrity(&self) -> Result<(), String> {
        // Check for duplicate structure IDs
        let mut structure_ids = std::collections::HashSet::new();
        for structure_id in &self.structure_data.plots {
            if !structure_ids.insert(structure_id) {
                return Err(format!("Duplicate structure ID: {}", structure_id));
            }
        }

        // Check for orphaned stages
        let structure_set: std::collections::HashSet<_> = self.structure_data.plots.iter().collect();
        for stage in &self.structure_data.plot_stages {
            if !structure_set.contains(&stage.structure_id) {
                return Err(format!("Orphaned stage: {} belongs to non-existent structure", stage.id));
            }
        }

        Ok(())
    }
}

impl Default for MigratedStructureTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolType for MigratedStructureTool {
    fn display_name(&self) -> &'static str {
        "Structure Tool"
    }
    
    fn tool_id(&self) -> &'static str {
        "structure_tool"
    }
    
    fn version(&self) -> &'static str {
        "2.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Narrative structure planning and analysis tool with database integration"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseAppState;
    use tokio::sync::RwLock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_structure_tool_initialization() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedStructureTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_structure", database_state).await;
        let result = tool.initialize(&mut database_context).await;
        
        assert!(result.is_ok());
        assert!(tool.database_context.is_some());
        assert!(tool.initialized_at.is_some());
        assert!(!tool.structure_templates.is_empty());
    }

    #[tokio::test]
    async fn test_structure_creation() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedStructureTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_structure", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        let result = tool.create_plot_structure(
            "test_project".to_string(),
            PlotType::Traditional,
            "Test Structure".to_string(),
            "A test plot structure".to_string(),
        ).await;
        
        assert!(result.is_success());
        assert_eq!(tool.structure_data.plots.len(), 1);
    }

    #[tokio::test]
    async fn test_template_application() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedStructureTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_structure", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create a structure first
        let structure_result = tool.create_plot_structure(
            "test_project".to_string(),
            PlotType::Traditional,
            "Template Test".to_string(),
            "Testing template application".to_string(),
        ).await;
        
        assert!(structure_result.is_success());
        let structure_id = structure_result.data.unwrap();
        
        // Apply template
        let template_result = tool.apply_structure_template(&structure_id, "Three-Act Structure").await;
        assert!(template_result.is_success());
        
        let created_stages = template_result.data.unwrap();
        assert_eq!(created_stages.len(), 3); // Three acts
    }

    #[tokio::test]
    async fn test_structure_stats() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedStructureTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_structure", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Simulate some planning operations
        tool.update_planning_metrics(Duration::from_secs(120));
        tool.update_planning_metrics(Duration::from_secs(90));
        tool.update_planning_metrics(Duration::from_secs(150));
        
        let stats = tool.get_structure_stats();
        assert_eq!(stats.planning_metrics.total_sessions, 3);
        assert!(stats.planning_metrics.avg_planning_time_ms > 0.0);
        assert!(stats.get_most_used_template().is_none()); // No templates used yet
    }

    #[tokio::test]
    async fn test_character_arc_creation() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedStructureTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_structure", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        let stages = vec![
            CharacterArcStage {
                name: "Introduction".to_string(),
                description: "Character is introduced".to_string(),
                before_state: "Normal life".to_string(),
                after_state: "Life disrupted".to_string(),
                key_events: vec!["Inciting incident".to_string()],
                growth: "Awareness".to_string(),
            },
            CharacterArcStage {
                name: "Development".to_string(),
                description: "Character faces challenges".to_string(),
                before_state: "Life disrupted".to_string(),
                after_state: "Growth and change".to_string(),
                key_events: vec!["Major challenge".to_string()],
                growth: "Transformation".to_string(),
            },
        ];
        
        let result = tool.create_character_arc(
            "Test Character".to_string(),
            "Hero's Journey".to_string(),
            stages,
        ).await;
        
        assert!(result.is_success());
    }

    #[tokio::test]
    async fn test_template_management() {
        let mut tool = MigratedStructureTool::new();
        
        // Should have default templates loaded after initialization
        assert!(tool.get_available_templates().is_empty()); // Not loaded until initialization
        
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut database_context = ToolDatabaseContext::new("test_structure", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        let templates = tool.get_available_templates();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.name == "Three-Act Structure"));
        assert!(templates.iter().any(|t| t.name == "Hero's Journey"));
    }
}