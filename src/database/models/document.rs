//! Document model for database storage
//! 
//! Represents documents associated with projects

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Document model representing a text document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique document identifier
    pub id: Uuid,
    
    /// Parent project identifier
    pub project_id: Uuid,
    
    /// Human-readable document title
    pub title: String,
    
    /// Optional document content (may be stored in chunks)
    pub content: Option<String>,
    
    /// Document type (markdown, plain_text, etc.)
    pub document_type: String,
    
    /// Word count for performance tracking
    pub word_count: usize,
    
    /// SHA-256 checksum for integrity verification
    pub checksum: String,
    
    /// Document creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Whether document is active/not deleted
    pub is_active: bool,
    
    /// Document version for tracking changes
    pub version: u32,
    
    /// Optional metadata as JSON
    pub metadata: Option<String>,
}

impl Document {
    /// Create a new document with basic fields
    pub fn new(project_id: Uuid, title: String, content: Option<String>, document_type: String) -> Self {
        let now = Utc::now();
        Document {
            id: Uuid::new_v4(),
            project_id,
            title,
            content,
            document_type,
            word_count: 0,
            checksum: String::new(), // Will be calculated by service
            created_at: now,
            updated_at: now,
            is_active: true,
            version: 1,
            metadata: None,
        }
    }
    
    /// Calculate word count from content
    pub fn calculate_word_count(mut self) -> Self {
        if let Some(ref content) = self.content {
            self.word_count = content.split_whitespace().count();
        }
        self
    }
    
    /// Set metadata for the document
    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self.updated_at = Utc::now();
        self
    }
    
    /// Update document content
    pub fn update_content(mut self, content: Option<String>, title: Option<String>) -> Self {
        if let Some(new_title) = title {
            self.title = new_title;
        }
        if let Some(new_content) = content {
            self.content = Some(new_content);
            self.word_count = self.content.as_ref()
                .map(|c| c.split_whitespace().count())
                .unwrap_or(0);
        }
        self.version += 1;
        self.updated_at = Utc::now();
        self
    }
    
    /// Mark document as deleted
    pub fn deactivate(mut self) -> Self {
        self.is_active = false;
        self.updated_at = Utc::now();
        self
    }
    
    /// Set checksum for integrity verification
    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = checksum;
        self
    }
}

/// Document version for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    /// Version identifier
    pub id: Uuid,
    
    /// Parent document identifier
    pub document_id: Uuid,
    
    /// Version number
    pub version: u32,
    
    /// Document title at this version
    pub title: String,
    
    /// Document content at this version
    pub content: String,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Optional change description
    pub change_description: Option<String>,
}

/// Document statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStatistics {
    /// Number of documents in project
    pub total_documents: usize,
    
    /// Total word count across all documents
    pub total_words: usize,
    
    /// Average words per document
    pub average_words_per_document: f64,
    
    /// Most recent document update
    pub most_recent_update: Option<DateTime<Utc>>,
    
    /// Document types distribution
    pub document_types: std::collections::HashMap<String, usize>,
}

impl Default for DocumentStatistics {
    fn default() -> Self {
        DocumentStatistics {
            total_documents: 0,
            total_words: 0,
            average_words_per_document: 0.0,
            most_recent_update: None,
            document_types: std::collections::HashMap::new(),
        }
    }
}