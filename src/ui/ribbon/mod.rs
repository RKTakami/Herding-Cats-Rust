//! Ribbon Interface Module
//!
//! Handles the ribbon-style UI interface with tabs, groups, and commands.

pub mod group;
pub mod dropdown;
pub mod tab;
pub mod insert_tab;
pub mod layout_tab;
pub mod references_tab;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ribbon tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonTab {
    pub id: String,
    pub title: String,
    pub groups: Vec<RibbonGroup>,
    pub is_active: bool,
}

impl RibbonTab {
    /// Create a new ribbon tab
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            groups: Vec::new(),
            is_active: false,
        }
    }

    /// Add a group to this tab
    pub fn add_group(&mut self, group: RibbonGroup) {
        self.groups.push(group);
    }

    /// Set this tab as active
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

/// Ribbon group (container for related buttons/controls)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonGroup {
    pub id: String,
    pub title: String,
    pub items: Vec<RibbonItem>,
    pub is_collapsed: bool,
}

impl RibbonGroup {
    /// Create a new ribbon group
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            items: Vec::new(),
            is_collapsed: false,
        }
    }

    /// Add an item to this group
    pub fn add_item(&mut self, item: RibbonItem) {
        self.items.push(item);
    }

    /// Toggle group collapse state
    pub fn toggle_collapse(&mut self) {
        self.is_collapsed = !self.is_collapsed;
    }
}

/// Ribbon item (button, dropdown, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RibbonItem {
    Button(RibbonButton),
    Dropdown(RibbonDropdown),
    SplitButton(RibbonSplitButton),
    ToggleButton(RibbonToggleButton),
    ComboBox(RibbonComboBox),
}

/// Ribbon button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonButton {
    pub id: String,
    pub text: String,
    pub icon: Option<String>,
    pub tooltip: Option<String>,
    pub enabled: bool,
    pub visible: bool,
}

impl RibbonButton {
    /// Create a new ribbon button
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            icon: None,
            tooltip: None,
            enabled: true,
            visible: true,
        }
    }
}

/// Ribbon dropdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonDropdown {
    pub id: String,
    pub text: String,
    pub items: Vec<RibbonDropdownItem>,
    pub enabled: bool,
    pub visible: bool,
}

impl RibbonDropdown {
    /// Create a new ribbon dropdown
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            items: Vec::new(),
            enabled: true,
            visible: true,
        }
    }

    /// Add an item to the dropdown
    pub fn add_item(&mut self, item: RibbonDropdownItem) {
        self.items.push(item);
    }
}

/// Ribbon dropdown item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonDropdownItem {
    pub id: String,
    pub text: String,
    pub icon: Option<String>,
    pub enabled: bool,
}

impl RibbonDropdownItem {
    /// Create a new dropdown item
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            icon: None,
            enabled: true,
        }
    }
}

/// Ribbon split button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonSplitButton {
    pub main_button: RibbonButton,
    pub dropdown: RibbonDropdown,
}

/// Ribbon toggle button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonToggleButton {
    pub button: RibbonButton,
    pub is_toggled: bool,
}

impl RibbonToggleButton {
    /// Create a new toggle button
    pub fn new(id: String, text: String) -> Self {
        Self {
            button: RibbonButton::new(id, text),
            is_toggled: false,
        }
    }

    /// Toggle the button state
    pub fn toggle(&mut self) {
        self.is_toggled = !self.is_toggled;
    }
}

/// Ribbon combo box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonComboBox {
    pub id: String,
    pub items: Vec<String>,
    pub selected_index: Option<usize>,
    pub enabled: bool,
    pub visible: bool,
}

impl RibbonComboBox {
    /// Create a new combo box
    pub fn new(id: String) -> Self {
        Self {
            id,
            items: Vec::new(),
            selected_index: None,
            enabled: true,
            visible: true,
        }
    }

    /// Add an item to the combo box
    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
        if self.selected_index.is_none() {
            self.selected_index = Some(0);
        }
    }

    /// Set the selected index
    pub fn set_selected_index(&mut self, index: usize) -> bool {
        if index < self.items.len() {
            self.selected_index = Some(index);
            true
        } else {
            false
        }
    }

    /// Get the currently selected item
    pub fn get_selected_item(&self) -> Option<&String> {
        if let Some(index) = self.selected_index {
            self.items.get(index)
        } else {
            None
        }
    }
}

/// Ribbon manager for handling ribbon interface
pub struct RibbonManager {
    pub tabs: Vec<RibbonTab>,
    pub active_tab_index: Option<usize>,
    pub command_handlers: HashMap<String, Box<dyn Fn()>>,
}

impl Default for RibbonManager {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_index: None,
            command_handlers: HashMap::new(),
        }
    }
}

impl RibbonManager {
    /// Create a new ribbon manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a tab to the ribbon
    pub fn add_tab(&mut self, tab: RibbonTab) {
        self.tabs.push(tab);
        if self.active_tab_index.is_none() && !self.tabs.is_empty() {
            self.active_tab_index = Some(0);
            self.tabs[0].set_active(true);
        }
    }

