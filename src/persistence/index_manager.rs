//! Index Manager for Multi-Level Project Indexing
//! 
//! This module provides comprehensive indexing capabilities for all writing tools,
//! including full-text search, cross-tool references, and performance optimization.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, BTreeSet};
use std::path::{PathBuf, Path};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Individual index entry for search and cross-referencing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub id: Uuid,
    pub tool_type: String,
    pub content_type: ContentType,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub keywords: Vec<String>,
    pub tags: Vec<String>,
    pub references: Vec<IndexReference>,
    pub referenced_by: Vec<IndexReference>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub word_count: usize,
    pub hash: String,
}

/// Content type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ContentType {
    Document,
    Chapter,
    Scene,
    Character,
    Location,
    PlotPoint,
    Note,
    Research,
    Object,
    Time,
    Place,
    Worldbuilding,
    Analysis,
}

/// Cross-reference between index entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexReference {
    pub target_id: Uuid,
    pub target_tool_type: String,
    pub reference_type: ReferenceType,
    pub strength: f32, // 0.0 to 1.0
    pub context: String,
}

/// Types of references between content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    Mentions,
    RelatedTo,
    PartOf,
    Contains,
    DependsOn,
    Contradicts,
    Supports,
    SimilarTo,
}

/// Index metadata for management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub index_version: u32,
    pub total_entries: usize,
    pub tool_distribution: HashMap<String, usize>,
    pub last_rebuilt: SystemTime,
    pub index_size_bytes: usize,
    pub average_word_count: f32,
    pub cross_references: usize,
}

/// Index statistics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexStatistics {
    pub total_items: usize,
    pub total_size_bytes: usize,
    pub average_query_time_ms: f32,
    pub total_queries: u64,
    pub cache_hit_rate: f32,
    pub index_versions: HashMap<u32, usize>,
    pub tool_breakdown: HashMap<String, usize>,
}

/// Main index manager coordinating all indexing operations
#[derive(Debug)]
pub struct IndexManager {
    project_path: PathBuf,
    indexes: Arc<RwLock<HashMap<String, ToolIndex>>>,
    master_index: Arc<RwLock<MasterIndex>>,
    cross_refs: Arc<RwLock<CrossReferenceManager>>,
}

/// Tool-specific index
#[derive(Debug)]
struct ToolIndex {
    tool_type: String,
    entries: HashMap<Uuid, IndexEntry>,
    inverted_index: HashMap<String, BTreeSet<Uuid>>,
    tag_index: HashMap<String, BTreeSet<Uuid>>,
    content_type_index: HashMap<ContentType, BTreeSet<Uuid>>,
    last_updated: SystemTime,
}

/// Master index across all tools
#[derive(Debug)]
struct MasterIndex {
    entries: HashMap<Uuid, IndexEntry>,
    full_text_index: HashMap<String, BTreeSet<Uuid>>,
    keyword_index: HashMap<String, BTreeSet<Uuid>>,
    temporal_index: BTreeMap<SystemTime, BTreeSet<Uuid>>,
    last_updated: SystemTime,
}

/// Cross-reference management
#[derive(Debug)]
struct CrossReferenceManager {
    references: HashMap<Uuid, Vec<IndexReference>>,
    reverse_references: HashMap<Uuid, Vec<IndexReference>>,
    reference_types: HashMap<ReferenceType, BTreeSet<(Uuid, Uuid)>>,
}

/// Index operation result
#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("Entry not found: {0}")]
    EntryNotFound(Uuid),
    
    #[error("Tool type not supported: {0}")]
    UnsupportedToolType(String),
    
    #[error("Index corruption detected")]
    IndexCorruption,
    
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
    
    #[error("Invalid reference: {0}")]
    InvalidReference(String),
    
    #[error("Index too large: {0}")]
    IndexTooLarge(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Performance error: {0}")]
    Performance(String),
}

/// Result type for index operations
pub type IndexResult<T> = Result<T, IndexError>;

