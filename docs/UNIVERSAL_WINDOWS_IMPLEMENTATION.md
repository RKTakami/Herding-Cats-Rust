# Universal Writing Tools - Universal Windows Implementation

## Overview

This document describes the implementation of the Universal Writing Tools approach, which provides a single window interface with dropdown menus in the title bar for accessing all writing tools. This approach offers a more streamlined and organized alternative to individual floating windows.

## Architecture

### Core Components

1. **UniversalWritingTools** (`src/ui/universal_writing_tools.rs`)
   - Main universal window implementation
   - Tool state management and switching logic
   - Configuration and preferences handling

2. **UniversalWritingToolsWindow** (`src/ui/universal_tools_ui.slint`)
   - Slint UI component for the universal window
   - Title bar with dropdown menu
   - Tool-specific toolbars and content areas
   - Status bar integration

3. **UniversalWritingToolsApp** (`src/universal_writing_tools_app.rs`)
    - Example application demonstrating the universal approach
    - UI callback management
    - Integration with existing tool infrastructure

4. **Universal Window State Management**
    - **Tool State Preservation**: Automatic saving and restoration of tool states
    - **Window Layout Management**: Persistent window positions and sizes
    - **Configuration Persistence**: User preferences and settings storage
    - **Cross-Session State**: State restoration across application restarts

## Key Features

### 1. Title Bar Dropdown Menu

The title bar contains a dropdown menu that allows users to quickly switch between tools:

```rust
// Tool selection in title bar
Button {
    text: current-tool + " ▼";
    clicked => { root.toggle_tools_dropdown(); }
}
```

**Available Tools:**
- 📚 **Hierarchy** - Manuscript structure organization (Ctrl+H)
- 📖 **Codex** - World building and reference materials (Ctrl+C)
- 💭 **Brainstorming** - Mindmap visualization (Ctrl+B)
- 🔬 **Analysis** - Writing structure analysis (Ctrl+A)
- 📊 **Plot** - Plot structure development (Ctrl+P)
- 🔎 **Research** - Research organization (Ctrl+R)
- 📝 **Notes** - Note taking and organization (Ctrl+N)
- 🏗️ **Structure** - Document structure management (Ctrl+T)

### 2. Tool-Specific Toolbars

Each tool has its own toolbar that appears when the tool is active:

**Hierarchy Tool Toolbar:**
- ➕ New Item - Create new hierarchy item
- 🗑️ Delete - Delete selected item
- ⬆️ Up - Move item up
- ⬇️ Down - Move item down

**Codex Tool Toolbar:**
- 📝 New Entry - Create new codex entry
- 🔍 Search - Search codex entries
- 📤 Export - Export codex data
- 📥 Import - Import codex data

**Brainstorming Tool Toolbar:**
- 💭 New Node - Create new brainstorming node
- 📐 Layout - Apply layout algorithm
- 🔍+ Zoom In - Zoom in on canvas
- 🔍- Zoom Out - Zoom out on canvas

**Analysis Tool Toolbar:**
- 📊 New Analysis - Create new analysis
- 💡 Insights - Generate insights
- 📋 Export - Export analysis summary
- 📋+ Import - Import analysis data

### 3. State Preservation

The universal window preserves the state of each tool when switching:

```rust
pub struct UniversalToolState {
    pub tool_type: ToolType,
    pub is_visible: bool,
    pub window_state: ToolWindowState,
    pub last_active: Option<std::time::SystemTime>,
}
```

### 4. Status Bar Integration

The status bar provides real-time information:
- Current active tool
- Database connection status
- Word count
- Session interaction statistics

### 5. Keyboard Shortcuts

Quick tool switching via keyboard shortcuts:
- **Ctrl+H** - Switch to Hierarchy tool
- **Ctrl+C** - Switch to Codex tool
- **Ctrl+B** - Switch to Brainstorming tool
- **Ctrl+A** - Switch to Analysis tool
- **Ctrl+P** - Switch to Plot tool
- **Ctrl+R** - Switch to Research tool
- **Ctrl+N** - Switch to Notes tool
- **Ctrl+T** - Switch to Structure tool

## Implementation Details

### Tool Switching Mechanism

```rust
pub fn switch_tool(&mut self, tool_type: ToolType) -> Result<()> {
    if tool_type == self.current_tool {
        return Ok(());
    }
    
    // Update tool states
    {
        let mut states = self.tool_states.lock().unwrap();
        
        // Hide current tool
        if let Some(current_state) = states.get_mut(&self.current_tool) {
            current_state.is_visible = false;
        }
        
        // Show new tool
        if let Some(new_state) = states.get_mut(&tool_type) {
            new_state.is_visible = true;
            new_state.last_active = Some(std::time::SystemTime::now());
        }
    }
    
    self.current_tool = tool_type;
    
    // Notify tool manager of tool switch
    {
        let mut manager = self.tool_manager.lock().unwrap();
        manager.broadcast_event(ToolEvent::ToolFocused(tool_type))?;
    }
    
    Ok(())
}
```

### Cross-Tool Integration

The universal window maintains compatibility with the existing tool infrastructure:

```rust
// Tool manager integration
pub tool_manager: Arc<Mutex<ToolManager>>,

// Event broadcasting
manager.broadcast_event(ToolEvent::ToolFocused(tool_type))?;

// State management
pub tool_states: Arc<Mutex<HashMap<ToolType, UniversalToolState>>>,
```

