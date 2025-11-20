//! Codex Tool Base Module
//!
//! Provides the foundational structures and interfaces for all codex tool components.
//! This module defines the common functionality shared across Story Summary,
//! Character Sheets, Objects, Time, and Place tools.

use crate::database::models::codex_service::CodexService;

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use log::{info, warn, error, debug};
use async_trait::async_trait;
use uuid::Uuid;

use crate::database::{
    models::codex::{*, CodexEntry, CodexEntryType, CodexStatus, CodexSortField, CodexQuery, EnhancedCodexEntry, CodexStatistics, CharacterData, PlaceData, TimeData, ObjectData, StoryData, CodexImportResult},
    DatabaseResult,
};

/// UI state management for codex tools
#[derive(Debug, Clone)]
pub struct CodexUiState {
    /// Currently selected codex entry
    pub selected_entry: Option<CodexEntry>,

    /// Current view mode (list, grid, detail)
    pub view_mode: CodexViewMode,

    /// Filter settings
    pub filter_settings: CodexFilterSettings,

    /// Search query
    pub search_query: String,

    /// Currently editing entry (for inline editing)
    pub editing_entry: Option<CodexEntry>,

    /// Expanded sections (for detailed views)
    pub expanded_sections: std::collections::HashSet<String>,

    /// Recently viewed entries for quick access
    pub recently_viewed: Vec<Uuid>,

    /// Bulk selection state
    pub selected_entries: std::collections::HashSet<Uuid>,
}

impl Default for CodexUiState {
    fn default() -> Self {
        Self {
            selected_entry: None,
            view_mode: CodexViewMode::List,
            filter_settings: CodexFilterSettings::default(),
            search_query: String::new(),
            editing_entry: None,
            expanded_sections: std::collections::HashSet::new(),
            recently_viewed: Vec::new(),
            selected_entries: std::collections::HashSet::new(),
        }
    }
}

/// Different view modes for codex display
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodexViewMode {
    /// List view with titles and summaries
    List,
    /// Grid view with cards
    Grid,
    /// Detailed view with full content
    Detail,
    /// Timeline view (for Time entries)
    Timeline,
    /// Map view (for Place entries)
    Map,
    /// Relationship view (for Character entries)
    Relationships,
}

impl CodexViewMode {
    /// Get display name for the view mode
    pub fn display_name(self) -> &'static str {
        match self {
            CodexViewMode::List => "List",
            CodexViewMode::Grid => "Grid",
            CodexViewMode::Detail => "Detail",
            CodexViewMode::Timeline => "Timeline",
            CodexViewMode::Map => "Map",
            CodexViewMode::Relationships => "Relationships",
        }
    }

    /// Get icon for the view mode
    pub fn icon(self) -> &'static str {
        match self {
            CodexViewMode::List => "📝",
            CodexViewMode::Grid => "🔲",
            CodexViewMode::Detail => "📄",
            CodexViewMode::Timeline => "⏰",
            CodexViewMode::Map => "🗺️",
            CodexViewMode::Relationships => "🔗",
        }
    }

    /// Check if this view mode is available for a specific entry type
    pub fn is_available_for(entry_type: CodexEntryType) -> &'static [CodexViewMode] {
        match entry_type {
            CodexEntryType::StorySummary => &[CodexViewMode::List, CodexViewMode::Detail],
            CodexEntryType::CharacterSheet => &[
                CodexViewMode::List,
                CodexViewMode::Detail,
                CodexViewMode::Relationships
            ],
            CodexEntryType::Object => &[CodexViewMode::List, CodexViewMode::Detail, CodexViewMode::Grid],
            CodexEntryType::Time => &[
                CodexViewMode::List,
                CodexViewMode::Detail,
                CodexViewMode::Timeline
            ],
            CodexEntryType::Place => &[
                CodexViewMode::List,
                CodexViewMode::Detail,
                CodexViewMode::Grid,
                CodexViewMode::Map
            ],
        }
    }
}

/// Filter settings for codex views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexFilterSettings {
    /// Filter by entry type
    pub entry_types: std::collections::HashSet<CodexEntryType>,

    /// Filter by status
    pub statuses: std::collections::HashSet<CodexStatus>,

    /// Only show favorite/starred entries
    pub only_favorites: bool,

    /// Only show recently modified
    pub only_recent: bool,

    /// Custom tag filters
    pub tags: Vec<String>,

    /// Sort field and direction
    pub sort_field: CodexSortField,
    pub sort_desc: bool,
}

