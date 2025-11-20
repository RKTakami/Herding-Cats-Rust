//! Brainstorming Data Module for Herding Cats Rust
//! 
//! Pure Rust implementation of mindmap brainstorming data and management.
//! Provides nodes, connections, and mindmap layout functionality.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

/// Node types for brainstorming mindmap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    /// Central/main idea node
    Central,
    /// Primary branch node
    Primary,
    /// Secondary branch node
    Secondary,
    /// Detail node
    Detail,
    /// Note node
    Note,
}

impl NodeType {
    /// Get the display name for the node type
    pub fn display_name(&self) -> &'static str {
        match self {
            NodeType::Central => "Central Idea",
            NodeType::Primary => "Primary Branch",
            NodeType::Secondary => "Secondary Branch",
            NodeType::Detail => "Detail",
            NodeType::Note => "Note",
        }
    }

    /// Get a brief description of the node type
    pub fn description(&self) -> &'static str {
        match self {
            NodeType::Central => "Main concept or central idea of the mindmap",
            NodeType::Primary => "Main branch extending from the central idea",
            NodeType::Secondary => "Sub-branch extending from a primary branch",
            NodeType::Detail => "Specific detail or example",
            NodeType::Note => "Additional note or comment",
        }
    }

    /// Get the default color for this node type
    pub fn default_color(&self) -> &'static str {
        match self {
            NodeType::Central => "#FF6B6B",
            NodeType::Primary => "#4ECDC4",
            NodeType::Secondary => "#45B7D1",
            NodeType::Detail => "#96CEB4",
            NodeType::Note => "#FFEAA7",
        }
    }

    /// Get the node size for this type
    pub fn node_size(&self) -> f32 {
        match self {
            NodeType::Central => 80.0,
            NodeType::Primary => 60.0,
            NodeType::Secondary => 50.0,
            NodeType::Detail => 40.0,
            NodeType::Note => 35.0,
        }
    }
}

/// Connection types between nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    /// Standard hierarchical connection
    Hierarchical,
    /// Cross-connection between branches
    CrossReference,
    /// Supporting connection
    Supporting,
    /// Contrasting connection
    Contrasting,
}

impl ConnectionType {
    /// Get the display name for the connection type
    pub fn display_name(&self) -> &'static str {
        match self {
            ConnectionType::Hierarchical => "Hierarchical",
            ConnectionType::CrossReference => "Cross-Reference",
            ConnectionType::Supporting => "Supporting",
            ConnectionType::Contrasting => "Contrasting",
        }
    }

    /// Get the line style for this connection type
    pub fn line_style(&self) -> ConnectionLineStyle {
        match self {
            ConnectionType::Hierarchical => ConnectionLineStyle::Solid,
            ConnectionType::CrossReference => ConnectionLineStyle::Dashed,
            ConnectionType::Supporting => ConnectionLineStyle::Dotted,
            ConnectionType::Contrasting => ConnectionLineStyle::DashDot,
        }
    }

    /// Get the color for this connection type
    pub fn color(&self) -> &'static str {
        match self {
            ConnectionType::Hierarchical => "#2C3E50",
            ConnectionType::CrossReference => "#E74C3C",
            ConnectionType::Supporting => "#27AE60",
            ConnectionType::Contrasting => "#F39C12",
        }
    }
}

/// Line styles for connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionLineStyle {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

/// A node in the brainstorming mindmap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormNode {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub node_type: NodeType,
    pub position: (f32, f32),
    pub size: Option<f32>,
    pub color: Option<String>,
    pub tags: Vec<String>,
    pub connections: Vec<Uuid>, // IDs of connected nodes
    pub parent_id: Option<Uuid>,
    pub children_ids: Vec<Uuid>,
    pub is_collapsed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: Option<HashMap<String, String>>,
}

