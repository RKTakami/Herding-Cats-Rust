//! Unified Drag and Drop System
//!
//! This module provides drag and drop functionality across all writing tools.
//!
//! NOTE: This file has been updated to remove Egui dependencies and focus on Slint-only implementation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ui_state::AppState;

/// Drag cursor types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragCursor {
    Default,
    Move,
    Copy,
    Link,
    Forbidden,
}

impl Default for DragCursor {
    fn default() -> Self {
        Self::Default
    }
}

/// Unified drag data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDragData {
    pub source_id: String,
    pub source_tool: String,
    pub data_type: DragDataType,
    pub data: serde_json::Value,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragDataType {
    HierarchyItem,
    CodexEntry,
    ResearchMaterial,
    Note,
    PlotPoint,
    AnalysisResult,
    TextSelection,
    File,
    Custom(String),
}

impl UnifiedDragData {
    /// Create new drag data
    pub fn new(source_id: String, source_tool: String, data_type: DragDataType, data: serde_json::Value) -> Self {
        Self {
            source_id,
            source_tool,
            data_type,
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to drag data
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Serialize drag data for transfer
    pub fn serialize(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("Failed to serialize drag data: {}", e))
    }
    
    /// Deserialize drag data from string
    pub fn deserialize(data: &str) -> Result<Self, String> {
        serde_json::from_str(data)
            .map_err(|e| format!("Failed to deserialize drag data: {}", e))
    }
}

/// Drop result
#[derive(Debug, Clone)]
pub enum DropResult {
    Success,
    Failure(String),
    Partial(Vec<String>),
}

/// Drop zone configuration
#[derive(Debug, Clone)]
pub struct DropZoneConfig {
    pub accepted_types: Vec<DragDataType>,
    pub allowed_sources: Vec<String>,
    pub visual_feedback: Option<DropVisualFeedback>,
    pub on_drop: Box<dyn Fn(&UnifiedDragData) -> DropResult>,
}

#[derive(Debug, Clone)]
pub enum DropVisualFeedback {
    Highlight,
    BorderChange,
    IconIndicator,
    Custom(String),
}

/// Drag operation state
#[derive(Debug, Clone)]
pub struct DragOperationState {
    pub is_dragging: bool,
    pub source_window: Option<String>,
    pub drag_data: Option<String>,
    pub drag_type: Option<String>,
    pub current_position: Option<(f32, f32)>,
    pub drop_targets: Vec<String>,
    pub visual_feedback: Option<DragVisualFeedback>,
}

#[derive(Debug, Clone)]
pub enum DragVisualFeedback {
    GhostElement(GhostElement),
    DropZoneHighlight(DropZoneHighlight),
    CursorChange(DragCursor),
}

#[derive(Debug, Clone)]
pub struct GhostElement {
    pub content: String,
    pub position_offset: (f32, f32),
    pub opacity: f32,
    pub size: Option<(f32, f32)>,
}

#[derive(Debug, Clone)]
pub struct DropZoneHighlight {
    pub zone_id: String,
    pub color: Option<String>,
    pub border_style: Option<String>,
    pub animation: Option<String>,
}

/// Drag and drop manager for Slint integration
pub struct DragDropManager {
    /// Current drag state
    pub drag_state: DragOperationState,
    /// Registered drop zones
    pub drop_zones: HashMap<String, DropZoneConfig>,
    /// Drag compatibility matrix
    pub compatibility_matrix: DragCompatibilityMatrix,
    /// Callback for drag state changes
    pub on_drag_state_changed: Option<Box<dyn Fn(&DragOperationState)>>,
}

/// Drag compatibility matrix
#[derive(Debug, Default)]
pub struct DragCompatibilityMatrix {
    // Research tool compatibilities
    pub research_to_notes: bool,
    pub research_to_hierarchy: bool,
    
    // Codex tool compatibilities
    pub codex_to_notes: bool,
    pub codex_to_hierarchy: bool,
    
    // Plot tool compatibilities
    pub plot_to_notes: bool,
    pub plot_to_hierarchy: bool,
    
    // Analysis tool compatibilities
    pub analysis_to_notes: bool,
    pub analysis_to_hierarchy: bool,
    
    // Notes tool compatibilities
    pub notes_to_analysis: bool,
    pub notes_to_hierarchy: bool,
    
    // Hierarchy tool compatibilities
    pub hierarchy_to_research: bool,
    pub hierarchy_to_codex: bool,
    pub hierarchy_to_plot: bool,
    pub hierarchy_to_analysis: bool,
    pub hierarchy_to_notes: bool,
}

impl DragDropManager {
    /// Create a new drag and drop manager
    pub fn new() -> Self {
        Self {
            drag_state: DragOperationState {
                is_dragging: false,
                source_window: None,
                drag_data: None,
                drag_type: None,
                current_position: None,
                drop_targets: Vec::new(),
                visual_feedback: None,
            },
            drop_zones: HashMap::new(),
            compatibility_matrix: DragCompatibilityMatrix::default(),
            on_drag_state_changed: None,
        }
    }
    
