//! Brainstorming Base Module
//!
//! Core functionality for the Brainstorming tool including state management,
//! event handling, and integration with the main application.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::ui::tools::brainstorming_data::{*, NodeType, ConnectionType, LayoutAlgorithm};
use crate::{database::{DatabaseService, DatabaseResult}};

/// Brainstorming tool UI state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormingUiState {
    pub selected_node: Option<Uuid>,
    pub hovered_node: Option<Uuid>,
    pub selected_connection: Option<Uuid>,
    pub dragged_node: Option<Uuid>,
    pub view_mode: BrainstormingViewMode,
    pub show_grid: bool,
    pub show_node_labels: bool,
    pub show_connection_labels: bool,
    pub search_query: String,
    pub filter_node_types: Vec<NodeType>,
    pub zoom_level: f32,
    pub pan_offset: (f32, f32),
    pub is_dragging: bool,
    pub drag_start_position: Option<(f32, f32)>,
}

impl Default for BrainstormingUiState {
    fn default() -> Self {
        Self {
            selected_node: None,
            hovered_node: None,
            selected_connection: None,
            dragged_node: None,
            view_mode: BrainstormingViewMode::Edit,
            show_grid: false,
            show_node_labels: true,
            show_connection_labels: false,
            search_query: String::new(),
            filter_node_types: vec![
                NodeType::Central,
                NodeType::Primary,
                NodeType::Secondary,
                NodeType::Detail,
                NodeType::Note,
            ],
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
            is_dragging: false,
            drag_start_position: None,
        }
    }
}

/// Brainstorming view modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrainstormingViewMode {
    /// Edit the mindmap structure
    Edit,
    /// Present the mindmap
    Present,
    /// Export/Import mode
    Export,
}

impl BrainstormingViewMode {
    /// Get the display name
    pub fn display_name(&self) -> &'static str {
        match self {
            BrainstormingViewMode::Edit => "Edit",
            BrainstormingViewMode::Present => "Present",
            BrainstormingViewMode::Export => "Export",
        }
    }
}

/// Brainstorming tool events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrainstormingEvent {
    /// A node was selected
    NodeSelected(Uuid),
    /// A node was created
    NodeCreated(Uuid),
    /// A node was deleted
    NodeDeleted(Uuid),
    /// A node was updated
    NodeUpdated(Uuid),
    /// A connection was created
    ConnectionCreated(Uuid),
    /// A connection was deleted
    ConnectionDeleted(Uuid),
    /// The mindmap was loaded
    MindmapLoaded(Uuid),
    /// The mindmap was saved
    MindmapSaved(Uuid),
    /// Search results were updated
    SearchResultsUpdated(Vec<Uuid>),
    /// View mode changed
    ViewModeChanged(BrainstormingViewMode),
}

/// Callback function type for events
pub type BrainstormingEventCallback = Box<dyn Fn(BrainstormingEvent) -> Result<(), String> + Send + Sync>;

/// Brainstorming tool base functionality
pub struct BrainstormingToolBase {
    /// Current mindmap data
    pub mindmap: Option<BrainstormingData>,
    /// UI state
    pub ui_state: BrainstormingUiState,
    /// Database service
    pub db_service: Arc<herding_cats_rust::database::EnhancedDatabaseService>,
    /// Event callback
    pub event_callback: Option<BrainstormingEventCallback>,
    /// Current project ID
    pub project_id: Option<Uuid>,
}

impl BrainstormingToolBase {
    /// Create a new brainstorming tool base
    pub fn new(db_service: Arc<herding_cats_rust::database::EnhancedDatabaseService>) -> Self {
        Self {
            mindmap: None,
            ui_state: BrainstormingUiState::default(),
            db_service,
            event_callback: None,
            project_id: None,
        }
    }

    /// Set the event callback
    pub fn set_event_callback<F>(&mut self, callback: F)
    where
        F: Fn(BrainstormingEvent) -> Result<(), String> + 'static + Send + Sync,
    {
        self.event_callback = Some(Box::new(callback));
    }

    /// Create a new mindmap
    pub fn create_mindmap(&mut self, title: String, description: String) -> Result<Uuid, String> {
        if let Some(project_id) = self.project_id {
            let mindmap = BrainstormingData::new(project_id, title, description);
            let mindmap_id = mindmap.id;

            self.mindmap = Some(mindmap);

            // Notify listeners
            self.notify_event(BrainstormingEvent::MindmapLoaded(mindmap_id))?;

            Ok(mindmap_id)
        } else {
            Err("No active project".to_string())
        }
    }

