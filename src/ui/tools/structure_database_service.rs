//! Structure Database Service
//!
//! Provides database operations for structure data using the new ToolDatabaseContext pattern.
//! This service replaces direct database access with safe, retry-aware operations.

use crate::ui::tools::database_integration::{ToolDatabaseContext, DatabaseOperationResult};
use crate::ui::tools::structure_tool_migrated::{
    StructureData, StructureAnalysis, StructureRecommendation, RecommendationType,
    CharacterArcStage, PlotType,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use async_trait::async_trait;

/// Service for managing structure data in the database
pub struct StructureDatabaseService {
    /// Database context for operations
    database_context: Option<ToolDatabaseContext>,
    /// Structure cache for frequently accessed data
    structure_cache: HashMap<String, StructureData>,
    /// Analysis cache
    analysis_cache: HashMap<String, StructureAnalysis>,
    /// Recommendation cache
    recommendation_cache: HashMap<String, Vec<StructureRecommendation>>,
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

/// Mock database storage for structure data
#[derive(Debug, Default)]
pub struct StructureDatabaseStore {
    /// Storage for structure data
    pub structures: HashMap<String, StructureData>,
    /// Storage for plot stages
    pub plot_stages: HashMap<String, Vec<crate::ui::tools::structure_tool_migrated::PlotStage>>,
    /// Storage for character arcs
    pub character_arcs: HashMap<String, Vec<crate::ui::tools::structure_tool_migrated::CharacterArc>>,
    /// Storage for world structures
    pub world_structures: HashMap<String, Vec<crate::ui::tools::structure_tool_migrated::WorldStructure>>,
}

impl StructureDatabaseStore {
    /// Create a new empty database store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add structure data
    pub fn add_structure(&mut self, project_id: String, structure_data: StructureData) {
        self.structures.insert(project_id, structure_data);
    }

    /// Get structure data
    pub fn get_structure(&self, project_id: &str) -> Option<&StructureData> {
        self.structures.get(project_id)
    }

    /// Add plot stage
    pub fn add_plot_stage(&mut self, stage_id: String, structure_id: String, stage: crate::ui::tools::structure_tool_migrated::PlotStage) {
        self.plot_stages
            .entry(structure_id)
            .or_insert_with(Vec::new)
            .push(stage);
    }

    /// Get plot stages for a structure
    pub fn get_plot_stages(&self, structure_id: &str) -> Vec<&crate::ui::tools::structure_tool_migrated::PlotStage> {
        self.plot_stages
            .get(structure_id)
            .unwrap_or(&vec![])
            .iter()
            .collect()
    }

    /// Add character arc
    pub fn add_character_arc(&mut self, arc_id: String, project_id: String, arc: crate::ui::tools::structure_tool_migrated::CharacterArc) {
        self.character_arcs
            .entry(project_id)
            .or_insert_with(Vec::new)
            .push(arc);
    }

    /// Get character arcs for a project
    pub fn get_character_arcs(&self, project_id: &str) -> Vec<&crate::ui::tools::structure_tool_migrated::CharacterArc> {
        self.character_arcs
            .get(project_id)
            .unwrap_or(&vec![])
            .iter()
            .collect()
    }

    /// Add world structure
    pub fn add_world_structure(&mut self, structure_id: String, project_id: String, world_structure: crate::ui::tools::structure_tool_migrated::WorldStructure) {
        self.world_structures
            .entry(project_id)
            .or_insert_with(Vec::new)
            .push(world_structure);
    }

    /// Get world structures for a project
    pub fn get_world_structures(&self, project_id: &str) -> Vec<&crate::ui::tools::structure_tool_migrated::WorldStructure> {
        self.world_structures
            .get(project_id)
            .unwrap_or(&vec![])
            .iter()
            .collect()
    }

    /// Create plot structure (simulated)
    pub fn create_plot_structure(
        &mut self,
        structure_id: &str,
        project_id: &str,
        plot_type: PlotType,
        title: &str,
        description: &str,
    ) -> Result<()> {
        // Create a new structure data entry
        let mut structure_data = StructureData::new();
        structure_data.plots = vec![structure_id.to_string()];
        structure_data.plot_type = plot_type;
        
        self.structures.insert(project_id.to_string(), structure_data);
        Ok(())
    }

    /// Add plot stage to structure (simulated)
    pub fn add_plot_stage_to_structure(
        &mut self,
        stage_id: &str,
        structure_id: &str,
        stage_name: &str,
        stage_description: &str,
        order: u32,
        word_target: Option<u32>,
    ) -> Result<()> {
        let stage = crate::ui::tools::structure_tool_migrated::PlotStage {
            id: stage_id.to_string(),
            structure_id: structure_id.to_string(),
            name: stage_name.to_string(),
            description: stage_description.to_string(),
            order,
            completed: false,
            word_target,
            word_count: 0,
            start_date: Some(Instant::now()),
            end_date: None,
        };
        
        self.plot_stages
            .entry(structure_id.to_string())
            .or_insert_with(Vec::new)
            .push(stage);
        
        Ok(())
    }

    /// Create character arc (simulated)
    pub fn create_character_arc(
        &mut self,
        arc_id: &str,
        character_name: &str,
        arc_type: &str,
        stages: &[CharacterArcStage],
    ) -> Result<()> {
        let arc = crate::ui::tools::structure_tool_migrated::CharacterArc {
            id: arc_id.to_string(),
            character_name: character_name.to_string(),
            arc_type: arc_type.to_string(),
            stages: stages.to_vec(),
            completed: false,
            current_stage: 0,
        };
        
        // For demo purposes, use a default project ID
        self.character_arcs
            .entry("default_project".to_string())
            .or_insert_with(Vec::new)
            .push(arc);
        
        Ok(())
    }

    /// Analyze structure (simulated)
    pub fn analyze_structure(&self, structure_id: &str) -> StructureAnalysis {
        let now = Instant::now();
        
        StructureAnalysis {
            coherence_score: 0.75, // 75% coherent
            completeness: 0.60, // 60% complete
            plot_holes: vec![
                crate::ui::tools::structure_tool_migrated::PlotHole {
                    description: "Missing character motivation".to_string(),
                    severity: 7,
                    location: "Act 2, Scene 3".to_string(),
                    suggested_fix: "Add scene showing character's personal stakes".to_string(),
                    impact: "High - affects character agency".to_string(),
                },
                crate::ui::tools::structure_tool_migrated::PlotHole {
                    description: "Underdeveloped subplot".to_string(),
                    severity: 5,
                    location: "Act 1, Setup".to_string(),
                    suggested_fix: "Expand secondary character introduction".to_string(),
                    impact: "Medium - affects story depth".to_string(),
                },
            ],
            pacing_analysis: crate::ui::tools::structure_tool_migrated::PacingAnalysis {
                pacing_score: 0.68,
                fast_sections: vec!["Action sequences".to_string()],
                slow_sections: vec!["Exposition scenes".to_string()],
                recommendations: vec![
                    "Consider tightening exposition in Act 1".to_string(),
                    "Add more tension to slow sections".to_string(),
                ],
                word_count_analysis: crate::ui::tools::structure_tool_migrated::WordCountAnalysis {
                    actual_vs_planned: vec![
                        ("Act 1".to_string(), 8000, 10000),
                        ("Act 2".to_string(), 25000, 30000),
                        ("Act 3".to_string(), 12000, 15000),
                    ],
                    distribution_by_section: {
                        let mut map = HashMap::new();
                        map.insert("Setup".to_string(), (8000, 0.18));
                        map.insert("Confrontation".to_string(), (25000, 0.56));
                        map.insert("Resolution".to_string(), (12000, 0.27));
                        map
                    },
                    pacing_indicators: vec![
                        crate::ui::tools::structure_tool_migrated::PacingIndicator {
                            indicator_type: "Scene length average".to_string(),
                            value: 2500.0,
                            threshold: 3000.0,
                            status: "Good".to_string(),
                        },
                        crate::ui::tools::structure_tool_migrated::PacingIndicator {
                            indicator_type: "Dialogue vs action ratio".to_string(),
                            value: 0.45,
                            threshold: 0.5,
                            status: "Warning".to_string(),
                        },
                    ],
                },
            },
            arc_integration_score: 0.62,
            worldbuilding_consistency: 0.71,
            generated_at: now,
        }
    }

    /// Generate recommendations (simulated)
    pub fn generate_recommendations(&self, project_id: &str) -> Vec<StructureRecommendation> {
        let mut recommendations = Vec::new();
        let now = Instant::now();
        
        // Add sample recommendations
        recommendations.push(StructureRecommendation {
            recommendation_type: RecommendationType::AddPlotElement,
            description: "Add a subplot to deepen character relationships".to_string(),
            priority: 8,
            expected_impact: "High - improves character depth and reader engagement".to_string(),
            complexity: "Medium".to_string(),
            related_elements: vec!["Character arcs".to_string(), "Relationships".to_string()],
        });
        
        recommendations.push(StructureRecommendation {
            recommendation_type: RecommendationType::FixPacing,
            description: "Tighten Act 2 pacing by reducing exposition".to_string(),
            priority: 7,
            expected_impact: "Medium - improves narrative flow".to_string(),
            complexity: "Low".to_string(),
            related_elements: vec!["Act 2".to_string(), "Pacing".to_string()],
        });
        
        recommendations.push(StructureRecommendation {
            recommendation_type: RecommendationType::ImproveCharacter,
            description: "Develop antagonist's motivation more clearly".to_string(),
            priority: 9,
            expected_impact: "High - creates stronger conflict".to_string(),
            complexity: "Medium".to_string(),
            related_elements: vec!["Antagonist".to_string(), "Motivation".to_string()],
        });
        
        recommendations
    }
}

/// Global database store instance (for demo purposes)
lazy_static::lazy_static! {
    static ref GLOBAL_STRUCTURE_DB: tokio::sync::RwLock<StructureDatabaseStore> = 
        tokio::sync::RwLock::new(StructureDatabaseStore::new());
}

impl StructureDatabaseService {
    /// Create a new structure database service
    pub fn new() -> Self {
        Self {
            database_context: None,
            structure_cache: HashMap::new(),
            analysis_cache: HashMap::new(),
            recommendation_cache: HashMap::new(),
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

    /// Get structure data for a project
    pub async fn get_structure_data_by_project(&self, project_id: &str) -> DatabaseOperationResult<StructureData> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_data) = self.structure_cache.get(project_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_data.clone(), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_structure_data_by_project",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_STRUCTURE_DB.read().await;
                    let structure_data = db_store.get_structure(project_id)
                        .cloned()
                        .unwrap_or_else(|| StructureData::new());
                    Ok::<StructureData, String>(structure_data)
                }),
                3,
            ).await;

            match result {
                Ok(data) => {
                    // Cache the results
                    self.structure_cache.insert(project_id.to_string(), data.clone());
                    DatabaseOperationResult::success(data, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_structure_data", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Create plot structure
    pub async fn create_plot_structure(
        &self,
        structure_id: &str,
        project_id: &str,
        plot_type: PlotType,
        title: &str,
        description: &str,
    ) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "create_plot_structure",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_STRUCTURE_DB.write().await;
                    db_store.create_plot_structure(structure_id, project_id, plot_type, title, description)
                        .map_err(|e| e.to_string())?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("create_structure", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Add plot stage
    pub async fn add_plot_stage(
        &self,
        stage_id: &str,
        structure_id: &str,
        stage_name: &str,
        stage_description: &str,
        order: u32,
        word_target: Option<u32>,
    ) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "add_plot_stage",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_STRUCTURE_DB.write().await;
                    db_store.add_plot_stage_to_structure(stage_id, structure_id, stage_name, stage_description, order, word_target)
                        .map_err(|e| e.to_string())?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("add_stage", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Create character arc
    pub async fn create_character_arc(
        &self,
        arc_id: &str,
        character_name: &str,
        arc_type: &str,
        stages: &[CharacterArcStage],
    ) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "create_character_arc",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_STRUCTURE_DB.write().await;
                    db_store.create_character_arc(arc_id, character_name, arc_type, stages)
                        .map_err(|e| e.to_string())?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("create_arc", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Analyze structure
    pub async fn analyze_structure(&self, structure_id: &str) -> DatabaseOperationResult<StructureAnalysis> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_analysis) = self.analysis_cache.get(structure_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_analysis.clone(), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "analyze_structure",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_STRUCTURE_DB.read().await;
                    let analysis = db_store.analyze_structure(structure_id);
                    Ok::<StructureAnalysis, String>(analysis)
                }),
                3,
            ).await;

            match result {
                Ok(analysis) => {
                    // Cache the results
                    self.analysis_cache.insert(structure_id.to_string(), analysis.clone());
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

    /// Generate recommendations
    pub async fn generate_recommendations(&self, project_id: &str) -> DatabaseOperationResult<Vec<StructureRecommendation>> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_recommendations) = self.recommendation_cache.get(project_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_recommendations.clone(), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "generate_recommendations",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_STRUCTURE_DB.read().await;
                    let recommendations = db_store.generate_recommendations(project_id);
                    Ok::<Vec<StructureRecommendation>, String>(recommendations)
                }),
                3,
            ).await;

            match result {
                Ok(recommendations) => {
                    // Cache the results
                    self.recommendation_cache.insert(project_id.to_string(), recommendations.clone());
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

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        self.cache_stats.clone()
    }

    /// Clear all caches
    pub fn clear_cache(&mut self) {
        self.structure_cache.clear();
        self.analysis_cache.clear();
        self.recommendation_cache.clear();
        self.cache_stats = CacheStats {
            hits: 0,
            misses: 0,
            hit_rate: 0.0,
        };
    }

    /// Update cache hit rate
    fn update_cache_hit_rate(&mut self) {
        let total_requests = self.cache_stats.hits + self.cache_stats.misses;
        if total_requests > 0 {
            self.cache_stats.hit_rate = (self.cache_stats.hits as f64 / total_requests as f64) * 100.0;
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
    async fn test_structure_database_service_creation() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        assert!(service.database_context.is_some());
    }

    #[tokio::test]
    async fn test_structure_data_retrieval() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.get_structure_data_by_project("test_project").await;
        assert!(result.is_success());
        
        let structure_data = result.data.unwrap();
        assert!(structure_data.plots.is_empty()); // Should be empty for non-existent project
    }

    #[tokio::test]
    async fn test_plot_structure_creation() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.create_plot_structure(
            "test_structure",
            "test_project",
            PlotType::Traditional,
            "Test Structure",
            "A test plot structure",
        ).await;
        
        assert!(result.is_success());
    }

    #[tokio::test]
    async fn test_plot_stage_addition() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.add_plot_stage(
            "test_stage",
            "test_structure",
            "Test Stage",
            "A test plot stage",
            1,
            Some(10000),
        ).await;
        
        assert!(result.is_success());
    }

    #[tokio::test]
    async fn test_character_arc_creation() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let stages = vec![
            CharacterArcStage {
                name: "Introduction".to_string(),
                description: "Character is introduced".to_string(),
                before_state: "Normal life".to_string(),
                after_state: "Life disrupted".to_string(),
                key_events: vec!["Inciting incident".to_string()],
                growth: "Awareness".to_string(),
            },
        ];
        
        let result = service.create_character_arc(
            "test_arc",
            "Test Character",
            "Hero's Journey",
            &stages,
        ).await;
        
        assert!(result.is_success());
    }

    #[tokio::test]
    async fn test_structure_analysis() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.analyze_structure("test_structure").await;
        assert!(result.is_success());
        
        let analysis = result.data.unwrap();
        assert!(analysis.coherence_score > 0.0 && analysis.coherence_score <= 1.0);
        assert!(!analysis.plot_holes.is_empty());
        assert!(analysis.pacing_analysis.pacing_score > 0.0 && analysis.pacing_analysis.pacing_score <= 1.0);
    }

    #[tokio::test]
    async fn test_recommendations_generation() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.generate_recommendations("test_project").await;
        assert!(result.is_success());
        
        let recommendations = result.data.unwrap();
        assert!(!recommendations.is_empty());
        
        // Verify recommendation structure
        let first_recommendation = &recommendations[0];
        assert!(!first_recommendation.description.is_empty());
        assert!(first_recommendation.priority > 0 && first_recommendation.priority <= 10);
        assert!(!first_recommendation.expected_impact.is_empty());
    }

    #[tokio::test]
    async fn test_cache_performance() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Perform multiple operations to test caching
        for _ in 0..5 {
            service.analyze_structure("test_structure").await;
        }
        
        let cache_stats = service.get_cache_stats();
        assert_eq!(cache_stats.misses, 1); // Only first call should miss
        assert_eq!(cache_stats.hits, 4); // Subsequent calls should hit
        assert_eq!(cache_stats.hit_rate, 80.0); // 4 out of 5 should be hits
    }

    #[tokio::test]
    async fn test_cache_clearing() {
        let mut service = StructureDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_structure_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Generate some cached data
        service.analyze_structure("test_structure").await;
        service.generate_recommendations("test_project").await;
        
        // Verify cache has data
        let cache_stats_before = service.get_cache_stats();
        assert!(cache_stats_before.hits == 0 && cache_stats_before.misses > 0);
        
        // Clear cache
        service.clear_cache();
        
        // Verify cache is empty
        let cache_stats_after = service.get_cache_stats();
        assert_eq!(cache_stats_after.hits, 0);
        assert_eq!(cache_stats_after.misses, 0);
        assert_eq!(cache_stats_after.hit_rate, 0.0);
    }
}