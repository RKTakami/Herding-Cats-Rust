//! Window State Persistence System for Herding Cats Rust
//!
//! Provides comprehensive persistence for dynamic window states including
//! position, size, visibility, z-order, and layout configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use uuid::Uuid;

/// Window state persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPersistenceConfig {
    pub auto_save_on_change: bool,
    pub save_interval_seconds: u64,
    pub max_saved_states: usize,
    pub include_window_content: bool,
    pub compress_saved_states: bool,
    pub backup_before_save: bool,
    pub restore_on_startup: bool,
}

impl Default for WindowPersistenceConfig {
    fn default() -> Self {
        Self {
            auto_save_on_change: true,
            save_interval_seconds: 30,
            max_saved_states: 5,
            include_window_content: false,
            compress_saved_states: false,
            backup_before_save: true,
            restore_on_startup: true,
        }
    }
}

/// Complete window state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub id: String,
    pub title: String,
    pub window_type: WindowType,
    pub position: WindowPosition,
    pub size: WindowSize,
    pub visibility: WindowVisibility,
    pub state: WindowUIState,
    pub z_index: i32,
    pub layout_info: WindowLayoutInfo,
    pub metadata: WindowMetadata,
    pub last_modified: SystemTime,
}

/// Window position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub screen: Option<String>, // For multi-monitor setups
}

/// Window size information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
    pub min_width: Option<i32>,
    pub min_height: Option<i32>,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
}

/// Window visibility state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowVisibility {
    pub is_visible: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
}

/// Window UI state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowUIState {
    pub is_dragging: bool,
    pub is_resizing: bool,
    pub resize_handle: Option<ResizeHandle>,
    pub focus_state: FocusState,
}

/// Resize handle information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResizeHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

/// Focus state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FocusState {
    Focused,
    Unfocused,
    Active,
}

/// Window layout information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowLayoutInfo {
    pub layout_group: Option<String>,
    pub layout_constraints: LayoutConstraints,
    pub docking_info: Option<DockingInfo>,
}

/// Layout constraints for window positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConstraints {
    pub snap_to_grid: bool,
    pub grid_size: Option<(i32, i32)>,
    pub maintain_aspect_ratio: bool,
    pub respect_screen_bounds: bool,
}

/// Docking information for window management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingInfo {
    pub is_docked: bool,
    pub dock_position: DockPosition,
    pub dock_size: Option<(i32, i32)>,
    pub dock_group: Option<String>,
}

/// Dock positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
    Floating,
}

/// Window metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowMetadata {
    pub created_at: SystemTime,
    pub first_opened: SystemTime,
    pub open_count: u32,
    pub total_open_time_seconds: u64,
    pub user_preferences: HashMap<String, serde_json::Value>,
}

/// Window types supported by the persistence system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WindowType {
    Hierarchy,
    Codex,
    Plot,
    Analysis,
    Notes,
    Research,
    Settings,
    Main,
    Custom(String),
}

/// Complete window layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowLayout {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub windows: HashMap<String, WindowState>,
    pub layout_settings: LayoutSettings,
    pub created_at: SystemTime,
    pub last_used: SystemTime,
    pub usage_count: u32,
}

/// Layout settings for the entire window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    pub auto_arrange: bool,
    pub remember_positions: bool,
    pub restore_on_startup: bool,
    pub default_layout: Option<String>,
    pub workspace_name: Option<String>,
}

/// Window state manager for persistence operations
#[derive(Debug)]
pub struct WindowStateManager {
    config: WindowPersistenceConfig,
    project_path: PathBuf,
    current_layout: Option<WindowLayout>,
    saved_layouts: HashMap<Uuid, WindowLayout>,
    auto_save_timer: Option<std::time::Instant>,
}

/// Window state persistence error types
#[derive(Debug, thiserror::Error)]
pub enum WindowPersistenceError {
    #[error("Layout not found: {0}")]
    LayoutNotFound(String),

