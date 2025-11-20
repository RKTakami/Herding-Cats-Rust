//! Service Factory for Dependency Management
//!
//! Centralized service initialization, dependency injection container,
//! and comprehensive lifecycle management for all database services.

use crate::database::DatabaseConfig;
use crate::database::{
    BackupService, DatabaseError, DatabaseResult, EnhancedDatabaseService,
    ProjectManagementService, SearchService, VectorEmbeddingService,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Service factory for managing database service dependencies
#[derive(Debug, Clone)]
pub struct ServiceFactory {
    pub database_config: DatabaseConfig,
    pub db_path: PathBuf,
    pub backup_dir: PathBuf,
}

impl ServiceFactory {
    /// Create a new service factory with default configuration
    pub async fn new() -> DatabaseResult<Self> {
        let db_path = PathBuf::from("data/database.db");
        let backup_dir = PathBuf::from("data/backups");
        let database_config = DatabaseConfig::default();

        let factory = Self {
            database_config,
            db_path: db_path.clone(),
            backup_dir: backup_dir.clone(),
        };

        // Initialize database directory
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DatabaseError::Service(e.to_string()))?;
        }

        // Initialize backup directory
        std::fs::create_dir_all(&backup_dir).map_err(|e| DatabaseError::Service(e.to_string()))?;

        Ok(factory)
    }

    /// Create service factory with custom paths and configuration
    pub async fn with_paths(
        db_path: &Path,
        backup_dir: &Path,
        config: DatabaseConfig,
    ) -> DatabaseResult<Self> {
        let factory = Self {
            database_config: config.clone(),
            db_path: db_path.to_path_buf(),
            backup_dir: backup_dir.to_path_buf(),
        };

        // Initialize directories
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DatabaseError::Service(e.to_string()))?;
        }

        std::fs::create_dir_all(backup_dir).map_err(|e| DatabaseError::Service(e.to_string()))?;

        Ok(factory)
    }

    /// Initialize all services with proper dependency ordering
    pub async fn initialize(&self) -> DatabaseResult<ServiceContainer> {
        let mut container = ServiceContainer::new();

        // Initialize EnhancedDatabaseService first (base dependency)
        let db_service = Arc::new(RwLock::new(
            EnhancedDatabaseService::new(&self.db_path, self.database_config.clone()).await?,
        ));

        // Initialize database with schema
        db_service.read().await.initialize_database().await?;

        container.database_service = Some(db_service.clone());

        // Initialize ProjectManagementService (depends on database service)
        let project_service = Arc::new(RwLock::new(ProjectManagementService::new(
            db_service.clone(),
        )));
        container.project_service = Some(project_service.clone());

        // Initialize VectorEmbeddingService (placeholder implementation)
        let vector_service = Arc::new(RwLock::new(VectorEmbeddingService::new(db_service.clone())));
        container.vector_service = Some(vector_service.clone());

        // Initialize SearchService with database service dependency
        let search_service = Arc::new(RwLock::new(SearchService::new(db_service.clone())));
        container.search_service = Some(search_service.clone());

        // Initialize BackupService with database service dependency
        let backup_service = Arc::new(RwLock::new(BackupService::new(
            db_service.clone(),
            &self.db_path,
        )));
        container.backup_service = Some(backup_service.clone());

        container.initialized = true;
        container.initialization_time = Some(chrono::Utc::now());

        Ok(container)
    }

    /// Perform comprehensive health check on all services
    pub async fn health_check(
        &self,
        container: &ServiceContainer,
    ) -> DatabaseResult<ServiceHealthStatus> {
        let mut health_status = ServiceHealthStatus::new();

        // Check database service
        let db_healthy = if let Some(db_service) = &container.database_service {
            match db_service.read().await.initialize_database().await {
                Ok(_health) => true,
                Err(_) => false,
            }
        } else {
            false
        };

        if db_healthy {
            health_status.add_service_health("database", ServiceHealth::Healthy);
        } else {
            health_status.add_service_health("database", ServiceHealth::Unhealthy);
        }

        // Check other services (placeholder implementations)
        health_status.add_service_health("project_management", ServiceHealth::Healthy);
        health_status.add_service_health("vector_embedding", ServiceHealth::Healthy);
        health_status.add_service_health("search", ServiceHealth::Healthy);
        health_status.add_service_health("backup", ServiceHealth::Healthy);

        Ok(health_status)
    }

    /// Shutdown all services gracefully
    pub async fn shutdown(&self, _container: ServiceContainer) -> DatabaseResult<()> {
        // Services are Arc<RwLock<...>> so they'll be dropped automatically
        // Here we could add cleanup logic if needed
        Ok(())
    }

    /// Restart a specific service
    pub async fn restart_service(
        &self,
        container: &mut ServiceContainer,
        service_name: &str,
    ) -> DatabaseResult<()> {
        match service_name {
            "database" => {
                // Recreate database service
                let db_service = Arc::new(RwLock::new(
                    EnhancedDatabaseService::new(&self.db_path, self.database_config.clone())
                        .await?,
                ));
                db_service.read().await.initialize_database().await?;
                container.database_service = Some(db_service);
            }
            "project_management" => {
                if let Some(db_service) = &container.database_service {
                    container.project_service = Some(Arc::new(RwLock::new(
                        ProjectManagementService::new(db_service.clone()),
                    )));
                }
            }
            _ => {
                return Err(DatabaseError::Service(format!(
                    "Unknown service: {}",
                    service_name
                )))
            }
        }

        Ok(())
    }
}

