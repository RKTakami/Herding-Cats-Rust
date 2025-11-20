//! Database Application State
//!
//! Core state management for database operations across the application.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

// Import ServiceContainer from service_factory
use crate::database::service_factory::ServiceContainer;

// Type aliases for service types
pub type EnhancedDatabaseService = crate::database::EnhancedDatabaseService;
pub type BackupService = crate::database::backup_service::BackupService;
pub type ProjectManagementService = crate::database::project_management::ProjectManagementService;

/// Core database application state
#[derive(Debug, Clone)]
pub struct DatabaseAppState {
    /// Database connection string
    pub connection_string: String,
    /// Database connection pool size
    pub pool_size: u32,
    /// Database connection timeout
    pub connection_timeout: Duration,
    /// Database operation timeout
    pub operation_timeout: Duration,
    /// Maximum retry attempts for failed operations
    pub max_retries: u32,
    /// Retry backoff duration
    pub retry_backoff: Duration,
    /// Database health status
    pub health_status: DatabaseHealthStatus,
    /// Last health check timestamp
    pub last_health_check: Option<Instant>,
    /// Total successful operations
    pub successful_operations: u64,
    /// Total failed operations
    pub failed_operations: u64,
    /// Total connection time
    pub total_connection_time: Duration,
    /// Average operation latency
    pub average_latency: Option<Duration>,
    /// Database version
    pub database_version: String,
    /// Migration version
    pub migration_version: u32,
    /// Configuration settings
    pub config: DatabaseConfig,
    /// Service container holding initialized services
    pub service_container: Option<ServiceContainer>,
}

/// Database health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DatabaseHealthStatus {
    /// Database is healthy and accepting connections
    Healthy,
    /// Database is experiencing issues
    Degraded,
    /// Database is unreachable
    Unhealthy,
    /// Database is starting up
    #[default]
    Starting,
    /// Database is shutting down
    ShuttingDown,
}

/// Database configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Enable query logging
    pub enable_query_logging: bool,
    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,
    /// Enable connection pooling
    pub enable_connection_pooling: bool,
    /// Maximum idle connections
    pub max_idle_connections: u32,
    /// Maximum lifetime for connections
    pub max_connection_lifetime: Option<Duration>,
    /// Minimum connections in pool
    pub min_connections: u32,
    /// Connection validation query
    pub validation_query: String,
    /// Enable SSL/TLS
    pub enable_ssl: bool,
    /// SSL certificate path
    pub ssl_cert_path: Option<String>,
    /// SSL key path
    pub ssl_key_path: Option<String>,
    /// SSL root certificate path
    pub ssl_root_cert_path: Option<String>,
    /// Connection retry strategy
    pub retry_strategy: RetryStrategy,
}

/// Database retry strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryStrategy {
    /// Exponential backoff retry
    ExponentialBackoff {
        base_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
    },
    /// Linear backoff retry
    LinearBackoff { delay: Duration, max_attempts: u32 },
    /// Fixed delay retry
    FixedDelay { delay: Duration, max_attempts: u32 },
}

/// Database operation statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Average operation latency
    pub average_latency: Option<Duration>,
    /// 95th percentile latency
    pub p95_latency: Option<Duration>,
    /// 99th percentile latency
    pub p99_latency: Option<Duration>,
    /// Maximum observed latency
    pub max_latency: Option<Duration>,
    /// Minimum observed latency
    pub min_latency: Option<Duration>,
    /// Operations per second
    pub ops_per_second: f64,
    /// Connection pool statistics
    pub pool_stats: Option<ConnectionPoolStats>,
    /// Error breakdown by type
    pub error_breakdown: std::collections::HashMap<String, u64>,
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct ConnectionPoolStats {
    /// Current number of connections
    pub current_connections: u32,
    /// Maximum number of connections
    pub max_connections: u32,
    /// Minimum number of connections
    pub min_connections: u32,
    /// Idle connections
    pub idle_connections: u32,
    /// Busy connections
    pub busy_connections: u32,
    /// Pending connection requests
    pub pending_requests: u32,
    /// Connection creation failures
    pub creation_failures: u64,
    /// Connection timeout count
    pub timeouts: u64,
}

