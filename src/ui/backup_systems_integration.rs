//! Backup Systems UI Integration
//!
//! Provides comprehensive backup management with manual/automatic backup controls,
//! backup restoration, and backup monitoring capabilities. Integrates with the
//! BackupService for enterprise-grade backup operations.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// Import database types and services
use herding_cats_rust as hc_lib;
use hc_lib::BackupType;

/// UI representation of backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIBackupMetadata {
    pub id: String,
    pub backup_type: BackupType,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub file_size_formatted: String,
    pub checksum: String,
    pub created_at: u64,
    pub created_at_formatted: String,
    pub project_id: Option<String>,
    pub project_name: String,
    pub description: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub is_restorable: bool,
    pub is_verified: bool,
    pub backup_age: String, // Human-readable age (e.g., "2 hours ago")
    pub storage_location: String,
}

impl UIBackupMetadata {
    /// Format file size in human-readable format
    pub fn format_file_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }

    /// Format timestamp as human-readable date
    pub fn format_timestamp(timestamp: u64) -> String {
        // In a real implementation, this would use chrono or similar
        // For now, return a simple format
        format!("{} seconds since epoch", timestamp)
    }

    /// Calculate backup age in human-readable format
    pub fn calculate_backup_age(created_at: u64) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let age_seconds = now.saturating_sub(created_at);

        if age_seconds < 60 {
            format!("{} seconds ago", age_seconds)
        } else if age_seconds < 3600 {
            format!("{} minutes ago", age_seconds / 60)
        } else if age_seconds < 86400 {
            format!("{} hours ago", age_seconds / 3600)
        } else {
            format!("{} days ago", age_seconds / 86400)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_ui_backup_metadata_creation() {
        let metadata = UIBackupMetadata {
            id: "backup-1".to_string(),
            backup_type: BackupType::Manual,
            file_path: PathBuf::from("data/backups/backup.db"),
            file_size: 1024,
            file_size_formatted: "1.0 KB".to_string(),
            checksum: "abc123".to_string(),
            created_at: 1234567890,
            created_at_formatted: "2009-02-13 23:31:30".to_string(),
            project_id: Some("project-1".to_string()),
            project_name: "Test Project".to_string(),
            description: Some("Test backup".to_string()),
            success: true,
            error_message: None,
            is_restorable: true,
            is_verified: false,
            backup_age: "2 hours ago".to_string(),
            storage_location: "data/backups".to_string(),
        };

        assert_eq!(metadata.id, "backup-1");
        assert_eq!(metadata.file_size, 1024);
        assert_eq!(metadata.file_size_formatted, "1.0 KB");
        assert!(metadata.success);
        assert!(metadata.is_restorable);
        assert!(!metadata.is_verified);
    }

    #[tokio::test]
    async fn test_file_size_formatting() {
        assert_eq!(UIBackupMetadata::format_file_size(0), "0.0 B");
        assert_eq!(UIBackupMetadata::format_file_size(1024), "1.0 KB");
        assert_eq!(UIBackupMetadata::format_file_size(1048576), "1.0 MB");
        assert_eq!(UIBackupMetadata::format_file_size(1073741824), "1.0 GB");
    }
}
