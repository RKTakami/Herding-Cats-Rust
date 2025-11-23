//! UI Tools Base Module
//!
//! Provides the foundational base classes and interfaces for all UI tools.

use crate::ui::tools::{
    api_contracts::{ToolApiContract, ToolLifecycleEvent},
    base_types::ToolType,
    database_integration::ToolDatabaseContext,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Base trait for all tools in the system
#[async_trait::async_trait]
pub trait ToolIntegration {
    /// Initialize the tool with database context
    async fn initialize(
        &mut self,
        database_context: &mut ToolDatabaseContext,
    ) -> Result<(), String>;

    /// Update the tool (called every frame/update cycle)
    fn update(&mut self) -> Result<(), String>;

    /// Render the tool UI
    fn render(&mut self) -> Result<(), String>;

    /// Cleanup resources
    async fn cleanup(&mut self) -> Result<(), String>;

    /// Get the tool type
    fn tool_type(&self) -> ToolType;

    /// Get the tool's display name
    fn display_name(&self) -> &str;

    /// Check if the tool is initialized
    fn is_initialized(&self) -> bool;

    /// Get initialization timestamp
    fn initialized_at(&self) -> Option<Instant>;
}

/// Base structure for all tools
#[derive(Debug, Clone)]
pub struct ToolBase {
    /// Tool type identifier
    pub tool_type: ToolType,
    /// Tool display name
    pub display_name: String,
    /// Whether the tool is initialized
    pub initialized: bool,
    /// Timestamp when tool was initialized
    pub initialized_at: Option<Instant>,
    /// Database context for tool operations
    pub database_context: Option<ToolDatabaseContext>,
    /// API contract for tool management
    pub api_contract: Arc<ToolApiContract>,
    /// Event handlers for tool events
    pub event_handlers: Vec<ToolEventHandler>,
    /// Tool configuration
    pub configuration: ToolConfiguration,
    /// Tool statistics
    pub statistics: ToolStatistics,
    /// Tool health status
    pub health_status: ToolHealthStatus,
    /// Last health check
    pub last_health_check: Option<Instant>,
}

/// Tool event handler
#[derive(Debug, Clone)]
pub struct ToolEventHandler {
    /// Tool identifier for this handler
    pub tool_id: String,
    /// Handler function identifier
    pub handler_id: String,
    /// Whether the handler is enabled
    pub enabled: bool,
    /// Handler priority (lower numbers execute first)
    pub priority: u32,
    /// Handler function (represented as a string identifier for serialization)
    pub handler_function: String,
}

/// Tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfiguration {
    /// Whether the tool is enabled
    pub enabled: bool,
    /// Tool-specific settings
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    /// Resource limits for the tool
    pub resource_limits: ResourceLimits,
    /// Performance tuning parameters
    pub performance_params: PerformanceParameters,
    /// Debug settings
    pub debug_settings: DebugSettings,
}

/// Resource limits for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,
    /// Maximum database connections
    pub max_db_connections: u32,
    /// Maximum concurrent operations
    pub max_concurrent_operations: u32,
    /// Maximum operation timeout in seconds
    pub max_operation_timeout: u64,
    /// Maximum log size in MB
    pub max_log_size_mb: u64,
}

/// Performance tuning parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceParameters {
    /// Cache size for frequently accessed data
    pub cache_size: usize,
    /// Batch operation size
    pub batch_size: usize,
    /// Polling interval for updates in milliseconds
    pub polling_interval_ms: u64,
    /// Maximum retry attempts for failed operations
    pub max_retries: u32,
    /// Retry backoff multiplier
    pub retry_backoff: f64,
    /// Performance monitoring enabled
    pub monitoring_enabled: bool,
}

/// Debug settings for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSettings {
    /// Enable verbose logging
    pub verbose_logging: bool,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Log level filter
    pub log_level: String,
    /// Performance threshold alerts
    pub performance_alerts: bool,
    /// Debug UI enabled
    pub debug_ui_enabled: bool,
    /// Mock data enabled for testing
    pub mock_data_enabled: bool,
}

/// Tool statistics
#[derive(Debug, Clone)]
pub struct ToolStatistics {
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
    /// Total operation time
    pub total_operation_time: Duration,
    /// Average operation time
    pub average_operation_time: Option<Duration>,
    /// Peak memory usage
    pub peak_memory_usage: u64,
    /// Current memory usage
    pub current_memory_usage: u64,
    /// Number of database operations
    pub database_operations: u64,
    /// Number of API calls
    pub api_calls: u64,
    /// Number of events processed
    pub events_processed: u64,
    /// Last reset timestamp
    pub last_reset: Option<Instant>,
}

/// Tool health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ToolHealthStatus {
    /// Tool is healthy and functioning normally
    Healthy,
    /// Tool is experiencing minor issues
    Degraded,
    /// Tool is experiencing major issues
    Unhealthy,
    /// Tool is starting up
    #[default]
    Starting,
    /// Tool is shutting down
    ShuttingDown,
    /// Tool has encountered a critical error
    Critical,
}

