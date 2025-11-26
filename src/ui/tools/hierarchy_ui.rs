//! Hierarchy UI Module for Slint
//!
//! Dynamic UI implementation for the hierarchy tool using Slint.
//! Provides tree view components and interactive hierarchy management.
//!
//! This module provides the interface between the Slint hierarchy tool
//! components and the underlying hierarchy logic.

use slint::{ComponentHandle, Model, ModelRc, VecModel, SharedString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::hierarchy_base::{HierarchyItem, HierarchyLevel, HierarchyTree, HierarchyNode};
use super::hierarchy_drag::{HierarchyDragHandler, HierarchyDragData, DragVisualFeedback};
use crate::ui_state::AppState;

/// Hierarchy UI state for managing UI-specific state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HierarchyUiState {
    pub selected_item: Option<String>,
    pub expanded_items: HashMap<String, bool>,
    pub show_context_menu: Option<String>,
    pub drag_source_item: Option<String>,
    pub drag_over_item: Option<String>,
    pub filter_text: String,
    pub show_only_chapters: bool,
    pub show_only_scenes: bool,
}

impl HierarchyUiState {
    /// Create a new hierarchy UI state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Toggle expansion state for an item
    pub fn toggle_expanded(&mut self, item_id: &str) {
        let current = self.expanded_items.get(item_id).copied().unwrap_or(false);
        self.expanded_items.insert(item_id.to_string(), !current);
    }
    
    /// Check if an item is expanded
    pub fn is_expanded(&self, item_id: &str) -> bool {
        self.expanded_items.get(item_id).copied().unwrap_or(false)
    }
    
    /// Select an item
    pub fn select_item(&mut self, item_id: Option<String>) {
        self.selected_item = item_id;
    }
    
    /// Check if an item is selected
    pub fn is_selected(&self, item_id: &str) -> bool {
        self.selected_item.as_deref() == Some(item_id)
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_item = None;
    }
    
    /// Set filter text
    pub fn set_filter(&mut self, filter: String) {
        self.filter_text = filter;
    }
    
    /// Clear filter
    pub fn clear_filter(&mut self) {
        self.filter_text.clear();
    }
}

mod shared_string_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use slint::SharedString;

    pub fn serialize<S>(value: &SharedString, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(value.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SharedString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SharedString::from(s))
    }
}

/// Hierarchy item data structure for Slint UI
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SlintHierarchyItem {
    #[serde(with = "shared_string_serde")]
    pub id: SharedString,
    #[serde(with = "shared_string_serde")]
    pub title: SharedString,
    pub level: u32, // Convert from HierarchyLevel for Slint compatibility
    #[serde(with = "shared_string_serde")]
    pub level_name: SharedString,
    pub has_children: bool,
    pub is_expanded: bool,
    pub depth: u32,
    pub word_count: u32,
    pub position: u32,
}

impl From<&HierarchyItem> for SlintHierarchyItem {
    fn from(item: &HierarchyItem) -> Self {
        let level_name = match item.level {
            HierarchyLevel::Unassigned => "Unassigned",
            HierarchyLevel::Manuscript => "Manuscript",
            HierarchyLevel::Chapter => "Chapter",
            HierarchyLevel::Scene => "Scene",
        };
        
        Self {
            id: SharedString::from(item.id.clone()),
            title: SharedString::from(item.title.clone()),
            level: item.level as u32,
            level_name: SharedString::from(level_name),
            has_children: !item.children.is_empty(),
            is_expanded: false, // This would be managed by UI state
            depth: item.level.depth(),
            word_count: 0,
            position: item.position,
        }
    }
}

/// Slint-compatible hierarchy tree model
pub struct SlintHierarchyModel {
    items: Vec<SlintHierarchyItem>,
    tree: HierarchyTree,
    expanded_items: HashMap<String, bool>,
}

impl SlintHierarchyModel {
    /// Create a new Slint hierarchy model
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            tree: HierarchyTree::new(),
            expanded_items: HashMap::new(),
        }
    }
    
    /// Update the model from hierarchy tree
    pub fn update_from_tree(&mut self, tree: &HierarchyTree) {
        self.tree = tree.clone();
        self.items = self.build_flat_list(tree);
    }
    
    /// Build a flat list for Slint ListView
    fn build_flat_list(&self, tree: &HierarchyTree) -> Vec<SlintHierarchyItem> {
        let mut items = Vec::new();
        let root_items = tree.get_root_items();
        
        for item in root_items {
            self.add_item_recursive(item, 0, &mut items);
        }
        
        items
    }
    
    /// Add item and its children recursively
    fn add_item_recursive(&self, item: &HierarchyItem, depth: usize, items: &mut Vec<SlintHierarchyItem>) {
        let mut slint_item: SlintHierarchyItem = item.into();
        slint_item.depth = depth as u32;
        
        // Check if item should be expanded
        slint_item.is_expanded = self.expanded_items.get(&item.id).copied().unwrap_or(false);
        
        items.push(slint_item.clone());
        
        // Add children if expanded
        if slint_item.is_expanded {
            let children = self.tree.get_children(&item.id);
            for child in children {
                self.add_item_recursive(child, depth + 1, items);
            }
        }
    }
    
    /// Toggle expansion for an item
    pub fn toggle_expansion(&mut self, item_id: &str) {
        let current = self.expanded_items.get(item_id).copied().unwrap_or(false);
        self.expanded_items.insert(item_id.to_string(), !current);
        self.update_from_tree(&self.tree.clone());
    }
    
    /// Get the items as a model for Slint
    pub fn as_model(&self) -> ModelRc<SlintHierarchyItem> {
        let model = VecModel::from(self.items.clone());
        ModelRc::new(model)
    }
    
    /// Get the underlying hierarchy tree
    pub fn get_tree(&self) -> &HierarchyTree {
        &self.tree
    }
    
    /// Get mutable access to the hierarchy tree
    pub fn get_tree_mut(&mut self) -> &mut HierarchyTree {
        &mut self.tree
    }
}

