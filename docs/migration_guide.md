# Migration Guide: UI Layer Architecture Upgrade

## Overview

This guide provides step-by-step instructions for migrating existing UI tools from the legacy patterns to the new unified architecture. The migration process is designed to be incremental and low-risk, allowing for gradual adoption while maintaining system stability.

## Prerequisites

Before starting the migration:

1. **Review the New Architecture Documentation**
   - Read [`docs/architecture_new_patterns.md`](./architecture_new_patterns.md)
   - Understand the core components: `ToolDatabaseContext`, `ThreadSafeToolRegistry`, and `ToolApiContract`

2. **Set Up Development Environment**
   - Ensure you have the latest codebase with new architecture components
   - Familiarize yourself with the migration helper tools in `src/ui/tools/migration_helpers.rs`
   - Set up debugging tools in `src/ui/tools/debugging_tools.rs`

3. **Backup and Version Control**
   - Create a backup branch for your current implementation
   - Ensure all changes are committed before starting migration
   - Consider using feature flags for gradual rollout

## Migration Process

### Phase 1: Preparation (Already Completed)

✅ **Foundation Setup**: The new architecture components are implemented and tested.

### Phase 2: Tool-by-Tool Migration

#### Step 1: Assess Current Implementation

For each tool you're migrating:

1. **Identify Legacy Patterns**
   ```rust
   // Look for these patterns in your code:
   lazy_static! {
       static ref GLOBAL_MANAGER: Mutex<SomeType> = Mutex::new(SomeType::new());
   }
   
   pub struct MyTool {
       database_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
       // Other legacy patterns...
   }
   ```

2. **Document Dependencies**
   - List all database operations used
   - Identify threading patterns and global state usage
   - Note any custom error handling logic
   - Document current tool lifecycle

3. **Create Migration Plan**
   ```rust
   use crate::ui::tools::migration_helpers::*;
   
   let helper = MigrationHelper::new();
   helper.start_assessment("my_tool").await?;
   
   let analysis = helper.analyze_database_patterns(tool_type, &mut database_context).await?;
   let plan = helper.generate_migration_plan(tool_type, &analysis).await?;
   ```

#### Step 2: Update Tool Structure

1. **Replace Database Access**
   ```rust
   // Before:
   pub struct MyTool {
       database_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
       // ... other fields
   }
   
   // After:
   pub struct MyTool {
       database_context: Option<ToolDatabaseContext>,
       api_contract: Arc<ToolApiContract>,
       // ... other fields
   }
   ```

2. **Implement Standardized Interface**
   ```rust
   #[async_trait]
   impl ToolIntegration for MyTool {
       async fn initialize(&mut self, database_context: &mut ToolDatabaseContext) -> Result<(), String> {
           self.database_context = Some(database_context.clone());
           self.api_contract = get_api_contract().clone();
           
           // Initialize tool-specific state
           Ok(())
       }
       
       fn update(&mut self) -> Result<(), String> {
           // Update tool logic
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

#### Step 3: Migrate Database Operations

1. **Replace Direct Database Calls**
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

2. **Update Error Handling**
   ```rust
   // Before:
   match result {
       Ok(data) => data,
       Err(e) => return Err(format!("Operation failed: {}", e)),
   }
   
   // After:
   match result {
       Ok(data) => DatabaseOperationResult::success(data, timing),
       Err(DatabaseError::NotFound(_)) => DatabaseOperationResult::not_found("Entity", id),
       Err(e) => DatabaseOperationResult::validation_error("field", e.to_string()),
   }
   ```

#### Step 4: Migrate Threading Patterns

1. **Replace Global State Access**
   ```rust
   // Before:
   fn get_global_state() -> MutexGuard<'static, MyState> {
       GLOBAL_STATE.lock().unwrap()
   }
   
   // After:
   async fn get_global_state() -> RwLockReadGuard<'static, MyState> {
       GLOBAL_STATE_CONTAINER.read().await
   }
   ```

2. **Update State Mutations**
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

#### Step 5: Integrate with API Contract

1. **Register Tool**
   ```rust
   async fn register_tool_with_api(&self) -> Result<(), String> {
       let contract = get_api_contract();
       contract.register_tool("my_tool".to_string()).await?;
       
       // Broadcast initialization event
       contract.broadcast_lifecycle(ToolLifecycleEvent::Initialized {
           tool_id: "my_tool".to_string(),
           timestamp: Instant::now(),
           version: env!("CARGO_PKG_VERSION").to_string(),
       }).await?;
       
       Ok(())
   }
   ```

2. **Implement Event Handling**
   ```rust
   async fn handle_events(&self) -> Result<(), String> {
       let contract = get_api_contract();
       let mut event_receiver = contract.subscribe_to_events("my_tool");
       
       while let Some(event) = event_receiver.recv().await {
           match event {
               ToolEvent::DataChanged { source, data } => {
                   // Handle data change
               }
               ToolEvent::ConfigurationChanged { config } => {
                   // Handle config change
               }
               // Handle other events...
           }
       }
       
       Ok(())
   }
   ```

### Step 6: Add Testing

1. **Unit Tests**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       use tokio::test;
       
       #[tokio::test]
       async fn test_tool_initialization() {
           let database_state = Arc::new(RwLock::new(DatabaseAppState::new()));
           let mut tool = MyTool::new();
           
           let mut database_context = ToolDatabaseContext::new("test_tool", database_state).await;
           let result = tool.initialize(&mut database_context).await;
           
           assert!(result.is_ok());
       }
   }
   ```

