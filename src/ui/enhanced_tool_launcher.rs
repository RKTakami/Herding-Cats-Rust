//! Enhanced Tool Launcher
//!
//! Provides intelligent tool launching with window management constraints:
//! - Only one instance of each tool can be opened
//! - Automatic window management and focus handling
//! - Independent tool window management

use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::{
    ui::tools::base_types::ToolType,
    ui::window_manager::{WindowId, WindowManager, WindowManagerError, WindowType},
};

use crate::ui::tools::individual_tool_windows::IndividualToolWindowManager;

/// Enhanced tool launcher that enforces window management constraints
pub struct EnhancedToolLauncher {
    /// Reference to the global window manager
    window_manager: Arc<Mutex<WindowManager>>,
    /// Reference to the individual tool window manager (for spawning actual windows)
    tool_window_manager: Arc<Mutex<Option<IndividualToolWindowManager>>>,
}

impl EnhancedToolLauncher {
    /// Create a new enhanced tool launcher
    pub fn new() -> Result<Self> {
        Ok(Self {
            window_manager: Arc::new(Mutex::new(WindowManager::new()?)),
            tool_window_manager: Arc::new(Mutex::new(None)),
        })
    }

    /// Register the individual tool window manager
    pub fn register_tool_manager(&self, manager: IndividualToolWindowManager) {
        let mut tool_mgr = self.tool_window_manager.lock().unwrap();
        *tool_mgr = Some(manager);
        println!("✅ IndividualToolWindowManager registered with EnhancedToolLauncher");
    }

    /// Create a new enhanced tool launcher with custom window manager
    pub fn with_window_manager(window_manager: Arc<Mutex<WindowManager>>) -> Self {
        Self {
            window_manager,
            tool_window_manager: Arc::new(Mutex::new(None)),
        }
    }

    /// Launch an individual tool window
    ///
    /// If the tool is already open, this will focus the existing window.
    /// If the tool is not open, this will create a new window.
    pub fn launch_tool(&self, tool_type: ToolType) -> Result<WindowId, WindowManagerError> {
        // Scope for WindowManager lock
        let (window_id, is_new) = {
            let mut wm = self.window_manager.lock().unwrap();

            // Check if tool is already open
            if wm.is_tool_open(tool_type) {
                // Tool is already open, focus the existing window
                if let Some(existing_window_id) = wm.find_open_tool_window(&tool_type) {
                    wm.focus_window(existing_window_id)?;
                    println!(
                        "🎯 Tool {} is already open - focusing existing window {}",
                        tool_type.display_name(),
                        existing_window_id.0
                    );
                    (existing_window_id, false)
                } else {
                    // Should not happen if is_tool_open is true
                    return Err(WindowManagerError::WindowNotFound { window_id: WindowId(0) });
                }
            } else {
                // Tool is not open, create new window
                let window_id = wm.open_window(WindowType::IndividualTool(tool_type))?;
                (window_id, true)
            }
        }; // wm lock dropped here

        // Actually spawn/show the window using the registered manager
        // This is done outside the WindowManager lock to prevent deadlocks
        // We also clone the manager and release the tool_window_manager lock to prevent deadlocks there too
        let manager_opt = {
            let guard = self.tool_window_manager.lock().unwrap();
            guard.clone()
        };

        if let Some(manager) = manager_opt {
            if is_new {
                println!("🖥️ Spawning Slint window for {}", tool_type.display_name());
            } else {
                println!("🔄 Ensuring Slint window is visible for {}", tool_type.display_name());
            }
            
            if let Err(e) = manager.open_tool_window(tool_type) {
                if is_new {
                    println!("❌ Failed to spawn Slint window: {}", e);
                } else {
                    println!("❌ Failed to re-show Slint window: {}", e);
                }
                // Note: We might want to rollback the WM state here, but for now we just log it
            }
        } else {
            println!("⚠️ No tool window manager registered! Window state updated but no UI shown.");
        }

        if is_new {
            println!(
                "🚀 Launched new {} tool window (ID: {})",
                tool_type.display_name(),
                window_id.0
            );
        }

        Ok(window_id)
    }


    /// Close a specific window
    pub fn close_window(&self, window_id: WindowId) -> Result<(), WindowManagerError> {
        let mut wm = self.window_manager.lock().unwrap();
        wm.close_window(window_id)?;

        println!("👋 Closed window {}", window_id.0);
        Ok(())
    }

