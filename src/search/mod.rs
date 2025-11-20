//! Search Engine for Herding Cats Writing Tools
//! 
//! This module provides comprehensive search capabilities across all writing tools
//! including hierarchy, codex, notes, research, plot, and analysis data.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::{PathBuf, Path};
use std::fs;
use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;

/// Search Engine main entry point
#[derive(Debug)]
pub struct SearchEngine {
    config: SearchConfig,
    index_manager: Arc<RwLock<IndexManager>>,
}

/// Search engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub max_results: usize,
    pub timeout_ms: u64,
    pub enable_fuzzy_matching: bool,
    pub enable_synonyms: bool,
    pub min_score_threshold: f32,
    pub enable_stemming: bool,
    pub enable_highlighting: bool,
    pub cache_results: bool,
    pub cache_ttl_seconds: u64,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_results: 100,
            timeout_ms: 5000,
            enable_fuzzy_matching: true,
            enable_synonyms: true,
            min_score_threshold: 0.1,
            enable_stemming: true,
            enable_highlighting: true,
            cache_results: true,
            cache_ttl_seconds: 3600,
        }
    }
}

/// Search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub id: String,
    pub tool_type: ToolType,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub score: f32,
    pub highlights: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub last_modified: SystemTime,
    pub created_at: SystemTime,
}

/// Tool types for search indexing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolType {
    Hierarchy,
    Codex,
    Notes,
    Research,
    Plot,
    Analysis,
    All,
}

/// Search request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub tool_types: Vec<ToolType>,
    pub operators: Vec<SearchOperator>,
    pub filters: HashMap<String, serde_json::Value>,
    pub sort_by: SearchSort,
    pub max_results: Option<usize>,
    pub offset: Option<usize>,
}

/// Search operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchOperator {
    And,
    Or,
    Not,
    Phrase(String),
    Wildcard(String),
}

/// Search sort options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchSort {
    Relevance,
    DateModified,
    DateCreated,
    Title,
    ToolType,
}

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
    pub total_results: usize,
    pub query_time_ms: u64,
    pub suggestions: Vec<String>,
    pub facets: HashMap<String, Vec<FacetItem>>,
}

/// Search facet item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetItem {
    pub value: String,
    pub count: usize,
    pub selected: bool,
}

/// Search error types
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Index not found: {0}")]
    IndexNotFound(String),
    
    #[error("Query parsing error: {0}")]
    QueryParseError(String),
    
    #[error("Search timeout after {0}ms")]
    Timeout(u64),
    
    #[error("Index corruption detected: {0}")]
    IndexCorruption(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Search engine result type
pub type SearchResult<T> = Result<T, SearchError>;

/// Index manager for search indices
#[derive(Debug, Default)]
pub struct IndexManager {
    indices: HashMap<ToolType, SearchIndex>,
    project_path: PathBuf,
}

/// Search index structure
#[derive(Debug, Clone)]
pub struct SearchIndex {
    pub tool_type: ToolType,
    pub documents: HashMap<String, SearchDocument>,
    pub inverted_index: HashMap<String, Vec<DocumentRef>>,
    pub last_updated: SystemTime,
}

/// Search document in index
#[derive(Debug, Clone)]
pub struct SearchDocument {
    pub id: String,
    pub title: String,
    pub content: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}

/// Document reference in inverted index
#[derive(Debug, Clone)]
struct DocumentRef {
    document_id: String,
    frequency: usize,
    positions: Vec<usize>,
}

/// Search statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineStats {
    pub index_count: usize,
    pub total_documents: usize,
    pub total_size_bytes: usize,
    pub average_query_time_ms: f64,
    pub total_queries: u64,
    pub cache_hit_rate: f32,
}

impl SearchEngine {
    /// Create new search engine instance
    pub fn new(config: SearchConfig, project_path: PathBuf) -> SearchResult<Self> {
        let index_manager = Arc::new(RwLock::new(IndexManager::new(project_path)?));
        
        Ok(Self {
            config,
            index_manager,
        })
    }
    
    /// Create new search engine with default configuration
    pub fn new_with_defaults(project_path: PathBuf) -> SearchResult<Self> {
        Self::new(SearchConfig::default(), project_path)
    }
    
