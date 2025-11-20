# Troubleshooting Guide: UI Layer Migration

## Overview

This guide helps developers identify and resolve common issues encountered during the migration from legacy UI patterns to the new unified architecture. It covers debugging techniques, common error patterns, and step-by-step solutions.

## Quick Reference

### Common Error Categories

| Error Type | Common Causes | Quick Fix |
|------------|---------------|-----------|
| **Compilation Errors** | Missing imports, type mismatches | Check imports and use migration helpers |
| **Runtime Errors** | Uninitialized components, database issues | Verify initialization order and database connectivity |
| **Threading Issues** | Deadlocks, race conditions | Use `Arc<RwLock<T>>` instead of `Mutex<T>` |
| **Performance Issues** | Slow operations, memory leaks | Enable monitoring and optimize bottlenecks |
| **Integration Issues** | Tool communication failures | Check event bus and API contract setup |

## Detailed Troubleshooting

### 1. Compilation Errors

#### Missing Import Errors
```
error[E0432]: unresolved import `crate::DatabaseResult`
```

**Solution:**
```rust
// Check what's available in the new architecture
use crate::ui::tools::database_integration::{ToolDatabaseContext, DatabaseOperationResult};
use crate::ui::tools::threading_patterns::{ThreadSafeToolRegistry, get_tool_registry};
use crate::ui::tools::api_contracts::ToolApiContract;
```

**Debug Steps:**
1. Check `src/ui/tools/mod.rs` for available exports
2. Use the migration helpers to identify correct imports
3. Verify the new architecture components are properly integrated

#### Type Mismatch Errors
```
error[E0308]: mismatched types
expected struct `DatabaseResult`
found struct `String`
```

**Solution:**
```rust
// Before (Legacy):
return Err("Database connection failed".to_string());

// After (New Architecture):
return DatabaseOperationResult::validation_error("connection", "Database connection failed");
```

#### Trait Implementation Errors
```
error[E0277]: the trait bound `MyTool: ToolIntegration` is not satisfied
```

**Solution:**
```rust
#[async_trait]
impl ToolIntegration for MyTool {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        // Implement initialization logic
        Ok(())
    }
    
    fn update(&mut self) -> Result<(), String> {
        // Implement update logic
        Ok(())
    }
    
    fn render(&mut self) -> Result<(), String> {
        // Implement rendering logic
        Ok(())
    }
    
    async fn cleanup(&mut self) -> Result<(), String> {
        // Implement cleanup logic
        Ok(())
    }
}
```

### 2. Runtime Errors

#### Database Context Not Available
```
Error: Database context not initialized
```

**Diagnosis:**
- Tool was not properly initialized
- Database context is None when trying to use it

**Solution:**
```rust
// Ensure proper initialization in tool constructor
pub async fn new(database_state: Arc<RwLock<DatabaseAppState>>) -> Self {
    let mut tool = MyTool {
        database_context: None,
        // ... other fields
    };
    
    // Initialize the tool
    let mut database_context = ToolDatabaseContext::new("my_tool", database_state).await;
    tool.initialize(&mut database_context).await.unwrap();
    
    tool
}

// Check context availability before use
async fn safe_database_operation(&mut self) -> DatabaseOperationResult<()> {
    if let Some(context) = &mut self.database_context {
        context.execute_with_retry(
            "operation_name",
            |service| Box::pin(async move { service.some_operation().await }),
            3,
        ).await
    } else {
        DatabaseOperationResult::not_found("Database context", "unavailable")
    }
}
```

#### Tool Registration Failures
```
Error: Tool 'my_tool' already registered
```

**Diagnosis:**
- Attempting to register the same tool multiple times
- Tool ID conflicts

**Solution:**
```rust
// Check if tool is already registered
async fn safe_tool_registration(tool_id: &str, tool: Arc<dyn Send + Sync>) -> Result<(), String> {
    let registry = get_tool_registry();
    
    // Check if tool already exists
    if registry.get_tool(tool_id).await.is_some() {
        return Err(format!("Tool '{}' is already registered", tool_id));
    }
    
    // Register the tool
    registry.register_tool(tool_id.to_string(), tool).await
}

// Use unique tool IDs
let unique_tool_id = format!("{}_{}", tool_type, uuid::Uuid::new_v4());
```