impl Default for CodexFilterSettings {
    fn default() -> Self {
        Self {
            entry_types: CodexEntryType::all_types().iter().copied().collect(),
            statuses: [CodexStatus::Draft, CodexStatus::InReview, CodexStatus::Final]
                .iter().copied().collect(),
            only_favorites: false,
            only_recent: false,
            tags: Vec::new(),
            sort_field: CodexSortField::UpdatedAt,
            sort_desc: true,
        }
    }
}

/// Actions that can be performed on codex entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodexAction {
    /// Create a new entry
    Create,
    /// Edit selected entry
    Edit,
    /// Delete selected entry(s)
    Delete,
    /// Duplicate entry
    Duplicate,
    /// Mark as favorite
    ToggleFavorite,
    /// Change status
    ChangeStatus(CodexStatus),
    /// Export entry
    Export,
    /// Import entries
    Import,
    /// Bulk operations
    BulkEdit,
    BulkDelete,
    BulkChangeStatus(CodexStatus),
}

/// Events that can occur in codex tools
#[derive(Debug, Clone)]
pub enum CodexEvent {
    /// Entry was selected
    EntrySelected(Uuid),
    /// Entry was created
    EntryCreated(CodexEntry),
    /// Entry was updated
    EntryUpdated(CodexEntry),
    /// Entry was deleted
    EntryDeleted(Uuid),
    /// View mode changed
    ViewModeChanged(CodexViewMode),
    /// Filter settings changed
    FilterSettingsChanged(CodexFilterSettings),
    /// Search query changed
    SearchQueryChanged(String),
    /// Action performed
    ActionPerformed(CodexAction, Vec<Uuid>),
    /// Bulk operation completed
    BulkOperationCompleted { action: CodexAction, count: usize },
}

/// Base trait for all codex tool implementations
pub trait CodexTool {
    /// Initialize the tool
    fn initialize(&mut self) -> DatabaseResult<()>;

    /// Update the tool state
    fn update(&mut self, delta_time: std::time::Duration) -> DatabaseResult<()>;

    /// Render the tool UI
    fn render(&mut self) -> DatabaseResult<()>;

    /// Handle codex events
    fn handle_event(&mut self, event: CodexEvent) -> DatabaseResult<()>;

    /// Get the current UI state
    fn get_ui_state(&self) -> &CodexUiState;

    /// Get the current UI state mutably
    fn get_ui_state_mut(&mut self) -> &mut CodexUiState;

    /// Get the entry type this tool handles
    fn get_entry_type(&self) -> CodexEntryType;

    /// Create a new entry with default values
    fn create_default_entry(&self, project_id: Uuid) -> CodexEntry;

    /// Validate an entry before saving
    fn validate_entry(&self, entry: &CodexEntry) -> Result<(), String>;

    /// Export entries to various formats
    fn export_entries(&self, entries: &[CodexEntry], format: &str) -> DatabaseResult<Vec<u8>>;

    /// Import entries from data
    fn import_entries(&mut self, data: &[u8], format: &str) -> DatabaseResult<CodexImportResult>;
}

/// Base implementation for common codex tool functionality
pub struct CodexToolBase {
    /// UI state
    pub ui_state: CodexUiState,

    /// Entry type this tool handles
    pub entry_type: CodexEntryType,

    /// Database service
    pub db_service: Arc<RwLock<dyn CodexService>>,

    /// Callback for event notifications
    pub event_callback: Option<Box<dyn Fn(CodexEvent) -> DatabaseResult<()>>>,
}

impl CodexToolBase {
    /// Create a new codex tool base
    pub fn new(
        entry_type: CodexEntryType,
        db_service: Arc<RwLock<dyn CodexService>>,
    ) -> Self {
        Self {
            ui_state: CodexUiState::default(),
            entry_type,
            db_service,
            event_callback: None,
        }
    }

    /// Set the event callback
    pub fn set_event_callback<F>(&mut self, callback: F)
    where
        F: Fn(CodexEvent) -> DatabaseResult<()> + 'static,
    {
        self.event_callback = Some(Box::new(callback));
    }

    /// Notify event callback
    pub fn notify_event(&self, event: CodexEvent) -> DatabaseResult<()> {
        if let Some(callback) = &self.event_callback {
            callback(event)
        } else {
            Ok(())
        }
    }

