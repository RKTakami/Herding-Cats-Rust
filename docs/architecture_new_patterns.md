# New Architecture Patterns Documentation

## Overview

This document describes the new unified architecture patterns implemented for the Herding Cats Rust UI layer. These patterns provide consistent database integration, safe threading models, and standardized API contracts across all tools.

## Core Components

### 1. ToolDatabaseContext

The `ToolDatabaseContext` provides unified database access for all UI tools, replacing direct database service usage with a consistent, retry-aware interface.

#### Key Features:
- **Unified Access**: Single interface for all database operations
- **Retry Logic**: Built-in exponential backoff for transient failures
- **Performance Monitoring**: Automatic timing and metrics collection
- **Error Categorization**: Structured error handling with specific error types

#### Usage Example:
```rust
// Before (Legacy Pattern)
if let Some(db_service) = &self.database_service {
    let entries = db_service.read().await.get_entries_by_project(project_id).await?;
}

// After (New Pattern)
let mut database_context = ToolDatabaseContext::new("codex_tool", database_state).await;
let entries = database_context.execute_with_retry(
    "load_entries",
    |service| Box::pin(async move { 
        service.get_entries_by_project(project_id).await
    }),
    3,
).await?;
```

### 2. ThreadSafeToolRegistry

The `ThreadSafeToolRegistry` provides safe global state management with proper async/await support and deadlock prevention.

#### Key Features:
- **Thread Safety**: Uses `Arc<RwLock<T>>` for safe concurrent access
- **Lifecycle Management**: Tracks tool registration, health, and errors
- **Event Broadcasting**: Notifies subscribers of tool lifecycle events
- **Health Monitoring**: Automatic health checks and error tracking

#### Usage Example:
```rust
// Before (Legacy Pattern)
lazy_static! {
    static ref GLOBAL_TOOL_MANAGER: std::sync::Mutex<ToolManager> = 
        std::sync::Mutex::new(ToolManager::new());
}

// After (New Pattern)
lazy_static! {
    static ref GLOBAL_TOOL_REGISTRY: ThreadSafeToolRegistry = ThreadSafeToolRegistry::new();
}

// Access the registry
let registry = get_tool_registry();
registry.register_tool("my_tool".to_string(), Arc::new(my_tool)).await?;
```

### 3. ToolApiContract

The `ToolApiContract` establishes consistent service interfaces and cross-tool communication through a standardized event bus.

#### Key Features:
- **Standardized Interfaces**: Consistent tool lifecycle and communication patterns
- **Event Bus**: Cross-tool communication through typed events
- **Service Discovery**: Registry for available services and tools
- **Configuration Management**: Centralized tool configuration and validation

#### Usage Example:
```rust
// Before (No Standardization)
// Each tool had different interfaces and communication patterns

// After (Standardized)
#[async_trait]
trait ToolIntegration {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String>;
    fn update(&mut self) -> Result<(), String>;
    fn render(&mut self) -> Result<(), String>;
    async fn cleanup(&mut self) -> Result<(), String>;
}

// Event broadcasting
let contract = get_api_contract();
contract.broadcast_lifecycle(ToolLifecycleEvent::Initialized {
    tool_id: "my_tool".to_string(),
    timestamp: Instant::now(),
    version: "2.0.0".to_string(),
}).await?;
```

## Migration Patterns

### Database Access Migration

#### Step 1: Replace Direct Database Service Usage
```rust
// Remove this pattern:
pub struct OldTool {
    database_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
}

// Replace with:
pub struct NewTool {
    database_context: Option<ToolDatabaseContext>,
}
```

#### Step 2: Update Database Operations
```rust
// Before:
async fn load_data(&self, project_id: &str) -> Result<Vec<Data>, String> {
    if let Some(db_service) = &self.database_service {
        let guard = db_service.read().await;
        guard.get_data_by_project(project_id).await
            .map_err(|e| format!("Database error: {}", e))
    } else {
        Err("Database service not available".to_string())
    }
}

// After:
async fn load_data(&mut self, project_id: &str) -> DatabaseOperationResult<Vec<Data>> {
    if let Some(context) = &mut self.database_context {
        context.execute_with_retry(
            "load_data",
            |service| Box::pin(async move {
                service.get_data_by_project(project_id).await
            }),
            3,
        ).await
    } else {
        DatabaseOperationResult::not_found("Database context", "unavailable")
    }
}
```

### Threading Model Migration

#### Step 1: Replace Global Static Managers
```rust
// Before:
lazy_static! {
    static ref GLOBAL_STATE: Mutex<MyState> = Mutex::new(MyState::new());
}

fn get_global_state() -> MutexGuard<'static, MyState> {
    GLOBAL_STATE.lock().unwrap()
}

// After:
lazy_static! {
    static ref GLOBAL_STATE_CONTAINER: GlobalStateContainer<MyState> = 
        GlobalStateContainer::new();
}

async fn get_global_state() -> RwLockReadGuard<'static, MyState> {
    GLOBAL_STATE_CONTAINER.read().await
}
```

#### Step 2: Implement Proper Async Patterns
```rust
// Before:
fn update_state(&self, data: MyData) -> Result<(), String> {
    let mut state = get_global_state();
    state.update(data);
    Ok(())
}

// After:
async fn update_state(&self, data: MyData) -> Result<(), String> {
    let mut state = get_global_state().await;
    state.update("my_tool", |s| {
        s.update(data);
        Ok(())
    }).await
}
```

### API Contract Migration