impl ToolBase {
    /// Create a new tool base
    pub fn new(
        tool_type: ToolType,
        display_name: &str,
        api_contract: Arc<ToolApiContract>,
    ) -> Self {
        Self {
            tool_type,
            display_name: display_name.to_string(),
            initialized: false,
            initialized_at: None,
            database_context: None,
            api_contract,
            event_handlers: Vec::new(),
            configuration: ToolConfiguration::default(),
            statistics: ToolStatistics::default(),
            health_status: ToolHealthStatus::Starting,
            last_health_check: None,
        }
    }

    /// Initialize the tool
    pub async fn initialize_tool(
        &mut self,
        database_context: ToolDatabaseContext,
    ) -> Result<(), String> {
        self.database_context = Some(database_context);
        self.initialized = true;
        self.initialized_at = Some(Instant::now());

        // Register with API contract
        self.api_contract
            .register_tool(format!(
                "{}_{}",
                self.tool_type.display_name(),
                self.display_name
            ))
            .await
            .map_err(|e| format!("Failed to register tool: {}", e))?;

        // Broadcast initialization event
        self.api_contract
            .broadcast_lifecycle(ToolLifecycleEvent::Registered {
                tool_id: format!("{}_{}", self.tool_type.display_name(), self.display_name),
                tool_type: self.tool_type.display_name().to_string(),
            })
            .await
            .map_err(|e| format!("Failed to broadcast lifecycle event: {}", e))?;

        self.health_status = ToolHealthStatus::Healthy;
        self.last_health_check = Some(Instant::now());

        Ok(())
    }

    /// Update tool statistics
    pub fn update_statistics(&mut self, operation_duration: Duration, success: bool) {
        if success {
            self.statistics.successful_operations += 1;
        } else {
            self.statistics.failed_operations += 1;
        }

        self.statistics.total_operation_time += operation_duration;

        // Update average operation time
        let total_ops = self.statistics.successful_operations + self.statistics.failed_operations;
        if total_ops > 0 {
            let avg_ns = self.statistics.total_operation_time.as_nanos() / total_ops as u128;
            self.statistics.average_operation_time = Some(Duration::from_nanos(avg_ns as u64));
        }
    }

    /// Update memory usage statistics
    pub fn update_memory_usage(&mut self, memory_usage: u64) {
        self.statistics.current_memory_usage = memory_usage;
        if memory_usage > self.statistics.peak_memory_usage {
            self.statistics.peak_memory_usage = memory_usage;
        }
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = ToolStatistics::default();
        self.statistics.last_reset = Some(Instant::now());
    }

    /// Check tool health
    pub async fn check_health(&mut self) -> Result<ToolHealthStatus, String> {
        // Check database connectivity
        if let Some(ref mut db_context) = self.database_context {
            let health_check_result = db_context
                .execute_with_retry(
                    "health_check",
                    |service| {
                        Box::pin(async move {
                            let conn = service.read().await;
                            conn.test_connection().await
                        })
                    },
                    3,
                )
                .await;

            // Check if the operation was successful
            match health_check_result {
                Ok(_) => {
                    // Health check passed
                }
                Err(_) => {
                    self.health_status = ToolHealthStatus::Unhealthy;
                    return Ok(ToolHealthStatus::Unhealthy);
                }
            }
        }

        // Check API contract connectivity
        let registered_tools = self.api_contract.get_registered_tools().await;
        if !registered_tools
            .iter()
            .any(|tool| tool.contains(&self.display_name))
        {
            self.health_status = ToolHealthStatus::Degraded;
            return Ok(ToolHealthStatus::Degraded);
        }

        // Check for excessive failures
        let total_ops = self.statistics.successful_operations + self.statistics.failed_operations;
        if total_ops > 0 {
            let failure_rate = self.statistics.failed_operations as f64 / total_ops as f64;
            if failure_rate > 0.1 {
                // More than 10% failure rate
                if failure_rate > 0.25 {
                    // More than 25% failure rate
                    self.health_status = ToolHealthStatus::Critical;
                } else {
                    self.health_status = ToolHealthStatus::Unhealthy;
                }
                return Ok(self.health_status.clone());
            }
        }

        self.health_status = ToolHealthStatus::Healthy;
        self.last_health_check = Some(Instant::now());
        Ok(ToolHealthStatus::Healthy)
    }

    /// Get success rate
    pub fn get_success_rate(&self) -> f64 {
        let total_ops = self.statistics.successful_operations + self.statistics.failed_operations;
        if total_ops == 0 {
            100.0 // No operations yet, assume perfect
        } else {
            (self.statistics.successful_operations as f64 / total_ops as f64) * 100.0
        }
    }

    /// Get operations per second
    pub fn get_ops_per_second(&self) -> f64 {
        let total_time = self.statistics.total_operation_time.as_secs_f64();
        if total_time == 0.0 {
            0.0
        } else {
            let total_ops =
                self.statistics.successful_operations + self.statistics.failed_operations;
            total_ops as f64 / total_time
        }
    }

