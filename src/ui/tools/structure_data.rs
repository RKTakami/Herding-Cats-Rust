//! Structure Data Module for Herding Cats Rust
//!
//! Pure Rust implementation of plot structure data and management.
//! Provides plot types, stages, and data persistence without EGUI dependencies.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{Utc, DateTime};

/// Available plot structure types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlotType {
    ThreePart,
    HeroesJourney,
    SaveTheCat,
    FreytagsPyramid,
    DanHarmonsCircle,
    BlakeSnyderBeatSheet,
}

impl PlotType {
    /// Get the display name for the plot type
    pub fn display_name(&self) -> &'static str {
        match self {
            PlotType::ThreePart => "Three-Part Structure",
            PlotType::HeroesJourney => "Hero's Journey",
            PlotType::SaveTheCat => "Save the Cat",
            PlotType::FreytagsPyramid => "Freytag's Pyramid",
            PlotType::DanHarmonsCircle => "Dan Harmon's Circle",
            PlotType::BlakeSnyderBeatSheet => "Blake Snyder's Beat Sheet",
        }
    }

    /// Get a brief description of the plot type
    pub fn description(&self) -> &'static str {
        match self {
            PlotType::ThreePart => "Classic three-act structure: Setup, Confrontation, Resolution",
            PlotType::HeroesJourney => "Joseph Campbell's monomyth: The hero's adventure and transformation",
            PlotType::SaveTheCat => "Blake Snyder's approach: Make your hero save the cat to endear them to readers",
            PlotType::FreytagsPyramid => "Gustav Freytag's five-part dramatic structure",
            PlotType::DanHarmonsCircle => "Dan Harmon's story circle: You, Need, Go, Search, Find, Take, Return, Changed",
            PlotType::BlakeSnyderBeatSheet => "15-beat structure for compelling storytelling",
        }
    }

    /// Get the plot beats/stages for this plot type
    pub fn get_plot_stages(&self) -> Vec<PlotStage> {
        match self {
            PlotType::ThreePart => vec![
                PlotStage::new("Act I: Setup", "Introduce characters, setting, and inciting incident"),
                PlotStage::new("Act II: Confrontation", "Rising action, obstacles, and character development"),
                PlotStage::new("Act III: Resolution", "Climax and resolution of the story"),
            ],
            PlotType::HeroesJourney => vec![
                PlotStage::new("1. Ordinary World", "Hero's normal life before the adventure"),
                PlotStage::new("2. Call to Adventure", "The hero is presented with a challenge"),
                PlotStage::new("3. Refusal of the Call", "Hero hesitates or refuses the adventure"),
                PlotStage::new("4. Meeting the Mentor", "Hero meets a wise guide or helper"),
                PlotStage::new("5. Crossing the Threshold", "Hero commits to the adventure"),
                PlotStage::new("6. Tests, Allies, Enemies", "Hero faces challenges and makes friends/enemies"),
                PlotStage::new("7. Approach to the Inmost Cave", "Hero approaches the central challenge"),
                PlotStage::new("8. Ordeal", "Hero faces their greatest challenge"),
                PlotStage::new("9. Reward", "Hero claims their reward or treasure"),
                PlotStage::new("10. The Road Back", "Hero begins the journey home"),
                PlotStage::new("11. Resurrection", "Final test and hero's ultimate transformation"),
                PlotStage::new("12. Return with the Elixir", "Hero returns home, changed"),
            ],
            PlotType::SaveTheCat => vec![
                PlotStage::new("1. Opening Image", "Shows the hero's world and nature"),
                PlotStage::new("2. Theme Stated", "What the story is about is stated"),
                PlotStage::new("3. Set-Up", "Establishes the hero's world and problems"),
                PlotStage::new("4. Catalyst", "Inciting incident that changes everything"),
                PlotStage::new("5. Debate", "Hero debates whether to take action"),
                PlotStage::new("6. Break into Two", "Hero commits to the journey"),
                PlotStage::new("7. B Story", "Introduction of the theme and love story"),
                PlotStage::new("8. Fun and Games", "Promise of the premise delivered"),
                PlotStage::new("9. Midpoint", "False victory or defeat changes everything"),
                PlotStage::new("10. Bad Guys Close In", "All is lost moment approaches"),
                PlotStage::new("11. All Is Lost", "The hero hits rock bottom"),
                PlotStage::new("12. Dark Night of the Soul", "Hero finds the strength to continue"),
                PlotStage::new("13. Break into Three", "Hero implements the solution"),
                PlotStage::new("14. Finale", "Hero wins the day with a new sense of life"),
                PlotStage::new("15. Final Image", "Shows how the hero has changed"),
            ],
            PlotType::FreytagsPyramid => vec![
                PlotStage::new("1. Exposition", "Introduction of characters and setting"),
                PlotStage::new("2. Rising Action", "Series of events building tension"),
                PlotStage::new("3. Climax", "Turning point of the story"),
                PlotStage::new("4. Falling Action", "Events following the climax"),
                PlotStage::new("5. Catastrophe/Denouement", "Resolution and conclusion"),
            ],
            PlotType::DanHarmonsCircle => vec![
                PlotStage::new("1. You", "A character is in a zone of comfort"),
                PlotStage::new("2. Need", "But they want something"),
                PlotStage::new("3. Go", "They enter an unfamiliar situation"),
                PlotStage::new("4. Search", "Adapt to it"),
                PlotStage::new("5. Find", "Get what they wanted"),
                PlotStage::new("6. Take", "Pay a heavy price for it"),
                PlotStage::new("7. Return", "Then return to their familiar situation"),
                PlotStage::new("8. Changed", "Having changed forever"),
            ],
            PlotType::BlakeSnyderBeatSheet => vec![
                PlotStage::new("1. Opening Image", "A visual that represents the hero's flawed life"),
                PlotStage::new("2. Theme Stated", "Someone says what the theme is"),
                PlotStage::new("3. Set-Up", "Shows us the hero's life and what's missing"),
                PlotStage::new("4. Catalyst", "Something happens that changes everything"),
                PlotStage::new("5. Debate", "Hero debates whether to act"),
                PlotStage::new("6. Break into Two", "Hero chooses to act and enters new world"),
                PlotStage::new("7. B Story", "Introduces the theme and love story"),
                PlotStage::new("8. Fun and Games", "The promise of the premise is delivered"),
                PlotStage::new("9. Midpoint", "The stakes are raised, hero has false victory/defeat"),
                PlotStage::new("10. Bad Guys Close In", "Everything starts to go wrong"),
                PlotStage::new("11. All Is Lost", "Hero's deepest moment of despair"),
                PlotStage::new("12. Dark Night of the Soul", "Hero reflects on what went wrong"),
                PlotStage::new("13. Break into Three", "Hero implements the solution"),
                PlotStage::new("14. Finale", "Hero wins the day with a new sense of life"),
                PlotStage::new("15. Final Image", "Visual that shows hero's transformation"),
            ],
        }
    }
}

