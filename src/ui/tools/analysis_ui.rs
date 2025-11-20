//! Analysis Tool UI Module
//!
//! Main UI implementation for the Analysis writing tool, providing structure analysis
//! for different writing types (story, research, etc.) with drag-and-drop integration.

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    database::DatabaseResult,
    tools::{
        structure_data::{StructureData, StructureManager, PlotType},
    },
};

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

    /// Get recommended plot types for this writing type
    pub fn recommended_plot_types(&self) -> Vec<PlotType> {
        match self {
            WritingType::Story => vec![
                PlotType::HeroesJourney,
                PlotType::SaveTheCat,
                PlotType::BlakeSnyderBeatSheet,
                PlotType::ThreePart,
            ],
            WritingType::ResearchPaper => vec![
                PlotType::ThreePart,
                PlotType::FreytagsPyramid,
            ],
            WritingType::Article => vec![
                PlotType::ThreePart,
            ],
            WritingType::Essay => vec![
                PlotType::ThreePart,
                PlotType::FreytagsPyramid,
            ],
            WritingType::Script => vec![
                PlotType::SaveTheCat,
                PlotType::BlakeSnyderBeatSheet,
                PlotType::DanHarmonsCircle,
            ],
            WritingType::Poem => vec![
                PlotType::ThreePart,
            ],
            WritingType::Other => vec![
                PlotType::ThreePart,
            ],
        }
    }
}

/// Analysis field types that can be dragged between tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisField {
    /// Story elements
    Character(String),      // Character name
    Setting(String),        // Setting description
    PlotPoint(String),      // Plot point description
    Theme(String),          // Theme description

    /// Research elements
    ThesisStatement(String),
    Evidence(String),
    CounterArgument(String),
    Conclusion(String),

    /// Structural elements
    Introduction(String),
    Body(String),
    ConclusionGeneral(String),
    Transition(String),
}

impl AnalysisField {
    pub fn field_type(&self) -> &'static str {
        match self {
            AnalysisField::Character(_) => "Character",
            AnalysisField::Setting(_) => "Setting",
            AnalysisField::PlotPoint(_) => "Plot Point",
            AnalysisField::Theme(_) => "Theme",
            AnalysisField::ThesisStatement(_) => "Thesis Statement",
            AnalysisField::Evidence(_) => "Evidence",
            AnalysisField::CounterArgument(_) => "Counter Argument",
            AnalysisField::Conclusion(_) => "Conclusion",
            AnalysisField::Introduction(_) => "Introduction",
            AnalysisField::Body(_) => "Body",
            AnalysisField::ConclusionGeneral(_) => "Conclusion",
            AnalysisField::Transition(_) => "Transition",
        }
    }

    pub fn content(&self) -> &String {
        match self {
            AnalysisField::Character(s) |
            AnalysisField::Setting(s) |
            AnalysisField::PlotPoint(s) |
            AnalysisField::Theme(s) |
            AnalysisField::ThesisStatement(s) |
            AnalysisField::Evidence(s) |
            AnalysisField::CounterArgument(s) |
            AnalysisField::Conclusion(s) |
            AnalysisField::Introduction(s) |
            AnalysisField::Body(s) |
            AnalysisField::ConclusionGeneral(s) |
            AnalysisField::Transition(s) => s,
        }
    }
}