impl IndexManager {
    /// Create new index manager
    pub fn new(project_path: PathBuf) -> IndexResult<Self> {
        // Ensure index directory exists
        let index_dir = project_path.join("index");
        fs::create_dir_all(&index_dir)?;
        
        let indexes = Arc::new(RwLock::new(HashMap::new()));
        let master_index = Arc::new(RwLock::new(MasterIndex::new()));
        let cross_refs = Arc::new(RwLock::new(CrossReferenceManager::new()));
        
        Ok(Self {
            project_path,
            indexes,
            master_index,
            cross_refs,
        })
    }
    
    /// Initialize project indexes
    pub fn initialize_project_indexes(&self, metadata: &super::ProjectMetadata) -> IndexResult<()> {
        // Create tool-specific indexes
        let tool_types = ["hierarchy", "codex", "notes", "research", "plot", "analysis"];
        
        // Initialize in-memory indexes
        // In a full implementation, these would be loaded from disk
        
        Ok(())
    }
    
    /// Add new index entry
    pub async fn add_entry(&self, entry: IndexEntry) -> IndexResult<()> {
        let mut indexes = self.indexes.write().await;
        let mut master_index = self.master_index.write().await;
        let mut cross_refs = self.cross_refs.write().await;
        
        // Add to tool-specific index
        let tool_index = indexes
            .entry(entry.tool_type.clone())
            .or_insert_with(|| ToolIndex::new(entry.tool_type.clone()));
        
        tool_index.add_entry(entry.clone())?;
        
        // Add to master index
        master_index.add_entry(entry.clone())?;
        
        // Update cross-references
        cross_refs.update_references(&entry)?;
        
        // Persist changes
        self.persist_indexes().await?;
        
        Ok(())
    }
    
