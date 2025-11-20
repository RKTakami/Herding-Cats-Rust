//! Codex Database Models
//!
//! Contains data structures for the Codex writing tool including:
//! - Story Summary
//! - Character Sheets  
//! - Objects
//! - Time
//! - Place

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Main codex entry representing any codex item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexEntry {
    /// Unique identifier
    pub id: Uuid,

    /// Project this codex entry belongs to
    pub project_id: Uuid,

    /// Type of codex entry
    pub entry_type: CodexEntryType,

    /// Human-readable title
    pub title: String,

    /// Detailed content/description
    pub content: String,

    /// Entry status
    pub status: CodexStatus,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,

    /// Whether entry is active/not deleted
    pub is_active: bool,

    /// Optional metadata as JSON for flexible attributes
    pub metadata: Option<String>,

    /// Sort order within the entry type
    pub sort_order: i32,
}

impl CodexEntry {
    /// Create a new codex entry
    pub fn new(
        project_id: Uuid,
        entry_type: CodexEntryType,
        title: String,
        content: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            entry_type,
            title,
            content,
            status: CodexStatus::Draft,
            created_at: now,
            updated_at: now,
            is_active: true,
            metadata: None,
            sort_order: 0,
        }
    }

    /// Update the content of this entry
    pub fn update_content(&mut self, title: String, content: String) -> &mut Self {
        self.title = title;
        self.content = content;
        self.updated_at = Utc::now();
        self
    }

    /// Set metadata for this entry
    pub fn set_metadata(&mut self, metadata: String) -> &mut Self {
        self.metadata = Some(metadata);
        self.updated_at = Utc::now();
        self
    }

    /// Mark entry as deleted
    pub fn delete(&mut self) -> &mut Self {
        self.is_active = false;
        self.updated_at = Utc::now();
        self
    }

    /// Set status
    pub fn set_status(&mut self, status: CodexStatus) -> &mut Self {
        self.status = status;
        self.updated_at = Utc::now();
        self
    }
}

/// Types of codex entries
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodexEntryType {
    /// Story Summary - Overall narrative overview
    StorySummary,
    /// Character Sheet - Individual character details
    CharacterSheet,
    /// Object - In-world items, artifacts, technology
    Object,
    /// Time - Timeline events, calendars, time periods
    Time,
    /// Place - Locations, settings, world geography
    Place,
}

impl CodexEntryType {
    /// Get display name for the entry type
    pub fn display_name(self) -> &'static str {
        match self {
            CodexEntryType::StorySummary => "Story Summary",
            CodexEntryType::CharacterSheet => "Character Sheet",
            CodexEntryType::Object => "Object",
            CodexEntryType::Time => "Time",
            CodexEntryType::Place => "Place",
        }
    }

    /// Get icon for the entry type
    pub fn icon(self) -> &'static str {
        match self {
            CodexEntryType::StorySummary => "📚",
            CodexEntryType::CharacterSheet => "👤",
            CodexEntryType::Object => "📦",
            CodexEntryType::Time => "⏰",
            CodexEntryType::Place => "🌍",
        }
    }

    /// Get description for the entry type
    pub fn description(self) -> &'static str {
        match self {
            CodexEntryType::StorySummary => "Overall narrative summary and plot overview",
            CodexEntryType::CharacterSheet => "Detailed character information and development",
            CodexEntryType::Object => "In-world items, artifacts, and technology",
            CodexEntryType::Time => "Timeline events, calendars, and time periods",
            CodexEntryType::Place => "Locations, settings, and world geography",
        }
    }

    /// Get all entry types
    pub fn all_types() -> &'static [CodexEntryType] {
        &[
            CodexEntryType::StorySummary,
            CodexEntryType::CharacterSheet,
            CodexEntryType::Object,
            CodexEntryType::Time,
            CodexEntryType::Place,
        ]
    }
}

