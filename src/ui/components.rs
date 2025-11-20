//! Unified UI Components Library
//!
//! Provides theme-aware UI components for consistent styling across
//! all application windows including main app and individual tool windows.

/// Component size variants
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl ComponentSize {
    pub fn to_height(&self) -> f32 {
        match self {
            ComponentSize::Small => 28.0,
            ComponentSize::Medium => 36.0,
            ComponentSize::Large => 44.0,
            ComponentSize::ExtraLarge => 52.0,
        }
    }

    pub fn to_font_size(&self) -> f32 {
        match self {
            ComponentSize::Small => 12.0,
            ComponentSize::Medium => 14.0,
            ComponentSize::Large => 16.0,
            ComponentSize::ExtraLarge => 18.0,
        }
    }
}

/// Button style variants
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Ghost,
    Link,
}

/// Theme-aware button component
#[derive(Debug, Clone)]
pub struct ThemedButton {
    pub text: String,
    pub style: ButtonStyle,
    pub size: ComponentSize,
    pub enabled: bool,
    pub tooltip: String,
}

impl ThemedButton {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: ButtonStyle::Primary,
            size: ComponentSize::Medium,
            enabled: true,
            tooltip: String::new(),
        }
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_tooltip(mut self, tooltip: &str) -> Self {
        self.tooltip = tooltip.to_string();
        self
    }
}

/// Menu item for theme-aware menus
#[derive(Debug, Clone)]
pub struct ThemedMenuItem {
    pub text: String,
    pub shortcut: String,
    pub enabled: bool,
    pub checked: bool,
    pub submenu: Option<Vec<ThemedMenuItem>>,
}

impl ThemedMenuItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            shortcut: String::new(),
            enabled: true,
            checked: false,
            submenu: None,
        }
    }

    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = shortcut.to_string();
        self
    }

    pub fn with_submenu(mut self, items: Vec<ThemedMenuItem>) -> Self {
        self.submenu = Some(items);
        self
    }
}

/// Toolbar item with theme support
#[derive(Debug, Clone)]
pub struct ThemedToolbarItem {
    pub text: String,
    pub icon: String,
    pub tooltip: String,
    pub enabled: bool,
    pub style: ButtonStyle,
}

impl ThemedToolbarItem {
    pub fn new(text: &str, icon: &str) -> Self {
        Self {
            text: text.to_string(),
            icon: icon.to_string(),
            tooltip: format!("{} ({})", text, icon),
            enabled: true,
            style: ButtonStyle::Primary,
        }
    }

    pub fn with_tooltip(mut self, tooltip: &str) -> Self {
        self.tooltip = tooltip.to_string();
        self
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }
}

/// Status bar item with theme support
#[derive(Debug, Clone)]
pub struct ThemedStatusItem {
    pub text: String,
    pub priority: u8, // 0 = low priority, 255 = high priority
    pub style: ButtonStyle,
}

impl ThemedStatusItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            priority: 100,
            style: ButtonStyle::Secondary,
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }
}

/// Input field with theme support
#[derive(Debug, Clone)]
pub struct ThemedInput {
    pub placeholder: String,
    pub value: String,
    pub enabled: bool,
    pub readonly: bool,
    pub multiline: bool,
    pub password: bool,
    pub size: ComponentSize,
}

impl ThemedInput {
    pub fn new() -> Self {
        Self {
            placeholder: String::new(),
            value: String::new(),
            enabled: true,
            readonly: false,
            multiline: false,
            password: false,
            size: ComponentSize::Medium,
        }
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }

    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    pub fn with_size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

/// Panel with theme support
#[derive(Debug, Clone)]
pub struct ThemedPanel {
    pub title: String,
    pub collapsible: bool,
    pub collapsed: bool,
    pub enabled: bool,
}

impl ThemedPanel {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            collapsible: false,
            collapsed: false,
            enabled: true,
        }
    }

    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }
}

/// Tab widget with theme support
#[derive(Debug, Clone)]
pub struct ThemedTab {
    pub title: String,
    pub content: String, // This would be a component reference in real implementation
    pub enabled: bool,
    pub closable: bool,
}

