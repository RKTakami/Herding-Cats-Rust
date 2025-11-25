//! Generated UI module for Herding Cats Rust Application
//!
//! This module re-exports types generated from .slint files during the build process.
//! It provides a consistent interface for the application to access Slint-generated types.

slint::include_modules!();

// Re-export all generated types from the Slint compilation
// These are generated automatically by the slint::include_modules!() macro

// Font Manager types
pub use font_manager_ui::FontManagerWindow;
pub use font_manager_ui::FontItem;
pub use font_manager_ui::FontCollection;

// Main application windows
pub use main_window_comprehensive::MainWindowComprehensiveWindow;

// Tool windows
pub use brainstorming_tool::BrainstormingTool;
pub use codex_window::CodexWindow;
pub use hierarchy_window::HierarchyWindow;
pub use universal_tools_ui::UniversalWritingToolsWindow;
pub use writing_tools::HierarchyTool as BaseHierarchyTool;
pub use writing_tools::CodexTool as BaseCodexTool;
pub use writing_tools::PlotTool as BasePlotTool;
pub use writing_tools::AnalysisTool as BaseAnalysisTool;
pub use writing_tools::NotesTool as BaseNotesTool;
pub use writing_tools::ResearchTool as BaseResearchTool;
pub use writing_tools::StructureTool as BaseStructureTool;

// Enhanced tool windows
pub use writing_tools_enhanced::HierarchyToolEnhanced;
pub use writing_tools_enhanced::CodexToolEnhanced;
pub use writing_tools_enhanced::PlotToolEnhanced;
pub use writing_tools_enhanced::AnalysisToolEnhanced;
pub use writing_tools_enhanced::NotesToolEnhanced;
pub use writing_tools_enhanced::ResearchToolEnhanced;
pub use writing_tools_enhanced::StructureToolEnhanced;

// UI components
pub use styles::StyleConfiguration;
pub use project_selector::ProjectSelector;
