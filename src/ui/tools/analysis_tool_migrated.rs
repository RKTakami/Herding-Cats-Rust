//! Migrated Analysis Tool Implementation
//!
//! This module provides the migrated version of the analysis tool using the new
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

use super::analysis_ui::{AnalysisTool as LegacyAnalysisTool, WritingType, AnalysisData};

/// Migrated analysis tool that implements the new architecture patterns
pub struct MigratedAnalysisTool {
    /// Database context for safe database operations
    database_context: Option<ToolDatabaseContext>,
    /// API contract for cross-tool communication
    api_contract: Arc<ToolApiContract>,
    /// Analysis data and configuration
    analysis_data: AnalysisData,
    /// Tool registry reference for lifecycle management
    tool_registry: &'static crate::ui::tools::threading_patterns::ThreadSafeToolRegistry,
    /// Tool initialization timestamp
    initialized_at: Option<Instant>,
    /// Last operation duration tracking
    last_operation_duration: Option<Duration>,
    /// Performance and analysis metrics
    analysis_metrics: AnalysisMetrics,
    /// Writing statistics cache
    writing_stats_cache: Option<WritingStats>,
}

/// Writing statistics for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingStats {
    /// Total word count
    pub total_words: u64,
    /// Total characters
    pub total_chars: u64,
    /// Average words per day
    pub avg_words_per_day: f64,
    /// Writing sessions count
    pub session_count: u64,
    /// Total writing time
    pub total_writing_time: Duration,
    /// Most productive hour
    pub most_productive_hour: u8,
    /// Writing streak days
    pub writing_streak_days: u32,
    /// Project breakdown
    pub project_breakdown: std::collections::HashMap<String, ProjectStats>,
}

/// Project-specific statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    /// Project name
    pub project_name: String,
    /// Words written for this project
    pub words: u64,
    /// Characters written
    pub chars: u64,
    /// Session count
    pub sessions: u64,
    /// Last active timestamp
    pub last_active: Option<Instant>,
}

/// Analysis metrics and statistics
#[derive(Debug, Clone)]
pub struct AnalysisMetrics {
    /// Total analysis operations performed
    pub total_analyses: u64,
    /// Average analysis time
    pub avg_analysis_time_ms: f64,
    /// Analysis success rate
    pub success_rate: f64,
    /// Cache hit rate for statistics
    pub cache_hit_rate: f64,
    /// Last analysis timestamp
    pub last_analysis_time: Option<Instant>,
}

impl Default for AnalysisMetrics {
    fn default() -> Self {
        Self {
            total_analyses: 0,
            avg_analysis_time_ms: 0.0,
            success_rate: 100.0,
            cache_hit_rate: 0.0,
            last_analysis_time: None,
        }
    }
}

impl MigratedAnalysisTool {
    /// Create a new migrated analysis tool
    pub fn new() -> Self {
        Self {
            database_context: None,
            api_contract: get_api_contract().clone(),
            analysis_data: AnalysisData::new(),
            tool_registry: get_tool_registry(),
            initialized_at: None,
            last_operation_duration: None,
            analysis_metrics: AnalysisMetrics::default(),
            writing_stats_cache: None,
        }
    }

    /// Load analysis data from database with retry logic
    pub async fn load_analysis_data(&mut self, project_id: &str) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let result = context.execute_with_retry(
                "load_analysis_data",
                |service| Box::pin(async move {
                    let analysis_data = service.get_analysis_data_by_project(project_id).await?;
                    Ok::<AnalysisData, String>(analysis_data)
                }),
                3,
            ).await;

