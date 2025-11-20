//! Codex Drag and Drop Module
//!
//! Drag and drop handlers for codex tool operations including
//! within-tool dragging and cross-tool integration with other writing tools.

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::{ui_state::AppState, database::models::codex::{CodexEntry, CodexEntryType, CodexStatus}};
use super::codex_base::{CodexToolBase, CodexAction, CodexEvent};

/// Codex drag operation types
#[derive(Debug, Clone)]
pub enum CodexDragOperation {
    MoveWithinCodex,
    MoveToOtherTool,
    CopyToOtherTool,
    LinkToOtherTool,
}

/// Codex drag data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexDragData {
    pub entry_id: String,
    pub entry_title: String,
    pub entry_type: CodexEntryType,
    pub entry_status: CodexStatus,
    pub source_tool: String,
    pub operation: CodexDragOperation,
    pub timestamp: u64,
    pub entry_summary: Option<String>, // Brief content summary for preview
}

impl CodexDragData {
    /// Create new drag data for moving within codex
    pub fn new_move(entry: &CodexEntry, source_tool: &str) -> Self {
        Self {
            entry_id: entry.id.to_string(),
            entry_title: entry.title.clone(),
            entry_type: entry.entry_type,
            entry_status: entry.status,
            source_tool: source_tool.to_string(),
            operation: CodexDragOperation::MoveWithinCodex,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            entry_summary: Some(entry.content.chars().take(100).collect()),
        }
    }

    /// Create new drag data for moving to another tool
    pub fn new_move_to_tool(entry: &CodexEntry, source_tool: &str) -> Self {
        Self {
            entry_id: entry.id.to_string(),
            entry_title: entry.title.clone(),
            entry_type: entry.entry_type,
            entry_status: entry.status,
            source_tool: source_tool.to_string(),
            operation: CodexDragOperation::MoveToOtherTool,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            entry_summary: Some(entry.content.chars().take(100).collect()),
        }
    }

    /// Create new drag data for copying to another tool
    pub fn new_copy_to_tool(entry: &CodexEntry, source_tool: &str) -> Self {
        Self {
            entry_id: entry.id.to_string(),
            entry_title: entry.title.clone(),
            entry_type: entry.entry_type,
            entry_status: entry.status,
            source_tool: source_tool.to_string(),
            operation: CodexDragOperation::CopyToOtherTool,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            entry_summary: Some(entry.content.chars().take(100).collect()),
        }
    }

    /// Create new drag data for linking to another tool
    pub fn new_link_to_tool(entry: &CodexEntry, source_tool: &str) -> Self {
        Self {
            entry_id: entry.id.to_string(),
            entry_title: entry.title.clone(),
            entry_type: entry.entry_type,
            entry_status: entry.status,
            source_tool: source_tool.to_string(),
            operation: CodexDragOperation::LinkToOtherTool,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            entry_summary: Some(entry.content.chars().take(100).collect()),
        }
    }

    /// Serialize drag data for AppState
    pub fn serialize(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("Failed to serialize codex drag data: {}", e))
    }

    /// Deserialize drag data from AppState
    pub fn deserialize(data: &str) -> Result<Self, String> {
        serde_json::from_str(data)
            .map_err(|e| format!("Failed to deserialize codex drag data: {}", e))
    }

    /// Get display name for the drag operation
    pub fn operation_display_name(&self) -> &'static str {
        match self.operation {
            CodexDragOperation::MoveWithinCodex => "Move",
            CodexDragOperation::MoveToOtherTool => "Move to Tool",
            CodexDragOperation::CopyToOtherTool => "Copy to Tool",
            CodexDragOperation::LinkToOtherTool => "Link to Tool",
        }
    }

    /// Get icon for the drag operation
    pub fn operation_icon(&self) -> &'static str {
        match self.operation {
            CodexDragOperation::MoveWithinCodex => "🔄",
            CodexDragOperation::MoveToOtherTool => "➡️",
            CodexDragOperation::CopyToOtherTool => "📋",
            CodexDragOperation::LinkToOtherTool => "🔗",
        }
    }
}

/// Codex drag validation result
#[derive(Debug, Clone)]
pub struct CodexDragValidationResult {
    pub can_drop: bool,
    pub reason: Option<String>,
    pub visual_feedback: CodexDragVisualFeedback,
    pub suggested_operation: Option<CodexDragOperation>,
}

/// Visual feedback for codex drag operations
#[derive(Debug, Clone)]
pub enum CodexDragVisualFeedback {
    Allow,
    Deny,
    IndicateCopy,
    IndicateLink,
    IndicateMove,
    ShowCompatibleTypes(Vec<CodexEntryType>),
}

/// Codex drag handler for managing drag operations
pub struct CodexDragHandler {
    base: Option<*mut CodexToolBase>,
    app_state: Option<*mut AppState>,
}

