//! Window Management System
//!
//! Manages individual tool window instances with the constraint that only one instance
//! of each tool can be opened. Universal windows have been removed in favor of
//! independent tool windows.

use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::ui::tools::ToolType;

/// Window instance identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u32);

impl std::fmt::Display for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Window state information
#[derive(Debug, Clone)]
pub struct WindowState {
    pub window_id: WindowId,
    pub window_type: WindowType,
    pub is_open: bool,
    pub is_focused: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub z_index: i32,
    pub creation_time: std::time::SystemTime,
}

/// Types of windows supported
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WindowType {
    /// Individual tool windows
    IndividualTool(ToolType),
}

impl WindowType {
    /// Get the display name for the window type
    pub fn display_name(&self) -> String {
        match self {
            WindowType::IndividualTool(tool) => format!("{} Tool", tool.display_name()),
        }
    }

    /// Check if this is an individual tool window
    pub fn is_individual_tool(&self) -> bool {
        matches!(self, WindowType::IndividualTool(_))
    }

    /// Get the tool type for individual tool windows
    pub fn get_tool_type(&self) -> Option<&ToolType> {
        match self {
            WindowType::IndividualTool(tool) => Some(tool),
        }
    }
}

/// Window management error types
#[derive(Debug, thiserror::Error)]
pub enum WindowManagerError {
    #[error("Window with ID {window_id} does not exist")]
    WindowNotFound { window_id: WindowId },

    #[error("Tool {tool:?} is already open in window {existing_window_id}")]
    ToolAlreadyOpen {
        tool: ToolType,
        existing_window_id: WindowId,
    },

    #[error("Maximum number of windows reached ({max_windows})")]
    MaxWindowsReached { max_windows: usize },


    #[error("Window {window_id} is not open")]
    WindowNotOpen { window_id: WindowId },

    #[error("Window {window_id} is already open")]
    WindowAlreadyOpen { window_id: WindowId },
}

/// Window manager configuration
#[derive(Debug, Clone)]
pub struct WindowManagerConfig {
    /// Maximum total windows allowed
    pub max_total_windows: usize,
    /// Auto-focus new windows
    pub auto_focus: bool,
    /// Window spacing for cascading
    pub window_spacing: (i32, i32),
}

impl Default for WindowManagerConfig {
    fn default() -> Self {
        Self {
            max_total_windows: 8, // One for each tool type
            auto_focus: true,
            window_spacing: (30, 30),
        }
    }
}

/// Core window manager implementation
pub struct WindowManager {
    /// Configuration settings
    config: WindowManagerConfig,
    /// All managed windows
    windows: HashMap<WindowId, WindowState>,
    /// Next window ID to assign
    next_window_id: Arc<Mutex<u32>>,
    /// Currently open tools (for individual tool windows)
    open_tools: HashSet<ToolType>,
}

