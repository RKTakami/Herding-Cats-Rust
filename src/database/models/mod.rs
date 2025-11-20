//! Database models module - Simplified placeholder implementations
//!
//! Contains data structures for database entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod analysis;
pub mod codex;
pub mod codex_service;
pub mod research;

/// Project model representing a logical grouping of documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_archived: bool,
    pub is_active: bool,
    pub settings: Option<String>,
}

impl Project {
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
}

/// Project statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectStatistics {
    pub document_count: usize,
    pub total_words: usize,
    pub storage_size: usize,
    pub last_document_update: Option<DateTime<Utc>>,
}

/// Document model representing a text document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub document_type: String,
    pub word_count: usize,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub version: u32,
    pub metadata: Option<String>,
}

impl Document {
    pub fn new(
        project_id: Uuid,
        title: String,
        content: Option<String>,
        document_type: String,
    ) -> Self {
        let now = Utc::now();
        Document {
            id: Uuid::new_v4(),
            project_id,
            title,
            content,
            document_type,
            word_count: 0,
            checksum: String::new(),
            created_at: now,
            updated_at: now,
            is_active: true,
            version: 1,
            metadata: None,
        }
    }
}

/// Document statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentStatistics {
    pub total_documents: usize,
    pub total_words: usize,
    pub average_words_per_document: f64,
    pub most_recent_update: Option<DateTime<Utc>>,
    pub document_types: std::collections::HashMap<String, usize>,
}

/// Document version for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub id: Uuid,
    pub document_id: Uuid,
    pub version: u32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub change_description: Option<String>,
}

/// Document embedding for vector operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentEmbedding {
    pub id: Uuid,
    pub document_id: Uuid,
    pub vector_data: Vec<f32>,
    pub model_name: String,
    pub chunk_index: usize,
    pub text_chunk: String,
    pub start_char: usize,
    pub end_char: usize,
    pub created_at: DateTime<Utc>,
    pub metadata: Option<String>,
}

/// Search result with similarity score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document_id: Uuid,
    pub title: String,
    pub similarity_score: f32,
    pub snippet: String,
    pub chunk_index: usize,
    pub start_char: usize,
    pub end_char: usize,
}

/// Batch embedding request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchEmbeddingRequest {
    pub document_ids: Vec<Uuid>,
    pub model_name: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

/// Embedding statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmbeddingStatistics {
    pub total_embeddings: usize,
    pub documents_with_embeddings: usize,
    pub average_embeddings_per_document: f64,
    pub vector_dimension: usize,
    pub models_used: std::collections::HashMap<String, usize>,
    pub average_chunk_size: f64,
}

/// Model-specific result types
pub type ModelResult<T> = Result<T, super::DatabaseError>;
