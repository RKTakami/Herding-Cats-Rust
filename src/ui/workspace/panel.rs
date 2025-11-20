//! Workspace Panel Module
//! 
//! Handles individual panel management and UI components.

use serde::{Deserialize, Serialize};
use crate::ui::workspace::{PanelType, PanelConfig, PanelContentProvider};

/// Panel state for UI management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    pub config: PanelConfig,
    pub is_expanded: bool,
    pub is_minimized: bool,
    pub z_index: u32,
    pub last_focus_time: u64,
}

impl PanelState {
    /// Create a new panel state from configuration
    pub fn new(config: PanelConfig) -> Self {
        Self {
            config,
            is_expanded: true,
            is_minimized: false,
            z_index: 1,
            last_focus_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Focus the panel (update z-index and focus time)
    pub fn focus(&mut self) {
        self.is_minimized = false;
        self.last_focus_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Minimize the panel
    pub fn minimize(&mut self) {
        self.is_minimized = true;
    }

    /// Restore the panel from minimized state
    pub fn restore(&mut self) {
        self.is_minimized = false;
        self.is_expanded = true;
    }

    /// Toggle panel expansion
    pub fn toggle_expansion(&mut self) {
        self.is_expanded = !self.is_expanded;
    }
}

/// Panel container for managing multiple panels
pub struct PanelContainer {
    panels: std::collections::HashMap<String, PanelState>,
    focused_panel: Option<String>,
}

impl Default for PanelContainer {
    fn default() -> Self {
        Self {
            panels: std::collections::HashMap::new(),
            focused_panel: None,
        }
    }
}

impl PanelContainer {
    /// Create a new panel container
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a panel to the container
    pub fn add_panel(&mut self, config: PanelConfig) -> String {
        let state = PanelState::new(config.clone());
        let panel_id = config.id.clone();
        self.panels.insert(panel_id.clone(), state);
        self.focus_panel(&panel_id);
        panel_id
    }

    /// Remove a panel from the container
    pub fn remove_panel(&mut self, panel_id: &str) -> bool {
        if self.panels.remove(panel_id).is_some() {
            if self.focused_panel.as_ref() == Some(panel_id) {
                // Find the next panel to focus
                if let Some(next_panel) = self.panels.keys().next() {
                    self.focused_panel = Some(next_panel.clone());
                } else {
                    self.focused_panel = None;
                }
            }
            true
        } else {
            false
        }
    }

    /// Focus a specific panel
    pub fn focus_panel(&mut self, panel_id: &str) -> bool {
        if let Some(panel) = self.panels.get_mut(panel_id) {
            panel.focus();
            self.focused_panel = Some(panel_id.to_string());
            
            // Update z-indexes to bring focused panel to front
            let mut max_z = 1;
            for (_, p) in self.panels.iter() {
                max_z = max_z.max(p.z_index);
            }
            panel.z_index = max_z + 1;
            
            true
        } else {
            false
        }
    }

    /// Get the currently focused panel
    pub fn get_focused_panel(&self) -> Option<&PanelState> {
        if let Some(id) = &self.focused_panel {
            self.panels.get(id)
        } else {
            None
        }
    }

    /// Get a specific panel by ID
    pub fn get_panel(&self, panel_id: &str) -> Option<&PanelState> {
        self.panels.get(panel_id)
    }

    /// Get a mutable reference to a specific panel
    pub fn get_panel_mut(&mut self, panel_id: &str) -> Option<&mut PanelState> {
        self.panels.get_mut(panel_id)
    }

    /// Get all panels sorted by z-index
    pub fn get_sorted_panels(&self) -> Vec<(&String, &PanelState)> {
        let mut panels: Vec<_> = self.panels.iter().collect();
        panels.sort_by_key(|(_, panel)| panel.z_index);
        panels
    }

    /// Minimize all panels
    pub fn minimize_all(&mut self) {
        for panel in self.panels.values_mut() {
            panel.minimize();
        }
        self.focused_panel = None;
    }

    /// Restore all panels
    pub fn restore_all(&mut self) {
        for panel in self.panels.values_mut() {
            panel.restore();
        }
        if let Some(focused_id) = &self.focused_panel {
            if let Some(panel) = self.panels.get_mut(focused_id) {
                panel.focus();
            }
        }
    }

    /// Get panel statistics
    pub fn get_panel_stats(&self) -> PanelStats {
        let total_panels = self.panels.len();
        let focused_panels = if self.focused_panel.is_some() { 1 } else { 0 };
        let minimized_panels = self.panels.values().filter(|p| p.is_minimized).count();
        let expanded_panels = self.panels.values().filter(|p| p.is_expanded).count();

        PanelStats {
            total_panels,
            focused_panels,
            minimized_panels,
            expanded_panels,
        }
    }

    /// Update panel configuration
    pub fn update_panel_config(&mut self, panel_id: &str, new_config: PanelConfig) -> bool {
        if let Some(panel) = self.panels.get_mut(panel_id) {
            panel.config = new_config;
            true
        } else {
            false
        }
    }

    /// Get panels by type
    pub fn get_panels_by_type(&self, panel_type: PanelType) -> Vec<&PanelState> {
        self.panels.values()
            .filter(|panel| panel.config.panel_type == panel_type)
            .collect()
    }

    /// Get all panel IDs
    pub fn get_panel_ids(&self) -> Vec<String> {
        self.panels.keys().cloned().collect()
    }
}

/// Panel statistics
#[derive(Debug)]
pub struct PanelStats {
    pub total_panels: usize,
    pub focused_panels: usize,
    pub minimized_panels: usize,
    pub expanded_panels: usize,
}

/// Panel layout manager
pub struct PanelLayoutManager {
    layout_constraints: LayoutConstraints,
}

#[derive(Debug, Clone)]
pub struct LayoutConstraints {
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub default_width: u32,
    pub default_height: u32,
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_width: 200,
            min_height: 150,
            max_width: Some(1920),
            max_height: Some(1080),
            default_width: 400,
            default_height: 600,
        }
    }
}

impl PanelLayoutManager {
    /// Create a new panel layout manager
    pub fn new() -> Self {
        Self {
            layout_constraints: LayoutConstraints::default(),
        }
    }

