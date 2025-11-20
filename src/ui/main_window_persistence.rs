//! Main Window State Persistence for Herding Cats Rust
//!
//! Provides persistence for the main application window including
//! position, size, maximized state, and visibility across application sessions.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// Main window state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainWindowState {
    pub window_id: String,
    pub title: String,
    pub position: WindowPosition,
    pub size: WindowSize,
    pub visibility: WindowVisibility,
    pub ui_state: MainWindowUIState,
    pub metadata: WindowMetadata,
    pub last_saved: SystemTime,
}

/// Window position on screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub z_order: i32,
}

/// Window size dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
    pub min_width: i32,
    pub min_height: i32,
}

/// Window visibility and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowVisibility {
    pub is_visible: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
}

/// UI state for main window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainWindowUIState {
    pub show_menu_bar: bool,
    pub show_tool_palette: bool,
    pub show_properties: bool,
    pub show_hierarchy: bool,
    pub show_codex: bool,
    pub active_panels: i32,
    pub document_title: String,
    pub is_editing: bool,
    pub is_maximized: bool,
}

/// Window metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowMetadata {
    pub window_type: String,
    pub application_version: String,
    pub platform: String,
    pub monitor_info: MonitorInfo,
}

/// Monitor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub primary_monitor: bool,
    pub monitor_bounds: (i32, i32, i32, i32), // (x, y, width, height)
}

/// Main window persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainWindowPersistenceConfig {
    pub enable_persistence: bool,
    pub auto_save_on_close: bool,
    pub auto_save_on_resize: bool,
    pub save_ui_state: bool,
    pub max_saved_states: usize,
    pub backup_before_save: bool,
}

impl Default for MainWindowPersistenceConfig {
    fn default() -> Self {
        Self {
            enable_persistence: true,
            auto_save_on_close: true,
            auto_save_on_resize: true,
            save_ui_state: true,
            max_saved_states: 5,
            backup_before_save: true,
        }
    }
}

/// Main window persistence manager
#[derive(Debug)]
pub struct MainWindowPersistenceManager {
    config: MainWindowPersistenceConfig,
    project_path: PathBuf,
    current_state: Option<MainWindowState>,
}

impl MainWindowPersistenceManager {
    /// Create a new main window persistence manager
    pub fn new(config: MainWindowPersistenceConfig, project_path: PathBuf) -> Self {
        Self {
            config,
            project_path,
            current_state: None,
        }
    }

    /// Create with default configuration
    pub fn new_with_defaults(project_path: PathBuf) -> Self {
        Self::new(MainWindowPersistenceConfig::default(), project_path)
    }

    /// Load main window state from file
    pub fn load_main_window_state(
        &mut self,
    ) -> Result<Option<MainWindowState>, MainWindowPersistenceError> {
        if !self.config.enable_persistence {
            return Ok(None);
        }

        let state_file = self.get_main_window_state_file();

        if !state_file.exists() {
            log::info!("📄 No main window state file found, using defaults");
            return Ok(None);
        }

        match fs::read_to_string(&state_file) {
            Ok(content) => match serde_json::from_str::<MainWindowState>(&content) {
                Ok(state) => {
                    log::info!(
                        "💾 Loaded main window state: {} at ({}, {}) size {}x{}",
                        state.title,
                        state.position.x,
                        state.position.y,
                        state.size.width,
                        state.size.height
                    );
                    self.current_state = Some(state.clone());
                    Ok(Some(state))
                }
                Err(e) => {
                    log::error!("❌ Failed to parse main window state: {}", e);
                    Err(MainWindowPersistenceError::InvalidState(format!(
                        "Parse error: {}",
                        e
                    )))
                }
            },
            Err(e) => {
                log::error!("❌ Failed to read main window state file: {}", e);
                Err(MainWindowPersistenceError::IoError(e))
            }
        }
    }

    /// Save main window state to file
    pub fn save_main_window_state(
        &self,
        state: &MainWindowState,
    ) -> Result<(), MainWindowPersistenceError> {
        if !self.config.enable_persistence {
            return Ok(());
        }

        let state_file = self.get_main_window_state_file();

        // Ensure directory exists
        if let Some(parent) = state_file.parent() {
            fs::create_dir_all(parent)?;
        }

        // Backup existing file if enabled
        if self.config.backup_before_save && state_file.exists() {
            let backup_file = state_file.with_extension("json.backup");
            if let Err(e) = fs::copy(&state_file, &backup_file) {
                log::warn!("⚠️ Failed to backup main window state: {}", e);
            }
        }

        let content = serde_json::to_string_pretty(state)
            .map_err(|e| MainWindowPersistenceError::SerializationError(e.to_string()))?;

        fs::write(&state_file, content).map_err(MainWindowPersistenceError::IoError)?;

        log::info!(
            "💾 Saved main window state: {} at ({}, {}) size {}x{}",
            state.title,
            state.position.x,
            state.position.y,
            state.size.width,
            state.size.height
        );

        Ok(())
    }

    /// Get current window state
    pub fn get_current_state(&self) -> Option<&MainWindowState> {
        self.current_state.as_ref()
    }

