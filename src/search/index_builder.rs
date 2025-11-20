//! Search Index Building and Maintenance
//! 
//! This module provides comprehensive index building capabilities for populating
//! and maintaining search indexes from all writing tools data.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::path::{PathBuf, Path};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Configuration for index building operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexBuilderConfig {
    pub batch_size: usize,
    pub max_concurrent_files: usize,
    pub enable_compression: bool,
    pub enable_incremental_updates: bool,
    pub rebuild_threshold: f32, // Percentage of changed items to trigger full rebuild
    pub cleanup_old_indexes: bool,
    pub validate_index_integrity: bool,
    pub parallel_processing: bool,
    pub memory_limit_mb: usize,
}

/// Index build operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildOperation {
    /// Build index for specific tool
    BuildToolIndex(String),
    
    /// Build all tool indexes
    BuildAllIndexes,
    
    /// Incremental update of existing index
    IncrementalUpdate(String),
    
    /// Rebuild corrupted index
    RebuildIndex(String),
    
    /// Merge multiple indexes
    MergeIndexes(Vec<String>),
    
    /// Validate existing index
    ValidateIndex(String),
}

/// Index build progress tracking
#[derive(Debug, Clone)]
pub struct BuildProgress {
    pub operation: BuildOperation,
    pub current_phase: BuildPhase,
    pub current_file: usize,
    pub total_files: usize,
    pub processed_items: usize,
    pub total_items: usize,
    pub percentage: f32,
    pub start_time: SystemTime,
    pub estimated_completion: Option<SystemTime>,
    pub current_file_path: Option<PathBuf>,
    pub errors: Vec<BuildError>,
    pub warnings: Vec<String>,
}

/// Index building phases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildPhase {
    Initializing,
    ScanningFiles,
    ProcessingContent,
    BuildingIndexes,
    Optimizing,
    Validating,
    Finalizing,
    Completed,
    Failed,
}

/// Build error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildError {
    pub phase: BuildPhase,
    pub file_path: Option<PathBuf>,
    pub message: String,
    pub recoverable: bool,
}

/// Index build statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildStatistics {
    pub total_build_time_ms: u64,
    pub files_processed: usize,
    pub items_indexed: usize,
    pub index_size_bytes: usize,
    pub compression_ratio: f32,
    pub average_processing_time_ms: f64,
    pub peak_memory_usage_mb: usize,
    pub errors_count: usize,
    pub warnings_count: usize,
}

/// Index optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSettings {
    pub enable_index_compression: bool,
    pub enable_term_frequency_optimization: bool,
    pub enable_stemming: bool,
    pub enable_synonym_mapping: bool,
    pub optimize_for_search_speed: bool,
    pub optimize_for_index_size: bool,
    pub custom_stop_words: HashSet<String>,
}

/// Main index builder
#[derive(Debug)]
pub struct IndexBuilder {
    config: IndexBuilderConfig,
    project_path: PathBuf,
    active_builds: Arc<RwLock<HashMap<Uuid, BuildProgress>>>,
    build_statistics: Arc<RwLock<HashMap<String, BuildStatistics>>>,
}

/// Index content processor
#[derive(Debug)]
pub struct ContentProcessor {
    supported_extensions: HashSet<String>,
    processors: HashMap<String, Box<dyn ContentProcessorTrait>>,
}

/// Content processor trait
#[async_trait::async_trait]
trait ContentProcessorTrait: Send + Sync {
    fn can_process(&self, file_path: &Path) -> bool;
    async fn process_file(&self, file_path: &Path) -> Result<ProcessedContent, BuildError>;
    fn get_content_type(&self) -> String;
}

/// Processed content from a file
#[derive(Debug, Clone)]
pub struct ProcessedContent {
    pub file_path: PathBuf,
    pub content_type: String,
    pub title: Option<String>,
    pub body_text: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub keywords: Vec<String>,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub word_count: usize,
    pub character_count: usize,
    pub created_at: Option<SystemTime>,
    pub modified_at: Option<SystemTime>,
}

