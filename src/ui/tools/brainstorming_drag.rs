//! Brainstorming Drag and Drop Module
//! 
//! Handles drag and drop functionality for brainstorming nodes and connections,
//! including cross-tool compatibility with other writing tools.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::tools::brainstorming_data::{BrainstormNode, BrainstormConnection, NodeType, ConnectionType};

/// Drag data types for brainstorming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrainstormingDragData {
    /// A node being dragged
    Node {
        node_id: Uuid,
        node_type: NodeType,
        title: String,
        content: String,
        position: (f32, f32),
    },
    /// A connection being dragged
    Connection {
        connection_id: Uuid,
        from_node_id: Uuid,
        to_node_id: Uuid,
        connection_type: ConnectionType,
    },
    /// Text content being dragged into the brainstorming tool
    TextContent {
        content: String,
        source_tool: String,
    },
    /// Node from another tool (e.g., hierarchy item, codex entry)
    ExternalNode {
        id: String,
        title: String,
        content: String,
        source_tool: String,
        source_type: String,
    },
}

impl BrainstormingDragData {
    /// Serialize drag data for transfer
    pub fn serialize(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    /// Deserialize drag data from transfer
    pub fn deserialize(data: &str) -> Result<Self, String> {
        serde_json::from_str(data).map_err(|e| e.to_string())
    }

    /// Get the display text for the drag data
    pub fn get_display_text(&self) -> String {
        match self {
            BrainstormingDragData::Node { title, .. } => title.clone(),
            BrainstormingDragData::Connection { .. } => "Connection".to_string(),
            BrainstormingDragData::TextContent { content, .. } => {
                content.chars().take(50).collect::<String>() + 
                if content.len() > 50 { "..." } else { "" }
            },
            BrainstormingDragData::ExternalNode { title, source_tool, .. } => {
                format!("{} (from {})", title, source_tool)
            },
        }
    }

    /// Get the source tool for cross-tool operations
    pub fn get_source_tool(&self) -> Option<&str> {
        match self {
            BrainstormingDragData::Node { .. } => Some("brainstorming"),
            BrainstormingDragData::Connection { .. } => Some("brainstorming"),
            BrainstormingDragData::TextContent { source_tool, .. } => Some(source_tool),
            BrainstormingDragData::ExternalNode { source_tool, .. } => Some(source_tool),
        }
    }

    /// Check if this drag data can be dropped on a node
    pub fn can_drop_on_node(&self, target_node_type: NodeType) -> bool {
        match self {
            BrainstormingDragData::Node { .. } => true, // Nodes can be connected
            BrainstormingDragData::Connection { .. } => false, // Connections are not dropped on nodes
            BrainstormingDragData::TextContent { .. } => true, // Text can be added to nodes
            BrainstormingDragData::ExternalNode { .. } => true, // External nodes can be connected
        }
    }

    /// Check if this drag data can be dropped on the canvas
    pub fn can_drop_on_canvas(&self) -> bool {
        match self {
            BrainstormingDragData::Node { .. } => true, // Nodes can be placed on canvas
            BrainstormingDragData::Connection { .. } => false, // Connections need nodes
            BrainstormingDragData::TextContent { .. } => true, // Text creates new nodes
            BrainstormingDragData::ExternalNode { .. } => true, // External nodes create new nodes
        }
    }
}

/// Drag operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragOperation {
    /// Move a node
    MoveNode,
    /// Create a connection
    CreateConnection,
    /// Create a new node from external data
    CreateNode,
    /// Add content to existing node
    AddContent,
}

/// Drag state for brainstorming
#[derive(Debug, Clone, Default)]
pub struct BrainstormingDragState {
    pub is_dragging: bool,
    pub drag_data: Option<BrainstormingDragData>,
    pub drag_start_position: Option<(f32, f32)>,
    pub drag_operation: Option<DragOperation>,
    pub drag_visual_feedback: Option<DragVisualFeedback>,
    pub drop_target: Option<DropTarget>,
}

/// Visual feedback for drag operations
#[derive(Debug, Clone)]
pub struct DragVisualFeedback {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: String,
    pub opacity: f32,
    pub shape: DragShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragShape {
    Node,
    Connection,
    Text,
    External,
}

/// Drop target types
#[derive(Debug, Clone)]
pub enum DropTarget {
    Node(Uuid),
    Canvas((f32, f32)),
    Connection(Uuid),
    Invalid,
}

/// Drag handler for brainstorming operations
pub struct BrainstormingDragHandler {
    pub drag_state: BrainstormingDragState,
    pub compatibility_rules: Vec<DragCompatibilityRule>,
}

impl BrainstormingDragHandler {
    /// Create a new drag handler
    pub fn new() -> Self {
        Self {
            drag_state: BrainstormingDragState::default(),
            compatibility_rules: Self::default_compatibility_rules(),
        }
    }