    /// Register a drop zone
    pub fn register_drop_zone(&mut self, zone_id: String, config: DropZoneConfig) {
        self.drop_zones.insert(zone_id, config);
    }
    
    /// Unregister a drop zone
    pub fn unregister_drop_zone(&mut self, zone_id: &str) {
        self.drop_zones.remove(zone_id);
    }
    
    /// Start a drag operation
    pub fn start_drag(&mut self, drag_data: &UnifiedDragData, source_window: &str) -> Result<(), String> {
        let serialized_data = drag_data.serialize()?;
        
        self.drag_state = DragOperationState {
            is_dragging: true,
            source_window: Some(source_window.to_string()),
            drag_data: Some(serialized_data),
            drag_type: Some(format!("{:?}", drag_data.data_type)),
            current_position: None,
            drop_targets: self.find_valid_drop_targets(drag_data),
            visual_feedback: None,
        };
        
        // Notify listeners of drag state change
        if let Some(callback) = &self.on_drag_state_changed {
            callback(&self.drag_state);
        }
        
        Ok(())
    }
    
    /// Update drag position
    pub fn update_drag_position(&mut self, position: (f32, f32)) {
        self.drag_state.current_position = Some(position);
        
        // Update visual feedback based on drop targets
        self.update_visual_feedback();
        
        // Notify listeners of drag state change
        if let Some(callback) = &self.on_drag_state_changed {
            callback(&self.drag_state);
        }
    }
    
    /// Handle drag over a potential drop target
    pub fn handle_drag_over(&mut self, target_id: &str) -> Option<DropResult> {
        if let Some(config) = self.drop_zones.get(target_id) {
            // Check if target accepts this drag data type
            if let Some(drag_data_str) = &self.drag_state.drag_data {
                if let Ok(drag_data) = UnifiedDragData::deserialize(drag_data_str) {
                    if config.accepted_types.contains(&drag_data.data_type) {
                        // Update visual feedback
                        self.drag_state.visual_feedback = Some(DragVisualFeedback::DropZoneHighlight(
                            DropZoneHighlight {
                                zone_id: target_id.to_string(),
                                color: Some("#4CAF50".to_string()),
                                border_style: Some("2px solid".to_string()),
                                animation: Some("pulse".to_string()),
                            }
                        ));
                        
                        // Notify listeners of drag state change
                        if let Some(callback) = &self.on_drag_state_changed {
                            callback(&self.drag_state);
                        }
                        
                        return None; // Continue drag operation
                    }
                }
            }
        }
        
        // No valid drop target
        self.drag_state.visual_feedback = Some(DragVisualFeedback::DropZoneHighlight(
            DropZoneHighlight {
                zone_id: target_id.to_string(),
                color: Some("#F44336".to_string()),
                border_style: Some("2px dashed".to_string()),
                animation: None,
            }
        ));
        
        // Notify listeners of drag state change
        if let Some(callback) = &self.on_drag_state_changed {
            callback(&self.drag_state);
        }
        
        None
    }
    
    /// Complete a drag operation with drop
    pub fn complete_drag(&mut self, target_id: &str) -> Result<DropResult, String> {
        if !self.drag_state.is_dragging {
            return Err("No active drag operation".to_string());
        }
        
        let result = if let (Some(target_config), Some(drag_data_str)) = 
            (self.drop_zones.get(target_id), &self.drag_state.drag_data) {
            
            if let Ok(drag_data) = UnifiedDragData::deserialize(drag_data_str) {
                if target_config.accepted_types.contains(&drag_data.data_type) {
                    // Execute drop handler
                    (target_config.on_drop)(&drag_data)
                } else {
                    DropResult::Failure("Incompatible data type".to_string())
                }
            } else {
                DropResult::Failure("Failed to deserialize drag data".to_string())
            }
        } else {
            DropResult::Failure("Invalid drop target or drag data".to_string())
        };
        
        // End drag operation
        self.end_drag();
        
        Ok(result)
    }
    
    /// Cancel current drag operation
    pub fn cancel_drag(&mut self) {
        self.end_drag();
    }
    
    /// End drag operation
    fn end_drag(&mut self) {
        self.drag_state = DragOperationState {
            is_dragging: false,
            source_window: None,
            drag_data: None,
            drag_type: None,
            current_position: None,
            drop_targets: Vec::new(),
            visual_feedback: None,
        };
        
        // Notify listeners of drag state change
        if let Some(callback) = &self.on_drag_state_changed {
            callback(&self.drag_state);
        }
    }
    
    /// Find valid drop targets for drag data
    fn find_valid_drop_targets(&self, drag_data: &UnifiedDragData) -> Vec<String> {
        let mut valid_targets = Vec::new();
        
        for (zone_id, config) in &self.drop_zones {
            if config.accepted_types.contains(&drag_data.data_type) &&
               (config.allowed_sources.is_empty() || 
                config.allowed_sources.contains(&drag_data.source_tool)) {
                valid_targets.push(zone_id.clone());
            }
        }
        
        valid_targets
    }
    