/// Index entry for building
#[derive(Debug, Clone)]
pub struct IndexBuildEntry {
    pub id: String,
    pub tool_type: String,
    pub content_type: String,
    pub title: String,
    pub content: String,
    pub processed_content: ProcessedContent,
    pub search_terms: Vec<String>,
    pub term_frequencies: HashMap<String, usize>,
    pub positions: HashMap<String, Vec<usize>>,
}

/// Result of index build operation
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub build_id: Uuid,
    pub operation: BuildOperation,
    pub success: bool,
    pub statistics: BuildStatistics,
    pub output_path: PathBuf,
    pub created_at: SystemTime,
    pub duration_ms: u64,
    pub warnings: Vec<String>,
    pub errors: Vec<BuildError>,
}

/// Index build error types
#[derive(Debug, thiserror::Error)]
pub enum IndexBuildError {
    #[error("File processing error: {0}")]
    FileProcessing(String),
    
    #[error("Index corruption detected: {0}")]
    IndexCorruption(String),
    
    #[error("Memory limit exceeded: {0}MB")]
    MemoryLimitExceeded(usize),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Build cancelled")]
    BuildCancelled,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Result type for index build operations
pub type BuildResult<T> = Result<T, IndexBuildError>;

impl IndexBuilder {
    /// Create new index builder
    pub fn new(config: IndexBuilderConfig, project_path: PathBuf) -> Self {
        Self {
            config,
            project_path,
            active_builds: Arc::new(RwLock::new(HashMap::new())),
            build_statistics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Build index for specific tool
    pub async fn build_tool_index(&self, tool_type: &str) -> BuildResult<BuildResult> {
        let build_id = Uuid::new_v4();
        let operation = BuildOperation::BuildToolIndex(tool_type.to_string());
        
        self.initialize_build_progress(build_id, operation.clone()).await;
        
        let start_time = SystemTime::now();
        let result = match self.perform_tool_index_build(tool_type).await {
            Ok(statistics) => BuildResult {
                build_id,
                operation,
                success: true,
                statistics,
                output_path: self.get_tool_index_path(tool_type),
                created_at: start_time,
                duration_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
                warnings: Vec::new(),
                errors: Vec::new(),
            },
            Err(error) => BuildResult {
                build_id,
                operation,
                success: false,
                statistics: BuildStatistics {
                    total_build_time_ms: 0,
                    files_processed: 0,
                    items_indexed: 0,
                    index_size_bytes: 0,
                    compression_ratio: 0.0,
                    average_processing_time_ms: 0.0,
                    peak_memory_usage_mb: 0,
                    errors_count: 1,
                    warnings_count: 0,
                },
                output_path: PathBuf::new(),
                created_at: start_time,
                duration_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
                warnings: Vec::new(),
                errors: vec![BuildError {
                    phase: BuildPhase::Failed,
                    file_path: None,
                    message: error.to_string(),
                    recoverable: false,
                }],
            },
        };
        
        self.update_build_statistics(&operation, &result.statistics).await;
        self.cleanup_build_progress(build_id).await;
        
        Ok(result)
    }
    
    /// Build all tool indexes
    pub async fn build_all_indexes(&self) -> BuildResult<Vec<BuildResult>> {
        let tool_types = ["hierarchy", "codex", "notes", "research", "plot", "analysis"];
        let mut results = Vec::new();
        
        for tool_type in &tool_types {
            match self.build_tool_index(tool_type).await {
                Ok(result) => results.push(result),
                Err(error) => {
                    // Log error but continue with other tools
                    eprintln!("Failed to build index for {}: {}", tool_type, error);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Perform incremental index update
    pub async fn incremental_update(&self, tool_type: &str, changes: Vec<IndexChange>) -> BuildResult<BuildResult> {
        let build_id = Uuid::new_v4();
        let operation = BuildOperation::IncrementalUpdate(tool_type.to_string());
        
        self.initialize_build_progress(build_id, operation.clone()).await;
        
        let start_time = SystemTime::now();
        let result = match self.perform_incremental_update(tool_type, changes).await {
            Ok(statistics) => BuildResult {
                build_id,
                operation,
                success: true,
                statistics,
                output_path: self.get_tool_index_path(tool_type),
                created_at: start_time,
                duration_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
                warnings: Vec::new(),
                errors: Vec::new(),
            },
            Err(error) => BuildResult {
                build_id,
                operation,
                success: false,
                statistics: BuildStatistics {
                    total_build_time_ms: 0,
                    files_processed: 0,
                    items_indexed: 0,
                    index_size_bytes: 0,
                    compression_ratio: 0.0,
                    average_processing_time_ms: 0.0,
                    peak_memory_usage_mb: 0,
                    errors_count: 1,
                    warnings_count: 0,
                },
                output_path: PathBuf::new(),
                created_at: start_time,
                duration_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
                warnings: Vec::new(),
                errors: vec![BuildError {
                    phase: BuildPhase::Failed,
                    file_path: None,
                    message: error.to_string(),
                    recoverable: false,
                }],
            },
        };
        
        self.update_build_statistics(&operation, &result.statistics).await;
        self.cleanup_build_progress(build_id).await;
        
        Ok(result)
    }
    
    /// Rebuild corrupted index
    pub async fn rebuild_index(&self, tool_type: &str) -> BuildResult<BuildResult> {
        // First, backup any existing index
        if let Err(e) = self.backup_existing_index(tool_type).await {
            eprintln!("Failed to backup existing index for {}: {}", tool_type, e);
        }
        
        // Then rebuild from scratch
        self.build_tool_index(tool_type).await
    }
    
    /// Validate index integrity
    pub async fn validate_index(&self, tool_type: &str) -> BuildResult<bool> {
        let index_path = self.get_tool_index_path(tool_type);
        
        if !index_path.exists() {
            return Err(IndexBuildError::IndexCorruption(
                format!("Index file not found: {}", index_path.display())
            ));
        }
        
        // Validate index file structure and content
        self.validate_index_structure(&index_path).await
    }
    
    /// Get build progress
    pub async fn get_build_progress(&self, build_id: Uuid) -> Option<BuildProgress> {
        let active_builds = self.active_builds.read().await;
        active_builds.get(&build_id).cloned()
    }
    
    /// Get all active builds
    pub async fn get_active_builds(&self) -> Vec<BuildProgress> {
        let active_builds = self.active_builds.read().await;
        active_builds.values().cloned().collect()
    }
    
    /// Cancel active build
    pub async fn cancel_build(&self, build_id: Uuid) -> BuildResult<()> {
        let mut active_builds = self.active_builds.write().await;
        if let Some(progress) = active_builds.get_mut(&build_id) {
            progress.phase = BuildPhase::Failed;
            return Ok(());
        }
        
        Err(IndexBuildError::BuildCancelled)
    }
    
    /// Get build statistics
    pub async fn get_build_statistics(&self, tool_type: &str) -> Option<BuildStatistics> {
        let statistics = self.build_statistics.read().await;
        statistics.get(tool_type).cloned()
    }
    
    /// Clean up old indexes
    pub async fn cleanup_old_indexes(&self, max_age_days: u32) -> BuildResult<usize> {
        if !self.config.cleanup_old_indexes {
            return Ok(0);
        }
        
        let cutoff_time = SystemTime::now() - std::time::Duration::from_secs(max_age_days as u64 * 24 * 60 * 60);
        let index_dir = self.project_path.join("index");
        
        let mut cleaned_count = 0;
        
        if index_dir.exists() {
            for entry in fs::read_dir(&index_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            if modified < cutoff_time {
                                fs::remove_file(&path)?;
                                cleaned_count += 1;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(cleaned_count)
    }
    
    // Private helper methods
    
    async fn initialize_build_progress(&self, build_id: Uuid, operation: BuildOperation) {
        let mut active_builds = self.active_builds.write().await;
        active_builds.insert(build_id, BuildProgress {
            operation,
            current_phase: BuildPhase::Initializing,
            current_file: 0,
            total_files: 0,
            processed_items: 0,
            total_items: 0,
            percentage: 0.0,
            start_time: SystemTime::now(),
            estimated_completion: None,
            current_file_path: None,
            errors: Vec::new(),
            warnings: Vec::new(),
        });
    }
    
    async fn cleanup_build_progress(&self, build_id: Uuid) {
        let mut active_builds = self.active_builds.write().await;
        active_builds.remove(&build_id);
    }
    
    async fn perform_tool_index_build(&self, tool_type: &str) -> BuildResult<BuildStatistics> {
        let tool_dir = self.project_path.join("content").join(tool_type);
        
        if !tool_dir.exists() {
            return Err(IndexBuildError::FileProcessing(
                format!("Tool directory not found: {}", tool_dir.display())
            ));
        }
        
        let start_time = SystemTime::now();
        let mut statistics = BuildStatistics {
            total_build_time_ms: 0,
            files_processed: 0,
            items_indexed: 0,
            index_size_bytes: 0,
            compression_ratio: 0.0,
            average_processing_time_ms: 0.0,
            peak_memory_usage_mb: 0,
            errors_count: 0,
            warnings_count: 0,
        };
        
        // Scan and process all files
        let file_list = self.scan_tool_directory(&tool_dir).await?;
        statistics.total_files = file_list.len();
        
        // Process files in batches
        let mut all_entries = Vec::new();
        let batch_size = self.config.batch_size;
        
        for batch in file_list.chunks(batch_size) {
            let batch_results = self.process_file_batch(batch, tool_type).await?;
            all_entries.extend(batch_results);
            statistics.files_processed += batch.len();
        }
        
        statistics.items_indexed = all_entries.len();
        
        // Build search index from entries
        let search_index = self.build_search_index(&all_entries, tool_type)?;
        statistics.index_size_bytes = self.calculate_index_size(&search_index)?;
        
        // Apply optimizations
        let optimized_index = self.apply_optimizations(search_index, tool_type)?;
        
        // Save index
        let output_path = self.get_tool_index_path(tool_type);
        self.save_index(&optimized_index, &output_path).await?;
        
        // Calculate final statistics
        statistics.total_build_time_ms = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        statistics.average_processing_time_ms = if statistics.files_processed > 0 {
            statistics.total_build_time_ms as f64 / statistics.files_processed as f64
        } else {
            0.0
        };
        
        Ok(statistics)
    }
    
    async fn scan_tool_directory(&self, tool_dir: &Path) -> BuildResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if tool_dir.is_dir() {
            for entry in fs::read_dir(tool_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && self.is_supported_file(&path) {
                    files.push(path);
                } else if path.is_dir() {
                    // Recursively scan subdirectories
                    let sub_files = self.scan_tool_directory(&path).await?;
                    files.extend(sub_files);
                }
            }
        }
        
        Ok(files)
    }
    
    fn is_supported_file(&self, path: &Path) -> bool {
        let extension = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        matches!(extension.as_str(), "json" | "md" | "txt" | "rtf")
    }
    
    async fn process_file_batch(&self, files: &[PathBuf], tool_type: &str) -> BuildResult<Vec<IndexBuildEntry>> {
        let mut entries = Vec::new();
        let processor = ContentProcessor::new();
        
        for file_path in files {
            match processor.process_file(file_path).await {
                Ok(content) => {
                    let entry = self.create_index_entry(content, tool_type)?;
                    entries.push(entry);
                }
                Err(error) => {
                    eprintln!("Error processing file {:?}: {}", file_path, error.message);
                    // Continue processing other files
                }
            }
        }
        
        Ok(entries)
    }
    
    fn create_index_entry(&self, processed_content: ProcessedContent, tool_type: &str) -> BuildResult<IndexBuildEntry> {
        let id = Uuid::new_v4().to_string();
        
        // Extract search terms
        let search_terms = self.extract_search_terms(&processed_content);
        
        // Calculate term frequencies
        let term_frequencies = self.calculate_term_frequencies(&processed_content.body_text);
        
        // Find term positions
        let positions = self.find_term_positions(&processed_content.body_text, &search_terms);
        
        Ok(IndexBuildEntry {
            id,
            tool_type: tool_type.to_string(),
            content_type: processed_content.content_type,
            title: processed_content.title.unwrap_or_else(|| "Untitled".to_string()),
            content: processed_content.body_text,
            processed_content,
            search_terms,
            term_frequencies,
            positions,
        })
    }
    
    fn extract_search_terms(&self, content: &ProcessedContent) -> Vec<String> {
        let mut terms = Vec::new();
        
        // Add title terms
        if let Some(title) = &content.title {
            terms.extend(self.tokenize_text(title));
        }
        
        // Add body text terms
        terms.extend(self.tokenize_text(&content.body_text));
        
        // Add keyword terms
        terms.extend(content.keywords.clone());
        
        // Add tag terms
        terms.extend(content.tags.iter().map(|t| t.to_lowercase()));
        
        // Remove duplicates and filter out common words
        let mut unique_terms: Vec<String> = terms.into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|term| term.len() > 2 && !self.is_stop_word(term))
            .collect();
        
        unique_terms.sort();
        unique_terms
    }
    
    fn tokenize_text(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|word| word.trim().to_lowercase())
            .filter(|word| !word.is_empty())
            .collect()
    }
    
    fn is_stop_word(&self, word: &str) -> bool {
        let stop_words = ["the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by"];
        stop_words.contains(&word)
    }
    
    fn calculate_term_frequencies(&self, text: &str) -> HashMap<String, usize> {
        let mut frequencies = HashMap::new();
        let terms = self.tokenize_text(text);
        
        for term in terms {
            *frequencies.entry(term).or_insert(0) += 1;
        }
        
        frequencies
    }
    
    fn find_term_positions(&self, text: &str, terms: &[String]) -> HashMap<String, Vec<usize>> {
        let mut positions = HashMap::new();
        let text_lower = text.to_lowercase();
        
        for term in terms {
            let mut term_positions = Vec::new();
            let mut start = 0;
            
            while let Some(pos) = text_lower[start..].find(&term.to_lowercase()) {
                term_positions.push(start + pos);
                start += pos + 1;
            }
            
            if !term_positions.is_empty() {
                positions.insert(term.clone(), term_positions);
            }
        }
        
        positions
    }
    
    fn build_search_index(&self, entries: &[IndexBuildEntry], tool_type: &str) -> BuildResult<SearchIndex> {
        let mut inverted_index = HashMap::new();
        let mut documents = HashMap::new();
        
        for entry in entries {
            // Add to documents map
            documents.insert(entry.id.clone(), entry.clone());
            
            // Build inverted index
            for term in &entry.search_terms {
                let term = term.to_lowercase();
                inverted_index
                    .entry(term)
                    .or_insert_with(Vec::new)
                    .push(IndexEntryRef {
                        document_id: entry.id.clone(),
                        frequency: *entry.term_frequencies.get(&term).unwrap_or(&0),
                        positions: entry.positions.get(&term).cloned().unwrap_or_default(),
                    });
            }
        }
        
        Ok(SearchIndex {
            tool_type: tool_type.to_string(),
            documents,
            inverted_index,
            last_updated: SystemTime::now(),
        })
    }
    
    fn apply_optimizations(&self, mut index: SearchIndex, tool_type: &str) -> BuildResult<SearchIndex> {
        // Apply compression if enabled
        if self.config.enable_compression {
            index = self.compress_index(index)?;
        }
        
        // Apply stemming if enabled
        if self.config.enable_stemming {
            index = self.apply_stemming(index)?;
        }
        
        // Optimize for search performance
        index = self.optimize_for_search(index)?;
        
        Ok(index)
    }
    
    fn compress_index(&self, index: SearchIndex) -> BuildResult<SearchIndex> {
        // Implementation would compress the index using appropriate algorithms
        // For now, return the index unchanged
        Ok(index)
    }
    
    fn apply_stemming(&self, index: SearchIndex) -> BuildResult<SearchIndex> {
        // Implementation would apply stemming to search terms
        // For now, return the index unchanged
        Ok(index)
    }
    
    fn optimize_for_search(&self, index: SearchIndex) -> BuildResult<SearchIndex> {
        // Sort inverted index entries by frequency for faster search
        let mut optimized_index = index;
        
        for entries in optimized_index.inverted_index.values_mut() {
            entries.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        }
        
        Ok(optimized_index)
    }
    
    fn calculate_index_size(&self, index: &SearchIndex) -> BuildResult<usize> {
        // Calculate approximate size of index in memory
        let mut size = 0;
        
        for doc in index.documents.values() {
            size += doc.content.len();
            size += doc.title.len();
            size += serde_json::to_string(&doc.processed_content.metadata)?.len();
        }
        
        for term_entries in index.inverted_index.values() {
            for entry in term_entries {
                size += entry.document_id.len();
                size += entry.positions.len() * std::mem::size_of::<usize>();
            }
        }
        
        Ok(size)
    }
    
    async fn save_index(&self, index: &SearchIndex, path: &Path) -> BuildResult<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Serialize and save index
        let content = serde_json::to_string_pretty(index)?;
        fs::write(path, content)?;
        
        Ok(())
    }
    
    async fn perform_incremental_update(&self, tool_type: &str, changes: Vec<IndexChange>) -> BuildResult<BuildStatistics> {
        // Load existing index
        let index_path = self.get_tool_index_path(tool_type);
        let mut index = if index_path.exists() {
            self.load_index(&index_path).await?
        } else {
            SearchIndex {
                tool_type: tool_type.to_string(),
                documents: HashMap::new(),
                inverted_index: HashMap::new(),
                last_updated: SystemTime::now(),
            }
        };
        
        // Apply changes
        for change in changes {
            match change {
                IndexChange::AddDocument(doc) => {
                    index.documents.insert(doc.id.clone(), doc);
                }
                IndexChange::UpdateDocument(id, doc) => {
                    index.documents.insert(id, doc);
                }
                IndexChange::RemoveDocument(id) => {
                    index.documents.remove(&id);
                }
            }
        }
        
        // Rebuild inverted index
        index.inverted_index = self.rebuild_inverted_index(&index.documents)?;
        
        // Save updated index
        self.save_index(&index, &index_path).await?;
        
        Ok(BuildStatistics {
            total_build_time_ms: 0,
            files_processed: 0,
            items_indexed: index.documents.len(),
            index_size_bytes: 0,
            compression_ratio: 0.0,
            average_processing_time_ms: 0.0,
            peak_memory_usage_mb: 0,
            errors_count: 0,
            warnings_count: 0,
        })
    }
    
    fn rebuild_inverted_index(&self, documents: &HashMap<String, IndexBuildEntry>) -> BuildResult<HashMap<String, Vec<IndexEntryRef>>> {
        let mut inverted_index = HashMap::new();
        
        for (doc_id, doc) in documents {
            for term in &doc.search_terms {
                let term = term.to_lowercase();
                inverted_index
                    .entry(term)
                    .or_insert_with(Vec::new)
                    .push(IndexEntryRef {
                        document_id: doc_id.clone(),
                        frequency: *doc.term_frequencies.get(&term).unwrap_or(&0),
                        positions: doc.positions.get(&term).cloned().unwrap_or_default(),
                    });
            }
        }
        
        Ok(inverted_index)
    }
    
    async fn load_index(&self, path: &Path) -> BuildResult<SearchIndex> {
        let content = fs::read_to_string(path)?;
        let index: SearchIndex = serde_json::from_str(&content)?;
        Ok(index)
    }
    
    async fn backup_existing_index(&self, tool_type: &str) -> BuildResult<()> {
        let index_path = self.get_tool_index_path(tool_type);
        if index_path.exists() {
            let backup_path = index_path.with_extension("json.bak");
            fs::copy(&index_path, &backup_path)?;
        }
        Ok(())
    }
    
    async fn validate_index_structure(&self, index_path: &Path) -> BuildResult<bool> {
        // Basic validation of index structure
        let content = fs::read_to_string(index_path)?;
        let index: SearchIndex = serde_json::from_str(&content)?;
        
        // Check for required fields and consistency
        if index.documents.is_empty() && index.inverted_index.is_empty() {
            return Err(IndexBuildError::IndexCorruption("Empty index".to_string()));
        }
        
        // Validate inverted index consistency
        for (term, entries) in &index.inverted_index {
            for entry in entries {
                if !index.documents.contains_key(&entry.document_id) {
                    return Err(IndexBuildError::IndexCorruption(
                        format!("Inverted index references non-existent document: {}", entry.document_id)
                    ));
                }
            }
        }
        
        Ok(true)
    }
    
    fn get_tool_index_path(&self, tool_type: &str) -> PathBuf {
        self.project_path.join("index").join(format!("{}_index.json", tool_type))
    }
    
    async fn update_build_statistics(&self, operation: &BuildOperation, statistics: &BuildStatistics) {
        let mut stats_map = self.build_statistics.write().await;
        
        let key = match operation {
            BuildOperation::BuildToolIndex(tool_type) | 
            BuildOperation::IncrementalUpdate(tool_type) |
            BuildOperation::RebuildIndex(tool_type) |
            BuildOperation::ValidateIndex(tool_type) => tool_type.clone(),
            _ => "all".to_string(),
        };
        
        stats_map.insert(key, statistics.clone());
    }
}

/// Search index structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchIndex {
    pub tool_type: String,
    pub documents: HashMap<String, IndexBuildEntry>,
    pub inverted_index: HashMap<String, Vec<IndexEntryRef>>,
    pub last_updated: SystemTime,
}

/// Index entry reference in inverted index
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexEntryRef {
    pub document_id: String,
    pub frequency: usize,
    pub positions: Vec<usize>,
}

/// Index change types
#[derive(Debug, Clone)]
pub enum IndexChange {
    AddDocument(IndexBuildEntry),
    UpdateDocument(String, IndexBuildEntry),
    RemoveDocument(String),
}

/// Content processor implementations
impl ContentProcessor {
    pub fn new() -> Self {
        let mut supported_extensions = HashSet::new();
        supported_extensions.insert("json".to_string());
        supported_extensions.insert("md".to_string());
        supported_extensions.insert("txt".to_string());
        supported_extensions.insert("rtf".to_string());
        
        let mut processors = HashMap::new();
        processors.insert("json".to_string(), Box::new(JsonProcessor) as Box<dyn ContentProcessorTrait>);
        processors.insert("md".to_string(), Box::new(MarkdownProcessor) as Box<dyn ContentProcessorTrait>);
        processors.insert("txt".to_string(), Box::new(TextProcessor) as Box<dyn ContentProcessorTrait>);
        
        Self {
            supported_extensions,
            processors,
        }
    }
}

#[async_trait::async_trait]
impl ContentProcessorTrait for ContentProcessor {
    fn can_process(&self, file_path: &Path) -> bool {
        let extension = file_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        self.supported_extensions.contains(&extension)
    }
    
    async fn process_file(&self, file_path: &Path) -> Result<ProcessedContent, BuildError> {
        let extension = file_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if let Some(processor) = self.processors.get(&extension) {
            processor.process_file(file_path).await
        } else {
            Err(BuildError {
                phase: BuildPhase::ProcessingContent,
                file_path: Some(file_path.to_path_buf()),
                message: format!("No processor available for file type: {}", extension),
                recoverable: false,
            })
        }
    }
    
    fn get_content_type(&self) -> String {
        "generic".to_string()
    }
}

/// JSON content processor
struct JsonProcessor;

#[async_trait::async_trait]
impl ContentProcessorTrait for JsonProcessor {
    fn can_process(&self, file_path: &Path) -> bool {
        file_path.extension().and_then(|s| s.to_str()) == Some("json")
    }
    
    async fn process_file(&self, file_path: &Path) -> Result<ProcessedContent, BuildError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| BuildError {
                phase: BuildPhase::ProcessingContent,
                file_path: Some(file_path.to_path_buf()),
                message: format!("Failed to read file: {}", e),
                recoverable: false,
            })?;
        
        let json_value: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| BuildError {
                phase: BuildPhase::ProcessingContent,
                file_path: Some(file_path.to_path_buf()),
                message: format!("Failed to parse JSON: {}", e),
                recoverable: false,
            })?;
        
        let title = json_value.get("title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let body_text = json_value.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let metadata = serde_json::to_value(&json_value)
            .unwrap_or_default();
        
        let metadata_map = if let Ok(map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(metadata.clone()) {
            map
        } else {
            HashMap::new()
        };
        
        Ok(ProcessedContent {
            file_path: file_path.to_path_buf(),
            content_type: "json".to_string(),
            title,
            body_text,
            metadata: metadata_map,
            keywords: Vec::new(),
            tags: Vec::new(),
            references: Vec::new(),
            word_count: body_text.split_whitespace().count(),
            character_count: body_text.len(),
            created_at: None,
            modified_at: fs::metadata(file_path).ok().and_then(|m| m.modified().ok()),
        })
    }
    
    fn get_content_type(&self) -> String {
        "json".to_string()
    }
}

/// Markdown content processor
struct MarkdownProcessor;

#[async_trait::async_trait]
impl ContentProcessorTrait for MarkdownProcessor {
    fn can_process(&self, file_path: &Path) -> bool {
        file_path.extension().and_then(|s| s.to_str()) == Some("md")
    }
    
    async fn process_file(&self, file_path: &Path) -> Result<ProcessedContent, BuildError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| BuildError {
                phase: BuildPhase::ProcessingContent,
                file_path: Some(file_path.to_path_buf()),
                message: format!("Failed to read file: {}", e),
                recoverable: false,
            })?;
        
        let lines: Vec<&str> = content.lines().collect();
        let title = lines.first().and_then(|line| {
            if line.starts_with('#') {
                Some(line.trim_start_matches('#').trim().to_string())
            } else {
                None
            }
        });
        
        // Remove markdown formatting for plain text content
        let body_text = content
            .replace(|c: char| c == '#' || c == '*' || c == '_' || c == '`' || c == '[' || c == ']')
            .replace("()", "")
            .replace("()", "");
        
        Ok(ProcessedContent {
            file_path: file_path.to_path_buf(),
            content_type: "markdown".to_string(),
            title,
            body_text,
            metadata: HashMap::new(),
            keywords: Vec::new(),
            tags: Vec::new(),
            references: Vec::new(),
            word_count: body_text.split_whitespace().count(),
            character_count: body_text.len(),
            created_at: None,
            modified_at: fs::metadata(file_path).ok().and_then(|m| m.modified().ok()),
        })
    }
    
    fn get_content_type(&self) -> String {
        "markdown".to_string()
    }
}

/// Plain text content processor
struct TextProcessor;

#[async_trait::async_trait]
impl ContentProcessorTrait for TextProcessor {
    fn can_process(&self, file_path: &Path) -> bool {
        file_path.extension().and_then(|s| s.to_str()) == Some("txt")
    }
    
