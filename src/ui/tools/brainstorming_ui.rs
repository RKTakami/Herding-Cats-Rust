//! Brainstorming UI Module
//!
//! Main UI implementation for the Brainstorming tool with mindmap visualization,
//! drag and drop support, and cross-tool integration.

use std::sync::{Arc, Mutex};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use uuid::Uuid;

use crate::ui::tools::{
    brainstorming_data::{*, NodeType, ConnectionType, LayoutAlgorithm},
    brainstorming_base::{BrainstormingToolBase, BrainstormingUiState, BrainstormingEvent, BrainstormingViewMode},
    brainstorming_drag::{BrainstormingDragHandler, BrainstormingDragData, DragOperation, DropTarget},
    ToolIntegration,
};
use crate::{database::{DatabaseService, DatabaseResult}};
use crate::ui_state::AppState;

/// Brainstorming tool UI component
pub struct BrainstormingTool {
    /// Base functionality
    pub base: BrainstormingToolBase,
    /// Drag and drop handler
    pub drag_handler: BrainstormingDragHandler,
    /// UI state for Slint integration
    pub slint_ui_state: Arc<Mutex<BrainstormingSlintState>>,
    /// Current app state
    pub app_state: Arc<Mutex<AppState>>,
    /// Tool window state
    pub window_open: bool,
    /// Node editing state
    pub editing_node: Option<Uuid>,
    /// New node creation state
    pub creating_node: Option<NewNodeState>,
}

/// Slint-specific UI state
#[derive(Debug, Clone)]
pub struct BrainstormingSlintState {
    pub nodes: Vec<SlintBrainstormNode>,
    pub connections: Vec<SlintBrainstormConnection>,
    pub selected_node_id: Option<String>,
    pub hovered_node_id: Option<String>,
    pub zoom_level: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub show_grid: bool,
    pub view_mode: String,
}

/// Node representation for Slint
#[derive(Debug, Clone)]
pub struct SlintBrainstormNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub node_type: String,
    pub color: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub is_collapsed: bool,
}

/// Connection representation for Slint
#[derive(Debug, Clone)]
pub struct SlintBrainstormConnection {
    pub id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub connection_type: String,
    pub color: String,
    pub label: Option<String>,
}

/// State for creating a new node
#[derive(Debug, Clone)]
pub struct NewNodeState {
    pub position: (f32, f32),
    pub node_type: NodeType,
}

impl BrainstormingTool {
    /// Create a new brainstorming tool
    pub fn new(db_service: Arc<crate::database::EnhancedDatabaseService>) -> Self {
        Self {
            base: BrainstormingToolBase::new(db_service.clone()),
            drag_handler: BrainstormingDragHandler::new(),
            slint_ui_state: Arc::new(Mutex::new(BrainstormingSlintState::default())),
            app_state: Arc::new(Mutex::new(AppState::default())),
            window_open: false,
            editing_node: None,
            creating_node: None,
        }
    }

    /// Set app state reference
    pub fn set_app_state(&mut self, app_state: Arc<Mutex<AppState>>) {
        self.app_state = app_state;
    }

    /// Open the brainstorming tool window
    pub fn open_window(&mut self) {
        self.window_open = true;
    }

    /// Close the brainstorming tool window
    pub fn close_window(&mut self) {
        self.window_open = false;
    }

    /// Toggle window visibility
    pub fn toggle_window(&mut self) {
        self.window_open = !self.window_open;
    }

    /// Create a new mindmap
    pub fn create_mindmap(&mut self, title: String, description: String) -> Result<(), String> {
        self.base.create_mindmap(title, description)?;
        self.update_slint_state();
        Ok(())
    }

    /// Load an existing mindmap
    pub fn load_mindmap(&mut self, mindmap_id: Uuid) -> Result<(), String> {
        self.base.load_mindmap(mindmap_id)?;
        self.update_slint_state();
        Ok(())
    }

    /// Add a new node
    pub fn add_node(
        &mut self,
        title: String,
        content: String,
        node_type: NodeType,
        position: (f32, f32),
    ) -> Result<Uuid, String> {
        let node_id = self.base.add_node(title, content, node_type, position)?;
        self.update_slint_state();
        Ok(node_id)
    }

    /// Add a central node
    pub fn add_central_node(&mut self, title: String, content: String) -> Result<Uuid, String> {
        let node_id = self.base.add_central_node(title, content)?;
        self.update_slint_state();
        Ok(node_id)
    }

    /// Update a node
    pub fn update_node(&mut self, node_id: Uuid, title: String, content: String) -> Result<(), String> {
        self.base.update_node(node_id, title, content)?;
        self.update_slint_state();
        Ok(())
    }

    /// Delete a node
    pub fn delete_node(&mut self, node_id: Uuid) -> Result<(), String> {
        self.base.delete_node(node_id)?;
        self.update_slint_state();
        Ok(())
    }