    /// Set layout constraints
    pub fn set_constraints(&mut self, constraints: LayoutConstraints) {
        self.layout_constraints = constraints;
    }

    /// Validate panel size
    pub fn validate_panel_size(&self, width: u32, height: u32) -> (u32, u32) {
        let width = width.max(self.layout_constraints.min_width);
        let height = height.max(self.layout_constraints.min_height);

        let width = if let Some(max_width) = self.layout_constraints.max_width {
            width.min(max_width)
        } else {
            width
        };

        let height = if let Some(max_height) = self.layout_constraints.max_height {
            height.min(max_height)
        } else {
            height
        };

        (width, height)
    }

    /// Calculate optimal panel position
    pub fn calculate_panel_position(
        &self,
        panel_count: usize,
        container_width: u32,
        container_height: u32,
    ) -> (i32, i32) {
        // Simple grid layout calculation
        let columns = (panel_count as f64).sqrt().ceil() as u32;
        let row = (panel_count / columns as usize) as u32;
        
        let panel_width = self.layout_constraints.default_width;
        let panel_height = self.layout_constraints.default_height;
        
        let x = ((panel_count % columns as usize) * panel_width as usize) as i32;
        let y = (row * panel_height) as i32;
        
        (x as i32, y as i32)
    }
}

/// Panel factory for creating panels
pub struct PanelFactory;

impl PanelFactory {
    /// Create a panel configuration with default settings
    pub fn create_panel(panel_type: PanelType) -> PanelConfig {
        PanelConfig::new(panel_type)
    }

    /// Create a floating panel
    pub fn create_floating_panel(panel_type: PanelType) -> PanelConfig {
        let mut config = Self::create_panel(panel_type);
        config.is_floating = true;
        config
    }

    /// Create a docked panel
    pub fn create_docked_panel(panel_type: PanelType, position: i32, size: (u32, u32)) -> PanelConfig {
        let mut config = Self::create_panel(panel_type);
        config.position = (position, position);
        config.size = size;
        config.is_floating = false;
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::workspace::PanelType;

    #[test]
    fn test_panel_container() {
        let mut container = PanelContainer::new();
        
        let config = PanelConfig::new(PanelType::DocumentEditor);
        let panel_id = container.add_panel(config);
        
        assert!(container.get_panel(&panel_id).is_some());
        assert_eq!(container.get_focused_panel().unwrap().config.panel_type, PanelType::DocumentEditor);
    }

    #[test]
    fn test_panel_focus() {
        let mut container = PanelContainer::new();
        
        let config1 = PanelConfig::new(PanelType::DocumentEditor);
        let config2 = PanelConfig::new(PanelType::Outline);
        
        let id1 = container.add_panel(config1);
        let id2 = container.add_panel(config2);
        
        // Focus should be on the second panel
        assert_eq!(container.focused_panel.as_ref(), Some(&id2));
        
        // Focus on first panel
        container.focus_panel(&id1);
        assert_eq!(container.focused_panel.as_ref(), Some(&id1));
    }

    #[test]
    fn test_panel_layout_manager() {
        let layout_manager = PanelLayoutManager::new();
        
        let (width, height) = layout_manager.validate_panel_size(100, 100);
        assert_eq!(width, 200); // Should be clamped to minimum
        assert_eq!(height, 150);
        
        let (width, height) = layout_manager.validate_panel_size(300, 200);
        assert_eq!(width, 300); // Should be within constraints
        assert_eq!(height, 200);
    }
}