    /// Perform search operation
    pub async fn search(&mut self, request: SearchRequest) -> SearchResult<SearchResponse> {
        let start_time = std::time::Instant::now();
        
        // Parse and validate query
        let parsed_query = self.parse_query(&request.query)?;
        
        // Get relevant indices
        let index_manager = self.index_manager.read().await;
        let relevant_indices = self.get_relevant_indices(&index_manager, &request.tool_types)?;
        drop(index_manager);
        
        // Perform search across all relevant indices
        let mut all_results = Vec::new();
        for index in relevant_indices {
            let index_results = self.search_index(index, &parsed_query, &request.filters)?;
            all_results.extend(index_results);
        }
        
        // Apply scoring and ranking
        let scored_results = self.score_and_rank_results(all_results, &parsed_query)?;
        
        // Apply sorting
        let sorted_results = self.apply_sorting(scored_results, &request.sort_by)?;
        
        // Apply pagination
        let paginated_results = self.apply_pagination(
            sorted_results,
            request.offset.unwrap_or(0),
            request.max_results.unwrap_or(self.config.max_results)
        );
        
        // Generate facets
        let facets = self.generate_facets(&paginated_results)?;
        
        // Generate suggestions for empty or poor results
        let suggestions = self.generate_suggestions(&request.query, &paginated_results)?;
        
        let query_time = start_time.elapsed().as_millis() as u64;
        
        Ok(SearchResponse {
            results: paginated_results,
            total_results: sorted_results.len(),
            query_time_ms: query_time,
            suggestions,
            facets,
        })
    }
    
    /// Build search index for a specific tool
    pub async fn build_index(&mut self, tool_type: ToolType) -> SearchResult<()> {
        let mut index_manager = self.index_manager.write().await;
        let documents = self.extract_tool_data(&index_manager.project_path, &tool_type)?;
        let index = self.build_index_from_documents(documents, tool_type.clone())?;
        index_manager.indices.insert(tool_type, index);
        Ok(())
    }
    
    /// Update search index incrementally
    pub async fn update_index(&mut self, tool_type: ToolType, changes: Vec<IndexChange>) -> SearchResult<()> {
        let mut index_manager = self.index_manager.write().await;
        if let Some(index) = index_manager.indices.get_mut(&tool_type) {
            self.apply_changes_to_index(index, changes)?;
        }
        Ok(())
    }
    
    /// Get search suggestions as user types
    pub async fn get_suggestions(&self, partial_query: &str, limit: usize) -> SearchResult<Vec<String>> {
        let index_manager = self.index_manager.read().await;
        self.generate_search_suggestions(partial_query, limit, &index_manager)
    }
    
    /// Clear search cache
    pub fn clear_cache(&self) -> SearchResult<()> {
        // Cache clearing implementation would go here
        Ok(())
    }
    
    /// Get search statistics
    pub async fn get_statistics(&self) -> SearchResult<SearchEngineStats> {
        let index_manager = self.index_manager.read().await;
        let mut total_docs = 0;
        let mut total_size = 0;
        
        for index in index_manager.indices.values() {
            total_docs += index.documents.len();
            for doc in index.documents.values() {
                total_size += doc.content.len() + serde_json::to_string(&doc.metadata)?.len();
            }
        }
        
        Ok(SearchEngineStats {
            index_count: index_manager.indices.len(),
            total_documents: total_docs,
            total_size_bytes: total_size,
            average_query_time_ms: 0.0, // Would be tracked in actual implementation
            total_queries: 0, // Would be tracked in actual implementation
            cache_hit_rate: 0.0, // Would be tracked in actual implementation
        })
    }
    
    // Private helper methods
    
    fn parse_query(&self, query: &str) -> SearchResult<ParsedQuery> {
        if query.trim().is_empty() {
            return Err(SearchError::QueryParseError("Empty query".to_string()));
        }
        
        let terms: Vec<String> = query
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();
        
        Ok(ParsedQuery {
            terms,
            phrases: self.extract_phrases(query),
        })
    }
    
    fn extract_phrases(&self, query: &str) -> Vec<String> {
        let mut phrases = Vec::new();
        let mut in_quote = false;
        let mut current_phrase = String::new();
        
        for c in query.chars() {
            if c == '"' {
                if in_quote && !current_phrase.is_empty() {
                    phrases.push(current_phrase.clone());
                    current_phrase.clear();
                }
                in_quote = !in_quote;
            } else if in_quote {
                current_phrase.push(c);
            }
        }
        
        phrases
    }
    