/// Analysis data structure
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisData {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
    pub writing_type: WritingType,
    pub recommended_plot_types: Vec<PlotType>,
    pub analysis_fields: Vec<AnalysisField>,
    pub insights: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AnalysisData {
    pub fn new(project_id: Uuid, title: String, description: String, writing_type: WritingType) -> Self {
        let recommended_plot_types = writing_type.recommended_plot_types();
        Self {
            id: Uuid::new_v4(),
            project_id,
            title,
            description,
            writing_type,
            recommended_plot_types,
            analysis_fields: Vec::new(),
            insights: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Add an analysis field
    pub fn add_field(&mut self, field: AnalysisField) {
        self.analysis_fields.push(field);
        self.updated_at = Utc::now();
    }

    /// Remove an analysis field by index
    pub fn remove_field(&mut self, index: usize) -> Result<AnalysisField, String> {
        if index < self.analysis_fields.len() {
            let field = self.analysis_fields.remove(index);
            self.updated_at = Utc::now();
            Ok(field)
        } else {
            Err("Field index out of range".to_string())
        }
    }

    /// Get fields by type
    pub fn get_fields_by_type(&self, field_type: &str) -> Vec<&AnalysisField> {
        self.analysis_fields
            .iter()
            .filter(|field| field.field_type() == field_type)
            .collect()
    }

    /// Generate insights based on current fields
    pub fn generate_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();

        // Character insights
        let characters: Vec<&AnalysisField> = self.get_fields_by_type("Character");
        if characters.len() < 2 {
            insights.push("Consider adding more characters to create richer interactions".to_string());
        } else if characters.len() > 10 {
            insights.push("You have many characters - consider consolidating to avoid confusion".to_string());
        }

        // Setting insights
        let settings: Vec<&AnalysisField> = self.get_fields_by_type("Setting");
        if settings.is_empty() {
            insights.push("Consider adding setting details to ground your story".to_string());
        }

        // Plot insights
        let plot_points: Vec<&AnalysisField> = self.get_fields_by_type("Plot Point");
        if plot_points.len() < 3 {
            insights.push("Consider adding more plot points for better story structure".to_string());
        }

        // Theme insights
        let themes: Vec<&AnalysisField> = self.get_fields_by_type("Theme");
        if themes.is_empty() {
            insights.push("Consider identifying central themes to give your work focus".to_string());
        }

        // Research-specific insights
        if self.writing_type == WritingType::ResearchPaper {
            let thesis = self.get_fields_by_type("Thesis Statement");
            if thesis.is_empty() {
                insights.push("Make sure to clearly state your thesis statement".to_string());
            }

            let evidence = self.get_fields_by_type("Evidence");
            if evidence.len() < 3 {
                insights.push("Consider adding more evidence to support your argument".to_string());
            }
        }

        insights
    }
}

/// Main analysis tool implementation
pub struct AnalysisTool {
    /// Current analysis data
    pub analysis: AnalysisData,

    /// Structure manager for plot integration
    pub structure_manager: StructureManager,

    /// UI state
    pub ui_state: AnalysisUiState,

    /// Drag and drop state
    pub drag_state: DragState,
}

/// Analysis tool UI state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisUiState {
    pub selected_writing_type: WritingType,
    pub show_insights: bool,
    pub show_field_editor: bool,
    pub editing_field_index: Option<usize>,
    pub new_field_type: String,
    pub new_field_content: String,
}

/// Drag and drop state
#[derive(Debug, Default)]
pub struct DragState {
    pub is_dragging: bool,
    pub dragged_field: Option<AnalysisField>,
    pub source_tool: Option<String>,
}

impl Default for AnalysisTool {
    fn default() -> Self {
        Self {
            analysis: AnalysisData::new(
                Uuid::nil(), // Will be set when project is loaded
                "New Analysis".to_string(),
                "Analyze your writing structure here".to_string(),
                WritingType::Story,
            ),
            structure_manager: StructureManager::new(),
            ui_state: AnalysisUiState::default(),
            drag_state: DragState::default(),
        }
    }
}

impl AnalysisTool {
    /// Create a new analysis tool
    pub fn new(project_id: Uuid) -> Self {
        let mut tool = Self::default();
        tool.analysis.project_id = project_id;
        tool
    }

    /// Set the writing type and update recommended plot types
    pub fn set_writing_type(&mut self, writing_type: WritingType) {
        self.analysis.writing_type = writing_type;
        self.analysis.recommended_plot_types = writing_type.recommended_plot_types();
        self.ui_state.selected_writing_type = writing_type;
        self.analysis.updated_at = Utc::now();
    }

    /// Add an analysis field
    pub fn add_analysis_field(&mut self, field_type: String, content: String) -> DatabaseResult<()> {
        let field = match field_type.as_str() {
            "Character" => AnalysisField::Character(content),
            "Setting" => AnalysisField::Setting(content),
            "Plot Point" => AnalysisField::PlotPoint(content),
            "Theme" => AnalysisField::Theme(content),
            "Thesis Statement" => AnalysisField::ThesisStatement(content),
            "Evidence" => AnalysisField::Evidence(content),
            "Counter Argument" => AnalysisField::CounterArgument(content),
            "Conclusion" => AnalysisField::Conclusion(content),
            "Introduction" => AnalysisField::Introduction(content),
            "Body" => AnalysisField::Body(content),
            "Conclusion" => AnalysisField::ConclusionGeneral(content),
            "Transition" => AnalysisField::Transition(content),
            _ => return Err("Unknown field type".into()),
        };

        self.analysis.add_field(field);
        Ok(())
    }

    /// Start dragging a field
    pub fn start_drag(&mut self, field_index: usize, source_tool: String) -> Result<(), String> {
        if let Some(field) = self.analysis.analysis_fields.get(field_index).cloned() {
            self.drag_state.is_dragging = true;
            self.drag_state.dragged_field = Some(field);
            self.drag_state.source_tool = Some(source_tool);
            Ok(())
        } else {
            Err("Field not found".to_string())
        }
    }

    /// Complete drag operation
    pub fn complete_drag(&mut self, target_tool: &str) -> Result<AnalysisField, String> {
        if let Some(field) = self.drag_state.dragged_field.take() {
            self.drag_state.is_dragging = false;
            self.drag_state.source_tool = None;

            // Generate insight about the drag operation
            if let Some(source) = &self.drag_state.source_tool {
                log::info!("Dragged field from {} to {}: {:?}", source, target_tool, field.field_type());
            }

            Ok(field)
        } else {
            Err("No field being dragged".to_string())
        }
    }

    /// Cancel drag operation
    pub fn cancel_drag(&mut self) {
        self.drag_state.is_dragging = false;
        self.drag_state.dragged_field = None;
        self.drag_state.source_tool = None;
    }

    /// Generate updated insights
    pub fn update_insights(&mut self) {
        self.analysis.insights = self.analysis.generate_insights();
    }

    /// Create a structure from analysis (for story writing)
    pub fn create_structure_from_analysis(&mut self) -> Option<Uuid> {
        if self.analysis.writing_type == WritingType::Story && !self.analysis.recommended_plot_types.is_empty() {
            let plot_type = self.analysis.recommended_plot_types[0];
            let structure_id = self.structure_manager.create_structure(
                self.analysis.project_id,
                plot_type,
                format!("Structure: {}", self.analysis.title),
                format!("Generated from analysis: {}", self.analysis.description),
            );

            // Map analysis fields to plot stages
            self.map_fields_to_stages(structure_id);

            Some(structure_id)
        } else {
            None
        }
    }

    /// Map analysis fields to plot structure stages
    fn map_fields_to_stages(&mut self, structure_id: Uuid) {
        if let Some(structure) = self.structure_manager.get_structure_mut(structure_id) {
            // Simple mapping - could be enhanced with AI or more sophisticated logic
            let characters = self.analysis.get_fields_by_type("Character");
            let plot_points = self.analysis.get_fields_by_type("Plot Point");
            let themes = self.analysis.get_fields_by_type("Theme");

            // Add character information to relevant stages
            for (i, stage) in structure.stages.iter_mut().enumerate() {
                if i == 0 && !characters.is_empty() {
                    // First stage often introduces main character
                    let char_content = characters[0].content();
                    stage.content = format!("{}\n\nMain Character: {}", stage.content, char_content);
                }

                if i < plot_points.len() {
                    // Map plot points to stages
                    let plot_content = plot_points[i].content();
                    stage.content = format!("{}\n\nPlot Point: {}", stage.content, plot_content);
                }

                if !themes.is_empty() {
                    // Add themes to all stages
                    let theme_content = themes[0].content();
                    stage.content = format!("{}\n\nTheme: {}", stage.content, theme_content);
                }
            }
        }
    }

    /// Export analysis summary
    pub fn export_summary(&self) -> String {
        let mut summary = format!("# Analysis Summary: {}\n\n", self.analysis.title);
        summary.push_str(&format!("**Description:** {}\n", self.analysis.description));
        summary.push_str(&format!("**Writing Type:** {}\n", self.analysis.writing_type.display_name()));
        summary.push_str(&format!("**Created:** {}\n\n", self.analysis.created_at.format("%Y-%m-%d %H:%M:%S")));

        summary.push_str("## Analysis Fields\n\n");
        for field in &self.analysis.analysis_fields {
            summary.push_str(&format!("### {} - {}\n", field.field_type(), field.content()));
        }

        if !self.analysis.insights.is_empty() {
            summary.push_str("\n## Generated Insights\n\n");
            for insight in &self.analysis.insights {
                summary.push_str(&format!("- {}\n", insight));
            }
        }

        summary.push_str("\n## Recommended Plot Types\n\n");
        for plot_type in &self.analysis.recommended_plot_types {
            summary.push_str(&format!("- **{}:** {}\n", plot_type.display_name(), plot_type.description()));
        }

        summary
    }
}