impl WindowManager {
    /// Create a new window manager with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(WindowManagerConfig::default())
    }

    /// Create a new window manager with custom configuration
    pub fn with_config(config: WindowManagerConfig) -> Result<Self> {
        // Validate configuration
        if config.max_total_windows == 0 {
            return Err(WindowManagerError::MaxWindowsReached { max_windows: 0 }.into());
        }

        Ok(Self {
            config,
            windows: HashMap::new(),
            next_window_id: Arc::new(Mutex::new(1)),
            open_tools: HashSet::new(),
        })
    }

    /// Open a new window
    pub fn open_window(&mut self, window_type: WindowType) -> Result<WindowId, WindowManagerError> {
        // Check if tool is already open (for individual tool windows)
        if let WindowType::IndividualTool(ref tool) = window_type {
            if self.open_tools.contains(tool) {
                let existing_window = self
                    .find_open_tool_window(tool)
                    .expect("Tool should have an open window if it's in open_tools");
                return Err(WindowManagerError::ToolAlreadyOpen {
                    tool: *tool,
                    existing_window_id: existing_window,
                });
            }
        }

        // Check maximum windows limit
        if self.windows.len() >= self.config.max_total_windows {
            return Err(WindowManagerError::MaxWindowsReached {
                max_windows: self.config.max_total_windows,
            });
        }


        // Generate new window ID
        let window_id = {
            let mut id_counter = self.next_window_id.lock().unwrap();
            let id = *id_counter;
            *id_counter += 1;
            WindowId(id)
        };

        // Calculate window position (cascade layout)
        let position = self.calculate_window_position();

        // Create new window state
        let window_state = WindowState {
            window_id,
            window_type: window_type.clone(),
            is_open: true,
            is_focused: self.config.auto_focus,
            position,
            size: (1000, 700), // Default size
            z_index: self.windows.len() as i32,
            creation_time: std::time::SystemTime::now(),
        };

        // Add to collections
        self.windows.insert(window_id, window_state);

        // Track tool state
        if let WindowType::IndividualTool(ref tool) = &window_type {
            self.open_tools.insert(*tool);
        }

        // Update focus
        if self.config.auto_focus {
            self.focus_window(window_id)?;
        }

        Ok(window_id)
    }

    /// Close a window
    pub fn close_window(&mut self, window_id: WindowId) -> Result<(), WindowManagerError> {
        let window_state = self
            .windows
            .get(&window_id)
            .ok_or(WindowManagerError::WindowNotFound { window_id })?;

        if !window_state.is_open {
            return Err(WindowManagerError::WindowNotOpen { window_id });
        }

        // Remove from tracking collections
        if let WindowType::IndividualTool(ref tool) = &window_state.window_type {
            self.open_tools.remove(tool);
        }

        // Remove window
        self.windows.remove(&window_id);

        Ok(())
    }

    /// Focus a window
    pub fn focus_window(&mut self, window_id: WindowId) -> Result<(), WindowManagerError> {
        let window_state = self
            .windows
            .get_mut(&window_id)
            .ok_or(WindowManagerError::WindowNotFound { window_id })?;

        if !window_state.is_open {
            return Err(WindowManagerError::WindowNotOpen { window_id });
        }

        // Store window count before modifications
        let window_count = self.windows.len();

        // Set all windows to unfocused first, then focus the target
        for (_, state) in self.windows.iter_mut() {
            if state.window_id == window_id {
                state.is_focused = true;
                state.z_index = window_count as i32; // Bring to front
            } else {
                state.is_focused = false;
            }
        }

        Ok(())
    }

    /// Get information about all open windows
    pub fn get_open_windows(&self) -> Vec<&WindowState> {
        self.windows
            .values()
            .filter(|state| state.is_open)
            .collect()
    }

    /// Get information about all open individual tool windows
    pub fn get_open_tool_windows(&self) -> Vec<&WindowState> {
        self.get_open_windows()
            .into_iter()
            .filter(|state| state.window_type.is_individual_tool())
            .collect()
    }


    /// Check if a tool is currently open
    pub fn is_tool_open(&self, tool: ToolType) -> bool {
        self.open_tools.contains(&tool)
    }

    /// Find the window ID for an open tool
    pub fn find_open_tool_window(&self, tool: &ToolType) -> Option<WindowId> {
        if !self.open_tools.contains(tool) {
            return None;
        }

        self.windows
            .values()
            .find(|state| {
                state.is_open
                    && matches!(&state.window_type, WindowType::IndividualTool(t) if t == tool)
            })
            .map(|state| state.window_id)
    }

    /// Get window state by ID
    pub fn get_window_state(&self, window_id: WindowId) -> Option<&WindowState> {
        self.windows.get(&window_id)
    }

    /// Get mutable window state by ID
    pub fn get_window_state_mut(&mut self, window_id: WindowId) -> Option<&mut WindowState> {
        self.windows.get_mut(&window_id)
    }

    /// Get current usage statistics
    pub fn get_statistics(&self) -> WindowStatistics {
        let open_windows = self.get_open_windows();
        let total_open = open_windows.len();
        let open_tools = open_windows
            .iter()
            .filter(|w| w.window_type.is_individual_tool())
            .count();

        WindowStatistics {
            total_open_windows: total_open,
            open_tools: open_tools,
            max_total_windows: self.config.max_total_windows,
            available_tools: ToolType::all_types().len() - open_tools,
        }
    }

    /// Calculate position for a new window (cascade layout)
    fn calculate_window_position(&self) -> (i32, i32) {
        let base_x = 100;
        let base_y = 100;
        let spacing_x = self.config.window_spacing.0;
        let spacing_y = self.config.window_spacing.1;

        let open_count = self.get_open_windows().len() as i32;

        (
            base_x + (open_count * spacing_x),
            base_y + (open_count * spacing_y),
        )
    }
}