    /// Check if tool should be restarted based on health
    pub fn should_restart(&self) -> bool {
        match self.health_status {
            ToolHealthStatus::Critical => true,
            ToolHealthStatus::Unhealthy => {
                // Check if unhealthy for too long
                if let Some(last_check) = self.last_health_check {
                    last_check.elapsed().as_secs() > 300 // 5 minutes
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl Default for ToolConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            settings: std::collections::HashMap::new(),
            resource_limits: ResourceLimits::default(),
            performance_params: PerformanceParameters::default(),
            debug_settings: DebugSettings::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_cpu_percent: 80.0,
            max_db_connections: 10,
            max_concurrent_operations: 5,
            max_operation_timeout: 30,
            max_log_size_mb: 100,
        }
    }
}

impl Default for PerformanceParameters {
    fn default() -> Self {
        Self {
            cache_size: 100,
            batch_size: 10,
            polling_interval_ms: 1000,
            max_retries: 3,
            retry_backoff: 2.0,
            monitoring_enabled: true,
        }
    }
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            verbose_logging: false,
            enable_profiling: false,
            log_level: "INFO".to_string(),
            performance_alerts: true,
            debug_ui_enabled: false,
            mock_data_enabled: false,
        }
    }
}

impl Default for ToolStatistics {
    fn default() -> Self {
        Self {
            successful_operations: 0,
            failed_operations: 0,
            total_operation_time: Duration::new(0, 0),
            average_operation_time: None,
            peak_memory_usage: 0,
            current_memory_usage: 0,
            database_operations: 0,
            api_calls: 0,
            events_processed: 0,
            last_reset: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_tool_base_creation() {
        let api_contract = Arc::new(ToolApiContract::new());
        let tool_base = ToolBase::new(ToolType::Hierarchy, "Test Tool", api_contract);

        assert_eq!(tool_base.tool_type, ToolType::Hierarchy);
        assert_eq!(tool_base.display_name, "Test Tool");
        assert!(!tool_base.initialized);
        assert!(tool_base.initialized_at.is_none());
        assert!(tool_base.database_context.is_none());
    }

    #[tokio::test]
    async fn test_tool_base_initialization() {
        let api_contract = Arc::new(ToolApiContract::new());
        let database_state = Arc::new(RwLock::new(crate::DatabaseAppState::new()));
        let database_context = ToolDatabaseContext::new("test_tool", database_state).await;

        let mut tool_base = ToolBase::new(ToolType::Hierarchy, "Test Tool", api_contract);
        let result = tool_base.initialize_tool(database_context).await;

        assert!(result.is_ok());
        assert!(tool_base.initialized);
        assert!(tool_base.initialized_at.is_some());
        assert!(tool_base.database_context.is_some());
        assert_eq!(tool_base.health_status, ToolHealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_tool_statistics() {
        let api_contract = Arc::new(ToolApiContract::new());
        let mut tool_base = ToolBase::new(ToolType::Hierarchy, "Test Tool", api_contract);

        // Test statistics update
        let duration = Duration::from_millis(100);
        tool_base.update_statistics(duration, true);

        assert_eq!(tool_base.statistics.successful_operations, 1);
        assert_eq!(tool_base.statistics.failed_operations, 0);
        assert_eq!(tool_base.statistics.total_operation_time, duration);
        assert!(tool_base.statistics.average_operation_time.is_some());

        // Test memory usage update
        tool_base.update_memory_usage(1024);
        assert_eq!(tool_base.statistics.current_memory_usage, 1024);
        assert_eq!(tool_base.statistics.peak_memory_usage, 1024);

        // Test success rate
        assert_eq!(tool_base.get_success_rate(), 100.0);
    }

    #[tokio::test]
    async fn test_tool_health_check() {
        use tempfile::NamedTempFile;
        use crate::database::{DatabaseConfig, EnhancedDatabaseService, ProjectManagementService};
        use crate::database::service_factory::ServiceContainer;
        use crate::database_app_state::{DatabaseAppState, DatabaseHealthStatus};
        
        // Create temp database
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();
        let config = DatabaseConfig::default();
        
        // Create service
        let db_service = EnhancedDatabaseService::new(&db_path, config).await.unwrap();
        let db_service = Arc::new(RwLock::new(db_service));
        
        // Create project service
        let project_service = ProjectManagementService::new(db_service.clone());
        let project_service = Arc::new(RwLock::new(project_service));
        
        // Create container
        let mut container = ServiceContainer::new();
        container.database_service = Some(db_service.clone());
        container.project_service = Some(project_service);
        container.initialized = true;
        
        // Create app state
        let mut app_state = DatabaseAppState::new();
        app_state.set_service_container(container);
        app_state.update_health_status(DatabaseHealthStatus::Healthy);
        
        let database_state = Arc::new(RwLock::new(app_state));
        let database_context = ToolDatabaseContext::new("test_tool", database_state).await;

        let api_contract = Arc::new(ToolApiContract::new());
        let mut tool_base = ToolBase::new(ToolType::Hierarchy, "Test Tool", api_contract);
        tool_base.initialize_tool(database_context).await.unwrap();

        let health = tool_base.check_health().await.unwrap();
        assert!(matches!(health, ToolHealthStatus::Healthy));
        assert!(tool_base.last_health_check.is_some());
    }
}