    /// Update existing index entry
    pub async fn update_entry(&self, entry_id: Uuid, updated_entry: IndexEntry) -> IndexResult<()> {
        let mut indexes = self.indexes.write().await;
        let mut master_index = self.master_index.write().await;
        
        // Update in tool-specific index
        let mut found = false;
        for tool_index in indexes.values_mut() {
            if tool_index.update_entry(entry_id, updated_entry.clone())? {
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(IndexError::EntryNotFound(entry_id));
        }
        
        // Update master index
        master_index.update_entry(updated_entry)?;
        
        // Persist changes
        self.persist_indexes().await?;
        
        Ok(())
    }
    
    /// Remove index entry
    pub async fn remove_entry(&self, entry_id: Uuid) -> IndexResult<()> {
        let mut indexes = self.indexes.write().await;
        let mut master_index = self.master_index.write().await;
        let mut cross_refs = self.cross_refs.write().await;
        
        // Remove from tool-specific index
        let mut found = false;
        for tool_index in indexes.values_mut() {
            if tool_index.remove_entry(entry_id) {
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(IndexError::EntryNotFound(entry_id));
        }
        
        // Remove from master index
        master_index.remove_entry(entry_id)?;
        
        // Remove cross-references
        cross_refs.remove_references(entry_id)?;
        
        // Persist changes
        self.persist_indexes().await?;
        
        Ok(())
    }
    
    /// Search across all indexes
    pub async fn search(&self, query: &str, tool_types: Option<Vec<String>>, limit: usize) -> IndexResult<Vec<IndexEntry>> {
        let master_index = self.master_index.read().await;
        let results = master_index.search(query, tool_types, limit)?;
        Ok(results)
    }
    
    /// Get entries by tool type
    pub async fn get_entries_by_tool(&self, tool_type: &str) -> IndexResult<Vec<IndexEntry>> {
        let indexes = self.indexes.read().await;
        if let Some(tool_index) = indexes.get(tool_type) {
            Ok(tool_index.get_all_entries())
        } else {
            Ok(vec![])
        }
    }
    
    /// Get entries by content type
    pub async fn get_entries_by_content_type(&self, content_type: ContentType) -> IndexResult<Vec<IndexEntry>> {
        let master_index = self.master_index.read().await;
        let results = master_index.get_by_content_type(content_type)?;
        Ok(results)
    }
    
    /// Get cross-references for an entry
    pub async fn get_references(&self, entry_id: Uuid) -> IndexResult<Vec<IndexReference>> {
        let cross_refs = self.cross_refs.read().await;
        let references = cross_refs.get_references(entry_id);
        let reverse_references = cross_refs.get_reverse_references(entry_id);
        
        let mut all_references = references;
        all_references.extend(reverse_references);
        
        Ok(all_references)
    }
    
    /// Find entries with specific tags
    pub async fn find_by_tags(&self, tags: Vec<String>) -> IndexResult<Vec<IndexEntry>> {
        let master_index = self.master_index.read().await;
        let results = master_index.find_by_tags(tags)?;
        Ok(results)
    }
    
    /// Get recent entries
    pub async fn get_recent_entries(&self, since: SystemTime, limit: usize) -> IndexResult<Vec<IndexEntry>> {
        let master_index = self.master_index.read().await;
        let results = master_index.get_recent_entries(since, limit)?;
        Ok(results)
    }
    
    /// Rebuild all indexes
    pub async fn rebuild_all_indexes(&self) -> IndexResult<()> {
        // Clear existing indexes
        let mut indexes = self.indexes.write().await;
        let mut master_index = self.master_index.write().await;
        let mut cross_refs = self.cross_refs.write().await;
        
        indexes.clear();
        master_index.clear();
        cross_refs.clear();
        
        // Scan project directory for all tool data
        let all_entries = self.scan_project_for_entries().await?;
        
        // Rebuild indexes
        for entry in all_entries {
            let tool_index = indexes
                .entry(entry.tool_type.clone())
                .or_insert_with(|| ToolIndex::new(entry.tool_type.clone()));
            
            tool_index.add_entry(entry.clone())?;
            master_index.add_entry(entry)?;
        }
        
        // Update cross-references
        cross_refs.build_references(&master_index.entries.values().cloned().collect())?;
        
        // Persist indexes
        self.persist_indexes().await?;
        
        Ok(())
    }
    
    /// Rebuild specific tool index
    pub async fn rebuild_tool_index(&self, tool_type: &str) -> IndexResult<()> {
        let mut indexes = self.indexes.write().await;
        let mut master_index = self.master_index.write().await;
        
        // Remove existing entries for this tool
        if let Some(tool_index) = indexes.get_mut(tool_type) {
            for entry_id in tool_index.get_all_entry_ids() {
                master_index.remove_entry(entry_id)?;
            }
            tool_index.clear();
        }
        
        // Scan for new entries
        let entries = self.scan_tool_directory(tool_type).await?;
        
        // Add new entries
        for entry in entries {
            let tool_index = indexes
                .entry(tool_type.to_string())
                .or_insert_with(|| ToolIndex::new(tool_type.to_string()));
            
            tool_index.add_entry(entry.clone())?;
            master_index.add_entry(entry)?;
        }
        
        // Update cross-references
        let mut cross_refs = self.cross_refs.write().await;
        cross_refs.build_references(&master_index.entries.values().cloned().collect())?;
        
        // Persist changes
        self.persist_indexes().await?;
        
        Ok(())
    }
    
    /// Get index statistics
    pub fn get_index_statistics(&self) -> IndexResult<IndexStatistics> {
        // This would collect actual statistics from the indexes
        // For now, return placeholder data
        
        Ok(IndexStatistics {
            total_items: 0,
            total_size_bytes: 0,
            average_query_time_ms: 0.0,
            total_queries: 0,
            cache_hit_rate: 0.0,
            index_versions: HashMap::new(),
            tool_breakdown: HashMap::new(),
        })
    }
    
    /// Get all index entries (for search integration)
    pub async fn get_all_entries(&self) -> IndexResult<Vec<IndexEntry>> {
        let master_index = self.master_index.read().await;
        Ok(master_index.entries.values().cloned().collect())
    }
    
    /// Update tool index (called from persistence manager)
    pub fn update_tool_index(&self, tool_type: &str, data: &super::ToolDataType) -> IndexResult<()> {
        // This would convert ToolDataType to IndexEntry and update the index
        // Implementation would depend on the specific data structure
        Ok(())
    }
    
    // Private helper methods
    
    async fn persist_indexes(&self) -> IndexResult<()> {
        let index_dir = self.project_path.join("index");
        
        // Persist master index
        let master_index = self.master_index.read().await;
        let master_path = index_dir.join("master_index.json");
        let content = serde_json::to_string_pretty(&*master_index)?;
        fs::write(master_path, content)?;
        
        // Persist tool-specific indexes
        let indexes = self.indexes.read().await;
        for (tool_type, tool_index) in indexes.iter() {
            let tool_path = index_dir.join(format!("{}_index.json", tool_type));
            let content = serde_json::to_string_pretty(tool_index)?;
            fs::write(tool_path, content)?;
        }
        
        // Persist cross-references
        let cross_refs = self.cross_refs.read().await;
        let refs_path = index_dir.join("cross_references.json");
        let content = serde_json::to_string_pretty(&*cross_refs)?;
        fs::write(refs_path, content)?;
        
        Ok(())
    }
    
    async fn scan_project_for_entries(&self) -> IndexResult<Vec<IndexEntry>> {
        let mut entries = Vec::new();
        
        // Scan each tool directory
        for tool_type in ["hierarchy", "codex", "notes", "research", "plot", "analysis"] {
            let tool_entries = self.scan_tool_directory(tool_type).await?;
            entries.extend(tool_entries);
        }
        
        Ok(entries)
    }
    
    async fn scan_tool_directory(&self, tool_type: &str) -> IndexResult<Vec<IndexEntry>> {
        let mut entries = Vec::new();
        let tool_dir = self.project_path.join("content").join(tool_type);
        
        if !tool_dir.exists() {
            return Ok(entries);
        }
        
        // Scan directory for data files
        if tool_dir.is_dir() {
            for entry in fs::read_dir(&tool_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(entry_data) = self.load_entry_from_file(&path, tool_type)? {
                        entries.push(entry_data);
                    }
                }
            }
        }
        
        Ok(entries)
    }
    
    fn load_entry_from_file(&self, path: &Path, tool_type: &str) -> IndexResult<Option<IndexEntry>> {
        match fs::read_to_string(path) {
            Ok(content) => {
                let json_value: serde_json::Value = serde_json::from_str(&content)?;
                
                // Convert JSON to IndexEntry based on tool type
                let entry = self.json_to_index_entry(json_value, tool_type, path)?;
                Ok(Some(entry))
            }
            Err(_) => Ok(None), // Skip files that can't be read
        }
    }
    
    fn json_to_index_entry(&self, json_value: serde_json::Value, tool_type: &str, path: &Path) -> IndexResult<IndexEntry> {
        // This would convert JSON data to IndexEntry based on tool type
        // For now, create a basic entry
        
        let metadata = fs::metadata(path)?;
        let modified_at = metadata.modified()?.into();
        
        let title = json_value.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| path.file_stem().unwrap_or_default().to_string_lossy())
            .to_string();
        
        let content = json_value.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let summary = if content.len() > 200 {
            format!("{}...", &content[..200])
        } else {
            content.clone()
        };
        
        let word_count = content.split_whitespace().count();
        let hash = self.calculate_content_hash(&content);
        
        Ok(IndexEntry {
            id: Uuid::new_v4(),
            tool_type: tool_type.to_string(),
            content_type: self.determine_content_type(&json_value),
            title,
            content,
            summary,
            keywords: Vec::new(),
            tags: Vec::new(),
            references: Vec::new(),
            referenced_by: Vec::new(),
            metadata: HashMap::new(),
            created_at: modified_at,
            modified_at,
            word_count,
            hash,
        })
    }
    
