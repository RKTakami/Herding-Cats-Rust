# Hierarchy Tool Implementation Plan

## Overview
This document outlines the comprehensive plan to implement the missing hierarchy tool functionality that meets all specified requirements.

## Current State Analysis
- **Status**: 0% complete - missing core implementation
- **Infrastructure Available**: Database services, AppState drag framework, window management
- **Missing Components**: All hierarchy-specific tool files and logic

## Implementation Roadmap

### Phase 1: Core Hierarchy Tool Structure
**Objective**: Create the foundational hierarchy tool files and basic UI structure

#### 1.1 Create Core Files
- [ ] `src/ui/tools/mod.rs` - Module definitions and exports
- [ ] `src/ui/tools/hierarchy_base.rs` - Core hierarchy data structures and logic
- [ ] `src/ui/tools/hierarchy_ui.rs` - Dynamic UI implementation (replacing static .slint)
- [ ] `src/ui/tools/hierarchy_drag.rs` - Drag and drop handlers
- [ ] `src/ui/tools/hierarchy_database.rs` - Database operations

#### 1.2 Basic Data Structures
```rust
// Hierarchy levels as specified
enum HierarchyLevel {
    Unassigned,
    Manuscript,
    Chapter,
    Scene,
}

// Enhanced HierarchyItem with recursive relationships
struct HierarchyItem {
    id: String,
    title: String,
    level: HierarchyLevel,
    parent_id: Option<String>,
    children: Vec<String>,
    position: u32, // For drag/drop ordering
    project_id: String,
}
```

### Phase 2: Dynamic Tree View Implementation
**Objective**: Replace static hierarchy_window.slint with recursive tree structure

#### 2.1 Tree View Component
- [ ] Create dynamic tree view component in Slint
- [ ] Implement expandable/collapsible nodes
- [ ] Support nested hierarchy: Unassigned/Manuscript > Chapter > Scene
- [ ] Visual indicators for hierarchy levels
- [ ] Context menus for item operations

#### 2.2 Recursive Structure Logic
- [ ] Tree traversal algorithms (depth-first, breadth-first)
- [ ] Parent-child relationship management
- [ ] Dynamic node expansion/collapse
- [ ] Lazy loading for large hierarchies
- [ ] Search and filter within hierarchy

### Phase 3: Drag and Drop Implementation
**Objective**: Implement complete drag and drop functionality