/// Status of codex entries
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodexStatus {
    /// Draft - Still being written
    Draft,
    /// In Review - Ready for feedback
    InReview,
    /// Final - Completed and approved
    Final,
    /// Archived - No longer actively used
    Archived,
}

impl CodexStatus {
    /// Get display name for the status
    pub fn display_name(self) -> &'static str {
        match self {
            CodexStatus::Draft => "Draft",
            CodexStatus::InReview => "In Review",
            CodexStatus::Final => "Final",
            CodexStatus::Archived => "Archived",
        }
    }

    /// Get color class for the status (for UI)
    pub fn color_class(self) -> &'static str {
        match self {
            CodexStatus::Draft => "status-draft",
            CodexStatus::InReview => "status-review",
            CodexStatus::Final => "status-final",
            CodexStatus::Archived => "status-archived",
        }
    }
}

/// Character-specific data for enhanced character sheets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    /// Character name variations
    pub names: Vec<String>,

    /// Physical description
    pub physical_description: Option<String>,

    /// Personality traits
    pub personality_traits: Vec<String>,

    /// Character goals and motivations
    pub goals: Vec<String>,

    /// Character fears and flaws
    pub fears: Vec<String>,

    /// Backstory summary
    pub backstory: Option<String>,

    /// Character arc/development
    pub arc: Option<String>,

    /// Relationships with other characters
    pub relationships: Vec<CharacterRelationship>,

    /// Skills and abilities
    pub skills: Vec<String>,

    /// Inventory/items
    pub inventory: Vec<String>,
}

/// Relationship between characters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRelationship {
    /// ID of the related character
    pub character_id: Uuid,

    /// Type of relationship
    pub relationship_type: String,

    /// Description of the relationship
    pub description: String,

    /// Relationship status (positive, negative, neutral)
    pub sentiment: RelationshipSentiment,
}

/// Sentiment of character relationships
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RelationshipSentiment {
    Positive,
    Negative,
    Neutral,
    Complex,
}

/// Place-specific data for enhanced location tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceData {
    /// Alternative names for this location
    pub alternative_names: Vec<String>,

    /// Geographic coordinates (if applicable)
    pub coordinates: Option<String>,

    /// Climate and weather patterns
    pub climate: Option<String>,

    /// Population details
    pub population: Option<String>,

    /// Cultural significance
    pub culture: Option<String>,

    /// Points of interest
    pub points_of_interest: Vec<String>,

    /// History and lore
    pub history: Option<String>,

    /// Current events affecting this location
    pub current_events: Vec<String>,
}

/// Time-specific data for timeline management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeData {
    /// Start date/time in narrative
    pub start_time: Option<String>,

    /// End date/time in narrative
    pub end_time: Option<String>,

    /// Duration of the event/period
    pub duration: Option<String>,

    /// Calendar system used
    pub calendar_system: Option<String>,

    /// Season or time period
    pub season: Option<String>,

    /// Historical context
    pub historical_context: Option<String>,

    /// Timeline era/age
    pub era: Option<String>,
}

/// Object-specific data for item tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectData {
    /// Object type/category
    pub object_type: Option<String>,

    /// Physical dimensions
    pub dimensions: Option<String>,

    /// Weight and materials
    pub weight_materials: Option<String>,

    /// Magical or special properties
    pub properties: Vec<String>,

    /// Current location/owner
    pub current_location: Option<String>,

    /// Historical significance
    pub history: Option<String>,

    /// Usage instructions or limitations
    pub usage: Option<String>,

    /// Value and rarity
    pub value: Option<String>,
}

/// Codex entry with enhanced data for specific types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCodexEntry {
    /// Base codex entry
    pub base: CodexEntry,

    /// Character-specific data (only for CharacterSheet entries)
    pub character_data: Option<CharacterData>,

    /// Place-specific data (only for Place entries)
    pub place_data: Option<PlaceData>,

    /// Time-specific data (only for Time entries)
    pub time_data: Option<TimeData>,

    /// Object-specific data (only for Object entries)
    pub object_data: Option<ObjectData>,

    /// Story summary-specific data (only for StorySummary entries)
    pub story_data: Option<StoryData>,
}

