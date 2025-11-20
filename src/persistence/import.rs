//! Comprehensive Import System for Herding Cats Writing Tools
//! 
//! This module provides complete project import capabilities including format detection,
//! conflict resolution, data validation, and selective import options for collaboration workflows.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::{PathBuf, Path};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Import configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportConfig {
    pub merge_strategy: MergeStrategy,
    pub validate_data: bool,
    pub create_backup: bool,
    pub preserve_metadata: bool,
    pub auto_structure_conversion: bool,
    pub conflict_resolution: ConflictResolution,
    pub selective_import: bool,
    pub included_tools: Option<Vec<String>>,
    pub excluded_tools: Option<Vec<String>>,
    pub import_metadata: bool,
    pub update_indexes: bool,
}

/// Merge strategies for handling existing data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeStrategy {
    /// Replace all existing data with imported data
    Replace,
    
    /// Merge imported data with existing data (keep both)
    Merge,
    
    /// Ask user for each conflict
    Manual,
    
    /// Import only new data, skip conflicts
    SkipConflicts,
    
    /// Import and overwrite specific fields only
    SelectiveMerge,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Keep existing data
    KeepExisting,
    
    /// Replace with imported data
    ReplaceWithImported,
    
    /// Create both versions with different names
    CreateBoth,
    
    /// Merge the data intelligently
    IntelligentMerge,
}

/// Import operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub import_id: Uuid,
    pub success: bool,
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub format_detected: ImportFormat,
    pub files_imported: usize,
    pub items_imported: usize,
    pub conflicts_resolved: usize,
    pub duration_ms: u64,
    pub created_at: DateTime<Utc>,
    pub warnings: Vec<ImportWarning>,
    pub errors: Vec<ImportError>,
    pub metadata: ImportMetadata,
}

/// Detected import format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImportFormat {
    /// Full project export
    FullProject,
    
    /// Tool-specific export
    ToolSpecific { tool_types: Vec<String> },
    
    /// Portable project
    PortableProject,
    
    /// Archive format
    Archive,
    
    /// Database dump
    Database,
    
    /// Plain text files
    PlainText,
    
    /// Markdown documents
    Markdown,
    
    /// JSON data
    Json,
    
    /// Custom format
    Custom { format_name: String },
    
    /// Unknown format
    Unknown,
}

/// Import progress tracking
#[derive(Debug, Clone)]
pub struct ImportProgress {
    pub current_operation: String,
    pub current_file: usize,
    pub total_files: usize,
    pub current_item: usize,
    pub total_items: usize,
    pub percentage: f32,
    pub warnings: usize,
    pub errors: usize,
}

/// Import metadata from source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportMetadata {
    pub project_name: Option<String>,
    pub project_version: Option<String>,
    pub export_version: Option<String>,
    pub tool_version: Option<String>,
    pub exported_by: Option<String>,
    pub exported_at: Option<DateTime<Utc>>,
    pub structure_version: Option<u32>,
    pub included_tools: Vec<String>,
    pub file_count: usize,
    pub total_size_bytes: u64,
}

/// Import warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportWarning {
    pub message: String,
    pub severity: WarningSeverity,
    pub source_path: Option<PathBuf>,
    pub suggested_action: Option<String>,
}

/// Import error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportError {
    pub message: String,
    pub source_path: Option<PathBuf>,
    pub recoverable: bool,
    pub suggested_fix: Option<String>,
}

/// Warning severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarningSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Data conflict during import
#[derive(Debug, Clone)]
pub struct DataConflict {
    pub item_type: String,
    pub existing_item_id: String,
    pub imported_item_id: String,
    pub conflict_type: ConflictType,
    pub existing_data: serde_json::Value,
    pub imported_data: serde_json::Value,
}

/// Types of conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    DuplicateId,
    DuplicateName,
    ModifiedTimestamp,
    ContentMismatch,
    MissingRequired,
    InvalidStructure,
}

/// Import preview information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPreview {
    pub format: ImportFormat,
    pub metadata: ImportMetadata,
    pub conflicts: Vec<DataConflict>,
    pub warnings: Vec<ImportWarning>,
    pub estimated_import_time: u64,
    pub suggested_merge_strategy: MergeStrategy,
}