impl Default for DatabaseAppState {
    fn default() -> Self {
        Self {
            connection_string: "sqlite://herding_cats.db".to_string(),
            pool_size: 10,
            connection_timeout: Duration::from_secs(30),
            operation_timeout: Duration::from_secs(60),
            max_retries: 3,
            retry_backoff: Duration::from_millis(100),
            health_status: DatabaseHealthStatus::Starting,
            last_health_check: None,
            successful_operations: 0,
            failed_operations: 0,
            total_connection_time: Duration::new(0, 0),
            average_latency: None,
            database_version: "1.0.0".to_string(),
            migration_version: 1,
            config: DatabaseConfig::default(),
            service_container: None,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            enable_query_logging: false,
            enable_performance_monitoring: true,
            enable_connection_pooling: true,
            max_idle_connections: 5,
            max_connection_lifetime: Some(Duration::from_secs(3600)), // 1 hour
            min_connections: 1,
            validation_query: "SELECT 1".to_string(),
            enable_ssl: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            ssl_root_cert_path: None,
            retry_strategy: RetryStrategy::ExponentialBackoff {
                base_delay: Duration::from_millis(100),
                max_delay: Duration::from_secs(5),
                multiplier: 2.0,
            },
        }
    }
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::ExponentialBackoff {
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            multiplier: 2.0,
        }
    }
}

impl DatabaseAppState {
    /// Create a new database application state
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new database application state with configuration
    pub fn with_config(config: DatabaseConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Create a new database application state with connection string
    pub fn with_connection_string(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            ..Default::default()
        }
    }

    /// Update health status
    pub fn update_health_status(&mut self, status: DatabaseHealthStatus) {
        self.health_status = status;
        self.last_health_check = Some(Instant::now());
    }

    /// Record a successful operation
    pub fn record_successful_operation(&mut self, duration: Duration) {
        self.successful_operations += 1;
        self.update_average_latency(duration);
    }

    /// Record a failed operation
    pub fn record_failed_operation(&mut self) {
        self.failed_operations += 1;
    }

    /// Update average latency
    fn update_average_latency(&mut self, duration: Duration) {
        let total_ops = self.successful_operations;
        if total_ops == 1 {
            self.average_latency = Some(duration);
        } else if let Some(current_avg) = self.average_latency {
            // Calculate new average: (old_avg * (n-1) + new_duration) / n
            let total_ops_f64 = total_ops as f64;
            let current_avg_f64 = current_avg.as_nanos() as f64;
            let duration_f64 = duration.as_nanos() as f64;
            let new_avg_ns =
                ((current_avg_f64 * (total_ops_f64 - 1.0) + duration_f64) / total_ops_f64) as u64;
            self.average_latency = Some(Duration::from_nanos(new_avg_ns));
        }
    }

    /// Get current health status
    pub fn get_health_status(&self) -> DatabaseHealthStatus {
        self.health_status.clone()
    }