    #[error("Invalid window state: {0}")]
    InvalidWindowState(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl From<String> for WindowPersistenceError {
    fn from(s: String) -> Self {
        WindowPersistenceError::Configuration(s)
    }
}

impl From<&str> for WindowPersistenceError {
    fn from(s: &str) -> Self {
        WindowPersistenceError::Configuration(s.to_string())
    }
}

/// Result type for window persistence operations
pub type WindowPersistenceResult<T> = Result<T, WindowPersistenceError>;

impl WindowStateManager {
    /// Create a new window state manager
    pub fn new(config: WindowPersistenceConfig, project_path: PathBuf) -> Self {
        Self {
            config,
            project_path,
            current_layout: None,
            saved_layouts: HashMap::new(),
            auto_save_timer: None,
        }
    }

    /// Initialize the window state manager
    pub async fn initialize(&mut self) -> WindowPersistenceResult<()> {
        // Ensure persistence directory exists
        let persistence_dir = self.get_persistence_dir();
        fs::create_dir_all(&persistence_dir)?;

        // Load saved layouts
        self.load_saved_layouts().await?;

        // Load last used layout if configured
        if self.config.restore_on_startup {
            self.load_last_layout().await?;
        }

        // Initialize auto-save timer
        if self.config.auto_save_on_change {
            self.auto_save_timer = Some(std::time::Instant::now());
        }

        Ok(())
    }

    /// Save the current window layout
    pub async fn save_current_layout(
        &mut self,
        name: Option<String>,
    ) -> WindowPersistenceResult<()> {
        if let Some(layout) = &self.current_layout {
            let layout_to_save = if let Some(layout_name) = name {
                // Create a named layout
                WindowLayout {
                    id: Uuid::new_v4(),
                    name: layout_name,
                    description: None,
                    windows: layout.windows.clone(),
                    layout_settings: layout.layout_settings.clone(),
                    created_at: SystemTime::now(),
                    last_used: SystemTime::now(),
                    usage_count: 1,
                }
            } else {
                // Update the current layout
                let mut updated_layout = layout.clone();
                updated_layout.last_used = SystemTime::now();
                updated_layout.usage_count += 1;
                updated_layout
            };

            // Save to file
            self.save_layout_to_file(&layout_to_save).await?;

            // Update saved layouts
            self.saved_layouts
                .insert(layout_to_save.id, layout_to_save.clone());

            // Set as current layout
            self.current_layout = Some(layout_to_save);
        }

        Ok(())
    }

    /// Load a specific layout by ID
    pub async fn load_layout(&mut self, layout_id: Uuid) -> WindowPersistenceResult<()> {
        if let Some(layout) = self.saved_layouts.get(&layout_id) {
            self.current_layout = Some(layout.clone());
            self.update_layout_usage(layout_id).await?;
        } else {
            return Err(WindowPersistenceError::LayoutNotFound(format!(
                "Layout with ID {} not found",
                layout_id
            )));
        }
        Ok(())
    }

    /// Load the last used layout
    pub async fn load_last_layout(&mut self) -> WindowPersistenceResult<()> {
        // Find the most recently used layout
        let mut last_layout = None;
        let mut last_used_time = SystemTime::UNIX_EPOCH;

        for layout in self.saved_layouts.values() {
            if layout.last_used > last_used_time {
                last_used_time = layout.last_used;
                last_layout = Some(layout);
            }
        }

        if let Some(layout) = last_layout {
            self.current_layout = Some(layout.clone());
        }

        Ok(())
    }

    /// Update window state in the current layout
    pub fn update_window_state(
        &mut self,
        window_id: &str,
        window_state: WindowState,
    ) -> WindowPersistenceResult<()> {
        if let Some(layout) = &mut self.current_layout {
            layout.windows.insert(window_id.to_string(), window_state);

            // Trigger auto-save if enabled
            if self.config.auto_save_on_change {
                self.schedule_auto_save();
            }
        }

        Ok(())
    }

    /// Get window state for a specific window
    pub fn get_window_state(&self, window_id: &str) -> Option<&WindowState> {
        self.current_layout
            .as_ref()
            .and_then(|layout| layout.windows.get(window_id))
    }

    /// Get all window states in current layout
    pub fn get_all_window_states(&self) -> HashMap<String, WindowState> {
        self.current_layout
            .as_ref()
            .map(|layout| layout.windows.clone())
            .unwrap_or_default()
    }

    /// Create a new window layout
    pub fn create_new_layout(
        &mut self,
        name: String,
        description: Option<String>,
    ) -> WindowPersistenceResult<()> {
        let layout = WindowLayout {
            id: Uuid::new_v4(),
            name,
            description,
            windows: HashMap::new(),
            layout_settings: LayoutSettings {
                auto_arrange: false,
                remember_positions: true,
                restore_on_startup: true,
                default_layout: None,
                workspace_name: None,
            },
            created_at: SystemTime::now(),
            last_used: SystemTime::now(),
            usage_count: 0,
        };

        self.current_layout = Some(layout);
        Ok(())
    }

    /// Delete a saved layout
    pub async fn delete_layout(&mut self, layout_id: Uuid) -> WindowPersistenceResult<()> {
        if let Some(layout) = self.saved_layouts.remove(&layout_id) {
            // Delete the layout file
            let layout_file = self.get_layout_file_path(layout_id);
            if layout_file.exists() {
                fs::remove_file(&layout_file)?;
            }

            // If this was the current layout, clear it
            if let Some(current) = &self.current_layout {
                if current.id == layout_id {
                    self.current_layout = None;
                }
            }
        }

        Ok(())
    }

    /// Get all saved layouts
    pub fn get_saved_layouts(&self) -> Vec<&WindowLayout> {
        self.saved_layouts.values().collect()
    }

    /// Export window layout to file
    pub async fn export_layout(
        &self,
        layout_id: Uuid,
        export_path: &Path,
    ) -> WindowPersistenceResult<()> {
        if let Some(layout) = self.saved_layouts.get(&layout_id) {
            let content = serde_json::to_string_pretty(layout)?;
            fs::write(export_path, content)?;
        } else {
            return Err(WindowPersistenceError::LayoutNotFound(format!(
                "Layout with ID {} not found",
                layout_id
            )));
        }

        Ok(())
    }

    /// Import window layout from file
    pub async fn import_layout(&mut self, import_path: &Path) -> WindowPersistenceResult<Uuid> {
        let content = fs::read_to_string(import_path)?;
        let layout: WindowLayout = serde_json::from_str(&content)?;

        // Generate new ID to avoid conflicts
        let new_id = Uuid::new_v4();
        let mut imported_layout = layout;
        imported_layout.id = new_id;
        imported_layout.name = format!("{} (Imported)", imported_layout.name);

        // Save the imported layout
        self.save_layout_to_file(&imported_layout).await?;
        self.saved_layouts.insert(new_id, imported_layout.clone());

        Ok(new_id)
    }

    /// Get persistence statistics
    pub fn get_persistence_stats(&self) -> WindowPersistenceStats {
        WindowPersistenceStats {
            total_saved_layouts: self.saved_layouts.len(),
            current_layout_name: self.current_layout.as_ref().map(|l| l.name.clone()),
            total_windows_in_current: self.current_layout.as_ref().map_or(0, |l| l.windows.len()),
            auto_save_enabled: self.config.auto_save_on_change,
            last_auto_save: self.auto_save_timer.map(|t| t.elapsed()),
        }
    }

    // Private helper methods

    fn get_persistence_dir(&self) -> PathBuf {
        self.project_path.join("window_persistence")
    }

    fn get_layouts_dir(&self) -> PathBuf {
        self.get_persistence_dir().join("layouts")
    }

    fn get_layout_file_path(&self, layout_id: Uuid) -> PathBuf {
        self.get_layouts_dir().join(format!("{}.json", layout_id))
    }

    async fn load_saved_layouts(&mut self) -> WindowPersistenceResult<()> {
        let layouts_dir = self.get_layouts_dir();

        if !layouts_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&layouts_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let layout: WindowLayout = serde_json::from_str(&content)?;
                self.saved_layouts.insert(layout.id, layout);
            }
        }

        Ok(())
    }

