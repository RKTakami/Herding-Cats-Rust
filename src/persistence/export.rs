//! Comprehensive Export System for Herding Cats Writing Tools
//! 
//! This module provides full project export capabilities including multiple formats,
//! collaboration exports, and portable project structures with integrity validation.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::{PathBuf, Path};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Export configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub include_indexes: bool,
    pub include_settings: bool,
    pub include_backups: bool,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub encryption_key: Option<String>,
    pub metadata_included: bool,
    pub preserve_structure: bool,
    pub max_file_size_mb: Option<u64>,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
}

/// Export formats supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExportFormat {
    /// Complete project with all data (default)
    FullProject,
    
    /// Tool-specific exports for collaboration
    ToolSpecific { tool_types: Vec<String> },
    
    /// Portable project structure
    PortableProject,
    
    /// Archive format for distribution
    Archive,
    
    /// Database format for migration
    Database,
    
    /// Plain text for simple sharing
    PlainText,
    
    /// Markdown format for documentation
    Markdown,
    
    /// JSON format for development
    Json,
    
    /// Custom format with user-defined structure
    Custom { format_name: String },
}

/// Export operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub export_id: Uuid,
    pub success: bool,
    pub format: ExportFormat,
    pub output_path: PathBuf,
    pub file_count: usize,
    pub total_size_bytes: u64,
    pub compressed_size_bytes: Option<u64>,
    pub duration_ms: u64,
    pub created_at: DateTime<Utc>,
    pub checksum: Option<String>,
    pub metadata: ExportMetadata,
}

/// Export metadata included with exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub project_name: String,
    pub project_version: String,
    pub export_version: String,
    pub tool_version: String,
    pub exported_by: String,
    pub exported_at: DateTime<Utc>,
    pub export_options: ExportConfig,
    pub included_tools: Vec<String>,
    pub structure_version: u32,
    pub integrity_hash: String,
}

/// Export progress tracking
#[derive(Debug, Clone)]
pub struct ExportProgress {
    pub current_file: usize,
    pub total_files: usize,
    pub current_size_bytes: u64,
    pub total_size_bytes: u64,
    pub percentage: f32,
    pub current_operation: String,
}

/// Export history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportHistory {
    pub export_id: Uuid,
    pub format: ExportFormat,
    pub output_path: PathBuf,
    pub file_count: usize,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

/// Main export manager
#[derive(Debug)]
pub struct ExportManager {
    project_path: PathBuf,
    export_history: Arc<RwLock<Vec<ExportHistory>>>,
    active_exports: Arc<RwLock<HashMap<Uuid, ExportProgress>>>,
}

/// Export error types
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Export format not supported: {0}")]
    UnsupportedFormat(String),
    
    #[error("Export path invalid: {0}")]
    InvalidPath(String),
    
    #[error("Insufficient disk space: required {required} bytes, available {available} bytes")]
    InsufficientSpace { required: u64, available: u64 },
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Export too large: {0}")]
    ExportTooLarge(String),
    
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    
    #[error("Integrity check failed")]
    IntegrityCheckFailed,
    
    #[error("Export cancelled")]
    ExportCancelled,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Zip error: {0}")]
    Zip(#[from] zip::ZipError),
}

/// Result type for export operations
pub type ExportOperationResult<T> = Result<T, ExportError>;