    /// Add a connection
    pub fn add_connection(
        &mut self,
        from_node_id: Uuid,
        to_node_id: Uuid,
        connection_type: ConnectionType,
    ) -> Result<Uuid, String> {
        let connection_id = self.base.add_connection(from_node_id, to_node_id, connection_type)?;
        self.update_slint_state();
        Ok(connection_id)
    }

    /// Delete a connection
    pub fn delete_connection(&mut self, connection_id: Uuid) -> Result<(), String> {
        self.base.delete_connection(connection_id)?;
        self.update_slint_state();
        Ok(())
    }

    /// Select a node
    pub fn select_node(&mut self, node_id: Option<Uuid>) -> Result<(), String> {
        self.base.select_node(node_id)?;

        let mut slint_state = self.slint_ui_state.lock().unwrap();
        slint_state.selected_node_id = node_id.map(|id| id.to_string());

        Ok(())
    }

    /// Start editing a node
    pub fn start_editing_node(&mut self, node_id: Uuid) {
        self.editing_node = Some(node_id);
    }

    /// Stop editing a node
    pub fn stop_editing_node(&mut self) {
        self.editing_node = None;
    }

    /// Start creating a new node
    pub fn start_creating_node(&mut self, position: (f32, f32), node_type: NodeType) {
        self.creating_node = Some(NewNodeState {
            position,
            node_type,
        });
    }

    /// Cancel creating a new node
    pub fn cancel_creating_node(&mut self) {
        self.creating_node = None;
    }

    /// Search nodes
    pub fn search_nodes(&self, query: &str) -> Vec<&BrainstormNode> {
        self.base.search_nodes(query)
    }

    /// Apply layout algorithm
    pub fn apply_layout(&mut self, algorithm: LayoutAlgorithm) {
        self.base.apply_layout(algorithm);
        self.update_slint_state();
    }

    /// Export mindmap as JSON
    pub fn export_json(&self) -> Result<String, String> {
        self.base.export_json()
    }

    /// Import mindmap from JSON
    pub fn import_json(&mut self, data: &str) -> Result<(), String> {
        self.base.import_json(data)?;
        self.update_slint_state();
        Ok(())
    }

    /// Update Slint state from internal state
    pub fn update_slint_state(&mut self) {
        if let Some(mindmap) = &self.base.mindmap {
            let mut slint_state = self.slint_ui_state.lock().unwrap();

            // Update nodes
            slint_state.nodes.clear();
            for node in mindmap.nodes.values() {
                slint_state.nodes.push(SlintBrainstormNode {
                    id: node.id.to_string(),
                    title: node.title.clone(),
                    content: node.content.clone(),
                    node_type: format!("{:?}", node.node_type),
                    color: node.get_color().to_string(),
                    x: node.position.0,
                    y: node.position.1,
                    size: node.get_size(),
                    is_collapsed: node.is_collapsed,
                });
            }

            // Update connections
            slint_state.connections.clear();
            for connection in mindmap.connections.values() {
                slint_state.connections.push(SlintBrainstormConnection {
                    id: connection.id.to_string(),
                    from_node_id: connection.from_node_id.to_string(),
                    to_node_id: connection.to_node_id.to_string(),
                    connection_type: format!("{:?}", connection.connection_type),
                    color: connection.get_color().to_string(),
                    label: connection.label.clone(),
                });
            }

            // Update UI state
            slint_state.zoom_level = self.base.ui_state.zoom_level;
            slint_state.pan_x = self.base.ui_state.pan_offset.0;
            slint_state.pan_y = self.base.ui_state.pan_offset.1;
            slint_state.show_grid = self.base.ui_state.show_grid;
            slint_state.view_mode = format!("{:?}", self.base.ui_state.view_mode);
        }
    }

    /// Handle drag start
    pub fn handle_drag_start(&mut self, node_id: Uuid, mouse_position: (f32, f32)) {
        if let Some(mindmap) = &self.base.mindmap {
            if let Some(node) = mindmap.get_node(&node_id) {
                self.drag_handler.start_node_drag(node, mouse_position);

                // Update app state
                let mut app_state = self.app_state.lock().unwrap();
                app_state.is_dragging = true;
                app_state.drag_source_window = Some("brainstorming".to_string());
                app_state.drag_type = Some("brainstorming_node".to_string());
            }
        }
    }

    /// Handle drag update
    pub fn handle_drag_update(&mut self, mouse_position: (f32, f32)) {
        self.drag_handler.update_drag_position(mouse_position);
    }

    /// Handle drag end
    pub fn handle_drag_end(&mut self, mouse_position: (f32, f32), canvas_bounds: (f32, f32, f32, f32)) {
        let nodes = if let Some(mindmap) = &self.base.mindmap {
            mindmap.nodes.values().collect()
        } else {
            Vec::new()
        };

        let drop_target = self.drag_handler.determine_drop_target(mouse_position, &nodes, canvas_bounds);

        if let Ok(result) = self.drag_handler.handle_drop(drop_target) {
            self.handle_drop_result(result);
        }

        // Clear app state
        let mut app_state = self.app_state.lock().unwrap();
        app_state.is_dragging = false;
        app_state.drag_source_window = None;
        app_state.drag_data = None;
        app_state.drag_type = None;
    }

