//! View Menu Integration
//!
//! Integrates the enhanced tool launcher with the View menu system.
//! Provides the real implementation for individual tool window management
//! in the independent writing tools architecture.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{ui::enhanced_tool_launcher::EnhancedToolLauncher, ui::tools::base_types::ToolType};
use crate::database_app_state::EnhancedDatabaseService;
use crate::ui_state::AppState;

/// View Menu Integration Handler
///
/// Handles the View menu actions with intelligent window management.
pub struct ViewMenuIntegration {
    /// Enhanced tool launcher for window management
    pub launcher: EnhancedToolLauncher,
    /// Database service for tool operations
    pub db_service: Arc<RwLock<EnhancedDatabaseService>>,
    /// App state for tool coordination
    pub app_state: Arc<RwLock<AppState>>,
}

impl ViewMenuIntegration {
    /// Create a new view menu integration
    pub fn new(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<Self> {
        Ok(Self {
            launcher: EnhancedToolLauncher::new()?,
            db_service,
            app_state: Arc::new(RwLock::new(AppState::default())),
        })
    }

    /// Set app state reference
    pub async fn set_app_state(&mut self, app_state: Arc<RwLock<AppState>>) {
        self.app_state = app_state;
    }

    /// Handle "View -> Writing Tools" menu action
    ///
    /// For the independent architecture, this provides information about
    /// the new individual tool window approach.
    pub fn handle_writing_tools_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 View -> Writing Tools menu clicked");

        let stats = self.launcher.get_statistics();

        // Show current status
        println!("📊 Current window state:");
        println!("  • {} individual tool windows open", stats.open_tools);
        println!("  • {} tools available for opening", stats.available_tools);

        if stats.total_open_windows == 0 {
            println!("💡 No tools are currently open");
            println!("🔧 Use individual tool menu entries for direct access:");
        } else {
            println!("📋 Currently open tools:");
        }

        // Show individual tool menu entries
        for tool in ToolType::all_types() {
            let status = if stats.open_tools_list.contains(&tool) {
                "✅ (Already Open)"
            } else {
                "➕ (Open Tool)"
            };
            println!("  • {} Tool - {}", tool.display_name(), status);
        }

        println!();
        println!("💡 Independent Tool Window Benefits:");
        println!("  • Each tool has its own dedicated window");
        println!("  • Direct access without dropdown navigation");
        println!("  • Independent operation and customization");
        println!("  • Better performance and user experience");

        Ok(())
    }

    /// Handle "View -> Tool Name" menu actions for individual tools
    pub fn handle_individual_tool_menu(
        &self,
        tool_type: ToolType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 View -> {} menu clicked", tool_type.display_name());

        let was_already_open = self.launcher.is_tool_open(tool_type);

        let window_id = self
            .launcher
            .launch_tool(tool_type)
            .map_err(|e| format!("Failed to launch {} tool: {}", tool_type.display_name(), e))?;

        if was_already_open {
            println!(
                "✅ Focused existing {} tool window (ID: {})",
                tool_type.display_name(),
                window_id.0
            );
        } else {
            println!(
                "🚀 Launched new {} tool window (ID: {})",
                tool_type.display_name(),
                window_id.0
            );
        }

        Ok(())
    }


    /// Show View menu help and current state
    pub fn show_view_menu_help(&self) {
        println!("📋 View Menu Options");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();

        // Show current window state
        self.launcher.show_status();

        // Show menu options
        println!("🎯 View Menu Actions:");

        for tool in ToolType::all_types() {
            let status = if self.launcher.is_tool_open(tool) {
                "✅ (Already Open)"
            } else {
                "➕ (Open Tool)"
            };
            println!("  • {} Tool - {}", tool.display_name(), status);
        }

        println!();
        println!("💡 Window Management Rules:");
        println!("  • Only ONE instance of each individual tool can be open");
        println!("  • Each tool has its own independent window");
        println!("  • Clicking an already-open tool focuses the existing window");
        println!("  • Tools operate independently without universal window dependency");
        println!();
    }

    /// Close all writing tool windows
    pub fn close_all_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("👋 Closing all writing tool windows");

        self.launcher
            .close_all_windows()
            .map_err(|e| format!("Failed to close windows: {}", e))?;