    /// Close all windows
    pub fn close_all_windows(&self) -> Result<(), WindowManagerError> {
        let mut wm = self.window_manager.lock().unwrap();
        let open_windows: Vec<WindowId> = wm
            .get_open_windows()
            .iter()
            .map(|state| state.window_id)
            .collect();

        for window_id in open_windows {
            wm.close_window(window_id)?;
            println!("👋 Closed window {}", window_id.0);
        }

        Ok(())
    }

    /// Get all open windows
    pub fn get_open_windows(&self) -> Vec<WindowStateInfo> {
        let wm = self.window_manager.lock().unwrap();
        wm.get_open_windows()
            .iter()
            .map(|state| WindowStateInfo {
                window_id: state.window_id,
                window_type: state.window_type.clone(),
                is_focused: state.is_focused,
                position: state.position,
                size: state.size,
                creation_time: state.creation_time,
            })
            .collect()
    }

    /// Get statistics about window usage
    pub fn get_statistics(&self) -> WindowStatisticsInfo {
        let wm = self.window_manager.lock().unwrap();
        let stats = wm.get_statistics();

        WindowStatisticsInfo {
            total_open_windows: stats.total_open_windows,
            open_tools: stats.open_tools,
            max_total_windows: stats.max_total_windows,
            available_tools: stats.available_tools,
            open_tools_list: self.get_open_tool_types(),
        }
    }

    /// Get list of currently open tool types
    pub fn get_open_tool_types(&self) -> Vec<ToolType> {
        let wm = self.window_manager.lock().unwrap();
        ToolType::all_types()
            .iter()
            .filter(|&&tool| wm.is_tool_open(tool))
            .copied()
            .collect()
    }

    /// Check if a specific tool is open
    pub fn is_tool_open(&self, tool_type: ToolType) -> bool {
        let wm = self.window_manager.lock().unwrap();
        wm.is_tool_open(tool_type)
    }

    /// Focus a specific window
    pub fn focus_window(&self, window_id: WindowId) -> Result<(), WindowManagerError> {
        let mut wm = self.window_manager.lock().unwrap();
        wm.focus_window(window_id)?;

        println!("🎯 Focused window {}", window_id.0);
        Ok(())
    }

    /// Show window management status
    pub fn show_status(&self) {
        let stats = self.get_statistics();

        println!("📊 Window Management Status");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!(
            "Total Open Windows: {}/{}",
            stats.total_open_windows, stats.max_total_windows
        );
        println!(
            "Open Individual Tools: {}/{}",
            stats.open_tools,
            ToolType::all_types().len()
        );
        println!(
            "Open Individual Tools: {}",
            stats.open_tools
        );
        println!(
            "Available Tools: {}",
            stats.available_tools
        );
        println!();

        if !stats.open_tools_list.is_empty() {
            println!("Open Individual Tools:");
            for tool in &stats.open_tools_list {
                println!("  • {} ✅", tool.display_name());
            }
            println!();
        }

        println!("Available Tools for Opening:");
        for tool in ToolType::all_types() {
            if !stats.open_tools_list.contains(&tool) {
                println!("  • {} ➕", tool.display_name());
            }
        }
        println!();

        let open_windows = self.get_open_windows();
        if !open_windows.is_empty() {
            println!("Active Windows:");
            for window_info in &open_windows {
                let focus_indicator = if window_info.is_focused {
                    " (FOCUSED)"
                } else {
                    ""
                };
                match window_info.window_type {
                    WindowType::IndividualTool(tool) => {
                        println!(
                            "  • {} Tool Window #{}{}",
                            tool.display_name(),
                            window_info.window_id.0,
                            focus_indicator
                        );
                    }
                }
            }
        } else {
            println!("No windows are currently open.");
        }
        println!();
    }

    /// Intelligent tool launcher that chooses the best window type
    pub fn intelligent_launch(&self, tool_type: ToolType) -> Result<WindowId, WindowManagerError> {
        println!(
            "🎯 Launching {} in individual tool window",
            tool_type.display_name()
        );
        self.launch_tool(tool_type)
    }

    /// Close a tool by type (if it's open as an individual window)
    pub fn close_tool(&self, tool_type: ToolType) -> Result<(), WindowManagerError> {
        let mut wm = self.window_manager.lock().unwrap();

        if let Some(window_id) = wm.find_open_tool_window(&tool_type) {
            wm.close_window(window_id)?;
            println!(
                "👋 Closed {} tool window (ID: {})",
                tool_type.display_name(),
                window_id.0
            );
            Ok(())
        } else {
            println!(
                "ℹ️ {} tool is not open as an individual window",
                tool_type.display_name()
            );
            Err(WindowManagerError::ToolAlreadyOpen {
                tool: tool_type,
                existing_window_id: WindowId(0),
            })
        }
    }

