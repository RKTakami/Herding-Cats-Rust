//! Document Persistence System
//!
//! Provides comprehensive document persistence with project state management,
//! auto-save functionality, and cross-tool synchronization. Integrates with
//! the database EnhancedDatabaseService for reliable document storage.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info, warn};

// Import database types and services
use crate::DatabaseAppState;

/// Document state for tracking changes and persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentState {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub content_hash: String,
    pub last_modified: u64,
    pub is_dirty: bool,
    pub is_saved: bool,
    pub auto_save_enabled: bool,
    pub version: u64,
    pub metadata: HashMap<String, String>,
}

impl DocumentState {
    /// Create a new document state
    pub fn new(id: String, project_id: String, title: String, content: String) -> Self {
        let content_hash = Self::calculate_content_hash(&content);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            project_id,
            title,
            content,
            content_hash,
            last_modified: now,
            is_dirty: false,
            is_saved: false,
            auto_save_enabled: true,
            version: 1,
            metadata: HashMap::new(),
        }
    }

    /// Update document content and mark as dirty
    pub fn update_content(&mut self, new_content: String) {
        let new_hash = Self::calculate_content_hash(&new_content);
        if new_hash != self.content_hash {
            self.content = new_content;
            self.content_hash = new_hash;
            self.is_dirty = true;
            self.last_modified = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            self.version += 1;
        }
    }

    /// Update document title
    pub fn update_title(&mut self, new_title: String) {
        if new_title != self.title {
            self.title = new_title;
            self.is_dirty = true;
            self.last_modified = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }
    }

    /// Mark document as saved
    pub fn mark_saved(&mut self) {
        self.is_dirty = false;
        self.is_saved = true;
    }

    /// Check if content has changed
    pub fn has_changed(&self, new_content: &str) -> bool {
        Self::calculate_content_hash(new_content) != self.content_hash
    }

    /// Calculate SHA-256 hash of content
    fn calculate_content_hash(content: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Document persistence manager
pub struct DocumentPersistenceManager {
    /// Reference to the central database application state
    pub database_state: Arc<RwLock<DatabaseAppState>>,

    /// Active document states by document ID
    pub active_documents: Arc<RwLock<HashMap<String, DocumentState>>>,

    /// Auto-save configuration
    pub auto_save_interval: std::time::Duration,

    /// Auto-save task handle
    pub auto_save_task: Option<tokio::task::JoinHandle<()>>,

    /// Auto-save channel for triggering saves
    pub auto_save_sender: Option<mpsc::UnboundedSender<String>>,

    /// Error messages for user feedback
    pub error_message: Option<String>,

    /// Success messages for user feedback
    pub success_message: Option<String>,

    /// Loading state indicator
    pub is_loading: bool,

    /// Event callbacks for UI updates
    pub on_document_saved: Option<Box<dyn Fn(&str) + Send + Sync>>,
    pub on_document_changed: Option<Box<dyn Fn(&str) + Send + Sync>>,
    pub on_auto_save_triggered: Option<Box<dyn Fn(&str) + Send + Sync>>,
}

impl DocumentPersistenceManager {
    /// Create a new document persistence manager
    ///
    /// Note:
    /// - Auto-save no longer spawns a background task that touches the database directly.
    /// - Instead, `auto_save_all` can be driven externally (e.g. via a timer in the main
    ///   application) to avoid non-Send futures in `tokio::spawn`.
    pub fn new(database_state: Arc<RwLock<DatabaseAppState>>) -> Self {
        Self {
            database_state,
            active_documents: Arc::new(RwLock::new(HashMap::new())),
            auto_save_interval: std::time::Duration::from_secs(300), // 5 minutes default
            auto_save_task: None,
            auto_save_sender: None,
            error_message: None,
            success_message: None,
            is_loading: false,
            on_document_saved: None,
            on_document_changed: None,
            on_auto_save_triggered: None,
        }
    }

    /// Initialize the document persistence manager
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing document persistence manager...");
        self.is_loading = true;

        // Load existing documents for current project
        if let Some(current_project_id) = self.get_current_project_id().await {
            self.load_project_documents(&current_project_id).await?;
        }

        self.is_loading = false;
        info!("Document persistence manager initialized successfully");
        Ok(())
    }

    /// Get current project ID from database state
    async fn get_current_project_id(&self) -> Option<String> {
        let db_state = self.database_state.read().await;
        db_state
            .get_current_project()
            .map(|project| project.to_string())
    }

    /// Load all documents for a project
    pub async fn load_project_documents(
        &self,
        project_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Loading documents for project: {}", project_id);

        let db_state = self.database_state.read().await;

        if !db_state.is_database_ready() {
            return Err("Database not ready".into());
        }

        let db_service = db_state
            .database_service()
            .ok_or("Database service not available")?;

        drop(db_state); // Release the read lock

        // Get current timestamp for auto-save
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // For now, create a sample document state
        // In a real implementation, this would query the database
        let sample_document = DocumentState::new(
            "doc-1".to_string(),
            project_id.to_string(),
            "Untitled Document".to_string(),
            "Start writing your story here...".to_string(),
        );

        let mut documents = self.active_documents.write().await;
        documents.insert(sample_document.id.clone(), sample_document);

        Ok(())
    }

    /// Create a new document
    pub async fn create_document(
        &self,
        title: String,
        content: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!("Creating new document: {}", title);

        let document_id = uuid::Uuid::new_v4().to_string();
        let project_id = self
            .get_current_project_id()
            .await
            .ok_or("No active project")?;

        let document_state = DocumentState::new(document_id.clone(), project_id, title, content);

        // Save to database
        let db_state = self.database_state.read().await;
        let db_service = db_state
            .database_service()
            .ok_or("Database service not available")?;

        drop(db_state); // Release the read lock

        db_service
            .read()
            .await
            .create_document(
                document_id.clone(),
                document_state.project_id.clone(),
                document_state.title.clone(),
                document_state.content.clone(),
            )
            .await
            .map_err(|e| format!("Failed to create document in database: {}", e))?;

        // Add to active documents
        let mut documents = self.active_documents.write().await;
        documents.insert(document_id.clone(), document_state);

        if let Some(callback) = &self.on_document_saved {
            callback(&document_id);
        }

        info!("Document created with ID: {}", document_id);
        Ok(document_id)
    }

    /// Load a specific document
    pub async fn load_document(
        &self,
        document_id: &str,
    ) -> Result<DocumentState, Box<dyn std::error::Error>> {
        info!("Loading document: {}", document_id);

        // Check if already loaded
        {
            let documents = self.active_documents.read().await;
            if let Some(state) = documents.get(document_id) {
                return Ok(state.clone());
            }
        }

        // Load from database
        let db_state = self.database_state.read().await;
        let db_service = db_state
            .database_service()
            .ok_or("Database service not available")?;

        drop(db_state); // Release the read lock

        let document_data = db_service
            .read()
            .await
            .get_document(document_id.to_string())
            .await
            .map_err(|e| format!("Failed to load document from database: {}", e))?;

        match document_data {
            Some(document_content) => {
                // Parse document content (in real implementation, this would be structured)
                let document_state = DocumentState::new(
                    document_id.to_string(),
                    "current-project".to_string(), // Would come from database
                    "Untitled Document".to_string(), // Would come from database
                    document_content,
                );

                // Add to active documents
                let mut documents = self.active_documents.write().await;
                documents.insert(document_id.to_string(), document_state.clone());

                Ok(document_state)
            }
            None => Err(format!("Document {} not found", document_id).into()),
        }
    }

    /// Save a document
    pub async fn save_document(&self, document_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Saving document: {}", document_id);

        let mut documents = self.active_documents.write().await;
        let document_state = documents.get_mut(document_id).ok_or(format!(
            "Document {} not found in active documents",
            document_id
        ))?;

        if !document_state.is_dirty {
            debug!("Document {} is not dirty, skipping save", document_id);
            return Ok(());
        }

        // Save to database
        let db_state = self.database_state.read().await;
        let db_service = db_state
            .database_service()
            .ok_or("Database service not available")?;

        drop(db_state); // Release the read lock

        db_service
            .read()
            .await
            .update_document(
                document_id.to_string(),
                document_state.title.clone(),
                document_state.content.clone(),
            )
            .await
            .map_err(|e| format!("Failed to save document to database: {}", e))?;

        // Mark as saved
        document_state.mark_saved();

        if let Some(callback) = &self.on_document_saved {
            callback(document_id);
        }

        info!("Document {} saved successfully", document_id);
        Ok(())
    }

    /// Auto-save all dirty documents
    pub async fn auto_save_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Performing auto-save for all documents");

        let documents = {
            let docs = self.active_documents.read().await;
            docs.iter()
                .filter(|(_, state)| state.is_dirty && state.auto_save_enabled)
                .map(|(id, _)| id.clone())
                .collect::<Vec<String>>()
        };

        for document_id in documents {
            if let Err(e) = self.save_document(&document_id).await {
                warn!("Auto-save failed for document {}: {}", document_id, e);
            } else if let Some(callback) = &self.on_auto_save_triggered {
                callback(&document_id);
            }
        }

        Ok(())
    }

    /// Update document content
    pub async fn update_document_content(
        &self,
        document_id: &str,
        new_content: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut documents = self.active_documents.write().await;
        let document_state = documents.get_mut(document_id).ok_or(format!(
            "Document {} not found in active documents",
            document_id
        ))?;

        document_state.update_content(new_content);

        if let Some(callback) = &self.on_document_changed {
            callback(document_id);
        }

        // Auto-save is now driven externally via `auto_save_all` on a timer owned by the app,
        // avoiding non-Send futures in internal spawns.
        Ok(())
    }

    /// Update document title
    pub async fn update_document_title(
        &self,
        document_id: &str,
        new_title: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut documents = self.active_documents.write().await;
        let document_state = documents.get_mut(document_id).ok_or(format!(
            "Document {} not found in active documents",
            document_id
        ))?;

        document_state.update_title(new_title);

        if let Some(callback) = &self.on_document_changed {
            callback(document_id);
        }

        Ok(())
    }

    /// Get document state
    pub async fn get_document_state(&self, document_id: &str) -> Option<DocumentState> {
        let documents = self.active_documents.read().await;
        documents.get(document_id).cloned()
    }

    /// Check if document has unsaved changes
    pub async fn has_unsaved_changes(&self, document_id: &str) -> bool {
        let documents = self.active_documents.read().await;
        documents
            .get(document_id)
            .map(|state| state.is_dirty)
            .unwrap_or(false)
    }

    /// Get all active documents for current project
    pub async fn get_active_documents(&self) -> Vec<DocumentState> {
        let documents = self.active_documents.read().await;
        documents.values().cloned().collect()
    }

    /// Close a document
    pub async fn close_document(
        &self,
        document_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Closing document: {}", document_id);

        let mut documents = self.active_documents.write().await;
        documents.remove(document_id);

        Ok(())
    }

    /// Close all documents
    pub async fn close_all_documents(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Closing all documents");

        let mut documents = self.active_documents.write().await;
        documents.clear();

        Ok(())
    }

    /// Set auto-save interval
    pub fn set_auto_save_interval(&mut self, interval: std::time::Duration) {
        self.auto_save_interval = interval;
    }

    /// Enable/disable auto-save for a document
    pub async fn set_auto_save_enabled(
        &self,
        document_id: &str,
        enabled: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut documents = self.active_documents.write().await;
        if let Some(state) = documents.get_mut(document_id) {
            state.auto_save_enabled = enabled;
        } else {
            return Err(format!("Document {} not found", document_id).into());
        }

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

    /// Set event callbacks
    pub fn set_callbacks(
        &mut self,
        on_document_saved: Option<Box<dyn Fn(&str) + Send + Sync>>,
        on_document_changed: Option<Box<dyn Fn(&str) + Send + Sync>>,
        on_auto_save_triggered: Option<Box<dyn Fn(&str) + Send + Sync>>,
    ) {
        self.on_document_saved = on_document_saved;
        self.on_document_changed = on_document_changed;
        self.on_auto_save_triggered = on_auto_save_triggered;
    }
}

impl Drop for DocumentPersistenceManager {
    fn drop(&mut self) {
        // No background auto-save task to clean up anymore.
        // Auto-save should be orchestrated by the application using `auto_save_all`.
    }
}