    fn determine_content_type(&self, json_value: &serde_json::Value) -> ContentType {
        // Determine content type based on JSON structure or tool type
        // This is a simplified implementation
        ContentType::Document
    }
    
    fn calculate_content_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

// ToolIndex implementation
impl ToolIndex {
    fn new(tool_type: String) -> Self {
        Self {
            tool_type,
            entries: HashMap::new(),
            inverted_index: HashMap::new(),
            tag_index: HashMap::new(),
            content_type_index: HashMap::new(),
            last_updated: SystemTime::now(),
        }
    }
    
    fn add_entry(&mut self, entry: IndexEntry) -> IndexResult<()> {
        if self.entries.contains_key(&entry.id) {
            return Err(IndexError::DuplicateEntry(entry.id.to_string()));
        }
        
        self.entries.insert(entry.id, entry.clone());
        self.update_inverted_index(&entry)?;
        self.update_tag_index(&entry)?;
        self.update_content_type_index(&entry)?;
        self.last_updated = SystemTime::now();
        
        Ok(())
    }
    
    fn update_entry(&mut self, entry_id: Uuid, entry: IndexEntry) -> IndexResult<bool> {
        if let Some(existing_entry) = self.entries.get(&entry_id) {
            // Remove from old indexes
            self.remove_from_indexes(existing_entry);
            
            // Add updated entry
            self.entries.insert(entry_id, entry.clone());
            self.update_inverted_index(&entry)?;
            self.update_tag_index(&entry)?;
            self.update_content_type_index(&entry)?;
            
            self.last_updated = SystemTime::now();
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    fn remove_entry(&mut self, entry_id: Uuid) -> bool {
        if let Some(entry) = self.entries.remove(&entry_id) {
            self.remove_from_indexes(&entry);
            self.last_updated = SystemTime::now();
            true
        } else {
            false
        }
    }
    
    fn get_all_entries(&self) -> Vec<IndexEntry> {
        self.entries.values().cloned().collect()
    }
    
    fn get_all_entry_ids(&self) -> Vec<Uuid> {
        self.entries.keys().cloned().collect()
    }
    
    fn clear(&mut self) {
        self.entries.clear();
        self.inverted_index.clear();
        self.tag_index.clear();
        self.content_type_index.clear();
        self.last_updated = SystemTime::now();
    }
    
    fn update_inverted_index(&mut self, entry: &IndexEntry) -> IndexResult<()> {
        // Update word-based inverted index
        for word in entry.content.to_lowercase().split_whitespace() {
            let word = word.trim();
            if !word.is_empty() {
                self.inverted_index
                    .entry(word.to_string())
                    .or_insert_with(BTreeSet::new)
                    .insert(entry.id);
            }
        }
        
        // Update keyword index
        for keyword in &entry.keywords {
            self.inverted_index
                .entry(keyword.to_lowercase())
                .or_insert_with(BTreeSet::new)
                .insert(entry.id);
        }
        
        Ok(())
    }
    
    fn update_tag_index(&mut self, entry: &IndexEntry) -> IndexResult<()> {
        for tag in &entry.tags {
            self.tag_index
                .entry(tag.clone())
                .or_insert_with(BTreeSet::new)
                .insert(entry.id);
        }
        Ok(())
    }
    
    fn update_content_type_index(&mut self, entry: &IndexEntry) -> IndexResult<()> {
        self.content_type_index
            .entry(entry.content_type.clone())
            .or_insert_with(BTreeSet::new)
            .insert(entry.id);
        Ok(())
    }
    
    fn remove_from_indexes(&mut self, entry: &IndexEntry) {
        // Remove from inverted index
        for word in entry.content.to_lowercase().split_whitespace() {
            let word = word.trim();
            if let Some(set) = self.inverted_index.get_mut(word) {
                set.remove(&entry.id);
                if set.is_empty() {
                    self.inverted_index.remove(word);
                }
            }
        }
        
        // Remove from tag index
        for tag in &entry.tags {
            if let Some(set) = self.tag_index.get_mut(tag) {
                set.remove(&entry.id);
                if set.is_empty() {
                    self.tag_index.remove(tag);
                }
            }
        }
        
        // Remove from content type index
        if let Some(set) = self.content_type_index.get_mut(&entry.content_type) {
            set.remove(&entry.id);
            if set.is_empty() {
                self.content_type_index.remove(&entry.content_type);
            }
        }
    }
}

// MasterIndex implementation
impl MasterIndex {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            full_text_index: HashMap::new(),
            keyword_index: HashMap::new(),
            temporal_index: BTreeMap::new(),
            last_updated: SystemTime::now(),
        }
    }
    
