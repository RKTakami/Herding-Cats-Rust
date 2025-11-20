/// Project-Wide Backup System
/// Provides comprehensive backup, recovery, and synchronization capabilities

use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

/// Backup types and strategies
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackupType {
    Full,           // Complete project backup
    Incremental,    // Only changes since last backup
    Differential,   // Changes since last full backup
    Selective,      // Specific tools/sections
}

/// Backup status tracking
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackupStatus {
    Pending,
    InProgress { progress: f32 },
    Completed,
    Failed { error: String },
    Cancelled,
}

/// Cloud storage provider types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudProvider {
    Local,
    Dropbox,
    GoogleDrive,
    OneDrive,
    Custom { name: String, config: HashMap<String, String> },
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub backup_type: BackupType,
    pub cloud_provider: Option<CloudProvider>,
    pub local_backup_dir: PathBuf,
    pub max_local_backups: usize,
    pub auto_backup_interval: Option<chrono::Duration>,
    pub encryption_enabled: bool,
    pub compression_enabled: bool,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
}

/// Represents a single backup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJob {
    pub id: Uuid,
    pub name: String,
    pub backup_type: BackupType,
    pub status: BackupStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub source_path: PathBuf,
    pub destination_path: Option<PathBuf>,
    pub cloud_destination: Option<CloudProvider>,
    pub file_count: usize,
    pub total_size_bytes: u64,
    pub compressed_size_bytes: Option<u64>,
    pub checksum: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Backup and synchronization manager
pub struct BackupManager {
    config: Arc<RwLock<BackupConfig>>,
    backup_jobs: Arc<RwLock<HashMap<Uuid, BackupJob>>>,
    active_operations: Arc<Mutex<HashMap<Uuid, BackupJob>>>,
    observers: Arc<Mutex<Vec<BackupObserver>>>,
}

/// Observer trait for backup events
pub trait BackupObserver: Send + Sync {
    fn on_backup_started(&self, job: &BackupJob);
    fn on_backup_progress(&self, job: &BackupJob, progress: f32);
    fn on_backup_completed(&self, job: &BackupJob);
    fn on_backup_failed(&self, job: &BackupJob, error: &str);
}

