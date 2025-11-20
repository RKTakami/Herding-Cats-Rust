//! Workspace Management Module
//! 
//! Handles multi-panel workspace layout, panel management, and window state persistence.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of panels available in the workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelType {
    DocumentEditor,
    Outline,
    Notes,
    Research,
    Brainstorming,
    Hierarchy,
    Codex,
    Plot,
    Analysis,
}

impl PanelType {
    /// Get the display name for the panel type
    pub fn display_name(&self) -> &'static str {
        match self {
            PanelType::DocumentEditor => "Document Editor",
            PanelType::Outline => "Outline",
            PanelType::Notes => "Notes",
            PanelType::Research => "Research",
            PanelType::Brainstorming => "Brainstorming",
            PanelType::Hierarchy => "Hierarchy",
            PanelType::Codex => "Codex",
            PanelType::Plot => "Plot",
            PanelType::Analysis => "Analysis",
        }
    }

    /// Check if the panel can be docked
    pub fn can_dock(&self) -> bool {
        true // All panels can dock
    }
}

/// Position for docking panels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

/// Configuration for a single panel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    pub id: String,
    pub panel_type: PanelType,
    pub title: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub is_visible: bool,
    pub is_floating: bool,
}

impl PanelConfig {
    /// Create a new panel configuration
    pub fn new(panel_type: PanelType) -> Self {
        Self {
            id: format!("panel_{}_{}", panel_type.display_name().to_lowercase().replace(' ', "_"), 
                       rand::random::<u32>()),
            panel_type,
            title: panel_type.display_name().to_string(),
            size: (300, 400),
            position: (100, 100),
            is_visible: true,
            is_floating: false,
        }
    }
}

/// Default workspace layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefaultLayout {
    WriterLayout,
    ResearchLayout,
    BrainstormingLayout,
    AnalysisLayout,
}

impl DefaultLayout {
    /// Create the default panels for this layout
    pub fn create_default_panels(&self) -> Vec<PanelConfig> {
        match self {
            DefaultLayout::WriterLayout => vec![
                PanelConfig::new(PanelType::DocumentEditor),
                PanelConfig::new(PanelType::Outline),
                PanelConfig::new(PanelType::Notes),
            ],
            DefaultLayout::ResearchLayout => vec![
                PanelConfig::new(PanelType::DocumentEditor),
                PanelConfig::new(PanelType::Research),
                PanelConfig::new(PanelType::Analysis),
            ],
            DefaultLayout::BrainstormingLayout => vec![
                PanelConfig::new(PanelType::Brainstorming),
                PanelConfig::new(PanelType::Notes),
                PanelConfig::new(PanelType::Plot),
            ],
            DefaultLayout::AnalysisLayout => vec![
                PanelConfig::new(PanelType::DocumentEditor),
                PanelConfig::new(PanelType::Analysis),
                PanelConfig::new(PanelType::Hierarchy),
            ],
        }
    }
}

/// Current workspace state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub panels: HashMap<String, PanelConfig>,
    pub floating_panels: Vec<String>,
    pub active_panel_id: Option<String>,
    pub current_layout: DefaultLayout,
}

impl WorkspaceState {
    /// Create a new empty workspace state
    pub fn new() -> Self {
        Self::default()
    }
}

/// Workspace manager for handling panel operations
pub struct WorkspaceManager {
    pub default_layout: DefaultLayout,
    pub is_initialized: bool,
    panels: HashMap<String, PanelConfig>,
    state: WorkspaceState,
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self {
            default_layout: DefaultLayout::WriterLayout,
            is_initialized: false,
            panels: HashMap::new(),
            state: WorkspaceState::new(),
        }
    }
}

impl WorkspaceManager {
    /// Create a new workspace manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize the workspace with a default layout
    pub fn initialize_with_default(&mut self, layout: DefaultLayout) {
        self.default_layout = layout;
        self.state.current_layout = layout;
        
        let panels = layout.create_default_panels();
        for panel in panels {
            self.panels.insert(panel.id.clone(), panel.clone());
            self.state.panels.insert(panel.id.clone(), panel);
        }
        
        self.is_initialized = true;
    }

    /// Add a new panel to the workspace
    pub fn add_panel(&mut self, panel_type: PanelType, dock_position: Option<DockPosition>) -> Result<String, String> {
        let config = PanelConfig::new(panel_type);
        let panel_id = config.id.clone();
        
        self.panels.insert(panel_id.clone(), config.clone());
        self.state.panels.insert(panel_id.clone(), config);
        
        Ok(panel_id)
    }

    /// Remove a panel from the workspace
    pub fn remove_panel(&mut self, panel_id: &str) -> Result<(), String> {
        if self.panels.remove(panel_id).is_some() {
            self.state.panels.remove(panel_id);
            self.state.floating_panels.retain(|id| id != panel_id);
            if self.state.active_panel_id.as_ref() == Some(panel_id) {
                self.state.active_panel_id = None;
            }
            Ok(())
        } else {
            Err(format!("Panel {} not found", panel_id))
        }
    }

