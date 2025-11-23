# Herding Cats Rust - Architecture Documentation

## Overview

Herding Cats Rust is an enterprise-grade writing assistant application built with Rust and Slint, featuring advanced AI integration, comprehensive database architecture, and a sophisticated multi-tool ecosystem.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              PRESENTATION LAYER                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐│
│  │   Slint UI      │ │   Ribbon UI     │ │ Multi-Panel     │ │ Window Manager  ││
│  │   Framework     │ │   Interface     │ │ Workspace       │ │ & Persistence   ││
│  └─────────────────┘ └─────────────────┘ └─────────────────┘ └─────────────────┘│
└─────────────────────────────────────────────────────────────────────────────────┘
                              │                       │
                              │                       │
┌─────────────────────────────────────────────────────────────────────────────────┐
│                            APPLICATION LAYER                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐│
│  │  ServiceFactory │ │   ToolManager   │ │ PerformanceMonitor│ │ ErrorManager    ││
│  │   & Services    │ │   & Registry    │ │   & Tracking    │ │ & Recovery      ││
│  └─────────────────┘ └─────────────────┘ └─────────────────┘ └─────────────────┘│
└─────────────────────────────────────────────────────────────────────────────────┘
                              │                       │
                              │                       │
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           BUSINESS LOGIC LAYER                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│  │ Hierarchy   │ │ Codex       │ │ Plot/Arc    │ │ Research    │ │ Analysis    ││
│  │ Tool        │ │ Tool        │ │ Tool        │ │ Tool        │ │ Tool        ││
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               ││
│  │ Structure   │ │ Brainstorm  │ │ Notes       │ │ AI Services │               ││
│  │ Tool        │ │ Tool        │ │ Tool        │ │ Integration │               ││
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘               ││
└─────────────────────────────────────────────────────────────────────────────────┘
                              │                       │
                              │                       │
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           DATA ACCESS LAYER                                     │
├─────────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐│
│  │ EnhancedDB     │ │ ProjectMgmt    │ │ SearchService   │ │ BackupService   ││
│  │ Service        │ │ Service        │ │               │ │                 ││
│  └─────────────────┘ └─────────────────┘ └─────────────────┘ └─────────────────┘│
│  ┌─────────────────┐ ┌─────────────────┐                                         ││
│  │ VectorEmbedding │ │ Analysis      │                                         ││
│  │ Service         │ │ Service       │                                         ││
│  └─────────────────┘ └─────────────────┘                                         ││
└─────────────────────────────────────────────────────────────────────────────────┘
                              │                       │
                              │                       │
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          INFRASTRUCTURE LAYER                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│  │ SQLite DB   │ │ File System │ │ AI Providers│ │ Security    │ │ Monitoring    ││
│  │ (WAL Mode)  │ │ (Persistence)│ │ (Multi-Cloud)│ │ (Encryption)│ │ (Metrics)    ││
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Presentation Layer

#### Slint UI Framework
- **Purpose**: Modern declarative UI framework for native performance
- **Features**: Hardware-accelerated rendering, cross-platform consistency
- **Components**: Main window, dialogs, custom controls, `FontManagerWindow`, `AiSettingsPopup`

#### Ribbon Interface
- **Purpose**: Professional ribbon-style UI with tabs and command groups
- **Features**: Context-sensitive tabs, collapsible groups, keyboard shortcuts
- **Components**: `RibbonManager`, `RibbonTab`, `RibbonGroup`, `RibbonItem`

#### Multi-Panel Workspace
- **Purpose**: Flexible workspace with dockable and floating panels
- **Features**: Panel docking, state persistence, layout management
- **Components**: `WorkspaceManager`, `PanelConfig`, `DefaultLayout`

#### Window Manager
- **Purpose**: Advanced window persistence and layout management
- **Features**: State saving/restoring, auto-save functionality, layout templates
- **Components**: `WindowStateManager`, `EnhancedWindowManager`

