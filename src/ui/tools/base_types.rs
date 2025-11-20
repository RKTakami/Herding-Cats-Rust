//! Base Types for UI Tools
//!
//! Centralized type definitions for the UI tools system to avoid import conflicts.

use crate::DatabaseAppState;
use serde::{Deserialize, Serialize};
use tokio::sync;

/// Types of tools in the system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolType {
    /// Hierarchy tool for managing document structure
    Hierarchy,
    /// Codex tool for managing knowledge base
    Codex,
    /// Brainstorming tool for creative thinking and idea generation
    Brainstorming,
    /// Analysis tool for data analysis
    Analysis,
    /// Plot tool for story plotting and narrative structure
    Plot,
    /// Notes tool for note taking and organization
    Notes,
    /// Research tool for research organization and management
    Research,
    /// Structure tool for data structure management
    Structure,
}

impl ToolType {
    /// Get the display name for the tool type
    pub fn display_name(&self) -> &'static str {
        match self {
            ToolType::Hierarchy => "Hierarchy",
            ToolType::Codex => "Codex",
            ToolType::Brainstorming => "Brainstorming",
            ToolType::Analysis => "Analysis",
            ToolType::Plot => "Plot",
            ToolType::Notes => "Notes",
            ToolType::Research => "Research",
            ToolType::Structure => "Structure",
        }
    }

    /// Get all tool types as a vector
    pub fn all_types() -> Vec<ToolType> {
        vec![
            ToolType::Hierarchy,
            ToolType::Codex,
            ToolType::Brainstorming,
            ToolType::Analysis,
            ToolType::Plot,
            ToolType::Notes,
            ToolType::Research,
            ToolType::Structure,
        ]
    }
}

/// Tool manager for coordinating multiple tools
pub struct ToolManager {
    /// Database state for the manager
    pub database_state: std::sync::Arc<sync::RwLock<DatabaseAppState>>,
}

impl ToolManager {
    /// Create a new tool manager
    pub async fn new(
        database_state: std::sync::Arc<sync::RwLock<DatabaseAppState>>,
    ) -> anyhow::Result<Self> {
        Ok(Self { database_state })
    }
}