impl BackupManager {
    /// Create new backup manager
    pub fn new() -> Self {
        let default_config = BackupConfig {
            backup_type: BackupType::Incremental,
            cloud_provider: None,
            local_backup_dir: PathBuf::from("backups"),
            max_local_backups: 10,
            auto_backup_interval: Some(chrono::Duration::hours(1)),
            encryption_enabled: true,
            compression_enabled: true,
            exclude_patterns: vec![
                "*.tmp".to_string(),
                "*.log".to_string(),
                ".git".to_string(),
                "target".to_string(),
            ],
            include_patterns: vec![],
        };

        Self {
            config: Arc::new(RwLock::new(default_config)),
            backup_jobs: Arc::new(RwLock::new(HashMap::new())),
            active_operations: Arc::new(Mutex::new(HashMap::new())),
            observers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register an observer for backup events
    pub fn register_observer(&self, observer: Box<dyn BackupObserver>) {
        let mut observers = self.observers.lock().unwrap();
        observers.push(observer);
    }

    /// Configure backup settings
    pub async fn configure(&self, config: BackupConfig) {
        let mut current_config = self.config.write().await;
        *current_config = config;
    }

    /// Create a new backup job
    pub async fn create_backup_job(&self, name: String, backup_type: BackupType, source_path: PathBuf) -> Uuid {
        let job_id = Uuid::new_v4();
        let config = self.config.read().await;

        let backup_job = BackupJob {
            id: job_id,
            name,
            backup_type,
            status: BackupStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            source_path,
            destination_path: Some(config.local_backup_dir.join(format!("backup_{}", job_id))),
            cloud_destination: config.cloud_provider.clone(),
            file_count: 0,
            total_size_bytes: 0,
            compressed_size_bytes: None,
            checksum: None,
            metadata: HashMap::new(),
        };

        let mut backup_jobs = self.backup_jobs.write().await;
        backup_jobs.insert(job_id, backup_job);

        job_id
    }

    /// Execute a backup job
    pub async fn execute_backup(&self, job_id: &Uuid) -> Result<(), BackupError> {
        let mut backup_jobs = self.backup_jobs.write().await;
        let mut active_operations = self.active_operations.lock().unwrap();
        
        if let Some(job) = backup_jobs.get_mut(job_id) {
            if job.status != BackupStatus::Pending {
                return Err(BackupError::InvalidState("Job already started".to_string()));
            }

            // Mark as active
            job.status = BackupStatus::InProgress { progress: 0.0 };
            job.started_at = Some(Utc::now());
            let job_clone = job.clone();
            active_operations.insert(*job_id, job_clone);

            drop(backup_jobs);
            drop(active_operations);

            // Notify observers
            self.notify_backup_started(job_id);

            // Execute backup in background task
            let job_id_clone = *job_id;
            let backup_manager_clone = Arc::new(self.clone());
            
            tokio::spawn(async move {
                if let Err(e) = backup_manager_clone.perform_backup(&job_id_clone).await {
                    backup_manager_clone.handle_backup_error(&job_id_clone, &e).await;
                }
            });

            Ok(())
        } else {
            Err(BackupError::NotFound("Backup job not found".to_string()))
        }
    }

    /// Cancel an active backup job
    pub async fn cancel_backup(&self, job_id: &Uuid) -> Result<(), BackupError> {
        let mut backup_jobs = self.backup_jobs.write().await;
        let mut active_operations = self.active_operations.lock().unwrap();
        
        if let Some(job) = backup_jobs.get_mut(job_id) {
            if matches!(job.status, BackupStatus::InProgress { .. }) {
                job.status = BackupStatus::Cancelled;
                active_operations.remove(job_id);
                Ok(())
            } else {
                Err(BackupError::InvalidState("Job not in progress".to_string()))
            }
        } else {
            Err(BackupError::NotFound("Backup job not found".to_string()))
        }
    }

    /// List all backup jobs
    pub async fn list_backups(&self) -> Vec<BackupJob> {
        let backup_jobs = self.backup_jobs.read().await;
        backup_jobs.values().cloned().collect()
    }

    /// Get backup job status
    pub async fn get_backup_status(&self, job_id: &Uuid) -> Option<BackupStatus> {
        let backup_jobs = self.backup_jobs.read().await;
        backup_jobs.get(job_id).map(|job| job.status.clone())
    }

    // Private helper methods

    async fn perform_backup(&self, job_id: &Uuid) -> Result<(), BackupError> {
        let backup_jobs = self.backup_jobs.read().await;
        let job = backup_jobs.get(job_id)
            .ok_or_else(|| BackupError::NotFound("Backup job not found".to_string()))?;
        
        let config = self.config.read().await;

        // Create backup directory
        if let Some(dest_path) = &job.destination_path {
            fs::create_dir_all(dest_path)?;
        }

        // Perform the actual backup
        let files = self.scan_source_directory(&job.source_path, &config).await?;
        
        let mut processed_files = 0;
        let total_files = files.len();
        let mut total_size = 0u64;

        for file_info in files {
            if !matches!(job.status, BackupStatus::InProgress { .. }) {
                break; // Job was cancelled
            }

            self.copy_file_with_options(&file_info, &job.destination_path, &config).await?;
            
            processed_files += 1;
            total_size += file_info.size_bytes;
            
            // Update progress
            let progress = processed_files as f32 / total_files as f32;
            self.update_backup_progress(job_id, progress).await;

            // Small delay to prevent overwhelming the system
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        // Mark as completed
        self.complete_backup(job_id, total_size, processed_files).await?;

        Ok(())
    }

    async fn scan_source_directory(&self, source_path: &Path, config: &BackupConfig) -> Result<Vec<BackupFile>, BackupError> {
        let mut files = Vec::new();
        
        if source_path.is_dir() {
            for entry in fs::read_dir(source_path)? {
                let entry = entry?;
                let path = entry.path();
                
                if self.should_include_file(&path, config) {
                    if path.is_file() {
                        let metadata = fs::metadata(&path)?;
                        let checksum = self.calculate_checksum(&path).await?;
                        
                        files.push(BackupFile {
                            path: path.clone(),
                            size_bytes: metadata.len(),
                            modified_at: metadata.modified()?.into(),
                            checksum,
                            compression_ratio: None,
                            is_encrypted: config.encryption_enabled,
                            tool_type: self.detect_tool_type(&path),
                        });
                    } else if path.is_dir() {
                        // Recursively scan subdirectories
                        let subdir_files = self.scan_source_directory(&path, config).await?;
                        files.extend(subdir_files);
                    }
                }
            }
        }

        Ok(files)
    }

    async fn copy_file_with_options(&self, file: &BackupFile, dest_dir: &Option<PathBuf>, config: &BackupConfig) -> Result<(), BackupError> {
        if let Some(dest_path) = dest_dir {
            let relative_path = file.path.strip_prefix(&file.path).unwrap_or(&file.path);
            let dest_file_path = dest_path.join(relative_path);
            
            // Create parent directory
            if let Some(parent) = dest_file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let source_content = fs::read(&file.path)?;
            let final_content = if config.compression_enabled {
                self.compress_data(&source_content)?
            } else {
                source_content
            };

            let final_content = if config.encryption_enabled {
                self.encrypt_data(&final_content)?
            } else {
                final_content
            };

            fs::write(dest_file_path, final_content)?;
        }

        Ok(())
    }

    fn should_include_file(&self, path: &Path, config: &BackupConfig) -> bool {
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
        // Simple glob matching - could be enhanced with regex
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
                // Middle match
                let middle = &pattern_part[1..pattern_part.len()-1];
                if !path_part.contains(middle) {
                    return false;
                }
            } else if pattern_part.starts_with('*') {
                // Suffix match
                let suffix = &pattern_part[1..];
                if !path_part.ends_with(suffix) {
                    return false;
                }
            } else if pattern_part.ends_with('*') {
                // Prefix match
                let prefix = &pattern_part[..pattern_part.len()-1];
                if !path_part.starts_with(prefix) {
                    return false;
                }
            } else {
                // Exact match
                if path_part != *pattern_part {
                    return false;
                }
            }
        }
        
        true
    }

    async fn calculate_checksum(&self, path: &Path) -> Result<String, BackupError> {
        use sha2::{Sha256, Digest};
        
        let content = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();
        
        Ok(format!("{:x}", result))
    }

    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, BackupError> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        encoder.finish().map_err(|e| BackupError::CompressionFailed(e.to_string()))
    }

    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, BackupError> {
        // Placeholder for encryption - would implement actual encryption here
        // For now, just return the data as-is
        Ok(data.to_vec())
    }

