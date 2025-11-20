//! UI State Management
//!
//! Central state management for the application's UI components.
//! Provides type definitions that are referenced throughout the codebase.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Hierarchy item for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyItem {
    pub id: String,
    pub title: String,
    pub level: u32, // Changed from i32 to u32 to match search module
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    // Added missing fields that are referenced in error messages
    pub content: String,
    pub word_count: u32,
    pub last_modified: u64,
    pub project_id: String,
    pub position: u32,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl HierarchyItem {
    pub fn new(id: String, title: String, level: u32, parent_id: Option<String>) -> Self {
        Self {
            id,
            title,
            level,
            parent_id,
            children: Vec::new(),
            content: String::new(),
            word_count: 0,
            last_modified: 0,
            project_id: String::new(),
            position: 0,
            metadata: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Create a new hierarchy item with all fields
    pub fn new_complete(
        id: String,
        title: String,
        level: u32,
        parent_id: Option<String>,
        project_id: String,
        position: u32,
    ) -> Self {
        Self {
            id,
            title,
            level,
            parent_id,
            children: Vec::new(),
            content: String::new(),
            word_count: 0,
            last_modified: 0,
            project_id,
            position,
            metadata: None,
            created_at: None,
            updated_at: None,
        }
    }
}

/// Tool window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolWindowConfig {
    pub display_name: &'static str,
    pub icon: &'static str,
    pub min_width: f32,
    pub min_height: f32,
    pub resizable: bool,
}

/// Tool window types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolWindowType {
    Hierarchy,
    Codex,
    Plot,
    Analysis,
    Notes,
    Research,
    Brainstorming,
}

/// Window position and size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Window persistence state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowPersistState {
    Normal,
    Minimized,
    Maximized,
    Hidden,
}

/// Window metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowMetadata {
    pub position: WindowPosition,
    pub state: WindowPersistState,
    pub z_index: i32,
    pub config: HashMap<String, serde_json::Value>,
}

/// Tool windows state management
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolWindowsState {
    pub window_positions: HashMap<String, (i32, i32, i32, i32)>, // (x, y, width, height)
    pub open_windows: HashMap<String, bool>,
    pub window_configs: HashMap<String, serde_json::Value>,
}

impl ToolWindowsState {
    pub fn get_window_metadata(&self, _window_name: &str) -> Option<&WindowMetadata> {
        // Placeholder implementation
        None
    }

    pub fn get_window_metadata_mut(&mut self, _window_name: &str) -> Option<&mut WindowMetadata> {
        // Placeholder implementation
        None
    }

    pub fn set_window_open(&mut self, window_name: &str, is_open: bool) {
        self.open_windows.insert(window_name.to_string(), is_open);
    }

    pub fn set_window_position(
        &mut self,
        _window_name: &str,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
    ) {
        // Placeholder implementation
    }

    pub fn get_window_position(&self, _window_name: &str) -> Option<WindowPosition> {
        // Placeholder implementation
        None
    }

    pub fn set_window_state(&mut self, _window_name: &str, _state: WindowPersistState) {
        // Placeholder implementation
    }

    pub fn get_window_state(&self, _window_name: &str) -> Option<WindowPersistState> {
        // Placeholder implementation
        None
    }

    pub fn set_window_z_index(&mut self, _window_name: &str, _z_index: i32) {
        // Placeholder implementation
    }

    pub fn get_window_z_index(&self, _window_name: &str) -> Option<i32> {
        // Placeholder implementation
        None
    }

    pub fn set_window_config(
        &mut self,
        _window_name: &str,
        _key: String,
        _value: serde_json::Value,
    ) {
        // Placeholder implementation
    }

    pub fn get_window_config(&self, _window_name: &str, _key: &str) -> Option<&serde_json::Value> {
        // Placeholder implementation
        None
    }

    pub fn get_open_windows(&self) -> Vec<&str> {
        self.open_windows
            .iter()
            .filter(|(_, &is_open)| is_open)
            .map(|(name, _)| name.as_str())
            .collect()
    }

    pub fn any_windows_open(&self) -> bool {
        self.open_windows.values().any(|&is_open| is_open)
    }