    /// Update visual feedback based on current drag state
    fn update_visual_feedback(&mut self) {
        if self.drag_state.is_dragging && self.drag_state.drop_targets.len() > 0 {
            // Show ghost element feedback
            if let Some(drag_data_str) = &self.drag_state.drag_data {
                if let Ok(drag_data) = UnifiedDragData::deserialize(drag_data_str) {
                    let content = match drag_data.data_type {
                        DragDataType::HierarchyItem => "📂 Hierarchy Item".to_string(),
                        DragDataType::CodexEntry => "📖 Codex Entry".to_string(),
                        DragDataType::ResearchMaterial => "🔍 Research Material".to_string(),
                        DragDataType::Note => "📝 Note".to_string(),
                        DragDataType::PlotPoint => "📈 Plot Point".to_string(),
                        DragDataType::AnalysisResult => "📊 Analysis Result".to_string(),
                        DragDataType::TextSelection => "✂️ Text Selection".to_string(),
                        DragDataType::File => "📁 File".to_string(),
                        DragDataType::Custom(ref s) => format!("🔧 {}", s),
                    };
                    
                    self.drag_state.visual_feedback = Some(DragVisualFeedback::GhostElement(
                        GhostElement {
                            content,
                            position_offset: (10.0, 10.0),
                            opacity: 0.8,
                            size: None,
                        }
                    ));
                }
            }
        } else {
            self.drag_state.visual_feedback = None;
        }
    }
    
    /// Set callback for drag state changes
    pub fn set_drag_state_changed_callback<F>(&mut self, callback: F)
    where
        F: Fn(&DragOperationState) + 'static,
    {
        self.on_drag_state_changed = Some(Box::new(callback));
    }
    
    /// Get drag compatibility between tools
    pub fn get_drag_compatibility(&self, source_tool: &str, target_tool: &str) -> bool {
        match (source_tool, target_tool) {
            ("Research", "Notes") => self.compatibility_matrix.research_to_notes,
            ("Research", "Hierarchy") => self.compatibility_matrix.research_to_hierarchy,
            ("Codex", "Notes") => self.compatibility_matrix.codex_to_notes,
            ("Codex", "Hierarchy") => self.compatibility_matrix.codex_to_hierarchy,
            ("Plot", "Notes") => self.compatibility_matrix.plot_to_notes,
            ("Plot", "Hierarchy") => self.compatibility_matrix.plot_to_hierarchy,
            ("Analysis", "Notes") => self.compatibility_matrix.analysis_to_notes,
            ("Analysis", "Hierarchy") => self.compatibility_matrix.analysis_to_hierarchy,
            ("Notes", "Analysis") => self.compatibility_matrix.notes_to_analysis,
            ("Notes", "Hierarchy") => self.compatibility_matrix.notes_to_hierarchy,
            ("Hierarchy", "Research") => self.compatibility_matrix.hierarchy_to_research,
            ("Hierarchy", "Codex") => self.compatibility_matrix.hierarchy_to_codex,
            ("Hierarchy", "Plot") => self.compatibility_matrix.hierarchy_to_plot,
            ("Hierarchy", "Analysis") => self.compatibility_matrix.hierarchy_to_analysis,
            ("Hierarchy", "Notes") => self.compatibility_matrix.hierarchy_to_notes,
            _ => false,
        }
    }
    
    /// Set up default compatibility matrix
    pub fn setup_default_compatibility(&mut self) {
        self.compatibility_matrix = DragCompatibilityMatrix {
            // Research can go to notes and hierarchy
            research_to_notes: true,
            research_to_hierarchy: true,
            
            // Codex can go to notes, but not hierarchy (would create circular references)
            codex_to_notes: true,
            codex_to_hierarchy: false,
            
            // Plot can go to notes, but not hierarchy
            plot_to_notes: true,
            plot_to_hierarchy: false,
            
            // Analysis can go to notes and hierarchy
            analysis_to_notes: true,
            analysis_to_hierarchy: true,
            
            // Notes can go to analysis and hierarchy
            notes_to_analysis: true,
            notes_to_hierarchy: true,
            
            // Hierarchy can send structure to most tools
            hierarchy_to_research: false,
            hierarchy_to_codex: true,
            hierarchy_to_plot: true,
            hierarchy_to_analysis: true,
            hierarchy_to_notes: true,
        };
    }
}

impl Default for DragDropManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DragOperationState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            source_window: None,
            drag_data: None,
            drag_type: None,
            current_position: None,
            drop_targets: Vec::new(),
            visual_feedback: None,
        }
    }
}

// Note: All Egui-based rendering methods have been removed
// Drag and drop visual feedback is now handled through Slint components