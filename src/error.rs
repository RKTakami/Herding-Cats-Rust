//! Comprehensive Error Handling System
//!
//! Provides structured error types and handling for the entire Herding Cats Rust application.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

/// Core database error types
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseError {
    #[error("Database connection failed: {message}")]
    ConnectionFailed { message: String },

    #[error("Database operation timed out after {timeout:?}")]
    Timeout { timeout: Duration },

    #[error("Database query failed: {query}, error: {error}")]
    QueryFailed { query: String, error: String },

    #[error("Database constraint violation: {constraint}, value: {value}")]
    ConstraintViolation { constraint: String, value: String },

    #[error("Database record not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Database record already exists: {entity} with id {id}")]
    AlreadyExists { entity: String, id: String },

    #[error("Database schema migration failed: {message}")]
    MigrationFailed { message: String },

    #[error("Database transaction failed: {message}")]
    TransactionFailed { message: String },

    #[error("Database pool exhausted: {message}")]
    PoolExhausted { message: String },

    #[error("Database configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Database integrity check failed: {message}")]
    IntegrityError { message: String },

    #[error("Database permission denied: {operation} on {resource}")]
    PermissionDenied { operation: String, resource: String },

    #[error("Database serialization error: {message}")]
    SerializationError { message: String },

    #[error("Database deserialization error: {message}")]
    DeserializationError { message: String },

    #[error("Database deadlock detected: {message}")]
    Deadlock { message: String },

    #[error("Database is in an invalid state: {state}")]
    InvalidState { state: String },

    #[error("Database backup/restore failed: {operation}, error: {error}")]
    BackupRestoreFailed { operation: String, error: String },

    #[error("Database health check failed: {message}")]
    HealthCheckFailed { message: String },

    #[error("Database version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },

    #[error("Unknown database error: {message}")]
    Unknown { message: String },

    // Additional variants needed by database modules
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Integrity check failed: {0}")]
    IntegrityCheck(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Core application error types
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    #[error("Tool initialization failed: {tool}, error: {error}")]
    ToolInitializationFailed { tool: String, error: String },

    #[error("Tool registration failed: {tool}, error: {error}")]
    ToolRegistrationFailed { tool: String, error: String },

    #[error("Tool not found: {tool}")]
    ToolNotFound { tool: String },

    #[error("Tool execution failed: {tool}, error: {error}")]
    ToolExecutionFailed { tool: String, error: String },

    #[error("Tool configuration error: {tool}, setting: {setting}, error: {error}")]
    ToolConfigurationError {
        tool: String,
        setting: String,
        error: String,
    },

    #[error("Tool lifecycle error: {tool}, phase: {phase}, error: {error}")]
    ToolLifecycleError {
        tool: String,
        phase: String,
        error: String,
    },

    #[error("Tool state corruption: {tool}, error: {error}")]
    ToolStateCorruption { tool: String, error: String },

    #[error("Tool dependency missing: {tool} requires {dependency}")]
    ToolDependencyMissing { tool: String, dependency: String },

    #[error("Tool version incompatibility: {tool}, expected: {expected}, got: {actual}")]
    ToolVersionIncompatibility {
        tool: String,
        expected: String,
        actual: String,
    },

    #[error("Tool permission error: {tool}, operation: {operation}, error: {error}")]
    ToolPermissionError {
        tool: String,
        operation: String,
        error: String,
    },

    #[error("Tool resource limit exceeded: {tool}, resource: {resource}, limit: {limit}")]
    ToolResourceLimitExceeded {
        tool: String,
        resource: String,
        limit: String,
    },

    #[error("Tool timeout: {tool}, timeout: {timeout:?}")]
    ToolTimeout { tool: String, timeout: Duration },

    #[error("Tool state invalid: {tool}, state: {state}")]
    ToolStateInvalid { tool: String, state: String },

    #[error("Tool data validation failed: {tool}, field: {field}, error: {error}")]
    ToolDataValidationFailed {
        tool: String,
        field: String,
        error: String,
    },

    #[error("Tool migration failed: {tool}, phase: {phase}, error: {error}")]
    ToolMigrationFailed {
        tool: String,
        phase: String,
        error: String,
    },

    #[error("Font error: {0}")]
    FontError(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Network error: {0}")]
    Network(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

/// Threading and concurrency error types
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ThreadingError {
    #[error("Thread pool exhausted: {message}")]
    ThreadPoolExhausted { message: String },

    #[error("Deadlock detected: {resource}, threads: {threads:?}")]
    Deadlock {
        resource: String,
        threads: Vec<String>,
    },

    #[error("Race condition detected: {operation}, resource: {resource}")]
    RaceCondition { operation: String, resource: String },

    #[error("Mutex lock failed: {resource}, error: {error}")]
    MutexLockFailed { resource: String, error: String },

    #[error("RwLock read failed: {resource}, error: {error}")]
    RwLockReadFailed { resource: String, error: String },

    #[error("RwLock write failed: {resource}, error: {error}")]
    RwLockWriteFailed { resource: String, error: String },

    #[error("Channel send failed: {channel}, error: {error}")]
    ChannelSendFailed { channel: String, error: String },

    #[error("Channel receive failed: {channel}, error: {error}")]
    ChannelReceiveFailed { channel: String, error: String },

    #[error("Task spawn failed: {task}, error: {error}")]
    TaskSpawnFailed { task: String, error: String },

    #[error("Task cancelled: {task}, reason: {reason}")]
    TaskCancelled { task: String, reason: String },

    #[error("Async operation timeout: {operation}, timeout: {timeout:?}")]
    AsyncTimeout {
        operation: String,
        timeout: Duration,
    },

    #[error("Future execution failed: {future}, error: {error}")]
    FutureExecutionFailed { future: String, error: String },

    #[error("Shared state corruption: {state}, error: {error}")]
    SharedStateCorruption { state: String, error: String },

    #[error("Atomic operation failed: {operation}, error: {error}")]
    AtomicOperationFailed { operation: String, error: String },

    #[error("Condition variable wait failed: {condition}, error: {error}")]
    ConditionVariableWaitFailed { condition: String, error: String },

    #[error("Semaphore acquire failed: {semaphore}, error: {error}")]
    SemaphoreAcquireFailed { semaphore: String, error: String },

    #[error("Barrier wait failed: {barrier}, error: {error}")]
    BarrierWaitFailed { barrier: String, error: String },
}

/// API and communication error types
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    #[error("API endpoint not found: {endpoint}")]
    EndpointNotFound { endpoint: String },

    #[error("API method not allowed: {method} for {endpoint}")]
    MethodNotAllowed { method: String, endpoint: String },

    #[error("API validation failed: {field}, error: {error}")]
    ValidationFailed { field: String, error: String },

    #[error("API authentication failed: {reason}")]
    AuthenticationFailed { reason: String },

    #[error("API authorization failed: {operation}, resource: {resource}")]
    AuthorizationFailed { operation: String, resource: String },

    #[error("API rate limit exceeded: {limit}, retry_after: {retry_after:?}")]
    RateLimitExceeded {
        limit: String,
        retry_after: Duration,
    },

    #[error("API serialization failed: {data}, error: {error}")]
    SerializationFailed { data: String, error: String },

    #[error("API deserialization failed: {data}, error: {error}")]
    DeserializationFailed { data: String, error: String },

    #[error("API request timeout: {endpoint}, timeout: {timeout:?}")]
    RequestTimeout { endpoint: String, timeout: Duration },

    #[error("API server error: {status}, error: {error}")]
    ServerError { status: u16, error: String },

    #[error("API client error: {status}, error: {error}")]
    ClientError { status: u16, error: String },

    #[error("API network error: {operation}, error: {error}")]
    NetworkError { operation: String, error: String },

    #[error("API payload too large: {size}, limit: {limit}")]
    PayloadTooLarge { size: String, limit: String },

    #[error("API content type not supported: {content_type}")]
    ContentTypeNotSupported { content_type: String },

    #[error("API version not supported: {version}")]
    VersionNotSupported { version: String },

    #[error("API event bus error: {operation}, error: {error}")]
    EventBusError { operation: String, error: String },

    #[error("API contract violation: {contract}, violation: {violation}")]
    ContractViolation { contract: String, violation: String },

    #[error("API configuration error: {config}, error: {error}")]
    ConfigurationError { config: String, error: String },

    #[error("API middleware error: {middleware}, error: {error}")]
    MiddlewareError { middleware: String, error: String },

    #[error("API unknown error: {message}")]
    Unknown { message: String },
}

/// Performance and monitoring error types
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceError {
    #[error("Performance monitoring failed: {metric}, error: {error}")]
    MonitoringFailed { metric: String, error: String },

    #[error("Performance metric collection failed: {collector}, error: {error}")]
    MetricCollectionFailed { collector: String, error: String },

    #[error("Performance threshold exceeded: {metric}, value: {value}, threshold: {threshold}")]
    ThresholdExceeded {
        metric: String,
        value: f64,
        threshold: f64,
    },

    #[error("Performance baseline establishment failed: {baseline}, error: {error}")]
    BaselineFailed { baseline: String, error: String },

    #[error("Performance optimization failed: {optimization}, error: {error}")]
    OptimizationFailed { optimization: String, error: String },

    #[error("Performance profiling failed: {profiler}, error: {error}")]
    ProfilingFailed { profiler: String, error: String },

    #[error("Performance alert failed: {alert}, error: {error}")]
    AlertFailed { alert: String, error: String },

    #[error("Performance dashboard update failed: {dashboard}, error: {error}")]
    DashboardUpdateFailed { dashboard: String, error: String },

    #[error("Performance log collection failed: {logger}, error: {error}")]
    LogCollectionFailed { logger: String, error: String },

    #[error("Performance report generation failed: {report}, error: {error}")]
    ReportGenerationFailed { report: String, error: String },

    #[error("Performance data aggregation failed: {aggregator}, error: {error}")]
    DataAggregationFailed { aggregator: String, error: String },

    #[error("Performance cache operation failed: {operation}, error: {error}")]
    CacheOperationFailed { operation: String, error: String },

    #[error("Performance sampling failed: {sampler}, error: {error}")]
    SamplingFailed { sampler: String, error: String },

    #[error("Performance trend analysis failed: {analyzer}, error: {error}")]
    TrendAnalysisFailed { analyzer: String, error: String },

    #[error("Performance prediction failed: {predictor}, error: {error}")]
    PredictionFailed { predictor: String, error: String },

    #[error("Performance benchmark failed: {benchmark}, error: {error}")]
    BenchmarkFailed { benchmark: String, error: String },

    #[error(
        "Performance regression detected: {metric}, old_value: {old_value}, new_value: {new_value}"
    )]
    RegressionDetected {
        metric: String,
        old_value: f64,
        new_value: f64,
    },

    #[error(
        "Performance anomaly detected: {metric}, value: {value}, expected_range: {expected_range}"
    )]
    AnomalyDetected {
        metric: String,
        value: f64,
        expected_range: String,
    },
}

