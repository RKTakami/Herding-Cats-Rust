use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::Instant;
use uuid::Uuid;

use crate::database::{DatabaseError, DatabaseResult, EnhancedDatabaseService};

/// Backup types supported by the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupType {
    Manual,
    Automatic,
    Emergency,
}

/// Backup metadata stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub id: String,
    pub backup_type: BackupType,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub checksum: String,
    pub created_at: u64,
    pub project_id: Option<String>,
    pub description: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Statistics about backups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStatistics {
    pub total_backups: usize,
    pub total_size: u64,
    pub last_backup: Option<u64>,
    pub successful_backups: usize,
    pub failed_backups: usize,
}

/// Backup service with comprehensive functionality
#[derive(Debug)]
pub struct BackupService {
    db_service: Arc<tokio::sync::RwLock<EnhancedDatabaseService>>,
    backup_directory: PathBuf,
}

impl BackupService {
    /// Create a new backup service
    pub fn new(
        db_service: Arc<tokio::sync::RwLock<EnhancedDatabaseService>>,
        database_path: &Path,
    ) -> Self {
        let backup_directory = database_path
            .parent()
            .unwrap_or_else(|| Path::new("data"))
            .join("backups");

        Self {
            db_service,
            backup_directory,
        }
    }

    /// Initialize backup service - create backup directory and setup
    pub async fn initialize(&self) -> DatabaseResult<()> {
        // Create backup directory if it doesn't exist
        tokio::fs::create_dir_all(&self.backup_directory)
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to create backup directory: {}", e))
            })?;

        // Initialize backup metadata table
        self.initialize_backup_metadata_table().await?;

        Ok(())
    }

    /// Create a manual backup
    pub async fn create_manual_backup(
        &self,
        project_id: Option<&str>,
        description: Option<&str>,
    ) -> DatabaseResult<String> {
        self.create_backup(BackupType::Manual, project_id, description)
            .await
    }

    /// Create an automatic backup
    pub async fn create_automatic_backup(
        &self,
        project_id: Option<&str>,
    ) -> DatabaseResult<String> {
        self.create_backup(BackupType::Automatic, project_id, None)
            .await
    }

    /// Create an emergency backup
    pub async fn create_emergency_backup(
        &self,
        project_id: Option<&str>,
        description: Option<&str>,
    ) -> DatabaseResult<String> {
        self.create_backup(BackupType::Emergency, project_id, description)
            .await
    }

    /// Create a backup with specified type
    async fn create_backup(
        &self,
        backup_type: BackupType,
        project_id: Option<&str>,
        description: Option<&str>,
    ) -> DatabaseResult<String> {
        let start_time = Instant::now();

        // Get database path
        let db_path = {
            let db = self.db_service.read().await;
            db.get_database_path().to_path_buf()
        };

        // Generate backup filename
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let backup_id = Uuid::new_v4();
        let backup_filename = format!("{}_{}.db", timestamp, backup_id);
        let backup_path = self.backup_directory.join(&backup_filename);

        // Copy database file
        match fs::copy(&db_path, &backup_path) {
            Ok(_bytes_copied) => {
                // Calculate checksum
                let checksum = self.calculate_file_checksum(&backup_path).await?;

                // Get file size
                let metadata = fs::metadata(&backup_path).map_err(|e| {
                    DatabaseError::Service(format!("Failed to get file metadata: {}", e))
                })?;
                let file_size = metadata.len();

                // Store backup metadata in database
                let metadata = BackupMetadata {
                    id: backup_id.to_string(),
                    backup_type: backup_type.clone(),
                    file_path: backup_path.clone(),
                    file_size,
                    checksum,
                    created_at: timestamp,
                    project_id: project_id.map(|s| s.to_string()),
                    description: description.map(|s| s.to_string()),
                    success: true,
                    error_message: None,
                };

                self.store_backup_metadata(&metadata).await?;

                // Clean up old automatic backups if needed
                if matches!(backup_type, BackupType::Automatic) {
                    self.cleanup_old_backups().await?;
                }

                tracing::info!(
                    "Backup created successfully: {} in {:?}",
                    backup_filename,
                    start_time.elapsed()
                );
                Ok(backup_id.to_string())
            }
            Err(e) => {
                // Store failed backup metadata
                let metadata = BackupMetadata {
                    id: backup_id.to_string(),
                    backup_type,
                    file_path: backup_path,
                    file_size: 0,
                    checksum: String::new(),
                    created_at: timestamp,
                    project_id: project_id.map(|s| s.to_string()),
                    description: description.map(|s| s.to_string()),
                    success: false,
                    error_message: Some(e.to_string()),
                };

                self.store_backup_metadata(&metadata).await?;

                tracing::error!("Backup failed: {}", e);
                Err(DatabaseError::Service(format!(
                    "Backup creation failed: {}",
                    e
                )))
            }
        }
    }

    /// List all backups with optional filtering
    pub async fn list_backups(
        &self,
        project_id: Option<&str>,
        limit: Option<usize>,
    ) -> DatabaseResult<Vec<BackupMetadata>> {
        let db = self.db_service.read().await;

        let limit = limit.unwrap_or(50);
        let project_id_str = project_id.map(|s| s.to_string());

        let query = if project_id_str.is_some() {
            "SELECT id, backup_type, file_path, file_size, checksum, created_at,
                    project_id, description, success, error_message
             FROM backup_metadata
             WHERE project_id = ?
             ORDER BY created_at DESC
             LIMIT ?"
        } else {
            "SELECT id, backup_type, file_path, file_size, checksum, created_at,
                    project_id, description, success, error_message
             FROM backup_metadata
             ORDER BY created_at DESC
             LIMIT ?"
        };

        let params = if let Some(pid) = &project_id_str {
            vec![pid.clone(), limit.to_string()]
        } else {
            vec![limit.to_string()]
        };

        let rows = db.query(query, &params).await?;

        let mut results = Vec::new();
        for row in rows {
            let backup_type_str = row.get(1).unwrap_or("\"Manual\"");
            let backup_type: BackupType =
                serde_json::from_str(backup_type_str).unwrap_or(BackupType::Manual);

            let metadata = BackupMetadata {
                id: row.get(0).unwrap_or("").to_string(),
                backup_type,
                file_path: PathBuf::from(row.get(2).unwrap_or("")),
                file_size: row.get(3).and_then(|s| s.parse().ok()).unwrap_or(0),
                checksum: row.get(4).unwrap_or("").to_string(),
                created_at: row.get(5).and_then(|s| s.parse().ok()).unwrap_or(0),
                project_id: row.get(6).map(|s| s.to_string()),
                description: row.get(7).map(|s| s.to_string()),
                success: row.get(8).map(|s| s == "true" || s == "1").unwrap_or(false),
                error_message: row.get(9).map(|s| s.to_string()),
            };
            results.push(metadata);
        }

        Ok(results)
    }

    /// Delete a specific backup
    pub async fn delete_backup(&self, backup_id: &str) -> DatabaseResult<()> {
        let db = self.db_service.read().await;

        // Get backup metadata
        let backup_rows = db
            .query(
                "SELECT file_path FROM backup_metadata WHERE id = ?",
                &[backup_id.to_string()],
            )
            .await?;

        // Delete physical backup file
        if let Some(row) = backup_rows.rows.first() {
            if let Some(path_str) = row.get(0) {
                let path = PathBuf::from(path_str);
                if path.exists() {
                    fs::remove_file(&path).map_err(|e| {
                        DatabaseError::Service(format!("Failed to delete backup file: {}", e))
                    })?;
                }
            }
        }

        // Delete metadata from database
        db.execute(
            "DELETE FROM backup_metadata WHERE id = ?",
            &[backup_id.to_string()],
        )
        .await?;

        Ok(())
    }

    /// Restore from a backup
    pub async fn restore_from_backup(&self, backup_id: &str) -> DatabaseResult<()> {
        let db = self.db_service.read().await;

        // Get backup metadata
        let backup_rows = db
            .query(
                "SELECT file_path FROM backup_metadata WHERE id = ? AND success = 1",
                &[backup_id.to_string()],
            )
            .await?;

        let backup_path = if let Some(row) = backup_rows.rows.first() {
            PathBuf::from(row.get(0).unwrap_or(""))
        } else {
            return Err(DatabaseError::Service("Backup file not found".to_string()));
        };

        if !backup_path.exists() {
            return Err(DatabaseError::Service("Backup file not found".to_string()));
        }

        // Get current database path
        let current_db_path = db.get_database_path().to_path_buf();

        // Copy backup to current database location
        fs::copy(&backup_path, &current_db_path)
            .map_err(|e| DatabaseError::Service(format!("Failed to restore backup: {}", e)))?;

        Ok(())
    }

    /// Clean up old backups based on retention policy
    pub async fn cleanup_old_backups(&self) -> DatabaseResult<()> {
        let db = self.db_service.read().await;

        // Keep only the last 30 automatic backups per project
        // This is a simplified version - in production you'd want more sophisticated retention
        db.execute(
            "DELETE FROM backup_metadata
             WHERE backup_type = 'Automatic'
             AND created_at < (
                 SELECT created_at FROM backup_metadata
                 WHERE backup_type = 'Automatic'
                 ORDER BY created_at DESC
                 LIMIT 1 OFFSET 30
             )",
            &[],
        )
        .await?;

        tracing::info!("Cleaned up old automatic backups");

        Ok(())
    }

    /// Get backup statistics
    pub async fn get_backup_statistics(
        &self,
        project_id: Option<&str>,
    ) -> DatabaseResult<BackupStatistics> {
        let db = self.db_service.read().await;

        let query = if let Some(_pid) = project_id {
            "SELECT COUNT(*), COALESCE(SUM(file_size), 0), MAX(created_at),
                    SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END),
                    SUM(CASE WHEN success = 0 THEN 1 ELSE 0 END)
             FROM backup_metadata
             WHERE project_id = ?"
        } else {
            "SELECT COUNT(*), COALESCE(SUM(file_size), 0), MAX(created_at),
                    SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END),
                    SUM(CASE WHEN success = 0 THEN 1 ELSE 0 END)
             FROM backup_metadata"
        };

        let params = if let Some(pid) = project_id {
            vec![pid.to_string()]
        } else {
            vec![]
        };

        let rows = db.query(query, &params).await?;

        if let Some(row) = rows.rows.first() {
            let total_backups: usize = row.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
            let total_size: u64 = row.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let last_backup: Option<u64> = row.get(2).and_then(|s| s.parse().ok());
            let successful_backups: usize = row.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
            let failed_backups: usize = row.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);

            Ok(BackupStatistics {
                total_backups,
                total_size,
                last_backup,
                successful_backups,
                failed_backups,
            })
        } else {
            Ok(BackupStatistics {
                total_backups: 0,
                total_size: 0,
                last_backup: None,
                successful_backups: 0,
                failed_backups: 0,
            })
        }
    }

    /// Calculate SHA-256 checksum of a file
    async fn calculate_file_checksum(&self, file_path: &Path) -> DatabaseResult<String> {
        let content = tokio::fs::read(file_path).await.map_err(|e| {
            DatabaseError::Service(format!("Failed to read file for checksum: {}", e))
        })?;

        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }

    /// Initialize backup metadata table (now handled by schema.sql)
    async fn initialize_backup_metadata_table(&self) -> DatabaseResult<()> {
        // Table creation is now handled by schema.sql initialization
        Ok(())
    }

    /// Store backup metadata in database
    async fn store_backup_metadata(&self, metadata: &BackupMetadata) -> DatabaseResult<()> {
        let db = self.db_service.read().await;

        let backup_type_str = serde_json::to_string(&metadata.backup_type).map_err(|e| {
            DatabaseError::Service(format!("Failed to serialize backup type: {}", e))
        })?;

        db.execute(
            "INSERT OR REPLACE INTO backup_metadata
             (id, backup_type, file_path, file_size, checksum, created_at,
              project_id, description, success, error_message)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                metadata.id.clone(),
                backup_type_str,
                metadata.file_path.to_str().unwrap_or("").to_string(),
                metadata.file_size.to_string(),
                metadata.checksum.clone(),
                metadata.created_at.to_string(),
                metadata.project_id.as_deref().unwrap_or("").to_string(),
                metadata.description.as_deref().unwrap_or("").to_string(),
                metadata.success.to_string(),
                metadata.error_message.as_deref().unwrap_or("").to_string(),
            ],
        )
        .await?;

        Ok(())
    }
}