### 3. Threading Issues

#### Deadlock Detection
```
thread 'main' panicked at 'deadlock detected'
```

**Diagnosis:**
- Multiple locks held simultaneously
- Circular dependencies in lock acquisition

**Solution:**
```rust
// Before (Problematic):
let mut lock1 = GLOBAL_STATE1.lock().unwrap();
let mut lock2 = GLOBAL_STATE2.lock().unwrap(); // Potential deadlock

// After (Safe):
let mut lock2 = GLOBAL_STATE2.write().await;
let mut lock1 = GLOBAL_STATE1.write().await; // Use async/await

// Alternative approach - acquire locks in consistent order
let (lock1, lock2) = tokio::join!(
    GLOBAL_STATE1.write(),
    GLOBAL_STATE2.write()
);
```

#### Race Condition Detection
```
Error: Data corruption detected
```

**Diagnosis:**
- Concurrent access to shared data without proper synchronization
- Inconsistent state updates

**Solution:**
```rust
// Use proper synchronization
async fn update_shared_data(data: MyData) -> Result<(), String> {
    let mut state = GLOBAL_STATE_CONTAINER.write().await;
    state.update("my_tool", |s| {
        s.apply_data(data);
        Ok(())
    }).await
}

// Use atomic operations for simple state
use std::sync::atomic::{AtomicBool, Ordering};

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

async fn initialize_once() -> Result<(), String> {
    if !IS_INITIALIZED.load(Ordering::SeqCst) {
        // Perform initialization
        IS_INITIALIZED.store(true, Ordering::SeqCst);
    }
    Ok(())
}
```

### 4. Performance Issues

#### Slow Database Operations
```
Warning: Database operation took 5000ms (threshold: 1000ms)
```

**Diagnosis:**
- Inefficient queries
- Lack of connection pooling
- Missing indexes

**Solution:**
```rust
// Enable connection pooling
let mut context = ToolDatabaseContext::new("my_tool", database_state).await;

// Optimize queries
let result = context.execute_with_retry(
    "optimized_query",
    |service| Box::pin(async move {
        // Use indexed queries and limit result sets
        service.get_limited_data(limit, offset).await
    }),
    3,
).await?;

// Add query timeouts
let timeout_config = OperationTimeout::new(5, 3); // 5 second timeout, 3 retries
```

#### Memory Leaks
```
Warning: High memory growth rate detected: 15.0%
```

**Diagnosis:**
- Unreleased database connections
- Circular references
- Large data structures not being cleaned up

**Solution:**
```rust
// Implement proper cleanup in tool lifecycle
async fn cleanup(&mut self) -> Result<(), String> {
    // Clear cached data
    self.cache.clear();
    
    // Close database connections
    self.database_context = None;
    
    // Clear event subscriptions
    if let Some(subscription) = self.event_subscription.take() {
        // Unsubscribe from events
    }
    
    Ok(())
}

// Use weak references to break cycles
use std::sync::Weak;

pub struct MyTool {
    parent_ref: Weak<Mutex<ParentType>>,
    // ... other fields
}
```

### 5. Integration Issues

#### Event Bus Communication Failures
```
Error: Event delivery failed
```

**Diagnosis:**
- Event bus not properly initialized
- Tool not subscribed to events
- Event type mismatches

**Solution:**
```rust
// Ensure event bus is initialized
async fn setup_event_handling(&mut self) -> Result<(), String> {
    let contract = get_api_contract();
    
    // Subscribe to events
    self.event_receiver = Some(contract.subscribe_to_events("my_tool"));
    
    // Start event handler task
    let receiver = self.event_receiver.as_mut().unwrap();
    tokio::spawn(async move {
        while let Some(event) = receiver.recv().await {
            handle_event(event);
        }
    });
    
    Ok(())
}

// Ensure proper event broadcasting
async fn broadcast_status_update(&self, status: &str) -> Result<(), String> {
    let contract = get_api_contract();
    contract.broadcast_event(ToolEvent::StatusChanged {
        tool_id: "my_tool".to_string(),
        status: status.to_string(),
        timestamp: Instant::now(),
    }).await
}
```