/// Migration-specific error types
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MigrationError {
    #[error("Migration analysis failed: {phase}, error: {error}")]
    AnalysisFailed { phase: String, error: String },

    #[error("Migration plan generation failed: {planner}, error: {error}")]
    PlanGenerationFailed { planner: String, error: String },

    #[error("Migration execution failed: {phase}, error: {error}")]
    ExecutionFailed { phase: String, error: String },

    #[error("Migration validation failed: {validator}, error: {error}")]
    ValidationFailed { validator: String, error: String },

    #[error("Migration rollback failed: {phase}, error: {error}")]
    RollbackFailed { phase: String, error: String },

    #[error("Migration compatibility check failed: {component}, error: {error}")]
    CompatibilityCheckFailed { component: String, error: String },

    #[error("Migration dependency resolution failed: {dependency}, error: {error}")]
    DependencyResolutionFailed { dependency: String, error: String },

    #[error("Migration configuration error: {config}, error: {error}")]
    ConfigurationError { config: String, error: String },

    #[error("Migration state corruption: {state}, error: {error}")]
    StateCorruption { state: String, error: String },

    #[error("Migration progress tracking failed: {tracker}, error: {error}")]
    ProgressTrackingFailed { tracker: String, error: String },

    #[error("Migration resource allocation failed: {resource}, error: {error}")]
    ResourceAllocationFailed { resource: String, error: String },

    #[error("Migration tool integration failed: {tool}, error: {error}")]
    ToolIntegrationFailed { tool: String, error: String },

    #[error("Migration database integration failed: {operation}, error: {error}")]
    DatabaseIntegrationFailed { operation: String, error: String },

    #[error("Migration threading integration failed: {operation}, error: {error}")]
    ThreadingIntegrationFailed { operation: String, error: String },

    #[error("Migration API contract integration failed: {operation}, error: {error}")]
    ApiContractIntegrationFailed { operation: String, error: String },

    #[error("Migration testing failed: {test}, error: {error}")]
    TestingFailed { test: String, error: String },

    #[error("Migration documentation generation failed: {document}, error: {error}")]
    DocumentationFailed { document: String, error: String },

    #[error("Migration deployment failed: {deployment}, error: {error}")]
    DeploymentFailed { deployment: String, error: String },

    #[error("Migration cleanup failed: {cleanup}, error: {error}")]
    CleanupFailed { cleanup: String, error: String },

    #[error("Migration unknown error: {message}")]
    Unknown { message: String },
}