    /// Load an existing mindmap
    pub fn load_mindmap(&mut self, mindmap_id: Uuid) -> Result<(), String> {
        // TODO: Implement database loading
        // For now, create a new mindmap
        let project_id = self.project_id.ok_or("No active project")?;
        let mindmap = BrainstormingData::new(project_id, "Loaded Mindmap".to_string(), "Loaded from database".to_string());

        self.mindmap = Some(mindmap);

        // Notify listeners
        self.notify_event(BrainstormingEvent::MindmapLoaded(mindmap_id))?;

        Ok(())
    }

    /// Save the current mindmap
    pub fn save_mindmap(&self) -> Result<(), String> {
        if let Some(mindmap) = &self.mindmap {
            // TODO: Implement database saving
            self.notify_event(BrainstormingEvent::MindmapSaved(mindmap.id))?;
            Ok(())
        } else {
            Err("No active mindmap".to_string())
        }
    }

    /// Add a new node to the mindmap
    pub fn add_node(
        &mut self,
        title: String,
        content: String,
        node_type: NodeType,
        position: (f32, f32),
    ) -> Result<Uuid, String> {
        let mindmap = self.mindmap.as_mut().ok_or("No active mindmap")?;

        let node = BrainstormNode::new(title, content, node_type, position);
        let node_id = mindmap.add_node(node);

        // Notify listeners
        self.notify_event(BrainstormingEvent::NodeCreated(node_id))?;

        Ok(node_id)
    }

    /// Add a central node (if none exists)
    pub fn add_central_node(&mut self, title: String, content: String) -> Result<Uuid, String> {
        let mindmap = self.mindmap.as_mut().ok_or("No active mindmap")?;

        // Check if central node already exists
        if mindmap.get_central_node().is_some() {
            return Err("Central node already exists".to_string());
        }

        let node = BrainstormNode::new_central(title, content);
        let node_id = mindmap.add_node(node);

        // Notify listeners
        self.notify_event(BrainstormingEvent::NodeCreated(node_id))?;

        Ok(node_id)
    }

    /// Update an existing node
    pub fn update_node(
        &mut self,
        node_id: Uuid,
        title: String,
        content: String,
    ) -> Result<(), String> {
        let mindmap = self.mindmap.as_mut().ok_or("No active mindmap")?;
        let node = mindmap.get_node_mut(&node_id).ok_or("Node not found")?;

        node.update_content(title, content);

        // Notify listeners
        self.notify_event(BrainstormingEvent::NodeUpdated(node_id))?;

        Ok(())
    }

    /// Delete a node and its connections
    pub fn delete_node(&mut self, node_id: Uuid) -> Result<BrainstormNode, String> {
        let mindmap = self.mindmap.as_mut().ok_or("No active mindmap")?;

        let node = mindmap.remove_node(&node_id)?;

        // Notify listeners
        self.notify_event(BrainstormingEvent::NodeDeleted(node_id))?;

        Ok(node)
    }

    /// Add a connection between two nodes
    pub fn add_connection(
        &mut self,
        from_node_id: Uuid,
        to_node_id: Uuid,
        connection_type: ConnectionType,
    ) -> Result<Uuid, String> {
        let mindmap = self.mindmap.as_mut().ok_or("No active mindmap")?;

        let connection_id = mindmap.add_connection(from_node_id, to_node_id, connection_type)?;

        // Notify listeners
        self.notify_event(BrainstormingEvent::ConnectionCreated(connection_id))?;

        Ok(connection_id)
    }

    /// Delete a connection
    pub fn delete_connection(&mut self, connection_id: Uuid) -> Result<BrainstormConnection, String> {
        let mindmap = self.mindmap.as_mut().ok_or("No active mindmap")?;

        let connection = mindmap.remove_connection(&connection_id)?;

        // Notify listeners
        self.notify_event(BrainstormingEvent::ConnectionDeleted(connection_id))?;

        Ok(connection)
    }

    /// Select a node
    pub fn select_node(&mut self, node_id: Option<Uuid>) -> Result<(), String> {
        self.ui_state.selected_node = node_id;

        if let Some(id) = node_id {
            // Notify listeners
            self.notify_event(BrainstormingEvent::NodeSelected(id))?;
        }

        Ok(())
    }

    /// Search nodes by content
    pub fn search_nodes(&self, query: &str) -> Vec<&BrainstormNode> {
        let mindmap = self.mindmap.as_ref().unwrap_or(&BrainstormingData::new(
            Uuid::new_v4(),
            "Empty".to_string(),
            "Empty".to_string(),
        ));

        let results = mindmap.search_nodes(query);

        // Notify listeners with search results
        if let Some(callback) = &self.event_callback {
            let node_ids: Vec<Uuid> = results.iter().map(|node| node.id).collect();
            let _ = callback(BrainstormingEvent::SearchResultsUpdated(node_ids));
        }

        results
    }

