//! Hierarchy Tool Base Module
//!
//! Core data structures and logic for the hierarchy tool implementation.
//! Provides the foundation for managing manuscript hierarchies with proper
//! level relationships and recursive structure.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export commonly used types
// pub use self::{HierarchyItem, HierarchyLevel, HierarchyTree, HierarchyNode};

/// Hierarchy levels as specified in the requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HierarchyLevel {
    Unassigned,
    Manuscript,
    Chapter,
    Scene,
}

impl HierarchyLevel {
    /// Get the display name for the hierarchy level
    pub fn display_name(&self) -> &'static str {
        match self {
            HierarchyLevel::Unassigned => "Unassigned",
            HierarchyLevel::Manuscript => "Manuscript",
            HierarchyLevel::Chapter => "Chapter",
            HierarchyLevel::Scene => "Scene",
        }
    }
    
    /// Get the level depth (used for indentation and ordering)
    pub fn depth(&self) -> u32 {
        match self {
            HierarchyLevel::Unassigned => 0,
            HierarchyLevel::Manuscript => 0, // Same level as Unassigned
            HierarchyLevel::Chapter => 1,
            HierarchyLevel::Scene => 2,
        }
    }
    
    /// Check if this level can be a parent of another level
    pub fn can_have_as_child(&self, child_level: HierarchyLevel) -> bool {
        match self {
            HierarchyLevel::Unassigned => {
                // Unassigned can have Chapter or Unassigned (notes) as children
                matches!(child_level, HierarchyLevel::Chapter | HierarchyLevel::Unassigned)
            }
            HierarchyLevel::Manuscript => {
                matches!(child_level, HierarchyLevel::Chapter)
            }
            HierarchyLevel::Chapter => {
                matches!(child_level, HierarchyLevel::Scene)
            }
            HierarchyLevel::Scene => false, // Scenes cannot have children
        }
    }
    
    /// Get all valid parent levels for this level
    pub fn valid_parent_levels(&self) -> Vec<HierarchyLevel> {
        match self {
            HierarchyLevel::Unassigned => vec![], // Unassigned is top level only
            HierarchyLevel::Manuscript => vec![], // Manuscript is top level only
            HierarchyLevel::Chapter => vec![HierarchyLevel::Manuscript], // Chapter can only be under Manuscript
            HierarchyLevel::Scene => vec![HierarchyLevel::Chapter],
        }
    }
    
    /// Check if this is a valid top-level item
    pub fn is_top_level(&self) -> bool {
        matches!(self, HierarchyLevel::Unassigned | HierarchyLevel::Manuscript)
    }
}

/// Enhanced hierarchy item with recursive relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyItem {
    pub id: String,
    pub title: String,
    pub level: HierarchyLevel,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub position: u32, // For drag/drop ordering
    pub project_id: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

impl HierarchyItem {
    /// Create a new hierarchy item
    pub fn new(id: String, title: String, level: HierarchyLevel, parent_id: Option<String>, project_id: String) -> Self {
        Self {
            id,
            title,
            level,
            parent_id,
            children: Vec::new(),
            position: 0,
            project_id,
            created_at: None,
            updated_at: None,
            metadata: None,
        }
    }
    
    /// Validate the hierarchy item structure
    pub fn validate(&self) -> Result<(), String> {
        // Validate ID
        if self.id.is_empty() {
            return Err("Hierarchy item ID cannot be empty".to_string());
        }
        
        // Validate title
        if self.title.trim().is_empty() {
            return Err("Hierarchy item title cannot be empty".to_string());
        }
        
        // Validate project ID
        if self.project_id.is_empty() {
            return Err("Hierarchy item must belong to a project".to_string());
        }
        
        // Validate parent-child relationship
        if let Some(ref parent_id) = self.parent_id {
            if parent_id == &self.id {
                return Err("Hierarchy item cannot be its own parent".to_string());
            }
        }
        
        // Validate level constraints
        if !self.level.is_top_level() && self.parent_id.is_none() {
            return Err(format!("{} items must have a parent", self.level.display_name()));
        }
        
        Ok(())
    }
    
