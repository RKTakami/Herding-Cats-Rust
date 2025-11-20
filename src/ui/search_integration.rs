//! Search Capabilities UI Integration
//!
//! Provides comprehensive search functionality with FTS5 integration,
//! real-time search results, and advanced filtering capabilities.
//! Integrates with the SearchService for enterprise-grade search capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, RwLock};
use tracing::info;

// Import database types and services
use crate::database_app_state::DatabaseAppState;
use crate::database::search_service::{SearchOptions, SearchService, SearchStatistics};

/// Search result item for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISearchResult {
    pub document_id: String,
    pub title: String,
    pub snippet: String,
    pub relevance_score: f32,
    pub project_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub document_type: String,
    pub is_bookmarked: bool,
    pub metadata: Option<String>, // Keep as Option<String> to match SearchResult
}

impl From<crate::database::search_service::SearchResult> for UISearchResult {
    fn from(result: crate::database::search_service::SearchResult) -> Self {
        Self {
            document_id: result.document_id.to_string(),
            title: result.title,
            snippet: result.snippet, // Keep as String, not Option<String>
            relevance_score: result.relevance_score,
            project_id: "unknown".to_string(), // Default value since project_id not available
            created_at: "unknown".to_string(), // Default value since created_at not available
            updated_at: "unknown".to_string(), // Default value since updated_at not available
            document_type: "text".to_string(), // Default value since document_type not available
            is_bookmarked: false,    // Would come from user preferences
            metadata: None,          // Default value since metadata not available
        }
    }
}

/// Search UI state management
#[derive(Debug, Clone)]
pub struct SearchUIState {
    pub current_query: String,
    pub search_options: SearchOptions,
    pub search_history: Vec<String>,
    pub recent_searches: Vec<UISearchResult>,
    pub current_results: Vec<UISearchResult>,
    pub selected_result: Option<String>,
    pub is_searching: bool,
    pub search_error: Option<String>,
    pub search_statistics: Option<SearchStatistics>,
    pub auto_search_enabled: bool,
    pub search_suggestions: Vec<String>,
    pub filters_active: bool,
    pub result_count: usize,
    pub current_page: u32,
    pub total_pages: u32,
}

impl Default for SearchUIState {
    fn default() -> Self {
        Self {
            current_query: String::new(),
            search_options: SearchOptions::default(),
            search_history: Vec::new(),
            recent_searches: Vec::new(),
            current_results: Vec::new(),
            selected_result: None,
            is_searching: false,
            search_error: None,
            search_statistics: None,
            auto_search_enabled: true,
            search_suggestions: Vec::new(),
            filters_active: false,
            result_count: 0,
            current_page: 1,
            total_pages: 1,
        }
    }
}

/// Search integration manager
pub struct SearchIntegrationManager {
    /// Reference to the central database application state
    pub database_state: Arc<RwLock<DatabaseAppState>>,

    /// Search UI state
    pub search_state: Arc<RwLock<SearchUIState>>,

    /// Search service instance
    pub search_service: Option<Arc<RwLock<SearchService>>>,

    /// Search history limit
    pub max_search_history: usize,

    /// Recent results limit
    pub max_recent_results: usize,

    /// Auto-search delay (milliseconds)
    pub auto_search_delay: Duration,

    /// Search analytics tracking
    pub search_analytics: Arc<RwLock<SearchAnalytics>>,

    /// Search event channel
    pub search_event_sender: Option<mpsc::UnboundedSender<SearchEvent>>,

    /// Error messages for user feedback
    pub error_message: Option<String>,

    /// Success messages for user feedback
    pub success_message: Option<String>,

    /// Loading state indicator
    pub is_loading: bool,

    /// Event callbacks for UI updates
    pub on_search_started: Option<Box<dyn Fn() + Send + Sync>>,
    pub on_search_completed: Option<Box<dyn Fn(&Vec<UISearchResult>) + Send + Sync>>,
    pub on_search_error: Option<Box<dyn Fn(&str) + Send + Sync>>,
    pub on_result_selected: Option<Box<dyn Fn(&str) + Send + Sync>>,
}