    /// Get nodes by type (filtered)
    pub fn get_nodes_by_type(&self, node_type: NodeType) -> Vec<&BrainstormNode> {
        let mindmap = self.mindmap.as_ref().unwrap_or(&BrainstormingData::new(
            Uuid::new_v4(),
            "Empty".to_string(),
            "Empty".to_string(),
        ));

        mindmap.get_nodes_by_type(node_type)
    }

    /// Get all nodes (filtered by UI state)
    pub fn get_filtered_nodes(&self) -> Vec<&BrainstormNode> {
        let mindmap = self.mindmap.as_ref().unwrap_or(&BrainstormingData::new(
            Uuid::new_v4(),
            "Empty".to_string(),
            "Empty".to_string(),
        ));

        let mut nodes: Vec<&BrainstormNode> = mindmap.nodes.values().collect();

        // Apply type filtering
        if !self.ui_state.filter_node_types.is_empty() {
            nodes.retain(|node| self.ui_state.filter_node_types.contains(&node.node_type));
        }

        nodes
    }

    /// Get all connections (filtered by UI state)
    pub fn get_filtered_connections(&self) -> Vec<&BrainstormConnection> {
        let mindmap = self.mindmap.as_ref().unwrap_or(&BrainstormingData::new(
            Uuid::new_v4(),
            "Empty".to_string(),
            "Empty".to_string(),
        ));

        let mut connections: Vec<&BrainstormConnection> = mindmap.connections.values().collect();

        // Apply filtering based on connected nodes
        if !self.ui_state.filter_node_types.is_empty() {
            connections.retain(|conn| {
                if let (Some(from_node), Some(to_node)) = (
                    mindmap.get_node(&conn.from_node_id),
                    mindmap.get_node(&conn.to_node_id),
                ) {
                    self.ui_state.filter_node_types.contains(&from_node.node_type) ||
                    self.ui_state.filter_node_types.contains(&to_node.node_type)
                } else {
                    false
                }
            });
        }

        connections
    }

    /// Export mindmap as JSON
    pub fn export_json(&self) -> Result<String, String> {
        let mindmap = self.mindmap.as_ref().ok_or("No active mindmap")?;
        mindmap.export_json()
    }

    /// Import mindmap from JSON
    pub fn import_json(&mut self, data: &str) -> Result<(), String> {
        let imported = BrainstormingData::import_json(data)?;

        // Update project ID to match current project
        if let Some(project_id) = self.project_id {
            // Note: This is a simplified approach. In a real implementation,
            // you might want to create a copy with a new ID and project association.
        }

        self.mindmap = Some(imported);

        Ok(())
    }

    /// Apply layout algorithm
    pub fn apply_layout(&mut self, algorithm: LayoutAlgorithm) {
        if let Some(mindmap) = &mut self.mindmap {
            mindmap.apply_layout(algorithm);
        }
    }

    /// Set current project
    pub fn set_project(&mut self, project_id: Option<Uuid>) {
        self.project_id = project_id;
        if project_id.is_none() {
            self.mindmap = None;
        }
    }

    /// Get the current mindmap ID
    pub fn get_current_mindmap_id(&self) -> Option<Uuid> {
        self.mindmap.as_ref().map(|m| m.id)
    }

    /// Get a reference to the current mindmap
    pub fn get_mindmap(&self) -> Option<&BrainstormingData> {
        self.mindmap.as_ref()
    }

    /// Get a mutable reference to the current mindmap
    pub fn get_mindmap_mut(&mut self) -> Option<&mut BrainstormingData> {
        self.mindmap.as_mut()
    }

    /// Notify event callback
    fn notify_event(&self, event: BrainstormingEvent) -> Result<(), String> {
        if let Some(callback) = &self.event_callback {
            callback(event)
        } else {
            Ok(())
        }
    }
}

impl Default for BrainstormingToolBase {
    fn default() -> Self {
        // This requires a default database service, which we can't provide
        // without knowing the specific implementation. For now, we'll panic
        // if this is called without proper initialization.
        panic!("BrainstormingToolBase::default() called without database service. Use BrainstormingToolBase::new() instead.")
    }
}

/// Mock database service for testing
pub struct MockBrainstormingDatabaseService;

impl MockBrainstormingDatabaseService {
    fn save_brainstorming_data(&self, _data: &BrainstormingData) -> DatabaseResult<()> {
        Ok(())
    }

    fn load_brainstorming_data(&self, _id: Uuid) -> DatabaseResult<Option<BrainstormingData>> {
        Ok(None)
    }

    fn list_brainstorming_data(&self, _project_id: Uuid) -> DatabaseResult<Vec<BrainstormingData>> {
        Ok(Vec::new())
    }
}