    /// Close all individual tool windows
    pub fn close_all_tool_windows(&self) -> Result<(), WindowManagerError> {
        let open_tools = self.get_open_tool_types();

        for tool in open_tools {
            self.close_tool(tool)?;
        }

        Ok(())
    }

}

/// Window state information for external consumption
#[derive(Debug, Clone)]
pub struct WindowStateInfo {
    pub window_id: WindowId,
    pub window_type: WindowType,
    pub is_focused: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub creation_time: std::time::SystemTime,
}

/// Window statistics information for external consumption
#[derive(Debug, Clone)]
pub struct WindowStatisticsInfo {
    pub total_open_windows: usize,
    pub open_tools: usize,
    pub max_total_windows: usize,
    pub available_tools: usize,
    pub open_tools_list: Vec<ToolType>,
}

/// Global enhanced tool launcher instance
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_ENHANCED_LAUNCHER: Arc<Mutex<EnhancedToolLauncher>> = {
        Arc::new(Mutex::new(
            EnhancedToolLauncher::new().expect("Failed to create global enhanced launcher"),
        ))
    };
}

/// Get a reference to the global enhanced launcher
pub fn get_enhanced_launcher() -> std::sync::MutexGuard<'static, EnhancedToolLauncher> {
    GLOBAL_ENHANCED_LAUNCHER.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_launcher_creation() {
        let launcher = EnhancedToolLauncher::new().unwrap();
        let stats = launcher.get_statistics();

        assert_eq!(stats.total_open_windows, 0);
        assert_eq!(stats.open_tools, 0);
        assert!(stats.open_tools_list.is_empty());
    }

    #[test]
    fn test_single_tool_instance_constraint() {
        let launcher = EnhancedToolLauncher::new().unwrap();

        // Launch tool first time
        let window1 = launcher.launch_tool(ToolType::Hierarchy).unwrap();
        assert!(launcher.is_tool_open(ToolType::Hierarchy));

        // Launch same tool again - should focus existing window
        let window2 = launcher.launch_tool(ToolType::Hierarchy).unwrap();
        assert_eq!(window1, window2);

        let stats = launcher.get_statistics();
        assert_eq!(stats.open_tools, 1);
        assert_eq!(stats.open_tools_list.len(), 1);
        assert!(stats.open_tools_list.contains(&ToolType::Hierarchy));
    }

    #[test]
    fn test_individual_tool_windows_only() {
        let launcher = EnhancedToolLauncher::new().unwrap();

        // Test that only individual tool windows are supported
        let window1 = launcher.launch_tool(ToolType::Hierarchy).unwrap();
        let window2 = launcher.launch_tool(ToolType::Codex).unwrap();

        let stats = launcher.get_statistics();
        assert_eq!(stats.total_open_windows, 2);
        assert_eq!(stats.open_tools, 2);
        assert_eq!(stats.open_tools_list.len(), 2);

        // Verify all windows are individual tool windows
        let open_windows = launcher.get_open_windows();
        assert_eq!(open_windows.len(), 2);
        for window in &open_windows {
            match window.window_type {
                WindowType::IndividualTool(_) => {}, // This is expected
            }
        }
    }

    #[test]
    fn test_mixed_window_types() {
        let launcher = EnhancedToolLauncher::new().unwrap();

        // Launch some individual tools
        launcher.launch_tool(ToolType::Hierarchy).unwrap();
        launcher.launch_tool(ToolType::Codex).unwrap();

        // Launch some individual tools
        launcher.launch_tool(ToolType::Codex).unwrap();
        launcher.launch_tool(ToolType::Analysis).unwrap();

        let stats = launcher.get_statistics();
        assert_eq!(stats.open_tools, 2);
        assert_eq!(stats.total_open_windows, 2);
        assert_eq!(stats.open_tools_list.len(), 2);
    }

    #[test]
    fn test_close_operations() {
        let launcher = EnhancedToolLauncher::new().unwrap();

        // Launch windows
        let tool_window = launcher.launch_tool(ToolType::Hierarchy).unwrap();
        let tool_window2 = launcher.launch_tool(ToolType::Plot).unwrap();

        assert_eq!(launcher.get_statistics().total_open_windows, 2);

        // Close tool window
        launcher.close_tool(ToolType::Hierarchy).unwrap();
        assert!(!launcher.is_tool_open(ToolType::Hierarchy));
        assert_eq!(launcher.get_statistics().total_open_windows, 1);

        // Close tool windows
        launcher.close_window(tool_window).unwrap();
        launcher.close_window(tool_window2).unwrap();
        assert_eq!(launcher.get_statistics().total_open_windows, 0);
    }
}