    /// Check if this item is a valid parent for another item
    pub fn can_have_as_child(&self, child_item: &HierarchyItem) -> bool {
        self.level.can_have_as_child(child_item.level)
    }
    
    /// Add a child to this item
    pub fn add_child(&mut self, child_id: String) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
            self.children.sort(); // Keep children sorted for consistent ordering
        }
    }
    
    /// Remove a child from this item
    pub fn remove_child(&mut self, child_id: &str) {
        if let Some(index) = self.children.iter().position(|id| id == child_id) {
            self.children.remove(index);
        }
    }
    
    /// Check if this item has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
    
    /// Get the display indentation level
    pub fn display_indent(&self) -> u32 {
        self.level.depth()
    }
    
    /// Create a root-level item (Unassigned or Manuscript)
    pub fn new_root(title: String, level: HierarchyLevel, project_id: String) -> Self {
        Self::new(title.clone(), title, level, None, project_id)
    }
    
    /// Create a chapter item
    pub fn new_chapter(title: String, parent_id: String, project_id: String) -> Self {
        Self::new(title.clone(), title, HierarchyLevel::Chapter, Some(parent_id), project_id)
    }
    
    /// Create a scene item
    pub fn new_scene(title: String, parent_id: String, project_id: String) -> Self {
        Self::new(title.clone(), title, HierarchyLevel::Scene, Some(parent_id), project_id)
    }
    
    /// Create a new item with validation
    pub fn create_item(
        id: Option<String>,
        title: String,
        level: HierarchyLevel,
        parent_id: Option<String>,
        project_id: String,
    ) -> Result<Self, String> {
        // Validate title
        if title.trim().is_empty() {
            return Err("Item title cannot be empty".to_string());
        }
        
        // Validate level constraints
        if !level.is_top_level() && parent_id.is_none() {
            return Err(format!("{} items must have a parent", level.display_name()));
        }
        
        // Generate ID if not provided
        let item_id = id.unwrap_or_else(|| format!("hierarchy_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()));
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();
        
        Ok(Self {
            id: item_id,
            title: title.trim().to_string(),
            level,
            parent_id,
            children: Vec::new(),
            position: 0,
            project_id,
            created_at: Some(timestamp.clone()),
            updated_at: Some(timestamp),
            metadata: None,
        })
    }
    
    /// Set metadata for the item
    pub fn set_metadata(&mut self, key: String, value: String) {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        
        if let Some(ref mut metadata) = self.metadata {
            metadata.insert(key, value);
        }
    }
    
    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.as_ref()?.get(key)
    }
    
    /// Remove metadata entry
    pub fn remove_metadata(&mut self, key: &str) {
        if let Some(ref mut metadata) = self.metadata {
            metadata.remove(key);
        }
    }
    
    /// Update item title
    pub fn update_title(&mut self, title: String) -> Result<(), String> {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        
        self.title = title.trim().to_string();
        self.updated_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string());
        Ok(())
    }
    
    /// Update item position
    pub fn update_position(&mut self, position: u32) {
        self.position = position;
        self.updated_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string());
    }
    
    /// Check if item can be deleted (has no dependencies)
    pub fn can_be_deleted(&self) -> bool {
        // Items can generally be deleted unless there are specific business rules
        true
    }
    
    /// Get deletion confirmation message
    pub fn get_deletion_confirmation_message(&self, child_count: usize) -> String {
        if child_count == 0 {
            format!("Delete '{}'?", self.title)
        } else {
            format!("Delete '{}' and {} child items?", self.title, child_count)
        }
    }
}

/// Hierarchy tree structure for managing relationships
#[derive(Debug, Default, Clone)]
pub struct HierarchyTree {
    items: HashMap<String, HierarchyItem>,
    root_items: Vec<String>, // Unassigned and Manuscript level items
}

