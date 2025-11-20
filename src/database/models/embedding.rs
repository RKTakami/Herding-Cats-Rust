//! Embedding model for vector storage
//! 
//! Represents vector embeddings for LLM integration and semantic search

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Document embedding for vector operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentEmbedding {
    /// Unique embedding identifier
    pub id: Uuid,
    
    /// Associated document identifier
    pub document_id: Uuid,
    
    /// Embedding vector data (stored as BLOB)
    pub vector_data: Vec<f32>,
    
    /// Model used to generate the embedding
    pub model_name: String,
    
    /// Chunk index within document (for chunked embeddings)
    pub chunk_index: usize,
    
    /// Original text chunk that generated this embedding
    pub text_chunk: String,
    
    /// Character offset in original document
    pub start_char: usize,
    pub end_char: usize,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Optional metadata
    pub metadata: Option<String>,
}

impl DocumentEmbedding {
    /// Create a new document embedding
    pub fn new(
        document_id: Uuid,
        vector_data: Vec<f32>,
        model_name: String,
        text_chunk: String,
        start_char: usize,
        end_char: usize,
        chunk_index: usize,
    ) -> Self {
        DocumentEmbedding {
            id: Uuid::new_v4(),
            document_id,
            vector_data,
            model_name,
            chunk_index,
            text_chunk,
            start_char,
            end_char,
            created_at: Utc::now(),
            metadata: None,
        }
    }
    
    /// Create embedding with metadata
    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self
    }
    
    /// Get vector dimension
    pub fn dimension(&self) -> usize {
        self.vector_data.len()
    }
    
    /// Calculate cosine similarity with another embedding
    pub fn cosine_similarity(&self, other: &DocumentEmbedding) -> f32 {
        let dot_product: f32 = self.vector_data.iter()
            .zip(other.vector_data.iter())
            .map(|(a, b)| a * b)
            .sum();
            
        let norm_a: f32 = self.vector_data.iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt();
            
        let norm_b: f32 = other.vector_data.iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt();
            
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}

/// Search result with similarity score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document identifier
    pub document_id: Uuid,
    
    /// Document title
    pub title: String,
    
    /// Similarity score (0.0 to 1.0)
    pub similarity_score: f32,
    
    /// Matching text snippet
    pub snippet: String,
    
    /// Chunk index that matched
    pub chunk_index: usize,
    
    /// Character range in document
    pub start_char: usize,
    pub end_char: usize,
}

impl SearchResult {
    /// Create a new search result
    pub fn new(
        document_id: Uuid,
        title: String,
        similarity_score: f32,
        snippet: String,
        chunk_index: usize,
        start_char: usize,
        end_char: usize,
    ) -> Self {
        SearchResult {
            document_id,
            title,
            similarity_score,
            snippet,
            chunk_index,
            start_char,
            end_char,
        }
    }
    
    /// Check if result meets minimum similarity threshold
    pub fn meets_threshold(&self, threshold: f32) -> bool {
        self.similarity_score >= threshold
    }
}

/// Embedding statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingStatistics {
    /// Total number of embeddings stored
    pub total_embeddings: usize,
    
    /// Number of documents with embeddings
    pub documents_with_embeddings: usize,
    
    /// Average embeddings per document
    pub average_embeddings_per_document: f64,
    
    /// Vector dimension
    pub vector_dimension: usize,
    
    /// Models used for embeddings
    pub models_used: std::collections::HashMap<String, usize>,
    
    /// Average chunk size
    pub average_chunk_size: f64,
}

impl Default for EmbeddingStatistics {
    fn default() -> Self {
        EmbeddingStatistics {
            total_embeddings: 0,
            documents_with_embeddings: 0,
            average_embeddings_per_document: 0.0,
            vector_dimension: 0,
            models_used: std::collections::HashMap::new(),
            average_chunk_size: 0.0,
        }
    }
}

/// Batch embedding request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchEmbeddingRequest {
    /// Documents to generate embeddings for
    pub document_ids: Vec<Uuid>,
    
    /// Model to use for embedding generation
    pub model_name: String,
    
    /// Chunk size for document splitting
    pub chunk_size: usize,
    
    /// Overlap between chunks
    pub chunk_overlap: usize,
}

impl BatchEmbeddingRequest {
    /// Create a new batch request
    pub fn new(
        document_ids: Vec<Uuid>,
        model_name: String,
        chunk_size: usize,
        chunk_overlap: usize,
    ) -> Self {
        BatchEmbeddingRequest {
            document_ids,
            model_name,
            chunk_size,
            chunk_overlap,
        }
    }
}