    pub fn close_all_windows(&mut self) {
        for is_open in self.open_windows.values_mut() {
            *is_open = false;
        }
    }

    pub fn open_all_windows(&mut self) {
        // This would need to know about all possible windows
    }

    pub fn validate_positions(&mut self) -> bool {
        // Placeholder implementation
        true
    }

    pub fn migrate_from_legacy(&mut self) {
        // Placeholder implementation
    }

    pub fn export_config(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    pub fn import_config(&mut self, config_json: &str) -> Result<(), String> {
        let imported: ToolWindowsState =
            serde_json::from_str(config_json).map_err(|e| e.to_string())?;
        *self = imported;
        Ok(())
    }
}

/// Service status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String,
    pub uptime: u64,
    pub memory_usage: u64,
    pub error_count: u64,
}

/// UI message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiMessage {
    SettingsChanged {
        key: String,
        value: String,
    },
    FileOperation {
        operation: FileOperationType,
        path: Option<String>,
    },
    WritingToolUpdate {
        tool_type: crate::ui::tools::ToolType,
        data: String,
    },
    ThemeChanged {
        theme: Theme,
    },
}

/// File operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperationType {
    New,
    Open,
    Save,
    SaveAs,
    Export,
}

/// Theme types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Custom(String),
}

/// Document model for UI
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentModel {
    pub content: String,
    #[serde(skip)]
    pub cursor_position: usize, // Transient state
    #[serde(skip)]
    pub selection_start: Option<usize>, // Transient state
    #[serde(skip)]
    pub selection_end: Option<usize>, // Transient state
}

impl DocumentModel {
    pub fn needs_save(&self) -> bool {
        // Placeholder implementation
        false
    }
}

/// Application state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    pub is_dragging: bool,
    pub drag_source_window: Option<String>,
    pub drag_data: Option<String>,
    pub drag_type: Option<String>,
    pub drag_over_main: bool,
}

/// Window configuration export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfigExport {
    pub window_positions: HashMap<String, WindowPosition>,
    pub window_states: HashMap<String, WindowPersistState>,
    pub z_indices: HashMap<String, i32>,
}

/// UI State Manager
#[derive(Debug, Clone, Default)]
pub struct UiStateManager {
    pub data: ToolWindowsState,
    pub service_registry: Option<crate::services::ServiceRegistry>,
    pub validation_errors: Vec<String>,
    pub current_user: String,
}

impl UiStateManager {
    pub fn update_settings(&mut self, _key: String, _value: String) {
        // Placeholder implementation
    }

    pub fn load_from_file(&mut self) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub async fn auto_save(&self) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub fn load_extended_from_file(&mut self) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub fn set_service_registry(&mut self, service_registry: crate::services::ServiceRegistry) {
        self.service_registry = Some(service_registry);
    }

    pub fn validate_state(&mut self) -> Vec<String> {
        // Placeholder implementation
        Vec::new()
    }

    pub async fn save_document_via_service(&mut self, _file_path: String) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub async fn load_document_via_service(&mut self, _file_path: String) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub fn update_settings_via_service(
        &mut self,
        _key: String,
        _value: String,
    ) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub async fn sync_writing_tools_data(&mut self) -> Result<(), String> {
        // Placeholder implementation
        Ok(())
    }

    pub fn get_service_status(&self) -> ServiceStatus {
        ServiceStatus {
            name: "UI State Manager".to_string(),
            status: "active".to_string(),
            uptime: 0,
            memory_usage: 0,
            error_count: 0,
        }
    }
}

/// Extended state for complex scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedState {
    pub tool_windows: ToolWindowsState,
    pub app_state: AppState,
    pub document_model: DocumentModel,
    pub service_status: HashMap<String, ServiceStatus>,
}

/// UUID wrapper for type safety
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UuidWrapper([u8; 16]);

impl UuidWrapper {
    pub fn new_v4() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];
        rng.fill(&mut bytes);
        // Set version and variant bits
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Self(bytes)
    }
}

impl std::fmt::Display for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, byte) in self.0.iter().enumerate() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                write!(f, "-")?;
            }
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