impl CodexDragHandler {
    /// Create a new codex drag handler
    pub fn new() -> Self {
        Self {
            base: None,
            app_state: None,
        }
    }

    /// Set the codex tool base reference
    pub fn set_base(&mut self, base: *mut CodexToolBase) {
        self.base = Some(base);
    }

    /// Set the app state reference
    pub fn set_app_state(&mut self, app_state: *mut AppState) {
        self.app_state = Some(app_state);
    }

    /// Start a drag operation for a codex entry
    pub fn start_drag(&mut self, entry: &CodexEntry, app_state: &mut AppState) -> Result<(), String> {
        let drag_data = CodexDragData::new_move(entry, "codex");
        let serialized_data = drag_data.serialize()?;

        // Update app state with drag information
        app_state.is_dragging = true;
        app_state.drag_source_window = Some("codex".to_string());
        app_state.drag_data = Some(serialized_data);
        app_state.drag_type = Some("codex_entry".to_string());

        // Show drag start visuals
        self.show_drag_start_visuals(entry);

        Ok(())
    }

    /// Start a drag operation with specific operation type
    pub fn start_drag_with_operation(&mut self, entry: &CodexEntry, operation: CodexDragOperation, app_state: &mut AppState) -> Result<(), String> {
        let drag_data = match operation {
            CodexDragOperation::MoveToOtherTool => CodexDragData::new_move_to_tool(entry, "codex"),
            CodexDragOperation::CopyToOtherTool => CodexDragData::new_copy_to_tool(entry, "codex"),
            CodexDragOperation::LinkToOtherTool => CodexDragData::new_link_to_tool(entry, "codex"),
            CodexDragOperation::MoveWithinCodex => CodexDragData::new_move(entry, "codex"),
        };

        let serialized_data = drag_data.serialize()?;

        // Update app state with drag information
        app_state.is_dragging = true;
        app_state.drag_source_window = Some("codex".to_string());
        app_state.drag_data = Some(serialized_data);
        app_state.drag_type = Some("codex_entry".to_string());

        // Show drag start visuals
        self.show_drag_start_visuals(entry);

        Ok(())
    }

    /// Validate if an entry can be dropped at a target location
    pub fn validate_drop(&self, drag_data: &CodexDragData, target_tool: &str) -> CodexDragValidationResult {
        // Check if dropping back to same tool
        if drag_data.source_tool == target_tool {
            return CodexDragValidationResult {
                can_drop: true,
                reason: None,
                visual_feedback: CodexDragVisualFeedback::IndicateMove,
                suggested_operation: Some(CodexDragOperation::MoveWithinCodex),
            };
        }

        // Validate cross-tool drops
        match target_tool {
            "hierarchy" => {
                // Codex entries can be linked to hierarchy items
                CodexDragValidationResult {
                    can_drop: true,
                    reason: None,
                    visual_feedback: CodexDragVisualFeedback::IndicateLink,
                    suggested_operation: Some(CodexDragOperation::LinkToOtherTool),
                }
            }
            "plot" => {
                // Story summaries and characters can be linked to plot
                if matches!(drag_data.entry_type, CodexEntryType::StorySummary | CodexEntryType::CharacterSheet) {
                    CodexDragValidationResult {
                        can_drop: true,
                        reason: None,
                        visual_feedback: CodexDragVisualFeedback::IndicateLink,
                        suggested_operation: Some(CodexDragOperation::LinkToOtherTool),
                    }
                } else {
                    CodexDragValidationResult {
                        can_drop: false,
                        reason: Some("Only Story Summaries and Character Sheets can be linked to Plot tool".to_string()),
                        visual_feedback: CodexDragVisualFeedback::Deny,
                        suggested_operation: None,
                    }
                }
            }
            "research" => {
                // Any codex entry can be linked to research
                CodexDragValidationResult {
                    can_drop: true,
                    reason: None,
                    visual_feedback: CodexDragVisualFeedback::IndicateLink,
                    suggested_operation: Some(CodexDragOperation::LinkToOtherTool),
                }
            }
            "notes" => {
                // Any codex entry can be copied to notes
                CodexDragValidationResult {
                    can_drop: true,
                    reason: None,
                    visual_feedback: CodexDragVisualFeedback::IndicateCopy,
                    suggested_operation: Some(CodexDragOperation::CopyToOtherTool),
                }
            }
            _ => {
                CodexDragValidationResult {
                    can_drop: false,
                    reason: Some(format!("Cannot drop codex entries into {}", target_tool)),
                    visual_feedback: CodexDragVisualFeedback::Deny,
                    suggested_operation: None,
                }
            }
        }
    }