/// Search analytics tracking
#[derive(Debug, Clone)]
pub struct SearchAnalytics {
    pub total_searches: u64,
    pub successful_searches: u64,
    pub failed_searches: u64,
    pub total_results_found: u64,
    pub average_search_time: Duration,
    pub popular_terms: HashMap<String, u32>,
    pub search_duration_history: Vec<Duration>,
    pub last_search_time: Option<SystemTime>,
}

impl Default for SearchAnalytics {
    fn default() -> Self {
        Self {
            total_searches: 0,
            successful_searches: 0,
            failed_searches: 0,
            total_results_found: 0,
            average_search_time: Duration::from_millis(0),
            popular_terms: HashMap::new(),
            search_duration_history: Vec::new(),
            last_search_time: None,
        }
    }
}

/// Search events for async processing
#[derive(Debug, Clone)]
pub enum SearchEvent {
    PerformSearch(String, SearchOptions),
    GetSuggestions(String),
    UpdateStatistics,
}

impl SearchIntegrationManager {
    /// Create a new search integration manager
    pub fn new(database_state: Arc<RwLock<DatabaseAppState>>) -> Self {
        let (tx, _rx) = mpsc::unbounded_channel();

        Self {
            database_state,
            search_state: Arc::new(RwLock::new(SearchUIState::default())),
            search_service: None,
            max_search_history: 50,
            max_recent_results: 20,
            auto_search_delay: Duration::from_millis(500),
            search_analytics: Arc::new(RwLock::new(SearchAnalytics::default())),
            search_event_sender: Some(tx),
            error_message: None,
            success_message: None,
            is_loading: false,
            on_search_started: None,
            on_search_completed: None,
            on_search_error: None,
            on_result_selected: None,
        }
    }

    /// Initialize the search integration manager
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing search integration manager...");
        self.is_loading = true;

        // Initialize search service
        {
            let db_state = self.database_state.read().await;
            if let Some(ref service_container) = db_state.service_container {
                if let Some(search_service) = service_container.search_service() {
                    self.search_service = Some(search_service);
                }
            }
        }

        if self.search_service.is_none() {
            return Err("Search service not available".into());
        }

        // Load search history from preferences (would be implemented)
        self.load_search_history().await?;

