//! Project model for database storage
//! 
//! Represents a project in the multi-project management system

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Project model representing a logical grouping of documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique project identifier
    pub id: Uuid,
    
    /// Human-readable project name
    pub name: String,
    
    /// Optional project description
    pub description: Option<String>,
    
    /// Project creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Whether project is archived
    pub is_archived: bool,
    
    /// Active project flag (only one active project at a time)
    pub is_active: bool,
    
    /// Optional project settings as JSON
    pub settings: Option<String>,
}

impl Project {
    /// Create a new project with basic fields
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Project {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: now,
            updated_at: now,
            is_archived: false,
            is_active: false,
            settings: None,
        }
    }
    
    /// Create project with settings
    pub fn with_settings(mut self, settings: String) -> Self {
        self.settings = Some(settings);
        self.updated_at = Utc::now();
        self
    }
    
    /// Mark project as archived
    pub fn archive(mut self) -> Self {
        self.is_archived = true;
        self.is_active = false; // Deactivate archived projects
        self.updated_at = Utc::now();
        self
    }
    
    /// Mark project as active
    pub fn activate(mut self) -> Self {
        self.is_active = true;
        self.updated_at = Utc::now();
        self
    }
    
    /// Update project information
    pub fn update(mut self, name: Option<String>, description: Option<String>) -> Self {
        if let Some(new_name) = name {
            self.name = new_name;
        }
        if let Some(new_description) = description {
            self.description = Some(new_description);
        }
        self.updated_at = Utc::now();
        self
    }
}

/// Project statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatistics {
    /// Number of documents in project
    pub document_count: usize,
    
    /// Total word count across all documents
    pub total_words: usize,
    
    /// Project storage size in bytes
    pub storage_size: usize,
    
    /// Last document modification time
    pub last_document_update: Option<DateTime<Utc>>,
}

impl Default for ProjectStatistics {
    fn default() -> Self {
        ProjectStatistics {
            document_count: 0,
            total_words: 0,
            storage_size: 0,
            last_document_update: None,
        }
    }
}