//! Research Writing Tool
//!
//! This module provides research writing functionality.
//!
//! NOTE: This file has been updated to remove Egui and eframe dependencies and focus on Slint-only implementation.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

use crate::database::models::codex::DatabaseResult;

/// Research material type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchMaterialType {
    Book,
    Article,
    Website,
    Video,
    Interview,
    Document,
    Other,
}

impl ResearchMaterialType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ResearchMaterialType::Book => "📚 Book",
            ResearchMaterialType::Article => "📄 Article",
            ResearchMaterialType::Website => "🌐 Website",
            ResearchMaterialType::Video => "🎥 Video",
            ResearchMaterialType::Interview => "💬 Interview",
            ResearchMaterialType::Document => "📋 Document",
            ResearchMaterialType::Other => "📦 Other",
        }
    }
}

/// Research material
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMaterial {
    pub id: Uuid,
    pub title: String,
    pub material_type: ResearchMaterialType,
    pub author: Option<String>,
    pub url: Option<String>,
    pub file_path: Option<std::path::PathBuf>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
    pub citation_info: Option<String>,
}

/// Research writing state
#[derive(Debug)]
pub struct ResearchWritingState {
    pub materials: Vec<ResearchMaterial>,
    pub selected_material_id: Option<Uuid>,
    pub filter_material_type: Option<ResearchMaterialType>,
    pub search_query: String,
    pub show_add_dialog: bool,
    pub upload_progress: f32,
    pub current_material: ResearchMaterial,
}

impl Default for ResearchWritingState {
    fn default() -> Self {
        Self {
            materials: Vec::new(),
            selected_material_id: None,
            filter_material_type: None,
            search_query: String::new(),
            show_add_dialog: false,
            upload_progress: 0.0,
            current_material: ResearchMaterial {
                id: Uuid::new_v4(),
                title: String::new(),
                material_type: ResearchMaterialType::Other,
                author: None,
                url: None,
                file_path: None,
                description: None,
                tags: Vec::new(),
                notes: String::new(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                citation_info: None,
            },
        }
    }
}

/// Research writing tool implementation for Slint integration
pub struct ResearchWritingTool {
    /// Research state
    state: Arc<Mutex<ResearchWritingState>>,
    /// Callback for when materials change
    on_materials_changed: Option<Box<dyn Fn()>>,
    /// Callback for creating new materials
    on_create_material: Option<Box<dyn Fn(ResearchMaterial) -> Result<Uuid, String>>>,
}

impl ResearchWritingTool {
    /// Create a new research writing tool
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ResearchWritingState::default())),
            on_materials_changed: None,
            on_create_material: None,
        }
    }
    
    /// Set callback for materials changed
    pub fn set_materials_changed_callback<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_materials_changed = Some(Box::new(callback));
    }
    
    /// Set callback for creating materials
    pub fn set_create_material_callback<F>(&mut self, callback: F)
    where
        F: Fn(ResearchMaterial) -> Result<Uuid, String> + 'static,
    {
        self.on_create_material = Some(Box::new(callback));
    }
    
    /// Add a new research material
    pub fn add_material(&self, material: ResearchMaterial) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        state.materials.push(material);
        
        // Notify listeners of materials change
        if let Some(callback) = &self.on_materials_changed {
            callback();
        }
        
        Ok(())
    }
    
    /// Remove a research material
    pub fn remove_material(&self, material_id: Uuid) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        if let Some(index) = state.materials.iter().position(|m| m.id == material_id) {
            state.materials.remove(index);
            
            // Clear selection if it was the selected material
            if state.selected_material_id == Some(material_id) {
                state.selected_material_id = None;
            }
            
            // Notify listeners of materials change
            if let Some(callback) = &self.on_materials_changed {
                callback();
            }
            
            Ok(())
        } else {
            Err(format!("Material with ID {} not found", material_id))
        }
    }
    
    /// Update a research material
    pub fn update_material(&self, material: &ResearchMaterial) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        if let Some(existing_material) = state.materials.iter_mut().find(|m| m.id == material.id) {
            *existing_material = material.clone();
            
            // Notify listeners of materials change
            if let Some(callback) = &self.on_materials_changed {
                callback();
            }
            
            Ok(())
        } else {
            Err(format!("Material with ID {} not found", material.id))
        }
    }
    
    /// Get all materials
    pub fn get_materials(&self) -> Vec<ResearchMaterial> {
        let state = self.state.lock().unwrap();
        state.materials.clone()
    }
    
    /// Get materials by type
    pub fn get_materials_by_type(&self, material_type: ResearchMaterialType) -> Vec<ResearchMaterial> {
        let state = self.state.lock().unwrap();
        state.materials.iter()
            .filter(|m| m.material_type == material_type)
            .cloned()
            .collect()
    }
    
    /// Search materials
    pub fn search_materials(&self, query: &str) -> Vec<ResearchMaterial> {
        let state = self.state.lock().unwrap();
        let query_lower = query.to_lowercase();
        
        state.materials.iter()
            .filter(|material| {
                material.title.to_lowercase().contains(&query_lower) ||
                material.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower)) ||
                material.notes.to_lowercase().contains(&query_lower) ||
                material.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }
    
    /// Select a material
    pub fn select_material(&self, material_id: Option<Uuid>) {
        let mut state = self.state.lock().unwrap();
        state.selected_material_id = material_id;
    }
    
    /// Get selected material
    pub fn get_selected_material(&self) -> Option<ResearchMaterial> {
        let state = self.state.lock().unwrap();
        if let Some(selected_id) = state.selected_material_id {
            state.materials.iter()
                .find(|m| m.id == selected_id)
                .cloned()
        } else {
            None
        }
    }
    
    /// Create citation for a material
    pub fn create_citation(&self, material_id: Uuid) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.materials.iter()
            .find(|m| m.id == material_id)
            .and_then(|material| {
                material.citation_info.clone().or_else(|| {
                    // Generate basic citation
                    let mut citation = String::new();
                    if let Some(author) = &material.author {
                        citation.push_str(author);
                        citation.push_str(". ");
                    }
                    citation.push_str(&material.title);
                    if let Some(url) = &material.url {
                        citation.push_str(". Available at: ");
                        citation.push_str(url);
                    }
                    Some(citation)
                })
            })
    }
    
    /// Export research materials
    pub fn export_materials(&self) -> String {
        let state = self.state.lock().unwrap();
        format!("Exporting {} research materials", state.materials.len())
    }
    
    /// Get material statistics
    pub fn get_material_statistics(&self) -> HashMap<ResearchMaterialType, usize> {
        let state = self.state.lock().unwrap();
        let mut stats = HashMap::new();
        
        for material in &state.materials {
            *stats.entry(material.material_type.clone()).or_insert(0) += 1;
        }
        
        stats
    }
    
    /// Update filter
    pub fn update_filter(&self, filter_type: Option<ResearchMaterialType>) {
        let mut state = self.state.lock().unwrap();
        state.filter_material_type = filter_type;
    }
    
    /// Update search query
    pub fn update_search_query(&self, query: String) {
        let mut state = self.state.lock().unwrap();
        state.search_query = query;
    }
}

impl Default for ResearchWritingTool {
    fn default() -> Self {
        Self::new()
    }
}

// Note: All Egui-based rendering methods have been removed
// Research writing interface is now handled through Slint components