//! UI module for Herding Cats Rust Application
//!
//! Contains all UI components, window management, visual elements, and theme management.
//!
//! This module provides:
//! - Centralized theme management across all windows
//! - Unified UI components with theme support
//! - Consistent styling and layout across the application
//! - Theme-aware toolbars and menus
//! - Theme preview and switching interfaces
//! - Window management and persistence
//! - Menu integration bridge for tool launching
//! - Comprehensive Slint logging and debugging

pub mod backup_systems_integration;
pub mod components;
pub mod comprehensive_error_handling;
pub mod document_persistence;
pub mod enhanced_tool_launcher;
pub mod enhanced_window_manager;
pub mod independent_tool_window_manager;
pub mod main_window_persistence;
pub mod menu_integration_bridge;
pub mod project_selector;
pub mod search_integration;
pub mod simplified_window_persistence;
pub mod theme_manager;
pub mod tools;
pub mod unified_toolbar;
pub mod view_menu_integration;
pub mod window_manager;
pub mod window_persistence_integration;
pub mod window_state_persistence;
pub mod font_manager_window;
pub mod generated;

// Re-export theme management types for easier access
// Re-export error handling and logging