#### Universal Windows Framework
- **Purpose**: Single-window interface with dropdown tool selection for streamlined workflow
- **Features**: Tool-specific toolbars, state preservation, keyboard shortcuts, status bar integration
- **Components**: `UniversalWritingTools`, `UniversalToolsConfig`, `UniversalWritingToolsWindow`

### 2. Application Layer

#### Service Factory Pattern
- **Purpose**: Centralized dependency injection and service orchestration
- **Features**: Service lifecycle management, health monitoring, graceful shutdown
- **Components**: `ServiceFactory`, `ServiceContainer`, `ServiceHealthStatus`

#### Tool Management System
- **Purpose**: Unified management of all writing tools
- **Features**: Tool registration, lifecycle management, cross-tool communication
- **Components**: `ToolManager`, `ToolRegistry`, `ToolEventHandler`

#### Performance Monitoring
- **Purpose**: Real-time performance metrics and optimization tracking
- **Features**: Operation timing, success rate tracking, performance analytics
- **Components**: `PerformanceMonitor`, `PerformanceTracker`, `OperationStats`

#### Error Management
- **Purpose**: Comprehensive error handling and user-friendly recovery
- **Features**: Error categorization, automatic recovery, user guidance
- **Components**: `ComprehensiveErrorHandlingManager`, `UserFriendlyError`

### 3. Business Logic Layer

#### Writing Tools Suite

##### Hierarchy Tool
- **Purpose**: Tree-structured document organization
- **Features**: Manuscript → Chapter → Scene hierarchy, drag-and-drop reorganization
- **Data Model**: `HierarchyItem`, `HierarchyData`, recursive tree structure

##### Codex Tool
- **Purpose**: World-building database with cross-references
- **Features**: Entity management, relationship mapping, search capabilities
- **Data Model**: `CodexEntry`, `CodexRelationship`, `CodexCategory`

##### Plot/Arc Tool
- **Purpose**: Story structure visualization and development
- **Features**: Multiple template support, beat tracking, progress monitoring
- **Data Model**: `PlotStructure`, `PlotBeat`, `StoryArc`

##### Research Tool
- **Purpose**: Mind mapping and research organization
- **Features**: Node creation, connection mapping, visual organization
- **Data Model**: `ResearchNode`, `ResearchConnection`, `ResearchMap`

##### Analysis Tool
- **Purpose**: Multi-panel writing analysis and metrics
- **Features**: Writing statistics, readability analysis, progress tracking
- **Data Model**: `WritingMetrics`, `AnalysisReport`, `ProgressData`

##### Structure Tool
- **Purpose**: Comprehensive plot development with multiple methodologies
- **Features**: 6 plot structure types, stage management, completion tracking
- **Data Model**: `StructureData`, `PlotStage`, `PlotType`

##### Brainstorming Tool
- **Purpose**: AI-powered idea generation and organization
- **Features**: Prompt-based generation, clustering, organization
- **Data Model**: `BrainstormSession`, `IdeaCluster`, `GeneratedIdea`

##### Notes Tool
- **Purpose**: Flexible note-taking and organization
- **Features**: Rich text support, categorization, search capabilities
- **Data Model**: `Note`, `NoteCategory`, `NoteTag`

#### AI Service Integration
- **Purpose**: Multi-provider AI capabilities with smart routing
- **Features**: Provider selection (OpenAI, Anthropic, Gemini, OpenRouter), cost optimization, fallback mechanisms
- **Components**: `AiServiceManager`, `AiRouter`, `UsageTracker`, `SecureStorageService`

### 4. Data Access Layer

#### Universal Window Services
- **UniversalWindowService**: Centralized management of universal window operations
- **ToolStateManager**: Cross-tool state preservation and restoration
- **ConfigurationService**: User preference management for window behavior
- **EventBroker**: Cross-tool communication and event coordination

#### Database Services

##### EnhancedDatabaseService
- **Purpose**: Enterprise-grade database operations with integrity checking
- **Features**: WAL mode, connection pooling, automatic repair, performance optimization
- **Configuration**: `DatabaseConfig` with cache settings, synchronous mode