#### API Contract Violations
```
Error: API contract validation failed
```

**Diagnosis:**
- Tool not implementing required interface methods
- Configuration validation failures
- Service registration issues

**Solution:**
```rust
// Implement all required methods
#[async_trait]
impl ToolIntegration for MyTool {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        // Required: Initialize tool with database context
        self.database_context = Some(database_context.clone());
        Ok(())
    }
    
    fn update(&mut self) -> Result<(), String> {
        // Required: Update tool state
        Ok(())
    }
    
    fn render(&mut self) -> Result<(), String> {
        // Required: Render tool UI
        Ok(())
    }
    
    async fn cleanup(&mut self) -> Result<(), String> {
        // Required: Cleanup resources
        Ok(())
    }
}

// Validate configuration
async fn validate_configuration(&self) -> Result<(), String> {
    let config = self.get_configuration();
    
    if config.database_timeout < Duration::from_secs(1) {
        return Err("Database timeout too low".to_string());
    }
    
    if config.max_connections == 0 {
        return Err("Max connections cannot be zero".to_string());
    }
    
    Ok(())
}
```

## Debugging Tools and Techniques

### 1. Using Migration Debugger

```rust
async fn debug_migration_issues() -> Result<(), String> {
    let debugger = MigrationDebugger::new();
    let session_id = debugger.start_debug_session("problematic_tool").await?;
    
    // Run comprehensive diagnostics
    let trace_result = debugger.trace_database_operations(
        &session_id, 
        &mut database_context, 
        100
    ).await?;
    
    let thread_report = debugger.monitor_thread_safety(
        &session_id, 
        "problematic_tool"
    ).await?;
    
    let memory_metrics = debugger.collect_memory_metrics(&session_id).await?;
    
    // Complete session and get report
    let report = debugger.complete_debug_session(&session_id).await?;
    
    // Print debugging information
    println!("Debug Report: {:#?}", report);
    
    Ok(())
}
```

### 2. Performance Monitoring

```rust
async fn monitor_performance() {
    let start_time = Instant::now();
    
    // Wrap operations with timing
    let result = database_context.execute_with_retry(
        "monitored_operation",
        |service| Box::pin(async move {
            let op_start = Instant::now();
            let result = service.expensive_operation().await;
            let op_duration = op_start.elapsed();
            
            // Log slow operations
            if op_duration.as_millis() > 1000 {
                warn!("Slow operation detected: {}ms", op_duration.as_millis());
            }
            
            result
        }),
        3,
    ).await;
    
    let total_duration = start_time.elapsed();
    
    // Log performance metrics
    info!("Operation completed in {}ms", total_duration.as_millis());
}
```

### 3. Error Tracking

```rust
async fn track_errors<T, F, Fut>(operation_name: &str, operation: F) -> Result<T, String>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, String>>,
{
    match operation().await {
        Ok(result) => {
            info!("Operation '{}' succeeded", operation_name);
            Ok(result)
        }
        Err(error) => {
            error!("Operation '{}' failed: {}", operation_name, error);
            
            // Track error for debugging
            if let Some(debugger) = get_debugger() {
                debugger.record_error(operation_name, &error).await;
            }
            
            Err(error)
        }
    }
}
```

## Common Migration Scenarios

### Scenario 1: Migrating from Direct Database Access

**Problem:** Tool directly accesses database service with blocking calls.

**Solution:**
```rust
// Before:
pub fn load_data(&self, id: &str) -> Result<Data, String> {
    let guard = self.db_service.lock().unwrap();
    guard.get_data(id).map_err(|e| e.to_string())
}

// After:
pub async fn load_data(&mut self, id: &str) -> DatabaseOperationResult<Data> {
    if let Some(context) = &mut self.database_context {
        context.execute_with_retry(
            "load_data",
            |service| Box::pin(async move { service.get_data(id).await }),
            3,
        ).await
    } else {
        DatabaseOperationResult::not_found("Database context", "unavailable")
    }
}
```

### Scenario 2: Replacing Global Static State

**Problem:** Using global static state with Mutex causing deadlocks.