    /// Check if database is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.health_status, DatabaseHealthStatus::Healthy)
    }

    /// Get success rate percentage
    pub fn get_success_rate(&self) -> f64 {
        let total_ops = self.successful_operations + self.failed_operations;
        if total_ops == 0 {
            0.0
        } else {
            (self.successful_operations as f64 / total_ops as f64) * 100.0
        }
    }

    /// Get operations per second
    pub fn get_ops_per_second(&self) -> f64 {
        let total_time = self.total_connection_time.as_secs_f64();
        if total_time == 0.0 {
            0.0
        } else {
            let total_ops = self.successful_operations + self.failed_operations;
            total_ops as f64 / total_time
        }
    }

    /// Update total connection time
    pub fn update_connection_time(&mut self, duration: Duration) {
        self.total_connection_time += duration;
    }

    /// Get database statistics
    pub fn get_stats(&self) -> DatabaseStats {
        DatabaseStats {
            total_operations: self.successful_operations + self.failed_operations,
            successful_operations: self.successful_operations,
            failed_operations: self.failed_operations,
            average_latency: self.average_latency,
            p95_latency: None, // Would need more detailed tracking
            p99_latency: None, // Would need more detailed tracking
            max_latency: None, // Would need more detailed tracking
            min_latency: None, // Would need more detailed tracking
            ops_per_second: self.get_ops_per_second(),
            pool_stats: None, // Would need connection pool integration
            error_breakdown: std::collections::HashMap::new(), // Would need error tracking
        }
    }

    /// Check if retry is available for operation
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_retries
    }

    /// Get retry delay for attempt
    pub fn get_retry_delay(&self, attempt: u32) -> Duration {
        match &self.config.retry_strategy {
            RetryStrategy::ExponentialBackoff {
                base_delay,
                max_delay,
                multiplier,
            } => {
                let base_delay_ms = base_delay.as_millis() as f64;
                let delay_ms = base_delay_ms * multiplier.powi(attempt as i32);
                let delay = Duration::from_millis(delay_ms as u64);
                delay.min(*max_delay)
            }
            RetryStrategy::LinearBackoff { delay, .. } => *delay,
            RetryStrategy::FixedDelay { delay, .. } => *delay,
        }
    }

    /// Validate configuration
    pub fn validate_config(&self) -> Result<()> {
        if self.connection_string.is_empty() {
            return Err(anyhow::anyhow!("Connection string cannot be empty"));
        }

        if self.pool_size == 0 {
            return Err(anyhow::anyhow!("Pool size must be greater than 0"));
        }

        if self.connection_timeout.as_secs() == 0 {
            return Err(anyhow::anyhow!("Connection timeout must be greater than 0"));
        }

        if self.operation_timeout.as_secs() == 0 {
            return Err(anyhow::anyhow!("Operation timeout must be greater than 0"));
        }

        if self.max_retries == 0 {
            return Err(anyhow::anyhow!("Max retries must be greater than 0"));
        }

        Ok(())
    }

    /// Get connection pool configuration
    pub fn get_pool_config(&self) -> PoolConfig {
        PoolConfig {
            min_connections: self.config.min_connections,
            max_connections: self.pool_size,
            max_idle_connections: self.config.max_idle_connections,
            connection_timeout: self.connection_timeout,
            idle_timeout: self.config.max_connection_lifetime,
        }
    }

    /// Get database service from service container
    pub fn database_service(&self) -> Option<Arc<RwLock<EnhancedDatabaseService>>> {
        self.service_container
            .as_ref()
            .and_then(|container| container.database_service.clone())
    }

    /// Get project service from service container
    pub fn project_service(&self) -> Option<Arc<RwLock<ProjectManagementService>>> {
        self.service_container
            .as_ref()
            .and_then(|container| container.project_service.clone())
    }

    /// Get backup service from service container
    pub fn backup_service(&self) -> Option<Arc<RwLock<BackupService>>> {
        self.service_container
            .as_ref()
            .and_then(|container| container.backup_service.clone())
    }

    /// Get current project ID
    pub fn get_current_project(&self) -> Option<&String> {
        // Check if project service is available and get current project
        self.service_container.as_ref().and({
            // For now, return None as project management is not fully implemented
            // This would typically query the project service for the current project
            None
        })
    }

    /// Check if database is ready
    pub fn is_database_ready(&self) -> bool {
        self.service_container
            .as_ref()
            .map(|container| container.is_healthy())
            .unwrap_or(false)
    }

    /// Set the service container (for initialization)
    pub fn set_service_container(&mut self, container: ServiceContainer) {
        self.service_container = Some(container);
    }
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Minimum number of connections
    pub min_connections: u32,
    /// Maximum number of connections
    pub max_connections: u32,
    /// Maximum idle connections
    pub max_idle_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout for connections
    pub idle_timeout: Option<Duration>,
}

/// Database operation result wrapper
#[derive(Debug, Clone)]
pub struct DatabaseOperationResult<T> {
    /// Whether the operation was successful
    pub success: bool,
    /// The result data if successful
    pub data: Option<T>,
    /// Error message if failed
    pub error: Option<String>,
    /// Operation duration
    pub duration: Duration,
    /// Retry count used
    pub retry_count: u32,
    /// Timestamp when operation completed
    pub completed_at: Instant,
}

impl<T> DatabaseOperationResult<T> {
    /// Create a successful result
    pub fn success(data: T, duration: Duration) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            duration,
            retry_count: 0,
            completed_at: Instant::now(),
        }
    }

    /// Create a failed result
    pub fn failure(error: String, duration: Duration) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            duration,
            retry_count: 0,
            completed_at: Instant::now(),
        }
    }

    /// Check if operation was successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Get result data or panic with error message
    pub fn unwrap(self) -> T {
        match self.data {
            Some(data) => data,
            None => panic!("Called unwrap on failed operation: {:?}", self.error),
        }
    }

    /// Get result data or return default
    pub fn unwrap_or(self, default: T) -> T {
        self.data.unwrap_or(default)
    }

    /// Get result data or compute default
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce(&str) -> T,
    {
        match self.data {
            Some(data) => data,
            None => f(self.error.as_ref().unwrap_or(&"Unknown error".to_string())),
        }
    }

    /// Convert to option
    pub fn ok(self) -> Option<T> {
        self.data
    }

    /// Get error message if failed
    pub fn err(self) -> Option<String> {
        match self.success {
            true => None,
            false => self.error,
        }
    }
}

