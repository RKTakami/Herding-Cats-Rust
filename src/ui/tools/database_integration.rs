//! Unified Database Integration Pattern for UI Tools
//!
//! Provides consistent, safe database access patterns for all UI tools
//! with proper async/await support and thread-safe operations.

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::{DatabaseError, DatabaseAppState, EnhancedDatabaseService};

/// Unified database context for UI tools
/// Provides consistent access to database services with proper error handling
#[derive(Debug, Clone)]
pub struct ToolDatabaseContext {
    /// Reference to the central database application state
    database_state: Arc<RwLock<DatabaseAppState>>,
    /// Direct access to enhanced database service (when available)
    db_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
    /// Tool identifier for logging and debugging
    tool_id: String,
}

impl ToolDatabaseContext {
    /// Create a new database context for a tool
    pub async fn new(tool_id: &str, database_state: Arc<RwLock<DatabaseAppState>>) -> Self {
        let db_service = if database_state.read().await.is_database_ready() {
            database_state.read().await.database_service()
        } else {
            debug!(
                "Tool '{}' - Database not ready, will retry on first access",
                tool_id
            );
            None
        };

        Self {
            database_state,
            db_service,
            tool_id: tool_id.to_string(),
        }
    }

    /// Get database service, initializing if necessary
    pub async fn get_database_service(
        &mut self,
    ) -> Result<Arc<RwLock<EnhancedDatabaseService>>, DatabaseError> {
        // Try cached service first
        if let Some(service) = &self.db_service {
            return Ok(service.clone());
        }

        // Try to get from application state
        let state = self.database_state.read().await;
        if !state.is_database_ready() {
            return Err(DatabaseError::Service(format!(
                "Database not initialized for tool '{}'",
                self.tool_id
            )));
        }

        let service = state.database_service().ok_or_else(|| {
            DatabaseError::Service(format!(
                "Database service unavailable for tool '{}'",
                self.tool_id
            ))
        })?;

        // Cache the service for future use
        self.db_service = Some(service.clone());
        Ok(service)
    }

    /// Ensure database is initialized (lazy initialization)
    pub async fn ensure_database_initialized(&mut self) -> Result<(), DatabaseError> {
        let state = self.database_state.write().await;
        if !state.is_database_ready() {
            debug!(
                "Tool '{}' - Database not ready, requires external initialization",
                self.tool_id
            );
            return Err(DatabaseError::Service(
                format!("Database services not initialized for tool '{}'. Please initialize database externally.", self.tool_id)
            ));
        }
        Ok(())
    }