    fn add_entry(&mut self, entry: IndexEntry) -> IndexResult<()> {
        if self.entries.contains_key(&entry.id) {
            return Err(IndexError::DuplicateEntry(entry.id.to_string()));
        }
        
        self.entries.insert(entry.id, entry.clone());
        self.update_full_text_index(&entry)?;
        self.update_keyword_index(&entry)?;
        self.update_temporal_index(&entry)?;
        self.last_updated = SystemTime::now();
        
        Ok(())
    }
    
    fn update_entry(&mut self, entry: IndexEntry) -> IndexResult<()> {
        self.entries.insert(entry.id, entry);
        self.last_updated = SystemTime::now();
        Ok(())
    }
    
    fn remove_entry(&mut self, entry_id: Uuid) -> IndexResult<()> {
        self.entries.remove(&entry_id);
        self.last_updated = SystemTime::now();
        Ok(())
    }
    
    fn clear(&mut self) {
        self.entries.clear();
        self.full_text_index.clear();
        self.keyword_index.clear();
        self.temporal_index.clear();
        self.last_updated = SystemTime::now();
    }
    
    fn search(&self, query: &str, tool_types: Option<Vec<String>>, limit: usize) -> IndexResult<Vec<IndexEntry>> {
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();
        
        let mut candidate_ids = BTreeSet::new();
        
        // Find candidates based on full-text search
        for term in query_terms {
            if let Some(ids) = self.full_text_index.get(term) {
                if candidate_ids.is_empty() {
                    candidate_ids = ids.clone();
                } else {
                    // Intersect for AND search
                    candidate_ids = candidate_ids.intersection(ids).cloned().collect();
                }
            }
        }
        
        // Filter by tool types if specified
        if let Some(ref types) = tool_types {
            candidate_ids.retain(|id| {
                if let Some(entry) = self.entries.get(id) {
                    types.contains(&entry.tool_type)
                } else {
                    false
                }
            });
        }
        
        // Score and rank results
        let mut results: Vec<(f32, IndexEntry)> = candidate_ids
            .into_iter()
            .filter_map(|id| {
                self.entries.get(&id).map(|entry| {
                    let score = self.calculate_relevance_score(entry, &query_terms);
                    (score, entry.clone())
                })
            })
            .collect();
        
        // Sort by relevance score
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return top results
        Ok(results.into_iter()
            .take(limit)
            .map(|(_, entry)| entry)
            .collect())
    }
    