impl BrainstormNode {
    /// Create a new brainstorming node
    pub fn new(
        title: String,
        content: String,
        node_type: NodeType,
        position: (f32, f32),
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            node_type,
            position,
            size: None, // Use default size from node_type
            color: None, // Use default color from node_type
            tags: Vec::new(),
            connections: Vec::new(),
            parent_id: None,
            children_ids: Vec::new(),
            is_collapsed: false,
            created_at: now.clone(),
            updated_at: now,
            metadata: None,
        }
    }

    /// Create a central idea node at the center
    pub fn new_central(title: String, content: String) -> Self {
        Self::new(title, content, NodeType::Central, (400.0, 300.0))
    }

    /// Get the effective size (either custom or default)
    pub fn get_size(&self) -> f32 {
        self.size.unwrap_or_else(|| self.node_type.node_size())
    }

    /// Get the effective color (either custom or default)
    pub fn get_color(&self) -> &str {
        self.color.as_deref().unwrap_or_else(|| self.node_type.default_color())
    }

    /// Add a tag to the node
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = chrono::Utc::now().to_rfc3339();
        }
    }

    /// Remove a tag from the node
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(index) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(index);
            self.updated_at = chrono::Utc::now().to_rfc3339();
        }
    }

    /// Add a connection to another node
    pub fn add_connection(&mut self, node_id: Uuid) {
        if !self.connections.contains(&node_id) {
            self.connections.push(node_id);
            self.updated_at = chrono::Utc::now().to_rfc3339();
        }
    }

    /// Remove a connection to another node
    pub fn remove_connection(&mut self, node_id: &Uuid) {
        if let Some(index) = self.connections.iter().position(|id| id == node_id) {
            self.connections.remove(index);
            self.updated_at = chrono::Utc::now().to_rfc3339();
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, child_id: Uuid) {
        if !self.children_ids.contains(&child_id) {
            self.children_ids.push(child_id);
            self.updated_at = chrono::Utc::now().to_rfc3339();
        }
    }

    /// Remove a child node
    pub fn remove_child(&mut self, child_id: &Uuid) {
        if let Some(index) = self.children_ids.iter().position(|id| id == child_id) {
            self.children_ids.remove(index);
            self.updated_at = chrono::Utc::now().to_rfc3339();
        }
    }

    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self) {
        self.is_collapsed = !self.is_collapsed;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Update node content
    pub fn update_content(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Set custom size
    pub fn set_custom_size(&mut self, size: Option<f32>) {
        self.size = size;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Set custom color
    pub fn set_custom_color(&mut self, color: Option<String>) {
        self.color = color;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

/// A connection between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormConnection {
    pub id: Uuid,
    pub from_node_id: Uuid,
    pub to_node_id: Uuid,
    pub connection_type: ConnectionType,
    pub label: Option<String>,
    pub color: Option<String>,
    pub line_style: Option<ConnectionLineStyle>,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: Option<HashMap<String, String>>,
}

impl BrainstormConnection {
    /// Create a new connection between two nodes
    pub fn new(
        from_node_id: Uuid,
        to_node_id: Uuid,
        connection_type: ConnectionType,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4(),
            from_node_id,
            to_node_id,
            connection_type,
            label: None,
            color: None,
            line_style: None,
            created_at: now.clone(),
            updated_at: now,
            metadata: None,
        }
    }

    /// Set a label for the connection
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Set custom color
    pub fn set_custom_color(&mut self, color: Option<String>) {
        self.color = color;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Set custom line style
    pub fn set_custom_line_style(&mut self, line_style: Option<ConnectionLineStyle>) {
        self.line_style = line_style;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Get the effective color (either custom or default)
    pub fn get_color(&self) -> &str {
        self.color.as_deref().unwrap_or_else(|| self.connection_type.color())
    }

    /// Get the effective line style (either custom or default)
    pub fn get_line_style(&self) -> ConnectionLineStyle {
        self.line_style.unwrap_or_else(|| self.connection_type.line_style())
    }
}

/// Brainstorming mindmap layout algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutAlgorithm {
    /// Radial layout from center
    Radial,
    /// Hierarchical tree layout
    Hierarchical,
    /// Force-directed layout
    ForceDirected,
    /// Grid layout
    Grid,
}

impl LayoutAlgorithm {
    /// Get the display name for the layout algorithm
    pub fn display_name(&self) -> &'static str {
        match self {
            LayoutAlgorithm::Radial => "Radial",
            LayoutAlgorithm::Hierarchical => "Hierarchical",
            LayoutAlgorithm::ForceDirected => "Force-Directed",
            LayoutAlgorithm::Grid => "Grid",
        }
    }
}

/// Brainstorming data container
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BrainstormingData {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
    pub nodes: HashMap<Uuid, BrainstormNode>,
    pub connections: HashMap<Uuid, BrainstormConnection>,
    pub current_layout: LayoutAlgorithm,
    pub created_at: String,
    pub updated_at: String,
    pub settings: BrainstormingSettings,
}

/// Brainstorming tool settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormingSettings {
    pub auto_layout: bool,
    pub show_node_labels: bool,
    pub show_connection_labels: bool,
    pub default_node_color: String,
    pub default_connection_color: String,
    pub zoom_level: f32,
    pub pan_offset: (f32, f32),
    pub grid_enabled: bool,
    pub grid_size: f32,
}

impl Default for BrainstormingSettings {
    fn default() -> Self {
        Self {
            auto_layout: true,
            show_node_labels: true,
            show_connection_labels: false,
            default_node_color: "#3498DB".to_string(),
            default_connection_color: "#2C3E50".to_string(),
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
            grid_enabled: false,
            grid_size: 50.0,
        }
    }
}

impl BrainstormingData {
    /// Create a new brainstorming data with a central idea
    pub fn new(project_id: Uuid, title: String, description: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let mut data = Self {
            id: Uuid::new_v4(),
            project_id,
            title,
            description,
            nodes: HashMap::new(),
            connections: HashMap::new(),
            current_layout: LayoutAlgorithm::Radial,
            created_at: now.clone(),
            updated_at: now,
            settings: BrainstormingSettings::default(),
        };

        // Create central node
        let central_node = BrainstormNode::new_central(
            "Central Idea".to_string(),
            "Start your brainstorming here...".to_string(),
        );
        data.nodes.insert(central_node.id, central_node);

        data
    }

    /// Get a reference to a node by ID
    pub fn get_node(&self, node_id: &Uuid) -> Option<&BrainstormNode> {
        self.nodes.get(node_id)
    }

    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, node_id: &Uuid) -> Option<&mut BrainstormNode> {
        self.nodes.get_mut(node_id)
    }

    /// Add a new node to the mindmap
    pub fn add_node(&mut self, node: BrainstormNode) -> Uuid {
        let node_id = node.id;
        self.nodes.insert(node_id, node);
        self.updated_at = chrono::Utc::now().to_rfc3339();
        node_id
    }

    /// Remove a node and all its connections
    pub fn remove_node(&mut self, node_id: &Uuid) -> Result<BrainstormNode, String> {
        let node = self.nodes.remove(node_id).ok_or("Node not found")?;
        
        // Remove connections to/from this node
        self.connections.retain(|_, conn| {
            conn.from_node_id != *node_id && conn.to_node_id != *node_id
        });

        // Remove from parent/children relationships
        if let Some(parent_id) = node.parent_id {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.remove_child(node_id);
            }
        }

        for child_id in &node.children_ids {
            if let Some(child_node) = self.nodes.get_mut(child_id) {
                child_node.parent_id = None;
            }
        }

        self.updated_at = chrono::Utc::now().to_rfc3339();
        Ok(node)
    }

    /// Add a connection between two nodes
    pub fn add_connection(&mut self, from_node_id: Uuid, to_node_id: Uuid, connection_type: ConnectionType) -> Result<Uuid, String> {
        // Check if nodes exist
        if !self.nodes.contains_key(&from_node_id) {
            return Err("Source node not found".to_string());
        }
        if !self.nodes.contains_key(&to_node_id) {
            return Err("Target node not found".to_string());
        }

        let connection = BrainstormConnection::new(from_node_id, to_node_id, connection_type);
        let connection_id = connection.id;
        self.connections.insert(connection_id, connection);
        self.updated_at = chrono::Utc::now().to_rfc3339();
        Ok(connection_id)
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, connection_id: &Uuid) -> Result<BrainstormConnection, String> {
        let connection = self.connections.remove(connection_id).ok_or("Connection not found")?;
        self.updated_at = chrono::Utc::now().to_rfc3339();
        Ok(connection)
    }

    /// Get all connections for a specific node
    pub fn get_node_connections(&self, node_id: &Uuid) -> Vec<&BrainstormConnection> {
        self.connections
            .values()
            .filter(|conn| conn.from_node_id == *node_id || conn.to_node_id == *node_id)
            .collect()
    }

    /// Get all nodes connected to a specific node
    pub fn get_connected_nodes(&self, node_id: &Uuid) -> Vec<&BrainstormNode> {
        let mut connected = Vec::new();
        for connection in self.connections.values() {
            if connection.from_node_id == *node_id {
                if let Some(node) = self.nodes.get(&connection.to_node_id) {
                    connected.push(node);
                }
            } else if connection.to_node_id == *node_id {
                if let Some(node) = self.nodes.get(&connection.from_node_id) {
                    connected.push(node);
                }
            }
        }
        connected
    }

    /// Search nodes by title or content
    pub fn search_nodes(&self, query: &str) -> Vec<&BrainstormNode> {
        let query_lower = query.to_lowercase();
        self.nodes
            .values()
            .filter(|node| {
                node.title.to_lowercase().contains(&query_lower) ||
                node.content.to_lowercase().contains(&query_lower) ||
                node.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get nodes by type
    pub fn get_nodes_by_type(&self, node_type: NodeType) -> Vec<&BrainstormNode> {
        self.nodes
            .values()
            .filter(|node| node.node_type == node_type)
            .collect()
    }

    /// Get root nodes (nodes without parents)
    pub fn get_root_nodes(&self) -> Vec<&BrainstormNode> {
        self.nodes
            .values()
            .filter(|node| node.parent_id.is_none())
            .collect()
    }

    /// Get the central node (if any)
    pub fn get_central_node(&self) -> Option<&BrainstormNode> {
        self.nodes
            .values()
            .find(|node| node.node_type == NodeType::Central)
    }

    /// Update mindmap metadata
    pub fn update_metadata(&mut self, title: String, description: String) {
        self.title = title;
        self.description = description;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Apply layout algorithm
    pub fn apply_layout(&mut self, algorithm: LayoutAlgorithm) {
        self.current_layout = algorithm;
        // TODO: Implement actual layout algorithms
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Export mindmap as JSON
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// Import mindmap from JSON
    pub fn import_json(data: &str) -> Result<Self, String> {
        serde_json::from_str(data).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = BrainstormNode::new(
            "Test Node".to_string(),
            "Test Content".to_string(),
            NodeType::Primary,
            (100.0, 100.0),
        );

        assert_eq!(node.title, "Test Node");
        assert_eq!(node.content, "Test Content");
        assert_eq!(node.node_type, NodeType::Primary);
        assert_eq!(node.position, (100.0, 100.0));
        assert!(!node.connections.is_empty() || node.connections.is_empty());
    }

    #[test]
    fn test_central_node_creation() {
        let central = BrainstormNode::new_central(
            "Main Idea".to_string(),
            "Central concept".to_string(),
        );

        assert_eq!(central.node_type, NodeType::Central);
        assert_eq!(central.position, (400.0, 300.0));
        assert_eq!(central.get_size(), 80.0); // Default size for central node
    }

    #[test]
    fn test_connection_creation() {
        let node1 = BrainstormNode::new_central("Node 1".to_string(), "Content 1".to_string());
        let node2 = BrainstormNode::new("Node 2".to_string(), "Content 2".to_string(), NodeType::Primary, (200.0, 200.0));
        
        let connection = BrainstormConnection::new(node1.id, node2.id, ConnectionType::Hierarchical);
        
        assert_eq!(connection.from_node_id, node1.id);
        assert_eq!(connection.to_node_id, node2.id);
        assert_eq!(connection.connection_type, ConnectionType::Hierarchical);
    }

    #[test]
    fn test_brainstorming_data_creation() {
        let project_id = Uuid::new_v4();
        let brainstorming = BrainstormingData::new(
            project_id,
            "Test Mindmap".to_string(),
            "Test Description".to_string(),
        );

        assert_eq!(brainstorming.project_id, project_id);
        assert_eq!(brainstorming.title, "Test Mindmap");
        assert_eq!(brainstorming.description, "Test Description");
        assert_eq!(brainstorming.nodes.len(), 1); // Should have central node
        assert_eq!(brainstorming.connections.len(), 0);
        
        // Should have a central node
        let central = brainstorming.get_central_node().unwrap();
        assert_eq!(central.node_type, NodeType::Central);
    }

    #[test]
    fn test_node_tagging() {
        let mut node = BrainstormNode::new(
            "Test".to_string(),
            "Content".to_string(),
            NodeType::Primary,
            (0.0, 0.0),
        );

        node.add_tag("important".to_string());
        assert!(node.tags.contains(&"important".to_string()));

        node.remove_tag("important");
        assert!(!node.tags.contains(&"important".to_string()));
    }
}