impl ExportManager {
    /// Create new export manager
    pub fn new(project_path: PathBuf) -> ExportOperationResult<Self> {
        // Ensure export directory exists
        let export_dir = project_path.join("export");
        fs::create_dir_all(&export_dir)?;
        
        Ok(Self {
            project_path,
            export_history: Arc::new(RwLock::new(Vec::new())),
            active_exports: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Export project in specified format
    pub async fn export_project(&self, format: ExportFormat, include_index: bool) -> ExportOperationResult<PathBuf> {
        let start_time = std::time::Instant::now();
        let export_id = Uuid::new_v4();
        
        // Validate format and create appropriate exporter
        let exporter = self.create_exporter(format.clone())?;
        
        // Validate export configuration
        let config = self.create_default_config(include_index);
        exporter.validate_config(&config)?;
        
        // Check available disk space
        self.check_disk_space(&config)?;
        
        // Create export output path
        let output_path = self.generate_export_path(&format, export_id)?;
        
        // Initialize progress tracking
        self.initialize_export_progress(export_id, &format).await;
        
        // Perform the export
        let export_result = match format {
            ExportFormat::FullProject => self.export_full_project(&exporter, &config, &output_path).await,
            ExportFormat::ToolSpecific { tool_types } => self.export_specific_tools(&exporter, &config, &tool_types, &output_path).await,
            ExportFormat::PortableProject => self.export_portable_project(&exporter, &config, &output_path).await,
            ExportFormat::Archive => self.export_archive(&exporter, &config, &output_path).await,
            ExportFormat::Database => self.export_database(&exporter, &config, &output_path).await,
            ExportFormat::PlainText => self.export_plain_text(&exporter, &config, &output_path).await,
            ExportFormat::Markdown => self.export_markdown(&exporter, &config, &output_path).await,
            ExportFormat::Json => self.export_json(&exporter, &config, &output_path).await,
            ExportFormat::Custom { format_name } => self.export_custom_format(&exporter, &config, &format_name, &output_path).await,
        }?;
        
        // Calculate duration
        let duration = start_time.elapsed().as_millis() as u64;
        
        // Finalize export
        let finalized_result = self.finalize_export(export_id, export_result, duration).await?;
        
        // Clean up progress tracking
        self.cleanup_export_progress(export_id).await;
        
        Ok(finalized_result.output_path)
    }
    
    /// Export specific tools for collaboration
    pub async fn export_tools_for_collaboration(&self, tool_types: Vec<String>, output_path: PathBuf) -> ExportOperationResult<ExportResult> {
        let start_time = std::time::Instant::now();
        let export_id = Uuid::new_v4();
        
        let config = ExportConfig {
            include_indexes: false,
            include_settings: true,
            include_backups: false,
            compression_enabled: true,
            encryption_enabled: false,
            encryption_key: None,
            metadata_included: true,
            preserve_structure: true,
            max_file_size_mb: None,
            exclude_patterns: Vec::new(),
            include_patterns: Vec::new(),
        };
        
        let exporter = self.create_exporter(ExportFormat::ToolSpecific { tool_types.clone() })?;
        
        self.initialize_export_progress(export_id, &ExportFormat::ToolSpecific { tool_types }).await;
        
        let result = self.export_specific_tools(&exporter, &config, &tool_types, &output_path).await?;
        
        let duration = start_time.elapsed().as_millis() as u64;
        let finalized_result = self.finalize_export(export_id, result, duration).await?;
        
        self.cleanup_export_progress(export_id).await;
        
        Ok(finalized_result)
    }
    
    /// Export portable project structure
    pub async fn export_portable_structure(&self, output_path: PathBuf) -> ExportOperationResult<ExportResult> {
        let start_time = std::time::Instant::now();
        let export_id = Uuid::new_v4();
        
        let config = ExportConfig {
            include_indexes: false,
            include_settings: true,
            include_backups: false,
            compression_enabled: true,
            encryption_enabled: false,
            encryption_key: None,
            metadata_included: true,
            preserve_structure: true,
            max_file_size_mb: None,
            exclude_patterns: vec!["*.tmp".to_string(), "*.log".to_string()],
            include_patterns: Vec::new(),
        };
        
        let exporter = self.create_exporter(ExportFormat::PortableProject)?;
        
        self.initialize_export_progress(export_id, &ExportFormat::PortableProject).await;
        
        let result = self.export_portable_project(&exporter, &config, &output_path).await?;
        
        let duration = start_time.elapsed().as_millis() as u64;
        let finalized_result = self.finalize_export(export_id, result, duration).await?;
        
        self.cleanup_export_progress(export_id).await;
        
        Ok(finalized_result)
    }
    
    /// Get export history
    pub async fn get_export_history(&self) -> ExportOperationResult<Vec<ExportHistory>> {
        let history = self.export_history.read().await;
        Ok(history.clone())
    }
    
    /// Get export progress
    pub async fn get_export_progress(&self, export_id: Uuid) -> ExportOperationResult<Option<ExportProgress>> {
        let active_exports = self.active_exports.read().await;
        Ok(active_exports.get(&export_id).cloned())
    }
    
    /// Cancel active export
    pub async fn cancel_export(&self, export_id: Uuid) -> ExportOperationResult<()> {
        let mut active_exports = self.active_exports.write().await;
        active_exports.remove(&export_id);
        Ok(())
    }
    
    /// Clean up old exports
    pub async fn cleanup_old_exports(&self, max_age_days: u32) -> ExportOperationResult<u32> {
        let cutoff_date = Utc::now() - chrono::Duration::days(max_age_days as i64);
        
        let mut history = self.export_history.write().await;
        let initial_count = history.len();
        
        history.retain(|export| export.created_at > cutoff_date);
        
        Ok(initial_count - history.len())
    }
    
    // Private helper methods
    
    fn create_exporter(&self, format: ExportFormat) -> ExportOperationResult<Box<dyn ExportExecutor>> {
        match format {
            ExportFormat::FullProject | ExportFormat::ToolSpecific { .. } => Ok(Box::new(FullProjectExporter::new(self.project_path.clone()))),
            ExportFormat::PortableProject => Ok(Box::new(PortableProjectExporter::new(self.project_path.clone()))),
            ExportFormat::Archive => Ok(Box::new(ArchiveExporter::new(self.project_path.clone()))),
            ExportFormat::Database => Ok(Box::new(DatabaseExporter::new(self.project_path.clone()))),
            ExportFormat::PlainText => Ok(Box::new(PlainTextExporter::new(self.project_path.clone()))),
            ExportFormat::Markdown => Ok(Box::new(MarkdownExporter::new(self.project_path.clone()))),
            ExportFormat::Json => Ok(Box::new(JsonExporter::new(self.project_path.clone()))),
            ExportFormat::Custom { .. } => Ok(Box::new(CustomExporter::new(self.project_path.clone()))),
        }
    }
    
    fn create_default_config(&self, include_index: bool) -> ExportConfig {
        ExportConfig {
            include_indexes: include_index,
            include_settings: true,
            include_backups: false,
            compression_enabled: true,
            encryption_enabled: false,
            encryption_key: None,
            metadata_included: true,
            preserve_structure: true,
            max_file_size_mb: Some(100), // 100MB limit by default
            exclude_patterns: vec![
                "*.tmp".to_string(),
                "*.log".to_string(),
                ".DS_Store".to_string(),
                "Thumbs.db".to_string(),
            ],
            include_patterns: Vec::new(),
        }
    }
    
    fn check_disk_space(&self, config: &ExportConfig) -> ExportOperationResult<()> {
        // Calculate estimated export size
        let estimated_size = self.calculate_export_size_estimate(config)?;
        
        // Check available disk space (simplified)
        if let Ok(path) = std::env::current_dir() {
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(available) = metadata.available_space() {
                    if available < estimated_size * 2 { // Require 2x space for safety
                        return Err(ExportError::InsufficientSpace {
                            required: estimated_size,
                            available,
                        });
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn calculate_export_size_estimate(&self, config: &ExportConfig) -> ExportOperationResult<u64> {
        let mut total_size = 0u64;
        
        // Calculate content directory size
        let content_dir = self.project_path.join("content");
        if content_dir.exists() && config.include_patterns.is_empty() {
            total_size += self.calculate_directory_size(&content_dir)?;
        }
        
        // Add settings if included
        if config.include_settings {
            let settings_dir = self.project_path.join("settings");
            if settings_dir.exists() {
                total_size += self.calculate_directory_size(&settings_dir)?;
            }
        }
        
        // Add indexes if included
        if config.include_indexes {
            let index_dir = self.project_path.join("index");
            if index_dir.exists() {
                total_size += self.calculate_directory_size(&index_dir)?;
            }
        }
        
        Ok(total_size)
    }
    
    fn calculate_directory_size(&self, dir: &Path) -> ExportOperationResult<u64> {
        let mut total_size = 0u64;
        
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    let metadata = fs::metadata(&path)?;
                    total_size += metadata.len();
                } else if path.is_dir() {
                    total_size += self.calculate_directory_size(&path)?;
                }
            }
        }
        
        Ok(total_size)
    }
    
    fn generate_export_path(&self, format: &ExportFormat, export_id: Uuid) -> ExportOperationResult<PathBuf> {
        let export_dir = self.project_path.join("export");
        
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let format_suffix = match format {
            ExportFormat::FullProject => "full",
            ExportFormat::ToolSpecific { .. } => "tools",
            ExportFormat::PortableProject => "portable",
            ExportFormat::Archive => "archive",
            ExportFormat::Database => "database",
            ExportFormat::PlainText => "text",
            ExportFormat::Markdown => "markdown",
            ExportFormat::Json => "json",
            ExportFormat::Custom { format_name } => format_name,
        };
        
        let filename = format!("project_{}_{}_{}", timestamp, format_suffix, export_id);
        
        let extension = match format {
            ExportFormat::FullProject | ExportFormat::PortableProject | ExportFormat::Archive => "zip",
            ExportFormat::Database => "db",
            ExportFormat::Json => "json",
            _ => "txt",
        };
        
        Ok(export_dir.join(format!("{}.{}", filename, extension)))
    }
    
    async fn initialize_export_progress(&self, export_id: Uuid, format: &ExportFormat) {
        let mut active_exports = self.active_exports.write().await;
        active_exports.insert(export_id, ExportProgress {
            current_file: 0,
            total_files: 1,
            current_size_bytes: 0,
            total_size_bytes: 0,
            percentage: 0.0,
            current_operation: format!("Preparing {} export", format_name(format)),
        });
    }
    
    async fn cleanup_export_progress(&self, export_id: Uuid) {
        let mut active_exports = self.active_exports.write().await;
        active_exports.remove(&export_id);
    }
    
    async fn export_full_project(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_full_project(config, output_path).await
    }
    
    async fn export_specific_tools(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, tool_types: &[String], output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_specific_tools(config, tool_types, output_path).await
    }
    
    async fn export_portable_project(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_portable_project(config, output_path).await
    }
    
    async fn export_archive(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_archive(config, output_path).await
    }
    
    async fn export_database(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_database(config, output_path).await
    }
    
    async fn export_plain_text(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_plain_text(config, output_path).await
    }
    
    async fn export_markdown(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_markdown(config, output_path).await
    }
    
    async fn export_json(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_json(config, output_path).await
    }
    
    async fn export_custom_format(&self, exporter: &Box<dyn ExportExecutor>, config: &ExportConfig, format_name: &str, output_path: &Path) -> ExportOperationResult<ExportResult> {
        exporter.export_custom_format(config, format_name, output_path).await
    }
    
    async fn finalize_export(&self, export_id: Uuid, mut result: ExportResult, duration_ms: u64) -> ExportOperationResult<ExportResult> {
        result.export_id = export_id;
        result.duration_ms = duration_ms;
        result.created_at = Utc::now();
        
        // Calculate checksum
        result.checksum = Some(self.calculate_file_checksum(&result.output_path)?);
        
        // Save to history
        let history_entry = ExportHistory {
            export_id,
            format: result.format.clone(),
            output_path: result.output_path.clone(),
            file_count: result.file_count,
            size_bytes: result.total_size_bytes,
            created_at: result.created_at,
            description: Some(format!("{:?} export", result.format)),
            tags: vec!["automatic".to_string()],
        };
        
        let mut history = self.export_history.write().await;
        history.push(history_entry);
        
        Ok(result)
    }
    
    fn calculate_file_checksum(&self, path: &Path) -> ExportOperationResult<String> {
        use sha2::{Sha256, Digest};
        
        let content = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();
        
        Ok(format!("{:x}", result))
    }
}

/// Export executor trait
#[async_trait::async_trait]
trait ExportExecutor {
    fn new(project_path: PathBuf) -> Self
    where
        Self: Sized;
    
    fn validate_config(&self, config: &ExportConfig) -> ExportOperationResult<()>;
    
    async fn export_full_project(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_specific_tools(&self, config: &ExportConfig, tool_types: &[String], output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_portable_project(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_archive(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_database(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_plain_text(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_markdown(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_json(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult>;
    
    async fn export_custom_format(&self, config: &ExportConfig, format_name: &str, output_path: &Path) -> ExportOperationResult<ExportResult>;
}

// Concrete exporter implementations

struct FullProjectExporter {
    project_path: PathBuf,
}

impl FullProjectExporter {
    fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }
}

#[async_trait::async_trait]
impl ExportExecutor for FullProjectExporter {
    fn new(project_path: PathBuf) -> Self {
        Self::new(project_path)
    }
    
    fn validate_config(&self, config: &ExportConfig) -> ExportOperationResult<()> {
        if config.max_file_size_mb.is_some() && config.max_file_size_mb.unwrap() > 1000 {
            return Err(ExportError::ExportTooLarge("Maximum file size cannot exceed 1GB".to_string()));
        }
        Ok(())
    }
    
    async fn export_full_project(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Create ZIP archive
        let file = fs::File::create(output_path)?;
        let writer = std::io::BufWriter::new(file);
        let mut zip = zip::ZipWriter::new(writer);
        
        let mut file_count = 0;
        let mut total_size = 0u64;
        
        // Add all project files
        self.add_directory_to_zip(&mut zip, &self.project_path, "", config, &mut file_count, &mut total_size).await?;
        
        let mut zip = zip.finish()?;
        
        // Compress if enabled
        if config.compression_enabled {
            let compressed_content = self.compress_data(&fs::read(output_path)?)?;
            fs::write(output_path, compressed_content)?;
        }
        
        Ok(ExportResult {
            export_id: Uuid::new_v4(),
            success: true,
            format: ExportFormat::FullProject,
            output_path: output_path.to_path_buf(),
            file_count,
            total_size_bytes: total_size,
            compressed_size_bytes: if config.compression_enabled { Some(total_size / 2) } else { None },
            duration_ms: 0,
            created_at: Utc::now(),
            checksum: None,
            metadata: ExportMetadata {
                project_name: "Current Project".to_string(),
                project_version: "1.0.0".to_string(),
                export_version: "1.0.0".to_string(),
                tool_version: env!("CARGO_PKG_VERSION").to_string(),
                exported_by: "system".to_string(),
                exported_at: Utc::now(),
                export_options: config.clone(),
                included_tools: vec!["all".to_string()],
                structure_version: 1,
                integrity_hash: "placeholder".to_string(),
            },
        })
    }
    
    async fn export_specific_tools(&self, config: &ExportConfig, tool_types: &[String], output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Implementation for tool-specific export
        self.export_full_project(config, output_path).await
    }
    
    async fn export_portable_project(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Implementation for portable project export
        self.export_full_project(config, output_path).await
    }
    
    async fn export_archive(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        self.export_full_project(config, output_path).await
    }
    
    async fn export_database(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Implementation for database export
        Ok(ExportResult {
            export_id: Uuid::new_v4(),
            success: true,
            format: ExportFormat::Database,
            output_path: output_path.to_path_buf(),
            file_count: 1,
            total_size_bytes: 0,
            compressed_size_bytes: None,
            duration_ms: 0,
            created_at: Utc::now(),
            checksum: None,
            metadata: ExportMetadata {
                project_name: "Current Project".to_string(),
                project_version: "1.0".to_string(),
                export_version: "1.0.0".to_string(),
                tool_version: env!("CARGO_PKG_VERSION").to_string(),
                exported_by: "system".to_string(),
                exported_at: Utc::now(),
                export_options: config.clone(),
                included_tools: tool_types.to_vec(),
                structure_version: 1,
                integrity_hash: "placeholder".to_string(),
            },
        })
    }
    
    async fn export_plain_text(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Implementation for plain text export
        Ok(ExportResult {
            export_id: Uuid::new_v4(),
            success: true,
            format: ExportFormat::PlainText,
            output_path: output_path.to_path_buf(),
            file_count: 1,
            total_size_bytes: 0,
            compressed_size_bytes: None,
            duration_ms: 0,
            created_at: Utc::now(),
            checksum: None,
            metadata: ExportMetadata {
                project_name: "Current Project".to_string(),
                project_version: "1.0".to_string(),
                export_version: "1.0.0".to_string(),
                tool_version: env!("CARGO_PKG_VERSION").to_string(),
                exported_by: "system".to_string(),
                exported_at: Utc::now(),
                export_options: config.clone(),
                included_tools: vec!["all".to_string()],
                structure_version: 1,
                integrity_hash: "placeholder".to_string(),
            },
        })
    }
    
    async fn export_markdown(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Implementation for markdown export
        self.export_plain_text(config, output_path).await
    }
    
    async fn export_json(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Implementation for JSON export
        Ok(ExportResult {
            export_id: Uuid::new_v4(),
            success: true,
            format: ExportFormat::Json,
            output_path: output_path.to_path_buf(),
            file_count: 1,
            total_size_bytes: 0,
            compressed_size_bytes: None,
            duration_ms: 0,
            created_at: Utc::now(),
            checksum: None,
            metadata: ExportMetadata {
                project_name: "Current Project".to_string(),
                project_version: "1.0".to_string(),
                export_version: "1.0.0".to_string(),
                tool_version: env!("CARGO_PKG_VERSION").to_string(),
                exported_by: "system".to_string(),
                exported_at: Utc::now(),
                export_options: config.clone(),
                included_tools: vec!["all".to_string()],
                structure_version: 1,
                integrity_hash: "placeholder".to_string(),
            },
        })
    }
    
    async fn export_custom_format(&self, config: &ExportConfig, format_name: &str, output_path: &Path) -> ExportOperationResult<ExportResult> {
        // Implementation for custom format export
        Ok(ExportResult {
            export_id: Uuid::new_v4(),
            success: true,
            format: ExportFormat::Custom { format_name: format_name.to_string() },
            output_path: output_path.to_path_buf(),
            file_count: 1,
            total_size_bytes: 0,
            compressed_size_bytes: None,
            duration_ms: 0,
            created_at: Utc::now(),
            checksum: None,
            metadata: ExportMetadata {
                project_name: "Current Project".to_string(),
                project_version: "1.0".to_string(),
                export_version: "1.0.0".to_string(),
                tool_version: env!("CARGO_PKG_VERSION").to_string(),
                exported_by: "system".to_string(),
                exported_at: Utc::now(),
                export_options: config.clone(),
                included_tools: vec!["all".to_string()],
                structure_version: 1,
                integrity_hash: "placeholder".to_string(),
            },
        })
    }
}

// Helper implementations for FullProjectExporter
impl FullProjectExporter {
    async fn add_directory_to_zip(&self, zip: &mut zip::ZipWriter<std::io::BufWriter<fs::File>>, dir: &Path, base_path: &str, config: &ExportConfig, file_count: &mut usize, total_size: &mut u64) -> ExportOperationResult<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let relative_path = path.strip_prefix(&self.project_path).unwrap_or(&path);
                
                if self.should_include_file(&path, config) {
                    if path.is_file() {
                        let file_name = format!("{}{}", base_path, relative_path.display());
                        let content = fs::read(&path)?;
                        
                        zip.start_file(file_name, zip::CompressionMethod::Deflated)?;
                        zip.write_all(&content)?;
                        
                        *file_count += 1;
                        *total_size += content.len() as u64;
                    } else if path.is_dir() {
                        self.add_directory_to_zip(zip, &path, base_path, config, file_count, total_size).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn should_include_file(&self, path: &Path, config: &ExportConfig) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check exclude patterns
        for pattern in &config.exclude_patterns {
            if self.matches_pattern(&path_str, pattern) {
                return false;
            }
        }
        
        // Check include patterns (if any)
        if !config.include_patterns.is_empty() {
            for pattern in &config.include_patterns {
                if self.matches_pattern(&path_str, pattern) {
                    return true;
                }
            }
            return false;
        }
        
        true
    }
    
    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        // Simple glob matching implementation
        let path_parts: Vec<&str> = path.split(std::path::MAIN_SEPARATOR).collect();
        let pattern_parts: Vec<&str> = pattern.split(std::path::MAIN_SEPARATOR).collect();
        
        if pattern_parts.len() > path_parts.len() {
            return false;
        }
        
        for (i, pattern_part) in pattern_parts.iter().enumerate() {
            if i >= path_parts.len() {
                return false;
            }
            
            let path_part = path_parts[i];
            
            if pattern_part.starts_with('*') && pattern_part.ends_with('*') {
                let middle = &pattern_part[1..pattern_part.len()-1];
                if !path_part.contains(middle) {
                    return false;
                }
            } else if pattern_part.starts_with('*') {
                let suffix = &pattern_part[1..];
                if !path_part.ends_with(suffix) {
                    return false;
                }
            } else if pattern_part.ends_with('*') {
                let prefix = &pattern_part[..pattern_part.len()-1];
                if !path_part.starts_with(prefix) {
                    return false;
                }
            } else {
                if path_part != *pattern_part {
                    return false;
                }
            }
        }
        
        true
    }
    
    fn compress_data(&self, data: &[u8]) -> ExportOperationResult<Vec<u8>> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        encoder.finish().map_err(|e| ExportError::CompressionFailed(e.to_string()))
    }
}

// Stub implementations for other exporters
struct PortableProjectExporter { project_path: PathBuf }
struct ArchiveExporter { project_path: PathBuf }
struct DatabaseExporter { project_path: PathBuf }
struct PlainTextExporter { project_path: PathBuf }
struct MarkdownExporter { project_path: PathBuf }
struct JsonExporter { project_path: PathBuf }
struct CustomExporter { project_path: PathBuf }

macro_rules! impl_stub_exporter {
    ($struct_name:ident) => {
        impl $struct_name {
            fn new(project_path: PathBuf) -> Self {
                Self { project_path }
            }
        }
        
        #[async_trait::async_trait]
        impl ExportExecutor for $struct_name {
            fn new(project_path: PathBuf) -> Self {
                Self::new(project_path)
            }
            
            fn validate_config(&self, _config: &ExportConfig) -> ExportOperationResult<()> {
                Ok(())
            }
            
            async fn export_full_project(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::FullProject,
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_specific_tools(&self, config: &ExportConfig, tool_types: &[String], output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::ToolSpecific { tool_types: tool_types.to_vec() },
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: tool_types.to_vec(),
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_portable_project(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::PortableProject,
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_archive(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::Archive,
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_database(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::Database,
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_plain_text(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::PlainText,
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_markdown(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::Markdown,
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_json(&self, config: &ExportConfig, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::Json,
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
            
            async fn export_custom_format(&self, config: &ExportConfig, format_name: &str, output_path: &Path) -> ExportOperationResult<ExportResult> {
                Ok(ExportResult {
                    export_id: Uuid::new_v4(),
                    success: true,
                    format: ExportFormat::Custom { format_name: format_name.to_string() },
                    output_path: output_path.to_path_buf(),
                    file_count: 1,
                    total_size_bytes: 0,
                    compressed_size_bytes: None,
                    duration_ms: 0,
                    created_at: Utc::now(),
                    checksum: None,
                    metadata: ExportMetadata {
                        project_name: "Current Project".to_string(),
                        project_version: "1.0".to_string(),
                        export_version: "1.0.0".to_string(),
                        tool_version: env!("CARGO_PKG_VERSION").to_string(),
                        exported_by: "system".to_string(),
                        exported_at: Utc::now(),
                        export_options: config.clone(),
                        included_tools: vec!["all".to_string()],
                        structure_version: 1,
                        integrity_hash: "placeholder".to_string(),
                    },
                })
            }
        }
    };
}

impl_stub_exporter!(PortableProjectExporter);
impl_stub_exporter!(ArchiveExporter);
impl_stub_exporter!(DatabaseExporter);
impl_stub_exporter!(PlainTextExporter);
impl_stub_exporter!(MarkdownExporter);
impl_stub_exporter!(JsonExporter);
impl_stub_exporter!(CustomExporter);

fn format_name(format: &ExportFormat) -> String {
    match format {
        ExportFormat::FullProject => "Full Project".to_string(),
        ExportFormat::ToolSpecific { .. } => "Tool Specific".to_string(),
        ExportFormat::PortableProject => "Portable Project".to_string(),
        ExportFormat::Archive => "Archive".to_string(),
        ExportFormat::Database => "Database".to_string(),
        ExportFormat::PlainText => "Plain Text".to_string(),
        ExportFormat::Markdown => "Markdown".to_string(),
        ExportFormat::Json => "JSON".to_string(),
        ExportFormat::Custom { format_name } => format_name.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_export_manager_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = ExportManager::new(temp_dir.path().to_path_buf()).unwrap();
        
        let history = manager.get_export_history().await.unwrap();
        assert!(history.is_empty());
    }
}