    /// Start dragging a node
    pub fn start_node_drag(&mut self, node: &BrainstormNode, mouse_position: (f32, f32)) {
        let drag_data = BrainstormingDragData::Node {
            node_id: node.id,
            node_type: node.node_type,
            title: node.title.clone(),
            content: node.content.clone(),
            position: node.position,
        };

        self.drag_state = BrainstormingDragState {
            is_dragging: true,
            drag_data: Some(drag_data),
            drag_start_position: Some(mouse_position),
            drag_operation: Some(DragOperation::MoveNode),
            drag_visual_feedback: Some(DragVisualFeedback {
                position: mouse_position,
                size: (node.get_size(), node.get_size()),
                color: node.get_color().to_string(),
                opacity: 0.7,
                shape: DragShape::Node,
            }),
            drop_target: None,
        };
    }

    /// Start dragging a connection
    pub fn start_connection_drag(&mut self, connection: &BrainstormConnection, mouse_position: (f32, f32)) {
        let drag_data = BrainstormingDragData::Connection {
            connection_id: connection.id,
            from_node_id: connection.from_node_id,
            to_node_id: connection.to_node_id,
            connection_type: connection.connection_type,
        };

        self.drag_state = BrainstormingDragState {
            is_dragging: true,
            drag_data: Some(drag_data),
            drag_start_position: Some(mouse_position),
            drag_operation: Some(DragOperation::CreateConnection),
            drag_visual_feedback: Some(DragVisualFeedback {
                position: mouse_position,
                size: (10.0, 10.0),
                color: connection.get_color().to_string(),
                opacity: 0.7,
                shape: DragShape::Connection,
            }),
            drop_target: None,
        };
    }

    /// Start dragging external content
    pub fn start_external_drag(
        &mut self,
        content: String,
        source_tool: String,
        source_type: String,
        mouse_position: (f32, f32),
    ) {
        let drag_data = BrainstormingDragData::TextContent {
            content,
            source_tool,
        };

        self.drag_state = BrainstormingDragState {
            is_dragging: true,
            drag_data: Some(drag_data),
            drag_start_position: Some(mouse_position),
            drag_operation: Some(DragOperation::CreateNode),
            drag_visual_feedback: Some(DragVisualFeedback {
                position: mouse_position,
                size: (60.0, 40.0),
                color: "#3498DB".to_string(),
                opacity: 0.7,
                shape: DragShape::Text,
            }),
            drop_target: None,
        };
    }

    /// Update drag position
    pub fn update_drag_position(&mut self, mouse_position: (f32, f32)) {
        if self.drag_state.is_dragging {
            if let Some(ref mut feedback) = self.drag_state.drag_visual_feedback {
                feedback.position = mouse_position;
            }
        }
    }

    /// Determine drop target
    pub fn determine_drop_target(
        &self,
        mouse_position: (f32, f32),
        nodes: &[&BrainstormNode],
        canvas_bounds: (f32, f32, f32, f32),
    ) -> DropTarget {
        // Check if over any node
        for node in nodes {
            let distance = Self::distance_to_point(mouse_position, node.position);
            if distance < node.get_size() / 2.0 {
                return DropTarget::Node(node.id);
            }
        }

        // Check if within canvas bounds
        let (x, y) = mouse_position;
        let (min_x, min_y, max_x, max_y) = canvas_bounds;
        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            return DropTarget::Canvas(mouse_position);
        }

        DropTarget::Invalid
    }

    /// Handle drop operation
    pub fn handle_drop(&mut self, target: DropTarget) -> Result<DropResult, String> {
        if !self.drag_state.is_dragging {
            return Err("No active drag operation".to_string());
        }

        let drag_data = self.drag_state.drag_data.as_ref().ok_or("No drag data")?;
        let operation = self.drag_state.drag_operation.ok_or("No drag operation")?;

        let result = match (operation, target) {
            (DragOperation::MoveNode, DropTarget::Canvas(new_position)) => {
                self.handle_move_node_to_canvas(drag_data, new_position)
            },
            (DragOperation::CreateConnection, DropTarget::Node(target_node_id)) => {
                self.handle_create_connection(drag_data, target_node_id)
            },
            (DragOperation::CreateNode, DropTarget::Canvas(position)) => {
                self.handle_create_node_from_external(drag_data, position)
            },
            (DragOperation::AddContent, DropTarget::Node(target_node_id)) => {
                self.handle_add_content_to_node(drag_data, target_node_id)
            },
            _ => Err("Invalid drop operation".to_string()),
        };

        // Clear drag state
        self.drag_state = BrainstormingDragState::default();

        result
    }

    /// Handle moving a node to canvas position
    fn handle_move_node_to_canvas(
        &self,
        drag_data: &BrainstormingDragData,
        new_position: (f32, f32),
    ) -> Result<DropResult, String> {
        if let BrainstormingDragData::Node { node_id, .. } = drag_data {
            Ok(DropResult::MoveNode {
                node_id: *node_id,
                new_position,
            })
        } else {
            Err("Invalid drag data for node move".to_string())
        }
    }

    /// Handle creating a connection
    fn handle_create_connection(
        &self,
        drag_data: &BrainstormingDragData,
        target_node_id: Uuid,
    ) -> Result<DropResult, String> {
        if let BrainstormingDragData::Connection { 
            from_node_id, connection_type, ..
        } = drag_data {
            Ok(DropResult::CreateConnection {
                from_node_id: *from_node_id,
                to_node_id: target_node_id,
                connection_type: *connection_type,
            })
        } else {
            Err("Invalid drag data for connection creation".to_string())
        }
    }

