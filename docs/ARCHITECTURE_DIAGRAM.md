# Herding Cats Rust - Architecture Diagrams

## System Overview Diagram

```mermaid
graph TB
    subgraph "Presentation Layer"
        A1[Slint UI Framework]
        A2[Ribbon Interface]
        A3[Multi-Panel Workspace]
        A4[Window Manager]
    end
    
    subgraph "Application Layer"
        B1[Service Factory]
        B2[Tool Manager]
        B3[Performance Monitor]
        B4[Error Manager]
    end
    
    subgraph "Business Logic Layer"
        C1[Hierarchy Tool]
        C2[Codex Tool]
        C3[Plot/Arc Tool]
        C4[Research Tool]
        C5[Analysis Tool]
        C6[Structure Tool]
        C7[Brainstorming Tool]
        C8[Notes Tool]
        C9[AI Service Integration]
    end
    
    subgraph "Data Access Layer"
        D1[Enhanced DB Service]
        D2[Project Management]
        D3[Search Service]
        D4[Backup Service]
        D5[Vector Embedding Service]
    end
    
    subgraph "Infrastructure Layer"
        E1[SQLite Database]
        E2[File System]
        E3[AI Providers]
        E4[Security Module]
        E5[Performance Metrics]
    end
    
    A1 --> B1
    A2 --> B2
    A3 --> B3
    A4 --> B4
    
    B1 --> C1
    B1 --> C2
    B1 --> C3
    B1 --> C4
    B1 --> C5
    B1 --> C6
    B1 --> C7
    B1 --> C8
    B1 --> C9
    
    C1 --> D1
    C2 --> D2
    C3 --> D3
    C4 --> D4
    C5 --> D5
    C6 --> D1
    C7 --> D3
    C8 --> D2
    C9 --> E3
    
    D1 --> E1
    D2 --> E1
    D3 --> E1
    D4 --> E2
    D5 --> E1
```

## Tool Integration Flow Diagram

```mermaid
sequenceDiagram
    participant UI as Slint UI
    participant TM as Tool Manager
    participant SF as Service Factory
    participant DB as Database Service
    participant AI as AI Service
    participant PM as Performance Monitor
    
    UI->>TM: Open Tool Request
    TM->>SF: Get Service Instance
    SF->>SF: Service Health Check
    SF->>TM: Return Service
    TM->>UI: Initialize Tool UI
    
    loop User Interactions
        UI->>TM: Tool Event (Create/Update/Delete)
        TM->>DB: Database Operation
        DB->>DB: Execute Query
        DB->>TM: Return Result
        TM->>AI: Contextual AI Request (if needed)
        AI->>AI: Process Request
        AI->>TM: Return AI Response
        TM->>PM: Log Performance Metrics
        TM->>UI: Update UI State
    end
    
    UI->>TM: Close Tool
    TM->>PM: Save Performance Data
    TM->>TM: Cleanup Resources
```

## Database Schema Diagram

```mermaid
erDiagram
    projects {
        string id PK
        string name
        string description
        datetime created_at
        datetime updated_at
        boolean is_archived
        boolean is_active
        text settings
    }
    
    documents {
        string id PK
        string project_id FK
        string title
        text content
        string document_type
        integer word_count
        string checksum
        datetime created_at
        datetime updated_at
        boolean is_active
        integer version
        text metadata
    }
    
    document_embeddings {
        string id PK
        string document_id FK
        blob vector_data
        string model_name
        integer chunk_index
        text text_chunk
        integer start_char
        integer end_char
        datetime created_at
        text metadata
    }
    
    document_versions {
        string id PK
        string document_id FK
        integer version
        string title
        text content
        datetime created_at
        text change_description
    }
    
    change_log {
        string id PK
        string project_id FK
        string document_id FK
        string change_type
        text change_description
        datetime timestamp
        string user_identifier
        text metadata
    }
    
    backup_metadata {
        string id PK
        string backup_type
        string file_path
        integer file_size
        string checksum
        integer created_at
        string project_id FK
        string description
        boolean success
        text error_message
    }
    
    project_settings {
        string id PK
        string project_id FK
        string setting_key
        string setting_value
        string setting_type
        datetime created_at
        datetime updated_at
    }
    
    projects ||--o{ documents : contains
    documents ||--o{ document_embeddings : has
    documents ||--o{ document_versions : versions
    projects ||--o{ change_log : logs
    documents ||--o{ change_log : logs
    projects ||--o{ backup_metadata : backups
    projects ||--o{ project_settings : settings
```

## Service Factory Pattern Diagram

