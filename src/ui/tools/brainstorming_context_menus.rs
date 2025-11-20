//! Brainstorming Context Menus Module
//! 
//! Context menu implementations for brainstorming nodes, connections, and canvas operations.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::tools::brainstorming_data::{NodeType, ConnectionType, BrainstormNode, BrainstormConnection};

/// Context menu types for brainstorming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrainstormingMenuType {
    /// Context menu for a node
    Node,
    /// Context menu for a connection
    Connection,
    /// Context menu for the canvas
    Canvas,
    /// Context menu for the toolbar
    Toolbar,
}

/// Menu item action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuItemAction {
    /// Create a new node
    CreateNode {
        node_type: NodeType,
        position: Option<(f32, f32)>,
    },
    /// Edit an existing node
    EditNode {
        node_id: Uuid,
    },
    /// Delete a node
    DeleteNode {
        node_id: Uuid,
    },
    /// Change node type
    ChangeNodeType {
        node_id: Uuid,
        new_type: NodeType,
    },
    /// Add tag to node
    AddTag {
        node_id: Uuid,
        tag: String,
    },
    /// Remove tag from node
    RemoveTag {
        node_id: Uuid,
        tag: String,
    },
    /// Create connection
    CreateConnection {
        from_node_id: Uuid,
        to_node_id: Option<Uuid>,
        connection_type: ConnectionType,
    },
    /// Edit connection
    EditConnection {
        connection_id: Uuid,
    },
    /// Delete connection
    DeleteConnection {
        connection_id: Uuid,
    },
    /// Change connection type
    ChangeConnectionType {
        connection_id: Uuid,
        new_type: ConnectionType,
    },
    /// Apply layout
    ApplyLayout {
        algorithm: crate::ui::tools::brainstorming_data::LayoutAlgorithm,
    },
    /// Import mindmap
    ImportMindmap,
    /// Export mindmap
    ExportMindmap {
        format: ExportFormat,
    },
    /// Search nodes
    SearchNodes {
        query: String,
    },
    /// Toggle grid
    ToggleGrid,
    /// Toggle labels
    ToggleLabels,
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Reset view
    ResetView,
    /// Copy node content
    CopyNodeContent {
        node_id: Uuid,
    },
    /// Copy connection info
    CopyConnectionInfo {
        connection_id: Uuid,
    },
    /// Duplicate node
    DuplicateNode {
        node_id: Uuid,
        new_position: Option<(f32, f32)>,
    },
    /// Collapse/expand node
    ToggleCollapse {
        node_id: Uuid,
    },
    /// Set node color
    SetNodeColor {
        node_id: Uuid,
        color: String,
    },
    /// Set connection color
    SetConnectionColor {
        connection_id: Uuid,
        color: String,
    },
    /// Add note
    AddNote {
        parent_node_id: Option<Uuid>,
        position: (f32, f32),
    },
    /// Clear search
    ClearSearch,
}

/// Export format types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Markdown format
    Markdown,
    /// Image format (PNG/SVG)
    Image,
}

/// Menu item for context menus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub action: MenuItemAction,
    pub enabled: bool,
    pub shortcut: Option<String>,
    pub separator_after: bool,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(
        id: String,
        label: String,
        action: MenuItemAction,
        enabled: bool,
    ) -> Self {
        Self {
            id,
            label,
            icon: None,
            action,
            enabled,
            shortcut: None,
            separator_after: false,
        }
    }

    /// Set icon for the menu item
    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set shortcut for the menu item
    pub fn with_shortcut(mut self, shortcut: String) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    /// Add separator after this item
    pub fn with_separator(mut self) -> Self {
        self.separator_after = true;
        self
    }
}

/// Context menu definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMenu {
    pub menu_type: BrainstormingMenuType,
    pub items: Vec<MenuItem>,
    pub position: (f32, f32),
    pub target_id: Option<String>,
}

