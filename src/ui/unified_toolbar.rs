//! Unified Toolbar System
//!
//! Provides theme-aware toolbar components that can be used consistently
//! across all application windows including main app and individual tool windows.
//!
//! ## Overview
//!
//! This refactored version improves:
//! - **Code organization and maintainability**: Better structure and separation of concerns
//! - **Reusability of common patterns**: Extracted reusable components and utilities
//! - **Type safety and error handling**: Enhanced validation and error handling
//! - **Documentation and clarity**: Comprehensive documentation and clear code structure
//! - **Extensibility for future enhancements**: Flexible architecture for future improvements
//!
//! ## Components
//!
//! - [`UnifiedToolbar`]: Main toolbar component with horizontal/vertical layouts
//! - [`ThemedMenuBar`]: Menu bar with theme-aware styling
//! - [`ThemedStatusBar`]: Status bar with multiple information displays
//! - [`ThemeSelector`]: Theme selection dropdown component
//! - [`EnhancedToolbar`]: Advanced toolbar with improved item management
//!
//! ## Usage
//!
//! ```rust
//! use crate::ui::unified_toolbar::{ToolbarItem, ThemeConfig};
//!
//! // Create toolbar items
//! let item = ToolbarItem::new("Bold", "B", true, "Bold text").unwrap();
//!
//! // Create theme configuration
//! let theme = ThemeConfig::from_colors("#f8f9fa", "#6c757d", "#dee2e6").unwrap();
//! ```
//!
//! ## Architecture
//!
//! The unified toolbar system follows these design principles:
//! - **Separation of concerns**: UI components separated from data models
//! - **Reusable patterns**: Common functionality extracted into utilities
//! - **Type safety**: Comprehensive validation and error handling
//! - **Theme consistency**: Centralized theme management
//! - **Extensibility**: Easy to add new components and features

/// Common toolbar item data structure
#[derive(Debug, Clone)]
pub struct ToolbarItem {
    /// Display label for the toolbar item
    pub label: String,
    /// Icon representation for the toolbar item
    pub icon: String,
    /// Whether the item is enabled and clickable
    pub enabled: bool,
    /// Tooltip text shown on hover
    pub tooltip: String,
}

impl ToolbarItem {
    /// Create a new toolbar item with validation
    pub fn new(
        label: &str,
        icon: &str,
        enabled: bool,
        tooltip: &str,
    ) -> Result<Self, &'static str> {
        if label.is_empty() {
            return Err("Toolbar item label cannot be empty");
        }
        Ok(ToolbarItem {
            label: label.to_string(),
            icon: icon.to_string(),
            enabled,
            tooltip: tooltip.to_string(),
        })
    }

    /// Validate the toolbar item
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.label.is_empty() {
            return Err("Toolbar item label cannot be empty");
        }
        if self.icon.is_empty() {
            return Err("Toolbar item icon cannot be empty");
        }
        Ok(())
    }
}

/// Theme configuration for toolbar components
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub background: String,
    pub text_color: String,
    pub border_color: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            background: "#f8f9fa".to_string(),
            text_color: "#6c757d".to_string(),
            border_color: "#dee2e6".to_string(),
        }
    }
}

impl ThemeConfig {
    /// Validate theme configuration
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.background.is_empty() {
            return Err("Theme background color cannot be empty");
        }
        if self.text_color.is_empty() {
            return Err("Theme text color cannot be empty");
        }
        if self.border_color.is_empty() {
            return Err("Theme border color cannot be empty");
        }
        Ok(())
    }

    /// Create a theme config from hex colors
    pub fn from_colors(
        background: &str,
        text_color: &str,
        border_color: &str,
    ) -> Result<Self, &'static str> {
        let config = ThemeConfig {
            background: background.to_string(),
            text_color: text_color.to_string(),
            border_color: border_color.to_string(),
        };
        config.validate()?;
        Ok(config)
    }
}

/// Common toolbar layout patterns and utilities
pub mod toolbar_patterns {
    use super::{ThemeConfig, ToolbarConfig, ToolbarItem};

    /// Predefined toolbar layouts for different tool categories
    pub const WRITING_TOOLS_LAYOUT: &[&str] = &["bold", "italic", "underline", "strikethrough"];
    pub const EDITING_TOOLS_LAYOUT: &[&str] = &["cut", "copy", "paste", "undo", "redo"];
    pub const VIEW_TOOLS_LAYOUT: &[&str] = &["zoom-in", "zoom-out", "fullscreen", "split"];

