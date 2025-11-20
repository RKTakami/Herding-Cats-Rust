//! Enhanced Window Manager for Herding Cats Rust
//! Provides resizeable and draggable tool windows with persistence

use crate::ui::window_state_persistence::{
    FocusState, LayoutConstraints, WindowLayoutInfo, WindowMetadata, WindowPersistenceConfig,
    WindowPersistenceResult, WindowPosition, WindowSize, WindowState, WindowStateManager,
    WindowType, WindowUIState, WindowVisibility,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Enhanced tool window with resize and drag capabilities
#[derive(Debug, Clone)]
pub struct EnhancedToolWindow {
    pub id: String,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_visible: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub is_dragging: bool,
    pub is_resizing: bool,
    pub z_index: i32,
}

/// Manager for handling enhanced tool windows with resize and drag
#[derive(Default, Debug)]
pub struct EnhancedWindowManager {
    windows: HashMap<String, EnhancedToolWindow>,
    active_windows: Vec<String>,
    next_z_index: i32,
    state_manager: Option<Arc<RwLock<WindowStateManager>>>,
}

impl EnhancedWindowManager {
    /// Create a new enhanced window manager
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.initialize_default_windows();
        manager
    }

    /// Create a new enhanced window manager with persistence
    pub fn new_with_persistence(project_path: PathBuf) -> WindowPersistenceResult<Self> {
        let config = WindowPersistenceConfig::default();
        let state_manager = WindowStateManager::new(config, project_path);

        let mut manager = Self {
            windows: HashMap::new(),
            active_windows: Vec::new(),
            next_z_index: 0,
            state_manager: Some(Arc::new(RwLock::new(state_manager))),
        };

        manager.initialize_default_windows();

        Ok(manager)
    }

    /// Initialize default tool windows
    fn initialize_default_windows(&mut self) {
        let default_windows = vec![
            (
                "hierarchy",
                "📊 Document Hierarchy - Enhanced",
                100,
                100,
                400,
                600,
            ),
            (
                "codex",
                "📖 World Building Codex - Enhanced",
                150,
                150,
                600,
                700,
            ),
            ("plot", "📈 Plot Development - Enhanced", 200, 200, 500, 650),
            (
                "analysis",
                "📈 Writing Analysis - Enhanced",
                250,
                250,
                450,
                550,
            ),
            ("notes", "📝 Research Notes - Enhanced", 300, 300, 500, 600),
            (
                "research",
                "🔍 Research & Sources - Enhanced",
                350,
                350,
                550,
                650,
            ),
        ];

        for (id, title, x, y, width, height) in default_windows {
            self.add_window(id, title, x, y, width, height);
        }
    }

    /// Add a new enhanced tool window
    pub fn add_window(&mut self, id: &str, title: &str, x: i32, y: i32, width: i32, height: i32) {
        let window = EnhancedToolWindow {
            id: id.to_string(),
            title: title.to_string(),
            x,
            y,
            width,
            height,
            is_visible: false,
            is_minimized: false,
            is_maximized: false,
            is_dragging: false,
            is_resizing: false,
            z_index: self.next_z_index,
        };

        self.next_z_index += 1;
        self.windows.insert(id.to_string(), window);
    }

    /// Show a tool window
    pub fn show_window(&mut self, id: &str) -> Result<(), String> {
        let title = {
            if let Some(window) = self.windows.get_mut(id) {
                window.is_visible = true;
                window.is_minimized = false;
                window.title.clone()
            } else {
                return Err(format!("Window '{}' not found", id));
            }
        };

        self.bring_to_front(id)?;

        // Persist the window state change
        self.persist_window_state(id)?;

        log::info!("🔧 Enhanced tool window '{}' is now visible", title);
        Ok(())
    }

    /// Hide a tool window
    pub fn hide_window(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.is_visible = false;
            window.is_dragging = false;
            window.is_resizing = false;
            log::info!("🔧 Enhanced tool window '{}' is now hidden", window.title);
            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Toggle window visibility
    pub fn toggle_window(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            if window.is_visible {
                self.hide_window(id)
            } else {
                self.show_window(id)
            }
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Start dragging a window
    pub fn start_drag(&mut self, id: &str, delta_x: i32, delta_y: i32) -> Result<(), String> {
        let (title, x, y) = {
            if let Some(window) = self.windows.get_mut(id) {
                window.is_dragging = true;
                window.x += delta_x;
                window.y += delta_y;
                (window.title.clone(), window.x, window.y)
            } else {
                return Err(format!("Window '{}' not found", id));
            }
        };

        self.bring_to_front(id)?;
        log::info!("📌 Started dragging window '{}' to ({}, {})", title, x, y);
        Ok(())
    }

    /// Stop dragging a window
    pub fn stop_drag(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.is_dragging = false;
            log::info!("📌 Stopped dragging window '{}'", window.title);
            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Start resizing a window
    pub fn start_resize(
        &mut self,
        id: &str,
        resize_type: &str,
        delta_x: i32,
        delta_y: i32,
    ) -> Result<(), String> {
        let (title, new_width, new_height) = {
            if let Some(window) = self.windows.get_mut(id) {
                window.is_resizing = true;

                match resize_type {
                    "bottom-right" => {
                        window.width = (window.width + delta_x).max(300);
                        window.height = (window.height + delta_y).max(200);
                        (window.title.clone(), window.width, window.height)
                    }
                    "right" => {
                        window.width = (window.width + delta_x).max(300);
                        (window.title.clone(), window.width, window.height)
                    }
                    "bottom" => {
                        window.height = (window.height + delta_y).max(200);
                        (window.title.clone(), window.width, window.height)
                    }
                    _ => {
                        return Err(format!("Unknown resize type: {}", resize_type));
                    }
                }
            } else {
                return Err(format!("Window '{}' not found", id));
            }
        };

        self.bring_to_front(id)?;
        log::info!(
            "📐 Started resizing window '{}' - type: {} size: {}x{}",
            title,
            resize_type,
            new_width,
            new_height
        );
        Ok(())
    }

    /// Stop resizing a window
    pub fn stop_resize(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.is_resizing = false;
            log::info!(
                "📐 Stopped resizing window '{}' final size: {}x{}",
                window.title,
                window.width,
                window.height
            );
            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Minimize a window
    pub fn minimize_window(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.is_minimized = !window.is_minimized;
            if window.is_minimized {
                window.is_dragging = false;
                window.is_resizing = false;
            }
            log::info!(
                "📦 Window '{}' {}",
                window.title,
                if window.is_minimized {
                    "minimized"
                } else {
                    "restored"
                }
            );
            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Maximize/restore a window
    pub fn maximize_window(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.is_maximized = !window.is_maximized;
            if window.is_maximized {
                window.is_dragging = false;
                window.is_resizing = false;
                // In a real implementation, you'd store the original size/position
                // and set to screen size
            }
            log::info!(
                "🖥️ Window '{}' {}",
                window.title,
                if window.is_maximized {
                    "maximized"
                } else {
                    "restored"
                }
            );
            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Close a window
    pub fn close_window(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.is_visible = false;
            window.is_dragging = false;
            window.is_resizing = false;
            self.active_windows.retain(|w| w != id);
            log::info!("❌ Closed window '{}'", window.title);
            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Bring a window to the front
    fn bring_to_front(&mut self, id: &str) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.z_index = self.next_z_index;
            self.next_z_index += 1;

            // Remove from active_windows if it exists
            self.active_windows.retain(|w| w != id);

            // Add to the end (top of z-order)
            self.active_windows.push(id.to_string());

            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Get window information
    pub fn get_window(&self, id: &str) -> Option<&EnhancedToolWindow> {
        self.windows.get(id)
    }

    /// Get all visible windows
    pub fn get_visible_windows(&self) -> Vec<&EnhancedToolWindow> {
        self.windows
            .values()
            .filter(|window| window.is_visible && !window.is_minimized)
            .collect()
    }

    /// Get all window IDs
    pub fn get_all_window_ids(&self) -> Vec<String> {
        self.windows.keys().cloned().collect()
    }

    /// Get active windows (z-order)
    pub fn get_active_windows(&self) -> &[String] {
        &self.active_windows
    }

    /// Check if a window is visible
    pub fn is_window_visible(&self, id: &str) -> bool {
        self.windows
            .get(id)
            .map(|window| window.is_visible && !window.is_minimized)
            .unwrap_or(false)
    }

    /// Get window position and size
    pub fn get_window_bounds(&self, id: &str) -> Option<(i32, i32, i32, i32)> {
        self.windows
            .get(id)
            .map(|window| (window.x, window.y, window.width, window.height))
    }

    /// Update window bounds
    pub fn update_window_bounds(
        &mut self,
        id: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(id) {
            window.x = x;
            window.y = y;
            window.width = width.max(300);
            window.height = height.max(200);
            Ok(())
        } else {
            Err(format!("Window '{}' not found", id))
        }
    }

    /// Get window title
    pub fn get_window_title(&self, id: &str) -> Option<&str> {
        self.windows.get(id).map(|window| window.title.as_str())
    }

    /// Check if window is being dragged
    pub fn is_window_dragging(&self, id: &str) -> bool {
        self.windows
            .get(id)
            .map(|window| window.is_dragging)
            .unwrap_or(false)
    }

    /// Check if window is being resized
    pub fn is_window_resizing(&self, id: &str) -> bool {
        self.windows
            .get(id)
            .map(|window| window.is_resizing)
            .unwrap_or(false)
    }

    /// Get total number of windows
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Get number of visible windows
    pub fn visible_window_count(&self) -> usize {
        self.windows
            .values()
            .filter(|w| w.is_visible && !w.is_minimized)
            .count()
    }

    /// Persist current window state
    fn persist_window_state(&self, window_id: &str) -> Result<(), String> {
        if let Some(state_manager) = &self.state_manager {
            // In a real implementation, this would be async
            // For now, we'll just log the intent
            log::debug!("Persisting window state for '{}'", window_id);
        }
        Ok(())
    }

    /// Load window state from persistence
    pub fn load_window_state(&mut self, window_id: &str) -> Result<(), String> {
        if let Some(state_manager) = &self.state_manager {
            // In a real async implementation, we would:
            // let state_manager = state_manager.read().await;
            // if let Some(window_state) = state_manager.get_window_state(window_id) {
            //     self.apply_window_state(window_id, window_state)?;
            // }
            log::debug!("Loading window state for '{}'", window_id);
        }
        Ok(())
    }

    /// Apply loaded window state to the window manager
    fn apply_window_state(
        &mut self,
        window_id: &str,
        window_state: &WindowState,
    ) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.x = window_state.position.x;
            window.y = window_state.position.y;
            window.width = window_state.size.width;
            window.height = window_state.size.height;
            window.is_visible = window_state.visibility.is_visible;
            window.is_minimized = window_state.visibility.is_minimized;
            window.is_maximized = window_state.visibility.is_maximized;
            window.z_index = window_state.z_index;
        }
        Ok(())
    }

    /// Save all current window states
    pub fn save_all_window_states(&self) -> Result<(), String> {
        if let Some(state_manager) = &self.state_manager {
            // In a real implementation, this would be async
            log::debug!("Saving all window states");
        }
        Ok(())
    }

    /// Get persistence statistics
    pub fn get_persistence_stats(&self) -> Option<serde_json::Value> {
        if let Some(_state_manager) = &self.state_manager {
            // In a real implementation, we would get actual stats
            Some(serde_json::json!({
                "persistence_enabled": true,
                "window_count": self.windows.len(),
                "visible_windows": self.visible_window_count()
            }))
        } else {
            Some(serde_json::json!({
                "persistence_enabled": false,
                "window_count": self.windows.len(),
                "visible_windows": self.visible_window_count()
            }))
        }
    }

    /// Get all window states (public for integration)
    pub fn get_all_window_states(&self) -> HashMap<String, WindowState> {
        let mut states = HashMap::new();
        for (id, window) in &self.windows {
            let window_state = WindowState {
                id: id.clone(),
                title: window.title.clone(),
                window_type: WindowType::Custom(id.clone()), // Simplified mapping
                position: WindowPosition {
                    x: window.x,
                    y: window.y,
                    screen: None,
                },
                size: WindowSize {
                    width: window.width,
                    height: window.height,
                    min_width: None,
                    min_height: None,
                    max_width: None,
                    max_height: None,
                },
                visibility: WindowVisibility {
                    is_visible: window.is_visible,
                    is_minimized: window.is_minimized,
                    is_maximized: window.is_maximized,
                    is_fullscreen: false,
                },
                state: WindowUIState {
                    is_dragging: window.is_dragging,
                    is_resizing: window.is_resizing,
                    resize_handle: None,
                    focus_state: FocusState::Unfocused,
                },
                z_index: window.z_index,
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
            };
            states.insert(id.clone(), window_state);
        }
        states
    }

    /// Apply window state from persistence (public for integration)
    pub fn apply_window_state_from_persistence(
        &mut self,
        window_id: &str,
        window_state: &WindowState,
    ) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.x = window_state.position.x;
            window.y = window_state.position.y;
            window.width = window_state.size.width;
            window.height = window_state.size.height;
            window.is_visible = window_state.visibility.is_visible;
            window.is_minimized = window_state.visibility.is_minimized;
            window.is_maximized = window_state.visibility.is_maximized;
            window.z_index = window_state.z_index;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_management() {
        let mut manager = EnhancedWindowManager::new();

        // Test showing a window
        assert!(manager.show_window("hierarchy").is_ok());
        assert!(manager.is_window_visible("hierarchy"));

        // Test dragging
        assert!(manager.start_drag("hierarchy", 10, 20).is_ok());
        assert!(manager.is_window_dragging("hierarchy"));
        assert!(manager.stop_drag("hierarchy").is_ok());
        assert!(!manager.is_window_dragging("hierarchy"));

        // Test resizing
        assert!(manager
            .start_resize("hierarchy", "bottom-right", 50, 30)
            .is_ok());
        assert!(manager.is_window_resizing("hierarchy"));
        assert!(manager.stop_resize("hierarchy").is_ok());
        assert!(!manager.is_window_resizing("hierarchy"));

        // Test minimize
        assert!(manager.minimize_window("hierarchy").is_ok());

        // Test maximize
        assert!(manager.maximize_window("hierarchy").is_ok());

        // Test close
        assert!(manager.close_window("hierarchy").is_ok());
        assert!(!manager.is_window_visible("hierarchy"));
    }

    #[test]
    fn test_window_bounds() {
        let mut manager = EnhancedWindowManager::new();
        let initial_bounds = manager.get_window_bounds("hierarchy").unwrap();

        manager
            .update_window_bounds("hierarchy", 100, 200, 500, 700)
            .unwrap();
        let new_bounds = manager.get_window_bounds("hierarchy").unwrap();

        assert_eq!(new_bounds, (100, 200, 500, 700));
        assert_ne!(initial_bounds, new_bounds);
    }
}