##### ProjectManagementService
- **Purpose**: Complete project lifecycle management
- **Features**: Multi-project support, project settings, archiving
- **Data Model**: `Project`, `ProjectSettings`, `ProjectStatistics`

##### SearchService
- **Purpose**: Full-text search with FTS5 and advanced filtering
- **Features**: Manual indexing control, search analytics, history tracking
- **Configuration**: `SearchOptions`, `SortField`, `DateRange`

##### BackupService
- **Purpose**: Comprehensive backup and recovery management
- **Features**: Multiple backup types, integrity verification, scheduling
- **Data Model**: `BackupMetadata`, `BackupType`, `BackupStatistics`

##### VectorEmbeddingService
- **Purpose**: AI-powered semantic search and document analysis
- **Features**: Vector storage, chunk management, semantic similarity
- **Data Model**: `DocumentEmbedding`, `EmbeddingChunk`, `SimilarityResult`

### 5. Infrastructure Layer

#### Database Schema
```sql
-- Core Tables
projects (id, name, description, created_at, updated_at, is_archived, is_active, settings)
documents (id, project_id, title, content, document_type, word_count, checksum, created_at, updated_at, is_active, version, metadata)
document_embeddings (id, document_id, vector_data, model_name, chunk_index, text_chunk, start_char, end_char, created_at, metadata)
document_fts (title, content) -- FTS5 virtual table
document_versions (id, document_id, version, title, content, created_at, change_description)
change_log (id, project_id, document_id, change_type, change_description, timestamp, user_identifier, metadata)
backup_metadata (id, backup_type, file_path, file_size, checksum, created_at, project_id, description, success, error_message)
project_settings (id, project_id, setting_key, setting_value, setting_type, created_at, updated_at)
```

#### Security Features
- **Encrypted Storage**: API keys and sensitive data encryption via system keyring (`keyring` crate)
- **Rate Limiting**: Protection against API abuse and DoS attacks
- **Input Validation**: Comprehensive validation with sanitization
- **Audit Logging**: Security event tracking and compliance

#### Performance Features
- **Connection Pooling**: Optimized database connection management
- **Caching Strategy**: Multi-tier caching for frequently accessed data
- **Query Optimization**: Prepared statements and index optimization
- **Memory Management**: Efficient allocation and cleanup patterns

## Data Flow

### 1. Application Startup
```
1. ServiceFactory::new() → Initialize service factory
2. ServiceFactory::initialize() → Create and configure all services
3. ServiceFactory::health_check() → Verify service health
4. UI::initialize() → Initialize Slint UI and tool registry
5. AppState::load() → Restore previous application state
```

### 2. Tool Integration
```
1. ToolManager::register_tool() → Register tool with manager
2. Tool::initialize() → Initialize tool-specific services
3. Tool::render() → Render tool UI in Slint
4. ToolEventHandler::handle_event() → Process cross-tool events
5. Tool::update() → Update tool state and UI
```

### 3. Database Operations
```
1. ServiceFactory::get_service() → Get database service
2. Service::operation() → Perform database operation
3. Transaction::begin() → Start transaction (if needed)
4. Query::execute() → Execute optimized query
5. Transaction::commit() → Commit transaction
6. Cache::update() → Update cache (if applicable)
```

### 4. AI Service Flow
```
1. AiRouter::route_request() → Select optimal provider
2. AiServiceProvider::send_request() → Send request to provider
3. UsageTracker::record_usage() → Track usage and costs
4. Response::process() → Process and validate response
5. Fallback::handle_failure() → Handle failures if needed
```

## Configuration

### Application Configuration
```toml
[database]
path = "data/database.db"
wal_mode = true
cache_size = 10000
backup_dir = "data/backups"

[ai]
providers = ["openai", "anthropic"]
cost_limit = 10.0
timeout_seconds = 30
fallback_enabled = true

[performance]
metrics_enabled = true
auto_save_interval = 30
monitoring_retention_days = 30

[security]
rate_limit_requests = 100
rate_limit_window = 60
encryption_enabled = true
audit_logging = true
```