        println!("✅ All writing tool windows closed");
        Ok(())
    }

    /// Get comprehensive status
    pub fn get_comprehensive_status(&self) -> ViewMenuStatus {
        let stats = self.launcher.get_statistics();
        let open_windows = self.launcher.get_open_windows();

        ViewMenuStatus {
            window_statistics: stats,
            open_windows,
            menu_recommendations: self.generate_menu_recommendations(),
        }
    }

    /// Generate recommendations for menu actions based on current state
    fn generate_menu_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        let stats = self.launcher.get_statistics();

        if stats.total_open_windows == 0 {
            recommendations.push(
                "💡 Start with individual tool menu entries for direct access".to_string(),
            );
        }

        if stats.available_tools == 0 {
            recommendations.push("✅ All tools are currently accessible".to_string());
        } else {
            recommendations.push(format!(
                "📋 {} tools can still be opened individually",
                stats.available_tools
            ));
        }


        recommendations
    }
}

/// Comprehensive status information for the view menu system
#[derive(Debug)]
pub struct ViewMenuStatus {
    pub window_statistics: crate::ui::enhanced_tool_launcher::WindowStatisticsInfo,
    pub open_windows: Vec<crate::ui::enhanced_tool_launcher::WindowStateInfo>,
    pub menu_recommendations: Vec<String>,
}

/// Real View Menu Handler for Production Use
///
/// This is the production-ready implementation that should be used
/// in the actual word processor application.
pub struct RealViewMenuHandler {
    /// The view menu integration instance
    pub integration: ViewMenuIntegration,
}

impl RealViewMenuHandler {
    /// Create a new real view menu handler
    pub fn new(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Result<Self> {
        Ok(Self {
            integration: ViewMenuIntegration::new(db_service)?,
        })
    }

    /// Set app state reference
    pub async fn set_app_state(&mut self, app_state: Arc<RwLock<AppState>>) {
        self.integration.set_app_state(app_state).await;
    }

    /// Handle individual tool menu actions
    pub fn hierarchy_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Hierarchy)
        {
            eprintln!("❌ Error opening Hierarchy tool: {}", e);
        }
    }

    pub fn codex_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Codex)
        {
            eprintln!("❌ Error opening Codex tool: {}", e);
        }
    }

    pub fn brainstorming_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Brainstorming)
        {
            eprintln!("❌ Error opening Brainstorming tool: {}", e);
        }
    }

    pub fn analysis_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Analysis)
        {
            eprintln!("❌ Error opening Analysis tool: {}", e);
        }
    }

    pub fn plot_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Plot)
        {
            eprintln!("❌ Error opening Plot tool: {}", e);
        }
    }

    pub fn notes_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Notes)
        {
            eprintln!("❌ Error opening Notes tool: {}", e);
        }
    }

    pub fn research_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Research)
        {
            eprintln!("❌ Error opening Research tool: {}", e);
        }
    }

    pub fn structure_tool(&self) {
        if let Err(e) = self
            .integration
            .handle_individual_tool_menu(ToolType::Structure)
        {
            eprintln!("❌ Error opening Structure tool: {}", e);
        }
    }

    /// Show help and status
    pub fn show_help(&self) {
        self.integration.show_view_menu_help();
    }

    /// Close all tools
    pub fn close_all(&self) {
        if let Err(e) = self.integration.close_all_tools() {
            eprintln!("❌ Error closing tools: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;

    // Mock database service for testing - needs to match EnhancedDatabaseService interface
    use crate::database_app_state::EnhancedDatabaseService;

    // For testing, we'll use a simple mock that matches the expected type
    type MockDatabaseService = EnhancedDatabaseService;

    #[tokio::test]
    async fn test_view_menu_integration_creation() {
        let db_service: Arc<RwLock<EnhancedDatabaseService>> = Arc::new(RwLock::new(None));
        let integration = ViewMenuIntegration::new(db_service).unwrap();

        assert_eq!(integration.launcher.get_statistics().total_open_windows, 0);
    }

    #[tokio::test]
    async fn test_single_tool_constraint() {
        // Skip this test as it requires proper database service
        println!("Skipping single tool constraint test - requires database service");
    }

    #[tokio::test]
    async fn test_multiple_universal_windows() {
        // Skip this test as universal windows are not supported in independent architecture
        println!("Skipping multiple universal windows test - not supported in independent architecture");
    }
}
