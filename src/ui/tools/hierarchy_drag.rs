//! Hierarchy Drag and Drop Module
//!
//! Drag and drop handlers for hierarchy tool operations including
//! within-tool dragging and cross-tool integration.

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::ui_state::AppState;
use crate::services::ServiceRegistry;
use super::hierarchy_base::{HierarchyItem, HierarchyLevel};

/// Hierarchy drag operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HierarchyDragOperation {
    MoveWithinHierarchy,
    MoveToOtherTool,
    CopyToOtherTool,
}

/// Hierarchy drag data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyDragData {
    pub item_id: String,
    pub item_title: String,
    pub item_level: HierarchyLevel,
    pub source_tool: String,
    pub operation: HierarchyDragOperation,
    pub timestamp: u64,
}

impl HierarchyDragData {
    /// Create new drag data for moving within hierarchy
    pub fn new_move(item: &HierarchyItem, source_tool: &str) -> Self {
        Self {
            item_id: item.id.clone(),
            item_title: item.title.clone(),
            item_level: item.level,
            source_tool: source_tool.to_string(),
            operation: HierarchyDragOperation::MoveWithinHierarchy,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
    
    /// Create new drag data for moving to another tool
    pub fn new_move_to_tool(item: &HierarchyItem, source_tool: &str) -> Self {
        Self {
            item_id: item.id.clone(),
            item_title: item.title.clone(),
            item_level: item.level,
            source_tool: source_tool.to_string(),
            operation: HierarchyDragOperation::MoveToOtherTool,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
    
    /// Create new drag data for copying to another tool
    pub fn new_copy_to_tool(item: &HierarchyItem, source_tool: &str) -> Self {
        Self {
            item_id: item.id.clone(),
            item_title: item.title.clone(),
            item_level: item.level,
            source_tool: source_tool.to_string(),
            operation: HierarchyDragOperation::CopyToOtherTool,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
    
    /// Serialize drag data for AppState
    pub fn serialize(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("Failed to serialize drag data: {}", e))
    }
    
    /// Deserialize drag data from AppState
    pub fn deserialize(data: &str) -> Result<Self, String> {
        serde_json::from_str(data)
            .map_err(|e| format!("Failed to deserialize drag data: {}", e))
    }
}

/// Hierarchy drag validation result
#[derive(Debug, Clone)]
pub struct DragValidationResult {
    pub can_drop: bool,
    pub reason: Option<String>,
    pub visual_feedback: DragVisualFeedback,
}

/// Visual feedback for drag operations
#[derive(Debug, Clone)]
pub enum DragVisualFeedback {
    Allow,
    Deny,
    IndicateParent(HierarchyLevel),
    IndicatePosition,
}

/// Hierarchy drag handler for managing drag operations
pub struct HierarchyDragHandler {
    app_state: Option<*mut AppState>,
    service_registry: Option<ServiceRegistry>,
}

impl HierarchyDragHandler {
    /// Create a new hierarchy drag handler
    pub fn new() -> Self {
        Self {
            app_state: None,
            service_registry: None,
        }
    }
    
    /// Set the app state reference
    pub fn set_app_state(&mut self, app_state: *mut AppState) {
        self.app_state = Some(app_state);
    }
    
    /// Set the service registry
    pub fn set_service_registry(&mut self, service_registry: ServiceRegistry) {
        self.service_registry = Some(service_registry);
    }
    
    /// Start a drag operation for a hierarchy item
    pub fn start_drag(&mut self, item: &HierarchyItem, app_state: &mut AppState) -> Result<(), String> {
        let drag_data = HierarchyDragData::new_move(item, "hierarchy");
        let serialized_data = drag_data.serialize()?;
        
        // Update app state with drag information
        app_state.set_drag_state(true, Some("hierarchy".to_string()), Some(serialized_data));
        
        // TODO: Add visual feedback to show drag is starting
        self.show_drag_start_visuals(item);
        
        Ok(())
    }
    
    /// Validate if an item can be dropped at a target location
    pub fn validate_drop(&self, drag_data: &HierarchyDragData, target_item: Option<&HierarchyItem>) -> DragValidationResult {
        // Check if we're trying to drop an item on itself
        if let Some(target) = target_item {
            if drag_data.item_id == target.id {
                return DragValidationResult {
                    can_drop: false,
                    reason: Some("Cannot drop item on itself".to_string()),
                    visual_feedback: DragVisualFeedback::Deny,
                };
            }
        }
        
        // Validate parent-child relationship
        match target_item {
            Some(target) => {
                // Check if target can accept this item as a child
                if !target.can_have_as_child_by_level(drag_data.item_level) {
                    return DragValidationResult {
                        can_drop: false,
                        reason: Some(format!(
                            "Cannot drop {} into {} - invalid hierarchy relationship",
                            drag_data.item_level.display_name(),
                            target.level.display_name()
                        )),
                        visual_feedback: DragVisualFeedback::Deny,
                    };
                }
                
                DragValidationResult {
                    can_drop: true,
                    reason: None,
                    visual_feedback: DragVisualFeedback::IndicateParent(target.level),
                }
            }
            None => {
                // Dropping at root level - only allow Unassigned/Manuscript
                if matches!(drag_data.item_level, HierarchyLevel::Unassigned | HierarchyLevel::Manuscript) {
                    DragValidationResult {
                        can_drop: true,
                        reason: None,
                        visual_feedback: DragVisualFeedback::Allow,
                    }
                } else {
                    DragValidationResult {
                        can_drop: false,
                        reason: Some("Only Unassigned and Manuscript items can be placed at root level".to_string()),
                        visual_feedback: DragVisualFeedback::Deny,
                    }
                }
            }
        }
    }
    
    /// Handle drop operation within hierarchy
    pub fn handle_drop_within_hierarchy(&mut self, drag_data: &HierarchyDragData, target_item: Option<&HierarchyItem>, position: Option<usize>) -> Result<(), String> {
        // This would need access to the hierarchy tree and database
        // For now, return success - actual implementation would update the hierarchy
        println!("Handling drop within hierarchy: {} -> {:?}", drag_data.item_title, target_item.map(|i| i.title.clone()));
        
        Ok(())
    }
    
    /// Handle drop operation to another tool
    pub fn handle_drop_to_other_tool(&mut self, drag_data: &HierarchyDragData, target_tool: &str) -> Result<(), String> {
        println!("Handling drop to other tool: {} -> {}", drag_data.item_title, target_tool);
        
        // This would notify the target tool about the incoming data
        match drag_data.operation {
            HierarchyDragOperation::MoveToOtherTool => {
                // Move item to target tool (item would be removed from hierarchy)
                println!("Moving item to tool: {}", target_tool);
            }
            HierarchyDragOperation::CopyToOtherTool => {
                // Copy item to target tool (item remains in hierarchy)
                println!("Copying item to tool: {}", target_tool);
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Complete a drag operation
    pub fn complete_drag(&mut self, app_state: &mut AppState) -> Result<(), String> {
        // Clear drag state
        app_state.set_drag_state(false, None, None);
        
        // Hide drag visuals
        self.hide_drag_visuals();
        
        Ok(())
    }
    
    /// Cancel a drag operation
    pub fn cancel_drag(&mut self, app_state: &mut AppState) -> Result<(), String> {
        // Clear drag state
        app_state.set_drag_state(false, None, None);
        
        // Hide drag visuals
        self.hide_drag_visuals();
        
        Ok(())
    }
    
    /// Show visual feedback for drag start
    fn show_drag_start_visuals(&self, item: &HierarchyItem) {
        // TODO: Implement visual feedback (ghost element, etc.)
        println!("Starting drag for: {}", item.title);
    }
    
    /// Hide drag visuals
    fn hide_drag_visuals(&self) {
        // TODO: Hide any visual feedback elements
        println!("Hiding drag visuals");
    }
    
    /// Update drag position feedback
    pub fn update_drag_feedback(&self, validation: &DragValidationResult) {
        match validation.visual_feedback {
            DragVisualFeedback::Allow => println!("Drag allowed"),
            DragVisualFeedback::Deny => println!("Drag denied: {:?}", validation.reason),
            DragVisualFeedback::IndicateParent(level) => println!("Can drop as child of {:?}", level),
            DragVisualFeedback::IndicatePosition => println!("Can drop at position"),
        }
    }
}

/// Extension trait for HierarchyItem to support drag validation
pub trait HierarchyDragValidation {
    /// Check if this item can have another item level as a child
    fn can_have_as_child_by_level(&self, child_level: HierarchyLevel) -> bool;
}

impl HierarchyDragValidation for HierarchyItem {
    fn can_have_as_child_by_level(&self, child_level: HierarchyLevel) -> bool {
        self.level.can_have_as_child(child_level)
    }
}

/// Global hierarchy drag handler instance
lazy_static! {
    pub static ref GLOBAL_HIERARCHY_DRAG_HANDLER: std::sync::Mutex<HierarchyDragHandler> = 
        std::sync::Mutex::new(HierarchyDragHandler::new());
}

/// Get a reference to the global hierarchy drag handler
pub fn get_hierarchy_drag_handler() -> std::sync::MutexGuard<'static, HierarchyDragHandler> {
    GLOBAL_HIERARCHY_DRAG_HANDLER.lock().unwrap()
}