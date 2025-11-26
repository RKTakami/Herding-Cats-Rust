//! Database module for Herding Cats application
//!
//! This module provides enterprise-grade database services including:
//! - Enhanced database operations with integrity checking
//! - Project management with multi-project support
//! - Vector embedding services for LLM integration
//! - Full-text search with FTS5 support
//! - Backup and recovery services
//! - Service factory for dependency management

use serde::{Deserialize, Serialize};
use sqlx;

pub mod analysis_service;
pub mod backup_service;
pub mod enhanced_database_sqlx;
pub mod project_management;
pub mod research_service;
pub mod search_service;
pub mod service_factory;
pub mod vector_embedding;

pub mod models;


// Re-export key types for easier import
pub use backup_service::BackupService;
pub use enhanced_database_sqlx::DatabaseConfig;
pub use enhanced_database_sqlx::EnhancedDatabaseService;
pub use project_management::ProjectManagementService;
pub use research_service::ResearchService;
pub use search_service::SearchService;
pub use service_factory::ServiceFactory;
pub use vector_embedding::VectorEmbeddingService;

/// DatabaseService type alias for EnhancedDatabaseService
pub type DatabaseService = EnhancedDatabaseService;

// Re-export models
pub use models::*;

/// Database-related error types
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseError {
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

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

// sqlx error conversion
impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        DatabaseError::Service(format!("SQLx error: {}", error))
    }
}

/// Database initialization result type
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// Module initialization helper - now async
pub async fn initialize_database() -> DatabaseResult<ServiceFactory> {
    ServiceFactory::new().await
}