impl HierarchyTree {
    /// Create a new empty hierarchy tree
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add an item to the hierarchy tree
    pub fn add_item(&mut self, item: HierarchyItem) -> Result<(), String> {
        // Validate the item
        item.validate()?;
        
        // Check for duplicate ID
        if self.items.contains_key(&item.id) {
            return Err(format!("Item with ID '{}' already exists", item.id));
        }
        
        // Validate parent exists if specified and check hierarchy rules
        if let Some(ref parent_id) = item.parent_id {
            if !self.items.contains_key(parent_id) {
                return Err(format!("Parent item with ID '{}' does not exist", parent_id));
            }
            
            // Get parent to validate relationship
            if let Some(parent) = self.items.get(parent_id) {
                if !parent.can_have_as_child(&item) {
                    return Err(format!(
                        "Cannot add {} as child of {} - invalid hierarchy relationship",
                        item.level.display_name(),
                        parent.level.display_name()
                    ));
                }
            }
            
            // Update parent's children list
            if let Some(parent) = self.items.get_mut(parent_id) {
                parent.add_child(item.id.clone());
            }
        } else {
            // Validate that non-root items have a parent
            if !item.level.is_top_level() {
                return Err(format!("{} items must have a parent", item.level.display_name()));
            }
        }
        
        // Add to appropriate root list if it's a root item
        if item.level.is_top_level() && item.parent_id.is_none() {
            // Ensure we don't add duplicates to root list
            if !self.root_items.contains(&item.id) {
                self.root_items.push(item.id.clone());
                self.root_items.sort(); // Keep root items sorted
            }
        }
        
        // Add the item to the tree
        self.items.insert(item.id.clone(), item);
        
        Ok(())
    }
    
    /// Remove an item from the hierarchy tree
    pub fn remove_item(&mut self, item_id: &str) -> Result<HierarchyItem, String> {
        let item = self.items.remove(item_id)
            .ok_or_else(|| format!("Item with ID '{}' not found", item_id))?;
        
        // Remove from root items if it's a root
        if item.level.is_top_level() {
            self.root_items.retain(|id| id != item_id);
        }
        
        // Remove from parent's children list
        if let Some(ref parent_id) = item.parent_id {
            if let Some(parent) = self.items.get_mut(parent_id) {
                parent.remove_child(item_id);
            }
        }
        
        // Remove all children recursively
        self.remove_children_recursive(item_id)?;
        
        Ok(item)
    }
    
    /// Remove all children of an item recursively
    fn remove_children_recursive(&mut self, parent_id: &str) -> Result<(), String> {
        // Use a more efficient approach to avoid potential infinite recursion
        let mut items_to_remove = Vec::new();
        
        // First pass: collect all items that need to be removed
        self.collect_descendants(parent_id, &mut items_to_remove);
        
        // Second pass: actually remove the items and update parent references
        for item_id in &items_to_remove {
            // Remove from parent's children list
            // Remove from parent's children list
            let parent_id = self.items.get(item_id).and_then(|item| item.parent_id.clone());
            
            if let Some(pid) = parent_id {
                if let Some(parent) = self.items.get_mut(&pid) {
                    parent.remove_child(item_id);
                }
            }
            
            // Remove from root items if applicable
            let is_top_level = self.items.get(item_id).map(|item| item.level.is_top_level()).unwrap_or(false);
            if is_top_level {
                self.root_items.retain(|id| id != item_id);
            }
        }
        
        // Third pass: remove the items from the main collection
        for item_id in items_to_remove {
            self.items.remove(&item_id);
        }
        
        Ok(())
    }
    
    /// Collect all descendants of an item (helper for recursive removal)
    fn collect_descendants(&self, parent_id: &str, items: &mut Vec<String>) {
        // Find direct children
        for (id, item) in &self.items {
            if item.parent_id.as_deref() == Some(parent_id) {
                items.push(id.clone());
                // Recursively collect their children
                self.collect_descendants(id, items);
            }
        }
    }
    
    /// Get an item by ID
    pub fn get_item(&self, item_id: &str) -> Option<&HierarchyItem> {
        self.items.get(item_id)
    }
    
    /// Get a mutable reference to an item
    pub fn get_item_mut(&mut self, item_id: &str) -> Option<&mut HierarchyItem> {
        self.items.get_mut(item_id)
    }
    
    /// Get all root items (Unassigned and Manuscript level)
    pub fn get_root_items(&self) -> Vec<&HierarchyItem> {
        self.root_items
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect()
    }
    