            match result {
                Ok(data) => {
                    self.analysis_data = data;
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_analysis_metrics(true, duration);
                    
                    // Broadcast successful load
                    self.broadcast_analysis_event("data_loaded").await;
                    
                    DatabaseOperationResult::success((), duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.update_analysis_metrics(false, duration);
                    DatabaseOperationResult::validation_error("load_analysis", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Perform comprehensive writing analysis
    pub async fn perform_analysis(&mut self, project_id: &str) -> DatabaseOperationResult<WritingStats> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let result = context.execute_with_retry(
                "perform_analysis",
                |service| Box::pin(async move {
                    let stats = service.analyze_writing_patterns(project_id).await?;
                    Ok::<WritingStats, String>(stats)
                }),
                3,
            ).await;

            match result {
                Ok(stats) => {
                    // Cache the results
                    self.writing_stats_cache = Some(stats.clone());
                    
                    let duration = start_time.elapsed();
                    self.last_operation_duration = Some(duration);
                    self.update_analysis_metrics(true, duration);
                    self.analysis_metrics.last_analysis_time = Some(Instant::now());
                    
                    // Broadcast analysis completion
                    self.broadcast_analysis_event("analysis_completed").await;
                    
                    DatabaseOperationResult::success(stats, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.update_analysis_metrics(false, duration);
                    DatabaseOperationResult::validation_error("perform_analysis", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Generate writing insights and recommendations
    pub async fn generate_insights(&self, project_id: &str) -> DatabaseOperationResult<Vec<WritingInsight>> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "generate_insights",
                |service| Box::pin(async move {
                    let insights = service.generate_writing_insights(project_id).await?;
                    Ok::<Vec<WritingInsight>, String>(insights)
                }),
                3,
            ).await;

            match result {
                Ok(insights) => {
                    let duration = start_time.elapsed();
                    
                    DatabaseOperationResult::success(insights, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("generate_insights", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Analyze writing trends over time
    pub async fn analyze_trends(&self, project_id: &str, days: u32) -> DatabaseOperationResult<Vec<TrendData>> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "analyze_trends",
                |service| Box::pin(async move {
                    let trends = service.analyze_writing_trends(project_id, days).await?;
                    Ok::<Vec<TrendData>, String>(trends)
                }),
                3,
            ).await;

            match result {
                Ok(trends) => {
                    let duration = start_time.elapsed();
                    
                    DatabaseOperationResult::success(trends, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("analyze_trends", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Compare writing metrics between projects
    pub async fn compare_projects(&self, project_ids: &[String]) -> DatabaseOperationResult<ProjectComparison> {
        let start_time = Instant::now();
        
        if let Some(context) = &self.database_context {
            let result = context.execute_with_retry(
                "compare_projects",
                |service| Box::pin(async move {
                    let comparison = service.compare_projects_metrics(project_ids).await?;
                    Ok::<ProjectComparison, String>(comparison)
                }),
                3,
            ).await;

            match result {
                Ok(comparison) => {
                    let duration = start_time.elapsed();
                    
                    DatabaseOperationResult::success(comparison, duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("compare_projects", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Save analysis configuration
    pub async fn save_analysis_config(&mut self, config: &AnalysisConfig) -> DatabaseOperationResult<()> {
        let start_time = Instant::now();
        
        if let Some(context) = &mut self.database_context {
            let result = context.execute_with_retry(
                "save_analysis_config",
                |service| Box::pin(async move {
                    service.save_analysis_config(config).await?;
                    Ok::<(), String>(())
                }),
                3,
            ).await;

            match result {
                Ok(_) => {
                    // Update local configuration
                    self.analysis_data.config = config.clone();
                    
                    let duration = start_time.elapsed();
                    
                    DatabaseOperationResult::success((), duration)
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    DatabaseOperationResult::validation_error("save_config", e.to_string())
                }
            }
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }

    /// Get analysis statistics
    pub fn get_analysis_stats(&self) -> AnalysisStats {
        let cached_stats = if self.writing_stats_cache.is_some() { 1 } else { 0 };
        let total_cache_operations = self.analysis_metrics.total_analyses;
        
        AnalysisStats {
            total_analyses: self.analysis_metrics.total_analyses,
            avg_analysis_time_ms: self.analysis_metrics.avg_analysis_time_ms,
            success_rate: self.analysis_metrics.success_rate,
            last_analysis_time: self.analysis_metrics.last_analysis_time,
            cache_hit_rate: if total_cache_operations > 0 {
                (cached_stats as f64 / total_cache_operations as f64) * 100.0
            } else {
                0.0
            },
            cached_data_available: self.writing_stats_cache.is_some(),
            analysis_data_size: self.analysis_data.get_data_size(),
        }
    }

    /// Get cached writing statistics
    pub fn get_cached_writing_stats(&self) -> Option<&WritingStats> {
        self.writing_stats_cache.as_ref()
    }

    /// Clear cached statistics
    pub fn clear_cache(&mut self) {
        self.writing_stats_cache = None;
    }

    /// Migrate from legacy analysis tool
    pub async fn migrate_from_legacy(
        legacy_tool: LegacyAnalysisTool,
        database_context: ToolDatabaseContext,
    ) -> Result<Self> {
        let mut migrated_tool = Self::new();
        
        // Initialize with database context
        let mut db_context = database_context;
        migrated_tool.initialize(&mut db_context).await?;
        
        // Copy analysis data
        migrated_tool.analysis_data = legacy_tool.analysis_data;
        
        Ok(migrated_tool)
    }

    /// Update analysis metrics
    fn update_analysis_metrics(&mut self, success: bool, duration: Duration) {
        self.analysis_metrics.total_analyses += 1;
        
        if success {
            // Update moving average for analysis time
            let current_avg = self.analysis_metrics.avg_analysis_time_ms;
            let operation_ms = duration.as_millis_f64();
            let count = self.analysis_metrics.total_analyses as f64;
            self.analysis_metrics.avg_analysis_time_ms = 
                (current_avg * (count - 1.0) + operation_ms) / count;
        } else {
            // Update success rate
            let successful_analyses = self.analysis_metrics.total_analyses - 1;
            self.analysis_metrics.success_rate = 
                (successful_analyses as f64 / self.analysis_metrics.total_analyses as f64) * 100.0;
        }
    }

    /// Broadcast analysis-related events
    async fn broadcast_analysis_event(&self, event_type: &str) {
        if let Err(e) = self.api_contract.broadcast_lifecycle(ToolLifecycleEvent::CustomEvent {
            tool_id: self.tool_type().display_name().to_string(),
            event_type: event_type.to_string(),
            timestamp: Instant::now(),
            data: Some(format!("Analysis event: {}", event_type)),
        }).await {
            warn!("Failed to broadcast analysis event: {}", e);
        }
    }
}

/// Writing insight and recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingInsight {
    /// Insight type
    pub insight_type: InsightType,
    /// Insight description
    pub description: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Recommendation action
    pub recommendation: String,
    /// Data supporting the insight
    pub supporting_data: serde_json::Value,
    /// Timestamp when insight was generated
    pub generated_at: Instant,
}

/// Types of writing insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    /// Productivity pattern
    ProductivityPattern,
    /// Writing habit analysis
    WritingHabit,
    /// Goal achievement prediction
    GoalPrediction,
    /// Style consistency analysis
    StyleConsistency,
    /// Progress milestone
    ProgressMilestone,
    /// Risk factor identification
    RiskFactor,
}

/// Writing trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendData {
    /// Timestamp for this data point
    pub timestamp: Instant,
    /// Metric value
    pub value: f64,
    /// Metric type
    pub metric_type: String,
    /// Trend direction
    pub trend_direction: TrendDirection,
}

/// Trend direction indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    /// Increasing trend
    Upward,
    /// Decreasing trend
    Downward,
    /// Stable trend
    Stable,
    /// Volatile trend
    Variable,
}

/// Project comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectComparison {
    /// Projects being compared
    pub projects: Vec<String>,
    /// Comparison metrics
    pub metrics: std::collections::HashMap<String, ProjectMetric>,
    /// Overall ranking
    pub ranking: Vec<String>,
    /// Comparative insights
    pub insights: Vec<String>,
    /// Generated at timestamp
    pub generated_at: Instant,
}

/// Individual project metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetric {
    /// Project identifier
    pub project_id: String,
    /// Metric value
    pub value: f64,
    /// Metric type
    pub metric_type: String,
    /// Comparison to average
    pub comparison_to_average: f64,
    /// Percentile ranking
    pub percentile: f64,
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Analysis frequency settings
    pub analysis_frequency: AnalysisFrequency,
    /// Metrics to track
    pub tracked_metrics: Vec<String>,
    /// Privacy settings
    pub privacy_settings: PrivacySettings,
    /// Notification preferences
    pub notification_preferences: NotificationPreferences,
    /// Custom analysis rules
    pub custom_rules: Vec<AnalysisRule>,
}

/// Analysis frequency settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisFrequency {
    /// Real-time analysis
    RealTime,
    /// Daily analysis
    Daily,
    /// Weekly analysis
    Weekly,
    /// Manual only
    Manual,
}

/// Privacy settings for analysis data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Whether to collect detailed metrics
    pub collect_detailed_metrics: bool,
    /// Whether to share anonymized data
    pub share_anonymized_data: bool,
    /// Data retention period in days
    pub data_retention_days: u32,
    /// Export data on request
    pub export_on_request: bool,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Enable insight notifications
    pub enable_insight_notifications: bool,
    /// Enable milestone notifications
    pub enable_milestone_notifications: bool,
    /// Enable trend change notifications
    pub enable_trend_notifications: bool,
    /// Notification frequency
    pub notification_frequency: String,
}

/// Custom analysis rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule condition
    pub condition: String,
    /// Rule action
    pub action: String,
    /// Rule priority
    pub priority: u32,
    /// Whether rule is enabled
    pub enabled: bool,
}

/// Analysis statistics summary
#[derive(Debug, Clone)]
pub struct AnalysisStats {
    /// Total number of analyses performed
    pub total_analyses: u64,
    /// Average analysis time in milliseconds
    pub avg_analysis_time_ms: f64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Last analysis timestamp
    pub last_analysis_time: Option<Instant>,
    /// Cache hit rate percentage
    pub cache_hit_rate: f64,
    /// Whether cached data is available
    pub cached_data_available: bool,
    /// Size of analysis data in bytes
    pub analysis_data_size: usize,
}

#[async_trait]
impl ToolIntegration for MigratedAnalysisTool {
    /// Initialize the analysis tool with database context
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
        self.broadcast_analysis_event("initialized").await;
        
        info!("Analysis tool initialized successfully");
        Ok(())
    }

    /// Update tool state
    fn update(&mut self) -> Result<(), String> {
        // Perform any periodic updates
        // For analysis tool, this might include:
        // - Scheduled analysis runs
        // - Cache invalidation
        // - Metrics aggregation
        
        // Validate analysis data integrity
        self.validate_analysis_data_integrity()?;
        
        Ok(())
    }

    /// Render the tool UI
    fn render(&mut self) -> Result<(), String> {
        // This would typically render the analysis UI
        // For now, we'll just validate that the tool is in a renderable state
        
        if self.database_context.is_none() {
            return Err("Database context not initialized".to_string());
        }
        
        Ok(())
    }

    /// Cleanup tool resources
    async fn cleanup(&mut self) -> Result<(), String> {
        // Broadcast cleanup event
        self.broadcast_analysis_event("cleanup_started").await;
        
        // Clear cached data
        self.writing_stats_cache = None;
        
        // Unregister from tool registry
        let tool_id = format!("{}_{}", self.tool_type().display_name(), uuid::Uuid::new_v4());
        self.tool_registry.unregister_tool(&tool_id).await?;
        
        // Clear database context
        self.database_context = None;
        
        info!("Analysis tool cleanup completed");
        Ok(())
    }
}

impl MigratedAnalysisTool {
    /// Validate analysis data integrity
    fn validate_analysis_data_integrity(&self) -> Result<(), String> {
        // Check for invalid configurations
        if self.analysis_data.config.tracked_metrics.is_empty() {
            return Err("No tracked metrics configured".to_string());
        }
        
        // Validate custom rules syntax
        for rule in &self.analysis_data.config.custom_rules {
            if rule.name.trim().is_empty() {
                return Err(format!("Empty rule name in custom rule: {:?}", rule));
            }
            if rule.condition.trim().is_empty() {
                return Err(format!("Empty condition in custom rule: {}", rule.name));
            }
        }
        
        Ok(())
    }
}

impl Default for MigratedAnalysisTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolType for MigratedAnalysisTool {
    fn display_name(&self) -> &'static str {
        "Analysis Tool"
    }
    
    fn tool_id(&self) -> &'static str {
        "analysis_tool"
    }
    
    fn version(&self) -> &'static str {
        "2.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Writing analytics and insights tool with database integration"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseAppState;
    use tokio::sync::RwLock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_analysis_tool_initialization() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedAnalysisTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_analysis", database_state).await;
        let result = tool.initialize(&mut database_context).await;
        
        assert!(result.is_ok());
        assert!(tool.database_context.is_some());
        assert!(tool.initialized_at.is_some());
    }

    #[tokio::test]
    async fn test_analysis_config_save() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedAnalysisTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_analysis", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Create test configuration
        let config = AnalysisConfig {
            analysis_frequency: AnalysisFrequency::Daily,
            tracked_metrics: vec!["word_count".to_string(), "session_time".to_string()],
            privacy_settings: PrivacySettings {
                collect_detailed_metrics: true,
                share_anonymized_data: false,
                data_retention_days: 365,
                export_on_request: true,
            },
            notification_preferences: NotificationPreferences {
                enable_insight_notifications: true,
                enable_milestone_notifications: true,
                enable_trend_notifications: false,
                notification_frequency: "daily".to_string(),
            },
            custom_rules: vec![],
        };
        
        let result = tool.save_analysis_config(&config).await;
        assert!(result.is_success());
        assert_eq!(tool.analysis_data.config.tracked_metrics.len(), 2);
    }

    #[tokio::test]
    async fn test_analysis_stats() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MigratedAnalysisTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_analysis", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Simulate some analysis operations
        tool.update_analysis_metrics(true, Duration::from_millis(500));
        tool.update_analysis_metrics(true, Duration::from_millis(600));
        tool.update_analysis_metrics(false, Duration::from_millis(200));
        
        let stats = tool.get_analysis_stats();
        assert_eq!(stats.total_analyses, 3);
        assert!(stats.avg_analysis_time_ms > 0.0);
        assert_eq!(stats.success_rate, 66.66666666666666); // 2 out of 3 successful
        assert!(stats.analysis_data_size >= 0);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let mut tool = MigratedAnalysisTool::new();
        
        // Initially no cached data
        assert!(tool.get_cached_writing_stats().is_none());
        
        // Add some cached data
        let test_stats = WritingStats {
            total_words: 1000,
            total_chars: 5000,
            avg_words_per_day: 500.0,
            session_count: 5,
            total_writing_time: Duration::from_hours(5),
            most_productive_hour: 14, // 2 PM
            writing_streak_days: 7,
            project_breakdown: std::collections::HashMap::new(),
        };
        
        tool.writing_stats_cache = Some(test_stats.clone());
        
        // Verify cache access
        assert!(tool.get_cached_writing_stats().is_some());
        assert_eq!(tool.get_cached_writing_stats().unwrap().total_words, 1000);
        
        // Clear cache
        tool.clear_cache();
        assert!(tool.get_cached_writing_stats().is_none());
    }
}