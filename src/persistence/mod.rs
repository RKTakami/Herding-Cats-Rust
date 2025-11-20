//! Persistence and Data Management System for Herding Cats Writing Tools
//! 
//! This module provides comprehensive project persistence including structure validation,
//! data management, and coordination with backup and search systems.

pub mod index_manager;
pub mod project_structure;
pub mod export;
pub mod import;
pub mod data_validator;
pub mod migration;
pub mod health_monitor;

pub use index_manager::{IndexManager, IndexEntry, IndexMetadata};
pub use project_structure::{ProjectStructure, StructureValidator, DirectoryType};
pub use export::{ExportManager, ExportConfig, ExportFormat};
pub use import::{ImportManager, ImportConfig, ImportResult};
pub use data_validator::{DataValidator, ValidationResult, ValidationIssue};
pub use migration::{MigrationManager, MigrationResult, MigrationStep};
pub use health_monitor::{HealthMonitor, HealthReport, HealthIssue};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;

/// Main persistence manager coordinating all persistence operations
#[derive(Debug)]
pub struct PersistenceManager {
    config: PersistenceConfig,
    project_path: PathBuf,
    structure_validator: Arc<RwLock<StructureValidator>>,
    index_manager: Arc<RwLock<IndexManager>>,
    export_manager: Arc<RwLock<ExportManager>>,
    import_manager: Arc<RwLock<ImportManager>>,
    data_validator: Arc<RwLock<DataValidator>>,
    health_monitor: Arc<RwLock<HealthMonitor>>,
}

/// Persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    pub auto_validate_on_load: bool,
    pub auto_backup_on_save: bool,
    pub backup_before_migration: bool,
    pub validate_integrity: bool,
    pub compress_exports: bool,
    pub encrypt_sensitive_data: bool,
    pub max_backup_versions: usize,
    pub auto_cleanup_old_versions: bool,
    pub enable_health_monitoring: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            auto_validate_on_load: true,
            auto_backup_on_save: true,
            backup_before_migration: true,
            validate_integrity: true,
            compress_exports: true,
            encrypt_sensitive_data: false,
            max_backup_versions: 10,
            auto_cleanup_old_versions: true,
            enable_health_monitoring: true,
        }
    }
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub author: String,
    pub description: Option<String>,
    pub tool_version: String,
    pub structure_version: u32,
}

/// Tool data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolDataType {
    HierarchyData(serde_json::Value),
    CodexData(serde_json::Value),
    NotesData(serde_json::Value),
    ResearchData(serde_json::Value),
    PlotData(serde_json::Value),
    AnalysisData(serde_json::Value),
    StructureData(serde_json::Value),
    AllToolData(HashMap<String, serde_json::Value>),
}

/// Persistence operation result
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    
    #[error("Structure validation failed: {0}")]
    StructureValidationFailed(String),
    
    #[error("Index error: {0}")]
    IndexError(String),
    
    #[error("Export error: {0}")]
    ExportError(String),
    
    #[error("Import error: {0}")]
    ImportError(String),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Health check failed: {0}")]
    HealthCheckError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
}

/// Result type for persistence operations
pub type PersistenceResult<T> = Result<T, PersistenceError>;

impl PersistenceManager {
    /// Create new persistence manager
    pub fn new(config: PersistenceConfig, project_path: PathBuf) -> PersistenceResult<Self> {
        // Ensure project directory exists
        fs::create_dir_all(&project_path)?;
        
        // Initialize sub-managers
        let structure_validator = Arc::new(RwLock::new(StructureValidator::new(project_path.clone())?));
        let index_manager = Arc::new(RwLock::new(IndexManager::new(project_path.clone())?));
        let export_manager = Arc::new(RwLock::new(ExportManager::new(project_path.clone())?));
        let import_manager = Arc::new(RwLock::new(ImportManager::new(project_path.clone())?));
        let data_validator = Arc::new(RwLock::new(DataValidator::new(project_path.clone())?));
        let health_monitor = Arc::new(RwLock::new(HealthMonitor::new(project_path.clone())?));
        
        Ok(Self {
            config,
            project_path,
            structure_validator,
            index_manager,
            export_manager,
            import_manager,
            data_validator,
            health_monitor,
        })
    }
    