    fn get_relevant_indices<'a>(&self, index_manager: &'a IndexManager, tool_types: &[ToolType]) -> SearchResult<Vec<&'a SearchIndex>> {
        let mut indices = Vec::new();
        
        for tool_type in tool_types {
            if let Some(index) = index_manager.indices.get(tool_type) {
                indices.push(index);
            }
        }
        
        if indices.is_empty() {
            return Err(SearchError::IndexNotFound("No indices found for specified tool types".to_string()));
        }
        
        Ok(indices)
    }
    
    fn search_index(&self, index: &SearchIndex, query: &ParsedQuery, filters: &HashMap<String, serde_json::Value>) -> SearchResult<Vec<SearchResultItem>> {
        let mut results = Vec::new();
        
        for (doc_id, doc) in &index.documents {
            // Apply filters first
            if !self.apply_filters(doc, filters)? {
                continue;
            }
            
            // Calculate relevance score
            let score = self.calculate_relevance_score(doc, query);
            
            if score >= self.config.min_score_threshold {
                let highlights = self.generate_highlights(doc, query);
                
                results.push(SearchResultItem {
                    id: doc.id.clone(),
                    tool_type: index.tool_type.clone(),
                    title: doc.title.clone(),
                    content: doc.content.clone(),
                    summary: self.generate_summary(&doc.content, &query.terms),
                    score,
                    highlights,
                    metadata: doc.metadata.clone(),
                    last_modified: doc.modified_at,
                    created_at: doc.created_at,
                });
            }
        }
        
        Ok(results)
    }
    
    fn calculate_relevance_score(&self, doc: &SearchDocument, query: &ParsedQuery) -> f32 {
        let mut score = 0.0;
        let content_lower = doc.content.to_lowercase();
        let title_lower = doc.title.to_lowercase();
        
        // Score title matches higher
        for term in &query.terms {
            if title_lower.contains(term) {
                score += 2.0;
            }
            if content_lower.contains(term) {
                score += 1.0;
            }
        }
        
        // Score phrase matches highest
        for phrase in &query.phrases {
            if content_lower.contains(&phrase.to_lowercase()) {
                score += 3.0;
            }
        }
        
        // Normalize score based on content length
        let content_length = doc.content.len() as f32;
        if content_length > 0.0 {
            score /= (content_length / 100.0).sqrt();
        }
        
        score
    }
    
    fn apply_filters(&self, doc: &SearchDocument, filters: &HashMap<String, serde_json::Value>) -> SearchResult<bool> {
        for (key, filter_value) in filters {
            if let Some(doc_value) = doc.metadata.get(key) {
                if doc_value != filter_value {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Ok(true)
    }
    
    fn generate_highlights(&self, doc: &SearchDocument, query: &ParsedQuery) -> Vec<String> {
        let mut highlights = Vec::new();
        let content = &doc.content;
        
        for term in &query.terms {
            if let Some(index) = content.to_lowercase().find(&term.to_lowercase()) {
                let start = index.saturating_sub(20).max(0);
                let end = (index + term.len() + 20).min(content.len());
                let snippet = &content[start..end];
                highlights.push(format!("...{}...", snippet));
            }
        }
        
        highlights
    }
    
    fn generate_summary(&self, content: &str, terms: &[String]) -> String {
        if content.len() <= 200 {
            return content.to_string();
        }
        
        let mut summary = content[..200].to_string();
        summary.push_str("...");
        summary
    }
    
    fn score_and_rank_results(&self, results: Vec<SearchResultItem>, query: &ParsedQuery) -> SearchResult<Vec<SearchResultItem>> {
        // Sort by relevance score
        let mut sorted = results;
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        Ok(sorted)
    }
    
    fn apply_sorting(&self, results: Vec<SearchResultItem>, sort_by: &SearchSort) -> SearchResult<Vec<SearchResultItem>> {
        match sort_by {
            SearchSort::Relevance => {
                let mut sorted = results;
                sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
                Ok(sorted)
            }
            SearchSort::DateModified => {
                let mut sorted = results;
                sorted.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
                Ok(sorted)
            }
            SearchSort::DateCreated => {
                let mut sorted = results;
                sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                Ok(sorted)
            }
            SearchSort::Title => {
                let mut sorted = results;
                sorted.sort_by(|a, b| a.title.cmp(&b.title));
                Ok(sorted)
            }
            SearchSort::ToolType => {
                let mut sorted = results;
                sorted.sort_by(|a, b| a.tool_type.cmp(&b.tool_type));
                Ok(sorted)
            }
        }
    }
    
    fn apply_pagination(&self, results: Vec<SearchResultItem>, offset: usize, limit: usize) -> Vec<SearchResultItem> {
        results.into_iter()
            .skip(offset)
            .take(limit)
            .collect()
    }
    
    fn generate_facets(&self, results: &[SearchResultItem]) -> SearchResult<HashMap<String, Vec<FacetItem>>> {
        let mut tool_type_counts = HashMap::new();
        let mut date_facet_counts = BTreeMap::new();
        
        for result in results {
            // Count by tool type
            *tool_type_counts.entry(format!("{:?}", result.tool_type)).or_insert(0) += 1;
            
            // Count by modification date (year-month)
            if let Ok(duration) = result.last_modified.duration_since(UNIX_EPOCH) {
                let seconds = duration.as_secs();
                let years = seconds / (365 * 24 * 60 * 60);
                let months = (seconds % (365 * 24 * 60 * 60)) / (30 * 24 * 60 * 60);
                let year_month = format!("{}-{:02}", 1970 + years as u64, months + 1);
                *date_facet_counts.entry(year_month).or_insert(0) += 1;
            }
        }
        
        let mut facets = HashMap::new();
        
        // Tool type facets
        let tool_facets: Vec<_> = tool_type_counts.into_iter()
            .map(|(value, count)| FacetItem {
                value,
                count,
                selected: false,
            })
            .collect();
        facets.insert("tool_type".to_string(), tool_facets);
        
        // Date facets
        let date_facets: Vec<_> = date_facet_counts.into_iter()
            .map(|(value, count)| FacetItem {
                value,
                count,
                selected: false,
            })
            .collect();
        facets.insert("modified_date".to_string(), date_facets);
        
        Ok(facets)
    }
    
    fn generate_suggestions(&self, query: &str, results: &[SearchResultItem]) -> SearchResult<Vec<String>> {
        if !results.is_empty() {
            return Ok(vec![]);
        }
        
        // If no results, provide alternative suggestions
        let mut suggestions = Vec::new();
        
        // Suggest broader search terms
        if query.contains(' ') {
            suggestions.push(query.split_whitespace().take(2).collect::<Vec<_>>().join(" "));
        }
        
        // Suggest related terms from existing content
        if results.len() < 5 {
            suggestions.push(format!("Try searching for: \"{}\"", query));
        }
        
        Ok(suggestions)
    }
    
    fn extract_tool_data(&self, project_path: &Path, tool_type: &ToolType) -> SearchResult<Vec<SearchDocument>> {
        let mut documents = Vec::new();
        
        match tool_type {
            ToolType::Hierarchy => {
                // Extract hierarchy data from content/hierarchy/ directory
                let hierarchy_path = project_path.join("content").join("hierarchy");
                if hierarchy_path.exists() {
                    documents.extend(self.scan_directory_for_documents(&hierarchy_path, tool_type.clone())?);
                }
            }
            ToolType::Codex => {
                let codex_path = project_path.join("content").join("codex");
                if codex_path.exists() {
                    documents.extend(self.scan_directory_for_documents(&codex_path, tool_type.clone())?);
                }
            }
            ToolType::Notes => {
                let notes_path = project_path.join("content").join("notes");
                if notes_path.exists() {
                    documents.extend(self.scan_directory_for_documents(&notes_path, tool_type.clone())?);
                }
            }
            ToolType::Research => {
                let research_path = project_path.join("content").join("research");
                if research_path.exists() {
                    documents.extend(self.scan_directory_for_documents(&research_path, tool_type.clone())?);
                }
            }
            ToolType::Plot => {
                let plot_path = project_path.join("content").join("plot");
                if plot_path.exists() {
                    documents.extend(self.scan_directory_for_documents(&plot_path, tool_type.clone())?);
                }
            }
            ToolType::Analysis => {
                let analysis_path = project_path.join("content").join("analysis");
                if analysis_path.exists() {
                    documents.extend(self.scan_directory_for_documents(&analysis_path, tool_type.clone())?);
                }
            }
            ToolType::All => {
                // Search all tool types
                for tt in [ToolType::Hierarchy, ToolType::Codex, ToolType::Notes, 
                          ToolType::Research, ToolType::Plot, ToolType::Analysis] {
                    documents.extend(self.extract_tool_data(project_path, &tt)?);
                }
            }
        }
        
        Ok(documents)
    }
    
    fn scan_directory_for_documents(&self, dir_path: &Path, tool_type: ToolType) -> SearchResult<Vec<SearchDocument>> {
        let mut documents = Vec::new();
        
        if dir_path.is_dir() {
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    if let Some(content) = self.read_text_file(&path)? {
                        let metadata = fs::metadata(&path)?;
                        let modified_at = metadata.modified()?.into();
                        
                        documents.push(SearchDocument {
                            id: Uuid::new_v4().to_string(),
                            title: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                            content,
                            metadata: HashMap::new(),
                            created_at: modified_at,
                            modified_at,
                        });
                    }
                }
            }
        }
        
        Ok(documents)
    }
    
    fn read_text_file(&self, path: &Path) -> SearchResult<Option<String>> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(Some(content)),
            Err(_) => Ok(None), // Skip files that can't be read as text
        }
    }
    
    fn build_index_from_documents(&self, documents: Vec<SearchDocument>, tool_type: ToolType) -> SearchResult<SearchIndex> {
        let mut inverted_index = HashMap::new();
        
        for doc in &documents {
            let content_lower = doc.content.to_lowercase();
            let title_lower = doc.title.to_lowercase();
            let combined_text = format!("{} {}", title_lower, content_lower);
            
            // Build inverted index
            for (pos, word) in combined_text.split_whitespace().enumerate() {
                let word = word.trim();
                if !word.is_empty() {
                    inverted_index.entry(word.to_string())
                        .or_insert_with(Vec::new)
                        .push(DocumentRef {
                            document_id: doc.id.clone(),
                            frequency: 1, // Simplified
                            positions: vec![pos],
                        });
                }
            }
        }
        
        let documents_map: HashMap<String, SearchDocument> = documents
            .into_iter()
            .map(|doc| (doc.id.clone(), doc))
            .collect();
        
        Ok(SearchIndex {
            tool_type,
            documents: documents_map,
            inverted_index,
            last_updated: SystemTime::now(),
        })
    }
    
    fn apply_changes_to_index(&self, index: &mut SearchIndex, changes: Vec<IndexChange>) -> SearchResult<()> {
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
        
        index.last_updated = SystemTime::now();
        Ok(())
    }
    
    fn generate_search_suggestions(&self, partial_query: &str, limit: usize, index_manager: &IndexManager) -> SearchResult<Vec<String>> {
        let mut suggestions = Vec::new();
        let partial_lower = partial_query.to_lowercase();
        
        // Collect terms from all indices
        let mut all_terms = HashMap::new();
        for index in index_manager.indices.values() {
            for term in index.inverted_index.keys() {
                if term.starts_with(&partial_lower) && term.len() > partial_lower.len() {
                    *all_terms.entry(term.clone()).or_insert(0) += 1;
                }
            }
        }
        
        // Sort by frequency and take top results
        let mut sorted_terms: Vec<_> = all_terms.into_iter().collect();
        sorted_terms.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (term, _) in sorted_terms.into_iter().take(limit) {
            suggestions.push(term);
        }
        
        Ok(suggestions)
    }
}