impl ContextMenu {
    /// Create a context menu for a node
    pub fn for_node(
        node: &BrainstormNode,
        position: (f32, f32),
        has_selection: bool,
    ) -> Self {
        let mut items = vec![
            MenuItem::new(
                "edit_node".to_string(),
                "Edit Node".to_string(),
                MenuItemAction::EditNode {
                    node_id: node.id,
                },
                true,
            ).with_icon("📝".to_string()),
            
            MenuItem::new(
                "duplicate_node".to_string(),
                "Duplicate Node".to_string(),
                MenuItemAction::DuplicateNode {
                    node_id: node.id,
                    new_position: None,
                },
                true,
            ).with_icon("📋".to_string()),
            
            MenuItem::new(
                "delete_node".to_string(),
                "Delete Node".to_string(),
                MenuItemAction::DeleteNode {
                    node_id: node.id,
                },
                true,
            ).with_icon("🗑️".to_string()).with_separator(),
        ];

        // Node type submenu
        for node_type in [NodeType::Primary, NodeType::Secondary, NodeType::Detail, NodeType::Note] {
            if node_type != node.node_type {
                items.push(MenuItem::new(
                    format!("change_to_{:?}", node_type),
                    format!("Change to {:?}", node_type),
                    MenuItemAction::ChangeNodeType {
                        node_id: node.id,
                        new_type: node_type,
                    },
                    true,
                ).with_icon(Self::get_node_type_icon(node_type)));
            }
        }

        if !node.tags.is_empty() {
            items.push(MenuItem::new(
                "tags_header".to_string(),
                "Tags".to_string(),
                MenuItemAction::EditNode {
                    node_id: node.id,
                },
                false,
            ).with_separator());
            
            // Tag management items would be added here dynamically
        }

        items.extend(vec![
            MenuItem::new(
                "toggle_collapse".to_string(),
                if node.is_collapsed { "Expand Node" } else { "Collapse Node" }.to_string(),
                MenuItemAction::ToggleCollapse {
                    node_id: node.id,
                },
                true,
            ).with_icon(if node.is_collapsed { "-expand".to_string() } else { "-collapse".to_string() }),
            
            MenuItem::new(
                "set_color".to_string(),
                "Set Color".to_string(),
                MenuItemAction::SetNodeColor {
                    node_id: node.id,
                    color: node.get_color().to_string(),
                },
                true,
            ).with_icon("🎨".to_string()),
            
            MenuItem::new(
                "copy_content".to_string(),
                "Copy Content".to_string(),
                MenuItemAction::CopyNodeContent {
                    node_id: node.id,
                },
                !node.content.is_empty(),
            ).with_icon("📄".to_string()),
        ]);

        Self {
            menu_type: BrainstormingMenuType::Node,
            items,
            position,
            target_id: Some(node.id.to_string()),
        }
    }

    /// Create a context menu for a connection
    pub fn for_connection(
        connection: &BrainstormConnection,
        position: (f32, f32),
    ) -> Self {
        let mut items = vec![
            MenuItem::new(
                "edit_connection".to_string(),
                "Edit Connection".to_string(),
                MenuItemAction::EditConnection {
                    connection_id: connection.id,
                },
                true,
            ).with_icon("✏️".to_string()),
            
            MenuItem::new(
                "delete_connection".to_string(),
                "Delete Connection".to_string(),
                MenuItemAction::DeleteConnection {
                    connection_id: connection.id,
                },
                true,
            ).with_icon("🗑️".to_string()),
        ];

        // Connection type submenu
        for conn_type in [ConnectionType::Hierarchical, ConnectionType::CrossReference, ConnectionType::Supporting, ConnectionType::Contrasting] {
            if conn_type != connection.connection_type {
                items.push(MenuItem::new(
                    format!("change_conn_to_{:?}", conn_type),
                    format!("Change to {:?}", conn_type),
                    MenuItemAction::ChangeConnectionType {
                        connection_id: connection.id,
                        new_type: conn_type,
                    },
                    true,
                ).with_icon(Self::get_connection_type_icon(conn_type)));
            }
        }

        items.push(MenuItem::new(
            "set_conn_color".to_string(),
            "Set Color".to_string(),
            MenuItemAction::SetConnectionColor {
                connection_id: connection.id,
                color: connection.get_color().to_string(),
            },
            true,
        ).with_icon("🎨".to_string()));

        Self {
            menu_type: BrainstormingMenuType::Connection,
            items,
            position,
            target_id: Some(connection.id.to_string()),
        }
    }

