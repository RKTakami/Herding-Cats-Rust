//! Hierarchy Tool Implementation
//!
//! This module provides the hierarchy tool implementation for managing
//! document hierarchy and structure using Slint.
//!
//! Note: This file was previously Egui-based but has been removed to focus on Slint-only implementation.
//! The hierarchy tool is now implemented through the Slint components in writing_tools_enhanced.slint.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ui_state::{HierarchyItem, ToolType, AppState};
use super::hierarchy_base::{HierarchyTree, HierarchyLevel};
use super::hierarchy_drag::HierarchyDragHandler;
use super::hierarchy_database::HierarchyDatabaseService;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Hierarchy tool implementation for Slint integration
pub struct HierarchyTool {
    /// Core hierarchy tree data structure
    hierarchy_tree: HierarchyTree,
    /// Drag and drop handler
    drag_handler: HierarchyDragHandler,
    /// Database service for persistence
    database_service: Option<Arc<RwLock<HierarchyDatabaseService>>>,
    /// Callback for when hierarchy changes
    on_hierarchy_changed: Option<Box<dyn Fn(&HierarchyTree)>>,
    /// Callback for creating new items
    on_create_item: Option<Box<dyn Fn(HierarchyLevel, Option<String>, String) -> Result<String, String>>>,
    /// Callback for deleting items
    on_delete_item: Option<Box<dyn Fn(&str) -> Result<HierarchyItem, String>>>,
}

impl HierarchyTool {
    /// Create a new hierarchy tool
    pub fn new() -> Self {
        Self {
            hierarchy_tree: HierarchyTree::new(),
            drag_handler: HierarchyDragHandler::new(),
            database_service: None,
            on_hierarchy_changed: None,
            on_create_item: None,
            on_delete_item: None,
        }
    }
    
    /// Set the database service
    pub fn set_database_service(&mut self, service: Arc<RwLock<HierarchyDatabaseService>>) {
        self.database_service = Some(service);
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
    
    /// Set callback for deleting items
    pub fn set_delete_item_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str) -> Result<HierarchyItem, String> + 'static,
    {
        self.on_delete_item = Some(Box::new(callback));
    }
    
    /// Load hierarchy data from database
    pub async fn load_hierarchy(&mut self, project_id: &str) -> Result<(), String> {
        if let Some(db_service) = &self.database_service {
            let items = db_service.read().await.get_items_by_project(project_id).await?;
            let mut tree = HierarchyTree::new();
            
            for item in items {
                if let Err(e) = tree.add_item(item) {
                    log::warn!("Failed to add hierarchy item: {}", e);
                }
            }
            
            self.hierarchy_tree = tree;
            
            // Notify listeners of hierarchy change
            if let Some(callback) = &self.on_hierarchy_changed {
                callback(&self.hierarchy_tree);
            }
            
            Ok(())
        } else {
            Err("Database service not available".to_string())
        }
    }
    
    /// Get the current hierarchy tree
    pub fn get_hierarchy(&self) -> &HierarchyTree {
        &self.hierarchy_tree
    }
    
    /// Get mutable access to hierarchy tree
    pub fn get_hierarchy_mut(&mut self) -> &mut HierarchyTree {
        &mut self.hierarchy_tree
    }
    
    /// Add a new item to the hierarchy
    pub async fn add_item(&mut self, item: HierarchyItem) -> Result<(), String> {
        self.hierarchy_tree.add_item(item.clone())?;
        
        // Persist to database if available
        if let Some(db_service) = &self.database_service {
            db_service.read().await.create_item(&item).await?;
        }
        
        // Notify listeners of hierarchy change
        if let Some(callback) = &self.on_hierarchy_changed {
            callback(&self.hierarchy_tree);
        }
        
        Ok(())
    }
    
    /// Remove an item from the hierarchy
    pub async fn remove_item(&mut self, item_id: &str) -> Result<HierarchyItem, String> {
        let removed_item = self.hierarchy_tree.remove_item(item_id)?;
        
        // Remove from database if available
        if let Some(db_service) = &self.database_service {
            db_service.read().await.delete_item(item_id).await?;
        }
        
        // Notify listeners of hierarchy change
        if let Some(callback) = &self.on_hierarchy_changed {
            callback(&self.hierarchy_tree);
        }
        
        Ok(removed_item)
    }
    
    /// Update an existing item
    pub async fn update_item(&mut self, item: &HierarchyItem) -> Result<(), String> {
        if let Some(existing_item) = self.hierarchy_tree.get_item_mut(&item.id) {
            *existing_item = item.clone();
            
            // Update database if available
            if let Some(db_service) = &self.database_service {
                db_service.read().await.update_item(item).await?;
            }
            
            // Notify listeners of hierarchy change
            if let Some(callback) = &self.on_hierarchy_changed {
                callback(&self.hierarchy_tree);
            }
            
            Ok(())
        } else {
            Err(format!("Item with ID '{}' not found", item.id))
        }
    }
    
    /// Move an item to a new parent
    pub async fn move_item(&mut self, item_id: &str, new_parent_id: Option<String>) -> Result<(), String> {
        self.hierarchy_tree.move_item(item_id, new_parent_id)?;
        
        // Update database if available
        if let Some(db_service) = &self.database_service {
            // This would need position information for proper database update
            let item = self.hierarchy_tree.get_item(item_id)
                .ok_or_else(|| format!("Item with ID '{}' not found", item_id))?;
            db_service.read().await.update_item(item).await?;
        }
        
        // Notify listeners of hierarchy change
        if let Some(callback) = &self.on_hierarchy_changed {
            callback(&self.hierarchy_tree);
        }
        
        Ok(())
    }
    
    /// Get selected item for external access
    pub fn get_selected_item(&self) -> Option<&HierarchyItem> {
        // This would be implemented when we have selection state
        None
    }
    
    /// Update items from external source
    pub fn update_items(&mut self, new_items: Vec<HierarchyItem>) {
        let mut tree = HierarchyTree::new();
        for item in new_items {
            if let Err(e) = tree.add_item(item) {
                log::warn!("Failed to add hierarchy item: {}", e);
            }
        }
        self.hierarchy_tree = tree;
        
        // Notify listeners of hierarchy change
        if let Some(callback) = &self.on_hierarchy_changed {
            callback(&self.hierarchy_tree);
        }
    }
    
    /// Export hierarchy to file
    pub fn export_hierarchy(&self) -> String {
        // Export implementation would go here
        format!("Exporting hierarchy with {} items", self.hierarchy_tree.len())
    }
    
    /// Refresh hierarchy data from database
    pub async fn refresh_hierarchy(&mut self, project_id: &str) -> Result<(), String> {
        self.load_hierarchy(project_id).await
    }
    
    /// Reset hierarchy to default state
    pub fn reset_hierarchy(&mut self) {
        self.hierarchy_tree.clear();
        
        // Notify listeners of hierarchy change
        if let Some(callback) = &self.on_hierarchy_changed {
            callback(&self.hierarchy_tree);
        }
    }
}

impl Default for HierarchyTool {
    fn default() -> Self {
        Self::new()
    }
}