    /// Handle drop operation within codex
    pub async fn handle_drop_within_codex(&mut self, drag_data: &CodexDragData, target_entry_id: Option<String>) -> Result<(), String> {
        if let Some(base_ptr) = self.base {
            let base = unsafe { &mut *base_ptr };

            // Find the entry being dragged
            if let Some(entry) = base.ui_state.selected_entry.as_ref() {
                match drag_data.operation {
                    CodexDragOperation::MoveWithinCodex => {
                        // Update entry's sort order or parent relationship
                        log::info!("Moving codex entry {} within codex", entry.title);

                        // Notify about the action
                        if let Some(callback) = &base.event_callback {
                            callback(CodexEvent::ActionPerformed(CodexAction::Edit, vec![entry.id]))
                                .map_err(|e| format!("Failed to notify event callback: {}", e))?;
                        }
                    }
                    _ => {
                        return Err("Invalid operation for within-codex drop".to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle drop operation to another tool
    pub async fn handle_drop_to_other_tool(&mut self, drag_data: &CodexDragData, target_tool: &str) -> Result<(), String> {
        log::info!("Handling drop to other tool: {} -> {}", drag_data.entry_title, target_tool);

        match drag_data.operation {
            CodexDragOperation::MoveToOtherTool => {
                // Move entry to target tool (entry would be removed from codex)
                log::info!("Moving entry to tool: {}", target_tool);

                // This would involve:
                // 1. Creating a reference/representation in the target tool
                // 2. Optionally removing the original entry
                // 3. Updating cross-tool relationships

            }
            CodexDragOperation::CopyToOtherTool => {
                // Copy entry to target tool (entry remains in codex)
                log::info!("Copying entry to tool: {}", target_tool);

                // This would involve:
                // 1. Creating a copy in the target tool
                // 2. Maintaining the original in codex
                // 3. Establishing copy relationship

            }
            CodexDragOperation::LinkToOtherTool => {
                // Create a link/relationship to target tool
                log::info!("Linking entry to tool: {}", target_tool);

                // This would involve:
                // 1. Creating a cross-tool reference
                // 2. Establishing relationship metadata
                // 3. Enabling navigation between tools

            }
            _ => {}
        }

        Ok(())
    }

    /// Complete a drag operation
    pub fn complete_drag(&mut self, app_state: &mut AppState) -> Result<(), String> {
        // Clear drag state
        app_state.is_dragging = false;
        app_state.drag_source_window = None;
        app_state.drag_data = None;
        app_state.drag_type = None;

        // Hide drag visuals
        self.hide_drag_visuals();

        Ok(())
    }

    /// Cancel a drag operation
    pub fn cancel_drag(&mut self, app_state: &mut AppState) -> Result<(), String> {
        // Clear drag state
        app_state.is_dragging = false;
        app_state.drag_source_window = None;
        app_state.drag_data = None;
        app_state.drag_type = None;

        // Hide drag visuals
        self.hide_drag_visuals();

        Ok(())
    }

    /// Show visual feedback for drag start
    fn show_drag_start_visuals(&self, entry: &CodexEntry) {
        // TODO: Implement visual feedback (ghost element, etc.)
        log::debug!("Starting drag for codex entry: {}", entry.title);
    }

    /// Hide drag visuals
    fn hide_drag_visuals(&self) {
        // TODO: Hide any visual feedback elements
        log::debug!("Hiding codex drag visuals");
    }

    /// Update drag position feedback
    pub fn update_drag_feedback(&self, validation: &CodexDragValidationResult) {
        match validation.visual_feedback {
            CodexDragVisualFeedback::Allow => log::debug!("Drag allowed"),
            CodexDragVisualFeedback::Deny => log::debug!("Drag denied: {:?}", validation.reason),
            CodexDragVisualFeedback::IndicateCopy => log::debug!("Suggesting copy operation"),
            CodexDragVisualFeedback::IndicateLink => log::debug!("Suggesting link operation"),
            CodexDragVisualFeedback::IndicateMove => log::debug!("Suggesting move operation"),
            CodexDragVisualFeedback::ShowCompatibleTypes(types) => {
                log::debug!("Compatible types: {:?}", types.iter().map(|t| t.display_name()).collect::<Vec<_>>());
            }
        }
    }

    /// Get available drag operations for an entry
    pub fn get_available_operations(&self, entry: &CodexEntry) -> Vec<CodexDragOperation> {
        let mut operations = vec![CodexDragOperation::MoveWithinCodex];

        // All entries can be copied to notes
        operations.push(CodexDragOperation::CopyToOtherTool);

        // All entries can be linked to research
        operations.push(CodexDragOperation::LinkToOtherTool);

        // Story summaries and characters can be linked to plot
        if matches!(entry.entry_type, CodexEntryType::StorySummary | CodexEntryType::CharacterSheet) {
            operations.push(CodexDragOperation::MoveToOtherTool);
        }

        operations
    }

    /// Handle cross-tool drag detection
    pub fn handle_cross_tool_drag(&self, app_state: &AppState) -> Option<CodexDragData> {
        if app_state.is_dragging && app_state.drag_type.as_deref() == Some("codex_entry") {
            if let Some(drag_data) = &app_state.drag_data {
                return CodexDragData::deserialize(drag_data).ok();
            }
        }
        None
    }

    /// Check if we're currently dragging a codex entry
    pub fn is_dragging_codex_entry(&self, app_state: &AppState) -> bool {
        app_state.is_dragging
            && app_state.drag_type.as_deref() == Some("codex_entry")
            && app_state.drag_source_window.as_deref() == Some("codex")
    }
}

/// Extension trait for CodexEntry to support drag operations
pub trait CodexDragSupport {
    /// Get a summary for drag preview
    fn get_drag_summary(&self) -> String;

    /// Check if entry can be dragged to a specific tool
    fn can_drag_to_tool(&self, target_tool: &str) -> bool;

    /// Get recommended drag operation for target tool
    fn get_recommended_operation(&self, target_tool: &str) -> Option<CodexDragOperation>;
}

impl CodexDragSupport for CodexEntry {
    fn get_drag_summary(&self) -> String {
        if self.content.len() > 150 {
            format!("{}...", &self.content[..147])
        } else {
            self.content.clone()
        }
    }

    fn can_drag_to_tool(&self, target_tool: &str) -> bool {
        match target_tool {
            "codex" => true, // Can always move within codex
            "hierarchy" => true, // Can link to hierarchy
            "plot" => matches!(self.entry_type, CodexEntryType::StorySummary | CodexEntryType::CharacterSheet),
            "research" => true, // Can link to research
            "notes" => true, // Can copy to notes
            _ => false,
        }
    }

    fn get_recommended_operation(&self, target_tool: &str) -> Option<CodexDragOperation> {
        match target_tool {
            "codex" => Some(CodexDragOperation::MoveWithinCodex),
            "hierarchy" => Some(CodexDragOperation::LinkToOtherTool),
            "plot" => {
                if matches!(self.entry_type, CodexEntryType::StorySummary | CodexEntryType::CharacterSheet) {
                    Some(CodexDragOperation::LinkToOtherTool)
                } else {
                    None
                }
            }
            "research" => Some(CodexDragOperation::LinkToOtherTool),
            "notes" => Some(CodexDragOperation::CopyToOtherTool),
            _ => None,
        }
    }
}

/// Global codex drag handler instance
lazy_static! {
    pub static ref GLOBAL_CODEX_DRAG_HANDLER: std::sync::Mutex<CodexDragHandler> =
        std::sync::Mutex::new(CodexDragHandler::new());
}

/// Get a reference to the global codex drag handler
pub fn get_codex_drag_handler() -> std::sync::MutexGuard<'static, CodexDragHandler> {
    GLOBAL_CODEX_DRAG_HANDLER.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_drag_data_creation() {
        let entry = CodexEntry::new(
            Uuid::new_v4(),
            CodexEntryType::CharacterSheet,
            "Test Character".to_string(),
            "A test character description.".to_string(),
        );

        let drag_data = CodexDragData::new_move(&entry, "codex");

        assert_eq!(drag_data.entry_title, "Test Character");
        assert_eq!(drag_data.entry_type, CodexEntryType::CharacterSheet);
        assert_eq!(drag_data.source_tool, "codex");
        assert!(drag_data.entry_summary.is_some());
    }

    #[test]
    fn test_drag_validation() {
        let handler = CodexDragHandler::new();

        let drag_data = CodexDragData {
            entry_id: "test".to_string(),
            entry_title: "Test".to_string(),
            entry_type: CodexEntryType::StorySummary,
            entry_status: CodexStatus::Draft,
            source_tool: "codex".to_string(),
            operation: CodexDragOperation::MoveToOtherTool,
            timestamp: 0,
            entry_summary: None,
        };

        let validation = handler.validate_drop(&drag_data, "plot");
        assert!(validation.can_drop);
        assert_eq!(validation.suggested_operation, Some(CodexDragOperation::LinkToOtherTool));
    }

    #[test]
    fn test_operation_icons() {
        let drag_data = CodexDragData {
            entry_id: "test".to_string(),
            entry_title: "Test".to_string(),
            entry_type: CodexEntryType::StorySummary,
            entry_status: CodexStatus::Draft,
            source_tool: "codex".to_string(),
            operation: CodexDragOperation::CopyToOtherTool,
            timestamp: 0,
            entry_summary: None,
        };

        assert_eq!(drag_data.operation_icon(), "📋");
        assert_eq!(drag_data.operation_display_name(), "Copy to Tool");
    }
}
