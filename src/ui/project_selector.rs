//! Project Selector UI Component
//!
//! Provides a comprehensive project management interface that integrates with the database
//! project management services. Includes project creation, selection, management, and
//! real-time statistics display.

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

// Import database types and services from the library root
use crate as hc_lib;
use hc_lib::database_app_state::DatabaseAppState;
use hc_lib::database::models::Project;
use hc_lib::database::project_management::ProjectSettings;
use hc_lib::database::project_management;

/// Project selector UI state and functionality
pub struct ProjectSelector {
    /// Reference to the central database application state
    pub database_state: Arc<RwLock<DatabaseAppState>>,

    /// Currently selected project ID
    pub current_project_id: Option<String>,

    /// List of available projects
    pub available_projects: Vec<Project>,

    /// Project creation form state
    pub project_creation_form: ProjectCreationForm,

    /// UI state for different views
    pub ui_state: ProjectUIState,

    /// Error messages for user feedback
    pub error_message: Option<String>,

    /// Success messages for user feedback
    pub success_message: Option<String>,

    /// Loading state indicator
    pub is_loading: bool,
}

/// Project creation form state
#[derive(Debug, Clone)]
pub struct ProjectCreationForm {
    pub name: String,
    pub description: String,
    pub auto_save_enabled: bool,
    pub backup_enabled: bool,
    pub search_enabled: bool,
    pub theme: String,
    pub font_size: u32,
}

impl Default for ProjectCreationForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            auto_save_enabled: true,
            backup_enabled: true,
            search_enabled: true,
            theme: "default".to_string(),
            font_size: 14,
        }
    }
}

/// UI state for different project management views
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectUIState {
    /// Show project list view
    #[default]
    ProjectList,
    /// Show project creation form
    CreateProject,
    /// Show project details/settings
    ProjectDetails(String), // Project ID
    /// Show project statistics
    ProjectStatistics(String), // Project ID
    /// Show project settings
    ProjectSettings(String), // Project ID
}

impl ProjectSelector {
    /// Create a new project selector
    pub fn new(database_state: Arc<RwLock<DatabaseAppState>>) -> Self {
        Self {
            database_state,
            current_project_id: None,
            available_projects: Vec::new(),
            project_creation_form: ProjectCreationForm::default(),
            ui_state: ProjectUIState::ProjectList,
            error_message: None,
            success_message: None,
            is_loading: false,
        }
    }

    /// Initialize the project selector by loading available projects
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing project selector...");
        self.is_loading = true;