### Build Configuration
```toml
[profile.release]
lto = true
opt-level = 3
codegen-units = 1

[profile.dev]
opt-level = 0
debug = true
```

## Testing Strategy

### Unit Tests
- **Coverage**: 80%+ code coverage across all modules
- **Focus**: Individual component functionality and edge cases
- **Tools**: Rust standard testing, mock objects for dependencies

### Integration Tests
- **Database**: Full database operation testing with real schemas
- **UI**: Slint component integration and interaction testing
- **Services**: End-to-end service orchestration testing

### Performance Tests
- **Database**: Query performance and optimization validation
- **Memory**: Memory usage and leak detection
- **AI Services**: Response time and throughput testing

### Acceptance Tests
- **User Workflows**: Complete user scenarios and workflows
- **Cross-Tool**: Integration between different writing tools
- **Error Recovery**: Error handling and recovery validation

## Deployment

### Production Build
```bash
cargo build --release
strip herding-cats-rust
upx --best herding-cats-rust  # Optional compression
```

### Distribution
- **Platforms**: Windows, macOS, Linux
- **Formats**: Native executables, installers
- **Dependencies**: Static linking for maximum compatibility

### Monitoring
- **Health Checks**: Service availability and performance metrics
- **Usage Analytics**: Feature usage and performance tracking
- **Error Reporting**: Automatic error collection and analysis

## Future Enhancements

### Planned Features
1. **Plugin Architecture**: Extensible plugin system for custom tools
2. **Collaboration**: Multi-user editing and real-time collaboration
3. **Cloud Sync**: Cloud-based project synchronization
4. **Advanced AI**: Fine-tuned models and custom prompt engineering
5. **Mobile Support**: Responsive UI for mobile devices

### Architecture Improvements
1. **Microservices**: Service decomposition for scalability
2. **Event Sourcing**: Event-driven architecture for auditability
3. **CQRS**: Command Query Responsibility Segregation for performance
4. **Containerization**: Docker support for deployment flexibility
5. **Universal Windows**: Enhanced single-window interface with improved tool integration

### Universal Windows Architecture
The Universal Windows framework provides a streamlined approach to accessing multiple writing tools through a single, organized interface:

#### Core Components
- **UniversalWritingTools**: Main universal window implementation with tool state management
- **UniversalToolsConfig**: Configuration management for window behavior and preferences
- **UniversalWritingToolsWindow**: Slint UI component with dropdown menu and tool-specific toolbars
- **Tool State Management**: Preservation of tool states during switches with automatic restoration

#### Integration Benefits
- **Space Efficiency**: Single window reduces desktop clutter and improves screen real estate utilization
- **Enhanced UX**: Consistent interface across all tools with easy switching via dropdown or keyboard shortcuts
- **State Preservation**: Tool states are maintained when switching, providing seamless workflow continuity
- **Professional Appearance**: Unified interface with status bar integration and contextual toolbars
- **Accessibility**: Keyboard-driven navigation with comprehensive shortcut system

#### Technical Implementation
- **Dropdown Menu System**: Title bar integration with tool selection interface
- **Dynamic Toolbars**: Context-sensitive toolbars that change based on active tool
- **State Synchronization**: Real-time state updates across tool switches
- **Event Broadcasting**: Tool switching events for cross-tool integration
- **Configuration Management**: User preferences for window behavior and tool defaults

## Conclusion

The Herding Cats Rust architecture represents a comprehensive, enterprise-grade approach to writing assistant applications. With its multi-layered design, advanced AI integration, and sophisticated tool ecosystem, it provides a solid foundation for professional writers and content creators.

The modular architecture ensures maintainability and extensibility, while the comprehensive testing and monitoring systems guarantee reliability and performance. As the application evolves, the architecture will continue to support new features and capabilities while maintaining its core principles of quality, security, and user experience.