    /// Update recently viewed entries
    pub fn update_recently_viewed(&mut self, entry_id: Uuid) {
        // Remove if already in list
        self.ui_state.recently_viewed.retain(|id| *id != entry_id);

        // Add to front
        self.ui_state.recently_viewed.insert(0, entry_id);

        // Keep only last 10
        if self.ui_state.recently_viewed.len() > 10 {
            self.ui_state.recently_viewed.truncate(10);
        }
    }

    /// Toggle section expansion
    pub fn toggle_section(&mut self, section_id: &str) {
        if self.ui_state.expanded_sections.contains(section_id) {
            self.ui_state.expanded_sections.remove(section_id);
        } else {
            self.ui_state.expanded_sections.insert(section_id.to_string());
        }
    }

    /// Check if section is expanded
    pub fn is_section_expanded(&self, section_id: &str) -> bool {
        self.ui_state.expanded_sections.contains(section_id)
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.ui_state.selected_entries.clear();
        self.ui_state.selected_entry = None;
    }

    /// Select entry
    pub fn select_entry(&mut self, entry: CodexEntry) -> DatabaseResult<()> {
        self.ui_state.selected_entry = Some(entry.clone());
        self.update_recently_viewed(entry.id);
        self.notify_event(CodexEvent::EntrySelected(entry.id))
    }

    /// Toggle entry selection for bulk operations
    pub fn toggle_entry_selection(&mut self, entry_id: Uuid) {
        if self.ui_state.selected_entries.contains(&entry_id) {
            self.ui_state.selected_entries.remove(&entry_id);
        } else {
            self.ui_state.selected_entries.insert(entry_id);
        }
    }

    /// Get entries matching current filters
    pub async fn get_filtered_entries(&self, project_id: Uuid) -> DatabaseResult<Vec<CodexEntry>> {
        let mut query = CodexQuery::default();
        query.project_id = Some(project_id);
        query.entry_type = Some(self.entry_type);
        query.status = self.ui_state.filter_settings.statuses.iter().next().copied();
        query.search_term = if self.ui_state.search_query.is_empty() {
            None
        } else {
            Some(self.ui_state.search_query.clone())
        };
        query.sort_by = Some(self.ui_state.filter_settings.sort_field);
        query.sort_desc = self.ui_state.filter_settings.sort_desc;
        query.limit = Some(100); // Reasonable default for UI

        self.db_service.read().await.list_entries(&query).await
    }

    /// Create enhanced entry data based on entry type
    pub fn create_enhanced_entry(&self, entry: &CodexEntry) -> EnhancedCodexEntry {
        let mut enhanced = EnhancedCodexEntry::new(
            entry.project_id,
            entry.entry_type,
            entry.title.clone(),
            entry.content.clone(),
        );

        // Copy base entry
        enhanced.base = entry.clone();

        // Initialize type-specific data
        match entry.entry_type {
            CodexEntryType::CharacterSheet => {
                enhanced.character_data = Some(CharacterData {
                    names: vec![entry.title.clone()],
                    physical_description: None,
                    personality_traits: Vec::new(),
                    goals: Vec::new(),
                    fears: Vec::new(),
                    backstory: None,
                    arc: None,
                    relationships: Vec::new(),
                    skills: Vec::new(),
                    inventory: Vec::new(),
                });
            }
            CodexEntryType::Place => {
                enhanced.place_data = Some(PlaceData {
                    alternative_names: Vec::new(),
                    coordinates: None,
                    climate: None,
                    population: None,
                    culture: None,
                    points_of_interest: Vec::new(),
                    history: None,
                    current_events: Vec::new(),
                });
            }
            CodexEntryType::Time => {
                enhanced.time_data = Some(TimeData {
                    start_time: None,
                    end_time: None,
                    duration: None,
                    calendar_system: None,
                    season: None,
                    historical_context: None,
                    era: None,
                });
            }
            CodexEntryType::Object => {
                enhanced.object_data = Some(ObjectData {
                    object_type: None,
                    dimensions: None,
                    weight_materials: None,
                    properties: Vec::new(),
                    current_location: None,
                    history: None,
                    usage: None,
                    value: None,
                });
            }
            CodexEntryType::StorySummary => {
                enhanced.story_data = Some(StoryData {
                    main_plot: None,
                    subplots: Vec::new(),
                    themes: Vec::new(),
                    structure: None,
                    point_of_view: None,
                    tone: None,
                    target_audience: None,
                });
            }
        }

        enhanced
    }