    /// Create standard toolbar items for writing tools with proper validation
    pub fn create_writing_toolbar_items() -> Result<Vec<ToolbarItem>, &'static str> {
        let items = vec![
            ToolbarItem::new("Bold", "B", true, "Bold text (Ctrl+B)")?,
            ToolbarItem::new("Italic", "I", true, "Italic text (Ctrl+I)")?,
            ToolbarItem::new("Underline", "U", true, "Underline text (Ctrl+U)")?,
        ];

        // Validate all items
        for item in &items {
            item.validate()?;
        }

        Ok(items)
    }

    /// Create standard theme configurations with validation
    pub fn get_light_theme() -> Result<ThemeConfig, &'static str> {
        ThemeConfig::from_colors("#f8f9fa", "#6c757d", "#dee2e6")
    }

    pub fn get_dark_theme() -> Result<ThemeConfig, &'static str> {
        ThemeConfig::from_colors("#343a40", "#ffffff", "#6c757d")
    }

    /// Create a complete toolbar configuration
    pub fn create_toolbar_config(
        layout: &[&str],
        theme: &ThemeConfig,
    ) -> Result<ToolbarConfig, String> {
        let mut items = Vec::new();

        for item_name in layout {
            let item = ToolbarItem::new(
                item_name,
                &item_name[0..1].to_uppercase(),
                true,
                &format!("{} tool", item_name),
            )?;
            items.push(item);
        }

        Ok(ToolbarConfig::new(items, theme.clone()))
    }
}

/// Complete toolbar configuration
#[derive(Debug, Clone)]
pub struct ToolbarConfig {
    pub items: Vec<ToolbarItem>,
    pub theme: ThemeConfig,
}

impl ToolbarConfig {
    /// Create a new toolbar configuration
    pub fn new(items: Vec<ToolbarItem>, theme: ThemeConfig) -> Self {
        ToolbarConfig { items, theme }
    }
}

impl ToolbarConfig {
    /// Validate the entire toolbar configuration
    pub fn validate(&self) -> Result<(), String> {
        self.theme.validate()?;

        if self.items.is_empty() {
            return Err("Toolbar must have at least one item".to_string());
        }

        for (i, item) in self.items.iter().enumerate() {
            if let Err(e) = item.validate() {
                return Err(format!("Item {}: {}", i, e));
            }
        }

        Ok(())
    }
}