**Solution:**
```rust
// Before:
lazy_static! {
    static ref GLOBAL_STATE: Mutex<MyState> = Mutex::new(MyState::new());
}

fn get_state() -> MutexGuard<'static, MyState> {
    GLOBAL_STATE.lock().unwrap()
}

// After:
lazy_static! {
    static ref GLOBAL_STATE_CONTAINER: GlobalStateContainer<MyState> = 
        GlobalStateContainer::new();
}

async fn get_state() -> RwLockReadGuard<'static, MyState> {
    GLOBAL_STATE_CONTAINER.read().await
}

async fn update_state(data: MyData) -> Result<(), String> {
    let mut state = GLOBAL_STATE_CONTAINER.write().await;
    state.update("my_tool", |s| {
        s.update(data);
        Ok(())
    }).await
}
```

### Scenario 3: Adding Tool Lifecycle Management

**Problem:** Tool lacks proper initialization and cleanup.

**Solution:**
```rust
// Before:
pub struct MyTool {
    data: Vec<Item>,
}

impl MyTool {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

// After:
#[async_trait]
impl ToolIntegration for MyTool {
    async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
        self.database_context = Some(database_context.clone());
        
        // Load initial data
        let result = self.load_initial_data().await;
        if result.is_success() {
            self.data = result.data.unwrap_or_default();
        }
        
        // Register with API contract
        let contract = get_api_contract();
        contract.register_tool("my_tool".to_string()).await?;
        
        Ok(())
    }
    
    async fn cleanup(&mut self) -> Result<(), String> {
        // Save state
        self.save_state().await?;
        
        // Clear resources
        self.data.clear();
        self.database_context = None;
        
        Ok(())
    }
}
```

## Prevention Strategies

### 1. Code Review Checklist

- [ ] All tools implement `ToolIntegration` trait
- [ ] Database operations use `ToolDatabaseContext`
- [ ] Global state uses `Arc<RwLock<T>>` not `Mutex<T>`
- [ ] Proper error handling with structured error types
- [ ] Event system integration for cross-tool communication
- [ ] Resource cleanup in `cleanup()` method
- [ ] Comprehensive unit and integration tests

### 2. Testing Strategy

```rust
#[cfg(test)]
mod migration_tests {
    use super::*;
    use tokio::test;
    
    #[tokio::test]
    async fn test_tool_initialization() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MyTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_tool", database_state).await;
        let result = tool.initialize(&mut database_context).await;
        
        assert!(result.is_ok());
        assert!(tool.database_context.is_some());
    }
    
    #[tokio::test]
    async fn test_concurrent_access() {
        let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
        let mut tool = MyTool::new();
        
        let mut database_context = ToolDatabaseContext::new("test_tool", database_state).await;
        tool.initialize(&mut database_context).await.unwrap();
        
        // Test concurrent operations don't deadlock
        let (result1, result2) = tokio::join!(
            tool.load_data("id1"),
            tool.load_data("id2")
        );
        
        assert!(result1.is_success());
        assert!(result2.is_success());
    }
}
```

### 3. Monitoring Setup

```rust
async fn setup_monitoring() {
    // Enable performance monitoring
    let debugger = MigrationDebugger::new();
    
    // Set up periodic health checks
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            
            let health_report = debugger.generate_health_report().await;
            if health_report.has_issues() {
                error!("Health check failed: {:#?}", health_report);
            }
        }
    });
}
```

## Getting Additional Help

### 1. Debugging Resources
- Use the migration debugger: `src/ui/tools/debugging_tools.rs`
- Check integration tests: `src/ui/tools/integration_tests.rs`
- Review architecture documentation: `docs/architecture_new_patterns.md`

### 2. When to Ask for Help
- Compilation errors persist after checking imports
- Runtime errors occur in production
- Performance issues can't be resolved
- Threading issues cause system instability

### 3. Information to Provide
- Complete error message and stack trace
- Code snippet showing the problematic area
- Steps to reproduce the issue
- Environment details (OS, Rust version, dependencies)
- Debugging output from migration tools

Remember: The migration tools are designed to catch and help resolve these issues. Use them proactively to identify and fix problems early in the migration process.