/// Window usage statistics
#[derive(Debug, Clone)]
pub struct WindowStatistics {
    pub total_open_windows: usize,
    pub open_tools: usize,
    pub max_total_windows: usize,
    pub available_tools: usize,
}

/// Global window manager instance
lazy_static! {
    pub static ref GLOBAL_WINDOW_MANAGER: Arc<Mutex<WindowManager>> = {
        Arc::new(Mutex::new(
            WindowManager::new().expect("Failed to create global window manager"),
        ))
    };
}

/// Get a reference to the global window manager
pub fn get_window_manager() -> std::sync::MutexGuard<'static, WindowManager> {
    GLOBAL_WINDOW_MANAGER.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_manager_creation() {
        let wm = WindowManager::new().unwrap();
        assert_eq!(wm.get_open_windows().len(), 0);
        assert_eq!(wm.get_statistics().total_open_windows, 0);
    }

    #[test]
    fn test_open_individual_tool_window() {
        let mut wm = WindowManager::new().unwrap();

        let window_id = wm
            .open_window(WindowType::IndividualTool(ToolType::Hierarchy))
            .unwrap();

        assert!(wm.is_tool_open(ToolType::Hierarchy));
        assert_eq!(
            wm.find_open_tool_window(&ToolType::Hierarchy),
            Some(window_id)
        );
        assert_eq!(wm.get_statistics().open_tools, 1);
    }

    #[test]
    fn test_prevent_duplicate_tool_windows() {
        let mut wm = WindowManager::new().unwrap();

        // Open first tool window
        wm.open_window(WindowType::IndividualTool(ToolType::Hierarchy))
            .unwrap();

        // Try to open the same tool again - should fail
        let result = wm.open_window(WindowType::IndividualTool(ToolType::Hierarchy));
        assert!(result.is_err());
        if let Err(WindowManagerError::ToolAlreadyOpen {
            tool,
            existing_window_id: _,
        }) = result
        {
            assert_eq!(tool, ToolType::Hierarchy);
        }
    }


    #[test]
    fn test_window_focus() {
        let mut wm = WindowManager::new().unwrap();

        let window1 = wm
            .open_window(WindowType::IndividualTool(ToolType::Hierarchy))
            .unwrap();
        let window2 = wm
            .open_window(WindowType::IndividualTool(ToolType::Codex))
            .unwrap();

        // Initially, window2 should be focused (auto-focus)
        assert!(!wm.get_window_state(window1).unwrap().is_focused);
        assert!(wm.get_window_state(window2).unwrap().is_focused);

        // Focus window1
        wm.focus_window(window1).unwrap();

        assert!(wm.get_window_state(window1).unwrap().is_focused);
        assert!(!wm.get_window_state(window2).unwrap().is_focused);
    }

    #[test]
    fn test_close_window() {
        let mut wm = WindowManager::new().unwrap();

        let window_id = wm
            .open_window(WindowType::IndividualTool(ToolType::Hierarchy))
            .unwrap();
        assert!(wm.is_tool_open(ToolType::Hierarchy));

        wm.close_window(window_id).unwrap();
        assert!(!wm.is_tool_open(ToolType::Hierarchy));
        assert!(wm.find_open_tool_window(&ToolType::Hierarchy).is_none());
    }

    #[test]
    fn test_max_windows_limit() {
        let config = WindowManagerConfig {
            max_total_windows: 2,
            ..Default::default()
        };
        let mut wm = WindowManager::with_config(config).unwrap();

        // Open two windows
        wm.open_window(WindowType::IndividualTool(ToolType::Hierarchy))
            .unwrap();
        wm.open_window(WindowType::IndividualTool(ToolType::Codex))
            .unwrap();

        // Try to open a third - should fail
        let result = wm.open_window(WindowType::IndividualTool(ToolType::Analysis));
        assert!(result.is_err());
        if let Err(WindowManagerError::MaxWindowsReached { max_windows }) = result {
            assert_eq!(max_windows, 2);
        }
    }
}