    /// Create a context menu for the canvas
    pub fn for_canvas(position: (f32, f32), has_central_node: bool) -> Self {
        let mut items = vec![
            MenuItem::new(
                "add_central".to_string(),
                "Add Central Idea".to_string(),
                MenuItemAction::CreateNode {
                    node_type: NodeType::Central,
                    position: Some(position),
                },
                !has_central_node,
            ).with_icon("🎯".to_string()),
            
            MenuItem::new(
                "add_primary".to_string(),
                "Add Primary Branch".to_string(),
                MenuItemAction::CreateNode {
                    node_type: NodeType::Primary,
                    position: Some(position),
                },
                has_central_node,
            ).with_icon("📊".to_string()),
            
            MenuItem::new(
                "add_secondary".to_string(),
                "Add Secondary Branch".to_string(),
                MenuItemAction::CreateNode {
                    node_type: NodeType::Secondary,
                    position: Some(position),
                },
                has_central_node,
            ).with_icon("📈".to_string()),
            
            MenuItem::new(
                "add_detail".to_string(),
                "Add Detail".to_string(),
                MenuItemAction::CreateNode {
                    node_type: NodeType::Detail,
                    position: Some(position),
                },
                has_central_node,
            ).with_icon("📋".to_string()),
            
            MenuItem::new(
                "add_note".to_string(),
                "Add Note".to_string(),
                MenuItemAction::AddNote {
                    parent_node_id: None,
                    position,
                },
                true,
            ).with_icon("📝".to_string()).with_separator(),
        ];

        // Layout submenu
        items.push(MenuItem::new(
            "layout_header".to_string(),
            "Layout".to_string(),
            MenuItemAction::ApplyLayout {
                algorithm: crate::ui::tools::brainstorming_data::LayoutAlgorithm::Radial,
            },
            false,
        ));

        for algorithm in [
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::Radial,
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::Hierarchical,
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::ForceDirected,
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::Grid,
        ] {
            items.push(MenuItem::new(
                format!("layout_{:?}", algorithm),
                format!("Apply {:?} Layout", algorithm),
                MenuItemAction::ApplyLayout { algorithm },
                true,
            ).with_icon(Self::get_layout_icon(algorithm)));
        }

        items.extend(vec![
            MenuItem::new(
                "import_mindmap".to_string(),
                "Import Mindmap".to_string(),
                MenuItemAction::ImportMindmap,
                true,
            ).with_icon("📥".to_string()),
            
            MenuItem::new(
                "export_json".to_string(),
                "Export as JSON".to_string(),
                MenuItemAction::ExportMindmap {
                    format: ExportFormat::Json,
                },
                true,
            ).with_icon("📤".to_string()),
            
            MenuItem::new(
                "export_markdown".to_string(),
                "Export as Markdown".to_string(),
                MenuItemAction::ExportMindmap {
                    format: ExportFormat::Markdown,
                },
                true,
            ).with_icon("📄".to_string()).with_separator(),
            
            MenuItem::new(
                "toggle_grid".to_string(),
                "Toggle Grid".to_string(),
                MenuItemAction::ToggleGrid,
                true,
            ).with_icon("🔲".to_string()),
            
            MenuItem::new(
                "toggle_labels".to_string(),
                "Toggle Labels".to_string(),
                MenuItemAction::ToggleLabels,
                true,
            ).with_icon("🏷️".to_string()),
            
            MenuItem::new(
                "zoom_in".to_string(),
                "Zoom In".to_string(),
                MenuItemAction::ZoomIn,
                true,
            ).with_icon("🔍".to_string()),
            
            MenuItem::new(
                "zoom_out".to_string(),
                "Zoom Out".to_string(),
                MenuItemAction::ZoomOut,
                true,
            ).with_icon("🔎".to_string()),
            
            MenuItem::new(
                "reset_view".to_string(),
                "Reset View".to_string(),
                MenuItemAction::ResetView,
                true,
            ).with_icon("🔄".to_string()),
        ]);

        Self {
            menu_type: BrainstormingMenuType::Canvas,
            items,
            position,
            target_id: None,
        }
    }

