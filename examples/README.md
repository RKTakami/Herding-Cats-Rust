# Herding Cats Rust - Examples

This directory contains examples and templates to help you get started with Herding Cats Rust and understand its capabilities.

## 📁 **Example Categories**

### **Configuration Examples**
- [`config_example.toml`](config_example.toml) - Complete configuration template
- [`ai_config_example.toml`](ai_config_example.toml) - AI provider configuration examples
- [`database_config_example.toml`](database_config_example.toml) - Database configuration options

### **Theme Examples**
- [`themes/`](themes/) - Theme configuration and color scheme examples
- [`theme_integration_examples/`](theme_integration_examples/) - Advanced theme customization

### **Integration Examples**
- [`service_integration/`](service_integration/) - Service factory and dependency injection examples
- [`api_usage/`](api_usage/) - API usage patterns and best practices
- [`database_queries/`](database_queries/) - Common database query patterns

### **Development Examples**
- [`custom_tools/`](custom_tools/) - How to create custom writing tools
- [`ai_extensions/`](ai_extensions/) - AI service extension examples
- [`performance_optimization/`](performance_optimization/) - Performance tuning examples

## 🚀 **Quick Start Examples**

### **Basic Configuration**
```toml
# config_example.toml
[database]
path = "data/herding-cats.db"
wal_mode = true
connection_pool_size = 10

[ai]
default_provider = "openai"
usage_budget = 100.0

[backup]
automatic = true
interval = 3600
retention_days = 30
```

### **AI Provider Setup**
```toml
# ai_config_example.toml
[ai.providers.openai]
api_key = "your-openai-api-key"
model = "gpt-3.5-turbo"
temperature = 0.7
max_tokens = 4000

[ai.providers.claude]
api_key = "your-claude-api-key"
model = "claude-3-sonnet"
temperature = 0.5
max_tokens = 4000
```

### **Creating a Custom Tool**
```rust
// custom_tools/example_tool.rs
use herding_cats_rust::prelude::*;

pub struct ExampleTool {
    name: String,
    service_factory: Arc<dyn ServiceFactory>,
}

impl WritingTool for ExampleTool {
    fn new(service_factory: Arc<dyn ServiceFactory>) -> Self {
        Self {
            name: "Example Tool".to_string(),
            service_factory,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, ctx: &Context) -> Option<slint::ComponentHandle> {
        // Tool UI implementation
        None
    }
}
```

## 📚 **Detailed Examples**

### **1. Service Integration**
See [`service_integration/`](service_integration/) for complete examples of:
- ServiceFactory usage patterns
- Dependency injection best practices
- Cross-service communication
- Error handling strategies

### **2. Database Operations**
See [`database_queries/`](database_queries/) for:
- FTS5 search query examples
- Vector embedding operations
- Transaction management
- Performance optimization patterns

### **3. AI Integration**
See [`ai_extensions/`](ai_extensions/) for:
- Multi-provider AI routing
- Custom prompt templates
- Response parsing and validation
- Usage tracking and monitoring

### **4. UI Development**
See [`theme_integration_examples/`](theme_integration_examples/) for:
- Custom theme creation
- Component styling
- Responsive design patterns
- Accessibility implementation

## 🔧 **Configuration Templates**

### **Development Configuration**
```toml
# For development environment
[database]
path = "data/dev.db"
wal_mode = true
connection_pool_size = 5
debug = true

[ai]
default_provider = "local"
debug = true
log_requests = true

[logging]
level = "debug"
file = "logs/dev.log"
```

### **Production Configuration**
```toml
# For production environment
[database]
path = "/var/lib/herding-cats/prod.db"
wal_mode = true
connection_pool_size = 20
backup_enabled = true
backup_interval = 300

[ai]
default_provider = "openai"
usage_budget = 1000.0
rate_limit = 60
fallback_providers = ["claude", "local"]

[security]
encryption_enabled = true
audit_logging = true
rate_limiting = true

[backup]
automatic = true
interval = 3600
retention_days = 90
encryption = true
```

