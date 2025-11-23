//! Base Types for UI Tools
//!
//! Centralized type definitions for the UI tools system to avoid import conflicts.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::DatabaseAppState;
use anyhow::Result;

/// Tool types available in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolType {
    Hierarchy,
    Codex,
    Plot,
    Analysis,
    Notes,
    Research,
    Brainstorming,
    Structure,
}

impl ToolType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ToolType::Hierarchy => "Hierarchy",
            ToolType::Codex => "Codex",
            ToolType::Plot => "Plot",
            ToolType::Analysis => "Analysis",
            ToolType::Notes => "Notes",
            ToolType::Research => "Research",
            ToolType::Brainstorming => "Brainstorming",
            ToolType::Structure => "Structure",
        }
    }

    pub fn all_types() -> Vec<ToolType> {
        vec![
            ToolType::Hierarchy,
            ToolType::Codex,
            ToolType::Plot,
            ToolType::Analysis,
            ToolType::Notes,
            ToolType::Research,
            ToolType::Brainstorming,
            ToolType::Structure,
        ]
    }
}

/// Manager for tool instances
pub struct ToolManager {
    database_state: Arc<RwLock<DatabaseAppState>>,
}

impl ToolManager {
    pub async fn new(database_state: Arc<RwLock<DatabaseAppState>>) -> Result<Self> {
        Ok(Self { database_state })
    }
}