#### 3.1 In-Tool Drag and Drop
- [ ] Connect to existing AppState drag framework
- [ ] Visual drag feedback (ghost elements, drop zones)
- [ ] Drag validation (Chapter can't be child of Scene, etc.)
- [ ] Real-time reordering during drag
- [ ] Drop zone highlighting

#### 3.2 Cross-Tool Drag Support
- [ ] Standardized drag data format
- [ ] Drag source/target coordination between tools
- [ ] Tool-specific drop handlers
- [ ] Cross-window drag operations
- [ ] Drag data serialization/deserialization

### Phase 4: Database Integration
**Objective**: Persist hierarchy changes and support drag operations

#### 4.1 Hierarchy Database Schema
```sql
CREATE TABLE hierarchy_items (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    level TEXT NOT NULL,
    parent_id TEXT,
    position INTEGER NOT NULL,
    project_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES hierarchy_items(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);
```

#### 4.2 CRUD Operations
- [ ] Create hierarchy item (with proper positioning)
- [ ] Update hierarchy item (title, position, parent)
- [ ] Delete hierarchy item (with cascade handling)
- [ ] Reorder items (drag-induced position changes)
- [ ] Bulk operations for performance

#### 4.3 Transaction Management
- [ ] Atomic drag operations (rollback on failure)
- [ ] Concurrency control for multi-user scenarios
- [ ] Change tracking and audit log
- [ ] Conflict resolution for simultaneous edits

### Phase 5: Cross-Tool Integration
**Objective**: Enable hierarchy items to be dragged into other tools

#### 5.1 Tool Communication Framework
- [ ] Event system for hierarchy changes
- [ ] Cross-tool data format standardization
- [ ] Tool registry for drag targets
- [ ] Real-time updates across tools

#### 5.2 Integration Points
- [ ] Hierarchy → Codex (link chapters to world elements)
- [ ] Hierarchy → Plot (connect scenes to plot points)
- [ ] Hierarchy → Notes (attach notes to hierarchy items)
- [ ] Hierarchy → Research (link research to chapters/scenes)

### Phase 6: UI/UX Polish
**Objective**: Complete the user experience

#### 6.1 Visual Design
- [ ] Consistent styling with other tools
- [ ] Responsive layout for different screen sizes
- [ ] Accessibility features (keyboard navigation, screen readers)
- [ ] Dark/light theme support

#### 6.2 Performance Optimization
- [ ] Virtual scrolling for large hierarchies
- [ ] Lazy loading of child nodes
- [ ] Debounced search and filtering
- [ ] Efficient drag operations

#### 6.3 Error Handling
- [ ] Graceful handling of database errors
- [ ] Validation feedback for invalid operations
- [ ] Recovery from drag/drop failures
- [ ] Network error handling for cloud sync

## Technical Implementation Details

### Database Integration Strategy
```rust
// Hierarchy service using existing database infrastructure
pub struct HierarchyService {
    db_service: Arc<RwLock<EnhancedDatabaseService>>,
}

impl HierarchyService {
    pub async fn create_item(&self, item: HierarchyItem) -> DatabaseResult<String> { /* */ }
    pub async fn update_item(&self, item: HierarchyItem) -> DatabaseResult<()> { /* */ }
    pub async fn move_item(&self, item_id: &str, new_parent: Option<&str>, position: u32) -> DatabaseResult<()> { /* */ }
    pub async fn get_hierarchy(&self, project_id: &str) -> DatabaseResult<Vec<HierarchyItem>> { /* */ }
}
```

### Drag and Drop Integration
```rust
// Integration with existing AppState
impl HierarchyDragHandler {
    pub fn handle_drag_start(&self, item_id: &str, app_state: &mut AppState) {
        app_state.set_drag_state(true, Some("hierarchy".to_string()), Some(item_id.to_string()));
    }
    
    pub fn handle_drop(&self, target_id: &str, drag_data: &str, app_state: &mut AppState) -> Result<(), String> {
        // Update database with new hierarchy structure
        // Notify other tools of changes
        // Update UI state
    }
}
```

### Cross-Tool Communication
```rust
// Event system for hierarchy changes
#[derive(Debug, Clone)]
pub enum HierarchyEvent {
    ItemCreated { item: HierarchyItem },
    ItemMoved { item_id: String, old_parent: Option<String>, new_parent: Option<String>, position: u32 },
    ItemDeleted { item_id: String },
    ItemUpdated { item: HierarchyItem },
}

// Tool notification system
pub trait HierarchyEventListener {
    fn on_hierarchy_event(&self, event: HierarchyEvent);
}
```

## Dependencies and Integration Points

### Required Existing Components
- [ ] `AppState` from `src/ui_state.rs` - Drag state management
- [ ] `ServiceFactory` from `src/database/service_factory.rs` - Database access
- [ ] `EnhancedWindowManager` from `src/ui/enhanced_window_manager.rs` - Window management
- [ ] `ToolWindowType` from `src/ui/mod.rs` - Tool type definitions

### New Components to Create
- [ ] `HierarchyService` - Database operations for hierarchy
- [ ] `HierarchyTreeComponent` - Dynamic tree view UI
- [ ] `HierarchyDragManager` - Drag and drop coordination
- [ ] `HierarchyEventBus` - Cross-tool communication

## Testing Strategy

### Unit Tests
- [ ] Hierarchy item creation and validation
- [ ] Tree structure operations (add, remove, move)
- [ ] Database CRUD operations
- [ ] Drag and drop state management

### Integration Tests
- [ ] Cross-tool drag operations
- [ ] Database transaction handling
- [ ] UI state synchronization
- [ ] Error recovery scenarios

### UI Tests
- [ ] Tree view expansion/collapse
- [ ] Drag and drop visual feedback
- [ ] Cross-tool integration workflows
- [ ] Performance with large hierarchies

## Timeline and Milestones

### Week 1: Foundation
- [ ] Create core hierarchy tool files
- [ ] Implement basic data structures
- [ ] Set up database schema

### Week 2: Tree View
- [ ] Create dynamic tree view component
- [ ] Implement recursive structure logic
- [ ] Basic hierarchy display

### Week 3: Drag and Drop
- [ ] Implement in-tool drag operations
- [ ] Add visual drag feedback
- [ ] Database integration for moves

### Week 4: Cross-Tool Integration
- [ ] Implement cross-tool drag support
- [ ] Create event system for notifications
- [ ] Integrate with other writing tools

### Week 5: Polish and Testing
- [ ] UI/UX improvements
- [ ] Performance optimization
- [ ] Comprehensive testing
- [ ] Documentation and error handling

## Success Criteria

### Functional Requirements
- [ ] ✅ Unassigned and Manuscript share top level
- [ ] ✅ Chapter can be child of Unassigned or Manuscript
- [ ] ✅ Scenes can be child of Chapter
- [ ] ✅ Drag and drop within hierarchy tool
- [ ] ✅ Drag and drop to other tool windows
- [ ] ✅ Database persistence of all changes
- [ ] ✅ Real-time updates across tools

### Technical Requirements
- [ ] ✅ 95% code coverage for hierarchy components
- [ ] ✅ Performance with 1000+ hierarchy items
- [ ] ✅ Cross-tool drag operations under 100ms
- [ ] ✅ Database transaction success rate >99.9%
- [ ] ✅ Accessibility compliance (WCAG 2.1 AA)

This comprehensive plan addresses all identified issues and provides a clear roadmap for implementing a fully functional hierarchy tool that meets the specified requirements.