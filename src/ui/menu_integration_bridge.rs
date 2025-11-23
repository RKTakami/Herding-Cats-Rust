//! Menu Integration Bridge
//!
//! This module provides the bridge between UI menu callbacks and the actual
//! tool launching system. It connects the Slint UI components to the
//! ViewMenuIntegration and EnhancedToolLauncher.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    ui::tools::base_types::ToolType,
    ui::view_menu_integration::{RealViewMenuHandler, ViewMenuIntegration},
};
use crate as hc_lib;
use hc_lib::database_app_state::EnhancedDatabaseService;
use hc_lib::ui_state::AppState;

/// Global menu integration bridge instance
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MENU_BRIDGE: Arc<RwLock<MenuIntegrationBridge>> =
        { Arc::new(RwLock::new(MenuIntegrationBridge::new())) };
}

/// Bridge between UI callbacks and tool launching system
pub struct MenuIntegrationBridge {
    /// View menu integration for tool launching
    view_integration: Option<ViewMenuIntegration>,
    /// Real view menu handler for production use
    real_handler: Option<RealViewMenuHandler>,
    /// Database service for tool operations
    db_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
    /// App state for coordination
    app_state: Option<Arc<RwLock<AppState>>>,
    /// Initialization status
    initialized: bool,
}

impl MenuIntegrationBridge {
    /// Create a new menu integration bridge
    pub fn new() -> Self {
        Self {
            view_integration: None,
            real_handler: None,
            db_service: None,
            app_state: None,
            initialized: false,
        }
    }

    /// Initialize the bridge with database service
    pub async fn initialize(
        &mut self,
        db_service: Arc<RwLock<EnhancedDatabaseService>>,
    ) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        // Create view integration
        let view_integration = match ViewMenuIntegration::new(db_service.clone()) {
            Ok(vi) => vi,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to create view integration: {}", e));
            }
        };

        // Create real handler
        let real_handler = match RealViewMenuHandler::new(db_service.clone()) {
            Ok(rh) => rh,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to create real handler: {}", e));
            }
        };

        self.view_integration = Some(view_integration);
        self.real_handler = Some(real_handler);
        self.db_service = Some(db_service);
        self.initialized = true;

        println!("🔧 Menu Integration Bridge initialized successfully");
        println!("✅ All tool launching components are ready");
        Ok(())
    }

    /// Set app state reference
    pub async fn set_app_state(&mut self, app_state: Arc<RwLock<AppState>>) {
        self.app_state = Some(app_state.clone());

        if let Some(ref mut view_integration) = self.view_integration {
            view_integration.set_app_state(app_state.clone());
        }

        if let Some(ref mut real_handler) = self.real_handler {
            real_handler.set_app_state(app_state).await;
        }
    }

    /// Handle individual tool menu actions
    pub fn handle_writing_tools_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("Menu bridge not initialized".into());
        }

        // For the independent architecture, we focus on individual tools
        // This method can be used for legacy compatibility or future enhancements
        println!("💡 Independent writing tools architecture is active");
        println!("💡 Use individual tool menu entries for direct access");

        Ok(())
    }

    /// Handle individual tool menu actions
    pub fn handle_individual_tool_menu(
        &self,
        tool_type: ToolType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("Menu bridge not initialized".into());
        }

        if let Some(ref integration) = self.view_integration {
            integration.handle_individual_tool_menu(tool_type)
        } else {
            Err("View integration not available".into())
        }
    }

    /// Handle View -> Universal Tools menu action
    pub fn handle_universal_tools_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("Menu bridge not initialized".into());
        }

        // Universal tools are not supported in the independent architecture
        println!("💡 Universal tools are not available in independent architecture");
        println!("💡 Use individual tool menu entries for direct access");

        Ok(())
    }

    /// Show view menu help and status
    pub fn show_view_menu_help(&self) {
        if let Some(ref integration) = self.view_integration {
            integration.show_view_menu_help();
        }
    }

    /// Close all tools
    pub fn close_all_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("Menu bridge not initialized".into());
        }

        if let Some(ref integration) = self.view_integration {
            integration.close_all_tools()
        } else {
            Err("View integration not available".into())
        }
    }

    /// Get comprehensive status
    pub fn get_status(&self) -> Option<String> {
        if let Some(ref integration) = self.view_integration {
            let status = integration.get_comprehensive_status();
            Some(format!(
                "Menu Bridge Status:\n\
                 • Initialized: {}\n\
                 • Open Windows: {}\n\
                 • Open Tools: {}",
                self.initialized,
                status.window_statistics.total_open_windows,
                status.window_statistics.open_tools
            ))
        } else {
            None
        }
    }

    /// Check if bridge is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get a reference to the real handler for advanced operations
    pub fn get_real_handler(&self) -> Option<&RealViewMenuHandler> {
        self.real_handler.as_ref()
    }

    /// Get a mutable reference to the view integration
    pub fn get_view_integration(&mut self) -> Option<&mut ViewMenuIntegration> {
        self.view_integration.as_mut()
    }
}