        match self.load_projects().await {
            Ok(()) => {
                info!("Project selector initialized successfully");
                self.is_loading = false;
                Ok(())
            }
            Err(e) => {
                error!("Failed to initialize project selector: {}", e);
                self.is_loading = false;
                self.set_error(format!("Failed to load projects: {}", e).to_string());
                Err(e)
            }
        }
    }

    /// Load all available projects from the database
    pub async fn load_projects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Loading projects from database...");

        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        drop(db_state); // Release the read lock

        // Create project management service from project service
        let project_management = project_service.read().await;

        let projects = project_management
            .get_all_projects()
            .await
            .map_err(|e| format!("Failed to load projects: {}", e))?;

        self.available_projects = projects;
        info!("Loaded {} projects", self.available_projects.len());

        Ok(())
    }

    /// Create a new project
    pub async fn create_project(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        info!("Creating new project: {}", self.project_creation_form.name);

        if self.project_creation_form.name.trim().is_empty() {
            self.set_error("Project name cannot be empty".to_string());
            return Err("Project name cannot be empty".into());
        }

        self.is_loading = true;
        self.clear_messages();

        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        let project_name = self.project_creation_form.name.clone();
        let project_description = if self.project_creation_form.description.trim().is_empty() {
            None
        } else {
            Some(self.project_creation_form.description.clone())
        };

        drop(db_state); // Release the read lock

        // Create project management service from project service
        let project_management = project_service.read().await;

        let project_id = project_management
            .create_project(project_name, project_description)
            .await
            .map_err(|e| format!("Failed to create project: {}", e))?;

        // Update project settings
        self.update_project_settings(&project_id).await?;

        // Reload projects to include the new one
        self.load_projects().await?;

        // Set as current project
        self.current_project_id = Some(project_id.clone());

        self.is_loading = false;
        self.set_success("Project created successfully!".to_string());

        info!("Project created with ID: {}", project_id);
        Ok(project_id)
    }

    /// Set the current active project
    pub async fn set_current_project(
        &mut self,
        project_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Setting current project: {}", project_id);

        self.is_loading = true;
        self.clear_messages();

        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        drop(db_state); // Release the read lock

        // Create project management service from project service
        let project_management = project_service.read().await;

        project_management
            .set_active_project(&Uuid::parse_str(project_id)?)
            .await
            .map_err(|e| format!("Failed to set active project: {}", e))?;

        self.current_project_id = Some(project_id.to_string());
        self.is_loading = false;

        info!("Current project set to: {}", project_id);
        Ok(())
    }

    /// Get statistics for a project
    pub async fn get_project_statistics(
        &self,
        project_id: &str,
    ) -> Result<
        project_management::ProjectStatistics,
        Box<dyn std::error::Error>,
    > {
        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        drop(db_state); // Release the read lock

        // Create project management service from project service
        let project_management = project_service.read().await;

        let statistics = project_management
            .get_project_statistics(&Uuid::parse_str(project_id)?)
            .await
            .map_err(|e| format!("Failed to get project statistics: {}", e))?;

        Ok(statistics)
    }

    /// Get project settings
    pub async fn get_project_settings(
        &self,
        project_id: &str,
    ) -> Result<Option<ProjectSettings>, Box<dyn std::error::Error>> {
        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        drop(db_state); // Release the read lock

        // Create project management service from project service
        let project_management = project_service.read().await;

        let settings = project_management
            .get_project_settings(&Uuid::parse_str(project_id)?)
            .await
            .map_err(|e| format!("Failed to get project settings: {}", e))?;

        Ok(settings)
    }

    /// Update project settings
    pub async fn update_project_settings(
        &self,
        project_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        drop(db_state); // Release the read lock

        let settings = ProjectSettings {
            auto_save_enabled: self.project_creation_form.auto_save_enabled,
            auto_save_interval: 300, // 5 minutes default
            backup_enabled: self.project_creation_form.backup_enabled,
            search_enabled: self.project_creation_form.search_enabled,
            theme: self.project_creation_form.theme.clone(),
            font_size: self.project_creation_form.font_size,
        };

        // Create project management service from project service
        let project_management = project_service.read().await;

        project_management
            .update_project_settings(&Uuid::parse_str(project_id)?, &settings)
            .await
            .map_err(|e| format!("Failed to update project settings: {}", e))?;

        Ok(())
    }

    /// Archive a project
    pub async fn archive_project(
        &mut self,
        project_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Archiving project: {}", project_id);

        self.is_loading = true;
        self.clear_messages();

        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        drop(db_state); // Release the read lock

        // Create project management service from project service
        let project_management = project_service.read().await;

        project_management
            .archive_project(&Uuid::parse_str(project_id)?)
            .await
            .map_err(|e| format!("Failed to archive project: {}", e))?;

        // Reload projects to reflect the change
        self.load_projects().await?;

        self.is_loading = false;
        self.set_success("Project archived successfully!".to_string());

        Ok(())
    }

    /// Delete a project
    pub async fn delete_project(
        &mut self,
        project_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Deleting project: {}", project_id);

        self.is_loading = true;
        self.clear_messages();

        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let project_service = db_state
            .project_service()
            .ok_or("Project service not available")?;

        drop(db_state); // Release the read lock

        // Create project management service from project service
        let project_management = project_service.read().await;

        project_management
            .delete_project(&Uuid::parse_str(project_id)?)
            .await
            .map_err(|e| format!("Failed to delete project: {}", e))?;

        // Reload projects to reflect the change
        self.load_projects().await?;

        // Clear current project if it was deleted
        if self.current_project_id.as_ref() == Some(&project_id.to_string()) {
            self.current_project_id = None;
        }

        self.is_loading = false;
        self.set_success("Project deleted successfully!".to_string());

        Ok(())
    }

    /// Clear error and success messages
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }

    /// Set an error message
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }

    /// Set a success message
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }

    /// Get a project by ID
    pub fn get_project_by_id(&self, project_id: &str) -> Option<&Project> {
        self.available_projects
            .iter()
            .find(|project| project.id.to_string() == project_id)
    }

    /// Check if a project is currently active
    pub fn is_project_active(&self, project_id: &str) -> bool {
        self.current_project_id.as_ref() == Some(&project_id.to_string())
    }

    /// Reset project creation form
    pub fn reset_creation_form(&mut self) {
        self.project_creation_form = ProjectCreationForm::default();
        self.clear_messages();
    }

    /// Validate project creation form
    pub fn validate_creation_form(&self) -> Result<(), String> {
        if self.project_creation_form.name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }

        if self.project_creation_form.name.len() > 100 {
            return Err("Project name cannot exceed 100 characters".to_string());
        }

        if let Some(existing_project) = self
            .available_projects
            .iter()
            .find(|p| p.name == self.project_creation_form.name)
        {
            return Err(format!(
                "A project named '{}' already exists",
                existing_project.name
            ));
        }

        Ok(())
    }
}