    fn detect_tool_type(&self, path: &Path) -> Option<String> {
        let path_str = path.to_string_lossy();
        
        if path_str.contains("hierarchy") {
            Some("hierarchy".to_string())
        } else if path_str.contains("codex") {
            Some("codex".to_string())
        } else if path_str.contains("notes") {
            Some("notes".to_string())
        } else if path_str.contains("research") {
            Some("research".to_string())
        } else if path_str.contains("plot") {
            Some("plot".to_string())
        } else if path_str.contains("analysis") {
            Some("analysis".to_string())
        } else {
            None
        }
    }

    async fn update_backup_progress(&self, job_id: &Uuid, progress: f32) {
        let mut backup_jobs = self.backup_jobs.write().await;
        if let Some(job) = backup_jobs.get_mut(job_id) {
            if let BackupStatus::InProgress { .. } = &mut job.status {
                job.status = BackupStatus::InProgress { progress };
                self.notify_backup_progress(job, progress);
            }
        }
    }

    async fn complete_backup(&self, job_id: &Uuid, total_size: u64, file_count: usize) -> Result<(), BackupError> {
        let mut backup_jobs = self.backup_jobs.write().await;
        let mut active_operations = self.active_operations.lock().unwrap();
        
        if let Some(job) = backup_jobs.get_mut(job_id) {
            job.status = BackupStatus::Completed;
            job.completed_at = Some(Utc::now());
            job.total_size_bytes = total_size;
            job.file_count = file_count;
            
            // Remove from active operations
            active_operations.remove(job_id);
            
            // Notify observers
            self.notify_backup_completed(job);
        }
        
        Ok(())
    }

    async fn handle_backup_error(&self, job_id: &Uuid, error: &BackupError) {
        let mut backup_jobs = self.backup_jobs.write().await;
        let mut active_operations = self.active_operations.lock().unwrap();
        
        if let Some(job) = backup_jobs.get_mut(job_id) {
            job.status = BackupStatus::Failed { error: error.to_string() };
            active_operations.remove(job_id);
            self.notify_backup_failed(job, &error.to_string());
        }
    }

    fn notify_backup_started(&self, job_id: &Uuid) {
        let backup_jobs = self.backup_jobs.read().unwrap();
        if let Some(job) = backup_jobs.get(job_id) {
            let observers = self.observers.lock().unwrap();
            for observer in &*observers {
                observer.on_backup_started(job);
            }
        }
    }

    fn notify_backup_progress(&self, job: &BackupJob, progress: f32) {
        let observers = self.observers.lock().unwrap();
        for observer in &*observers {
            observer.on_backup_progress(job, progress);
        }
    }

    fn notify_backup_completed(&self, job: &BackupJob) {
        let observers = self.observers.lock().unwrap();
        for observer in &*observers {
            observer.on_backup_completed(job);
        }
    }

    fn notify_backup_failed(&self, job: &BackupJob, error: &str) {
        let observers = self.observers.lock().unwrap();
        for observer in &*observers {
            observer.on_backup_failed(job, error);
        }
    }
}

impl Clone for BackupManager {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            backup_jobs: self.backup_jobs.clone(),
            active_operations: self.active_operations.clone(),
            observers: self.observers.clone(),
        }
    }
}

/// Backup file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupFile {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified_at: DateTime<Utc>,
    pub checksum: String,
    pub compression_ratio: Option<f32>,
    pub is_encrypted: bool,
    pub tool_type: Option<String>,
}

/// Error types for backup operations
#[derive(Debug, thiserror::Error)]
pub enum BackupError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
}

/// Global singleton for backup management
use once_cell::sync::Lazy;
pub static BACKUP_MANAGER: Lazy<BackupManager> = Lazy::new(BackupManager::new);

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_backup_creation() {
        let manager = BackupManager::new();
        
        let job_id = manager.create_backup_job(
            "Test Backup".to_string(),
            BackupType::Full,
            PathBuf::from("test_data"),
        ).await;
        
        assert!(!job_id.is_nil());
    }
}