    /// Set the active panel
    pub fn set_active_panel(&mut self, panel_id: Option<String>) {
        self.state.active_panel_id = panel_id;
    }

    /// Get all available panel types
    pub fn get_available_panel_types() -> Vec<PanelType> {
        vec![
            PanelType::DocumentEditor,
            PanelType::Outline,
            PanelType::Notes,
            PanelType::Research,
            PanelType::Brainstorming,
            PanelType::Hierarchy,
            PanelType::Codex,
            PanelType::Plot,
            PanelType::Analysis,
        ]
    }

    /// Get all available default layouts
    pub fn get_available_default_layouts() -> Vec<(DefaultLayout, &'static str, &'static str)> {
        vec![
            (DefaultLayout::WriterLayout, "Writer Layout", "Optimized for document writing"),
            (DefaultLayout::ResearchLayout, "Research Layout", "Optimized for research and analysis"),
            (DefaultLayout::BrainstormingLayout, "Brainstorming Layout", "Optimized for idea generation"),
            (DefaultLayout::AnalysisLayout, "Analysis Layout", "Optimized for data analysis"),
        ]
    }

    /// Get workspace statistics
    pub fn get_statistics(&self) -> WorkspaceStatistics {
        let total_panels = self.panels.len();
        let docked_panels = self.panels.values().filter(|p| !p.is_floating).count();
        let floating_panels = self.panels.values().filter(|p| p.is_floating).count();

        WorkspaceStatistics {
            total_panels,
            docked_panels,
            floating_panels,
        }
    }

    /// Save the current workspace state
    pub fn save_state(&self) -> Result<String, String> {
        serde_json::to_string(&self.state)
            .map_err(|e| format!("Failed to serialize workspace state: {}", e))
    }

    /// Load a workspace state
    pub fn load_state(&mut self, state_json: &str) -> Result<(), String> {
        let state: WorkspaceState = serde_json::from_str(state_json)
            .map_err(|e| format!("Failed to deserialize workspace state: {}", e))?;
        
        self.state = state;
        self.panels.clear();
        for (id, config) in &self.state.panels {
            self.panels.insert(id.clone(), config.clone());
        }
        
        self.is_initialized = true;
        Ok(())
    }
}

/// Workspace statistics
#[derive(Debug)]
pub struct WorkspaceStatistics {
    pub total_panels: usize,
    pub docked_panels: usize,
    pub floating_panels: usize,
}

/// Content provider for workspace panels
pub trait PanelContentProvider {
    /// Get the content for the panel
    fn get_content(&self) -> String;
    
    /// Set the content for the panel
    fn set_content(&mut self, content: String);
    
    /// Check if the content has been modified
    fn is_dirty(&self) -> bool;
    
    /// Clear the dirty flag
    fn clear_dirty(&mut self);
}

/// Outline content provider
pub struct OutlineContent {
    pub items: Vec<String>,
    pub is_dirty: bool,
}

impl OutlineContent {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            is_dirty: false,
        }
    }
}

impl PanelContentProvider for OutlineContent {
    fn get_content(&self) -> String {
        self.items.join("\n")
    }

    fn set_content(&mut self, content: String) {
        self.items = content.lines().map(|s| s.to_string()).collect();
        self.is_dirty = true;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn clear_dirty(&mut self) {
        self.is_dirty = false;
    }
}

/// Notes content provider
pub struct NotesContent {
    pub notes: Vec<String>,
    pub is_dirty: bool,
}

impl NotesContent {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            is_dirty: false,
        }
    }
}

impl PanelContentProvider for NotesContent {
    fn get_content(&self) -> String {
        self.notes.join("\n")
    }

    fn set_content(&mut self, content: String) {
        self.notes = content.lines().map(|s| s.to_string()).collect();
        self.is_dirty = true;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn clear_dirty(&mut self) {
        self.is_dirty = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_manager_creation() {
        let mut manager = WorkspaceManager::new();
        assert!(!manager.is_initialized);
        
        manager.initialize_with_default(DefaultLayout::WriterLayout);
        assert!(manager.is_initialized);
        assert!(manager.get_statistics().total_panels > 0);
    }

    #[test]
    fn test_panel_operations() {
        let mut manager = WorkspaceManager::new();
        manager.initialize_with_default(DefaultLayout::WriterLayout);
        
        let panel_id = manager.add_panel(PanelType::Research, None).unwrap();
        assert!(manager.panels.contains_key(&panel_id));
        
        manager.remove_panel(&panel_id).unwrap();
        assert!(!manager.panels.contains_key(&panel_id));
    }

    #[test]
    fn test_workspace_state_serialization() {
        let mut manager = WorkspaceManager::new();
        manager.initialize_with_default(DefaultLayout::WriterLayout);
        
        let state_json = manager.save_state().unwrap();
        assert!(!state_json.is_empty());
        
        let mut new_manager = WorkspaceManager::new();
        assert!(new_manager.load_state(&state_json).is_ok());
        assert!(new_manager.is_initialized);
    }
}