    /// Validate entry content
    pub fn validate_entry_content(&self, entry: &CodexEntry) -> Result<(), String> {
        if entry.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        if entry.title.len() > 200 {
            return Err("Title cannot exceed 200 characters".to_string());
        }

        if entry.content.len() > 500000 {
            return Err("Content cannot exceed 500,000 characters".to_string());
        }

        // Type-specific validation
        match entry.entry_type {
            CodexEntryType::CharacterSheet => {
                // Character-specific validation
                if entry.content.len() < 10 {
                    return Err("Character description should be more detailed".to_string());
                }
            }
            CodexEntryType::Place => {
                // Place-specific validation
                if entry.content.len() < 10 {
                    return Err("Place description should be more detailed".to_string());
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Mock implementation of CodexService for testing
pub struct MockCodexService;

#[async_trait]
impl CodexService for MockCodexService {
    async fn initialize_schema(&self) -> DatabaseResult<()> {
        Ok(())
    }

    async fn create_entry(&self, _entry: &CodexEntry) -> DatabaseResult<uuid::Uuid> {
        Ok(uuid::Uuid::new_v4())
    }

    async fn get_entry(&self, _entry_id: &uuid::Uuid) -> DatabaseResult<Option<CodexEntry>> {
        Ok(None)
    }

    async fn update_entry(&self, _entry: &CodexEntry) -> DatabaseResult<()> {
        Ok(())
    }

    async fn delete_entry(&self, _entry_id: &uuid::Uuid) -> DatabaseResult<()> {
        Ok(())
    }

    async fn list_entries(&self, _query: &CodexQuery) -> DatabaseResult<Vec<CodexEntry>> {
        Ok(Vec::new())
    }

    async fn count_entries(&self, _query: &CodexQuery) -> DatabaseResult<i64> {
        Ok(0)
    }

    async fn get_statistics(&self, _project_id: &uuid::Uuid) -> DatabaseResult<CodexStatistics> {
        Ok(CodexStatistics::default())
    }

    async fn create_enhanced_entry(&self, _entry: &EnhancedCodexEntry) -> DatabaseResult<uuid::Uuid> {
        Ok(uuid::Uuid::new_v4())
    }

    async fn get_enhanced_entry(&self, _entry_id: &uuid::Uuid) -> DatabaseResult<Option<EnhancedCodexEntry>> {
        Ok(None)
    }

    async fn search_entries(&self, _project_id: &uuid::Uuid, _search_term: &str) -> DatabaseResult<Vec<CodexEntry>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_ui_state_default() {
        let state = CodexUiState::default();
        assert!(state.selected_entry.is_none());
        assert!(state.editing_entry.is_none());
        assert_eq!(state.view_mode, CodexViewMode::List);
        assert!(state.search_query.is_empty());
    }

    #[test]
    fn test_view_mode_icons() {
        assert_eq!(CodexViewMode::List.icon(), "📝");
        assert_eq!(CodexViewMode::Grid.icon(), "🔲");
        assert_eq!(CodexViewMode::Detail.icon(), "📄");
        assert_eq!(CodexViewMode::Timeline.icon(), "⏰");
        assert_eq!(CodexViewMode::Map.icon(), "🗺️");
        assert_eq!(CodexViewMode::Relationships.icon(), "🔗");
    }

    #[test]
    fn test_filter_settings_default() {
        let filters = CodexFilterSettings::default();
        assert!(!filters.entry_types.is_empty());
        assert!(!filters.statuses.is_empty());
        assert!(!filters.only_favorites);
        assert!(!filters.only_recent);
        assert_eq!(filters.sort_field, CodexSortField::UpdatedAt);
        assert!(filters.sort_desc);
    }

    #[test]
    fn test_base_tool_creation() {
        let db_service = Arc::new(RwLock::new(MockCodexService));
        let base = CodexToolBase::new(CodexEntryType::CharacterSheet, db_service);

        assert_eq!(base.entry_type, CodexEntryType::CharacterSheet);
        assert!(base.ui_state.selected_entry.is_none());
        assert!(base.event_callback.is_none());
    }
}
