//! Research Tool Window
//!
//! This module provides the research tool window implementation.
//!
//! NOTE: This file has been updated to remove Egui and eframe dependencies and focus on Slint-only implementation.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::database::models::codex::DatabaseResult;
use super::research_writing::{ResearchWritingTool, ResearchMaterial, ResearchMaterialType};

/// Window action enum
#[derive(Debug, Clone)]
pub enum WindowAction {
    Close,
    Minimize,
    Maximize,
    Move,
    Resize,
}

/// Research tool window implementation for Slint integration
pub struct ResearchToolWindow {
    /// Window properties
    pub title: String,
    pub id: String,
    pub is_open: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub z_index: i32,
    
    /// Research tool instance
    pub research_tool: ResearchWritingTool,
    
    /// Window state
    pub is_dragging: bool,
    pub is_resizing: bool,
    pub drag_offset: (i32, i32),
    pub resize_handle: String,
    
    /// Dialog states
    pub show_add_dialog: bool,
    pub show_citation_dialog: bool,
    pub show_error_dialog: bool,
    pub show_success_dialog: bool,
    
    /// Dialog content
    pub error_message: String,
    pub success_message: String,
}

impl ResearchToolWindow {
    /// Create a new research tool window
    pub fn new(title: String, window_id: String) -> Self {
        Self {
            title,
            id: window_id,
            is_open: true,
            position: (100, 100),
            size: (600, 700),
            z_index: 1,
            research_tool: ResearchWritingTool::new(),
            is_dragging: false,
            is_resizing: false,
            drag_offset: (0, 0),
            resize_handle: String::new(),
            show_add_dialog: false,
            show_citation_dialog: false,
            show_error_dialog: false,
            show_success_dialog: false,
            error_message: String::new(),
            success_message: String::new(),
        }
    }
    
    /// Set window position
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }
    
    /// Set window size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = (width, height);
    }
    
    /// Set window as open/closed
    pub fn set_open(&mut self, is_open: bool) {
        self.is_open = is_open;
    }
    
    /// Start dragging window
    pub fn start_drag(&mut self, mouse_x: i32, mouse_y: i32) {
        self.is_dragging = true;
        self.drag_offset = (mouse_x - self.position.0, mouse_y - self.position.1);
    }
    
    /// Update drag position
    pub fn update_drag(&mut self, mouse_x: i32, mouse_y: i32) {
        if self.is_dragging {
            self.position = (mouse_x - self.drag_offset.0, mouse_y - self.drag_offset.1);
        }
    }
    
    /// Stop dragging
    pub fn stop_drag(&mut self) {
        self.is_dragging = false;
    }
    
    /// Start resizing window
    pub fn start_resize(&mut self, handle: &str) {
        self.is_resizing = true;
        self.resize_handle = handle.to_string();
    }
    
    /// Update resize
    pub fn update_resize(&mut self, mouse_x: i32, mouse_y: i32) {
        // Resize logic would be implemented here
        // For now, just set a default size
        self.size = (600, 700);
    }
    
    /// Stop resizing
    pub fn stop_resize(&mut self) {
        self.is_resizing = false;
        self.resize_handle.clear();
    }
    
    /// Minimize window
    pub fn minimize(&mut self) {
        // Minimize logic
    }
    
    /// Maximize window
    pub fn maximize(&mut self) {
        // Maximize logic
    }
    
    /// Restore window
    pub fn restore(&mut self) {
        // Restore logic
    }
    
    /// Show add material dialog
    pub fn show_add_material_dialog(&mut self) {
        self.show_add_dialog = true;
    }
    
    /// Hide add material dialog
    pub fn hide_add_material_dialog(&mut self) {
        self.show_add_dialog = false;
    }
    
    /// Show citation dialog
    pub fn show_citation_dialog(&mut self) {
        self.show_citation_dialog = true;
    }
    
    /// Hide citation dialog
    pub fn hide_citation_dialog(&mut self) {
        self.show_citation_dialog = false;
    }
    
    /// Show error message
    pub fn show_error(&mut self, message: String) {
        self.error_message = message;
        self.show_error_dialog = true;
    }
    
    /// Show success message
    pub fn show_success(&mut self, message: String) {
        self.success_message = message;
        self.show_success_dialog = true;
    }
    
    /// Hide all dialogs
    pub fn hide_all_dialogs(&mut self) {
        self.show_add_dialog = false;
        self.show_citation_dialog = false;
        self.show_error_dialog = false;
        self.show_success_dialog = false;
    }
    
    /// Add material through the research tool
    pub fn add_material(&self, material: ResearchMaterial) -> Result<(), String> {
        self.research_tool.add_material(material)
    }
    
    /// Remove material through the research tool
    pub fn remove_material(&self, material_id: Uuid) -> Result<(), String> {
        self.research_tool.remove_material(material_id)
    }
    
    /// Update material through the research tool
    pub fn update_material(&self, material: &ResearchMaterial) -> Result<(), String> {
        self.research_tool.update_material(material)
    }
    
    /// Get all materials
    pub fn get_materials(&self) -> Vec<ResearchMaterial> {
        self.research_tool.get_materials()
    }
    
    /// Search materials
    pub fn search_materials(&self, query: &str) -> Vec<ResearchMaterial> {
        self.research_tool.search_materials(query)
    }
    
    /// Create citation
    pub fn create_citation(&self, material_id: Uuid) -> Option<String> {
        self.research_tool.create_citation(material_id)
    }
    
    /// Select material
    pub fn select_material(&self, material_id: Option<Uuid>) {
        self.research_tool.select_material(material_id);
    }
    
    /// Get selected material
    pub fn get_selected_material(&self) -> Option<ResearchMaterial> {
        self.research_tool.get_selected_material()
    }
    
    /// Export materials
    pub fn export_materials(&self) -> String {
        self.research_tool.export_materials()
    }
    
    /// Get material statistics
    pub fn get_material_statistics(&self) -> std::collections::HashMap<ResearchMaterialType, usize> {
        self.research_tool.get_material_statistics()
    }
    
    /// Update material filter
    pub fn update_filter(&self, filter_type: Option<ResearchMaterialType>) {
        self.research_tool.update_filter(filter_type);
    }
    
    /// Update search query
    pub fn update_search_query(&self, query: String) {
        self.research_tool.update_search_query(query);
    }
}

impl Default for ResearchToolWindow {
    fn default() -> Self {
        Self::new("🔍 Research & Sources".to_string(), "research_window".to_string())
    }
}

// Note: All Egui and eframe-based rendering methods have been removed
// The research window is now handled through Slint components