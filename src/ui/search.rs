//! Search Interface
//!
//! This module provides search functionality across all writing tools.
//!
//! NOTE: This file has been updated to remove Egui dependencies and focus on Slint-only implementation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::search::SearchService;
use crate::database::models::codex::CodexEntry;
use crate::ui::tools::hierarchy_base::HierarchyItem;

/// Search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tool_type: SearchToolType,
    pub relevance_score: f32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchToolType {
    Hierarchy,
    Codex,
    Notes,
    Research,
    Plot,
    Analysis,
}

/// Search interface for Slint integration
pub struct SearchInterface {
    /// Search service for performing searches
    search_service: Option<SearchService>,
    /// Current search query
    current_query: String,
    /// Search filters
    filters: SearchFilters,
    /// Search results
    results: Vec<SearchResultItem>,
    /// Selected result index
    selected_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SearchFilters {
    pub tool_types: Vec<SearchToolType>,
    pub date_range: Option<(String, String)>,
    pub tags: Vec<String>,
    pub limit: usize,
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            tool_types: vec![
                SearchToolType::Hierarchy,
                SearchToolType::Codex,
                SearchToolType::Notes,
                SearchToolType::Research,
                SearchToolType::Plot,
                SearchToolType::Analysis,
            ],
            date_range: None,
            tags: Vec::new(),
            limit: 100,
        }
    }
}

impl SearchInterface {
    /// Create a new search interface
    pub fn new() -> Self {
        Self {
            search_service: None,
            current_query: String::new(),
            filters: SearchFilters::default(),
            results: Vec::new(),
            selected_index: None,
        }
    }
    
    /// Set the search service
    pub fn set_search_service(&mut self, service: SearchService) {
        self.search_service = Some(service);
    }
    
    /// Perform a search
    pub async fn perform_search(&mut self, query: &str) -> Result<(), String> {
        if let Some(search_service) = &self.search_service {
            self.current_query = query.to_string();
            let results = search_service.search(query, &self.filters).await?;
            self.results = results;
            self.selected_index = None;
            Ok(())
        } else {
            Err("Search service not available".to_string())
        }
    }
    
    /// Get current search results
    pub fn get_results(&self) -> &[SearchResultItem] {
        &self.results
    }
    
    /// Get selected result
    pub fn get_selected_result(&self) -> Option<&SearchResultItem> {
        if let Some(index) = self.selected_index {
            self.results.get(index)
        } else {
            None
        }
    }
    
    /// Select a result by index
    pub fn select_result(&mut self, index: usize) {
        if index < self.results.len() {
            self.selected_index = Some(index);
        }
    }
    
    /// Clear search results
    pub fn clear_results(&mut self) {
        self.results.clear();
        self.selected_index = None;
        self.current_query.clear();
    }
    
    /// Update search filters
    pub fn update_filters(&mut self, filters: SearchFilters) {
        self.filters = filters;
    }
    
    /// Get current query
    pub fn get_current_query(&self) -> &str {
        &self.current_query
    }
    
    /// Export search results
    pub fn export_results(&self) -> String {
        format!("Exporting {} search results for query: {}", 
                self.results.len(), self.current_query)
    }
    
    /// Get search statistics
    pub fn get_search_stats(&self) -> SearchStats {
        let mut stats = std::collections::HashMap::new();
        
        for result in &self.results {
            *stats.entry(result.tool_type.clone()).or_insert(0) += 1;
        }
        
        SearchStats {
            total_results: self.results.len(),
            results_by_tool: stats,
            query: self.current_query.clone(),
        }
    }
}

/// Search statistics
#[derive(Debug)]
pub struct SearchStats {
    pub total_results: usize,
    pub results_by_tool: std::collections::HashMap<SearchToolType, usize>,
    pub query: String,
}

impl Default for SearchInterface {
    fn default() -> Self {
        Self::new()
    }
}

// Note: All Egui-based UI methods have been removed
// Search interface is now handled through Slint components