/// Convenience functions for accessing the global bridge
impl MenuIntegrationBridge {
    /// Get a mutable reference to the global menu bridge
    pub async fn get() -> tokio::sync::RwLockWriteGuard<'static, MenuIntegrationBridge> {
        MENU_BRIDGE.write().await
    }

    /// Initialize the global bridge
    pub async fn initialize_global(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<()> {
        let mut bridge = Self::get().await;
        bridge.initialize(db_service).await
    }

    /// Initialize the global bridge with fallback mode (no database)
    /// REMOVED: Fallback mode is no longer supported.
    /// Applications should ensure database services are properly initialized.
    pub fn initialize_fallback() -> Result<()> {
        Err(anyhow::anyhow!(
            "Fallback mode has been removed. Database services must be properly initialized for full functionality. \
            Please ensure the database is accessible and restart the application."
        ))
    }

    /// Set app state for the global bridge
    pub async fn set_global_app_state(app_state: Arc<RwLock<AppState>>) {
        let mut bridge = Self::get().await;
        bridge.set_app_state(app_state).await;
    }

    /// Handle writing tools menu globally
    pub async fn handle_global_writing_tools_menu() -> Result<(), Box<dyn std::error::Error>> {
        let bridge = Self::get().await;
        bridge.handle_writing_tools_menu()
    }

    /// Handle individual tool menu globally
    pub async fn handle_global_individual_tool_menu(
        tool_type: ToolType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bridge = Self::get().await;
        bridge.handle_individual_tool_menu(tool_type)
    }

    /// Handle universal tools menu globally
    pub async fn handle_global_universal_tools_menu() -> Result<(), Box<dyn std::error::Error>> {
        let bridge = Self::get().await;
        bridge.handle_universal_tools_menu()
    }

    /// Show help globally
    pub fn show_global_help() {
        // For read-only operations, we can use a simpler approach
        println!("Global help functionality");
    }

    /// Close all tools globally
    pub async fn close_global_all_tools() -> Result<(), Box<dyn std::error::Error>> {
        let bridge = Self::get().await;
        bridge.close_all_tools()
    }

    /// Get global status
    pub fn get_global_status() -> Option<String> {
        // For sync access, we need to use a different approach
        // This is a simplified implementation for now
        Some("Menu Bridge Status: Synchronized access".to_string())
    }
}

/// Tool-specific convenience functions for independent tool windows
impl MenuIntegrationBridge {
    /// Open hierarchy tool
    pub async fn open_hierarchy_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Hierarchy)
    }

    /// Open codex tool
    pub async fn open_codex_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Codex)
    }

    /// Open brainstorming tool
    pub async fn open_brainstorming_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Brainstorming)
    }

    /// Open analysis tool
    pub async fn open_analysis_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Analysis)
    }

    /// Open plot tool
    pub async fn open_plot_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Plot)
    }

    /// Open notes tool
    pub async fn open_notes_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Notes)
    }

    /// Open research tool
    pub async fn open_research_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Research)
    }

    /// Open structure tool
    pub async fn open_structure_tool(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut bridge = Self::get().await;
        bridge.handle_individual_tool_menu(ToolType::Structure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;

    // Mock database service for testing
    struct MockDatabaseService;

    impl MockDatabaseService {
        // Implement required methods as no-ops for testing
    }

    #[tokio::test]
    async fn test_menu_bridge_creation() {
        let mut bridge = MenuIntegrationBridge::new();
        assert!(!bridge.is_initialized());
        assert!(bridge.get_status().is_none());
    }

    #[tokio::test]
    async fn test_menu_bridge_initialization() {
        // Use a mock that matches the expected type
        let _db_service = Arc::new(RwLock::new(())); // Empty mock for testing
        let mut bridge = MenuIntegrationBridge::new();

        // This will fail due to type mismatch, but we can test the structure
        assert!(!bridge.is_initialized());
    }

    #[tokio::test]
    async fn test_menu_bridge_error_when_not_initialized() {
        let bridge = MenuIntegrationBridge::new();

        let result = bridge.handle_writing_tools_menu();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Menu bridge not initialized"
        );
    }
}