    fn get_by_content_type(&self, content_type: ContentType) -> IndexResult<Vec<IndexEntry>> {
        Ok(self.entries
            .values()
            .filter(|entry| entry.content_type == content_type)
            .cloned()
            .collect())
    }
    
    fn find_by_tags(&self, tags: Vec<String>) -> IndexResult<Vec<IndexEntry>> {
        let mut candidate_ids = BTreeSet::new();
        
        for tag in tags {
            if let Some(ids) = self.get_ids_by_tag(&tag) {
                if candidate_ids.is_empty() {
                    candidate_ids = ids;
                } else {
                    candidate_ids = candidate_ids.intersection(&ids).cloned().collect();
                }
            }
        }
        
        Ok(candidate_ids
            .into_iter()
            .filter_map(|id| self.entries.get(&id).cloned())
            .collect())
    }
    
    fn get_recent_entries(&self, since: SystemTime, limit: usize) -> IndexResult<Vec<IndexEntry>> {
        let recent_ids: BTreeSet<_> = self.temporal_index
            .range(since..)
            .flat_map(|(_, ids)| ids.clone())
            .collect();
        
        Ok(recent_ids
            .into_iter()
            .filter_map(|id| self.entries.get(&id).cloned())
            .take(limit)
            .collect())
    }
    
    fn update_full_text_index(&mut self, entry: &IndexEntry) -> IndexResult<()> {
        for word in entry.content.to_lowercase().split_whitespace() {
            let word = word.trim();
            if !word.is_empty() {
                self.full_text_index
                    .entry(word.to_string())
                    .or_insert_with(BTreeSet::new)
                    .insert(entry.id);
            }
        }
        Ok(())
    }
    
    fn update_keyword_index(&mut self, entry: &IndexEntry) -> IndexResult<()> {
        for keyword in &entry.keywords {
            self.keyword_index
                .entry(keyword.to_lowercase())
                .or_insert_with(BTreeSet::new)
                .insert(entry.id);
        }
        Ok(())
    }
    