    async fn save_layout_to_file(&self, layout: &WindowLayout) -> WindowPersistenceResult<()> {
        let layouts_dir = self.get_layouts_dir();
        fs::create_dir_all(&layouts_dir)?;

        let layout_file = self.get_layout_file_path(layout.id);
        let content = serde_json::to_string_pretty(layout)?;
        fs::write(&layout_file, content)?;

        Ok(())
    }

    async fn update_layout_usage(&mut self, layout_id: Uuid) -> WindowPersistenceResult<()> {
        if let Some(layout) = self.saved_layouts.get_mut(&layout_id).cloned() {
            let mut updated_layout = layout;
            updated_layout.last_used = SystemTime::now();
            updated_layout.usage_count += 1;
            self.save_layout_to_file(&updated_layout).await?;
            self.saved_layouts.insert(layout_id, updated_layout);
        }

        Ok(())
    }

    fn schedule_auto_save(&mut self) {
        if let Some(timer) = &self.auto_save_timer {
            if timer.elapsed().as_secs() >= self.config.save_interval_seconds {
                // Trigger auto-save (in a real implementation, this would be async)
                self.auto_save_timer = Some(std::time::Instant::now());
            }
        }
    }
}

/// Window persistence statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPersistenceStats {
    pub total_saved_layouts: usize,
    pub current_layout_name: Option<String>,
    pub total_windows_in_current: usize,
    pub auto_save_enabled: bool,
    pub last_auto_save: Option<std::time::Duration>,
}