    /// Initialize a new project with proper structure
    pub async fn initialize_project(&self, metadata: ProjectMetadata) -> PersistenceResult<()> {
        // Validate and create project structure
        {
            let mut validator = self.structure_validator.write().await;
            validator.ensure_project_structure(&metadata)?;
        }
        
        // Create project metadata file
        self.save_project_metadata(&metadata).await?;
        
        // Initialize indexes
        {
            let mut index_manager = self.index_manager.write().await;
            index_manager.initialize_project_indexes(&metadata)?;
        }
        
        // Initial health check
        if self.config.enable_health_monitoring {
            let health_report = self.health_monitor.read().await.check_project_health().await?;
            if !health_report.is_healthy() {
                return Err(PersistenceError::HealthCheckError(
                    "Project health check failed during initialization".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Load existing project
    pub async fn load_project(&self) -> PersistenceResult<ProjectMetadata> {
        // Validate project structure
        if self.config.auto_validate_on_load {
            {
                let validator = self.structure_validator.read().await;
                validator.validate_project_structure()?;
            }
            
            // Data integrity validation
            if self.config.validate_integrity {
                let validator = self.data_validator.read().await;
                let validation_result = validator.validate_project_integrity().await?;
                if !validation_result.is_valid() {
                    return Err(PersistenceError::ValidationError(
                        "Project data integrity validation failed".to_string()
                    ));
                }
            }
        }
        
        // Load project metadata
        self.load_project_metadata().await
    }
    
    /// Save tool data
    pub async fn save_tool_data(&self, tool_type: &str, data: &ToolDataType) -> PersistenceResult<()> {
        // Auto backup before save if enabled
        if self.config.auto_backup_on_save {
            // This would trigger a backup operation
            // For now, just log the intent
        }
        
        // Save to appropriate directory based on tool type
        let save_path = self.get_tool_save_path(tool_type)?;
        self.save_tool_data_to_file(save_path, data).await?;
        
        // Update indexes
        {
            let mut index_manager = self.index_manager.write().await;
            index_manager.update_tool_index(tool_type, data)?;
        }
        
        // Update project metadata
        self.update_project_modified_time().await?;
        
        Ok(())
    }
    
    /// Load tool data
    pub async fn load_tool_data(&self, tool_type: &str) -> PersistenceResult<ToolDataType> {
        let load_path = self.get_tool_load_path(tool_type)?;
        self.load_tool_data_from_file(&load_path).await
    }
    
    /// Export project
    pub async fn export_project(&self, format: ExportFormat, include_index: bool) -> PersistenceResult<PathBuf> {
        let export_manager = self.export_manager.read().await;
        export_manager.export_project(format, include_index).await
    }
    
    /// Import project
    pub async fn import_project(&self, source_path: &Path, config: ImportConfig) -> PersistenceResult<ImportResult> {
        let import_manager = self.import_manager.read().await;
        import_manager.import_project(source_path, config).await
    }
    
    /// Validate project structure
    pub async fn validate_structure(&self) -> PersistenceResult<bool> {
        let validator = self.structure_validator.read().await;
        Ok(validator.validate_project_structure().is_ok())
    }
    
    /// Get project statistics
    pub async fn get_project_stats(&self) -> PersistenceResult<ProjectStatistics> {
        let mut stats = ProjectStatistics::default();
        
        // Get directory statistics
        {
            let validator = self.structure_validator.read().await;
            stats = validator.get_directory_statistics()?;
        }
        
        // Get index statistics
        {
            let index_manager = self.index_manager.read().await;
            let index_stats = index_manager.get_index_statistics()?;
            stats.total_indexed_items = index_stats.total_items;
            stats.index_size_bytes = index_stats.total_size_bytes;
        }
        
        // Get health information
        if self.config.enable_health_monitoring {
            let health_monitor = self.health_monitor.read().await;
            let health_report = health_monitor.get_latest_health_report().await?;
            stats.health_score = health_report.overall_score;
            stats.last_health_check = Some(health_report.checked_at);
        }
        
        Ok(stats)
    }
    
    /// Run project migration
    pub async fn migrate_project(&self, target_version: Option<String>) -> PersistenceResult<MigrationResult> {
        // Backup before migration if enabled
        if self.config.backup_before_migration {
            // This would trigger a backup operation
        }
        
        // Run migration
        let migration_result = {
            let migration_manager = MigrationManager::new(self.project_path.clone());
            migration_manager.migrate_project(target_version).await?
        };
        
        // Update indexes after migration
        {
            let mut index_manager = self.index_manager.write().await;
            index_manager.rebuild_all_indexes().await?;
        }
        
        // Validate after migration
        if self.config.validate_integrity {
            let validator = self.data_validator.read().await;
            let validation_result = validator.validate_project_integrity().await?;
            if !validation_result.is_valid() {
                return Err(PersistenceError::ValidationError(
                    "Post-migration validation failed".to_string()
                ));
            }
        }
        
        Ok(migration_result)
    }
    
    /// Check project health
    pub async fn check_project_health(&self) -> PersistenceResult<HealthReport> {
        let health_monitor = self.health_monitor.read().await;
        health_monitor.check_project_health().await
    }
    
    /// Clean up old backup versions
    pub async fn cleanup_old_versions(&self) -> PersistenceResult<u32> {
        if !self.config.auto_cleanup_old_versions {
            return Ok(0);
        }
        
        let backup_dir = self.project_path.join("backups");
        if !backup_dir.exists() {
            return Ok(0);
        }
        
        let mut cleaned_count = 0u32;
        
        // Get all backup directories and sort by modification time
        let mut backup_dirs: Vec<(PathBuf, SystemTime)> = Vec::new();
        if backup_dir.is_dir() {
            for entry in fs::read_dir(&backup_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    let metadata = fs::metadata(&path)?;
                    let modified = metadata.modified()?.into();
                    backup_dirs.push((path, modified));
                }
            }
        }
        
        // Sort by modification time (oldest first)
        backup_dirs.sort_by(|a, b| a.1.cmp(&b.1));
        
        // Keep only the most recent versions
        let to_remove = if backup_dirs.len() > self.config.max_backup_versions {
            backup_dirs.len() - self.config.max_backup_versions
        } else {
            0
        };
        
        for (path, _) in backup_dirs.into_iter().take(to_remove) {
            fs::remove_dir_all(&path)?;
            cleaned_count += 1;
        }
        
        Ok(cleaned_count)
    }
    
    // Private helper methods
    
    async fn save_project_metadata(&self, metadata: &ProjectMetadata) -> PersistenceResult<()> {
        let metadata_path = self.project_path.join("project.json");
        let content = serde_json::to_string_pretty(metadata)?;
        fs::write(metadata_path, content)?;
        Ok(())
    }
    
    async fn load_project_metadata(&self) -> PersistenceResult<ProjectMetadata> {
        let metadata_path = self.project_path.join("project.json");
        let content = fs::read_to_string(&metadata_path)?;
        let metadata: ProjectMetadata = serde_json::from_str(&content)?;
        Ok(metadata)
    }
    
    async fn update_project_modified_time(&self) -> PersistenceResult<()> {
        let mut metadata = self.load_project_metadata().await?;
        metadata.modified_at = SystemTime::now();
        self.save_project_metadata(&metadata).await?;
        Ok(())
    }
    
    fn get_tool_save_path(&self, tool_type: &str) -> PersistenceResult<PathBuf> {
        let tool_dir = match tool_type.to_lowercase().as_str() {
            "hierarchy" => "hierarchy",
            "codex" => "codex",
            "notes" => "notes",
            "research" => "research",
            "plot" => "plot",
            "analysis" => "analysis",
            "structure" => "structure",
            _ => return Err(PersistenceError::Configuration(format!("Unknown tool type: {}", tool_type))),
        };
        
        Ok(self.project_path
            .join("content")
            .join(tool_dir)
            .join("data.json"))
    }
    
    fn get_tool_load_path(&self, tool_type: &str) -> PersistenceResult<PathBuf> {
        self.get_tool_save_path(tool_type)
    }
    
    async fn save_tool_data_to_file(&self, path: PathBuf, data: &ToolDataType) -> PersistenceResult<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = match data {
            ToolDataType::HierarchyData(json) => serde_json::to_string_pretty(json)?,
            ToolDataType::CodexData(json) => serde_json::to_string_pretty(json)?,
            ToolDataType::NotesData(json) => serde_json::to_string_pretty(json)?,
            ToolDataType::ResearchData(json) => serde_json::to_string_pretty(json)?,
            ToolDataType::PlotData(json) => serde_json::to_string_pretty(json)?,
            ToolDataType::AnalysisData(json) => serde_json::to_string_pretty(json)?,
            ToolDataType::StructureData(json) => serde_json::to_string_pretty(json)?,
            ToolDataType::AllToolData(map) => serde_json::to_string_pretty(map)?,
        };
        
        fs::write(path, content)?;
        Ok(())
    }
    
    async fn load_tool_data_from_file(&self, path: &Path) -> PersistenceResult<ToolDataType> {
        if !path.exists() {
            return Err(PersistenceError::ProjectNotFound(format!("Tool data file not found: {}", path.display())));
        }
        
        let content = fs::read_to_string(path)?;
        
        // Try to determine tool type from path
        let tool_type = if path.to_string_lossy().contains("hierarchy") {
            "hierarchy"
        } else if path.to_string_lossy().contains("codex") {
            "codex"
        } else if path.to_string_lossy().contains("notes") {
            "notes"
        } else if path.to_string_lossy().contains("research") {
            "research"
        } else if path.to_string_lossy().contains("plot") {
            "plot"
        } else if path.to_string_lossy().contains("analysis") {
            "analysis"
        } else if path.to_string_lossy().contains("structure") {
            "structure"
        } else {
            return Err(PersistenceError::Configuration("Cannot determine tool type from path".to_string()));
        };
        
        let json_value: serde_json::Value = serde_json::from_str(&content)?;
        
        let tool_data = match tool_type {
            "hierarchy" => ToolDataType::HierarchyData(json_value),
            "codex" => ToolDataType::CodexData(json_value),
            "notes" => ToolDataType::NotesData(json_value),
            "research" => ToolDataType::ResearchData(json_value),
            "plot" => ToolDataType::PlotData(json_value),
            "analysis" => ToolDataType::AnalysisData(json_value),
            "structure" => ToolDataType::StructureData(json_value),
            _ => return Err(PersistenceError::Configuration(format!("Unknown tool type: {}", tool_type))),
        };
        
        Ok(tool_data)
    }
}

/// Project statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectStatistics {
    pub total_files: usize,
    pub total_size_bytes: u64,
    pub content_files: usize,
    pub settings_files: usize,
    pub backup_files: usize,
    pub total_indexed_items: usize,
    pub index_size_bytes: usize,
    pub health_score: f32,
    pub last_health_check: Option<SystemTime>,
}

/// Tool directory types for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolDirectoryType {
    Hierarchy,
    Codex,
    Notes,
    Research,
    Plot,
    Analysis,
    Structure,
    Settings,
    Index,
    Backup,
}

/// Integration with search module
impl PersistenceManager {
    /// Trigger search index rebuild
    pub async fn rebuild_search_indexes(&self) -> PersistenceResult<()> {
        let index_manager = self.index_manager.read().await;
        index_manager.rebuild_all_indexes().await
    }
    
