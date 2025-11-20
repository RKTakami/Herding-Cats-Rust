# Herding Cats Rust Theme System Guide

## Overview

The Herding Cats Rust application features a comprehensive, centralized theme management system that provides unified styling across all application windows including the main app window and individual writing tool windows. This guide covers the theme system architecture, usage, and customization options.

## Table of Contents

1. [Theme System Architecture](#theme-system-architecture)
2. [Built-in Themes](#built-in-themes)
3. [Using Themes in Code](#using-themes-in-code)
4. [Creating Custom Themes](#creating-custom-themes)
5. [Theme-Aware Components](#theme-aware-components)
6. [Theme Persistence](#theme-persistence)
7. [Best Practices](#best-practices)
8. [API Reference](#api-reference)

## Theme System Architecture

### Core Components

The theme system consists of several key components:

- **ThemeManager**: Central theme management with switching capabilities
- **ThemeColors**: Complete color palette definition
- **Theme**: Complete theme definition with metadata
- **ThemeType**: Enum for built-in and custom theme types
- **Unified Toolbar System**: Theme-aware toolbars and menus
- **Theme Preview Interface**: Visual theme selection and preview

### File Structure

```
src/
├── ui/
│   ├── theme_manager.rs      # Core theme management
│   ├── components.rs         # Theme-aware UI components
│   ├── unified_toolbar.rs    # Theme-aware toolbars
│   └── theme_preview.rs      # Theme preview interface
├── settings.rs               # Theme persistence
└── docs/
    └── THEME_SYSTEM_GUIDE.md # This documentation
```

## Built-in Themes

The application comes with three built-in themes:

### Light Theme
- **Description**: Clean and bright theme for comfortable writing
- **Use Case**: Daytime writing, bright environments
- **Colors**: White backgrounds, dark text, blue accents

### Dark Theme
- **Description**: Sleek dark theme for focused writing sessions
- **Use Case**: Low-light environments, reduced eye strain
- **Colors**: Dark backgrounds, light text, purple accents

### High Contrast Theme
- **Description**: High contrast theme for maximum accessibility
- **Use Case**: Accessibility requirements, visual impairments
- **Colors**: Black and white only, maximum contrast

## Using Themes in Code

### Basic Theme Usage

```rust
use herding_cats_rust::ui::{get_theme_manager, get_current_theme_colors, get_theme_color};

// Get the current theme manager
let theme_manager = get_theme_manager();

// Get all available themes
let available_themes = theme_manager.get_available_themes();

// Switch to dark theme
theme_manager.set_theme(ThemeType::Dark).unwrap();

// Get current theme colors
let colors = get_current_theme_colors();
println!("Primary background: {}", colors.primary_bg);

// Get specific color
let button_color = get_theme_color("button-bg");
```

### Theme-Aware Window Creation

```rust
use herding_cats_rust::ui::{ThemedButton, ButtonStyle, ComponentFactory};

// Create theme-aware buttons
let primary_button = ThemedButton::new("Save")
    .with_style(ButtonStyle::Primary)
    .with_size(ComponentSize::Medium);

let secondary_button = ThemedButton::new("Cancel")
    .with_style(ButtonStyle::Secondary)
    .with_size(ComponentSize::Medium);

// Create toolbar items
let toolbar_items = vec![
    ComponentFactory::create_toolbar_item("New", "📄"),
    ComponentFactory::create_toolbar_item("Open", "📂"),
    ComponentFactory::create_toolbar_item("Save", "💾"),
];
```

### Slint Integration

```rust
// In your .slint file
import { Button, Rectangle } from "std-widgets.slint";

export component ThemedWindow inherits Window {
    // Get theme colors from Rust
    in-out property<string> primary-bg: "#ffffff";
    in-out property<string> secondary-bg: "#f8f9fa";
    in-out property<string> accent: "#007bff";
    
    Rectangle {
        background: primary-bg;
        
        Button {
            text: "Theme-Aware Button";
            background: accent;
            color: "#ffffff";
        }
    }
}
```

## Creating Custom Themes

### Programmatic Custom Theme Creation

```rust
use herding_cats_rust::ui::{ThemeColors, Theme};

// Define custom colors
let custom_colors = ThemeColors {
    primary_bg: "#1e1e1e".to_string(),
    secondary_bg: "#2d2d30".to_string(),
    accent: "#0078d7".to_string(),
    text_primary: "#ffffff".to_string(),
    text_secondary: "#cccccc".to_string(),
    // ... other colors
};

// Create custom theme
let custom_theme = Theme::from_custom_colors("My Custom Theme", custom_colors);

// Add to theme manager
let mut theme_manager = get_theme_manager();
theme_manager.add_theme(custom_theme);

// Switch to custom theme
theme_manager.set_theme_by_name("My Custom Theme").unwrap();
```

### Theme Color Guidelines

When creating custom themes, follow these guidelines:

1. **Contrast Ratio**: Ensure text-background contrast meets WCAG AA standards (4.5:1 for normal text)
2. **Color Harmony**: Use complementary colors that work well together
3. **Accessibility**: Consider color blindness and visual impairments
4. **Consistency**: Maintain consistent color usage across components

### Color Palette Structure

```rust
ThemeColors {
    // Background colors
    primary_bg: "#ffffff",      // Main content background
    secondary_bg: "#f8f9fa",    // Secondary areas, panels
    panel_bg: "#ffffff",        // Individual panels
    window_bg: "#ffffff",       // Window background
    editor_bg: "#ffffff",       // Text editor background
    input_bg: "#ffffff",        // Input fields
    
    // Text colors
    text_primary: "#212529",    // Main text
    text_secondary: "#6c757d",  // Secondary text
    text_accent: "#007bff",     // Accent text, links
    
    // Interactive colors
    button_bg: "#007bff",       // Primary buttons
    button_text: "#ffffff",     // Button text
    button_hover: "#0056b3",    // Button hover
    
    // Structural colors
    accent: "#007bff",          // Accents, highlights
    border: "#dee2e6",          // Borders, separators
    menu_bg: "#343a40",         // Menu backgrounds
    toolbar_bg: "#f8f9fa",      // Toolbar backgrounds
    status_bg: "#f8f9fa",       // Status bar backgrounds
    title_bg: "#e9ecef",        // Title bar backgrounds
    input_border: "#ced4da",    // Input field borders
    input_focus: "#80bdff",     // Input focus borders
}
```

## Theme-Aware Components

### Available Component Types

1. **ThemedButton**: Theme-aware buttons with multiple styles
2. **ThemedMenuItem**: Menu items with theme support
3. **ThemedToolbarItem**: Toolbar buttons with theme colors
4. **ThemedInput**: Input fields with theme styling
5. **ThemedPanel**: Container panels with theme backgrounds
6. **ThemedTab**: Tab widgets with theme support

### Component Styles

```rust
use herding_cats_rust::ui::ButtonStyle;

// Button style variants
ButtonStyle::Primary,    // Main actions, brand color
ButtonStyle::Secondary,  // Secondary actions, neutral
ButtonStyle::Success,    // Positive actions, green
ButtonStyle::Danger,     // Destructive actions, red
ButtonStyle::Warning,    // Warning actions, orange
ButtonStyle::Info,       // Informational, blue
ButtonStyle::Ghost,      // Minimal, transparent
ButtonStyle::Link,       // Link-style buttons
```

## Theme Persistence

### Automatic Theme Saving

Themes are automatically saved to `theme_settings.json` when changed:

```rust
use herding_cats_rust::settings::{load_theme_settings, save_theme_settings};

// Load saved theme settings
let settings = load_theme_settings();
println!("Current theme: {}", settings.current_theme);

// Theme changes are automatically persisted
let mut theme_manager = get_theme_manager();
theme_manager.set_theme(ThemeType::Dark).unwrap();
// Settings are automatically saved
```

### Theme Settings Structure

```rust
ThemeSettings {
    current_theme: "Dark".to_string(),           // Current active theme
    theme_variant: None,                         // Custom theme variant
    use_system_theme: false,                     // Follow system theme
    theme_transition_enabled: true,              // Enable theme transitions
    high_contrast_mode: false,                   // High contrast override
    reduce_animations: false,                    // Reduce motion preference
    custom_colors: None,                         // Custom theme colors
}
```

## Best Practices

### 1. Theme Development

- **Test Across All Windows**: Ensure themes work consistently across main app and tool windows
- **Accessibility First**: Always consider accessibility requirements
- **Performance**: Avoid expensive color calculations in hot paths
- **Consistency**: Maintain consistent color usage and component styling

### 2. Component Usage

- **Use Component Factory**: Prefer `ComponentFactory` for consistent component creation
- **Theme-Aware Styling**: Always use theme colors instead of hardcoded values
- **Responsive Design**: Ensure components work at different sizes and resolutions
- **Keyboard Navigation**: Support keyboard navigation in custom components

### 3. Color Selection

- **Semantic Colors**: Use semantic color names (primary, secondary) rather than literal (blue, green)
- **Contrast Testing**: Test color combinations for adequate contrast
- **Color Blindness**: Consider common color vision deficiencies
- **Cultural Context**: Be aware of cultural color associations

### 4. Performance Optimization

- **Color Caching**: Cache computed colors when possible
- **Lazy Loading**: Load theme resources on demand
- **Minimal Recomputation**: Avoid recalculating theme colors unnecessarily
- **Efficient Updates**: Batch theme updates when possible

## API Reference

### Core Theme Types

```rust
// Theme type enum
enum ThemeType {
    Light,
    Dark,
    HighContrast,
    Custom(String),
}

// Theme colors structure
struct ThemeColors {
    // 20+ color properties for complete theme definition
}

// Complete theme definition
struct Theme {
    name: String,
    theme_type: ThemeType,
    colors: ThemeColors,
    description: String,
    author: String,
    version: String,
}
```

### Theme Manager Methods

```rust
impl ThemeManager {
    // Get available themes
    fn get_available_themes(&self) -> Vec<ThemeType>;
    fn get_available_theme_names(&self) -> Vec<String>;
    
    // Theme operations
    fn set_theme(&self, theme_type: ThemeType) -> Result<(), String>;
    fn set_theme_by_name(&self, theme_name: &str) -> Result<(), String>;
    fn get_current_theme(&self) -> Theme;
    fn get_current_theme_name(&self) -> String;
    
    // Custom theme support
    fn create_custom_theme(&mut self, name: &str, colors: ThemeColors) -> Result<(), String>;
    fn add_theme(&mut self, theme: Theme);
    fn remove_theme(&mut self, theme_type: &ThemeType) -> bool;
    
    // Color access
    fn get_colors(&self) -> ThemeColors;
    fn get_color(&self, color_name: &str) -> String;
}
```

### Component Factory Methods

```rust
impl ComponentFactory {
    // Button creation
    fn create_primary_button(text: &str) -> ThemedButton;
    fn create_secondary_button(text: &str) -> ThemedButton;
    fn create_danger_button(text: &str) -> ThemedButton;
    
    // Other components
    fn create_menu_item(text: &str) -> ThemedMenuItem;
    fn create_toolbar_item(text: &str, icon: &str) -> ThemedToolbarItem;
    fn create_status_item(text: &str) -> ThemedStatusItem;
    fn create_input() -> ThemedInput;
    fn create_panel(title: &str) -> ThemedPanel;
    fn create_tab(title: &str) -> ThemedTab;
}
```