        self.is_loading = false;
        info!("Search integration manager initialized successfully");
        Ok(())
    }

    /// Perform a search
    pub async fn search(
        &self,
        query: &str,
        options: Option<SearchOptions>,
    ) -> Result<Vec<UISearchResult>, Box<dyn std::error::Error>> {
        info!("Performing search: {}", query);

        let start_time = SystemTime::now();

        // Validate search service
        let search_service = self
            .search_service
            .as_ref()
            .ok_or("Search service not available")?;

        // Update UI state
        {
            let mut state = self.search_state.write().await;
            state.is_searching = true;
            state.search_error = None;
            state.current_query = query.to_string();
        }

        if let Some(callback) = &self.on_search_started {
            callback();
        }

        // Perform search
        let search_options = options.unwrap_or_default();
        let search_options_clone = search_options.clone();
        let search_results = search_service
            .read()
            .await
            .search_documents_advanced(query, Some(search_options))
            .await
            .map_err(|e| format!("Search failed: {}", e))?;

        // Convert to UI results
        let ui_results: Vec<UISearchResult> = search_results
            .into_iter()
            .map(|result| UISearchResult {
                document_id: result.document_id.to_string(),
                title: result.title,
                snippet: result.snippet,
                relevance_score: result.relevance_score,
                project_id: "unknown".to_string(),
                created_at: "unknown".to_string(),
                updated_at: "unknown".to_string(),
                document_type: "text".to_string(),
                is_bookmarked: false,
                metadata: None,
            })
            .collect();

        // Update UI state
        {
            let mut state = self.search_state.write().await;
            state.is_searching = false;
            state.current_results = ui_results.clone();
            state.result_count = ui_results.len();
            state.total_pages =
                ((ui_results.len() as f32) / search_options_clone.limit as f32).ceil() as u32;
            state.current_page = 1;
            state.filters_active = search_options_clone.project_filter.is_some()
                || search_options_clone.document_type_filter.is_some()
                || search_options_clone.date_range.is_some();
        }

        // Update analytics
        let _ = self
            .update_search_analytics(query, &ui_results, start_time)
            .await;

        // Add to search history
        self.add_to_search_history(query).await;

        if let Some(callback) = &self.on_search_completed {
            callback(&ui_results);
        }

        let duration = start_time.elapsed().unwrap_or_default();
        info!(
            "Search completed in {:?}: {} results found",
            duration,
            ui_results.len()
        );

        Ok(ui_results)
    }

    /// Get search suggestions
    pub async fn get_suggestions(
        &self,
        partial_query: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let search_service = self
            .search_service
            .as_ref()
            .ok_or("Search service not available")?;

        let suggestions = search_service
            .read()
            .await
            .get_search_suggestions(partial_query, 10)
            .await
            .map_err(|e| format!("Failed to get suggestions: {}", e))?;

        // Update UI state with suggestions
        {
            let mut state = self.search_state.write().await;
            state.search_suggestions = suggestions.clone();
        }

        Ok(suggestions)
    }

    /// Get search statistics
    pub async fn get_search_statistics(
        &self,
    ) -> Result<SearchStatistics, Box<dyn std::error::Error>> {
        let search_service = self
            .search_service
            .as_ref()
            .ok_or("Search service not available")?;

        let stats = search_service
            .read()
            .await
            .get_search_statistics()
            .await
            .map_err(|e| format!("Failed to get search statistics: {}", e))?;

        // Update UI state
        {
            let mut state = self.search_state.write().await;
            state.search_statistics = Some(stats.clone());
        }

        Ok(stats)
    }

    /// Update search analytics
    async fn update_search_analytics(
        &self,
        query: &str,
        results: &[UISearchResult],
        start_time: SystemTime,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut analytics = self.search_analytics.write().await;

        analytics.total_searches += 1;
        analytics.successful_searches += 1;
        analytics.total_results_found += results.len() as u64;

        // Update popular terms
        let term_count = analytics
            .popular_terms
            .entry(query.to_string())
            .or_insert(0);
        *term_count += 1;

        // Update timing
        if let Ok(duration) = start_time.elapsed() {
            analytics.search_duration_history.push(duration);
            if analytics.search_duration_history.len() > 100 {
                analytics.search_duration_history.remove(0);
            }

            // Calculate average
            let total_duration = analytics
                .search_duration_history
                .iter()
                .fold(Duration::from_millis(0), |acc, d| acc + *d);
            analytics.average_search_time =
                total_duration / analytics.search_duration_history.len() as u32;
        }

        analytics.last_search_time = Some(start_time);

        Ok(())
    }

    /// Add query to search history
    async fn add_to_search_history(&self, query: &str) {
        if query.trim().is_empty() {
            return;
        }

        let mut state = self.search_state.write().await;

        // Remove existing entry if present
        state.search_history.retain(|q| q != query);

        // Add to front
        state.search_history.insert(0, query.to_string());

        // Limit history size
        if state.search_history.len() > self.max_search_history {
            state.search_history.truncate(self.max_search_history);
        }
    }

    /// Load search history (placeholder - would load from preferences)
    async fn load_search_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would load from user preferences
        // For now, initialize with empty history
        let mut state = self.search_state.write().await;
        state.search_history.clear();
        Ok(())
    }

    /// Select a search result
    pub async fn select_result(&self, result_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = self.search_state.write().await;
        state.selected_result = Some(result_id.to_string());

        if let Some(callback) = &self.on_result_selected {
            callback(result_id);
        }

        Ok(())
    }

    /// Clear search results
    pub async fn clear_results(&self) {
        let mut state = self.search_state.write().await;
        state.current_results.clear();
        state.selected_result = None;
        state.result_count = 0;
        state.current_page = 1;
        state.total_pages = 1;
    }

    /// Update search options
    pub async fn update_search_options(&self, options: SearchOptions) {
        let mut state = self.search_state.write().await;
        state.search_options = options;
    }

    /// Get current search state
    pub async fn get_search_state(&self) -> SearchUIState {
        let state = self.search_state.read().await;
        state.clone()
    }

    /// Clear error and success messages
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }

    /// Set an error message
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }

    /// Set a success message
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }

    /// Set event callbacks
    pub fn set_callbacks(
        &mut self,
        on_search_started: Option<Box<dyn Fn() + Send + Sync>>,
        on_search_completed: Option<Box<dyn Fn(&Vec<UISearchResult>) + Send + Sync>>,
        on_search_error: Option<Box<dyn Fn(&str) + Send + Sync>>,
        on_result_selected: Option<Box<dyn Fn(&str) + Send + Sync>>,
    ) {
        self.on_search_started = on_search_started;
        self.on_search_completed = on_search_completed;
        self.on_search_error = on_search_error;
        self.on_result_selected = on_result_selected;
    }

    /// Get search analytics
    pub async fn get_analytics(&self) -> SearchAnalytics {
        let analytics = self.search_analytics.read().await;
        analytics.clone()
    }

    /// Update search index (trigger manual index update)
    pub async fn update_search_index(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let search_service = self
            .search_service
            .as_ref()
            .ok_or("Search service not available")?;

        search_service
            .read()
            .await
            .update_search_index()
            .await
            .map_err(|e| format!("Failed to update search index: {}", e))?;

        self.set_success("Search index updated successfully".to_string());
        Ok(())
    }

    /// Get popular search terms
    pub async fn get_popular_terms(
        &self,
        limit: usize,
    ) -> Result<Vec<(String, u32)>, Box<dyn std::error::Error>> {
        let search_service = self
            .search_service
            .as_ref()
            .ok_or("Search service not available")?;

        let terms = search_service
            .read()
            .await
            .get_popular_terms(limit)
            .await
            .map_err(|e| format!("Failed to get popular terms: {}", e))?;

        Ok(terms)
    }

    /// Track search click (for analytics)
    pub async fn track_search_click(
        &self,
        document_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let search_service = self
            .search_service
            .as_ref()
            .ok_or("Search service not available")?;

        search_service
            .read()
            .await
            .track_click(&document_id.parse()?)
            .await
            .map_err(|e| format!("Failed to track search click: {}", e))?;

        Ok(())
    }
}

