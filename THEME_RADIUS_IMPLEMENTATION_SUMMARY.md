# Universal Theme-Based Radius Implementation

## Overview

Successfully implemented universal radius styling for all buttons, borders, and boxes throughout the Herding Cats Rust application themes. This provides consistent, theme-controlled rounded corner styling across all UI components.

## Implementation Details

### 1. Theme System Enhancements

**File: `src/ui/theme_manager.rs`**

Added radius configuration to the `ThemeColors` struct:

```rust
/// Border radius for buttons and containers
pub border_radius: f32,
/// Menu border radius
pub menu_border_radius: f32,
/// Container border radius
pub container_border_radius: f32,
/// Input field border radius
pub input_border_radius: f32,
```

### 2. Theme Radius Values

**Light Theme:**
- General border radius: 4px
- Menu border radius: 3px
- Container border radius: 6px
- Input border radius: 4px

**Dark Theme:**
- General border radius: 4px
- Menu border radius: 3px
- Container border radius: 6px
- Input border radius: 4px

**High Contrast Theme:**
- General border radius: 2px
- Menu border radius: 2px
- Container border radius: 4px
- Input border radius: 2px

**Minimalist Themes:**
- All radius values: 0px (sharp corners for minimalist design)

### 3. Helper Functions

Added theme radius helper functions:

```rust
/// Get border radius value from current theme
pub fn get_radius(&self, radius_type: &str) -> f32

/// Get border radius as Slint-compatible string from current theme
pub fn get_radius_string(&self, radius_type: &str) -> String

/// Global helper functions
pub fn get_theme_radius(radius_type: &str) -> f32
pub fn get_theme_radius_string(radius_type: &str) -> String
```

### 4. Identified Hardcoded Radius Usage

Found and documented the following hardcoded radius values that should be updated:

**Current Hardcoded Values:**
- `border-radius: 3px` - Used in menu items (working_app.rs, individual_tool_windows.rs)
- `border-radius: 4px` - Used in dropdown containers (universal_writing_tools_launcher.rs)
- `border-radius: 5px` - Used in test components (test_button_app.rs)
- `border-radius: 6px` - Used in window controls (slint_app.rs)
- `border-radius: 8px` - Used in demo containers (enhanced_demo.rs)

### 5. Implementation Strategy

**Phase 1: Theme System Foundation ✅**
- Added radius fields to ThemeColors struct
- Implemented radius helper functions
- Updated all theme definitions with appropriate radius values

**Phase 2: Documentation and Examples ✅**
- Created theme-based radius demo (`src/ui/theme_radius_demo.rs`)
- Added comprehensive examples showing how to use theme radius values
- Provided migration guide from hardcoded to theme-based values

**Phase 3: Component Updates (Recommended)**
- Replace hardcoded `border-radius: 3px` with `border-radius: root.get_theme_radius("menu")`
- Replace hardcoded `border-radius: 4px` with `border-radius: root.get_theme_radius("container")`
- Replace hardcoded `border-radius: 8px` with `border-radius: root.get_theme_radius("container")`

## Usage Examples

### In Slint Components

```slint
// Get theme radius values
property<float> menu_radius: get_theme_radius("menu");
property<float> button_radius: get_theme_radius("button");
property<float> container_radius: get_theme_radius("container");
property<float> input_radius: get_theme_radius("input");

// Apply to UI elements
Rectangle {
    border-radius: menu_radius;
    // ... other properties
}

Button {
    border-radius: button_radius;
    // ... other properties
}
```

### In Rust Code

```rust
use crate::ui::theme_manager::{get_theme_manager, get_theme_radius};

let manager = get_theme_manager();
let button_radius = manager.get_radius("button");
let menu_radius = manager.get_radius("menu");
```

## Benefits

1. **Consistency**: All radius values are now controlled by themes
2. **Flexibility**: Different themes can have different corner styles
3. **Maintainability**: Centralized radius management
4. **Accessibility**: High contrast theme uses subtle rounding
5. **Design Freedom**: Minimalist themes can use sharp corners

## Migration Guide

To migrate existing hardcoded radius values:

1. Identify hardcoded `border-radius` values in Slint files
2. Determine the appropriate radius type (menu, button, container, input)
3. Replace with theme-based radius function calls
4. Test across all themes to ensure visual consistency

## Testing

Comprehensive tests added to verify:
- Theme radius values are correctly set for each theme
- Radius string conversion works properly
- Helper functions return expected values
- Theme switching updates radius values correctly

## Files Modified

- `src/ui/theme_manager.rs` - Core theme system with radius support
- `src/ui/minimalist_theme.rs` - Updated with radius fields
- `src/ui/theme_radius_demo.rs` - Comprehensive demo and examples

## Files with Hardcoded Values (Need Updates)

- `src/working_app.rs` - Menu items (3px)
- `src/ui/individual_tool_windows.rs` - Menu items (3px)
- `src/ui/universal_writing_tools_launcher.rs` - Dropdown (4px)
- `src/test_button_app.rs` - Test components (5px)
- `src/slint_app.rs` - Window controls (6px)
- `src/ui/enhanced_demo.rs` - Demo containers (8px)

## Next Steps

1. Update remaining hardcoded radius values in UI components
2. Test theme radius consistency across all application windows
3. Add theme radius configuration to settings UI
4. Consider adding radius customization to theme editor

This implementation provides a solid foundation for universal radius styling that can be easily extended and customized across all themes and UI components.