    /// Create main window state from current application state
    pub fn create_window_state(
        &self,
        window_title: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        is_maximized: bool,
        is_minimized: bool,
        ui_state: MainWindowUIState,
    ) -> MainWindowState {
        let monitor_info = self.get_current_monitor_info();

        MainWindowState {
            window_id: "main_window".to_string(),
            title: window_title,
            position: WindowPosition { x, y, z_order: 0 },
            size: WindowSize {
                width,
                height,
                min_width: 800,
                min_height: 600,
            },
            visibility: WindowVisibility {
                is_visible: true,
                is_minimized,
                is_maximized,
                is_fullscreen: false,
            },
            ui_state,
            metadata: WindowMetadata {
                window_type: "main_application".to_string(),
                application_version: "2.0.0".to_string(),
                platform: std::env::consts::OS.to_string(),
                monitor_info,
            },
            last_saved: SystemTime::now(),
        }
    }

    /// Get the main window state file path
    fn get_main_window_state_file(&self) -> PathBuf {
        self.project_path
            .join("window_persistence")
            .join("main_window_state.json")
    }

    /// Get current monitor information
    fn get_current_monitor_info(&self) -> MonitorInfo {
        // This is a simplified implementation - in a real app you'd get actual monitor info
        MonitorInfo {
            primary_monitor: true,
            monitor_bounds: (0, 0, 1920, 1080), // Default 1080p monitor
        }
    }

    /// Clean up old saved states
    pub fn cleanup_old_states(&self) -> Result<u32, MainWindowPersistenceError> {
        let persistence_dir = self.get_persistence_dir();

        if !persistence_dir.exists() {
            return Ok(0);
        }

        // This is a placeholder implementation for the main window state
        // In a full implementation, you might have multiple saved states
        Ok(0)
    }

    /// Get persistence directory
    fn get_persistence_dir(&self) -> PathBuf {
        self.project_path.join("window_persistence")
    }

    /// Get persistence statistics
    pub fn get_persistence_stats(&self) -> MainWindowPersistenceStats {
        MainWindowPersistenceStats {
            state_file_exists: self.get_main_window_state_file().exists(),
            current_state_loaded: self.current_state.is_some(),
            last_saved: self.current_state.as_ref().map(|s| s.last_saved),
            file_size_bytes: self
                .get_main_window_state_file()
                .exists()
                .then(|| fs::metadata(self.get_main_window_state_file()).ok())
                .flatten()
                .map(|m| m.len())
                .unwrap_or(0),
        }
    }
}

/// Main window persistence statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainWindowPersistenceStats {
    pub state_file_exists: bool,
    pub current_state_loaded: bool,
    pub last_saved: Option<SystemTime>,
    pub file_size_bytes: u64,
}

/// Main window persistence errors
#[derive(Debug, thiserror::Error)]
pub enum MainWindowPersistenceError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid window state: {0}")]
    InvalidState(String),

    #[error("File not found: {0}")]
    FileNotFound(String),
}

/// Result type for main window persistence operations
pub type MainWindowPersistenceResult<T> = Result<T, MainWindowPersistenceError>;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_main_window_state_creation() {
        let ui_state = MainWindowUIState {
            show_menu_bar: true,
            show_tool_palette: true,
            show_properties: false,
            show_hierarchy: false,
            show_codex: false,
            active_panels: 1,
            document_title: "Test Document".to_string(),
            is_editing: true,
            is_maximized: false,
        };

        let state = MainWindowState {
            window_id: "test_window".to_string(),
            title: "Test Window".to_string(),
            position: WindowPosition {
                x: 100,
                y: 100,
                z_order: 0,
            },
            size: WindowSize {
                width: 800,
                height: 600,
                min_width: 800,
                min_height: 600,
            },
            visibility: WindowVisibility {
                is_visible: true,
                is_minimized: false,
                is_maximized: false,
                is_fullscreen: false,
            },
            ui_state,
            metadata: WindowMetadata {
                window_type: "test".to_string(),
                application_version: "1.0.0".to_string(),
                platform: "test".to_string(),
                monitor_info: MonitorInfo {
                    primary_monitor: true,
                    monitor_bounds: (0, 0, 1920, 1080),
                },
            },
            last_saved: SystemTime::now(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: MainWindowState = serde_json::from_str(&serialized).unwrap();

        assert_eq!(state.window_id, deserialized.window_id);
        assert_eq!(state.position.x, deserialized.position.x);
        assert_eq!(state.size.width, deserialized.size.width);
    }

    #[test]
    fn test_main_window_persistence_manager() {
        let temp_dir = tempdir().unwrap();
        let config = MainWindowPersistenceConfig::default();
        let manager =
            MainWindowPersistenceManager::new(config.clone(), temp_dir.path().to_path_buf());

        let ui_state = MainWindowUIState {
            show_menu_bar: true,
            show_tool_palette: true,
            show_properties: false,
            show_hierarchy: false,
            show_codex: false,
            active_panels: 1,
            document_title: "Test Document".to_string(),
            is_editing: true,
            is_maximized: false,
        };

        let state = manager.create_window_state(
            "Test Window".to_string(),
            100,
            100,
            800,
            600,
            false,
            false,
            ui_state,
        );

        // Test save and load
        assert!(manager.save_main_window_state(&state).is_ok());
        let mut manager = MainWindowPersistenceManager::new(config, temp_dir.path().to_path_buf());
        assert!(manager.load_main_window_state().is_ok());
    }
}