    /// Get children of a specific item
    pub fn get_children(&self, parent_id: &str) -> Vec<&HierarchyItem> {
        let parent = match self.items.get(parent_id) {
            Some(p) => p,
            None => return Vec::new(),
        };
        
        parent.children
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect()
    }
    
    /// Move an item to a new parent
    /// Move an item to a new parent
    pub fn move_item(&mut self, item_id: &str, new_parent_id: Option<String>) -> Result<(), String> {
        // 1. Get item info (cloned) to avoid holding borrow
        let (item_level, old_parent_id) = {
            let item = self.items.get(item_id)
                .ok_or_else(|| format!("Item with ID '{}' not found", item_id))?;
            (item.level, item.parent_id.clone())
        };
        
        // 2. Validate new parent relationship
        if let Some(ref pid) = new_parent_id {
            let new_parent = self.items.get(pid)
                .ok_or_else(|| format!("Parent item with ID '{}' not found", pid))?;
            
            if !new_parent.level.can_have_as_child(item_level) {
                return Err(format!(
                    "Cannot move {} to {} - invalid parent-child relationship",
                    item_level.display_name(),
                    new_parent.level.display_name()
                ));
            }
        } else {
             // Validate top level
             if !item_level.is_top_level() {
                 return Err(format!("{} items must have a parent", item_level.display_name()));
             }
        }

        // 3. Update old parent
        if let Some(ref pid) = old_parent_id {
            if let Some(old_parent) = self.items.get_mut(pid) {
                old_parent.remove_child(item_id);
            }
        }
        
        // 4. Update root items list
        if item_level.is_top_level() {
            if old_parent_id.is_none() {
                self.root_items.retain(|id| id != item_id);
            }
            if new_parent_id.is_none() {
                self.root_items.push(item_id.to_string());
                self.root_items.sort();
            }
        }
        
        // 5. Update new parent
        if let Some(ref pid) = new_parent_id {
            if let Some(new_parent) = self.items.get_mut(pid) {
                new_parent.add_child(item_id.to_string());
            }
        }
        
        // 6. Update item itself
        if let Some(item) = self.items.get_mut(item_id) {
            item.parent_id = new_parent_id;
        }
        
        Ok(())
    }
    
    /// Create and add a new item to the hierarchy
    pub fn create_item(
        &mut self,
        title: String,
        level: HierarchyLevel,
        parent_id: Option<String>,
        project_id: String,
    ) -> Result<String, String> {
        let item = HierarchyItem::create_item(None, title, level, parent_id, project_id)?;
        let item_id = item.id.clone();
        self.add_item(item)?;
        Ok(item_id)
    }
    
    /// Delete an item and all its children with confirmation
    pub fn delete_item_with_confirmation(&mut self, item_id: &str) -> Result<HierarchyItem, String> {
        if let Some(item) = self.items.get(item_id) {
            // Count total items that will be deleted (including children)
            let total_items = self.count_items_in_subtree(item_id);
            
            if total_items > 1 {
                // Would normally show a confirmation dialog here
                println!("Deleting {} and {} children", item.title, total_items - 1);
            }
            
            self.remove_item(item_id)
        } else {
            Err(format!("Item with ID '{}' not found", item_id))
        }
    }
    
    /// Count items in a subtree
    pub fn count_items_in_subtree(&self, item_id: &str) -> usize {
        let mut count = 1; // Count the item itself
        
        // Get direct children
        let children = self.get_children(item_id);
        for child in children {
            count += self.count_items_in_subtree(&child.id);
        }
        
        count
    }
    
    /// Check if an item can be moved to a new parent
    pub fn can_move_item(&self, item_id: &str, new_parent_id: Option<String>) -> Result<(), String> {
        let item = self.items.get(item_id)
            .ok_or_else(|| format!("Item with ID '{}' not found", item_id))?;
        
        // Check if item would create a circular reference
        if let Some(ref new_parent_id) = new_parent_id {
            if new_parent_id == item_id {
                return Err("Cannot move item to itself".to_string());
            }
            
            // Check if new parent exists
            let new_parent = self.items.get(new_parent_id)
                .ok_or_else(|| format!("Parent item with ID '{}' not found", new_parent_id))?;
            
            // Validate parent-child relationship
            if !new_parent.can_have_as_child(item) {
                return Err(format!(
                    "Cannot move {} to {} - invalid parent-child relationship",
                    item.level.display_name(),
                    new_parent.level.display_name()
                ));
            }
            
            // Check for circular references in the hierarchy
            if self.would_create_circular_reference(item_id, new_parent_id) {
                return Err("Cannot move item - would create circular reference".to_string());
            }
        } else if !item.level.is_top_level() {
            return Err(format!("{} items must have a parent", item.level.display_name()));
        }
        
        Ok(())
    }
    