/// Story-specific data for narrative tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryData {
    /// Main plot summary
    pub main_plot: Option<String>,

    /// Subplots
    pub subplots: Vec<String>,

    /// Themes and motifs
    pub themes: Vec<String>,

    /// Narrative structure
    pub structure: Option<String>,

    /// Point of view
    pub point_of_view: Option<String>,

    /// Tone and style
    pub tone: Option<String>,

    /// Target audience
    pub target_audience: Option<String>,
}

impl EnhancedCodexEntry {
    /// Create a new enhanced codex entry with appropriate data structure
    pub fn new(
        project_id: Uuid,
        entry_type: CodexEntryType,
        title: String,
        content: String,
    ) -> Self {
        let base = CodexEntry::new(project_id, entry_type, title, content);

        Self {
            base,
            character_data: None,
            place_data: None,
            time_data: None,
            object_data: None,
            story_data: None,
        }
    }

    /// Set character data for CharacterSheet entries
    pub fn with_character_data(mut self, data: CharacterData) -> Self {
        self.character_data = Some(data);
        self
    }

    /// Set place data for Place entries
    pub fn with_place_data(mut self, data: PlaceData) -> Self {
        self.place_data = Some(data);
        self
    }

    /// Set time data for Time entries
    pub fn with_time_data(mut self, data: TimeData) -> Self {
        self.time_data = Some(data);
        self
    }

    /// Set object data for Object entries
    pub fn with_object_data(mut self, data: ObjectData) -> Self {
        self.object_data = Some(data);
        self
    }

    /// Set story data for StorySummary entries
    pub fn with_story_data(mut self, data: StoryData) -> Self {
        self.story_data = Some(data);
        self
    }
}

/// Query parameters for filtering codex entries
#[derive(Debug, Default)]
pub struct CodexQuery {
    /// Filter by project ID
    pub project_id: Option<Uuid>,

    /// Filter by entry type
    pub entry_type: Option<CodexEntryType>,

    /// Filter by status
    pub status: Option<CodexStatus>,

    /// Filter by active/inactive
    pub is_active: Option<bool>,

    /// Search in title and content
    pub search_term: Option<String>,

    /// Sort field
    pub sort_by: Option<CodexSortField>,

    /// Sort direction
    pub sort_desc: bool,

    /// Pagination
    pub limit: Option<i64>,
    pub offset: i64,
}

/// Fields that can be used for sorting codex entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodexSortField {
    Title,
    CreatedAt,
    UpdatedAt,
    SortOrder,
    Status,
    EntryType,
}

/// Statistics for codex data
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CodexStatistics {
    /// Total number of codex entries
    pub total_entries: i64,

    /// Entries by type
    pub entries_by_type: std::collections::HashMap<String, i64>,

    /// Entries by status
    pub entries_by_status: std::collections::HashMap<String, i64>,

    /// Total word count across all entries
    pub total_word_count: i64,

    /// Last updated timestamp
    pub last_updated: Option<DateTime<Utc>>,

    /// Breakdown by project
    pub projects_breakdown: std::collections::HashMap<Uuid, i64>,
}

/// Result of a codex import operation
#[derive(Debug, Default)]
pub struct CodexImportResult {
    /// Number of entries successfully imported
    pub imported_count: usize,

    /// Number of entries that failed to import
    pub failed_count: usize,

    /// List of errors encountered
    pub errors: Vec<String>,

    /// Time taken for the import
    pub duration_ms: u128,
}

/// Result of a codex export operation
#[derive(Debug, Default)]
pub struct CodexExportResult {
    /// Number of entries exported
    pub exported_count: usize,

    /// Export format used
    pub format: String,

    /// File path where export was saved
    pub file_path: Option<String>,

    /// Time taken for the export
    pub duration_ms: u128,
}
