//! Codex Tool UI
//!
//! This module provides the codex tool UI component for managing
//! world building elements and reference materials.
//!
//! NOTE: This file has been updated to remove Egui dependencies and focus on Slint-only implementation.
//! The codex tool UI is now implemented through Slint components in writing_tools_enhanced.slint.

use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::database::models::codex::{CodexEntry, CodexEntryType};
use crate::{ui_state::AppState, DatabaseResult, database::models::codex::{CodexEntry, CodexEntryType}};
use crate::ui::tools::codex_base::CodexToolBase;
use crate::database::models::codex_service::CodexDatabaseService;

/// Codex tool implementation for Slint integration
pub struct CodexTool {
    /// Base codex functionality
    base: CodexToolBase,
    /// Database service for persistence
    database_service: Option<Arc<RwLock<crate::database::EnhancedDatabaseService>>>,
    /// Callback for when codex changes
    on_codex_changed: Option<Box<dyn Fn()>>,
    /// Callback for creating new entries
    on_create_entry: Option<Box<dyn Fn(CodexEntryType, String) -> Result<String, String>>>,
    /// All codex entries
    entries: Vec<CodexEntry>,
}

impl CodexTool {
    /// Create a new codex tool
    pub fn new(db_service: Arc<RwLock<dyn crate::database::models::codex_service::CodexService>>) -> Self {
        Self {
            base: CodexToolBase::new(CodexEntryType::CharacterSheet, db_service),
            database_service: None,
            on_codex_changed: None,
            on_create_entry: None,
            entries: Vec::new(),
        }
    }

    /// Set the database service
    pub fn set_database_service(&mut self, service: Arc<RwLock<crate::database::EnhancedDatabaseService>>) {
        self.database_service = Some(service);
    }

    /// Set callback for codex changes
    pub fn set_codex_changed_callback<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_codex_changed = Some(Box::new(callback));
    }

    /// Set callback for creating new entries
    pub fn set_create_entry_callback<F>(&mut self, callback: F)
    where
        F: Fn(CodexEntryType, String) -> Result<String, String> + 'static,
    {
        self.on_create_entry = Some(Box::new(callback));
    }

    /// Load codex entries from database
    pub async fn load_entries(&mut self, project_id: Uuid) -> DatabaseResult<()> {
        if let Some(db_service) = &self.database_service {
            let entries = db_service.read().await.get_entries_by_project(project_id).await?;
            self.base.entries = entries;

            // Notify listeners of codex change
            if let Some(callback) = &self.on_codex_changed {
                callback();
            }

            Ok(())
        } else {
            Err("Database service not available".to_string())
        }
    }

    /// Get all codex entries
    pub fn get_entries(&self) -> &[CodexEntry] {
        &self.entries
    }

    /// Get entries by type
    pub fn get_entries_by_type(&self, entry_type: CodexEntryType) -> Vec<&CodexEntry> {
        self.entries.iter()
            .filter(|entry| entry.entry_type == entry_type)
            .collect()
    }

    /// Create a new codex entry
    pub async fn create_entry(&mut self, entry_type: CodexEntryType, title: String) -> Result<String, String> {
        let entry = CodexEntry::new(Uuid::new_v4(), entry_type, title.clone(), String::new());
        self.entries.push(entry.clone());

        // Persist to database if available
        if let Some(db_service) = &self.database_service {
            db_service.read().await.create_entry(&entry).await?;
        }

        // Notify listeners of codex change
        if let Some(callback) = &self.on_codex_changed {
            callback();
        }

        Ok(entry.id.to_string())
    }

    /// Update an existing entry
    pub async fn update_entry(&mut self, entry: &CodexEntry) -> DatabaseResult<()> {
        // Find and update the entry
        if let Some(index) = self.entries.iter().position(|e| e.id == entry.id) {
            self.entries[index] = entry.clone();
        }

        // Update database if available
        if let Some(db_service) = &self.database_service {
            db_service.read().await.update_entry(entry).await?;
        }

        // Notify listeners of codex change
        if let Some(callback) = &self.on_codex_changed {
            callback();
        }

        Ok(())
    }

    /// Delete an entry
    pub async fn delete_entry(&mut self, entry_id: Uuid) -> DatabaseResult<()> {
        self.entries.retain(|entry| entry.id != entry_id);

        // Remove from database if available
        if let Some(db_service) = &self.database_service {
            db_service.read().await.delete_entry(entry_id).await?;
        }

        // Notify listeners of codex change
        if let Some(callback) = &self.on_codex_changed {
            callback();
        }

        Ok(())
    }

    /// Search entries
    pub fn search_entries(&self, query: &str) -> Vec<&CodexEntry> {
        self.entries.iter()
            .filter(|entry| entry.title.contains(query) || entry.content.contains(query))
            .collect()
    }

    /// Get entry by ID
    pub fn get_entry(&self, entry_id: Uuid) -> Option<&CodexEntry> {
        self.entries.iter().find(|entry| entry.id == entry_id)
    }

    /// Get mutable entry by ID
    pub fn get_entry_mut(&mut self, entry_id: Uuid) -> Option<&mut CodexEntry> {
        self.entries.iter_mut().find(|entry| entry.id == entry_id)
    }

    /// Export codex data
    pub fn export_codex(&self) -> String {
        format!("Exporting codex with {} entries", self.base.entries.len())
    }

    /// Refresh codex data from database
    pub async fn refresh_codex(&mut self, project_id: Uuid) -> DatabaseResult<()> {
        self.load_entries(project_id).await
    }

    /// Get statistics about codex entries
    pub fn get_entry_statistics(&self) -> std::collections::HashMap<CodexEntryType, usize> {
        let mut stats = std::collections::HashMap::new();

        for entry in &self.base.entries {
            *stats.entry(entry.entry_type).or_insert(0) += 1;
        }

        stats
    }
}

impl Default for CodexTool {
    fn default() -> Self {
        // Create a mock database service for default implementation
        let mock_db_service = Arc::new(RwLock::new(crate::ui::tools::codex_base::MockCodexService));
        Self::new(mock_db_service)
    }
}

/// Sub-tool interface for different codex entry types
pub trait CodexSubTool {
    /// Render the sub-tool UI
    /// NOTE: This interface has been updated for Slint-only implementation
    fn render(&mut self, base: &mut CodexTool) -> DatabaseResult<()>;
}

// Note: All Egui-based sub-tool implementations have been removed to focus on Slint
// The specific entry types (StorySummary, CharacterSheet, etc.) are now handled through Slint components