#### Step 1: Implement Standardized Tool Interface
```rust
// Before:
impl MyTool {
    pub fn new() -> Self { /* ... */ }
    pub fn load(&mut self, data: Data) -> Result<(), String> { /* ... */ }
    pub fn save(&self) -> Result<(), String> { /* ... */ }
}

// After:
#[async_trait]
impl ToolIntegration for MyTool {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        // Initialize with database context
        Ok(())
    }
    
    fn update(&mut self) -> Result<(), String> {
        // Update tool state
        Ok(())
    }
    
    fn render(&mut self) -> Result<(), String> {
        // Render tool UI
        Ok(())
    }
    
    async fn cleanup(&mut self) -> Result<(), String> {
        // Cleanup resources
        Ok(())
    }
}
```

#### Step 2: Integrate with Event System
```rust
// Register tool with API contract
let contract = get_api_contract();
contract.register_tool("my_tool".to_string()).await?;

// Broadcast lifecycle events
contract.broadcast_lifecycle(ToolLifecycleEvent::Initialized {
    tool_id: "my_tool".to_string(),
    timestamp: Instant::now(),
    version: env!("CARGO_PKG_VERSION").to_string(),
}).await?;
```

## Error Handling Patterns

### Structured Error Types
```rust
#[derive(Debug, Clone)]
pub enum DatabaseError {
    ConnectionFailed(String),
    NotFound(String),
    ValidationError(String),
    ConstraintViolation(String),
    Timeout(String),
}

#[derive(Debug, Clone)]
pub struct DatabaseOperationResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<DatabaseError>,
    pub timing: Option<Duration>,
    pub retry_count: u32,
}
```

### Error Handling Examples
```rust
// Before:
match result {
    Ok(data) => data,
    Err(e) => return Err(format!("Database error: {}", e)),
}

// After:
match result {
    Ok(data) => DatabaseOperationResult::success(data, timing),
    Err(DatabaseError::NotFound(_)) => DatabaseOperationResult::not_found("Entity", id),
    Err(e) => DatabaseOperationResult::validation_error("field", e.to_string()),
}
```

## Performance Considerations

### Connection Pooling
- Database contexts use connection pooling to minimize connection overhead
- Configure pool size based on expected concurrent usage
- Monitor connection usage and adjust pool settings accordingly

### Caching Strategy
- Implement read-through caching for frequently accessed data
- Use cache invalidation strategies to maintain data consistency
- Consider cache warming for critical startup operations

### Memory Management
- Use `Arc<RwLock<T>>` for shared data instead of cloning
- Implement proper cleanup in tool lifecycle methods
- Monitor memory usage patterns and optimize allocations

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_database_context_operations() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut context = ToolDatabaseContext::new("test_tool", database_state).await;
        
        // Test basic operations
        let result = context.execute_with_retry(
            "test_operation",
            |service| Box::pin(async move { Ok::<String, String>("success".to_string()) }),
            3,
        ).await;
        
        assert!(result.is_success());
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_integration() {
        let mut tool_manager = ToolManager::new(
            Arc::new(RwLock::new(DatabaseAppState::new()))
        ).await;
        
        // Test tool registration and lifecycle
        let test_tool = TestTool::new();
        tool_manager.register_tool(test_tool).await.unwrap();
        
        // Verify tool is accessible
        assert!(tool_manager.get_tool("test_tool").await.is_some());
    }
}
```

## Best Practices

### 1. Always Use Async/Await
- Use async patterns for all database and I/O operations
- Avoid blocking calls in async contexts
- Use proper timeout configurations

### 2. Implement Proper Error Handling
- Use structured error types instead of strings
- Provide meaningful error messages
- Implement appropriate retry logic

### 3. Follow Resource Management
- Implement proper cleanup in tool lifecycle methods
- Use RAII patterns for resource management
- Monitor resource usage and detect leaks

### 4. Maintain Backward Compatibility
- Provide migration paths for existing code
- Use feature flags for gradual rollout
- Maintain compatibility during transition periods

## Troubleshooting

### Common Issues

1. **Deadlocks in Threading Code**
   - Use `Arc<RwLock<T>>` instead of `Mutex<T>`
   - Avoid holding multiple locks simultaneously
   - Use proper lock ordering if multiple locks are needed

2. **Database Connection Leaks**
   - Use connection pooling provided by `ToolDatabaseContext`
   - Implement proper timeout handling
   - Monitor connection usage patterns

3. **Memory Leaks in Global State**
   - Implement proper cleanup in tool lifecycle methods
   - Use weak references when appropriate
   - Monitor memory usage with debugging tools

4. **Race Conditions in Tool Registration**
   - Use `ThreadSafeToolRegistry` for tool management
   - Implement proper synchronization for shared state
   - Use atomic operations for simple state updates

### Debugging Tools

The migration includes comprehensive debugging and monitoring tools:

- **MigrationDebugger**: Comprehensive debugging for migration issues
- **MigrationTestingPipeline**: Automated testing for migration validation
- **PerformanceMonitoring**: Real-time performance metrics collection
- **ThreadSafetyMonitor**: Detection of threading issues and deadlocks

### Getting Help

For additional support with the new architecture patterns:

1. Review the integration tests in `src/ui/tools/integration_tests.rs`
2. Use the migration helpers in `src/ui/tools/migration_helpers.rs`
3. Consult the debugging tools in `src/ui/tools/debugging_tools.rs`
4. Check the testing pipeline in `src/ui/tools/testing_pipeline.rs`

## Migration Checklist

- [ ] Replace all direct database service usage with `ToolDatabaseContext`
- [ ] Migrate global static managers to `ThreadSafeToolRegistry`
- [ ] Implement `ToolIntegration` trait for all tools
- [ ] Add proper error handling with structured error types
- [ ] Update threading patterns to use async/await
- [ ] Integrate tools with the event bus system
- [ ] Add comprehensive tests for new patterns
- [ ] Update documentation and examples
- [ ] Validate performance meets requirements
- [ ] Complete migration testing and validation