/// Generic result type for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// Generic result type for application operations
pub type AppResult<T> = Result<T, AppError>;

/// Generic result type for threading operations
pub type ThreadingResult<T> = Result<T, ThreadingError>;

/// Generic result type for API operations
pub type ApiResult<T> = Result<T, ApiError>;

/// Generic result type for performance operations
pub type PerformanceResult<T> = Result<T, PerformanceError>;

/// Generic result type for migration operations
pub type MigrationResult<T> = Result<T, MigrationError>;

/// Comprehensive error type for writing tool operations
/// This is the main error type used throughout the application
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum WritingToolError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    #[error("Application error: {0}")]
    App(#[from] AppError),
    #[error("Threading error: {0}")]
    Threading(#[from] ThreadingError),
    #[error("API error: {0}")]
    Api(#[from] ApiError),
    #[error("Performance error: {0}")]
    Performance(#[from] PerformanceError),
    #[error("Migration error: {0}")]
    Migration(#[from] MigrationError),

    // Specific writing tool errors
    #[error("Voice recognition disabled")]
    VoiceRecognitionDisabled,
    #[error("Speech synthesis disabled")]
    SpeechSynthesisDisabled,
    #[error("Context matcher not found: {0}")]
    ContextMatcherNotFound(String),
    #[error("Command handler not found: {0}")]
    CommandHandlerNotFound(String),
    #[error("Session already active")]
    SessionAlreadyActive,
    #[error("Invalid session ID")]
    InvalidSessionId,
    #[error("No active session")]
    NoActiveSession,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Access denied")]
    AccessDenied,
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Sync error: {0}")]
    SyncError(String),
    #[error("Command error: {0}")]
    CommandError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),
    #[error("Recognition engine unavailable")]
    RecognitionEngineUnavailable,
    #[error("Synthesis engine unavailable")]
    SynthesisEngineUnavailable,
    #[error("Invalid audio format")]
    InvalidAudioFormat,
    #[error("Audio processing error")]
    AudioProcessingError,
    #[error("Script not found: {0}")]
    ScriptNotFound(Uuid),
    #[error("Workflow not found: {0}")]
    WorkflowNotFound(Uuid),
    #[error("Macro not found: {0}")]
    MacroNotFound(Uuid),
    #[error("Execution timeout")]
    ExecutionTimeout,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Invalid script")]
    InvalidScript,
    #[error("Security error: {0}")]
    SecurityError(String),

    // Additional error variants needed by automation and other modules
    #[error("System error: {0}")]
    SystemError(String),
    #[error("File system error: {0}")]
    FileSystemError(String),
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low impact, informational
    Low,
    /// Medium impact, should be addressed
    Medium,
    /// High impact, needs immediate attention
    High,
    /// Critical impact, blocks functionality
    Critical,
}

/// Error category for grouping and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Database-related errors
    Database,
    /// Application logic errors
    Application,
    /// Threading and concurrency errors
    Threading,
    /// API and communication errors
    Api,
    /// Performance and monitoring errors
    Performance,
    /// Migration-specific errors
    Migration,
    /// Configuration errors
    Configuration,
    /// Network and I/O errors
    Network,
    /// Security and authentication errors
    Security,
    /// Unknown or uncategorized errors
    Unknown,
}

/// Structured error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Unique error identifier
    pub error_id: String,
    /// Error message
    pub message: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Error category
    pub category: ErrorCategory,
    /// Error source (module, component, etc.)
    pub source: String,
    /// Error timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    /// Additional context
    pub context: std::collections::HashMap<String, String>,
    /// Suggested resolution
    pub suggested_resolution: Option<String>,
    /// Error cause chain
    pub cause_chain: Vec<String>,
}

impl ErrorInfo {
    /// Create a new error info from error details
    pub fn new<E>(error: &E, source: &str, severity: ErrorSeverity, category: ErrorCategory) -> Self
    where
        E: std::error::Error + ?Sized,
    {
        let mut context = std::collections::HashMap::new();
        context.insert(
            "error_type".to_string(),
            std::any::type_name::<E>().to_string(),
        );

        Self {
            error_id: uuid::Uuid::new_v4().to_string(),
            message: error.to_string(),
            severity,
            category,
            source: source.to_string(),
            timestamp: chrono::Utc::now(),
            stack_trace: None,
            context,
            suggested_resolution: None,
            cause_chain: vec![error.to_string()],
        }
    }

    /// Add context to error info
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// Set suggested resolution
    pub fn with_resolution(mut self, resolution: &str) -> Self {
        self.suggested_resolution = Some(resolution.to_string());
        self
    }

    /// Add to cause chain
    pub fn add_cause(mut self, cause: &str) -> Self {
        self.cause_chain.push(cause.to_string());
        self
    }
}

/// Error handler trait for consistent error processing
pub trait ErrorHandler {
    /// Handle an error and return error info
    fn handle_error<E>(&self, error: &E, source: &str) -> ErrorInfo
    where
        E: std::error::Error + ?Sized;

    /// Log an error
    fn log_error(&self, error_info: &ErrorInfo);

    /// Report an error to external systems
    fn report_error(&self, error_info: &ErrorInfo);

    /// Get error resolution suggestions
    fn get_resolution_suggestions(&self, error_info: &ErrorInfo) -> Vec<String>;

    /// Check if error should trigger alert
    fn should_alert(&self, error_info: &ErrorInfo) -> bool;
}

/// Default error handler implementation
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error<E>(&self, error: &E, source: &str) -> ErrorInfo
    where
        E: std::error::Error + ?Sized,
    {
        let severity = match error.to_string().to_lowercase().as_str() {
            s if s.contains("critical") || s.contains("fatal") => ErrorSeverity::Critical,
            s if s.contains("error") || s.contains("failed") => ErrorSeverity::High,
            s if s.contains("warning") || s.contains("warn") => ErrorSeverity::Medium,
            _ => ErrorSeverity::Low,
        };

        let category = ErrorCategory::Unknown; // Would be determined by error type

        ErrorInfo::new(error, source, severity, category)
    }

    fn log_error(&self, error_info: &ErrorInfo) {
        match error_info.severity {
            ErrorSeverity::Critical => eprintln!(
                "CRITICAL ERROR [{}]: {}",
                error_info.source, error_info.message
            ),
            ErrorSeverity::High => {
                eprintln!("ERROR [{}]: {}", error_info.source, error_info.message)
            }
            ErrorSeverity::Medium => {
                eprintln!("WARNING [{}]: {}", error_info.source, error_info.message)
            }
            ErrorSeverity::Low => eprintln!("INFO [{}]: {}", error_info.source, error_info.message),
        }
    }

    fn report_error(&self, _error_info: &ErrorInfo) {
        // Implementation would send to external error reporting service
    }

    fn get_resolution_suggestions(&self, error_info: &ErrorInfo) -> Vec<String> {
        match error_info.category {
            ErrorCategory::Database => vec![
                "Check database connection".to_string(),
                "Verify database permissions".to_string(),
                "Review database logs".to_string(),
            ],
            ErrorCategory::Application => vec![
                "Review application configuration".to_string(),
                "Check system resources".to_string(),
                "Restart application services".to_string(),
            ],
            ErrorCategory::Threading => vec![
                "Review thread synchronization".to_string(),
                "Check for deadlocks".to_string(),
                "Verify async/await usage".to_string(),
            ],
            _ => vec![
                "Contact support team".to_string(),
                "Check system logs".to_string(),
                "Review error details".to_string(),
            ],
        }
    }

    fn should_alert(&self, error_info: &ErrorInfo) -> bool {
        matches!(
            error_info.severity,
            ErrorSeverity::Critical | ErrorSeverity::High
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_database_error_creation() {
        let error = DatabaseError::ConnectionFailed {
            message: "Connection timeout".to_string(),
        };
        assert!(error.to_string().contains("Connection failed"));
    }

    #[test]
    fn test_app_error_creation() {
        let error = AppError::ToolInitializationFailed {
            tool: "test_tool".to_string(),
            error: "Initialization error".to_string(),
        };
        assert!(error.to_string().contains("Tool initialization failed"));
    }

    #[test]
    fn test_error_severity_levels() {
        assert!(matches!(ErrorSeverity::Critical, ErrorSeverity::Critical));
        assert!(matches!(ErrorSeverity::High, ErrorSeverity::High));
        assert!(matches!(ErrorSeverity::Medium, ErrorSeverity::Medium));
        assert!(matches!(ErrorSeverity::Low, ErrorSeverity::Low));
    }

    #[test]
    fn test_error_info_creation() {
        let error = DatabaseError::ConnectionFailed {
            message: "Test error".to_string(),
        };

        let error_info = ErrorInfo::new(
            &error,
            "test_source",
            ErrorSeverity::High,
            ErrorCategory::Database,
        );

        assert_eq!(error_info.source, "test_source");
        assert_eq!(error_info.severity, ErrorSeverity::High);
        assert_eq!(error_info.category, ErrorCategory::Database);
        assert!(error_info.message.contains("Test error"));
        assert!(!error_info.error_id.is_empty());
    }

    #[test]
    fn test_error_info_with_context() {
        let error = DatabaseError::Timeout {
            timeout: Duration::from_secs(30),
        };

        let error_info = ErrorInfo::new(
            &error,
            "test_source",
            ErrorSeverity::High,
            ErrorCategory::Database,
        )
        .with_context("timeout_value", "30")
        .with_resolution("Increase timeout or check database performance")
        .add_cause("Database server overloaded");

        assert_eq!(
            error_info.context.get("timeout_value"),
            Some(&"30".to_string())
        );
        assert_eq!(
            error_info.suggested_resolution,
            Some("Increase timeout or check database performance".to_string())
        );
        assert!(error_info
            .cause_chain
            .contains(&"Database server overloaded".to_string()));
    }

    #[test]
    fn test_default_error_handler() {
        let handler = DefaultErrorHandler;
        let error = DatabaseError::ConnectionFailed {
            message: "Connection timeout".to_string(),
        };

        let error_info = handler.handle_error(&error, "test_source");
        assert_eq!(error_info.source, "test_source");
        assert!(matches!(error_info.severity, ErrorSeverity::High));

        handler.log_error(&error_info);
        let suggestions = handler.get_resolution_suggestions(&error_info);
        assert!(!suggestions.is_empty());

        assert!(handler.should_alert(&error_info));
    }

    #[test]
    fn test_error_result_types() {
        let db_result: DatabaseResult<String> = Ok("success".to_string());
        assert!(db_result.is_ok());

        let db_error: DatabaseResult<String> = Err(DatabaseError::NotFound {
            entity: "User".to_string(),
            id: "123".to_string(),
        });
        assert!(db_error.is_err());

        let app_result: AppResult<i32> = Ok(42);
        assert!(app_result.is_ok());

        let app_error: AppResult<i32> = Err(AppError::ToolNotFound {
            tool: "nonexistent_tool".to_string(),
        });
        assert!(app_error.is_err());
    }
}
