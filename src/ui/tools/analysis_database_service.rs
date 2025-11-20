//! Analysis Database Service
//!
//! Provides database operations for analysis data using the new ToolDatabaseContext pattern.
//! This service replaces direct database access with safe, retry-aware operations.

use crate::ui::tools::database_integration::{ToolDatabaseContext, DatabaseOperationResult};
use crate::ui::tools::analysis_tool_migrated::{
    AnalysisData, WritingStats, WritingInsight, InsightType, TrendData, TrendDirection,
    ProjectComparison, ProjectMetric, AnalysisConfig, WritingType,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use async_trait::async_trait;

/// Service for managing analysis data in the database
pub struct AnalysisDatabaseService {
    /// Database context for operations
    database_context: Option<ToolDatabaseContext>,
    /// Analysis cache for frequently accessed data
    analysis_cache: HashMap<String, WritingStats>,
    /// Insight cache
    insight_cache: HashMap<String, Vec<WritingInsight>>,
    /// Trend cache
    trend_cache: HashMap<String, Vec<TrendData>>,
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

/// Mock database storage for analysis data
#[derive(Debug, Default)]
pub struct AnalysisDatabaseStore {
    /// Storage for analysis configurations
    pub configs: HashMap<String, AnalysisConfig>,
    /// Storage for project analysis data
    pub project_analyses: HashMap<String, Vec<WritingStats>>,
    /// Storage for writing insights
    pub insights: HashMap<String, Vec<WritingInsight>>,
    /// Storage for trend data
    pub trends: HashMap<String, Vec<TrendData>>,
    /// Storage for project comparisons
    pub comparisons: HashMap<String, ProjectComparison>,
}

impl AnalysisDatabaseStore {
    /// Create a new empty database store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add analysis configuration
    pub fn add_config(&mut self, project_id: String, config: AnalysisConfig) {
        self.configs.insert(project_id, config);
    }

    /// Get analysis configuration
    pub fn get_config(&self, project_id: &str) -> Option<&AnalysisConfig> {
        self.configs.get(project_id)
    }

    /// Add writing statistics for a project
    pub fn add_writing_stats(&mut self, project_id: String, stats: WritingStats) {
        let project_stats = self.project_analyses
            .entry(project_id)
            .or_insert_with(Vec::new);
        project_stats.push(stats);
    }

    /// Get writing statistics for a project
    pub fn get_writing_stats(&self, project_id: &str) -> Vec<&WritingStats> {
        self.project_analyses
            .get(project_id)
            .unwrap_or(&vec![])
            .iter()
            .collect()
    }

    /// Add writing insights
    pub fn add_insights(&mut self, project_id: String, insights: Vec<WritingInsight>) {
        self.insights.insert(project_id, insights);
    }

    /// Get writing insights
    pub fn get_insights(&self, project_id: &str) -> Vec<&WritingInsight> {
        self.insights
            .get(project_id)
            .unwrap_or(&vec![])
            .iter()
            .collect()
    }

    /// Add trend data
    pub fn add_trends(&mut self, project_id: String, trends: Vec<TrendData>) {
        self.trends.insert(project_id, trends);
    }

    /// Get trend data
    pub fn get_trends(&self, project_id: &str) -> Vec<&TrendData> {
        self.trends
            .get(project_id)
            .unwrap_or(&vec![])
            .iter()
            .collect()
    }

    /// Add project comparison
    pub fn add_comparison(&mut self, comparison_id: String, comparison: ProjectComparison) {
        self.comparisons.insert(comparison_id, comparison);
    }

    /// Get project comparison
    pub fn get_comparison(&self, comparison_id: &str) -> Option<&ProjectComparison> {
        self.comparisons.get(comparison_id)
    }

    /// Generate analysis data for a project (simulated)
    pub fn generate_analysis_data(&self, project_id: &str) -> AnalysisData {
        let mut data = AnalysisData::new();
        
        // Set default configuration
        data.config = AnalysisConfig {
            analysis_frequency: crate::ui::tools::analysis_tool_migrated::AnalysisFrequency::Weekly,
            tracked_metrics: vec![
                "word_count".to_string(),
                "session_time".to_string(),
                "daily_progress".to_string(),
                "productivity_trends".to_string(),
            ],
            privacy_settings: crate::ui::tools::analysis_tool_migrated::PrivacySettings {
                collect_detailed_metrics: true,
                share_anonymized_data: false,
                data_retention_days: 365,
                export_on_request: true,
            },
            notification_preferences: crate::ui::tools::analysis_tool_migrated::NotificationPreferences {
                enable_insight_notifications: true,
                enable_milestone_notifications: true,
                enable_trend_notifications: false,
                notification_frequency: "weekly".to_string(),
            },
            custom_rules: vec![],
        };
        
        data
    }

    /// Analyze writing patterns (simulated)
    pub fn analyze_writing_patterns(&self, project_id: &str) -> WritingStats {
        // Generate realistic writing statistics
        let now = Instant::now();
        
        WritingStats {
            total_words: 15000 + (project_id.len() as u64 * 1000),
            total_chars: 90000 + (project_id.len() as u64 * 6000),
            avg_words_per_day: 750.0 + (project_id.len() as f64 * 50.0),
            session_count: 45 + (project_id.len() as u64 * 3),
            total_writing_time: Duration::from_hours(60 + (project_id.len() as u64 * 4)),
            most_productive_hour: 14, // 2 PM
            writing_streak_days: 12 + (project_id.len() as u32),
            project_breakdown: {
                let mut breakdown = HashMap::new();
                breakdown.insert(project_id.to_string(), crate::ui::tools::analysis_tool_migrated::ProjectStats {
                    project_name: project_id.to_string(),
                    words: 15000,
                    chars: 90000,
                    sessions: 45,
                    last_active: Some(now),
                });
                breakdown
            },
        }
    }

    /// Generate writing insights (simulated)
    pub fn generate_writing_insights(&self, project_id: &str) -> Vec<WritingInsight> {
        let mut insights = Vec::new();
        let now = Instant::now();
        
        // Productivity pattern insight
        insights.push(WritingInsight {
            insight_type: InsightType::ProductivityPattern,
            description: format!("You are most productive on {} afternoons for project {}", 
                              "weekday", project_id),
            confidence: 0.85,
            recommendation: "Schedule your most challenging writing tasks during your peak productivity hours".to_string(),
            supporting_data: serde_json::json!({
                "peak_hour": 14,
                "productivity_score": 0.85,
                "consistency": 0.78
            }),
            generated_at: now,
        });
        
        // Writing habit insight
        insights.push(WritingInsight {
            insight_type: InsightType::WritingHabit,
            description: format!("You maintain a consistent {}-day writing streak for project {}", 
                              12, project_id),
            confidence: 0.92,
            recommendation: "Keep up the excellent consistency! Consider setting a new streak goal".to_string(),
            supporting_data: serde_json::json!({
                "current_streak": 12,
                "average_streak": 8,
                "consistency_score": 0.92
            }),
            generated_at: now,
        });
        
        // Progress milestone insight
        insights.push(WritingInsight {
            insight_type: InsightType::ProgressMilestone,
            description: format!("You've written {} words for project {} - that's {}% of a typical novel!",
                              15000, project_id, (15000.0 / 50000.0 * 100.0).round()),
            confidence: 0.98,
            recommendation: "Great progress! Consider outlining the next major section to maintain momentum".to_string(),
            supporting_data: serde_json::json!({
                "current_words": 15000,
                "novel_average": 50000,
                "progress_percentage": 30.0
            }),
            generated_at: now,
        });
        
        insights
    }

    /// Analyze writing trends (simulated)
    pub fn analyze_writing_trends(&self, project_id: &str, days: u32) -> Vec<TrendData> {
        let mut trends = Vec::new();
        let now = Instant::now();
        
        for i in 0..days.min(30) { // Limit to 30 days for performance
            let timestamp = now - Duration::from_secs((i as u64) * 24 * 60 * 60); // Go back day by day
            
            // Generate trend data with some variation
            let base_words = 500.0;
            let variation = (i as f64 * 0.1).sin() * 100.0; // Sin wave pattern
            let words_written = base_words + variation;
            
            let trend_direction = if variation > 50.0 {
                TrendDirection::Upward
            } else if variation < -50.0 {
                TrendDirection::Downward
            } else {
                TrendDirection::Stable
            };
            
            trends.push(TrendData {
                timestamp,
                value: words_written,
                metric_type: "daily_word_count".to_string(),
                trend_direction,
            });
        }
        
        trends
    }

    /// Compare project metrics (simulated)
    pub fn compare_projects_metrics(&self, project_ids: &[String]) -> ProjectComparison {
        let mut metrics = HashMap::new();
        let mut project_metrics = Vec::new();
        
        for (i, project_id) in project_ids.iter().enumerate() {
            let base_words = 10000 + (i * 5000) as f64;
            let productivity_score = 0.7 + (i as f64 * 0.1);
            
            let metric = ProjectMetric {
                project_id: project_id.clone(),
                value: base_words,
                metric_type: "total_words".to_string(),
                comparison_to_average: productivity_score,
                percentile: 50.0 + (i as f64 * 10.0),
            };
            
            metrics.insert(project_id.clone(), metric);
            project_metrics.push(project_id.clone());
        }
        
        // Sort by productivity score (descending)
        project_metrics.sort_by(|a, b| {
            metrics.get(b).unwrap().comparison_to_average
                .partial_cmp(&metrics.get(a).unwrap().comparison_to_average)
                .unwrap()
        });
        
        ProjectComparison {
            projects: project_ids.to_vec(),
            metrics,
            ranking: project_metrics,
            insights: vec![
                "Project productivity varies significantly across projects".to_string(),
                "Consider analyzing time allocation between projects".to_string(),
            ],
            generated_at: Instant::now(),
        }
    }
}

/// Global database store instance (for demo purposes)
lazy_static::lazy_static! {
    static ref GLOBAL_ANALYSIS_DB: tokio::sync::RwLock<AnalysisDatabaseStore> = 
        tokio::sync::RwLock::new(AnalysisDatabaseStore::new());
}

impl AnalysisDatabaseService {
    /// Create a new analysis database service
    pub fn new() -> Self {
        Self {
            database_context: None,
            analysis_cache: HashMap::new(),
            insight_cache: HashMap::new(),
            trend_cache: HashMap::new(),
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

    /// Get analysis data for a project
    pub async fn get_analysis_data_by_project(&self, project_id: &str) -> DatabaseOperationResult<AnalysisData> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "get_analysis_data_by_project",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_ANALYSIS_DB.read().await;
                    let analysis_data = db_store.generate_analysis_data(project_id);
                    Ok::<AnalysisData, String>(analysis_data)
                }),
                3,
            ).await;

            match result {
                Ok(data) => {
                    DatabaseOperationResult::success(data, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("get_analysis_data", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Analyze writing patterns for a project
    pub async fn analyze_writing_patterns(&self, project_id: &str) -> DatabaseOperationResult<WritingStats> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_stats) = self.analysis_cache.get(project_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_stats.clone(), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "analyze_writing_patterns",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_ANALYSIS_DB.read().await;
                    let stats = db_store.analyze_writing_patterns(project_id);
                    Ok::<WritingStats, String>(stats)
                }),
                3,
            ).await;

            match result {
                Ok(stats) => {
                    // Cache the results
                    self.analysis_cache.insert(project_id.to_string(), stats.clone());
                    DatabaseOperationResult::success(stats, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("analyze_patterns", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Generate writing insights
    pub async fn generate_writing_insights(&self, project_id: &str) -> DatabaseOperationResult<Vec<WritingInsight>> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_insights) = self.insight_cache.get(project_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_insights.clone(), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "generate_writing_insights",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_ANALYSIS_DB.read().await;
                    let insights = db_store.generate_writing_insights(project_id);
                    Ok::<Vec<WritingInsight>, String>(insights)
                }),
                3,
            ).await;

            match result {
                Ok(insights) => {
                    // Cache the results
                    self.insight_cache.insert(project_id.to_string(), insights.clone());
                    DatabaseOperationResult::success(insights, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("generate_insights", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Analyze writing trends
    pub async fn analyze_writing_trends(&self, project_id: &str, days: u32) -> DatabaseOperationResult<Vec<TrendData>> {
        let start_time = Instant::now();
        
        // Try cache first
        if let Some(cached_trends) = self.trend_cache.get(project_id) {
            self.cache_stats.hits += 1;
            self.update_cache_hit_rate();
            return DatabaseOperationResult::success(cached_trends.clone(), start_time.elapsed());
        }
        
        // Cache miss
        self.cache_stats.misses += 1;
        self.update_cache_hit_rate();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "analyze_writing_trends",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_ANALYSIS_DB.read().await;
                    let trends = db_store.analyze_writing_trends(project_id, days);
                    Ok::<Vec<TrendData>, String>(trends)
                }),
                3,
            ).await;

            match result {
                Ok(trends) => {
                    // Cache the results
                    self.trend_cache.insert(project_id.to_string(), trends.clone());
                    DatabaseOperationResult::success(trends, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("analyze_trends", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Compare projects metrics
    pub async fn compare_projects_metrics(&self, project_ids: &[String]) -> DatabaseOperationResult<ProjectComparison> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "compare_projects_metrics",
                |service| Box::pin(async move {
                    let db_store = GLOBAL_ANALYSIS_DB.read().await;
                    let comparison = db_store.compare_projects_metrics(project_ids);
                    Ok::<ProjectComparison, String>(comparison)
                }),
                3,
            ).await;

            match result {
                Ok(comparison) => {
                    DatabaseOperationResult::success(comparison, start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("compare_projects", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Save analysis configuration
    pub async fn save_analysis_config(&self, config: &AnalysisConfig) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "save_analysis_config",
                |service| Box::pin(async move {
                    let mut db_store = GLOBAL_ANALYSIS_DB.write().await;
                    // For demo purposes, we'll use a default project ID
                    db_store.add_config("default_project".to_string(), config.clone());
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    DatabaseOperationResult::success((), start_time.elapsed())
                }
                Err(e) => {
                    DatabaseOperationResult::validation_error("save_config", e.to_string())
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
        self.analysis_cache.clear();
        self.insight_cache.clear();
        self.trend_cache.clear();
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
    async fn test_analysis_database_service_creation() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        assert!(service.database_context.is_some());
    }

    #[tokio::test]
    async fn test_analysis_data_retrieval() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.get_analysis_data_by_project("test_project").await;
        assert!(result.is_success());
        
        let analysis_data = result.data.unwrap();
        assert_eq!(analysis_data.config.tracked_metrics.len(), 4);
        assert!(analysis_data.config.privacy_settings.collect_detailed_metrics);
    }

    #[tokio::test]
    async fn test_writing_patterns_analysis() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.analyze_writing_patterns("test_project").await;
        assert!(result.is_success());
        
        let stats = result.data.unwrap();
        assert!(stats.total_words > 0);
        assert!(stats.avg_words_per_day > 0.0);
        assert_eq!(stats.project_breakdown.len(), 1);
        
        // Test cache hit
        let cached_result = service.analyze_writing_patterns("test_project").await;
        assert!(cached_result.is_success());
        
        let cache_stats = service.get_cache_stats();
        assert_eq!(cache_stats.hits, 1);
        assert_eq!(cache_stats.misses, 1); // First call was a miss, second was a hit
    }

    #[tokio::test]
    async fn test_writing_insights_generation() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.generate_writing_insights("test_project").await;
        assert!(result.is_success());
        
        let insights = result.data.unwrap();
        assert!(!insights.is_empty());
        
        // Verify insight structure
        let first_insight = &insights[0];
        assert!(!first_insight.description.is_empty());
        assert!(first_insight.confidence > 0.0 && first_insight.confidence <= 1.0);
        assert!(!first_insight.recommendation.is_empty());
    }

    #[tokio::test]
    async fn test_writing_trends_analysis() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let result = service.analyze_writing_trends("test_project", 7).await;
        assert!(result.is_success());
        
        let trends = result.data.unwrap();
        assert_eq!(trends.len(), 7); // Should have 7 days of data
        
        // Verify trend structure
        for trend in &trends {
            assert!(trend.value > 0.0);
            assert_eq!(trend.metric_type, "daily_word_count");
        }
    }

    #[tokio::test]
    async fn test_project_comparison() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let project_ids = vec![
            "project_alpha".to_string(),
            "project_beta".to_string(),
            "project_gamma".to_string(),
        ];
        
        let result = service.compare_projects_metrics(&project_ids).await;
        assert!(result.is_success());
        
        let comparison = result.data.unwrap();
        assert_eq!(comparison.projects.len(), 3);
        assert_eq!(comparison.metrics.len(), 3);
        assert_eq!(comparison.ranking.len(), 3);
        assert!(!comparison.insights.is_empty());
    }

    #[tokio::test]
    async fn test_analysis_config_save() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        
        let config = AnalysisConfig {
            analysis_frequency: crate::ui::tools::analysis_tool_migrated::AnalysisFrequency::Daily,
            tracked_metrics: vec!["word_count".to_string(), "session_time".to_string()],
            privacy_settings: crate::ui::tools::analysis_tool_migrated::PrivacySettings {
                collect_detailed_metrics: true,
                share_anonymized_data: false,
                data_retention_days: 365,
                export_on_request: true,
            },
            notification_preferences: crate::ui::tools::analysis_tool_migrated::NotificationPreferences {
                enable_insight_notifications: true,
                enable_milestone_notifications: true,
                enable_trend_notifications: false,
                notification_frequency: "daily".to_string(),
            },
            custom_rules: vec![],
        };
        
        let result = service.save_analysis_config(&config).await;
        assert!(result.is_success());
    }

    #[tokio::test]
    async fn test_cache_performance() {
        let mut service = AnalysisDatabaseService::new();
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_analysis_db", database_state).await;
        
        service.initialize(database_context).await;
        
        // Perform multiple operations to test caching
        for _ in 0..5 {
            service.analyze_writing_patterns("test_project").await;
        }
        
        let cache_stats = service.get_cache_stats();
        assert_eq!(cache_stats.misses, 1); // Only first call should miss
        assert_eq!(cache_stats.hits, 4); // Subsequent calls should hit
        assert_eq!(cache_stats.hit_rate, 80.0); // 4 out of 5 should be hits
    }
}