impl Default for ProjectSelector {
    fn default() -> Self {
        // This requires a database state, so we'll need to create it with a placeholder
        // In practice, this should be created through the new() method
        Self {
            database_state: Arc::new(RwLock::new(DatabaseAppState::new())),
            current_project_id: None,
            available_projects: Vec::new(),
            project_creation_form: ProjectCreationForm::default(),
            ui_state: ProjectUIState::ProjectList,
            error_message: None,
            success_message: None,
            is_loading: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation_form_default() {
        let form = ProjectCreationForm::default();
        assert_eq!(form.name, "");
        assert_eq!(form.description, "");
        assert!(form.auto_save_enabled);
        assert!(form.backup_enabled);
        assert!(form.search_enabled);
        assert_eq!(form.theme, "default");
        assert_eq!(form.font_size, 14);
    }

    #[test]
    fn test_project_ui_state_defaults() {
        let state = ProjectUIState::default();
        assert_eq!(state, ProjectUIState::ProjectList);
    }

    #[test]
    fn test_validate_creation_form_empty_name() {
        let mut selector = ProjectSelector::default();
        selector.project_creation_form.name = "".to_string();

        let result = selector.validate_creation_form();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Project name cannot be empty");
    }

    #[test]
    fn test_validate_creation_form_too_long() {
        let mut selector = ProjectSelector::default();
        selector.project_creation_form.name = "a".repeat(101);

        let result = selector.validate_creation_form();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Project name cannot exceed 100 characters"
        );
    }

    #[test]
    fn test_validate_creation_form_valid() {
        let mut selector = ProjectSelector::default();
        selector.project_creation_form.name = "Test Project".to_string();

        let result = selector.validate_creation_form();
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_project_active() {
        let mut selector = ProjectSelector::default();
        let project_id = "test-id";
        selector.current_project_id = Some(project_id.to_string());

        assert!(selector.is_project_active(project_id));
        assert!(!selector.is_project_active("other-id"));
    }

    #[test]
    fn test_reset_creation_form() {
        let mut selector = ProjectSelector::default();
        selector.project_creation_form.name = "Test".to_string();
        selector.project_creation_form.description = "Description".to_string();
        selector.set_error("Test error".to_string());

        selector.reset_creation_form();

        assert_eq!(selector.project_creation_form.name, "");
        assert_eq!(selector.project_creation_form.description, "");
        assert!(selector.project_creation_form.auto_save_enabled);
        assert!(selector.error_message.is_none());
    }
}