/// Utility functions for window state management
impl WindowState {
    /// Create a default window state
    pub fn new(id: String, title: String, window_type: WindowType) -> Self {
        Self {
            id,
            title,
            window_type,
            position: WindowPosition {
                x: 100,
                y: 100,
                screen: None,
            },
            size: WindowSize {
                width: 400,
                height: 300,
                min_width: Some(200),
                min_height: Some(150),
                max_width: None,
                max_height: None,
            },
            visibility: WindowVisibility {
                is_visible: false,
                is_minimized: false,
                is_maximized: false,
                is_fullscreen: false,
            },
            state: WindowUIState {
                is_dragging: false,
                is_resizing: false,
                resize_handle: None,
                focus_state: FocusState::Unfocused,
            },
            z_index: 0,
            layout_info: WindowLayoutInfo {
                layout_group: None,
                layout_constraints: LayoutConstraints {
                    snap_to_grid: false,
                    grid_size: None,
                    maintain_aspect_ratio: false,
                    respect_screen_bounds: true,
                },
                docking_info: None,
            },
            metadata: WindowMetadata {
                created_at: SystemTime::now(),
                first_opened: SystemTime::now(),
                open_count: 0,
                total_open_time_seconds: 0,
                user_preferences: HashMap::new(),
            },
            last_modified: SystemTime::now(),
        }
    }

    /// Update position and mark as modified
    pub fn update_position(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
        self.last_modified = SystemTime::now();
    }

    /// Update size and mark as modified
    pub fn update_size(&mut self, width: i32, height: i32) {
        self.size.width = width.max(self.size.min_width.unwrap_or(1));
        self.size.height = height.max(self.size.min_height.unwrap_or(1));
        self.last_modified = SystemTime::now();
    }

    /// Update visibility state
    pub fn update_visibility(&mut self, visible: bool, minimized: bool, maximized: bool) {
        self.visibility.is_visible = visible;
        self.visibility.is_minimized = minimized;
        self.visibility.is_maximized = maximized;
        self.last_modified = SystemTime::now();
    }

    /// Record that the window was opened
    pub fn record_open(&mut self) {
        self.metadata.open_count += 1;
        self.metadata.first_opened = SystemTime::now();
        self.last_modified = SystemTime::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_window_state_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let config = WindowPersistenceConfig::default();
        let manager = WindowStateManager::new(config, temp_dir.path().to_path_buf());

        assert_eq!(manager.get_saved_layouts().len(), 0);
    }

    #[test]
    fn test_window_state_creation() {
        let window_state = WindowState::new(
            "test-window".to_string(),
            "Test Window".to_string(),
            WindowType::Hierarchy,
        );

        assert_eq!(window_state.id, "test-window");
        assert_eq!(window_state.title, "Test Window");
        assert_eq!(window_state.window_type, WindowType::Hierarchy);
        assert!(!window_state.visibility.is_visible);
    }

    #[test]
    fn test_window_state_updates() {
        let mut window_state = WindowState::new(
            "test-window".to_string(),
            "Test Window".to_string(),
            WindowType::Hierarchy,
        );

        window_state.update_position(200, 150);
        assert_eq!(window_state.position.x, 200);
        assert_eq!(window_state.position.y, 150);

        window_state.update_size(600, 400);
        assert_eq!(window_state.size.width, 600);
        assert_eq!(window_state.size.height, 400);

        window_state.update_visibility(true, false, false);
        assert!(window_state.visibility.is_visible);
        assert!(!window_state.visibility.is_minimized);
    }
}