    /// Handle drop result
    fn handle_drop_result(&mut self, result: crate::ui::tools::brainstorming_drag::DropResult) {
        match result {
            crate::ui::tools::brainstorming_drag::DropResult::MoveNode { node_id, new_position } => {
                if let Some(mindmap) = &mut self.base.mindmap {
                    if let Some(node) = mindmap.get_node_mut(&node_id) {
                        node.position = new_position;
                    }
                }
            },
            crate::ui::tools::brainstorming_drag::DropResult::CreateConnection { from_node_id, to_node_id, connection_type } => {
                let _ = self.add_connection(from_node_id, to_node_id, connection_type);
            },
            crate::ui::tools::brainstorming_drag::DropResult::CreateNode { title, content, node_type, position } => {
                let _ = self.add_node(title, content, node_type, position);
            },
            crate::ui::tools::brainstorming_drag::DropResult::AddContent { node_id, additional_content } => {
                if let Some(mindmap) = &mut self.base.mindmap {
                    if let Some(node) = mindmap.get_node_mut(&node_id) {
                        node.content.push_str("\n\n");
                        node.content.push_str(&additional_content);
                    }
                }
            },
        }

        self.update_slint_state();
    }

    /// Handle external drag data
    pub fn handle_external_drag(&mut self, drag_data: &str, mouse_position: (f32, f32)) {
        if let Ok(data) = BrainstormingDragData::deserialize(drag_data) {
            self.drag_handler.start_external_drag(
                match data {
                    BrainstormingDragData::TextContent { content, .. } => content,
                    BrainstormingDragData::ExternalNode { title, content, .. } => {
                        format!("{}\n\n{}", title, content)
                    },
                    _ => return,
                },
                match data {
                    BrainstormingDragData::TextContent { source_tool, .. } => source_tool,
                    BrainstormingDragData::ExternalNode { source_tool, .. } => source_tool,
                    _ => return,
                },
                "text".to_string(),
                mouse_position,
            );

            // Update app state
            let mut app_state = self.app_state.lock().unwrap();
            app_state.is_dragging = true;
            app_state.drag_source_window = Some("external".to_string());
            app_state.drag_type = Some("external_content".to_string());
        }
    }

    /// Get current mindmap ID
    pub fn get_current_mindmap_id(&self) -> Option<Uuid> {
        self.base.get_current_mindmap_id()
    }

    /// Check if tool is open
    pub fn is_open(&self) -> bool {
        self.window_open
    }

    /// Get mindmap data for export
    pub fn get_mindmap_data(&self) -> Option<&BrainstormingData> {
        self.base.get_mindmap()
    }
}

impl Default for SlintBrainstormNode {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            content: String::new(),
            node_type: "Detail".to_string(),
            color: "#3498DB".to_string(),
            x: 0.0,
            y: 0.0,
            size: 50.0,
            is_collapsed: false,
        }
    }
}

impl Default for SlintBrainstormConnection {
    fn default() -> Self {
        Self {
            id: String::new(),
            from_node_id: String::new(),
            to_node_id: String::new(),
            connection_type: "Hierarchical".to_string(),
            color: "#2C3E50".to_string(),
            label: None,
        }
    }
}

impl Default for BrainstormingSlintState {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            selected_node_id: None,
            hovered_node_id: None,
            zoom_level: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            show_grid: false,
            view_mode: "Edit".to_string(),
        }
    }
}

impl ToolIntegration for BrainstormingTool {
    fn initialize(&mut self) -> Result<(), String> {
        // Set up event callback
        self.base.set_event_callback(|event| {
            match event {
                BrainstormingEvent::NodeSelected(_) => {
                    // Handle node selection
                },
                BrainstormingEvent::NodeCreated(_) => {
                    // Handle node creation
                },
                BrainstormingEvent::NodeUpdated(_) => {
                    // Handle node update
                },
                BrainstormingEvent::NodeDeleted(_) => {
                    // Handle node deletion
                },
                BrainstormingEvent::ConnectionCreated(_) => {
                    // Handle connection creation
                },
                BrainstormingEvent::ConnectionDeleted(_) => {
                    // Handle connection deletion
                },
                BrainstormingEvent::MindmapLoaded(_) => {
                    // Handle mindmap loading
                },
                BrainstormingEvent::MindmapSaved(_) => {
                    // Handle mindmap saving
                },
                BrainstormingEvent::SearchResultsUpdated(_) => {
                    // Handle search results
                },
                BrainstormingEvent::ViewModeChanged(_) => {
                    // Handle view mode change
                },
            }
            Ok(())
        });

        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        // Update UI state from internal state
        self.update_slint_state();
        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
        // This would be called by the main UI system
        // For Slint integration, the actual rendering is handled by Slint
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), String> {
        // Clean up resources
        self.window_open = false;
        self.editing_node = None;
        self.creating_node = None;
        Ok(())
    }
}