/// Index change for incremental updates
#[derive(Debug, Clone)]
pub enum IndexChange {
    AddDocument(SearchDocument),
    UpdateDocument(String, SearchDocument),
    RemoveDocument(String),
}

/// Parsed query structure
#[derive(Debug, Clone)]
struct ParsedQuery {
    terms: Vec<String>,
    phrases: Vec<String>,
}

impl IndexManager {
    fn new(project_path: PathBuf) -> SearchResult<Self> {
        Ok(Self {
            indices: HashMap::new(),
            project_path,
        })
    }
}

/// Global search engine instance
use once_cell::sync::Lazy;
use std::sync::Mutex;

static SEARCH_ENGINE: Lazy<Mutex<Option<SearchEngine>>> = Lazy::new(|| Mutex::new(None));

/// Initialize global search engine
pub fn init_search_engine(config: SearchConfig, project_path: PathBuf) -> SearchResult<()> {
    let mut engine = SEARCH_ENGINE.lock().unwrap();
    *engine = Some(SearchEngine::new(config, project_path)?);
    Ok(())
}

/// Get global search engine instance
pub async fn get_search_engine() -> Option<SearchEngine> {
    let engine = SEARCH_ENGINE.lock().unwrap();
    engine.clone()
}

/// Initialize search engine with defaults
pub fn init_search_engine_with_defaults(project_path: PathBuf) -> SearchResult<()> {
    init_search_engine(SearchConfig::default(), project_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_engine_creation() {
        let engine = SearchEngine::new_with_defaults(PathBuf::from("test_project")).unwrap();
        let stats = engine.get_statistics().await.unwrap();
        assert_eq!(stats.index_count, 0);
    }

    #[test]
    fn test_query_parsing() {
        let config = SearchConfig::default();
        let engine = SearchEngine::new(config, PathBuf::from("test")).unwrap();
        
        let parsed = engine.parse_query("hello world").unwrap();
        assert_eq!(parsed.terms.len(), 2);
        assert_eq!(parsed.terms[0], "hello");
        assert_eq!(parsed.terms[1], "world");
    }
}