impl ThemedTab {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: String::new(),
            enabled: true,
            closable: false,
        }
    }

    pub fn with_closable(mut self) -> Self {
        self.closable = true;
        self
    }
}

/// Color utility functions for theme conversion
pub mod color_utils {
    use super::ButtonStyle;
    use crate::ui::theme_manager::ThemeColors;
    use slint::Color;
    use std::collections::HashMap;

    /// Convert hex color string to Slint Color
    pub fn hex_to_color(hex: &str) -> Color {
        if hex.starts_with('#') && hex.len() == 7 {
            let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
            Color::from_argb_u8(255, r, g, b)
        } else {
            Color::from_argb_u8(255, 255, 255, 255) // White fallback
        }
    }

    /// Convert theme colors to Slint-compatible HashMap
    pub fn theme_colors_to_slint(colors: &ThemeColors) -> HashMap<String, Color> {
        let mut slint_colors = HashMap::new();

        slint_colors.insert("primary-bg".to_string(), hex_to_color(&colors.primary_bg));
        slint_colors.insert(
            "secondary-bg".to_string(),
            hex_to_color(&colors.secondary_bg),
        );
        slint_colors.insert("accent".to_string(), hex_to_color(&colors.accent));
        slint_colors.insert(
            "text-primary".to_string(),
            hex_to_color(&colors.text_primary),
        );
        slint_colors.insert(
            "text-secondary".to_string(),
            hex_to_color(&colors.text_secondary),
        );
        slint_colors.insert("text-accent".to_string(), hex_to_color(&colors.text_accent));
        slint_colors.insert("border".to_string(), hex_to_color(&colors.border));
        slint_colors.insert("button-bg".to_string(), hex_to_color(&colors.button_bg));
        slint_colors.insert("button-text".to_string(), hex_to_color(&colors.button_text));
        slint_colors.insert(
            "button-hover".to_string(),
            hex_to_color(&colors.button_hover),
        );
        slint_colors.insert("menu-bg".to_string(), hex_to_color(&colors.menu_bg));
        slint_colors.insert("toolbar-bg".to_string(), hex_to_color(&colors.toolbar_bg));
        slint_colors.insert("status-bg".to_string(), hex_to_color(&colors.status_bg));
        slint_colors.insert("editor-bg".to_string(), hex_to_color(&colors.editor_bg));
        slint_colors.insert("title-bg".to_string(), hex_to_color(&colors.title_bg));
        slint_colors.insert("window-bg".to_string(), hex_to_color(&colors.window_bg));
        slint_colors.insert("panel-bg".to_string(), hex_to_color(&colors.panel_bg));
        slint_colors.insert("input-bg".to_string(), hex_to_color(&colors.input_bg));
        slint_colors.insert("input-text".to_string(), hex_to_color(&colors.input_text));
        slint_colors.insert(
            "input-border".to_string(),
            hex_to_color(&colors.input_border),
        );
        slint_colors.insert("input-focus".to_string(), hex_to_color(&colors.input_focus));

        slint_colors
    }

    /// Get button colors based on style and theme
    pub fn get_button_colors(
        style: &ButtonStyle,
        colors: &ThemeColors,
    ) -> (String, String, String) {
        match style {
            ButtonStyle::Primary => (
                colors.button_bg.clone(),
                colors.button_text.clone(),
                colors.button_hover.clone(),
            ),
            ButtonStyle::Secondary => (
                colors.secondary_bg.clone(),
                colors.text_primary.clone(),
                colors.border.clone(),
            ),
            ButtonStyle::Success => (
                "#28a745".to_string(),
                "#ffffff".to_string(),
                "#20c997".to_string(),
            ),
            ButtonStyle::Danger => (
                "#dc3545".to_string(),
                "#ffffff".to_string(),
                "#bd2130".to_string(),
            ),
            ButtonStyle::Warning => (
                "#ffc107".to_string(),
                "#212529".to_string(),
                "#fd7e14".to_string(),
            ),
            ButtonStyle::Info => (
                "#17a2b8".to_string(),
                "#ffffff".to_string(),
                "#138496".to_string(),
            ),
            ButtonStyle::Ghost => (
                "transparent".to_string(),
                colors.text_primary.clone(),
                colors.accent.clone(),
            ),
            ButtonStyle::Link => (
                "transparent".to_string(),
                colors.accent.clone(),
                colors.text_accent.clone(),
            ),
        }
    }
}