```mermaid
classDiagram
    class ServiceFactory {
        -DatabaseConfig database_config
        -PathBuf db_path
        -PathBuf backup_dir
        +new() ServiceFactory
        +initialize() ServiceContainer
        +health_check(container) ServiceHealthStatus
        +shutdown(container) Result
        +restart_service(container, service_name) Result
    }
    
    class ServiceContainer {
        +database_service: Option<Arc<RwLock<EnhancedDatabaseService>>>
        +project_service: Option<Arc<RwLock<ProjectManagementService>>>
        +vector_service: Option<Arc<RwLock<VectorEmbeddingService>>>
        +search_service: Option<Arc<RwLock<SearchService>>>
        +backup_service: Option<Arc<RwLock<BackupService>>>
        +is_healthy() bool
    }
    
    class ServiceHealthStatus {
        +overall_health: ServiceHealth
        +service_healths: HashMap<string, ServiceHealth>
        +timestamp: DateTime<Utc>
        +issues: Vec<string>
        +add_service_health(service_name, health)
    }
    
    class ServiceHealth {
        <<enumeration>>
        Healthy
        Unhealthy
        Error
    }
    
    ServiceFactory --> ServiceContainer
    ServiceFactory --> ServiceHealthStatus
    ServiceHealthStatus --> ServiceHealth
```

## Tool Registry System Diagram

```mermaid
classDiagram
    class ToolManager {
        +registry: ToolRegistry
        +event_handlers: Vec<Box<dyn ToolEventHandler>>
        +initialize_tools() Result
        +register_event_handler(handler)
        +broadcast_event(event) Result
    }
    
    class ToolRegistry {
        -tool_states: HashMap<ToolType, ToolWindowState>
        +register_tool(tool_type)
        +set_tool_state(tool_type, state)
        +get_tool_state(tool_type) Option
        +open_tool(tool_type)
        +close_tool(tool_type)
        +is_tool_open(tool_type) bool
    }
    
    class ToolWindowState {
        +is_open: bool
        +position: (i32, i32)
        +size: (u32, u32)
        +z_index: i32
    }
    
    class ToolEvent {
        <<enumeration>>
        ToolOpened(ToolType)
        ToolClosed(ToolType)
        ToolFocused(ToolType)
        DataChanged{tool, data}
        DragStarted{tool, data}
        DragCompleted{tool, target_tool, data}
    }
    
    class ToolType {
        <<enumeration>>
        Hierarchy
        Codex
        Notes
        Research
        Plot
        Analysis
        Structure
        Brainstorming
    }
    
    ToolManager --> ToolRegistry
    ToolManager --> ToolEvent
    ToolRegistry --> ToolWindowState
    ToolRegistry --> ToolType
```

## AI Service Architecture Diagram

```mermaid
graph TB
    subgraph "AI Service Layer"
        A1[AI Service Manager]
        A2[AI Router]
        A3[Usage Tracker]
        A4[Provider Fallback]
    end
    
    subgraph "AI Providers"
        B1[OpenAI]
        B2[Anthropic Claude]
        B3[Local Models]
        B4[Custom Providers]
    end
    
    subgraph "Security & Management"
        C1[AI Key Manager]
        C2[Encrypted Storage]
        C3[Rate Limiter]
        C4[Cost Analyzer]
    end
    
    subgraph "Configuration"
        D1[AI Configuration]
        D2[Provider Priority]
        D3[Cost Limits]
        D4[Timeout Settings]
    end
    
    A1 --> A2
    A2 --> A3
    A3 --> A4
    
    A2 --> B1
    A2 --> B2
    A2 --> B3
    A2 --> B4
    
    A1 --> C1
    C1 --> C2
    C1 --> C3
    A3 --> C4
    
    A1 --> D1
    D1 --> D2
    D1 --> D3
    D1 --> D4
```

## Performance Monitoring Architecture

```mermaid
graph LR
    subgraph "Performance Components"
        A[PerformanceMonitor]
        B[PerformanceTracker]
        C[OperationStats]
        D[PerformanceSummary]
    end
    
    subgraph "Data Collection"
        E[Metrics Collection]
        F[Success Rate Tracking]
        G[Duration Monitoring]
        H[Error Logging]
    end
    
    subgraph "Analysis & Reporting"
        I[Performance Analytics]
        J[Optimization Suggestions]
        K[Bottleneck Detection]
        L[Report Generation]
    end
    
    A --> B
    A --> C
    A --> D
    
    B --> E
    B --> F
    B --> G
    B --> H
    
    A --> I
    I --> J
    I --> K
    I --> L
```

## Cross-Tool Data Flow Diagram