impl Default for SearchIntegrationManager {
    fn default() -> Self {
        Self::new(Arc::new(RwLock::new(DatabaseAppState::new())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_search_ui_state_creation() {
        let state = SearchUIState::default();

        assert_eq!(state.current_query, "");
        assert_eq!(state.search_history.len(), 0);
        assert_eq!(state.current_results.len(), 0);
        assert!(!state.is_searching);
        assert!(state.search_error.is_none());
        assert!(state.auto_search_enabled);
        assert_eq!(state.current_page, 1);
        assert_eq!(state.total_pages, 1);
    }

    #[tokio::test]
    async fn test_search_result_conversion() {
        use uuid::Uuid;

        // Build a minimal SearchResult consistent with current struct definition
        use crate::SearchResult;
        let search_result = SearchResult {
            document_id: Uuid::new_v4(),
            title: "Test Document".to_string(),
            similarity_score: 0.95,
            snippet: "This is a test document".to_string(),
            chunk_index: 0,
            start_char: 0,
            end_char: 20,
        };

        // Convert to UISearchResult via From implementation
        let ui_result: UISearchResult = search_result.into();

        // Validate mapped fields
        assert_eq!(ui_result.title, "Test Document");
        assert_eq!(ui_result.snippet, "This is a test document");
        assert!((ui_result.relevance_score - 0.95).abs() < f32::EPSILON);

        // Metadata defaults
        assert_eq!(ui_result.project_id, "unknown");
        assert_eq!(ui_result.document_type, "text");
        assert!(!ui_result.is_bookmarked);
    }

    #[tokio::test]
    async fn test_search_analytics_creation() {
        let analytics = SearchAnalytics::default();

        assert_eq!(analytics.total_searches, 0);
        assert_eq!(analytics.successful_searches, 0);
        assert_eq!(analytics.failed_searches, 0);
        assert_eq!(analytics.total_results_found, 0);
        assert_eq!(analytics.popular_terms.len(), 0);
        assert!(analytics.last_search_time.is_none());
    }

    #[tokio::test]
    async fn test_search_integration_manager_creation() {
        let db_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let manager = SearchIntegrationManager::new(db_state);

        assert!(!manager.is_loading);
        assert!(manager.error_message.is_none());
        assert!(manager.success_message.is_none());
        assert!(manager.search_service.is_none());
        assert_eq!(manager.max_search_history, 50);
        assert_eq!(manager.auto_search_delay.as_millis(), 500);
    }
}
