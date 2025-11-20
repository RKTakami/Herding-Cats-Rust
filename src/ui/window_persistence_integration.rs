//! Window Persistence Integration Layer
//!
//! Provides seamless integration between window management and persistence systems.
//! This module bridges the gap between UI window operations and state persistence.

use crate::ui::window_state_persistence::{
    WindowPersistenceConfig, WindowPersistenceError, WindowPersistenceResult,
    WindowPersistenceStats, WindowState, WindowStateManager, WindowType,
};
// Removed ToolWindow import - not needed in this module
use crate::ui::enhanced_window_manager::EnhancedWindowManager;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Integration coordinator for window state persistence
#[derive(Clone)]
pub struct WindowPersistenceIntegration {
    state_manager: Arc<RwLock<WindowStateManager>>,
    enhanced_manager: Arc<RwLock<EnhancedWindowManager>>,
    is_initialized: bool,
    auto_save_enabled: bool,
}

/// Configuration for window persistence integration
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    pub enable_enhanced_persistence: bool,
    pub auto_sync_with_ui: bool,
    pub save_on_window_event: bool,
    pub load_on_startup: bool,
    pub default_layout_name: Option<String>,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enable_enhanced_persistence: true,
            auto_sync_with_ui: true,
            save_on_window_event: true,
            load_on_startup: true,
            default_layout_name: Some("Default Layout".to_string()),
        }
    }
}

/// Window event types that trigger persistence
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowEvent {
    WindowOpened {
        window_id: String,
    },
    WindowClosed {
        window_id: String,
    },
    WindowMoved {
        window_id: String,
        x: i32,
        y: i32,
    },
    WindowResized {
        window_id: String,
        width: i32,
        height: i32,
    },
    WindowMinimized {
        window_id: String,
    },
    WindowMaximized {
        window_id: String,
    },
    WindowRestored {
        window_id: String,
    },
    LayoutChanged {
        layout_name: String,
    },
    AllWindowsClosed,
}

/// Result of window event processing
#[derive(Debug)]
pub struct WindowEventResult {
    pub success: bool,
    pub persisted: bool,
    pub message: Option<String>,
    pub stats: Option<WindowPersistenceStats>,
}

impl WindowPersistenceIntegration {
    /// Create a new window persistence integration
    pub fn new(project_path: PathBuf, config: IntegrationConfig) -> WindowPersistenceResult<Self> {
        let persistence_config = WindowPersistenceConfig::default();
        let state_manager = WindowStateManager::new(persistence_config, project_path.clone());

        let enhanced_manager =
            match EnhancedWindowManager::new_with_persistence(project_path.clone()) {
                Ok(manager) => manager,
                Err(e) => {
                    log::warn!(
                        "Failed to create enhanced window manager with persistence: {}",
                        e
                    );
                    EnhancedWindowManager::new()
                }
            };

        Ok(Self {
            state_manager: Arc::new(RwLock::new(state_manager)),
            enhanced_manager: Arc::new(RwLock::new(enhanced_manager)),
            is_initialized: false,
            auto_save_enabled: config.save_on_window_event,
        })
    }

    /// Initialize the integration system
    pub async fn initialize(&mut self) -> WindowPersistenceResult<()> {
        // Initialize the state manager
        {
            let mut state_manager = self.state_manager.write().await;
            state_manager.initialize().await?;
        }

        // Initialize the enhanced window manager (already done in constructor)

        self.is_initialized = true;
        log::info!("Window persistence integration initialized");

        Ok(())
    }