/// Layout helper functions
pub mod layout {
    use super::ComponentSize;

    /// Calculate spacing based on component size
    pub fn get_spacing(size: &ComponentSize) -> f32 {
        match size {
            ComponentSize::Small => 4.0,
            ComponentSize::Medium => 8.0,
            ComponentSize::Large => 12.0,
            ComponentSize::ExtraLarge => 16.0,
        }
    }

    /// Calculate padding based on component size
    pub fn get_padding(size: &ComponentSize) -> f32 {
        match size {
            ComponentSize::Small => 6.0,
            ComponentSize::Medium => 12.0,
            ComponentSize::Large => 16.0,
            ComponentSize::ExtraLarge => 20.0,
        }
    }
}

/// Component factory for creating theme-aware components
pub struct ComponentFactory;

impl ComponentFactory {
    /// Create a primary action button
    pub fn create_primary_button(text: &str) -> ThemedButton {
        ThemedButton::new(text)
            .with_style(ButtonStyle::Primary)
            .with_size(ComponentSize::Medium)
    }

    /// Create a secondary action button
    pub fn create_secondary_button(text: &str) -> ThemedButton {
        ThemedButton::new(text)
            .with_style(ButtonStyle::Secondary)
            .with_size(ComponentSize::Medium)
    }

    /// Create a danger/action button
    pub fn create_danger_button(text: &str) -> ThemedButton {
        ThemedButton::new(text)
            .with_style(ButtonStyle::Danger)
            .with_size(ComponentSize::Medium)
    }

    /// Create a menu item
    pub fn create_menu_item(text: &str) -> ThemedMenuItem {
        ThemedMenuItem::new(text)
    }

    /// Create a toolbar item
    pub fn create_toolbar_item(text: &str, icon: &str) -> ThemedToolbarItem {
        ThemedToolbarItem::new(text, icon).with_style(ButtonStyle::Primary)
    }

    /// Create a status item
    pub fn create_status_item(text: &str) -> ThemedStatusItem {
        ThemedStatusItem::new(text).with_style(ButtonStyle::Secondary)
    }

    /// Create an input field
    pub fn create_input() -> ThemedInput {
        ThemedInput::new().with_size(ComponentSize::Medium)
    }

    /// Create a panel
    pub fn create_panel(title: &str) -> ThemedPanel {
        ThemedPanel::new(title)
    }

    /// Create a tab
    pub fn create_tab(title: &str) -> ThemedTab {
        ThemedTab::new(title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = ThemedButton::new("Test Button")
            .with_style(ButtonStyle::Primary)
            .with_size(ComponentSize::Large);

        assert_eq!(button.text, "Test Button");
        assert_eq!(button.style, ButtonStyle::Primary);
        assert_eq!(button.size, ComponentSize::Large);
    }

    #[test]
    fn test_menu_item_creation() {
        let menu_item = ThemedMenuItem::new("File")
            .with_shortcut("Ctrl+F")
            .with_submenu(vec![
                ThemedMenuItem::new("New"),
                ThemedMenuItem::new("Open"),
            ]);

        assert_eq!(menu_item.text, "File");
        assert_eq!(menu_item.shortcut, "Ctrl+F");
        assert!(menu_item.submenu.is_some());
    }

    #[test]
    fn test_component_factory() {
        let button = ComponentFactory::create_primary_button("Save");
        assert_eq!(button.text, "Save");
        assert_eq!(button.style, ButtonStyle::Primary);

        let toolbar_item = ComponentFactory::create_toolbar_item("New", "plus");
        assert_eq!(toolbar_item.text, "New");
        assert_eq!(toolbar_item.icon, "plus");
    }

    #[test]
    fn test_color_utils() {
        let hex_color = "#ff0000";
        let color = color_utils::hex_to_color(hex_color);
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn test_layout_helpers() {
        assert_eq!(layout::get_spacing(&ComponentSize::Small), 4.0);
        assert_eq!(layout::get_spacing(&ComponentSize::Medium), 8.0);
        assert_eq!(layout::get_padding(&ComponentSize::Large), 16.0);
    }
}