    /// Get icon for node type
    fn get_node_type_icon(node_type: NodeType) -> String {
        match node_type {
            NodeType::Central => "🎯".to_string(),
            NodeType::Primary => "📊".to_string(),
            NodeType::Secondary => "📈".to_string(),
            NodeType::Detail => "📋".to_string(),
            NodeType::Note => "📝".to_string(),
        }
    }

    /// Get icon for connection type
    fn get_connection_type_icon(conn_type: ConnectionType) -> String {
        match conn_type {
            ConnectionType::Hierarchical => "🔗".to_string(),
            ConnectionType::CrossReference => "↩️".to_string(),
            ConnectionType::Supporting => "✅".to_string(),
            ConnectionType::Contrasting => "❌".to_string(),
        }
    }

    /// Get icon for layout algorithm
    fn get_layout_icon(algorithm: crate::ui::tools::brainstorming_data::LayoutAlgorithm) -> String {
        match algorithm {
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::Radial => "⭕".to_string(),
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::Hierarchical => "🌳".to_string(),
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::ForceDirected => "⚡".to_string(),
            crate::ui::tools::brainstorming_data::LayoutAlgorithm::Grid => "🔲".to_string(),
        }
    }

    /// Get the action for a menu item by ID
    pub fn get_action(&self, item_id: &str) -> Option<&MenuItemAction> {
        self.items.iter()
            .find(|item| item.id == item_id)
            .map(|item| &item.action)
    }

    /// Check if menu item is enabled
    pub fn is_item_enabled(&self, item_id: &str) -> bool {
        self.items.iter()
            .find(|item| item.id == item_id)
            .map(|item| item.enabled)
            .unwrap_or(false)
    }

    /// Update item enabled state
    pub fn update_item_enabled(&mut self, item_id: &str, enabled: bool) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == item_id) {
            item.enabled = enabled;
        }
    }
}

/// Context menu handler for brainstorming operations
pub struct BrainstormingContextMenuHandler;

impl BrainstormingContextMenuHandler {
    /// Create appropriate context menu based on context
    pub fn create_menu(
        menu_type: BrainstormingMenuType,
        position: (f32, f32),
        node: Option<&BrainstormNode>,
        connection: Option<&BrainstormConnection>,
        has_central_node: bool,
        has_selection: bool,
    ) -> ContextMenu {
        match menu_type {
            BrainstormingMenuType::Node => {
                if let Some(node) = node {
                    ContextMenu::for_node(node, position, has_selection)
                } else {
                    panic!("Node context menu requires node data");
                }
            },
            BrainstormingMenuType::Connection => {
                if let Some(connection) = connection {
                    ContextMenu::for_connection(connection, position)
                } else {
                    panic!("Connection context menu requires connection data");
                }
            },
            BrainstormingMenuType::Canvas => {
                ContextMenu::for_canvas(position, has_central_node)
            },
            BrainstormingMenuType::Toolbar => {
                // Toolbar menu would be implemented separately
                ContextMenu::for_canvas(position, has_central_node)
            },
        }
    }