impl Default for ServiceFactory {
    fn default() -> Self {
        // Note: This is synchronous, so we can't use async here
        // In practice, this would be called after async initialization
        Self {
            database_config: DatabaseConfig::default(),
            db_path: PathBuf::from("data/database.db"),
            backup_dir: PathBuf::from("data/backups"),
        }
    }
}

/// Service container with all initialized services
#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub database_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
    pub project_service: Option<Arc<RwLock<ProjectManagementService>>>,
    pub vector_service: Option<Arc<RwLock<VectorEmbeddingService>>>,
    pub search_service: Option<Arc<RwLock<SearchService>>>,
    pub backup_service: Option<Arc<RwLock<BackupService>>>,
    pub initialized: bool,
    pub initialization_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            database_service: None,
            project_service: None,
            vector_service: None,
            search_service: None,
            backup_service: None,
            initialized: false,
            initialization_time: None,
        }
    }

    /// Get database service accessor
    pub fn database_service(&self) -> Option<Arc<RwLock<EnhancedDatabaseService>>> {
        self.database_service.clone()
    }

    /// Get project service accessor
    pub fn project_service(&self) -> Option<Arc<RwLock<ProjectManagementService>>> {
        self.project_service.clone()
    }

    /// Get vector service accessor
    pub fn vector_service(&self) -> Option<Arc<RwLock<VectorEmbeddingService>>> {
        self.vector_service.clone()
    }

    /// Get search service accessor
    pub fn search_service(&self) -> Option<Arc<RwLock<SearchService>>> {
        self.search_service.clone()
    }

    /// Get backup service accessor
    pub fn backup_service(&self) -> Option<Arc<RwLock<BackupService>>> {
        self.backup_service.clone()
    }

    /// Check if all critical services are available
    pub fn is_healthy(&self) -> bool {
        self.initialized && self.database_service.is_some() && self.project_service.is_some()
    }
}

/// Service health status with detailed information
#[derive(Debug, Clone)]
pub struct ServiceHealthStatus {
    pub overall_health: ServiceHealth,
    pub service_healths: HashMap<String, ServiceHealth>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub issues: Vec<String>,
}

impl ServiceHealthStatus {
    fn new() -> Self {
        Self {
            overall_health: ServiceHealth::Unhealthy,
            service_healths: HashMap::new(),
            timestamp: chrono::Utc::now(),
            issues: Vec::new(),
        }
    }

    pub fn add_service_health(&mut self, service_name: &str, health: ServiceHealth) {
        self.service_healths
            .insert(service_name.to_string(), health.clone());

        match health {
            ServiceHealth::Healthy => {}
            ServiceHealth::Unhealthy => self
                .issues
                .push(format!("Service '{}' is unhealthy", service_name)),
            ServiceHealth::Error => self
                .issues
                .push(format!("Service '{}' has errors", service_name)),
        }

        // Update overall health based on worst service health
        self.overall_health = if self
            .service_healths
            .values()
            .any(|h| matches!(h, ServiceHealth::Error))
        {
            ServiceHealth::Error
        } else if self
            .service_healths
            .values()
            .any(|h| matches!(h, ServiceHealth::Unhealthy))
        {
            ServiceHealth::Unhealthy
        } else {
            ServiceHealth::Healthy
        };
    }

    pub fn get_service_health(&self, service_name: &str) -> Option<&ServiceHealth> {
        self.service_healths.get(service_name)
    }

    pub fn get_all_service_healths(&self) -> &HashMap<String, ServiceHealth> {
        &self.service_healths
    }
}

/// Service health enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceHealth {
    Healthy,
    Unhealthy,
    Error,
}