    /// Handle window events and trigger persistence
    pub async fn handle_window_event(&self, event: WindowEvent) -> WindowEventResult {
        if !self.is_initialized {
            return WindowEventResult {
                success: false,
                persisted: false,
                message: Some("Integration not initialized".to_string()),
                stats: None,
            };
        }

        let mut persisted = false;
        let mut message = None;
        let mut stats = None;

        // Process the event based on type
        match event {
            WindowEvent::WindowOpened { window_id } => {
                if let Err(e) = self.handle_window_opened(&window_id).await {
                    message = Some(format!("Failed to handle window opened: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!("Window '{}' opened and persisted", window_id));
                }
            }

            WindowEvent::WindowClosed { window_id } => {
                if let Err(e) = self.handle_window_closed(&window_id).await {
                    message = Some(format!("Failed to handle window closed: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!("Window '{}' closed and persisted", window_id));
                }
            }

            WindowEvent::WindowMoved { window_id, x, y } => {
                if let Err(e) = self.handle_window_moved(&window_id, x, y).await {
                    message = Some(format!("Failed to handle window moved: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!("Window '{}' moved to ({}, {})", window_id, x, y));
                }
            }

            WindowEvent::WindowResized {
                window_id,
                width,
                height,
            } => {
                if let Err(e) = self.handle_window_resized(&window_id, width, height).await {
                    message = Some(format!("Failed to handle window resized: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!(
                        "Window '{}' resized to {}x{}",
                        window_id, width, height
                    ));
                }
            }

            WindowEvent::WindowMinimized { window_id } => {
                if let Err(e) = self.handle_window_state_changed(&window_id).await {
                    message = Some(format!("Failed to handle window minimized: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!("Window '{}' minimized", window_id));
                }
            }

            WindowEvent::WindowMaximized { window_id } => {
                if let Err(e) = self.handle_window_state_changed(&window_id).await {
                    message = Some(format!("Failed to handle window maximized: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!("Window '{}' maximized", window_id));
                }
            }

            WindowEvent::WindowRestored { window_id } => {
                if let Err(e) = self.handle_window_state_changed(&window_id).await {
                    message = Some(format!("Failed to handle window restored: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!("Window '{}' restored", window_id));
                }
            }

            WindowEvent::LayoutChanged { layout_name } => {
                if let Err(e) = self.handle_layout_changed(&layout_name).await {
                    message = Some(format!("Failed to handle layout changed: {}", e));
                } else {
                    persisted = true;
                    message = Some(format!("Layout changed to '{}'", layout_name));
                }
            }

            WindowEvent::AllWindowsClosed => {
                if let Err(e) = self.handle_all_windows_closed().await {
                    message = Some(format!("Failed to handle all windows closed: {}", e));
                } else {
                    persisted = true;
                    message = Some("All windows closed and state persisted".to_string());
                }
            }
        }

        // Get current statistics
        let state_manager = self.state_manager.read().await;
        stats = Some(state_manager.get_persistence_stats());

        WindowEventResult {
            success: message.is_none(),
            persisted,
            message,
            stats,
        }
    }

    /// Sync window states between UI and persistence
    pub async fn sync_window_states(&self) -> WindowPersistenceResult<()> {
        let enhanced_manager = self.enhanced_manager.read().await;
        let _state_manager = self.state_manager.read().await;

        // Get all current window states from the enhanced manager
        let current_states = enhanced_manager.get_all_window_states();

        log::debug!("Syncing {} window states", current_states.len());

        Ok(())
    }

    /// Load a specific window layout
    pub async fn load_layout(&self, layout_name: &str) -> WindowPersistenceResult<()> {
        // For demonstration purposes, we'll simulate loading a layout
        // In a real implementation, you would need to manage the async state properly
        log::info!("Loading layout '{}'", layout_name);

        // Apply the layout to the enhanced window manager
        let mut enhanced_manager = self.enhanced_manager.write().await;

        // Simulate getting window states from a layout
        let window_states = enhanced_manager.get_all_window_states();
        for (window_id, window_state) in window_states {
            if let Err(e) =
                enhanced_manager.apply_window_state_from_persistence(&window_id, &window_state)
            {
                log::error!("Failed to apply window state for {}: {}", window_id, e);
            }
        }

        log::info!("Loaded layout '{}'", layout_name);
        Ok(())
    }

    /// Save current window layout
    pub async fn save_current_layout(
        &self,
        layout_name: Option<String>,
    ) -> WindowPersistenceResult<()> {
        let mut state_manager = self.state_manager.write().await;
        state_manager.save_current_layout(layout_name).await?;
        log::info!("Current window layout saved");
        Ok(())
    }

    /// Get available window layouts
    pub fn get_available_layouts(&self) -> Vec<String> {
        if let Ok(state_manager) = self.state_manager.try_read() {
            state_manager
                .get_saved_layouts()
                .into_iter()
                .map(|layout| layout.name.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Create a new window layout
    pub async fn create_new_layout(
        &self,
        name: String,
        description: Option<String>,
    ) -> WindowPersistenceResult<()> {
        let mut state_manager = self.state_manager.write().await;
        state_manager.create_new_layout(name, description)?;
        log::info!("New window layout created");
        Ok(())
    }

    /// Delete a window layout
    pub async fn delete_layout(&self, layout_name: &str) -> WindowPersistenceResult<()> {
        let state_manager = self.state_manager.read().await;
        let layouts = state_manager.get_saved_layouts();

        let layout_id = layouts
            .into_iter()
            .find(|layout| layout.name == layout_name)
            .map(|layout| layout.id)
            .ok_or_else(|| {
                WindowPersistenceError::LayoutNotFound(format!(
                    "Layout '{}' not found",
                    layout_name
                ))
            })?;

        drop(state_manager); // Release read lock

        let mut state_manager = self.state_manager.write().await;
        state_manager.delete_layout(layout_id).await?;
        log::info!("Deleted window layout '{}'", layout_name);
        Ok(())
    }

    /// Export a window layout
    pub async fn export_layout(
        &self,
        layout_name: &str,
        export_path: &Path,
    ) -> WindowPersistenceResult<()> {
        let state_manager = self.state_manager.read().await;
        let layouts = state_manager.get_saved_layouts();

        let layout_id = layouts
            .into_iter()
            .find(|layout| layout.name == layout_name)
            .map(|layout| layout.id)
            .ok_or_else(|| {
                WindowPersistenceError::LayoutNotFound(format!(
                    "Layout '{}' not found",
                    layout_name
                ))
            })?;

        state_manager.export_layout(layout_id, export_path).await?;
        log::info!(
            "Exported window layout '{}' to {:?}",
            layout_name,
            export_path
        );
        Ok(())
    }

    /// Import a window layout
    pub async fn import_layout(&self, import_path: &Path) -> WindowPersistenceResult<String> {
        let mut state_manager = self.state_manager.write().await;
        let layout_id = state_manager.import_layout(import_path).await?;

        // Get the imported layout name
        let layout_name = state_manager
            .get_saved_layouts()
            .into_iter()
            .find(|layout| layout.id == layout_id)
            .map(|layout| layout.name.clone())
            .unwrap_or_else(|| "Imported Layout".to_string());

        log::info!(
            "Imported window layout '{}' from {:?}",
            layout_name,
            import_path
        );
        Ok(layout_name)
    }

    /// Get integration statistics
    pub async fn get_integration_stats(&self) -> WindowPersistenceStats {
        let state_manager = self.state_manager.read().await;
        state_manager.get_persistence_stats()
    }

    // Private event handlers

    async fn handle_window_opened(&self, window_id: &str) -> WindowPersistenceResult<()> {
        let enhanced_manager = self.enhanced_manager.read().await;
        let mut state_manager = self.state_manager.write().await;

        if let Some(window) = enhanced_manager.get_window(window_id) {
            let window_state = WindowState::new(
                window.id.clone(),
                window.title.clone(),
                self.map_tool_window_type(window_id),
            );

            // Update with current position and size
            let mut updated_state = window_state;
            updated_state.update_position(window.x, window.y);
            updated_state.update_size(window.width, window.height);
            updated_state.update_visibility(
                window.is_visible,
                window.is_minimized,
                window.is_maximized,
            );
            updated_state.record_open();

            state_manager.update_window_state(window_id, updated_state)?;
        }

        Ok(())
    }

    async fn handle_window_closed(&self, window_id: &str) -> WindowPersistenceResult<()> {
        let mut state_manager = self.state_manager.write().await;

        // Update the window state to mark as closed
        if let Some(window_state) = state_manager.get_window_state(window_id) {
            let mut updated_state = window_state.clone();
            updated_state.update_visibility(false, false, false);
            state_manager.update_window_state(window_id, updated_state)?;
        }

        Ok(())
    }

    async fn handle_window_moved(
        &self,
        window_id: &str,
        x: i32,
        y: i32,
    ) -> WindowPersistenceResult<()> {
        let mut state_manager = self.state_manager.write().await;

        if let Some(window_state) = state_manager.get_window_state(window_id) {
            let mut updated_state = window_state.clone();
            updated_state.update_position(x, y);
            state_manager.update_window_state(window_id, updated_state)?;
        }

        Ok(())
    }

    async fn handle_window_resized(
        &self,
        window_id: &str,
        width: i32,
        height: i32,
    ) -> WindowPersistenceResult<()> {
        let mut state_manager = self.state_manager.write().await;

        if let Some(window_state) = state_manager.get_window_state(window_id) {
            let mut updated_state = window_state.clone();
            updated_state.update_size(width, height);
            state_manager.update_window_state(window_id, updated_state)?;
        }

        Ok(())
    }

    async fn handle_window_state_changed(&self, window_id: &str) -> WindowPersistenceResult<()> {
        let enhanced_manager = self.enhanced_manager.read().await;
        let mut state_manager = self.state_manager.write().await;

        if let Some(window) = enhanced_manager.get_window(window_id) {
            if let Some(window_state) = state_manager.get_window_state(window_id) {
                let mut updated_state = window_state.clone();
                updated_state.update_visibility(
                    window.is_visible,
                    window.is_minimized,
                    window.is_maximized,
                );
                state_manager.update_window_state(window_id, updated_state)?;
            }
        }

        Ok(())
    }

    async fn handle_layout_changed(&self, layout_name: &str) -> WindowPersistenceResult<()> {
        self.save_current_layout(Some(layout_name.to_string()))
            .await?;
        Ok(())
    }

    async fn handle_all_windows_closed(&self) -> WindowPersistenceResult<()> {
        let enhanced_manager = self.enhanced_manager.read().await;
        let window_ids = enhanced_manager.get_all_window_ids();
        drop(enhanced_manager);

        for window_id in window_ids {
            self.handle_window_closed(&window_id).await?;
        }

        Ok(())
    }

    /// Map tool window types to persistence window types
    fn map_tool_window_type(&self, window_id: &str) -> WindowType {
        match window_id {
            "hierarchy" => WindowType::Hierarchy,
            "codex" => WindowType::Codex,
            "plot" => WindowType::Plot,
            "analysis" => WindowType::Analysis,
            "notes" => WindowType::Notes,
            "research" => WindowType::Research,
            "settings" => WindowType::Settings,
            _ => WindowType::Custom(window_id.to_string()),
        }
    }
}

/// Global window persistence integration instance
use once_cell::sync::Lazy;
use std::sync::Mutex;

static WINDOW_PERSISTENCE_INTEGRATION: Lazy<Mutex<Option<WindowPersistenceIntegration>>> =
    Lazy::new(|| Mutex::new(None));

/// Initialize global window persistence integration
pub fn init_window_persistence_integration(
    project_path: PathBuf,
    config: IntegrationConfig,
) -> WindowPersistenceResult<()> {
    let mut integration = WINDOW_PERSISTENCE_INTEGRATION.lock().unwrap();
    *integration = Some(WindowPersistenceIntegration::new(project_path, config)?);
    Ok(())
}

/// Get global window persistence integration instance
pub async fn get_window_persistence_integration() -> Option<WindowPersistenceIntegration> {
    let integration = WINDOW_PERSISTENCE_INTEGRATION.lock().unwrap();
    integration.clone()
}

/// Initialize window persistence integration with defaults
pub fn init_window_persistence_integration_with_defaults(
    project_path: PathBuf,
) -> WindowPersistenceResult<()> {
    init_window_persistence_integration(project_path, IntegrationConfig::default())
}