2. **Integration Tests**
   ```rust
   #[tokio::test]
   async fn test_tool_integration() {
       let helper = MigrationHelper::new();
       let pipeline = MigrationTestingPipeline::new(TestConfiguration::default());
       
       let plan = create_test_migration_plan();
       let result = pipeline.run_migration_tests("my_tool", &plan).await.unwrap();
       
       assert!(result.overall_success);
   }
   ```

### Step 7: Validate and Deploy

1. **Run Migration Tests**
   ```rust
   let debugger = MigrationDebugger::new();
   let session_id = debugger.start_debug_session("my_tool").await?;
   
   // Run comprehensive tests
   let trace_result = debugger.trace_database_operations(&session_id, &mut database_context, 100).await?;
   let thread_report = debugger.monitor_thread_safety(&session_id, "my_tool").await?;
   let memory_metrics = debugger.collect_memory_metrics(&session_id).await?;
   
   let report = debugger.complete_debug_session(&session_id).await?;
   ```

2. **Performance Validation**
   - Verify that performance meets or exceeds previous levels
   - Check memory usage patterns
   - Validate concurrent operation handling
   - Ensure error rates are acceptable

3. **Deploy with Monitoring**
   - Enable debugging tools in development environment
   - Monitor error rates and performance metrics
   - Be prepared to rollback if issues are detected

## Common Migration Scenarios

### Migrating a Hierarchy Tool

```rust
// Before:
pub struct HierarchyTool {
    database_service: Option<Arc<RwLock<HierarchyDatabaseService>>>,
    selected_item: Option<String>,
    // ... other fields
}

// After:
pub struct HierarchyTool {
    database_context: Option<ToolDatabaseContext>,
    api_contract: Arc<ToolApiContract>,
    selected_item: Option<String>,
    // ... other fields
}

impl HierarchyTool {
    async fn load_hierarchy(&mut self, project_id: &str) -> DatabaseOperationResult<Vec<HierarchyItem>> {
        if let Some(context) = &mut self.database_context {
            context.execute_with_retry(
                "load_hierarchy",
                |service| Box::pin(async move {
                    service.get_hierarchy_items(project_id).await
                }),
                3,
            ).await
        } else {
            DatabaseOperationResult::not_found("Database context", "unavailable")
        }
    }
}
```

### Migrating a Codex Tool

```rust
// Before:
pub struct CodexTool {
    database_service: Option<Arc<RwLock<EnhancedDatabaseService>>>,
    entry_type: CodexEntryType,
    // ... other fields
}

// After:
pub struct CodexTool {
    database_context: Option<ToolDatabaseContext>,
    api_contract: Arc<ToolApiContract>,
    entry_type: CodexEntryType,
    // ... other fields
}

impl CodexTool {
    async fn create_entry(&mut self, entry: &CodexEntry) -> DatabaseOperationResult<()> {
        if let Some(context) = &mut self.database_context {
            context.execute_with_retry(
                "create_entry",
                |service| Box::pin(async move {
                    service.create_codex_entry(entry).await
                }),
                3,
            ).await
        } else {
            DatabaseOperationResult::validation_error("Database context", "unavailable")
        }
    }
}
```

## Troubleshooting

### Common Issues and Solutions