### Configuration Options

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalToolsConfig {
    pub window_title: String,
    pub default_tool: ToolType,
    pub show_tool_tooltips: bool,
    pub enable_keyboard_shortcuts: bool,
    pub remember_tool_states: bool,
}
```

## Benefits of Universal Window Approach

### 1. Space Efficiency
- Single window instead of multiple floating windows
- Better screen real estate utilization
- Reduced desktop clutter
- **Multi-Monitor Support**: Optimized for multi-monitor setups with consistent experience

### 2. Performance Benefits
- **Reduced Memory Usage**: Single window reduces resource consumption
- **Faster Tool Switching**: Instant transitions between tools
- **Optimized Rendering**: Shared UI components reduce GPU overhead
- **Efficient State Management**: Centralized state reduces complexity

### 2. Improved User Experience
- Consistent interface across all tools
- Easy tool switching via dropdown or keyboard shortcuts
- Unified status and feedback

### 3. Better Organization
- Logical grouping of related writing tools
- Clear visual hierarchy
- Professional appearance

### 4. Enhanced Productivity
- Quick access to all tools from title bar
- Context preservation between tool switches
- Reduced window management overhead

### 5. Maintainability
- Centralized UI management
- Shared tool infrastructure
- Easier updates and modifications

## Advanced Features

#### State Management
The universal window framework includes sophisticated state management:

```rust
pub struct UniversalToolState {
    pub tool_type: ToolType,
    pub is_visible: bool,
    pub window_state: ToolWindowState,
    pub last_active: Option<std::time::SystemTime>,
    pub tool_specific_state: HashMap<String, serde_json::Value>,
}
```

#### Event System
Comprehensive event handling for cross-tool communication:

```rust
pub enum UniversalWindowEvent {
    ToolSwitched { from: ToolType, to: ToolType },
    StateChanged { tool: ToolType, state: ToolState },
    ConfigurationUpdated { config: UniversalToolsConfig },
    WindowAction { action: WindowAction },
}
```

#### Performance Monitoring
Built-in performance tracking for tool switching and state management:

```rust
pub struct PerformanceMetrics {
    pub tool_switch_time_ms: f64,
    pub state_save_time_ms: f64,
    pub state_restore_time_ms: f64,
    pub memory_usage_kb: u64,
}
```

## Usage Examples

### Basic Usage

```rust
use herding_cats_rust::universal_writing_tools_app::run_universal_writing_tools;

// Run the universal writing tools application
let db_service = /* your database service */;
run_universal_writing_tools(db_service)?;
```

### Custom Configuration

```rust
let config = UniversalToolsConfig {
    window_title: "My Writing Studio".to_string(),
    default_tool: ToolType::Codex,
    show_tool_tooltips: true,
    enable_keyboard_shortcuts: true,
    remember_tool_states: true,
};

let mut app = UniversalWritingToolsApp::new(db_service, config)?;
app.initialize()?;
app.run()?;
```

### Tool Switching

```rust
// Programmatically switch tools
app.universal_tools.switch_tool(ToolType::Brainstorming)?;

// Check current tool
let current = app.universal_tools.current_tool;
println!("Current tool: {:?}", current);

// Get tool display information
let display_name = app.universal_tools.current_tool_display_name();
let shortcut = app.universal_tools.current_tool_shortcut();
```

## Integration with Existing Tools

The universal window approach integrates seamlessly with existing individual tool windows:

### Backward Compatibility
- Existing tool implementations remain unchanged
- Individual tool windows can still be used
- Tool-specific functionality preserved

### Migration Path
1. **Phase 1**: Implement universal window framework
2. **Phase 2**: Integrate existing tools into universal framework
3. **Phase 3**: Add cross-tool drag-and-drop functionality
4. **Phase 4**: User preference system for window mode selection

### Tool Integration Points
- **Tool Registry**: Register tools with universal manager
- **State Management**: Preserve tool state during switches
- **Event System**: Broadcast tool switching events
- **UI Updates**: Update toolbars and content areas

## Future Enhancements

### 1. Advanced Drag-and-Drop
- Cross-tool content transfer
- Visual drag feedback
- Drop zone indicators

### 2. Customizable Layouts
- Resizable tool panes
- Floating tool windows option
- Tabbed interface mode

### 3. Enhanced Toolbars
- Context-sensitive toolbar items
- Tool-specific keyboard shortcuts
- Customizable toolbar layout

### 4. Workspace Management
- Save/restore workspace layouts
- Multiple workspace support
- Tool-specific workspace preferences

### 5. Advanced Configuration
- Per-tool configuration options
- Theme and appearance customization
- Advanced keyboard shortcut mapping
- **State Persistence**: Automatic save/restore of tool states across sessions
- **Performance Optimization**: Built-in metrics and optimization suggestions
- **Accessibility Features**: Enhanced keyboard navigation and screen reader support

## Testing

The implementation includes comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_universal_app_creation() { /* ... */ }
    
    #[test]
    fn test_tool_switching() { /* ... */ }
    
    #[test]
    fn test_keyboard_shortcuts() { /* ... */ }
    
    #[test]
    fn test_status_info() { /* ... */ }
}
```

## Conclusion

The Universal Writing Tools implementation provides a modern, efficient, and user-friendly approach to accessing multiple writing tools through a single, organized interface. The title bar dropdown menu, tool-specific toolbars, and state preservation features create a seamless workflow that enhances productivity while maintaining the full functionality of individual tools.

This approach represents a significant improvement over traditional multi-window interfaces and provides a foundation for future enhancements in writing tool integration and user experience.