    /// Handle creating a node from external content
    fn handle_create_node_from_external(
        &self,
        drag_data: &BrainstormingDragData,
        position: (f32, f32),
    ) -> Result<DropResult, String> {
        match drag_data {
            BrainstormingDragData::TextContent { content, source_tool } => {
                Ok(DropResult::CreateNode {
                    title: format!("From {}", source_tool),
                    content: content.clone(),
                    node_type: NodeType::Detail,
                    position,
                })
            },
            BrainstormingDragData::ExternalNode { title, content, source_tool, .. } => {
                Ok(DropResult::CreateNode {
                    title: format!("{} (from {})", title, source_tool),
                    content: content.clone(),
                    node_type: NodeType::Detail,
                    position,
                })
            },
            _ => Err("Invalid drag data for node creation".to_string()),
        }
    }

    /// Handle adding content to existing node
    fn handle_add_content_to_node(
        &self,
        drag_data: &BrainstormingDragData,
        target_node_id: Uuid,
    ) -> Result<DropResult, String> {
        match drag_data {
            BrainstormingDragData::TextContent { content, .. } => {
                Ok(DropResult::AddContent {
                    node_id: target_node_id,
                    additional_content: content.clone(),
                })
            },
            _ => Err("Invalid drag data for content addition".to_string()),
        }
    }

    /// Check if drag operation is compatible with target
    pub fn is_compatible(&self, drag_data: &BrainstormingDragData, target: &DropTarget) -> bool {
        for rule in &self.compatibility_rules {
            if rule.matches(drag_data, target) {
                return rule.allowed;
            }
        }
        false
    }

    /// Get default compatibility rules
    fn default_compatibility_rules() -> Vec<DragCompatibilityRule> {
        vec![
            DragCompatibilityRule::new(
                DragOperation::MoveNode,
                vec!["brainstorming"],
                vec!["canvas"],
                true,
            ),
            DragCompatibilityRule::new(
                DragOperation::CreateConnection,
                vec!["brainstorming"],
                vec!["node"],
                true,
            ),
            DragCompatibilityRule::new(
                DragOperation::CreateNode,
                vec!["text", "external"],
                vec!["canvas"],
                true,
            ),
            DragCompatibilityRule::new(
                DragOperation::AddContent,
                vec!["text"],
                vec!["node"],
                true,
            ),
        ]
    }

    /// Calculate distance between two points
    fn distance_to_point(point1: (f32, f32), point2: (f32, f32)) -> f32 {
        let dx = point1.0 - point2.0;
        let dy = point1.1 - point2.1;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Compatibility rule for drag and drop operations
#[derive(Debug, Clone)]
pub struct DragCompatibilityRule {
    pub operation: DragOperation,
    pub allowed_sources: Vec<String>,
    pub allowed_targets: Vec<String>,
    pub allowed: bool,
}

impl DragCompatibilityRule {
    /// Create a new compatibility rule
    pub fn new(
        operation: DragOperation,
        allowed_sources: Vec<String>,
        allowed_targets: Vec<String>,
        allowed: bool,
    ) -> Self {
        Self {
            operation,
            allowed_sources,
            allowed_targets,
            allowed,
        }
    }

    /// Check if this rule matches the current drag operation
    pub fn matches(&self, drag_data: &BrainstormingDragData, target: &DropTarget) -> bool {
        let source_matches = match drag_data {
            BrainstormingDragData::Node { .. } => self.allowed_sources.contains(&"brainstorming".to_string()),
            BrainstormingDragData::Connection { .. } => self.allowed_sources.contains(&"brainstorming".to_string()),
            BrainstormingDragData::TextContent { source_tool, .. } => self.allowed_sources.contains(source_tool),
            BrainstormingDragData::ExternalNode { source_tool, .. } => self.allowed_sources.contains(source_tool),
        };

        let target_matches = match target {
            DropTarget::Node(_) => self.allowed_targets.contains(&"node".to_string()),
            DropTarget::Canvas(_) => self.allowed_targets.contains(&"canvas".to_string()),
            DropTarget::Connection(_) => self.allowed_targets.contains(&"connection".to_string()),
            DropTarget::Invalid => false,
        };

        source_matches && target_matches
    }
}

/// Result of a drop operation
#[derive(Debug, Clone)]
pub enum DropResult {
    /// Move a node to a new position
    MoveNode {
        node_id: Uuid,
        new_position: (f32, f32),
    },
    /// Create a new connection
    CreateConnection {
        from_node_id: Uuid,
        to_node_id: Uuid,
        connection_type: ConnectionType,
    },
    /// Create a new node
    CreateNode {
        title: String,
        content: String,
        node_type: NodeType,
        position: (f32, f32),
    },
    /// Add content to existing node
    AddContent {
        node_id: Uuid,
        additional_content: String,
    },
}

impl Default for BrainstormingDragHandler {
    fn default() -> Self {
        Self::new()
    }
}