/// Unified toolbar component definition
slint::slint! {
    import { Button, HorizontalBox, VerticalBox } from "std-widgets.slint";

    // Common base component for themed containers
    global ThemedContainer {
        // Base theme properties for all themed components
        in-out property <brush> background: #f8f9fa;
    in-out property <brush> border-color: #00000000; // Transparent by default for minimalist design
    in-out property <brush> text-color: #6c757d;

    // Component behavior flags
        in-out property <bool> show-border: false; // Disabled by default for minimalist design
    in-out property <bool> show-separators: false; // Disabled by default for minimalist design
    in-out property <bool> is-visible: true;

    // Layout properties
        in-out property <float> padding: 8;
    in-out property <float> spacing: 4;
    in-out property <float> border-width: 0; // No borders by default
    in-out property <float> border-radius: 0; // No rounded corners by default
}

    // Reusable toolbar button component
    //
    // Provides a consistent button appearance across all toolbar components.
    // Handles click events and maintains proper sizing and styling.
    component ToolbarButton {
    in-out property <string> text: "";
    in-out property <bool> enabled: true;
    callback clicked();
    Button {
        text: root.text;
        enabled: root.enabled;
        clicked => {
            root.clicked();
        }
    }
}

    // Reusable themed rectangle container
    //
    // Provides a consistent container for toolbar items with optional borders.
    // Supports theming and layout customization.
    component ThemedRectangle {
    in-out property <brush> background: #00000000;
    in-out property <bool> show-border: false;
    in-out property <float> inner-padding: 8;
    in-out property <float> inner-spacing: 4;
    @children
}

    // Theme-aware toolbar component
    export component UnifiedToolbar {
    in-out property <brush> background: #f8f9fa;
    in-out property <bool> show-separators: true;
    in-out property <string> layout: "horizontal"; // horizontal or vertical

        // Toolbar items data
        in-out property <[string]> item-labels: [];
    in-out property <[string]> item-icons: [];
    in-out property <[bool]> item-enabled: [];
    in-out property <[string]> item-tooltips: [];

    // Callbacks
        callback item-clicked(index: int);
    callback theme-changed();

    // Initialize toolbar
        init => {
            // Subscribe to theme changes
            root.theme-changed();
    }

        // Update appearance when theme changes
        theme-changed => {
            // This would be called when theme changes to update colors
            // In a real implementation, this would be connected to the theme manager
            // TODO: Implement theme synchronization with global theme manager
        }

        // Layout based on orientation - using conditional visibility
        HorizontalBox {
        visible: layout == "horizontal";
        spacing: 0;
        padding: 8px;

        // Toolbar items
            for label[index] in item-labels: Rectangle {
            background: #00000000;
            border-color: #00000000; // Always transparent for minimalist design
                border-width: 0px; // No borders for minimalist design
                border-radius: 0px; // No rounded corners for minimalist design
                HorizontalBox {
                padding: 8px;
                spacing: 4px;
                ToolbarButton {
                    text: item-icons[index] + " " + label;
                    enabled: item-enabled[index];
                    clicked => {
                        root.item-clicked(index);
                    }
                }
            }
        }
        Rectangle { } // Spacer
        }

    VerticalBox {
        visible: layout == "vertical";
        spacing: 0;
        padding: 8px;

        // Toolbar items
            for label[index] in item-labels: Rectangle {
            background: #00000000;
            border-color: #00000000; // Always transparent for minimalist design
                border-width: 0px; // No borders for minimalist design
                border-radius: 0px; // No rounded corners for minimalist design
                HorizontalBox {
                padding: 6px;
                spacing: 4px;
                ToolbarButton {
                    text: item-icons[index] + " " + label;
                    enabled: item-enabled[index];
                    preferred-width: 120px;
                    clicked => {
                        root.item-clicked(index);
                    }
                }
            }
        }
        Rectangle { } // Spacer
        }
}

    // Themed menu bar component
    export component ThemedMenuBar {
    in-out property <brush> background: #343a40;
    in-out property <brush> text-color: #ffffff;
    in-out property <[string]> menu-labels: ["File", "View", "Theme", "Help"];
    in-out property <[string]> menu-shortcuts: ["", "", "", "", ""];

    // Callbacks
        callback menu-selected(index: int);
    callback theme-changed();
    HorizontalBox {
        spacing: 0;
        padding: 12px;

        // Menu items
            for label[index] in menu-labels: Rectangle {
            background: #00000000; // Transparent background for minimalist design
                border-color: #00000000; // No borders
                border-width: 0px;
            HorizontalBox {
                padding: 8px;
                ToolbarButton {
                    text: label + (menu-shortcuts[index] != "" ? " (" + menu-shortcuts[index] + ")" : "");
                    enabled: true;
                    clicked => {
                        root.menu-selected(index);
                    }
                }
            }
        }
        Rectangle { } // Spacer
        }
}

    // Status bar component
    //
    // Displays application status information including:
    // - Current status message
    // - Word count
    // - Line and column position
    // - Current theme information
    //
    // The status bar provides real-time feedback to users about their document
    // and application state.
    export component ThemedStatusBar {
        // Visual properties
        in-out property <brush> background: #f8f9fa;
    in-out property <brush> text-color: #6c757d;

    // Status information
        in-out property <string> status-text: "Ready";
    in-out property <string> word-count: "0 words";
    in-out property <string> line-info: "Ln 1, Col 1";
    in-out property <string> theme-info: "Theme: Light";
    HorizontalBox {
        spacing: 24px;
        padding: 8px;

        // Current application status
        Text {
            text: "Status: " + status-text;
            color: text-color;
            font-size: 12px;
            vertical-alignment: center;
        }

        // Document word count with special green styling
        Text {
            text: word-count;
            color: #28a745;  // Success green color
            font-size: 12px;
            vertical-alignment: center;
        }

        // Current cursor position
        Text {
            text: line-info;
            color: text-color;
            font-size: 12px;
            vertical-alignment: center;
        }

        Rectangle { } // Spacer for right-aligned content

        // Current theme information
        Text {
            text: theme-info;
            color: text-color;
            font-size: 11px;
            vertical-alignment: center;
        }
    }
}

    // Theme selector dropdown
    export component ThemeSelector {
    in-out property <brush> background: #343a40;
    in-out property <brush> text-color: #ffffff;
    in-out property <bool> is-visible: false;
    in-out property <[string]> theme-names: ["Light", "Dark", "High Contrast"];
    in-out property <[string]> theme-descriptions: ["Clean and bright", "Sleek and modern", "High contrast for accessibility"];

    // Callbacks
        callback theme-selected(index: int);
    callback close-selector();
    VerticalBox {
        visible: is-visible;
        spacing: 0;
        padding: 8px;

        // Theme options
            for name[index] in theme-names: Rectangle {
            background: #00000000;
            HorizontalBox {
                padding: 8px;
                ToolbarButton {
                    text: name;
                    clicked => {
                        root.theme-selected(index);
                    }
                }
            }
        }

        // Close button
            Rectangle {
            background: #00000000;
            preferred-height: 36px;
            HorizontalBox {
                padding: 8px;
                ToolbarButton {
                    text: "Close";
                    clicked => {
                        root.close-selector();
                    }
                }
            }
        }
    }
}

    // Enhanced toolbar with better item management
    //
    // Advanced toolbar component with improved validation, error handling,
    // and flexible display options. Supports both icon-only and label-only
    // modes for different use cases.
    export component EnhancedToolbar {
        // Visual properties
        in-out property <brush> background: #f8f9fa;
    in-out property <bool> show-separators: true;
    in-out property <string> layout: "horizontal";

    // Display options
        in-out property <bool> show-labels: true;
    in-out property <bool> show-icons: true;

    // Item data arrays
        in-out property <[string]> item-labels: [];
    in-out property <[string]> item-icons: [];
    in-out property <[bool]> item-enabled: [];
    in-out property <[string]> item-tooltips: [];
    in-out property <[string]> item-styles: []; // Additional styling options

        // Callbacks
        callback item-clicked(index: int);
    callback theme-changed();

    // Enhanced validation with bounds checking
        function is-valid-index(index: int) -> bool {
        return index >= 0 && index < item-labels.length && item-labels.length > 0;
    }

    // Validate array bounds before access
        function safe-get-label(index: int) -> string {
        if (root.is-valid-index(index)) {
            return item-labels[index];
        } else {
            return "";
        }
    }

    // Validate array bounds before access for icons
        function safe-get-icon(index: int) -> string {
        if (root.is-valid-index(index) && index < item-icons.length) {
            return item-icons[index];
        } else {
            return "";
        }
    }

    // Initialize toolbar
        init => {
        root.theme-changed();
    }

        // Update appearance when theme changes
        theme-changed => {
            // Theme change handling
        }

        // Enhanced toolbar items layout
        HorizontalBox {
        visible: layout == "horizontal";
        spacing: 0;
        padding: 8px;

        // Enhanced toolbar items
            for label[index] in item-labels: ThemedRectangle {
            background: #00000000;
            show-border: show-separators;
            HorizontalBox {
                padding: 8px;
                spacing: 4px;
                ToolbarButton {
                    text: show-icons && show-labels ? (root.safe-get-icon(index) + " " + label) : (show-labels ? label : root.safe-get-icon(index));
                    enabled: root.is-valid-index(index) && item-enabled[index];
                    visible: is-valid-index(index);
                    clicked => {
                        if (root.is-valid-index(index)) {
                            root.item-clicked(index);
                        }
                    }
                }
            }
        }
        Rectangle { } // Spacer
    }

    VerticalBox {
        visible: layout == "vertical";
        spacing: 0;
        padding: 8px;
        for label[index] in item-labels: ThemedRectangle {
            background: #00000000;
            show-border: show-separators;
            HorizontalBox {
                padding: 6px;
                spacing: 4px;
                ToolbarButton {
                    text: show-icons && show-labels ? (root.safe-get-icon(index) + " " + label) : (show-labels ? label : root.safe-get-icon(index));
                    enabled: root.is-valid-index(index) && item-enabled[index];
                    preferred-width: 120px;
                    visible: is-valid-index(index);
                    clicked => {
                        if (root.is-valid-index(index)) {
                            root.item-clicked(index);
                        }
                    }
                }
            }
        }
        Rectangle { } // Spacer
        }
}

    // End of slint! macro block
    // Note: The slint! macro block ends at line 599, but the Rust code continues
}

// End of file