/// Database connection info
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Connection ID
    pub id: String,
    /// Connection state
    pub state: ConnectionState,
    /// Last used timestamp (not serialized)
    pub last_used: Option<Instant>,
    /// Connection creation timestamp (not serialized)
    pub created_at: Instant,
    /// Number of operations performed
    pub operation_count: u64,
    /// Total time spent on operations
    pub total_operation_time: Duration,
    /// Connection error count
    pub error_count: u64,
}

/// Database connection state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ConnectionState {
    /// Connection is available for use
    Idle,
    /// Connection is currently in use
    Busy,
    /// Connection is being established
    #[default]
    Connecting,
    /// Connection is being closed
    Closing,
    /// Connection has encountered an error
    Errored,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_database_app_state_creation() {
        let state = DatabaseAppState::new();
        assert_eq!(state.connection_string, "sqlite://herding_cats.db");
        assert_eq!(state.pool_size, 10);
        assert_eq!(state.health_status, DatabaseHealthStatus::Starting);
        assert_eq!(state.successful_operations, 0);
        assert_eq!(state.failed_operations, 0);
    }

    #[test]
    fn test_database_app_state_with_config() {
        let config = DatabaseConfig {
            enable_query_logging: true,
            enable_performance_monitoring: false,
            ..Default::default()
        };
        let state = DatabaseAppState::with_config(config);
        assert!(state.config.enable_query_logging);
        assert!(!state.config.enable_performance_monitoring);
    }

    #[test]
    fn test_health_status_update() {
        let mut state = DatabaseAppState::new();
        state.update_health_status(DatabaseHealthStatus::Healthy);
        assert_eq!(state.health_status, DatabaseHealthStatus::Healthy);
        assert!(state.last_health_check.is_some());
    }

    #[test]
    fn test_operation_recording() {
        let mut state = DatabaseAppState::new();
        let duration = Duration::from_millis(100);

        state.record_successful_operation(duration);
        assert_eq!(state.successful_operations, 1);
        assert!(state.average_latency.is_some());

        state.record_failed_operation();
        assert_eq!(state.failed_operations, 1);
    }

    #[test]
    fn test_success_rate_calculation() {
        let mut state = DatabaseAppState::new();

        // No operations yet
        assert_eq!(state.get_success_rate(), 0.0);

        // One successful operation
        state.record_successful_operation(Duration::from_millis(100));
        assert_eq!(state.get_success_rate(), 100.0);

        // One failed operation
        state.record_failed_operation();
        assert_eq!(state.get_success_rate(), 50.0);
    }

    #[test]
    fn test_database_operation_result_success() {
        let data = "test_result".to_string();
        let duration = Duration::from_millis(50);
        let result = DatabaseOperationResult::success(data.clone(), duration);

        assert!(result.is_success());
        assert_eq!(result.data.unwrap(), data);
        assert!(result.error.is_none());
        assert_eq!(result.duration, duration);
    }

    #[test]
    fn test_database_operation_result_failure() {
        let error = "Connection failed".to_string();
        let duration = Duration::from_millis(100);
        let result = DatabaseOperationResult::<String>::failure(error.clone(), duration);

        assert!(!result.is_success());
        assert!(result.data.is_none());
        assert_eq!(result.error.unwrap(), error);
        assert_eq!(result.duration, duration);
    }

    #[test]
    fn test_retry_strategy() {
        let state = DatabaseAppState::new();

        // Should retry for attempts less than max_retries
        assert!(state.should_retry(0));
        assert!(state.should_retry(2));
        assert!(!state.should_retry(3)); // max_retries is 3, so attempt 3 should not retry

        // Get retry delay
        let delay1 = state.get_retry_delay(0);
        let delay2 = state.get_retry_delay(1);
        assert!(delay2 >= delay1); // Exponential backoff should increase delay
    }

    #[test]
    fn test_connection_info_creation() {
        let connection_info = ConnectionInfo {
            id: "conn_123".to_string(),
            state: ConnectionState::Idle,
            last_used: None,
            created_at: Instant::now(),
            operation_count: 0,
            total_operation_time: Duration::new(0, 0),
            error_count: 0,
        };

        assert_eq!(connection_info.id, "conn_123");
        assert_eq!(connection_info.state, ConnectionState::Idle);
        assert_eq!(connection_info.operation_count, 0);
    }
}