    fn update_temporal_index(&mut self, entry: &IndexEntry) -> IndexResult<()> {
        self.temporal_index
            .entry(entry.modified_at)
            .or_insert_with(BTreeSet::new)
            .insert(entry.id);
        Ok(())
    }
    
    fn get_ids_by_tag(&self, tag: &str) -> Option<BTreeSet<Uuid>> {
        // This would search through all entries for the tag
        // For now, return None
        None
    }
    
    fn calculate_relevance_score(&self, entry: &IndexEntry, query_terms: &[&str]) -> f32 {
        let content_lower = entry.content.to_lowercase();
        let title_lower = entry.title.to_lowercase();
        
        let mut score = 0.0;
        
        for term in query_terms {
            let term_lower = term.to_lowercase();
            
            // Title matches get higher score
            if title_lower.contains(&term_lower) {
                score += 2.0;
            }
            
            // Content matches
            if content_lower.contains(&term_lower) {
                score += 1.0;
            }
            
            // Keyword matches get highest score
            if entry.keywords.iter().any(|k| k.to_lowercase() == term_lower) {
                score += 3.0;
            }
        }
        
        // Normalize by content length
        let content_length = entry.content.len() as f32;
        if content_length > 0.0 {
            score /= (content_length / 100.0).sqrt();
        }
        
        score
    }
}

// CrossReferenceManager implementation
impl CrossReferenceManager {
    fn new() -> Self {
        Self {
            references: HashMap::new(),
            reverse_references: HashMap::new(),
            reference_types: HashMap::new(),
        }
    }
    
    fn update_references(&mut self, entry: &IndexEntry) -> IndexResult<()> {
        // Update direct references
        self.references
            .entry(entry.id)
            .or_insert_with(Vec::new)
            .extend(entry.references.clone());
        
        // Update reverse references
        for reference in &entry.references {
            self.reverse_references
                .entry(reference.target_id)
                .or_insert_with(Vec::new)
                .push(IndexReference {
                    target_id: entry.id,
                    target_tool_type: entry.tool_type.clone(),
                    reference_type: reference.reference_type.clone(),
                    strength: reference.strength,
                    context: reference.context.clone(),
                });
        }
        
        Ok(())
    }
    
    fn remove_references(&mut self, entry_id: Uuid) -> IndexResult<()> {
        self.references.remove(&entry_id);
        self.reverse_references.remove(&entry_id);
        
        // Clean up reference type index
        for (_, ref_set) in self.reference_types.iter_mut() {
            ref_set.retain(|(from, to)| *from != entry_id && *to != entry_id);
        }
        
        Ok(())
    }
    
    fn get_references(&self, entry_id: Uuid) -> Vec<IndexReference> {
        self.references.get(&entry_id).cloned().unwrap_or_default()
    }
    
    fn get_reverse_references(&self, entry_id: Uuid) -> Vec<IndexReference> {
        self.reverse_references.get(&entry_id).cloned().unwrap_or_default()
    }
    
    fn build_references(&mut self, entries: &[IndexEntry]) -> IndexResult<()> {
        // Clear existing references
        self.references.clear();
        self.reverse_references.clear();
        self.reference_types.clear();
        
        // Build new references
        for entry in entries {
            self.update_references(entry)?;
        }
        
        Ok(())
    }
    
    fn clear(&mut self) {
        self.references.clear();
        self.reverse_references.clear();
        self.reference_types.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_index_manager_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf()).unwrap();
        
        let entry = IndexEntry {
            id: Uuid::new_v4(),
            tool_type: "test".to_string(),
            content_type: ContentType::Document,
            title: "Test Entry".to_string(),
            content: "This is test content for indexing".to_string(),
            summary: "Test summary".to_string(),
            keywords: vec!["test".to_string()],
            tags: vec!["test".to_string()],
            references: vec![],
            referenced_by: vec![],
            metadata: HashMap::new(),
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
            word_count: 5,
            hash: "test_hash".to_string(),
        };
        
        manager.add_entry(entry).await.unwrap();
    }
}