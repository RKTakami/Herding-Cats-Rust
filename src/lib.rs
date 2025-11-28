//! Herding Cats - Main Library Module
//!
//! This is the main library module for the Herding Cats application.
//! It exports all major subsystems including the database integration.

pub mod automation;
pub mod ipc_bridge;
pub mod database;
pub mod database_app_state;
pub mod error;
pub mod file_ops;
pub mod services;
pub mod settings;

pub mod classify;
pub mod convert;
pub mod security;
pub mod font_manager;

// Re-export database types for easier access
pub use database::{
    initialize_database, BackupService, DatabaseConfig, DatabaseService, EnhancedDatabaseService,
    ProjectManagementService, ResearchService, SearchService, ServiceFactory,
    VectorEmbeddingService,
};

// Re-export ServiceContainer from service_factory
pub use database::service_factory::ServiceContainer;

// Re-export settings module
pub use settings::{
    load_theme_settings, save_theme_settings, ThemeSettings,
};

// Re-export models
pub use database::models::{
    BatchEmbeddingRequest, Document, DocumentEmbedding, DocumentStatistics, DocumentVersion,
    EmbeddingStatistics, ModelResult, Project, ProjectStatistics, SearchResult,
};

// Re-export codex models
pub use database::models::codex::{
    CharacterData, CodexEntry, CodexEntryType, CodexExportResult, CodexImportResult, CodexQuery,
    CodexSortField, CodexStatistics, CodexStatus, EnhancedCodexEntry, ObjectData, PlaceData,
    StoryData, TimeData,
};

// Re-export search service types
pub use database::search_service::{
    DateRange, SearchOptions, SearchStatistics, SortField, SortOrder,
};

// Re-export database app state types
pub use database_app_state::{
    ConnectionPoolStats, DatabaseAppState, DatabaseHealthStatus,
    DatabaseOperationResult as AppDatabaseOperationResult, DatabaseStats, PoolConfig,
};

// Re-export UI state types


// Re-export database module's error types as primary
pub use database::{DatabaseError, DatabaseResult};

// Re-export backup service types
pub use database::backup_service::{BackupMetadata, BackupStatistics, BackupType};

// Re-export automation types for easier access
pub use automation::EventType;

/// Application version
pub const VERSION: &str = "2.0.0";

/// Application name
pub const NAME: &str = "Herding Cats";

/// Initialize the application database services
///
/// This function creates and initializes all database services with default settings.
/// It's a convenient way to get started with the database system.
pub async fn init_database() -> Result<ServiceFactory, Box<dyn std::error::Error>> {
    let service_factory = ServiceFactory::new()
        .await
        .map_err(|e| format!("Failed to initialize database services: {}", e))?;

    Ok(service_factory)
}

/// Get database configuration from the database module
pub fn get_database_config() -> crate::database::DatabaseConfig {
    crate::database::DatabaseConfig::default()
}