    /// Set the active tab by index
    pub fn set_active_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            // Deactivate current tab
            if let Some(current_index) = self.active_tab_index {
                if current_index < self.tabs.len() {
                    self.tabs[current_index].set_active(false);
                }
            }
            
            // Activate new tab
            self.tabs[index].set_active(true);
            self.active_tab_index = Some(index);
            true
        } else {
            false
        }
    }

    /// Set the active tab by ID
    pub fn set_active_tab_by_id(&mut self, tab_id: &str) -> bool {
        if let Some(index) = self.tabs.iter().position(|tab| tab.id == tab_id) {
            self.set_active_tab(index);
            true
        } else {
            false
        }
    }

    /// Get the currently active tab
    pub fn get_active_tab(&self) -> Option<&RibbonTab> {
        if let Some(index) = self.active_tab_index {
            self.tabs.get(index)
        } else {
            None
        }
    }

    /// Get the active tab index
    pub fn get_active_tab_index(&self) -> Option<usize> {
        self.active_tab_index
    }

    /// Execute a command by ID
    pub fn execute_command(&self, command_id: &str) -> bool {
        if let Some(handler) = self.command_handlers.get(command_id) {
            handler();
            true
        } else {
            false
        }
    }

    /// Register a command handler
    pub fn register_command<F>(&mut self, command_id: String, handler: F)
    where
        F: Fn() + 'static,
    {
        self.command_handlers.insert(command_id, Box::new(handler));
    }

    /// Get ribbon statistics
    pub fn get_ribbon_stats(&self) -> RibbonStats {
        let total_tabs = self.tabs.len();
        let total_groups = self.tabs.iter().map(|tab| tab.groups.len()).sum();
        let total_items = self.tabs.iter()
            .map(|tab| {
                tab.groups.iter()
                    .map(|group| group.items.len())
                    .sum::<usize>()
            })
            .sum();

        RibbonStats {
            total_tabs,
            total_groups,
            total_items,
            active_tab_index: self.active_tab_index,
        }
    }

    /// Create default ribbon tabs for a document editor
    pub fn create_default_ribbon() -> Self {
        let mut manager = Self::new();

        // Home tab
        let mut home_tab = RibbonTab::new("home".to_string(), "Home".to_string());
        
        // Clipboard group
        let mut clipboard_group = RibbonGroup::new("clipboard".to_string(), "Clipboard".to_string());
        clipboard_group.add_item(RibbonItem::SplitButton(RibbonSplitButton {
            main_button: RibbonButton::new("paste".to_string(), "Paste".to_string()),
            dropdown: RibbonDropdown::new("paste_dropdown".to_string(), "Paste Options".to_string()),
        }));
        clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("cut".to_string(), "Cut".to_string())));
        clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("copy".to_string(), "Copy".to_string())));
        home_tab.add_group(clipboard_group);

        // Font group
        let mut font_group = RibbonGroup::new("font".to_string(), "Font".to_string());
        font_group.add_item(RibbonItem::Button(RibbonButton::new("bold".to_string(), "Bold".to_string())));
        font_group.add_item(RibbonItem::Button(RibbonButton::new("italic".to_string(), "Italic".to_string())));
        font_group.add_item(RibbonItem::Button(RibbonButton::new("underline".to_string(), "Underline".to_string())));
        home_tab.add_group(font_group);

        manager.add_tab(home_tab);

        // Insert tab
        let mut insert_tab = RibbonTab::new("insert".to_string(), "Insert".to_string());
        
        let mut insert_group = RibbonGroup::new("insert".to_string(), "Insert".to_string());
        insert_group.add_item(RibbonItem::Button(RibbonButton::new("table".to_string(), "Table".to_string())));
        insert_group.add_item(RibbonItem::Button(RibbonButton::new("image".to_string(), "Image".to_string())));
        insert_tab.add_group(insert_group);

        manager.add_tab(insert_tab);

        manager
    }
}

/// Ribbon statistics
#[derive(Debug)]
pub struct RibbonStats {
    pub total_tabs: usize,
    pub total_groups: usize,
    pub total_items: usize,
    pub active_tab_index: Option<usize>,
}

/// Template registry for managing document templates
pub mod templates;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ribbon_manager_creation() {
        let mut manager = RibbonManager::new();
        
        let tab = RibbonTab::new("test".to_string(), "Test".to_string());
        manager.add_tab(tab);
        
        assert_eq!(manager.tabs.len(), 1);
        assert_eq!(manager.get_active_tab_index(), Some(0));
    }

    #[test]
    fn test_ribbon_tab_operations() {
        let mut tab = RibbonTab::new("test".to_string(), "Test".to_string());
        
        let group = RibbonGroup::new("group1".to_string(), "Group 1".to_string());
        tab.add_group(group);
        
        assert_eq!(tab.groups.len(), 1);
        assert_eq!(tab.groups[0].title, "Group 1");
    }

    #[test]
    fn test_ribbon_manager_default() {
        let manager = RibbonManager::create_default_ribbon();
        
        assert_eq!(manager.tabs.len(), 2); // Home and Insert tabs
        assert!(manager.get_active_tab().is_some());
        
        let stats = manager.get_ribbon_stats();
        assert!(stats.total_tabs > 0);
        assert!(stats.total_groups > 0);
    }
}