    /// Handle menu item selection
    pub fn handle_menu_action(
        action: &MenuItemAction,
        // Additional parameters would be passed here for the actual implementation
    ) -> Result<(), String> {
        match action {
            MenuItemAction::CreateNode { node_type, position } => {
                // Handle node creation
                println!("Creating {:?} node at {:?}", node_type, position);
            },
            MenuItemAction::EditNode { node_id } => {
                // Handle node editing
                println!("Editing node: {}", node_id);
            },
            MenuItemAction::DeleteNode { node_id } => {
                // Handle node deletion
                println!("Deleting node: {}", node_id);
            },
            MenuItemAction::ChangeNodeType { node_id, new_type } => {
                // Handle node type change
                println!("Changing node {} to {:?}", node_id, new_type);
            },
            MenuItemAction::AddTag { node_id, tag } => {
                // Handle tag addition
                println!("Adding tag '{}' to node {}", tag, node_id);
            },
            MenuItemAction::RemoveTag { node_id, tag } => {
                // Handle tag removal
                println!("Removing tag '{}' from node {}", tag, node_id);
            },
            MenuItemAction::CreateConnection { from_node_id, to_node_id, connection_type } => {
                // Handle connection creation
                println!("Creating {:?} connection from {} to {:?}", 
                    connection_type, from_node_id, to_node_id);
            },
            MenuItemAction::EditConnection { connection_id } => {
                // Handle connection editing
                println!("Editing connection: {}", connection_id);
            },
            MenuItemAction::DeleteConnection { connection_id } => {
                // Handle connection deletion
                println!("Deleting connection: {}", connection_id);
            },
            MenuItemAction::ChangeConnectionType { connection_id, new_type } => {
                // Handle connection type change
                println!("Changing connection {} to {:?}", connection_id, new_type);
            },
            MenuItemAction::ApplyLayout { algorithm } => {
                // Handle layout application
                println!("Applying {:?} layout", algorithm);
            },
            MenuItemAction::ImportMindmap => {
                // Handle mindmap import
                println!("Importing mindmap");
            },
            MenuItemAction::ExportMindmap { format } => {
                // Handle mindmap export
                println!("Exporting mindmap as {:?}", format);
            },
            MenuItemAction::SearchNodes { query } => {
                // Handle node search
                println!("Searching for: {}", query);
            },
            MenuItemAction::ToggleGrid => {
                // Handle grid toggle
                println!("Toggling grid");
            },
            MenuItemAction::ToggleLabels => {
                // Handle labels toggle
                println!("Toggling labels");
            },
            MenuItemAction::ZoomIn => {
                // Handle zoom in
                println!("Zooming in");
            },
            MenuItemAction::ZoomOut => {
                // Handle zoom out
                println!("Zooming out");
            },
            MenuItemAction::ResetView => {
                // Handle view reset
                println!("Resetting view");
            },
            MenuItemAction::CopyNodeContent { node_id } => {
                // Handle content copying
                println!("Copying content from node {}", node_id);
            },
            MenuItemAction::CopyConnectionInfo { connection_id } => {
                // Handle connection info copying
                println!("Copying info from connection {}", connection_id);
            },
            MenuItemAction::DuplicateNode { node_id, new_position } => {
                // Handle node duplication
                println!("Duplicating node {} to {:?}", node_id, new_position);
            },
            MenuItemAction::ToggleCollapse { node_id } => {
                // Handle node collapse/expand
                println!("Toggling collapse for node {}", node_id);
            },
            MenuItemAction::SetNodeColor { node_id, color } => {
                // Handle node color change
                println!("Setting color {} for node {}", color, node_id);
            },
            MenuItemAction::SetConnectionColor { connection_id, color } => {
                // Handle connection color change
                println!("Setting color {} for connection {}", color, connection_id);
            },
            MenuItemAction::AddNote { parent_node_id, position } => {
                // Handle note addition
                println!("Adding note at {:?} with parent {:?}", position, parent_node_id);
            },
            MenuItemAction::ClearSearch => {
                // Handle search clear
                println!("Clearing search");
            },
        }
        
        Ok(())
    }
}