### **Testing Configuration**
```toml
# For testing environment
[database]
path = ":memory:"
wal_mode = false
connection_pool_size = 1
test_mode = true

[ai]
default_provider = "mock"
test_mode = true
mock_responses = true

[logging]
level = "warn"
file = "/tmp/test.log"
```

## 🎯 **Use Case Examples**

### **Writer Workflow**
```rust
// Complete writing workflow example
async fn writing_workflow(service_factory: Arc<dyn ServiceFactory>) -> Result<()> {
    let project_service = service_factory.get_project_service();
    let writing_tool = service_factory.get_writing_tool("hierarchy");
    let ai_service = service_factory.get_ai_service();

    // Create or load project
    let project = project_service.create_project("My Novel".to_string()).await?;

    // Generate outline with AI
    let outline_prompt = "Create a detailed outline for a fantasy novel about...";
    let outline = ai_service.generate_text(outline_prompt.into()).await?;

    // Create hierarchy structure
    writing_tool.import_outline(&outline).await?;

    // Start writing with real-time assistance
    let chapter_content = ai_service.chat(vec![
        Message::new("user", "Write the first chapter based on this outline..."),
    ]).await?;

    Ok(())
}
```

### **Research and Analysis**
```rust
// Research workflow with semantic search
async fn research_workflow(service_factory: Arc<dyn ServiceFactory>) -> Result<()> {
    let search_service = service_factory.get_search_service();
    let vector_service = service_service.get_vector_embedding_service();
    let analysis_service = service_factory.get_analysis_service();

    // Add research materials
    let research_doc = Document::new("research", "character_backstory.txt", content);
    vector_service.embed_document(&research_doc).await?;

    // Semantic search for relevant information
    let query = "character motivation and backstory";
    let results = search_service.semantic_search(query, 5).await?;

    // Analyze writing patterns
    let analysis = analysis_service.analyze_document(&research_doc).await?;

    Ok(())
}
```

## 📊 **Performance Examples**

### **Optimized Database Queries**
```rust
// Batch operations for performance
async fn batch_operations(service_factory: Arc<dyn ServiceFactory>) -> Result<()> {
    let project_service = service_factory.get_project_service();

    // Batch insert projects
    let projects = vec![
        Project::new("Project 1"),
        Project::new("Project 2"),
        // ... more projects
    ];

    project_service.batch_create_projects(&projects).await?;

    // Batch update with transaction
    project_service.batch_update_projects(|tx| {
        // Multiple operations in single transaction
        Ok(())
    }).await?;

    Ok(())
}
```

### **Caching Strategies**
```rust
// Implementing multi-tier caching
struct CachedProjectService {
    inner: Arc<dyn ProjectService>,
    memory_cache: Arc<Mutex<LruCache<ProjectId, Project>>>,
    disk_cache: Arc<dyn CacheStorage>,
}

impl CachedProjectService {
    async fn get_project_cached(&self, id: ProjectId) -> Result<Project> {
        // Check memory cache first
        if let Some(project) = self.memory_cache.lock().await.get(&id) {
            return Ok(project.clone());
        }

        // Check disk cache
        if let Some(project) = self.disk_cache.get(&id).await? {
            self.memory_cache.lock().await.put(id, project.clone());
            return Ok(project);
        }

        // Fetch from database
        let project = self.inner.get_project(id).await?;
        self.disk_cache.set(&id, &project).await?;
        self.memory_cache.lock().await.put(id, project.clone());

        Ok(project)
    }
}
```

## 🔍 **Debugging Examples**

### **Error Handling and Logging**
```rust
// Comprehensive error handling example
async fn robust_operation(service_factory: Arc<dyn ServiceFactory>) -> Result<()> {
    let error_handler = service_factory.get_error_handler();

    match operation().await {
        Ok(result) => {
            error_handler.record_success("operation").await?;
            Ok(result)
        }
        Err(e) => {
            error_handler.handle_error(&e).await?;

            // Automatic recovery attempt
            if error_handler.can_retry(&e) {
                error_handler.record_retry("operation").await?;
                operation().await
            } else {
                Err(e)
            }
        }
    }
}
```