```mermaid
graph TD
    A[Hierarchy Tool] --> B[Codex Tool]
    A --> C[Plot Tool]
    A --> D[Analysis Tool]
    
    B --> E[Research Tool]
    B --> F[AI Services]
    
    C --> G[Structure Tool]
    C --> D
    
    D --> H[Performance Monitor]
    D --> I[Error Manager]
    
    E --> J[Brainstorming Tool]
    E --> B
    
    F --> K[Notes Tool]
    F --> L[Database Layer]
    
    G --> A
    G --> C
    
    J --> E
    J --> F
    
    K --> D
    K --> B
    
    L --> M[Backup Service]
    L --> N[Search Service]
    
    M --> O[File System]
    N --> P[FTS5 Index]
```

## Window Management Architecture

```mermaid
classDiagram
    class WindowManager {
        +state_manager: WindowStateManager
        +enhanced_manager: EnhancedWindowManager
        +persistence_config: WindowPersistenceConfig
        +initialize() Result
        +handle_window_event(event) WindowEventResult
        +save_layout(name) Result
        +load_layout(name) Result
    }
    
    class WindowStateManager {
        +windows: HashMap<string, WindowState>
        +active_layout: Option<string>
        +capture_window_state() HashMap<string, WindowState>
        +apply_window_state(state) Result
        +save_current_layout() Result
        +load_last_layout() Result
    }
    
    class WindowState {
        +id: string
        +title: string
        +position: WindowPosition
        +size: WindowSize
        +visibility: WindowVisibility
        +z_index: i32
        +metadata: WindowMetadata
    }
    
    class WindowPosition {
        +x: i32
        +y: i32
    }
    
    class WindowSize {
        +width: u32
        +height: u32
    }
    
    class WindowVisibility {
        +visible: bool
        +minimized: bool
        +maximized: bool
    }
    
    WindowManager --> WindowStateManager
    WindowManager --> EnhancedWindowManager
    WindowStateManager --> WindowState
    WindowState --> WindowPosition
    WindowState --> WindowSize
    WindowState --> WindowVisibility
```

## Security Architecture Diagram

```mermaid
graph TB
    subgraph "Security Layers"
        A[Authentication Layer]
        B[Authorization Layer]
        C[Data Protection]
        D[Audit Logging]
    end
    
    subgraph "Components"
        A1[User Authentication]
        A2[API Key Management]
        A3[Session Management]
        
        B1[Role-Based Access]
        B2[Permission Checks]
        B3[Resource Authorization]
        
        C1[Data Encryption]
        C2[Integrity Verification]
        C3[Secure Storage]
        
        D1[Activity Logging]
        D2[Security Events]
        D3[Compliance Reporting]
    end
    
    subgraph "Infrastructure"
        E[Encrypted Key Storage]
        F[SHA-256 Checksums]
        G[Audit Trail]
        H[Rate Limiting]
    end
    
    A --> A1
    A --> A2
    A --> A3
    
    B --> B1
    B --> B2
    B --> B3
    
    C --> C1
    C --> C2
    C --> C3
    
    D --> D1
    D --> D2
    D --> D3
    
    A2 --> E
    C2 --> F
    D1 --> G
    B2 --> H
```

## Component Interaction Matrix

| Component | Hierarchy | Codex | Plot | Research | Analysis | Structure | Brainstorming | Notes |
|-----------|-----------|-------|------|----------|----------|-----------|---------------|-------|
| **Database** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **AI Services** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Search** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Backup** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Performance** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **Error Handling** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

## Deployment Architecture

```mermaid
graph TB
    subgraph "Development Environment"
        A1[Source Code]
        A2[Local Database]
        A3[Development Server]
    end
    
    subgraph "Build Pipeline"
        B1[Cargo Build]
        B2[Static Analysis]
        B3[Unit Tests]
        B4[Integration Tests]
        B5[Performance Tests]
    end
    
    subgraph "Production Environment"
        C1[Compiled Binary]
        C2[SQLite Database]
        C3[Configuration Files]
        C4[User Data Directory]
    end
    
    subgraph "External Services"
        D1[AI Provider APIs]
        D2[Update Server]
        D3[Analytics Service]
    end
    
    A1 --> B1
    A2 --> B3
    A3 --> B4
    
    B1 --> B2
    B2 --> B3
    B3 --> B4
    B4 --> B5
    B5 --> C1
    
    C1 --> C2
    C1 --> C3
    C1 --> C4
    
    C1 --> D1
    C1 --> D2
    C1 --> D3
```

These architecture diagrams provide a comprehensive visual representation of the Herding Cats Rust application's structure, showing the relationships between components, data flow patterns, and integration points across the entire system.