/// A stage/beat in a plot structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotStage {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub content: String,
    pub page_target: Option<u32>,
    pub completed: bool,
}

impl PlotStage {
    /// Create a new plot stage
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            content: String::new(),
            page_target: None,
            completed: false,
        }
    }

    /// Create a copy of this stage
    pub fn clone_stage(&self) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: self.title.clone(),
            description: self.description.clone(),
            content: self.content.clone(),
            page_target: self.page_target,
            completed: false,
        }
    }
}

/// Structure tool state and data
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StructureData {
    pub id: Uuid,
    pub project_id: Uuid,
    pub plot_type: PlotType,
    pub title: String,
    pub description: String,
    pub stages: Vec<PlotStage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl StructureData {
    /// Create a new structure data with a specific plot type
    pub fn new(project_id: Uuid, plot_type: PlotType, title: String, description: String) -> Self {
        let stages = plot_type.get_plot_stages();
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            plot_type,
            title,
            description,
            stages,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the number of completed stages
    pub fn completed_stages_count(&self) -> usize {
        self.stages.iter().filter(|stage| stage.completed).count()
    }

    /// Get the total number of stages
    pub fn total_stages_count(&self) -> usize {
        self.stages.len()
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f32 {
        let completed = self.completed_stages_count();
        let total = self.total_stages_count();
        if total == 0 { 0.0 } else { (completed as f32 / total as f32) * 100.0 }
    }

    /// Toggle completion of a stage by index
    pub fn toggle_stage_completion(&mut self, stage_index: usize) -> Result<(), String> {
        if let Some(stage) = self.stages.get_mut(stage_index) {
            stage.completed = !stage.completed;
            stage.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Stage not found".to_string())
        }
    }

    /// Update stage content
    pub fn update_stage_content(&mut self, stage_index: usize, content: String) -> Result<(), String> {
        if let Some(stage) = self.stages.get_mut(stage_index) {
            stage.content = content;
            stage.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Stage not found".to_string())
        }
    }

    /// Set stage page target
    pub fn set_stage_page_target(&mut self, stage_index: usize, page_target: Option<u32>) -> Result<(), String> {
        if let Some(stage) = self.stages.get_mut(stage_index) {
            stage.page_target = page_target;
            stage.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Stage not found".to_string())
        }
    }

    /// Get filtered stages based on completion status
    pub fn get_filtered_stages(&self, hide_completed: bool) -> Vec<&PlotStage> {
        if hide_completed {
            self.stages.iter().filter(|stage| !stage.completed).collect()
        } else {
            self.stages.iter().collect()
        }
    }

    /// Search stages by title or description
    pub fn search_stages(&self, query: &str) -> Vec<&PlotStage> {
        let query_lower = query.to_lowercase();
        self.stages.iter()
            .filter(|stage| {
                stage.title.to_lowercase().contains(&query_lower) ||
                stage.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

/// Structure manager for handling multiple structures
#[derive(Debug, Default)]
pub struct StructureManager {
    pub structures: HashMap<Uuid, StructureData>,
    pub current_structure_id: Option<Uuid>,
}

impl StructureManager {
    /// Create a new structure manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new structure
    pub fn create_structure(&mut self, project_id: Uuid, plot_type: PlotType, title: String, description: String) -> Uuid {
        let structure = StructureData::new(project_id, plot_type, title, description);
        let id = structure.id;
        self.structures.insert(id, structure);
        self.current_structure_id = Some(id);
        id
    }

    /// Get a structure by ID
    pub fn get_structure(&self, id: Uuid) -> Option<&StructureData> {
        self.structures.get(&id)
    }

    /// Get mutable reference to a structure
    pub fn get_structure_mut(&mut self, id: Uuid) -> Option<&mut StructureData> {
        self.structures.get_mut(&id)
    }

    /// Get the current active structure
    pub fn get_current_structure(&self) -> Option<&StructureData> {
        if let Some(id) = self.current_structure_id {
            self.structures.get(&id)
        } else {
            None
        }
    }

    /// Get mutable reference to current structure
    pub fn get_current_structure_mut(&mut self) -> Option<&mut StructureData> {
        if let Some(id) = self.current_structure_id {
            self.structures.get_mut(&id)
        } else {
            None
        }
    }

    /// Set the current active structure
    pub fn set_current_structure(&mut self, id: Option<Uuid>) {
        self.current_structure_id = id;
    }

    /// Get all structures for a project
    pub fn get_project_structures(&self, project_id: Uuid) -> Vec<&StructureData> {
        self.structures
            .values()
            .filter(|structure| structure.project_id == project_id)
            .collect()
    }

    /// Delete a structure
    pub fn delete_structure(&mut self, id: Uuid) -> Result<(), String> {
        if self.structures.remove(&id).is_some() {
            if self.current_structure_id == Some(id) {
                // Set first available structure as current, or None if none exist
                let remaining_ids: Vec<Uuid> = self.structures.keys().copied().collect();
                self.current_structure_id = remaining_ids.first().copied();
            }
            Ok(())
        } else {
            Err("Structure not found".to_string())
        }
    }

    /// Update structure metadata
    pub fn update_structure_metadata(&mut self, id: Uuid, title: String, description: String) -> Result<(), String> {
        if let Some(structure) = self.structures.get_mut(&id) {
            structure.title = title;
            structure.description = description;
            structure.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Structure not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_type_display_names() {
        assert_eq!(PlotType::ThreePart.display_name(), "Three-Part Structure");
        assert_eq!(PlotType::HeroesJourney.display_name(), "Hero's Journey");
        assert_eq!(PlotType::SaveTheCat.display_name(), "Save the Cat");
    }

    #[test]
    fn test_plot_stages_count() {
        assert_eq!(PlotType::ThreePart.get_plot_stages().len(), 3);
        assert_eq!(PlotType::HeroesJourney.get_plot_stages().len(), 12);
        assert_eq!(PlotType::SaveTheCat.get_plot_stages().len(), 15);
        assert_eq!(PlotType::FreytagsPyramid.get_plot_stages().len(), 5);
        assert_eq!(PlotType::DanHarmonsCircle.get_plot_stages().len(), 8);
        assert_eq!(PlotType::BlakeSnyderBeatSheet.get_plot_stages().len(), 15);
    }

    #[test]
    fn test_structure_creation() {
        let project_id = Uuid::new_v4();
        let structure = StructureData::new(
            project_id,
            PlotType::ThreePart,
            "Test Structure".to_string(),
            "Test Description".to_string(),
        );

        assert_eq!(structure.plot_type, PlotType::ThreePart);
        assert_eq!(structure.title, "Test Structure");
        assert_eq!(structure.description, "Test Description");
        assert_eq!(structure.stages.len(), 3);
        assert_eq!(structure.project_id, project_id);
        assert_eq!(structure.completed_stages_count(), 0);
        assert_eq!(structure.completion_percentage(), 0.0);
    }

    #[test]
    fn test_structure_completion() {
        let project_id = Uuid::new_v4();
        let mut structure = StructureData::new(
            project_id,
            PlotType::ThreePart,
            "Test Structure".to_string(),
            "Test Description".to_string(),
        );

        // Complete first stage
        structure.toggle_stage_completion(0).unwrap();
        assert_eq!(structure.completed_stages_count(), 1);
        assert_eq!(structure.completion_percentage(), 33.333332);

        // Complete second stage
        structure.toggle_stage_completion(1).unwrap();
        assert_eq!(structure.completed_stages_count(), 2);
        assert_eq!(structure.completion_percentage(), 66.666664);

        // Complete all stages
        structure.toggle_stage_completion(2).unwrap();
        assert_eq!(structure.completed_stages_count(), 3);
        assert_eq!(structure.completion_percentage(), 100.0);
    }

    #[test]
    fn test_structure_manager() {
        let mut manager = StructureManager::new();
        let project_id = Uuid::new_v4();

        // Create structure
        let structure_id = manager.create_structure(
            project_id,
            PlotType::ThreePart,
            "Test Structure".to_string(),
            "Test Description".to_string(),
        );

        // Get structure
        let structure = manager.get_structure(structure_id).unwrap();
        assert_eq!(structure.title, "Test Structure");

        // Get current structure
        let current = manager.get_current_structure().unwrap();
        assert_eq!(current.id, structure_id);

        // Test project structures
        let project_structures = manager.get_project_structures(project_id);
        assert_eq!(project_structures.len(), 1);
        assert_eq!(project_structures[0].id, structure_id);
    }
}