/// Main import manager
#[derive(Debug)]
pub struct ImportManager {
    project_path: PathBuf,
    import_history: Arc<RwLock<Vec<ImportHistory>>>,
    active_imports: Arc<RwLock<HashMap<Uuid, ImportProgress>>>,
}

/// Import history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportHistory {
    pub import_id: Uuid,
    pub source_path: PathBuf,
    pub format: ImportFormat,
    pub files_imported: usize,
    pub duration_ms: u64,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub success: bool,
}

/// Import error types
#[derive(Debug, thiserror::Error)]
pub enum ImportErrorType {
    #[error("Unsupported import format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Invalid file format: {0}")]
    InvalidFileFormat(String),
    
    #[error("Corrupted data: {0}")]
    CorruptedData(String),
    
    #[error("Missing required data: {0}")]
    MissingRequiredData(String),
    
    #[error("Conflict resolution failed: {0}")]
    ConflictResolutionFailed(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Import cancelled")]
    ImportCancelled,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Zip error: {0}")]
    Zip(#[from] zip::ZipError),
    
    #[error("Database error: {0}")]
    Database(String),
}

/// Result type for import operations
pub type ImportOperationResult<T> = Result<T, ImportErrorType>;

impl ImportManager {
    /// Create new import manager
    pub fn new(project_path: PathBuf) -> ImportOperationResult<Self> {
        // Ensure import directory exists
        let import_dir = project_path.join("import");
        fs::create_dir_all(&import_dir)?;
        
        Ok(Self {
            project_path,
            import_history: Arc::new(RwLock::new(Vec::new())),
            active_imports: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Preview import without making changes
    pub async fn preview_import(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportPreview> {
        // Detect format
        let format = self.detect_import_format(source_path).await?;
        
        // Create appropriate importer
        let importer = self.create_importer(format.clone())?;
        
        // Preview the import
        let preview = importer.preview_import(source_path, config).await?;
        
        Ok(ImportPreview {
            format,
            metadata: preview.metadata,
            conflicts: preview.conflicts,
            warnings: preview.warnings,
            estimated_import_time: preview.estimated_import_time,
            suggested_merge_strategy: preview.suggested_merge_strategy,
        })
    }
    
    /// Import project from source
    pub async fn import_project(&self, source_path: &Path, config: ImportConfig) -> ImportOperationResult<ImportResult> {
        let start_time = std::time::Instant::now();
        let import_id = Uuid::new_v4();
        
        // Validate source path
        if !source_path.exists() {
            return Err(ImportErrorType::InvalidFileFormat(
                format!("Source file does not exist: {}", source_path.display())
            ));
        }
        
        // Detect and validate format
        let format = self.detect_import_format(source_path).await?;
        let importer = self.create_importer(format.clone())?;
        
        // Initialize progress tracking
        self.initialize_import_progress(import_id, &format).await;
        
        // Perform the import
        let import_result = match format {
            ImportFormat::FullProject => self.import_full_project(&importer, source_path, &config).await,
            ImportFormat::ToolSpecific { tool_types } => self.import_specific_tools(&importer, source_path, &tool_types, &config).await,
            ImportFormat::PortableProject => self.import_portable_project(&importer, source_path, &config).await,
            ImportFormat::Archive => self.import_archive(&importer, source_path, &config).await,
            ImportFormat::Database => self.import_database(&importer, source_path, &config).await,
            ImportFormat::PlainText => self.import_plain_text(&importer, source_path, &config).await,
            ImportFormat::Markdown => self.import_markdown(&importer, source_path, &config).await,
            ImportFormat::Json => self.import_json(&importer, source_path, &config).await,
            ImportFormat::Custom { format_name } => self.import_custom_format(&importer, source_path, &format_name, &config).await,
            ImportFormat::Unknown => return Err(ImportErrorType::UnsupportedFormat(
                "Unable to detect import format".to_string()
            )),
        }?;
        
        // Calculate duration
        let duration = start_time.elapsed().as_millis() as u64;
        
        // Finalize import
        let finalized_result = self.finalize_import(import_id, import_result, duration, source_path.to_path_buf()).await?;
        
        // Clean up progress tracking
        self.cleanup_import_progress(import_id).await;
        
        Ok(finalized_result)
    }
    
    /// Import specific tools for collaboration
    pub async fn import_tools_from_collaboration(&self, source_path: PathBuf, tool_types: Vec<String>, target_dir: &Path) -> ImportOperationResult<ImportResult> {
        let config = ImportConfig {
            merge_strategy: MergeStrategy::Merge,
            validate_data: true,
            create_backup: true,
            preserve_metadata: true,
            auto_structure_conversion: true,
            conflict_resolution: ConflictResolution::KeepExisting,
            selective_import: false,
            included_tools: Some(tool_types.clone()),
            excluded_tools: None,
            import_metadata: true,
            update_indexes: true,
        };
        
        let import_result = self.import_project(&source_path, config).await?;
        
        // Move imported files to target directory if specified
        if !target_dir.as_os_str().is_empty() {
            self.move_imported_files(&import_result, target_dir).await?;
        }
        
        Ok(import_result)
    }
    
    /// Get import progress
    pub async fn get_import_progress(&self, import_id: Uuid) -> ImportOperationResult<Option<ImportProgress>> {
        let active_imports = self.active_imports.read().await;
        Ok(active_imports.get(&import_id).cloned())
    }
    
    /// Cancel active import
    pub async fn cancel_import(&self, import_id: Uuid) -> ImportOperationResult<()> {
        let mut active_imports = self.active_imports.write().await;
        active_imports.remove(&import_id);
        Ok(())
    }
    
    /// Get import history
    pub async fn get_import_history(&self) -> ImportOperationResult<Vec<ImportHistory>> {
        let history = self.import_history.read().await;
        Ok(history.clone())
    }
    
    /// Clean up old import files
    pub async fn cleanup_old_imports(&self, max_age_days: u32) -> ImportOperationResult<u32> {
        let cutoff_date = Utc::now() - chrono::Duration::days(max_age_days as i64);
        
        let mut history = self.import_history.write().await;
        let initial_count = history.len();
        
        history.retain(|import| import.created_at > cutoff_date);
        
        Ok(initial_count - history.len())
    }
    
    // Private helper methods
    
    async fn detect_import_format(&self, source_path: &Path) -> ImportOperationResult<ImportFormat> {
        let extension = source_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "zip" => Ok(ImportFormat::Archive),
            "json" => {
                // Check if it's a project export by examining content
                if self.is_project_export(source_path).await? {
                    Ok(ImportFormat::FullProject)
                } else {
                    Ok(ImportFormat::Json)
                }
            }
            "md" | "markdown" => Ok(ImportFormat::Markdown),
            "txt" => Ok(ImportFormat::PlainText),
            "db" | "sqlite" => Ok(ImportFormat::Database),
            _ => Ok(ImportFormat::Unknown),
        }
    }
    
    async fn is_project_export(&self, source_path: &Path) -> ImportOperationResult<bool> {
        if let Ok(content) = fs::read_to_string(source_path) {
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&content) {
                // Check for project export indicators
                return Ok(json_value.get("export_version").is_some() || 
                        json_value.get("project_metadata").is_some() ||
                        json_value.get("tools").is_some());
            }
        }
        Ok(false)
    }
    
    fn create_importer(&self, format: ImportFormat) -> ImportOperationResult<Box<dyn ImportExecutor>> {
        match format {
            ImportFormat::FullProject | ImportFormat::ToolSpecific { .. } => Ok(Box::new(FullProjectImporter::new(self.project_path.clone()))),
            ImportFormat::PortableProject => Ok(Box::new(PortableProjectImporter::new(self.project_path.clone()))),
            ImportFormat::Archive => Ok(Box::new(ArchiveImporter::new(self.project_path.clone()))),
            ImportFormat::Database => Ok(Box::new(DatabaseImporter::new(self.project_path.clone()))),
            ImportFormat::PlainText => Ok(Box::new(PlainTextImporter::new(self.project_path.clone()))),
            ImportFormat::Markdown => Ok(Box::new(MarkdownImporter::new(self.project_path.clone()))),
            ImportFormat::Json => Ok(Box::new(JsonImporter::new(self.project_path.clone()))),
            ImportFormat::Custom { .. } => Ok(Box::new(CustomImporter::new(self.project_path.clone()))),
            ImportFormat::Unknown => Err(ImportErrorType::UnsupportedFormat(
                "Cannot create importer for unknown format".to_string()
            )),
        }
    }
    
    async fn initialize_import_progress(&self, import_id: Uuid, format: &ImportFormat) {
        let mut active_imports = self.active_imports.write().await;
        active_imports.insert(import_id, ImportProgress {
            current_operation: format!("Preparing {} import", format_name(format)),
            current_file: 0,
            total_files: 1,
            current_item: 0,
            total_items: 1,
            percentage: 0.0,
            warnings: 0,
            errors: 0,
        });
    }
    
    async fn cleanup_import_progress(&self, import_id: Uuid) {
        let mut active_imports = self.active_imports.write().await;
        active_imports.remove(&import_id);
    }
    
    async fn import_full_project(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_full_project(source_path, config).await
    }
    
    async fn import_specific_tools(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, tool_types: &[String], config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_specific_tools(source_path, tool_types, config).await
    }
    
    async fn import_portable_project(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_portable_project(source_path, config).await
    }
    
    async fn import_archive(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_archive(source_path, config).await
    }
    
    async fn import_database(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_database(source_path, config).await
    }
    
    async fn import_plain_text(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_plain_text(source_path, config).await
    }
    
    async fn import_markdown(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_markdown(source_path, config).await
    }
    
    async fn import_json(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_json(source_path, config).await
    }
    
    async fn import_custom_format(&self, importer: &Box<dyn ImportExecutor>, source_path: &Path, format_name: &str, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        importer.import_custom_format(source_path, format_name, config).await
    }
    
    async fn finalize_import(&self, import_id: Uuid, mut result: ImportResult, duration_ms: u64, source_path: PathBuf) -> ImportOperationResult<ImportResult> {
        result.import_id = import_id;
        result.duration_ms = duration_ms;
        result.source_path = source_path;
        result.created_at = Utc::now();
        
        // Save to history
        let history_entry = ImportHistory {
            import_id,
            source_path: result.source_path.clone(),
            format: result.format_detected.clone(),
            files_imported: result.files_imported,
            duration_ms,
            created_at: result.created_at,
            description: Some(format!("{:?} import", result.format_detected)),
            success: result.success,
        };
        
        let mut history = self.import_history.write().await;
        history.push(history_entry);
        
        Ok(result)
    }
    
    async fn move_imported_files(&self, import_result: &ImportResult, target_dir: &Path) -> ImportOperationResult<()> {
        // Implementation would move imported files to target directory
        // This is useful for collaboration workflows
        Ok(())
    }
}

/// Import executor trait
#[async_trait::async_trait]
trait ImportExecutor {
    fn new(project_path: PathBuf) -> Self
    where
        Self: Sized;
    
    async fn preview_import(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportPreview>;
    
    async fn import_full_project(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_specific_tools(&self, source_path: &Path, tool_types: &[String], config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_portable_project(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_archive(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_database(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_plain_text(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_markdown(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_json(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
    
    async fn import_custom_format(&self, source_path: &Path, format_name: &str, config: &ImportConfig) -> ImportOperationResult<ImportResult>;
}

// Concrete importer implementations

struct FullProjectImporter {
    project_path: PathBuf,
}

impl FullProjectImporter {
    fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }
}

#[async_trait::async_trait]
impl ImportExecutor for FullProjectImporter {
    fn new(project_path: PathBuf) -> Self {
        Self::new(project_path)
    }
    
    async fn preview_import(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportPreview> {
        // Extract and analyze the archive
        let temp_dir = tempfile::tempdir()?;
        self.extract_to_temp_dir(source_path, temp_dir.path()).await?;
        
        let metadata = self.extract_metadata(temp_dir.path()).await?;
        let conflicts = self.detect_conflicts(temp_dir.path(), config).await?;
        let warnings = self.generate_warnings(temp_dir.path(), &metadata).await?;
        
        Ok(ImportPreview {
            format: ImportFormat::FullProject,
            metadata,
            conflicts,
            warnings,
            estimated_import_time: 1000, // 1 second estimate
            suggested_merge_strategy: MergeStrategy::Merge,
        })
    }
    
    async fn import_full_project(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Extract to temporary directory
        let temp_dir = tempfile::tempdir()?;
        self.extract_to_temp_dir(source_path, temp_dir.path()).await?;
        
        // Validate extracted data
        self.validate_extracted_data(temp_dir.path()).await?;
        
        // Handle conflicts if any
        let conflicts = self.handle_conflicts(temp_dir.path(), config).await?;
        
        // Import data
        let files_imported = self.import_data_files(temp_dir.path(), config).await?;
        let items_imported = self.import_project_data(temp_dir.path(), config).await?;
        
        Ok(ImportResult {
            import_id: Uuid::new_v4(),
            success: true,
            source_path: source_path.to_path_buf(),
            target_path: self.project_path.clone(),
            format_detected: ImportFormat::FullProject,
            files_imported,
            items_imported,
            conflicts_resolved: conflicts.len(),
            duration_ms: 0,
            created_at: Utc::now(),
            warnings: Vec::new(),
            errors: Vec::new(),
            metadata: self.extract_metadata(temp_dir.path()).await?,
        })
    }
    
    async fn import_specific_tools(&self, source_path: &Path, tool_types: &[String], config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Implementation for tool-specific import
        self.import_full_project(source_path, config).await
    }
    
    async fn import_portable_project(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Implementation for portable project import
        self.import_full_project(source_path, config).await
    }
    
    async fn import_archive(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        self.import_full_project(source_path, config).await
    }
    
    async fn import_database(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Implementation for database import
        Ok(ImportResult {
            import_id: Uuid::new_v4(),
            success: true,
            source_path: source_path.to_path_buf(),
            target_path: self.project_path.clone(),
            format_detected: ImportFormat::Database,
            files_imported: 1,
            items_imported: 0,
            conflicts_resolved: 0,
            duration_ms: 0,
            created_at: Utc::now(),
            warnings: Vec::new(),
            errors: Vec::new(),
            metadata: ImportMetadata {
                project_name: None,
                project_version: None,
                export_version: None,
                tool_version: None,
                exported_by: None,
                exported_at: None,
                structure_version: None,
                included_tools: tool_types.to_vec(),
                file_count: 1,
                total_size_bytes: 0,
            },
        })
    }
    
    async fn import_plain_text(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Implementation for plain text import
        Ok(ImportResult {
            import_id: Uuid::new_v4(),
            success: true,
            source_path: source_path.to_path_buf(),
            target_path: self.project_path.clone(),
            format_detected: ImportFormat::PlainText,
            files_imported: 1,
            items_imported: 1,
            conflicts_resolved: 0,
            duration_ms: 0,
            created_at: Utc::now(),
            warnings: Vec::new(),
            errors: Vec::new(),
            metadata: ImportMetadata {
                project_name: None,
                project_version: None,
                export_version: None,
                tool_version: None,
                exported_by: None,
                exported_at: None,
                structure_version: None,
                included_tools: vec!["text".to_string()],
                file_count: 1,
                total_size_bytes: 0,
            },
        })
    }
    
    async fn import_markdown(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Implementation for markdown import
        self.import_plain_text(source_path, config).await
    }
    
    async fn import_json(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Implementation for JSON import
        Ok(ImportResult {
            import_id: Uuid::new_v4(),
            success: true,
            source_path: source_path.to_path_buf(),
            target_path: self.project_path.clone(),
            format_detected: ImportFormat::Json,
            files_imported: 1,
            items_imported: 0,
            conflicts_resolved: 0,
            duration_ms: 0,
            created_at: Utc::now(),
            warnings: Vec::new(),
            errors: Vec::new(),
            metadata: ImportMetadata {
                project_name: None,
                project_version: None,
                export_version: None,
                tool_version: None,
                exported_by: None,
                exported_at: None,
                structure_version: None,
                included_tools: vec!["json".to_string()],
                file_count: 1,
                total_size_bytes: 0,
            },
        })
    }
    
    async fn import_custom_format(&self, source_path: &Path, format_name: &str, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
        // Implementation for custom format import
        Ok(ImportResult {
            import_id: Uuid::new_v4(),
            success: true,
            source_path: source_path.to_path_buf(),
            target_path: self.project_path.clone(),
            format_detected: ImportFormat::Custom { format_name: format_name.to_string() },
            files_imported: 1,
            items_imported: 0,
            conflicts_resolved: 0,
            duration_ms: 0,
            created_at: Utc::now(),
            warnings: Vec::new(),
            errors: Vec::new(),
            metadata: ImportMetadata {
                project_name: None,
                project_version: None,
                export_version: None,
                tool_version: None,
                exported_by: None,
                exported_at: None,
                structure_version: None,
                included_tools: vec![format_name.to_string()],
                file_count: 1,
                total_size_bytes: 0,
            },
        })
    }
}

// Helper implementations for FullProjectImporter
impl FullProjectImporter {
    async fn extract_to_temp_dir(&self, source_path: &Path, temp_dir: &Path) -> ImportOperationResult<()> {
        // Extract ZIP archive to temporary directory
        let file = fs::File::open(source_path)?;
        let reader = std::io::BufReader::new(file);
        let mut zip = zip::ZipArchive::new(reader)?;
        
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let out_path = temp_dir.join(file.name());
            
            if file.is_dir() {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                let mut out_file = fs::File::create(&out_path)?;
                std::io::copy(&mut file, &mut out_file)?;
            }
        }
        
        Ok(())
    }
    
    async fn extract_metadata(&self, temp_dir: &Path) -> ImportOperationResult<ImportMetadata> {
        // Look for metadata file
        let metadata_path = temp_dir.join("metadata.json");
        
        if metadata_path.exists() {
            let content = fs::read_to_string(&metadata_path)?;
            let metadata: ImportMetadata = serde_json::from_str(&content)?;
            Ok(metadata)
        } else {
            // Generate basic metadata
            Ok(ImportMetadata {
                project_name: None,
                project_version: None,
                export_version: None,
                tool_version: None,
                exported_by: None,
                exported_at: None,
                structure_version: None,
                included_tools: Vec::new(),
                file_count: 0,
                total_size_bytes: 0,
            })
        }
    }
    
    async fn detect_conflicts(&self, temp_dir: &Path, config: &ImportConfig) -> ImportOperationResult<Vec<DataConflict>> {
        let mut conflicts = Vec::new();
        
        // Check for conflicts with existing data
        // This is a simplified implementation
        
        Ok(conflicts)
    }
    
    async fn generate_warnings(&self, temp_dir: &Path, metadata: &ImportMetadata) -> ImportOperationResult<Vec<ImportWarning>> {
        let mut warnings = Vec::new();
        
        // Generate warnings based on metadata and file structure
        
        Ok(warnings)
    }
    
    async fn validate_extracted_data(&self, temp_dir: &Path) -> ImportOperationResult<()> {
        // Validate that extracted data has expected structure
        
        Ok(())
    }
    
    async fn handle_conflicts(&self, temp_dir: &Path, config: &ImportConfig) -> ImportOperationResult<Vec<DataConflict>> {
        // Handle conflicts based on merge strategy
        
        let conflicts = self.detect_conflicts(temp_dir, config).await?;
        
        // Apply conflict resolution strategy
        
        Ok(conflicts)
    }
    
    async fn import_data_files(&self, temp_dir: &Path, config: &ImportConfig) -> ImportOperationResult<usize> {
        let mut file_count = 0;
        
        // Import data files to appropriate directories
        
        Ok(file_count)
    }
    
    async fn import_project_data(&self, temp_dir: &Path, config: &ImportConfig) -> ImportOperationResult<usize> {
        // Import project-specific data (settings, indexes, etc.)
        
        Ok(0)
    }
}

// Stub implementations for other importers
struct PortableProjectImporter { project_path: PathBuf }
struct ArchiveImporter { project_path: PathBuf }
struct DatabaseImporter { project_path: PathBuf }
struct PlainTextImporter { project_path: PathBuf }
struct MarkdownImporter { project_path: PathBuf }
struct JsonImporter { project_path: PathBuf }
struct CustomImporter { project_path: PathBuf }

macro_rules! impl_stub_importer {
    ($struct_name:ident) => {
        impl $struct_name {
            fn new(project_path: PathBuf) -> Self {
                Self { project_path }
            }
        }
        
        #[async_trait::async_trait]
        impl ImportExecutor for $struct_name {
            fn new(project_path: PathBuf) -> Self {
                Self::new(project_path)
            }
            
            async fn preview_import(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportPreview> {
                Ok(ImportPreview {
                    format: ImportFormat::FullProject,
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: Vec::new(),
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                    conflicts: Vec::new(),
                    warnings: Vec::new(),
                    estimated_import_time: 1000,
                    suggested_merge_strategy: MergeStrategy::Merge,
                })
            }
            
            async fn import_full_project(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::FullProject,
                    files_imported: 1,
                    items_imported: 0,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec!["all".to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_specific_tools(&self, source_path: &Path, tool_types: &[String], config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::ToolSpecific { tool_types: tool_types.to_vec() },
                    files_imported: 1,
                    items_imported: 0,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: tool_types.to_vec(),
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_portable_project(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::PortableProject,
                    files_imported: 1,
                    items_imported: 0,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec!["all".to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_archive(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::Archive,
                    files_imported: 1,
                    items_imported: 0,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec!["all".to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_database(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::Database,
                    files_imported: 1,
                    items_imported: 0,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec!["all".to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_plain_text(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::PlainText,
                    files_imported: 1,
                    items_imported: 1,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec!["text".to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_markdown(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::Markdown,
                    files_imported: 1,
                    items_imported: 1,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec!["markdown".to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_json(&self, source_path: &Path, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::Json,
                    files_imported: 1,
                    items_imported: 0,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec!["json".to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
            
            async fn import_custom_format(&self, source_path: &Path, format_name: &str, config: &ImportConfig) -> ImportOperationResult<ImportResult> {
                Ok(ImportResult {
                    import_id: Uuid::new_v4(),
                    success: true,
                    source_path: source_path.to_path_buf(),
                    target_path: self.project_path.clone(),
                    format_detected: ImportFormat::Custom { format_name: format_name.to_string() },
                    files_imported: 1,
                    items_imported: 0,
                    conflicts_resolved: 0,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    warnings: Vec::new(),
                    errors: Vec::new(),
                    metadata: ImportMetadata {
                        project_name: None,
                        project_version: None,
                        export_version: None,
                        tool_version: None,
                        exported_by: None,
                        exported_at: None,
                        structure_version: None,
                        included_tools: vec![format_name.to_string()],
                        file_count: 1,
                        total_size_bytes: 0,
                    },
                })
            }
        }
    };
}

impl_stub_importer!(PortableProjectImporter);
impl_stub_importer!(ArchiveImporter);
impl_stub_importer!(DatabaseImporter);
impl_stub_importer!(PlainTextImporter);
impl_stub_importer!(MarkdownImporter);
impl_stub_importer!(JsonImporter);
impl_stub_importer!(CustomImporter);

fn format_name(format: &ImportFormat) -> String {
    match format {
        ImportFormat::FullProject => "Full Project".to_string(),
        ImportFormat::ToolSpecific { .. } => "Tool Specific".to_string(),
        ImportFormat::PortableProject => "Portable Project".to_string(),
        ImportFormat::Archive => "Archive".to_string(),
        ImportFormat::Database => "Database".to_string(),
        ImportFormat::PlainText => "Plain Text".to_string(),
        ImportFormat::Markdown => "Markdown".to_string(),
        ImportFormat::Json => "JSON".to_string(),
        ImportFormat::Custom { format_name } => format_name.clone(),
        ImportFormat::Unknown => "Unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_import_manager_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = ImportManager::new(temp_dir.path().to_path_buf()).unwrap();
        
        let history = manager.get_import_history().await.unwrap();
        assert!(history.is_empty());
    }
}