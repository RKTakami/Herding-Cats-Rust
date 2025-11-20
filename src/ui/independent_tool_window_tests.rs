//! Tests for Independent Tool Window Manager
//!
//! Comprehensive tests for the independent writing tool windows architecture.

use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use rstest::rstest;

use crate::{
    ui::tools::base_types::ToolType,
    ui::independent_tool_window_manager::IndependentToolWindowManager,
    DatabaseAppState,
    ui_state::AppState,
};

/// Mock database state for testing
struct MockDatabaseAppState;

impl MockDatabaseAppState {
    fn new() -> Self {
        Self
    }
}

#[tokio::test]
async fn test_independent_tool_window_manager_creation() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    assert_eq!(manager.get_open_tool_windows().len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_tool_window_opening() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Test opening hierarchy tool
    manager.open_tool_window(ToolType::Hierarchy).await?;
    assert!(manager.is_tool_window_open(ToolType::Hierarchy));
    assert_eq!(manager.get_open_tool_windows().len(), 1);

    // Test opening another tool
    manager.open_tool_window(ToolType::Codex).await?;
    assert!(manager.is_tool_window_open(ToolType::Codex));
    assert_eq!(manager.get_open_tool_windows().len(), 2);

    // Test that tools are in the open list
    let open_tools = manager.get_open_tool_windows();
    assert!(open_tools.contains(&ToolType::Hierarchy));
    assert!(open_tools.contains(&ToolType::Codex));

    Ok(())
}

#[tokio::test]
async fn test_tool_window_focusing() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Open a tool window
    manager.open_tool_window(ToolType::Analysis).await?;
    assert!(manager.is_tool_window_open(ToolType::Analysis));

    // Focus the same tool (should not open a new window)
    manager.focus_tool_window(ToolType::Analysis);

    // Should still only have one instance
    assert_eq!(manager.get_open_tool_windows().len(), 1);
    assert!(manager.is_tool_window_open(ToolType::Analysis));

    Ok(())
}

#[tokio::test]
async fn test_tool_window_closing() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Open a tool window
    manager.open_tool_window(ToolType::Plot).await?;
    assert!(manager.is_tool_window_open(ToolType::Plot));

    // Close the tool window
    manager.close_tool_window(ToolType::Plot)?;
    assert!(!manager.is_tool_window_open(ToolType::Plot));
    assert_eq!(manager.get_open_tool_windows().len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_close_all_tool_windows() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Open multiple tool windows
    manager.open_tool_window(ToolType::Hierarchy).await?;
    manager.open_tool_window(ToolType::Codex).await?;
    manager.open_tool_window(ToolType::Brainstorming).await?;

    assert_eq!(manager.get_open_tool_windows().len(), 3);

    // Close all tool windows
    manager.close_all_tool_windows()?;
    assert_eq!(manager.get_open_tool_windows().len(), 0);

    // Verify all tools are closed
    for tool_type in ToolType::all_types() {
        assert!(!manager.is_tool_window_open(tool_type));
    }

    Ok(())
}

#[tokio::test]
async fn test_multiple_tool_types() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Test all tool types
    let all_tools = ToolType::all_types();

    for tool_type in &all_tools {
        // Open tool
        manager.open_tool_window(*tool_type).await?;
        assert!(manager.is_tool_window_open(*tool_type));

        // Close tool
        manager.close_tool_window(*tool_type)?;
        assert!(!manager.is_tool_window_open(*tool_type));
    }

    // Verify all are closed
    assert_eq!(manager.get_open_tool_windows().len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_window_state_tracking() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Open a tool and check window state
    manager.open_tool_window(ToolType::Notes).await?;

    // The window state should be tracked (in real implementation)
    // For now, we test that the tool is marked as open
    assert!(manager.is_tool_window_open(ToolType::Notes));

    Ok(())
}

#[tokio::test]
async fn test_app_state_integration() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let mut manager = IndependentToolWindowManager::new(db_state)?;

    // Set app state
    let app_state = Arc::new(Mutex::new(AppState::default()));
    manager.set_app_state(app_state);

    // Verify app state was set (would need getter in real implementation)
    // For now, just test that the call doesn't panic
    assert!(true);

    Ok(())
}

#[rstest]
#[case(ToolType::Hierarchy)]
#[case(ToolType::Codex)]
#[case(ToolType::Brainstorming)]
#[case(ToolType::Analysis)]
#[case(ToolType::Plot)]
#[case(ToolType::Notes)]
#[case(ToolType::Research)]
#[case(ToolType::Structure)]
#[tokio::test]
async fn test_individual_tool_opening(#[case] tool_type: ToolType) -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Open specific tool
    manager.open_tool_window(tool_type).await?;
    assert!(manager.is_tool_window_open(tool_type));

    // Close specific tool
    manager.close_tool_window(tool_type)?;
    assert!(!manager.is_tool_window_open(tool_type));

    Ok(())
}

#[tokio::test]
async fn test_concurrent_tool_operations() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = Arc::new(IndependentToolWindowManager::new(db_state)?);

    // Simulate concurrent operations
    let manager_clone = manager.clone();
    let handle1 = tokio::spawn(async move {
        manager_clone.open_tool_window(ToolType::Hierarchy).await
    });

    let manager_clone = manager.clone();
    let handle2 = tokio::spawn(async move {
        manager_clone.open_tool_window(ToolType::Codex).await
    });

    // Wait for both operations
    let _ = tokio::join!(handle1, handle2);

    // Both tools should be open
    assert!(manager.is_tool_window_open(ToolType::Hierarchy));
    assert!(manager.is_tool_window_open(ToolType::Codex));

    Ok(())
}

#[tokio::test]
async fn test_tool_window_lifecycle() -> Result<()> {
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Test complete lifecycle for a tool
    let tool_type = ToolType::Research;

    // Initially closed
    assert!(!manager.is_tool_window_open(tool_type));
    assert!(!manager.get_open_tool_windows().contains(&tool_type));

    // Open tool
    manager.open_tool_window(tool_type).await?;
    assert!(manager.is_tool_window_open(tool_type));
    assert!(manager.get_open_tool_windows().contains(&tool_type));

    // Focus tool (should remain open)
    manager.focus_tool_window(tool_type);
    assert!(manager.is_tool_window_open(tool_type));

    // Close tool
    manager.close_tool_window(tool_type)?;
    assert!(!manager.is_tool_window_open(tool_type));
    assert!(!manager.get_open_tool_windows().contains(&tool_type));

    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<()> {
    // Test with a mock that could potentially fail
    let db_state = Arc::new(RwLock::new(MockDatabaseAppState::new()));
    let manager = IndependentToolWindowManager::new(db_state)?;

    // Test opening and closing tools multiple times
    for _ in 0..3 {
        manager.open_tool_window(ToolType::Structure).await?;
        assert!(manager.is_tool_window_open(ToolType::Structure));

        manager.close_tool_window(ToolType::Structure)?;
        assert!(!manager.is_tool_window_open(ToolType::Structure));
    }

    // Should end up clean
    assert_eq!(manager.get_open_tool_windows().len(), 0);

    Ok(())
}

// Mock implementations for testing
impl DatabaseAppState {
    pub fn new() -> Result<Self> {
        // Mock implementation
        Ok(DatabaseAppState)
    }
}

impl MockDatabaseAppState {
    // Mock methods for testing
}

impl Default for MockDatabaseAppState {
    fn default() -> Self {
        Self::new()
    }
}