/// Hierarchy tool UI component for Slint integration
pub struct HierarchyTool {
    ui_state: HierarchyUiState,
    hierarchy_tree: HierarchyTree,
    drag_handler: HierarchyDragHandler,
    slint_model: SlintHierarchyModel,
    /// Callback for when hierarchy changes
    on_hierarchy_changed: Option<Box<dyn Fn(&HierarchyTree)>>,
    /// Callback for creating new items
    on_create_item: Option<Box<dyn Fn(HierarchyLevel, Option<String>, String) -> Result<String, String>>>,
}

impl HierarchyTool {
    /// Create a new hierarchy tool
    pub fn new() -> Self {
        Self {
            ui_state: HierarchyUiState::new(),
            hierarchy_tree: HierarchyTree::new(),
            drag_handler: HierarchyDragHandler::new(),
            slint_model: SlintHierarchyModel::new(),
            on_hierarchy_changed: None,
            on_create_item: None,
        }
    }
    
    /// Set callback for hierarchy changes
    pub fn set_hierarchy_changed_callback<F>(&mut self, callback: F)
    where
        F: Fn(&HierarchyTree) + 'static,
    {
        self.on_hierarchy_changed = Some(Box::new(callback));
    }
    
    /// Set callback for creating new items
    pub fn set_create_item_callback<F>(&mut self, callback: F)
    where
        F: Fn(HierarchyLevel, Option<String>, String) -> Result<String, String> + 'static,
    {
        self.on_create_item = Some(Box::new(callback));
    }
    
    /// Load hierarchy data
    pub fn load_hierarchy(&mut self, tree: HierarchyTree) {
        self.hierarchy_tree = tree;
        self.slint_model.update_from_tree(&self.hierarchy_tree);
    }
    
    /// Get the current hierarchy tree
    pub fn get_hierarchy(&self) -> &HierarchyTree {
        &self.hierarchy_tree
    }
    
    /// Get the Slint model
    pub fn get_slint_model(&self) -> &SlintHierarchyModel {
        &self.slint_model
    }
    
    /// Toggle expansion for an item
    pub fn toggle_expansion(&mut self, item_id: &str) {
        self.slint_model.toggle_expansion(item_id);
        
        // Notify listeners of hierarchy change
        if let Some(callback) = &self.on_hierarchy_changed {
            callback(&self.hierarchy_tree);
        }
    }
    
    /// Select an item
    pub fn select_item(&mut self, item_id: &str) {
        self.ui_state.select_item(Some(item_id.to_string()));
    }
    
    /// Create a new hierarchy item
    pub fn create_new_item(
        &mut self,
        title: String,
        level: HierarchyLevel,
        parent_id: Option<String>,
    ) -> Result<String, String> {
        let project_id = "project_1".to_string(); // This should come from app state
        let id = format!("ui_hierarchy_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis());
            
        let item = HierarchyItem::new(id.clone(), title, level, parent_id, project_id);
        self.hierarchy_tree.add_item(item)?;
        
        // Update Slint model
        self.slint_model.update_from_tree(&self.hierarchy_tree);
        
        // Notify listeners of hierarchy change
        if let Some(callback) = &self.on_hierarchy_changed {
            callback(&self.hierarchy_tree);
        }
        
        Ok(id)
    }
    
    /// Delete selected item
    pub fn delete_selected_item(&mut self) -> Result<(), String> {
        if let Some(selected_id) = &self.ui_state.selected_item {
            self.hierarchy_tree.remove_item(selected_id)?;
            
            // Update Slint model
            self.slint_model.update_from_tree(&self.hierarchy_tree);
            
            // Clear selection
            self.ui_state.clear_selection();
            
            // Notify listeners of hierarchy change
            if let Some(callback) = &self.on_hierarchy_changed {
                callback(&self.hierarchy_tree);
            }
            
            Ok(())
        } else {
            Err("No item selected".to_string())
        }
    }
    
    /// Apply filter to hierarchy
    pub fn apply_filter(&mut self, filter_text: String) {
        self.ui_state.set_filter(filter_text);
        // Filter would be applied in the Slint component
    }
    
    /// Export hierarchy data
    pub fn export_hierarchy(&self) -> String {
        format!("Exporting hierarchy with {} items", self.hierarchy_tree.len())
    }
}