1. **Compilation Errors**
   ```
   Error: cannot find function `get_tool_registry` in crate `crate::ui::tools::threading_patterns`
   ```
   **Solution**: Ensure you've imported the function:
   ```rust
   use crate::ui::tools::threading_patterns::get_tool_registry;
   ```

2. **Runtime Errors**
   ```
   Error: Database context not initialized
   ```
   **Solution**: Ensure `initialize()` is called before using database operations:
   ```rust
   async fn setup_tool() {
       let mut tool = MyTool::new();
       let mut database_context = ToolDatabaseContext::new("my_tool", database_state).await;
       tool.initialize(&mut database_context).await?;
   }
   ```

3. **Threading Issues**
   ```
   Error: deadlock detected
   ```
   **Solution**: Use `Arc<RwLock<T>>` instead of `Mutex<T>` and avoid nested locks.

4. **Performance Issues**
   ```
   Error: operation timeout exceeded
   ```
   **Solution**: Adjust timeout settings in `ToolDatabaseContext` or `OperationTimeout`.

### Debugging Tools

1. **Use Migration Debugger**
   ```rust
   let debugger = MigrationDebugger::new();
   let session_id = debugger.start_debug_session("problematic_tool").await?;
   
   // Run specific diagnostics
   let trace = debugger.trace_database_operations(&session_id, &mut context, 50).await?;
   let thread_report = debugger.monitor_thread_safety(&session_id, "problematic_tool").await?;
   
   let report = debugger.complete_debug_session(&session_id).await?;
   println!("Debug report: {:#?}", report);
   ```

2. **Enable Tracing**
   ```rust
   use tracing::{debug, error, info};
   
   // Add tracing to your tool operations
   async fn my_operation(&mut self) -> Result<(), String> {
       debug!("Starting my_operation");
       
       // Operation logic here
       
       info!("Completed my_operation successfully");
       Ok(())
   }
   ```

## Best Practices

### 1. Incremental Migration
- Migrate one tool at a time
- Test thoroughly after each migration
- Keep legacy and new patterns working in parallel during transition

### 2. Comprehensive Testing
- Write unit tests for all new methods
- Add integration tests for tool interactions
- Use the testing pipeline for validation

### 3. Performance Monitoring
- Monitor database operation latency
- Track memory usage patterns
- Validate concurrent operation handling

### 4. Error Handling
- Use structured error types consistently
- Implement proper retry logic
- Provide meaningful error messages

### 5. Documentation
- Update tool documentation with new patterns
- Add examples for common operations
- Document any breaking changes

## Migration Checklist

For each tool migration:

- [ ] Analyze current implementation and identify legacy patterns
- [ ] Create migration plan using `MigrationHelper`
- [ ] Replace direct database service usage with `ToolDatabaseContext`
- [ ] Implement `ToolIntegration` trait
- [ ] Update threading patterns to use `Arc<RwLock<T>>`
- [ ] Integrate with `ToolApiContract` and event system
- [ ] Update error handling to use structured error types
- [ ] Add comprehensive unit and integration tests
- [ ] Run migration testing pipeline
- [ ] Validate performance and fix any regressions
- [ ] Document changes and update examples
- [ ] Deploy with monitoring enabled

## Support and Resources

- **Architecture Documentation**: [`docs/architecture_new_patterns.md`](./architecture_new_patterns.md)
- **API Reference**: Check inline documentation in source files
- **Integration Tests**: [`src/ui/tools/integration_tests.rs`](../src/ui/tools/integration_tests.rs)
- **Migration Helpers**: [`src/ui/tools/migration_helpers.rs`](../src/ui/tools/migration_helpers.rs)
- **Debugging Tools**: [`src/ui/tools/debugging_tools.rs`](../src/ui/tools/debugging_tools.rs)
- **Testing Pipeline**: [`src/ui/tools/testing_pipeline.rs`](../src/ui/tools/testing_pipeline.rs)

## Getting Help

If you encounter issues during migration:

1. **Check the Debugging Tools**: Use the comprehensive debugging tools to identify issues
2. **Review Integration Tests**: Look at working examples in the integration tests
3. **Consult Documentation**: Review the architecture patterns documentation
4. **Ask for Help**: Reach out to the development team with specific error messages and code examples

Remember: The migration is designed to be incremental and safe. Take your time with each step and ensure thorough testing before proceeding to the next tool.