    /// Execute a database operation with automatic retry and error handling
    pub async fn execute_with_retry<F, Fut, T>(
        &mut self,
        operation_name: &str,
        operation: F,
        max_retries: usize,
    ) -> Result<T, DatabaseError>
    where
        F: Fn(Arc<RwLock<EnhancedDatabaseService>>) -> Fut,
        Fut: std::future::Future<Output = Result<T, DatabaseError>>,
    {
        for attempt in 1..=max_retries {
            // Get database service for this attempt
            let db_service = self.get_database_service().await?;

            // Create the operation for this attempt
            let operation_future = operation(db_service);

            match operation_future.await {
                Ok(result) => {
                    if attempt > 1 {
                        debug!(
                            "Tool '{}' - Operation '{}' succeeded on attempt {}",
                            self.tool_id, operation_name, attempt
                        );
                    }
                    return Ok(result);
                }
                Err(DatabaseError::Connection(_)) if attempt < max_retries => {
                    warn!(
                        "Tool '{}' - Connection error on attempt {} for '{}', retrying...",
                        self.tool_id, attempt, operation_name
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis((100 * attempt) as u64))
                        .await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }

        Err(DatabaseError::Service(format!(
            "Tool '{}' - Operation '{}' failed after {} attempts",
            self.tool_id, operation_name, max_retries
        )))
    }

    /// Get tool identifier for logging
    pub fn tool_id(&self) -> &str {
        &self.tool_id
    }

    /// Check if database is ready
    pub async fn is_database_ready(&self) -> bool {
        self.database_state.read().await.is_database_ready()
    }

    /// Check if database context is connected
    pub async fn is_connected(&self) -> bool {
        self.is_database_ready().await
    }
}

/// Tool database service trait for consistent interface
#[async_trait]
pub trait ToolDatabaseService: Send + Sync {
    /// Initialize the service
    async fn initialize(&self) -> Result<(), DatabaseError>;

    /// Check if service is healthy
    async fn health_check(&self) -> Result<bool, DatabaseError>;

    /// Get service name for logging
    fn service_name(&self) -> &'static str;
}

/// Database operation result wrapper with consistent error handling
#[derive(Debug, Clone)]
pub enum DatabaseOperationResult<T> {
    Success {
        data: T,
        operation_time_ms: u64,
    },
    NotFound {
        entity_type: &'static str,
        entity_id: String,
    },
    ValidationError {
        field: String,
        message: String,
    },
    ConnectionError {
        message: String,
    },
    PermissionError {
        message: String,
    },
    UnknownError {
        message: String,
    },
}

impl<T> DatabaseOperationResult<T> {
    /// Create a success result with timing
    pub fn success(data: T, operation_time_ms: u64) -> Self {
        Self::Success {
            data,
            operation_time_ms,
        }
    }

    /// Create a not found result
    pub fn not_found(entity_type: &'static str, entity_id: String) -> Self {
        Self::NotFound {
            entity_type,
            entity_id,
        }
    }

    /// Create a validation error result
    pub fn validation_error(field: String, message: String) -> Self {
        Self::ValidationError { field, message }
    }

    /// Check if operation was successful
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Extract data if successful, otherwise return error
    pub fn into_result(self) -> Result<T, DatabaseError> {
        match self {
            Self::Success { data, .. } => Ok(data),
            Self::NotFound {
                entity_type,
                entity_id,
            } => Err(DatabaseError::NotFound(format!(
                "{} with id {}",
                entity_type, entity_id
            ))),
            Self::ValidationError { field, message } => Err(DatabaseError::ValidationError(
                format!("Validation error in field '{}': {}", field, message),
            )),
            Self::ConnectionError { message } => Err(DatabaseError::Connection(message)),
            Self::PermissionError { message } => Err(DatabaseError::Service(format!(
                "Permission denied: {}",
                message
            ))),
            Self::UnknownError { message } => Err(DatabaseError::Service(format!(
                "Unknown database error: {}",
                message
            ))),
        }
    }
}

/// Database migration helper for tool-specific schema changes
pub struct ToolDatabaseMigrator {
    context: ToolDatabaseContext,
    tool_name: String,
    current_version: u32,
}

impl ToolDatabaseMigrator {
    /// Create a new migrator for a tool
    pub fn new(context: ToolDatabaseContext, tool_name: &str, current_version: u32) -> Self {
        Self {
            context,
            tool_name: tool_name.to_string(),
            current_version,
        }
    }

    /// Execute tool-specific migrations
    pub async fn migrate_to_version(&mut self, target_version: u32) -> Result<(), DatabaseError> {
        if target_version <= self.current_version {
            debug!(
                "Tool '{}' - Already at version {} or newer",
                self.tool_name, self.current_version
            );
            return Ok(());
        }

        debug!(
            "Tool '{}' - Migrating from version {} to {}",
            self.tool_name, self.current_version, target_version
        );

        for version in (self.current_version + 1)..=target_version {
            self.migrate_to_version_internal(version).await?;
        }

        self.current_version = target_version;
        Ok(())
    }

    async fn migrate_to_version_internal(&mut self, version: u32) -> Result<(), DatabaseError> {
        let migration_sql = self.get_migration_sql(version)?;

        self.context
            .execute_with_retry(
                &format!("migration_v{}", version),
                move |db_service: Arc<RwLock<EnhancedDatabaseService>>| {
                    let sql = migration_sql.clone();
                    Box::pin(async move {
                        let connection = db_service.read().await;
                        connection.execute(&sql, &[]).await.map_err(|e| {
                            DatabaseError::Service(format!("Database migration failed: {}", e))
                        })
                    })
                },
                3,
            )
            .await?;

        debug!(
            "Tool '{}' - Successfully applied migration v{}",
            self.tool_name, version
        );
        Ok(())
    }

    fn get_migration_sql(&self, version: u32) -> Result<String, DatabaseError> {
        match (self.tool_name.as_str(), version) {
            // Tool-specific migration SQL would be defined here
            ("hierarchy", 1) => Ok(include_str!("migrations/hierarchy_v1.sql").to_string()),
            ("codex", 1) => Ok(include_str!("migrations/codex_v1.sql").to_string()),
            _ => Err(DatabaseError::Service(format!(
                "No migration found for tool '{}' version {}",
                self.tool_name, version
            ))),
        }
    }
}

/// Connection pool configuration for UI tools
#[derive(Debug, Clone)]
pub struct ToolDatabaseConfig {
    pub max_connections: u32,
    pub connection_timeout_ms: u64,
    pub command_timeout_ms: u64,
    pub retry_attempts: usize,
    pub enable_query_logging: bool,
}

impl Default for ToolDatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 5,
            connection_timeout_ms: 5000,
            command_timeout_ms: 30000,
            retry_attempts: 3,
            enable_query_logging: cfg!(debug_assertions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_tool_database_context_creation() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut context = ToolDatabaseContext::new("test_tool", database_state).await;

        assert_eq!(context.tool_id(), "test_tool");
        assert!(!context.is_database_ready().await);
    }

    #[tokio::test]
    async fn test_database_operation_result() {
        let success_result: DatabaseOperationResult<String> =
            DatabaseOperationResult::success("test_data".to_string(), 10);

        assert!(success_result.is_success());
        let data = success_result.into_result().unwrap();
        assert_eq!(data, "test_data");
    }

    #[tokio::test]
    async fn test_validation_error_result() {
        let error_result: DatabaseOperationResult<String> =
            DatabaseOperationResult::validation_error(
                "title".to_string(),
                "Title is required".to_string(),
            );

        assert!(!error_result.is_success());
        let result = error_result.into_result();
        assert!(result.is_err());

        match result {
            Err(DatabaseError::ValidationError(_)) => {
                // Expected
            }
            _ => {
                panic!("Expected validation error");
            }
        }
    }
}