    async fn process_file(&self, file_path: &Path) -> Result<ProcessedContent, BuildError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| BuildError {
                phase: BuildPhase::ProcessingContent,
                file_path: Some(file_path.to_path_buf()),
                message: format!("Failed to read file: {}", e),
                recoverable: false,
            })?;
        
        let lines: Vec<&str> = content.lines().collect();
        let title = lines.first().map(|s| s.to_string());
        
        Ok(ProcessedContent {
            file_path: file_path.to_path_buf(),
            content_type: "text".to_string(),
            title,
            body_text: content,
            metadata: HashMap::new(),
            keywords: Vec::new(),
            tags: Vec::new(),
            references: Vec::new(),
            word_count: content.split_whitespace().count(),
            character_count: content.len(),
            created_at: None,
            modified_at: fs::metadata(file_path).ok().and_then(|m| m.modified().ok()),
        })
    }
    
    fn get_content_type(&self) -> String {
        "text".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_builder_config() {
        let config = IndexBuilderConfig::default();
        assert_eq!(config.batch_size, 100);
        assert!(config.enable_compression);
    }

    #[test]
    fn test_content_processor() {
        let processor = ContentProcessor::new();
        assert!(processor.can_process(Path::new("test.json")));
        assert!(!processor.can_process(Path::new("test.unknown")));
    }
}