    /// Check if moving an item would create a circular reference
    fn would_create_circular_reference(&self, item_id: &str, new_parent_id: &str) -> bool {
        // Check if new_parent_id is in the subtree of item_id
        self.is_item_in_subtree(new_parent_id, item_id)
    }
    
    /// Check if an item is in the subtree of another item
    fn is_item_in_subtree(&self, item_id: &str, root_id: &str) -> bool {
        if item_id == root_id {
            return true;
        }
        
        let children = self.get_children(root_id);
        for child in children {
            if self.is_item_in_subtree(item_id, &child.id) {
                return true;
            }
        }
        
        false
    }
    
    /// Get items by level
    pub fn get_items_by_level(&self, level: HierarchyLevel) -> Vec<&HierarchyItem> {
        self.items.values()
            .filter(|item| item.level == level)
            .collect()
    }
    
    /// Get all leaf items (items with no children)
    pub fn get_leaf_items(&self) -> Vec<&HierarchyItem> {
        self.items.values()
            .filter(|item| !item.has_children())
            .collect()
    }
    
    /// Get all branch items (items with children)
    pub fn get_branch_items(&self) -> Vec<&HierarchyItem> {
        self.items.values()
            .filter(|item| item.has_children())
            .collect()
    }
    
    /// Get the full hierarchy as a nested structure
    pub fn get_hierarchy_structure(&self) -> Vec<HierarchyNode> {
        let mut structure = Vec::new();
        
        for root_item in self.get_root_items() {
            let node = self.build_hierarchy_node(root_item);
            structure.push(node);
        }
        
        structure
    }
    
    /// Build a hierarchy node for a given item
    fn build_hierarchy_node(&self, item: &HierarchyItem) -> HierarchyNode {
        let children = self.get_children(&item.id)
            .into_iter()
            .map(|child| self.build_hierarchy_node(child))
            .collect();
        
        HierarchyNode {
            item: item.clone(),
            children,
        }
    }
    
    /// Get all items in the tree
    pub fn get_all_items(&self) -> Vec<&HierarchyItem> {
        self.items.values().collect()
    }
    
    /// Get the size of the hierarchy tree
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Check if the hierarchy tree is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    /// Clear the hierarchy tree
    pub fn clear(&mut self) {
        self.items.clear();
        self.root_items.clear();
    }
}

/// Node in the hierarchy tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyNode {
    pub item: HierarchyItem,
    pub children: Vec<HierarchyNode>,
}

impl HierarchyNode {
    /// Check if this node has any children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
    
    /// Get the depth of this node in the hierarchy
    pub fn depth(&self) -> u32 {
        self.get_max_depth(0)
    }
    
    /// Recursively calculate maximum depth
    fn get_max_depth(&self, current_depth: u32) -> u32 {
        let mut max_depth = current_depth;
        for child in &self.children {
            max_depth = max_depth.max(child.get_max_depth(current_depth + 1));
        }
        max_depth
    }
    
    /// Find a node by item ID
    pub fn find_node(&self, item_id: &str) -> Option<&HierarchyNode> {
        if self.item.id == item_id {
            return Some(self);
        }
        
        for child in &self.children {
            if let Some(found) = child.find_node(item_id) {
                return Some(found);
            }
        }
        
        None
    }
    
    /// Find a mutable node by item ID
    pub fn find_node_mut(&mut self, item_id: &str) -> Option<&mut HierarchyNode> {
        if self.item.id == item_id {
            return Some(self);
        }
        
        for child in &mut self.children {
            if let Some(found) = child.find_node_mut(item_id) {
                return Some(found);
            }
        }
        
        None
    }
}