### **Performance Monitoring**
```rust
// Performance tracking example
async fn monitored_operation(service_factory: Arc<dyn ServiceFactory>) -> Result<()> {
    let metrics = service_factory.get_metrics_service();
    let start_time = Instant::now();

    // Operation with timing
    let result = operation().await;

    let duration = start_time.elapsed();
    metrics.record_timing("operation", duration).await?;
    metrics.record_success("operation").await?;

    // Alert if too slow
    if duration > Duration::from_millis(100) {
        metrics.record_slow_operation("operation", duration).await?;
    }

    result
}
```

## 📈 **Analytics Examples**

### **Usage Analytics**
```rust
// Track tool usage and patterns
async fn analytics_example(service_factory: Arc<dyn ServiceFactory>) -> Result<()> {
    let analytics = service_factory.get_analytics_service();

    // Track tool usage
    analytics.track_tool_usage("hierarchy", "create_node").await?;

    // Track AI usage
    analytics.track_ai_usage("openai", "gpt-3.5-turbo", 1000).await?;

    // Track performance metrics
    analytics.track_performance_metric("query_time", 45.2).await?;

    // Generate usage report
    let report = analytics.generate_usage_report().await?;

    Ok(())
}
```

## 🚀 **Getting Started**

1. **Copy Configuration Templates**:
   ```bash
   cp examples/config_example.toml config/herding-cats.toml
   cp examples/ai_config_example.toml config/ai.toml
   ```

2. **Explore Examples**:
   ```bash
   # Browse through the examples directory
   ls examples/

   # Check specific example categories
   ls examples/service_integration/
   ls examples/database_queries/
   ```

3. **Run Examples**:
   ```bash
   # Build the project
   cargo build --release

   # Run specific examples (if available)
   cargo run --example service_integration
   ```

4. **Customize for Your Needs**:
   - Adapt configuration examples to your environment
   - Use code examples as starting points for custom development
   - Follow best practices demonstrated in the examples

## 📋 **Example Index**

### **Configuration**
- [`config_example.toml`](config_example.toml) - Basic configuration template
- [`ai_config_example.toml`](ai_config_example.toml) - AI configuration examples
- [`database_config_example.toml`](database_config_example.toml) - Database optimization

### **Code Examples**
- [`service_integration/`](service_integration/) - Service architecture patterns
- [`api_usage/`](api_usage/) - API usage examples
- [`custom_tools/`](custom_tools/) - Tool development examples
- [`performance_optimization/`](performance_optimization/) - Performance tuning

### **Data and Assets**
- [`themes/`](themes/) - Theme and styling examples
- [`sample_data/`](sample_data/) - Sample project data
- [`migrations/`](migrations/) - Database migration examples

### **Documentation**
- [`README.md`](README.md) - This file
- [`API_GUIDE.md`](API_GUIDE.md) - API usage documentation
- [`BEST_PRACTICES.md`](BEST_PRACTICES.md) - Development best practices

## 🤝 **Contributing Examples**

We welcome contributions to the examples directory! If you have:

1. **Configuration Templates**: For different use cases or environments
2. **Code Examples**: Demonstrating specific features or patterns
3. **Integration Guides**: For third-party services or tools
4. **Best Practices**: Documentation or code showing recommended approaches

Please submit a pull request with your examples!

## 📞 **Need Help?**

- **Documentation**: Check the main [README.md](../README.md) and [docs/](../docs/) directory
- **Issues**: [GitHub Issues](https://github.com/RKTakami/herding-cats-rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/RKTakami/herding-cats-rust/discussions)
- **Support**: [SUPPORT.md](../SUPPORT.md)

---

**Explore the examples to get the most out of Herding Cats Rust! 🚀**