    /// Get index entries for search
    pub async fn get_search_index_entries(&self) -> PersistenceResult<Vec<IndexEntry>> {
        let index_manager = self.index_manager.read().await;
        index_manager.get_all_entries().await
    }
    
    /// Update search index for specific tool
    pub async fn update_search_index(&self, tool_type: &str) -> PersistenceResult<()> {
        let mut index_manager = self.index_manager.write().await;
        index_manager.rebuild_tool_index(tool_type).await
    }
}

/// Global persistence manager
use once_cell::sync::Lazy;
use std::sync::Mutex;

static PERSISTENCE_MANAGER: Lazy<Mutex<Option<PersistenceManager>>> = Lazy::new(|| Mutex::new(None));

/// Initialize global persistence manager
pub fn init_persistence_manager(config: PersistenceConfig, project_path: PathBuf) -> PersistenceResult<()> {
    let mut manager = PERSISTENCE_MANAGER.lock().unwrap();
    *manager = Some(PersistenceManager::new(config, project_path)?);
    Ok(())
}

/// Get global persistence manager
pub async fn get_persistence_manager() -> Option<PersistenceManager> {
    let manager = PERSISTENCE_MANAGER.lock().unwrap();
    manager.clone()
}

/// Initialize persistence manager with defaults
pub fn init_persistence_manager_with_defaults(project_path: PathBuf) -> PersistenceResult<()> {
    init_persistence_manager(PersistenceConfig::default(), project_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_persistence_manager_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = PersistenceConfig::default();
        let manager = PersistenceManager::new(config, temp_dir.path().to_path_buf()).unwrap();
        
        let metadata = ProjectMetadata {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            version: "1.0.0".to_string(),
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
            author: "Test Author".to_string(),
            description: Some("Test project description".to_string()),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
            structure_version: 1,
        };
        
        manager.initialize_project(metadata).await.unwrap();
    }
}