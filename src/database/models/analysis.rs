//! Analysis Data Models
//!
//! Database models and types for the Analysis tool, providing structure analysis
//! for different writing types with drag-and-drop integration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Available writing types for analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WritingType {
    Story,
    ResearchPaper,
    Article,
    Essay,
    Script,
    Poem,
    Other,
}

impl WritingType {
    pub fn display_name(&self) -> &'static str {
        match self {
            WritingType::Story => "Story/Fiction",
            WritingType::ResearchPaper => "Research Paper",
            WritingType::Article => "Article",
            WritingType::Essay => "Essay",
            WritingType::Script => "Script/Screenplay",
            WritingType::Poem => "Poetry",
            WritingType::Other => "Other",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            WritingType::Story => "Fictional narrative with characters, plot, and setting",
            WritingType::ResearchPaper => "Academic research with thesis, evidence, and conclusion",
            WritingType::Article => "Informative or persuasive non-fiction writing",
            WritingType::Essay => "Shorter academic or personal writing piece",
            WritingType::Script => "Screenplay or stage play with dialogue and action",
            WritingType::Poem => "Poetic form with meter, rhyme, and imagery",
            WritingType::Other => "Other writing format",
        }
    }
}

/// Analysis field types that can be dragged between tools
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AnalysisFieldType {
    Character,
    Setting,
    PlotPoint,
    Theme,
    ThesisStatement,
    Evidence,
    CounterArgument,
    Conclusion,
    Introduction,
    Body,
    Transition,
}

impl AnalysisFieldType {
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisFieldType::Character => "Character",
            AnalysisFieldType::Setting => "Setting",
            AnalysisFieldType::PlotPoint => "Plot Point",
            AnalysisFieldType::Theme => "Theme",
            AnalysisFieldType::ThesisStatement => "Thesis Statement",
            AnalysisFieldType::Evidence => "Evidence",
            AnalysisFieldType::CounterArgument => "Counter Argument",
            AnalysisFieldType::Conclusion => "Conclusion",
            AnalysisFieldType::Introduction => "Introduction",
            AnalysisFieldType::Body => "Body",
            AnalysisFieldType::Transition => "Transition",
        }
    }
}

/// Analysis field database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisField {
    pub id: Uuid,
    pub analysis_id: Uuid,
    pub field_type: AnalysisFieldType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Analysis data database model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
    pub writing_type: WritingType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Analysis with related fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisWithFields {
    pub analysis: Analysis,
    pub fields: Vec<AnalysisField>,
    pub insights: Vec<String>,
}

impl Default for Analysis {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id: Uuid::nil(),
            title: "New Analysis".to_string(),
            description: "Analyze your writing structure here".to_string(),
            writing_type: WritingType::Story,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Database query parameters for analysis
#[derive(Debug, Clone)]
pub struct AnalysisQuery {
    pub project_id: Option<Uuid>,
    pub writing_type: Option<WritingType>,
    pub search_query: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Analysis statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStats {
    pub total_analyses: i64,
    pub by_writing_type: std::collections::HashMap<String, i64>,
    pub total_fields: i64,
    pub avg_fields_per_analysis: f64,
}

/// Create analysis table SQL
pub const CREATE_ANALYSIS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    writing_type writing_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Indexes
    INDEX idx_analyses_project_id (project_id),
    INDEX idx_analyses_writing_type (writing_type),
    INDEX idx_analyses_created_at (created_at)
);
"#;

/// Create analysis fields table SQL
pub const CREATE_ANALYSIS_FIELDS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS analysis_fields (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    analysis_id UUID NOT NULL REFERENCES analyses(id) ON DELETE CASCADE,
    field_type analysis_field_type NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Indexes
    INDEX idx_analysis_fields_analysis_id (analysis_id),
    INDEX idx_analysis_fields_field_type (field_type)
);
"#;

/// Create analysis triggers SQL
pub const CREATE_ANALYSIS_TRIGGERS_SQL: &str = r#"
-- Trigger to update updated_at timestamp for analyses
CREATE TRIGGER update_analyses_updated_at 
    BEFORE UPDATE ON analyses 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger to update updated_at timestamp for analysis_fields
CREATE TRIGGER update_analysis_fields_updated_at 
    BEFORE UPDATE ON analysis_fields 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
"#;

/// Insert analysis SQL
pub const INSERT_ANALYSIS_SQL: &str = r#"
INSERT INTO analyses (id, project_id, title, description, writing_type, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING *;
"#;

/// Update analysis SQL
pub const UPDATE_ANALYSIS_SQL: &str = r#"
UPDATE analyses 
SET title = $2, description = $3, writing_type = $4, updated_at = $5
WHERE id = $1 AND project_id = $6
RETURNING *;
"#;

/// Get analysis by ID SQL
pub const GET_ANALYSIS_SQL: &str = r#"
SELECT * FROM analyses WHERE id = $1 AND project_id = $2;
"#;

/// Get analyses by project SQL
pub const GET_ANALYSES_BY_PROJECT_SQL: &str = r#"
SELECT * FROM analyses 
WHERE project_id = $1 
ORDER BY updated_at DESC
LIMIT $2 OFFSET $3;
"#;

/// Delete analysis SQL
pub const DELETE_ANALYSIS_SQL: &str = r#"
DELETE FROM analyses WHERE id = $1 AND project_id = $2;
"#;

/// Insert analysis field SQL
pub const INSERT_ANALYSIS_FIELD_SQL: &str = r#"
INSERT INTO analysis_fields (id, analysis_id, field_type, content, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;
"#;

/// Update analysis field SQL
pub const UPDATE_ANALYSIS_FIELD_SQL: &str = r#"
UPDATE analysis_fields 
SET field_type = $2, content = $3, updated_at = $4
WHERE id = $1 AND analysis_id = $5
RETURNING *;
"#;

/// Get analysis fields SQL
pub const GET_ANALYSIS_FIELDS_SQL: &str = r#"
SELECT * FROM analysis_fields WHERE analysis_id = $1 ORDER BY field_type, created_at;
"#;

/// Delete analysis field SQL
pub const DELETE_ANALYSIS_FIELD_SQL: &str = r#"
DELETE FROM analysis_fields WHERE id = $1 AND analysis_id = $2;
"#;

/// Search analysis fields SQL
pub const SEARCH_ANALYSIS_FIELDS_SQL: &str = r#"
SELECT af.* FROM analysis_fields af
JOIN analyses a ON af.analysis_id = a.id
WHERE a.project_id = $1 
AND (
    af.content ILIKE $2 
    OR af.field_type::text ILIKE $2
)
ORDER BY af.created_at DESC
LIMIT $3 OFFSET $4;
"#;

/// Analysis statistics SQL
pub const GET_ANALYSIS_STATS_SQL: &str = r#"
SELECT 
    COUNT(DISTINCT a.id) as total_analyses,
    COUNT(DISTINCT af.id) as total_fields,
    AVG(field_counts.field_count) as avg_fields_per_analysis
FROM analyses a
LEFT JOIN (
    SELECT analysis_id, COUNT(*) as field_count
    FROM analysis_fields 
    GROUP BY analysis_id
) field_counts ON a.id = field_counts.analysis_id
WHERE a.project_id = $1;
"#;

/// Analysis stats by writing type SQL
pub const GET_ANALYSIS_STATS_BY_TYPE_SQL: &str = r#"
SELECT 
    writing_type,
    COUNT(*) as count
FROM analyses 
WHERE project_id = $1
GROUP BY writing_type
ORDER BY count DESC;
"#;
