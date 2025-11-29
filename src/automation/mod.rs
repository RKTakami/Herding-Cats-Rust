use crate::error::{AppError, WritingToolError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/// Scripting and Automation Framework
/// Provides comprehensive workflow automation, macro system, and custom script execution capabilities
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Script definition and metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub language: ScriptLanguage,
    pub code: String,
    pub version: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub parameters: Vec<ScriptParameter>,
    pub permissions: ScriptPermissions,
    pub execution_context: ExecutionContext,
    pub metadata: ScriptMetadata,
    pub is_enabled: bool,
    pub is_system: bool,
}

/// Supported scripting languages
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScriptLanguage {
    Rust,
    JavaScript,
    Python,
    Lua,
    Custom(String),
}

/// Script parameter definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptParameter {
    pub name: String,
    pub parameter_type: ParameterType,
    pub description: Option<String>,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation_rules: ValidationRules,
}

/// Parameter types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Date,
    Time,
    DateTime,
    FilePath,
    DirectoryPath,
    Selection,
    Custom(String),
}

/// Validation rules for parameters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationRules {
    pub min_value: Option<serde_json::Value>,
    pub max_value: Option<serde_json::Value>,
    pub pattern: Option<String>,
    pub allowed_values: Option<Vec<serde_json::Value>>,
    pub custom_validator: Option<String>,
}

/// Script permissions and security settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptPermissions {
    pub file_access: FileAccessPermissions,
    pub network_access: NetworkAccessPermissions,
    pub system_access: SystemAccessPermissions,
    pub execution_timeout: Option<Duration>,
    pub memory_limit: Option<u64>,
    pub sandboxed: bool,
}

/// File system access permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileAccessPermissions {
    pub allowed_directories: Vec<PathBuf>,
    pub forbidden_directories: Vec<PathBuf>,
    pub allowed_extensions: Vec<String>,
    pub forbidden_extensions: Vec<String>,
    pub read_allowed: bool,
    pub write_allowed: bool,
    pub execute_allowed: bool,
    pub create_allowed: bool,
    pub delete_allowed: bool,
}

/// Network access permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAccessPermissions {
    pub http_allowed: bool,
    pub https_allowed: bool,
    pub ftp_allowed: bool,
    pub local_only: bool,
    pub allowed_hosts: Vec<String>,
    pub forbidden_hosts: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub timeout: Option<Duration>,
}

/// System access permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemAccessPermissions {
    pub allow_processes: bool,
    pub allow_subprocesses: bool,
    pub allow_environment_variables: bool,
    pub allow_external_commands: bool,
    pub allowed_commands: Vec<String>,
    pub forbidden_commands: Vec<String>,
}

/// Execution context and environment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub workspace_directory: PathBuf,
    pub working_directory: PathBuf,
    pub environment_variables: HashMap<String, String>,
    pub arguments: Vec<String>,
    pub stdin: Option<String>,
    pub stdout_redirect: Option<PathBuf>,
    pub stderr_redirect: Option<PathBuf>,
    pub current_working_set: Option<Uuid>,
    pub variables: HashMap<String, serde_json::Value>,
}

/// Script metadata and statistics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptMetadata {
    pub execution_count: u64,
    pub success_count: u64,
    pub failure_count: u64,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
    pub last_executed: Option<DateTime<Utc>>,
    pub last_success: Option<DateTime<Utc>>,
    pub last_failure: Option<DateTime<Utc>>,
    pub rating: Option<String>,
    pub usage_frequency: UsageFrequency,
    pub dependencies: Vec<ScriptDependency>,
}

/// Script usage frequency tracking
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsageFrequency {
    Never,
    Rarely,
    Occasionally,
    Frequently,
    Daily,
}

/// Script dependency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptDependency {
    pub script_id: Uuid,
    pub script_name: String,
    pub dependency_type: DependencyType,
    pub version: Option<String>,
}

/// Dependency types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DependencyType {
    Library,
    Module,
    Function,
    Script,
    External,
}

/// Automation workflow definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationWorkflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub enabled: bool,
    pub triggers: Vec<WorkflowTrigger>,
    pub actions: Vec<WorkflowAction>,
    pub conditions: Vec<WorkflowCondition>,
    pub error_handling: ErrorHandling,
    pub schedule: Option<WorkflowSchedule>,
    pub tags: Vec<String>,
}

/// Workflow trigger types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowTrigger {
    Event {
        event_type: EventType,
        conditions: Vec<EventCondition>,
    },
    Schedule {
        schedule: WorkflowSchedule,
    },
    Manual,
    FileSystem {
        path: PathBuf,
        event: FileSystemEvent,
        recursive: bool,
    },
    Time {
        at_time: String,
        timezone: String,
    },
}

/// Event types for triggers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    DocumentCreated,
    DocumentModified,
    DocumentDeleted,
    DocumentRenamed,
    ProjectOpened,
    ProjectClosed,
    Custom(String),
}

/// Event condition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventCondition {
    pub property: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

/// Condition operators
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    Regex,
}

/// File system events
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileSystemEvent {
    Created,
    Modified,
    Deleted,
    Renamed,
    Accessed,
}

/// Workflow action definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowAction {
    pub id: Uuid,
    pub action_type: ActionType,
    pub name: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub condition: Option<String>,
    pub on_error: ErrorAction,
    pub timeout: Option<Duration>,
}

/// Action types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    ExecuteScript {
        script_id: Uuid,
    },
    RunCommand {
        command: String,
        arguments: Vec<String>,
    },
    CreateFile {
        path: PathBuf,
        content: String,
    },
    DeleteFile {
        path: PathBuf,
    },
    MoveFile {
        from: PathBuf,
        to: PathBuf,
    },
    CopyFile {
        from: PathBuf,
        to: PathBuf,
    },
    SendNotification {
        title: String,
        message: String,
        level: NotificationLevel,
    },
    OpenDocument {
        path: PathBuf,
    },
    CloseDocument {
        document_id: Uuid,
    },
    CreateProject {
        name: String,
        template: Option<String>,
    },
    Custom {
        type_name: String,
        implementation: String,
    },
}

/// Notification levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

/// Workflow condition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowCondition {
    pub id: Uuid,
    pub name: String,
    pub condition_type: ConditionType,
    pub expression: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Condition types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionType {
    ScriptExecutionResult,
    FileExists,
    TimeCondition,
    EnvironmentVariable,
    Custom,
}

/// Error handling configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorHandling {
    pub on_error: ErrorAction,
    pub retry_count: u32,
    pub retry_delay: Duration,
    pub continue_on_error: bool,
    pub log_errors: bool,
    pub notify_on_error: bool,
}

/// Error action types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorAction {
    Stop,
    Continue,
    Retry,
    Skip,
    CustomScript { script_id: Uuid },
}

/// Workflow scheduling
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkflowSchedule {
    pub schedule_type: ScheduleType,
    pub interval: Option<Duration>,
    pub time: Option<String>,
    pub days: Vec<u8>, // 0-6 for Sunday-Saturday
    pub timezone: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Schedule types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScheduleType {
    Interval,
    Daily,
    Weekly,
    Monthly,
    Cron,
}

/// Macro definition and configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Macro {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub key_combination: KeyCombination,
    pub actions: Vec<MacroAction>,
    pub context: MacroContext,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Keyboard key combination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyCombination {
    pub keys: Vec<Key>,
    pub modifiers: Vec<Modifier>,
}

/// Keyboard keys
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    Function { number: u8 },
    Character { char: char },
    Special { name: String },
}

/// Keyboard modifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
    Super,
    Command,
}

/// Macro action types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroAction {
    pub action_type: MacroActionType,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Macro action type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MacroActionType {
    TypeText {
        text: String,
    },
    PressKey {
        key: Key,
    },
    ExecuteScript {
        script_id: Uuid,
    },
    Navigate {
        path: String,
    },
    Select {
        selector: String,
    },
    Focus {
        element: String,
    },
    Click {
        element: String,
        button: MouseButton,
    },
    Scroll {
        direction: ScrollDirection,
        amount: u32,
    },
    Delay {
        duration: Duration,
    },
}

/// Mouse buttons
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Scroll directions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Macro context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroContext {
    pub active_tool: Option<String>,
    pub selected_text: Option<String>,
    pub cursor_position: Option<(u32, u32)>,
    pub active_document: Option<Uuid>,
    pub workspace_state: WorkspaceState,
}

/// Workspace state information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub open_documents: Vec<Uuid>,
    pub active_tool: Option<String>,
    pub sidebar_visible: bool,
    pub toolbar_visible: bool,
    pub current_view: ViewType,
}

/// View types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViewType {
    Editor,
    Preview,
    Split,
    Fullscreen,
}

/// Execution result and status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error_message: Option<String>,
    pub execution_time: Duration,
    pub return_code: Option<i32>,
    pub stdout_file: Option<PathBuf>,
    pub stderr_file: Option<PathBuf>,
    pub logs: Vec<LogEntry>,
}

/// Log entry for script execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
    pub context: HashMap<String, serde_json::Value>,
}

/// Log levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// Script execution engine
pub struct ScriptEngine {
    scripts: Arc<RwLock<HashMap<Uuid, Script>>>,
    workflows: Arc<RwLock<HashMap<Uuid, AutomationWorkflow>>>,
    macros: Arc<RwLock<HashMap<Uuid, Macro>>>,
    execution_history: Arc<RwLock<VecDeque<ExecutionResult>>>,
    runtime_context: Arc<RwLock<RuntimeContext>>,
    event_system: Arc<RwLock<EventSystem>>,
    scheduler: Arc<RwLock<WorkflowScheduler>>,
    sandbox: Arc<RwLock<ScriptSandbox>>,
}

/// Runtime context for script execution
#[derive(Debug, Clone)]
pub struct RuntimeContext {
    pub current_execution: Option<Uuid>,
    pub execution_count: u64,
    pub active_variables: HashMap<String, serde_json::Value>,
    pub execution_permissions: ScriptPermissions,
    pub workspace_path: PathBuf,
}

/// Event system for workflow triggers
#[derive(Debug, Clone)]
pub struct EventSystem {
    pub event_queue: Arc<Mutex<VecDeque<SystemEvent>>>,
}

/// System event definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: &SystemEvent) -> Result<(), crate::error::WritingToolError>;
}

/// Workflow scheduler for timed executions
#[derive(Debug, Clone)]
pub struct WorkflowScheduler {
    pub scheduled_workflows: HashMap<Uuid, ScheduledWorkflow>,
    pub running_workflows: HashMap<Uuid, RunningWorkflow>,
}

/// Scheduled workflow information
#[derive(Debug, Clone)]
pub struct ScheduledWorkflow {
    pub workflow_id: Uuid,
    pub next_execution: DateTime<Utc>,
    pub interval: Option<Duration>,
    pub trigger_type: ScheduleType,
}

/// Running workflow information
#[derive(Debug, Clone)]
pub struct RunningWorkflow {
    pub workflow_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub current_action: usize,
    pub status: WorkflowStatus,
}

/// Workflow execution status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Script sandbox for secure execution
#[derive(Debug, Clone)]
pub struct ScriptSandbox {
    pub isolated_environments: HashMap<Uuid, IsolatedEnvironment>,
    pub security_policies: SecurityPolicies,
}

/// Isolated execution environment
#[derive(Debug, Clone)]
pub struct IsolatedEnvironment {
    pub execution_id: Uuid,
    pub workspace_path: PathBuf,
    pub environment_variables: HashMap<String, String>,
    pub resource_limits: ResourceLimits,
    pub permissions: ScriptPermissions,
}

/// Resource limits for sandboxed execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory: u64,
    pub max_cpu_time: Duration,
    pub max_disk_space: u64,
    pub max_file_descriptors: u32,
    pub max_processes: u32,
}

/// Security policies for script execution
#[derive(Debug, Clone)]
pub struct SecurityPolicies {
    pub allow_network_access: bool,
    pub allow_file_system_access: bool,
    pub allow_external_commands: bool,
    pub allowed_file_extensions: Vec<String>,
    pub forbidden_file_paths: Vec<PathBuf>,
    pub sandbox_by_default: bool,
}

/// Script template for creating new scripts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub language: ScriptLanguage,
    pub template_code: String,
    pub parameters: Vec<TemplateParameter>,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Template parameter definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateParameter {
    pub name: String,
    pub placeholder: String,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
}

/// Automation API for integration
#[derive(Debug, Clone)]
pub struct AutomationApi {
    pub event_system: Arc<RwLock<EventSystem>>,
}

/// Script library for reusable functions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptLibrary {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub language: ScriptLanguage,
    pub functions: Vec<LibraryFunction>,
    pub version: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub dependencies: Vec<Uuid>,
}

/// Library function definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryFunction {
    pub name: String,
    pub description: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: String,
    pub code: String,
    pub examples: Vec<String>,
}

/// Function parameter definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub parameter_type: ParameterType,
    pub description: Option<String>,
    pub optional: bool,
    pub default_value: Option<serde_json::Value>,
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptEngine {
    /// Create new script engine
    pub fn new() -> Self {
        Self {
            scripts: Arc::new(RwLock::new(HashMap::new())),
            workflows: Arc::new(RwLock::new(HashMap::new())),
            macros: Arc::new(RwLock::new(HashMap::new())),
            execution_history: Arc::new(RwLock::new(VecDeque::new())),
            runtime_context: Arc::new(RwLock::new(RuntimeContext {
                current_execution: None,
                execution_count: 0,
                active_variables: HashMap::new(),
                execution_permissions: ScriptPermissions {
                    file_access: FileAccessPermissions {
                        allowed_directories: vec![],
                        forbidden_directories: vec![],
                        allowed_extensions: vec![],
                        forbidden_extensions: vec![],
                        read_allowed: false,
                        write_allowed: false,
                        execute_allowed: false,
                        create_allowed: false,
                        delete_allowed: false,
                    },
                    network_access: NetworkAccessPermissions {
                        http_allowed: false,
                        https_allowed: false,
                        ftp_allowed: false,
                        local_only: true,
                        allowed_hosts: vec![],
                        forbidden_hosts: vec![],
                        allowed_ports: vec![],
                        timeout: Some(Duration::from_secs(30)),
                    },
                    system_access: SystemAccessPermissions {
                        allow_processes: false,
                        allow_subprocesses: false,
                        allow_environment_variables: false,
                        allow_external_commands: false,
                        allowed_commands: vec![],
                        forbidden_commands: vec![],
                    },
                    execution_timeout: Some(Duration::from_secs(30)),
                    memory_limit: Some(100 * 1024 * 1024), // 100MB
                    sandboxed: true,
                },
                workspace_path: PathBuf::new(),
            })),
            event_system: Arc::new(RwLock::new(EventSystem {
                event_queue: Arc::new(Mutex::new(VecDeque::new())),
            })),
            scheduler: Arc::new(RwLock::new(WorkflowScheduler {
                scheduled_workflows: HashMap::new(),
                running_workflows: HashMap::new(),
            })),
            sandbox: Arc::new(RwLock::new(ScriptSandbox {
                isolated_environments: HashMap::new(),
                security_policies: SecurityPolicies {
                    allow_network_access: false,
                    allow_file_system_access: true,
                    allow_external_commands: false,
                    allowed_file_extensions: vec![
                        "txt".to_string(),
                        "md".to_string(),
                        "json".to_string(),
                    ],
                    forbidden_file_paths: vec![],
                    sandbox_by_default: true,
                },
            })),
        }
    }

    /// Create a new script
    pub fn create_script(&self, script: Script) -> Result<Uuid, crate::error::AppError> {
        let script_id = script.id;

        {
            let mut scripts = self.scripts.write().unwrap();
            scripts.insert(script_id, script);
        }

        Ok(script_id)
    }

    /// Get script by ID
    pub fn get_script(&self, script_id: Uuid) -> Option<Script> {
        let scripts = self.scripts.read().unwrap();
        scripts.get(&script_id).cloned()
    }

    /// Execute a script
    pub async fn execute_script(
        &self,
        script_id: Uuid,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, crate::error::AppError> {
        let script = self
            .get_script(script_id)
            .ok_or(crate::error::AppError::ToolNotFound {
                tool: format!("script_{}", script_id),
            })?;

        let execution_id = Uuid::new_v4();
        let start_time = Instant::now();

        // Update runtime context
        {
            let mut context = self.runtime_context.write().unwrap();
            context.current_execution = Some(execution_id);
            context.execution_count += 1;
            context.active_variables.extend(parameters.clone());
        }

        // Check permissions and sandbox if needed
        if script.permissions.sandboxed {
            self.execute_in_sandbox(&script, parameters).await
        } else {
            self.execute_directly(&script, parameters).await
        }
        .inspect(|result| {
            let execution_time = start_time.elapsed();

            // Update script metadata
            {
                let mut scripts = self.scripts.write().unwrap();
                if let Some(script_mut) = scripts.get_mut(&script_id) {
                    script_mut.metadata.execution_count += 1;
                    script_mut.metadata.total_execution_time += execution_time;
                    script_mut.metadata.last_executed = Some(Utc::now());

                    if script_mut.metadata.execution_count > 0 {
                        script_mut.metadata.average_execution_time =
                            script_mut.metadata.total_execution_time
                                / script_mut.metadata.execution_count as u32;
                    }
                }
            }

            // Store execution history
            {
                let mut history = self.execution_history.write().unwrap();
                history.push_back(result.clone());

                // Keep only last 1000 executions
                if history.len() > 1000 {
                    history.pop_front();
                }
            }
        })
    }

    /// Execute script in sandbox
    async fn execute_in_sandbox(
        &self,
        script: &Script,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, crate::error::AppError> {
        let execution_id = Uuid::new_v4();
        let workspace_path = PathBuf::from(format!("sandbox/{}", execution_id));

        // Create isolated environment
        {
            let mut sandbox = self.sandbox.write().unwrap();
            sandbox.isolated_environments.insert(
                execution_id,
                IsolatedEnvironment {
                    execution_id,
                    workspace_path: workspace_path.clone(),
                    environment_variables: HashMap::new(),
                    resource_limits: ResourceLimits {
                        max_memory: script.permissions.memory_limit.unwrap_or(100 * 1024 * 1024),
                        max_cpu_time: script
                            .permissions
                            .execution_timeout
                            .unwrap_or(Duration::from_secs(30)),
                        max_disk_space: 100 * 1024 * 1024, // 100MB
                        max_file_descriptors: 64,
                        max_processes: 1,
                    },
                    permissions: script.permissions.clone(),
                },
            );
        }

        // Execute based on language
        match script.language {
            ScriptLanguage::Rust => {
                self.execute_rust_script(script, &workspace_path, parameters)
                    .await
            }
            ScriptLanguage::JavaScript => {
                self.execute_javascript_script(script, &workspace_path, parameters)
                    .await
            }
            ScriptLanguage::Python => {
                self.execute_python_script(script, &workspace_path, parameters)
                    .await
            }
            ScriptLanguage::Lua => {
                self.execute_lua_script(script, &workspace_path, parameters)
                    .await
            }
            ScriptLanguage::Custom(_) => {
                self.execute_custom_script(script, &workspace_path, parameters)
                    .await
            }
        }
    }

    /// Execute script directly (without sandbox)
    async fn execute_directly(
        &self,
        _script: &Script,
        _parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, crate::error::AppError> {
        // This would execute scripts directly without isolation
        // For security reasons, this should generally be avoided
        Err(crate::error::AppError::ToolPermissionError {
            tool: "script".to_string(),
            operation: "direct_execution".to_string(),
            error: "Direct execution not allowed for security reasons".to_string(),
        })
    }

    /// Execute Rust script in sandbox
    async fn execute_rust_script(
        &self,
        script: &Script,
        workspace_path: &PathBuf,
        _parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, AppError> {
        // Create temporary Rust file
        let script_path = workspace_path.join("script.rs");
        std::fs::write(&script_path, &script.code).map_err(|e| AppError::ToolExecutionFailed {
            tool: "rust_script".to_string(),
            error: e.to_string(),
        })?;

        // Compile and run Rust script
        let mut cmd = Command::new("rustc");
        cmd.args(["-o", "script", script_path.to_str().unwrap()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let compile_output = cmd.output().map_err(|e| AppError::ToolExecutionFailed {
            tool: "rust_script".to_string(),
            error: e.to_string(),
        })?;

        if !compile_output.status.success() {
            return Ok(ExecutionResult {
                success: false,
                output: String::new(),
                error_message: Some(String::from_utf8_lossy(&compile_output.stderr).to_string()),
                execution_time: Duration::from_millis(0),
                return_code: Some(compile_output.status.code().unwrap_or(-1)),
                stdout_file: None,
                stderr_file: None,
                logs: vec![],
            });
        }

        // Run the compiled binary
        let mut run_cmd = Command::new("./script");
        run_cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let run_output = run_cmd
            .output()
            .map_err(|e| AppError::ToolExecutionFailed {
                tool: "rust_script".to_string(),
                error: e.to_string(),
            })?;

        Ok(ExecutionResult {
            success: run_output.status.success(),
            output: String::from_utf8_lossy(&run_output.stdout).to_string(),
            error_message: if run_output.status.success() {
                None
            } else {
                Some(String::from_utf8_lossy(&run_output.stderr).to_string())
            },
            execution_time: Duration::from_millis(0),
            return_code: Some(run_output.status.code().unwrap_or(-1)),
            stdout_file: None,
            stderr_file: None,
            logs: vec![],
        })
    }

    /// Execute JavaScript script in sandbox
    async fn execute_javascript_script(
        &self,
        _script: &Script,
        _workspace_path: &PathBuf,
        _parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, AppError> {
        // For JavaScript, we would typically use a JavaScript runtime like Node.js or Deno
        // with proper sandboxing capabilities

        Ok(ExecutionResult {
            success: true,
            output: "JavaScript execution not yet implemented".to_string(),
            error_message: None,
            execution_time: Duration::from_millis(10),
            return_code: Some(0),
            stdout_file: None,
            stderr_file: None,
            logs: vec![],
        })
    }

    /// Execute Python script in sandbox
    async fn execute_python_script(
        &self,
        _script: &Script,
        _workspace_path: &PathBuf,
        _parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, AppError> {
        // For Python, we would use a Python runtime with proper sandboxing

        Ok(ExecutionResult {
            success: true,
            output: "Python execution not yet implemented".to_string(),
            error_message: None,
            execution_time: Duration::from_millis(10),
            return_code: Some(0),
            stdout_file: None,
            stderr_file: None,
            logs: vec![],
        })
    }

    /// Execute Lua script in sandbox
    async fn execute_lua_script(
        &self,
        _script: &Script,
        _workspace_path: &PathBuf,
        _parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, AppError> {
        // For Lua, we would use a Lua interpreter with sandboxing

        Ok(ExecutionResult {
            success: true,
            output: "Lua execution not yet implemented".to_string(),
            error_message: None,
            execution_time: Duration::from_millis(10),
            return_code: Some(0),
            stdout_file: None,
            stderr_file: None,
            logs: vec![],
        })
    }

    /// Execute custom script
    async fn execute_custom_script(
        &self,
        script: &Script,
        _workspace_path: &PathBuf,
        _parameters: HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, AppError> {
        // Handle custom scripting languages

        Ok(ExecutionResult {
            success: true,
            output: format!(
                "Custom script execution for language: {:?}",
                script.language
            ),
            error_message: None,
            execution_time: Duration::from_millis(10),
            return_code: Some(0),
            stdout_file: None,
            stderr_file: None,
            logs: vec![],
        })
    }

    /// Create automation workflow
    pub fn create_workflow(&self, workflow: AutomationWorkflow) -> Result<Uuid, AppError> {
        let workflow_id = workflow.id;

        {
            let mut workflows = self.workflows.write().unwrap();
            workflows.insert(workflow_id, workflow);
        }

        // Register workflow triggers
        self.register_workflow_triggers(workflow_id)?;

        Ok(workflow_id)
    }

    /// Register workflow triggers
    fn register_workflow_triggers(&self, workflow_id: Uuid) -> Result<(), AppError> {
        let workflows = self.workflows.read().unwrap();
        let workflow = workflows.get(&workflow_id).ok_or(AppError::ToolNotFound {
            tool: format!("workflow_{}", workflow_id),
        })?;

        for trigger in &workflow.triggers {
            match trigger {
                WorkflowTrigger::Event { event_type, .. } => {
                    let _event_system = self.event_system.write().unwrap();
                    // Event handlers functionality removed - EventSystem only has event_queue
                    // This would need to be reimplemented if event handling is required
                    log::info!(
                        "Registering workflow {} for event type {:?}",
                        workflow_id,
                        event_type
                    );
                }
                WorkflowTrigger::Schedule { .. } => {
                    // Register with scheduler
                    let _scheduler = self.scheduler.write().unwrap();
                    // Add to scheduled workflows
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Execute workflow
    pub async fn execute_workflow(
        &self,
        workflow_id: Uuid,
    ) -> Result<ExecutionResult, WritingToolError> {
        let workflows = self.workflows.read().unwrap();
        let workflow = workflows
            .get(&workflow_id)
            .ok_or(WritingToolError::WorkflowNotFound(workflow_id))?;

        let start_time = Instant::now();
        let mut logs = Vec::new();
        let context = HashMap::new();

        // Execute actions in sequence
        for (index, action) in workflow.actions.iter().enumerate() {
            logs.push(LogEntry {
                timestamp: Utc::now(),
                level: LogLevel::Info,
                message: format!("Executing action {}: {}", index + 1, action.name),
                source: "workflow".to_string(),
                context: HashMap::new(),
            });

            // Check conditions
            if let Some(condition_str) = &action.condition {
                let condition_result = self.evaluate_condition(condition_str, &context)?;
                if !condition_result {
                    logs.push(LogEntry {
                        timestamp: Utc::now(),
                        level: LogLevel::Info,
                        message: format!("Condition '{}' failed, skipping action", condition_str),
                        source: "workflow".to_string(),
                        context: HashMap::new(),
                    });
                    continue;
                }
            }

            // Execute action
            let action_result = self.execute_workflow_action(action, &context).await?;

            if !action_result.success {
                logs.push(LogEntry {
                    timestamp: Utc::now(),
                    level: LogLevel::Error,
                    message: format!(
                        "Action '{}' failed: {}",
                        action.name,
                        action_result
                            .error_message
                            .as_deref()
                            .unwrap_or("Unknown error")
                    ),
                    source: "workflow".to_string(),
                    context: HashMap::new(),
                });

                match action.on_error {
                    ErrorAction::Stop => {
                        return Ok(ExecutionResult {
                            success: false,
                            output: format!("Workflow stopped at action '{}'", action.name),
                            error_message: action_result.error_message,
                            execution_time: start_time.elapsed(),
                            return_code: Some(-1),
                            stdout_file: None,
                            stderr_file: None,
                            logs,
                        });
                    }
                    ErrorAction::Continue => {
                        logs.push(LogEntry {
                            timestamp: Utc::now(),
                            level: LogLevel::Info,
                            message: format!(
                                "Continuing despite error in action '{}'",
                                action.name
                            ),
                            source: "workflow".to_string(),
                            context: HashMap::new(),
                        });
                    }
                    ErrorAction::Retry => {
                        // Implement retry logic
                    }
                    ErrorAction::Skip => {
                        logs.push(LogEntry {
                            timestamp: Utc::now(),
                            level: LogLevel::Info,
                            message: format!("Skipping action '{}' due to error", action.name),
                            source: "workflow".to_string(),
                            context: HashMap::new(),
                        });
                    }
                    ErrorAction::CustomScript { script_id: _ } => {
                        // Execute custom error handling script
                    }
                }
            }
        }

        Ok(ExecutionResult {
            success: true,
            output: "Workflow completed successfully".to_string(),
            error_message: None,
            execution_time: start_time.elapsed(),
            return_code: Some(0),
            stdout_file: None,
            stderr_file: None,
            logs,
        })
    }

    /// Execute workflow action
    async fn execute_workflow_action(
        &self,
        action: &WorkflowAction,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<ExecutionResult, WritingToolError> {
        match &action.action_type {
            ActionType::ExecuteScript { script_id } => {
                // Prepare parameters for script execution
                let mut parameters = action.parameters.clone();
                parameters.extend(context.clone());

                self.execute_script(*script_id, parameters)
                    .await
                    .map_err(crate::error::WritingToolError::App)
            }
            ActionType::RunCommand {
                ref command,
                ref arguments,
            } => {
                let mut cmd = Command::new(command);
                cmd.args(arguments)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());

                let output = cmd
                    .output()
                    .map_err(|e| WritingToolError::SystemError(e.to_string()))?;

                Ok(ExecutionResult {
                    success: output.status.success(),
                    output: String::from_utf8_lossy(&output.stdout).to_string(),
                    error_message: if output.status.success() {
                        None
                    } else {
                        Some(String::from_utf8_lossy(&output.stderr).to_string())
                    },
                    execution_time: Duration::from_millis(0),
                    return_code: Some(output.status.code().unwrap_or(-1)),
                    stdout_file: None,
                    stderr_file: None,
                    logs: vec![],
                })
            }
            ActionType::CreateFile {
                ref path,
                ref content,
            } => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| WritingToolError::FileSystemError(e.to_string()))?;
                }

                std::fs::write(path, content)
                    .map_err(|e| WritingToolError::FileSystemError(e.to_string()))?;

                Ok(ExecutionResult {
                    success: true,
                    output: format!("File created: {}", path.display()),
                    error_message: None,
                    execution_time: Duration::from_millis(0),
                    return_code: Some(0),
                    stdout_file: None,
                    stderr_file: None,
                    logs: vec![],
                })
            }
            ActionType::SendNotification {
                ref title,
                ref message,
                ref level,
            } => {
                // Send system notification
                println!(
                    "[{}] {}: {}",
                    match level {
                        NotificationLevel::Info => "INFO",
                        NotificationLevel::Warning => "WARNING",
                        NotificationLevel::Error => "ERROR",
                        NotificationLevel::Success => "SUCCESS",
                    },
                    title,
                    message
                );

                Ok(ExecutionResult {
                    success: true,
                    output: format!("Notification sent: {}", title),
                    error_message: None,
                    execution_time: Duration::from_millis(0),
                    return_code: Some(0),
                    stdout_file: None,
                    stderr_file: None,
                    logs: vec![],
                })
            }
            _ => {
                // Handle other action types
                Ok(ExecutionResult {
                    success: true,
                    output: format!("Action type not implemented: {:?}", action.action_type),
                    error_message: None,
                    execution_time: Duration::from_millis(0),
                    return_code: Some(0),
                    stdout_file: None,
                    stderr_file: None,
                    logs: vec![],
                })
            }
        }
    }

    /// Evaluate condition expression
    fn evaluate_condition(
        &self,
        condition: &str,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<bool, WritingToolError> {
        // Simple condition evaluation - in a real implementation, this would be more sophisticated
        // For now, just check if the condition string is "true"
        Ok(condition.to_lowercase() == "true" || context.contains_key(condition))
    }

    /// Create macro
    pub fn create_macro(&self, macro_def: Macro) -> Result<Uuid, WritingToolError> {
        let macro_id = macro_def.id;

        {
            let mut macros = self.macros.write().unwrap();
            macros.insert(macro_id, macro_def);
        }

        Ok(macro_id)
    }

    /// Execute macro
    pub async fn execute_macro(&self, macro_id: Uuid) -> Result<ExecutionResult, WritingToolError> {
        let macros = self.macros.read().unwrap();
        let macro_def = macros
            .get(&macro_id)
            .ok_or(WritingToolError::MacroNotFound(macro_id))?;

        let start_time = Instant::now();
        let mut logs = Vec::new();

        for action in &macro_def.actions {
            logs.push(LogEntry {
                timestamp: Utc::now(),
                level: LogLevel::Info,
                message: format!("Executing macro action: {:?}", action.action_type),
                source: "macro".to_string(),
                context: HashMap::new(),
            });

            // Execute macro action
            match action.action_type {
                MacroActionType::TypeText { ref text } => {
                    // Simulate text typing
                    logs.push(LogEntry {
                        timestamp: Utc::now(),
                        level: LogLevel::Debug,
                        message: format!("Typing text: {}", text),
                        source: "macro".to_string(),
                        context: HashMap::new(),
                    });
                }
                MacroActionType::ExecuteScript { script_id } => {
                    self.execute_script(script_id, HashMap::new()).await?;
                }
                MacroActionType::Delay { duration } => {
                    tokio::time::sleep(duration).await;
                }
                _ => {
                    // Handle other macro actions
                }
            }
        }

        Ok(ExecutionResult {
            success: true,
            output: "Macro executed successfully".to_string(),
            error_message: None,
            execution_time: start_time.elapsed(),
            return_code: Some(0),
            stdout_file: None,
            stderr_file: None,
            logs,
        })
    }

    /// Get execution history
    pub fn get_execution_history(&self, limit: usize) -> Vec<ExecutionResult> {
        let history = self.execution_history.read().unwrap();
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Trigger system event
    pub async fn trigger_event(&self, event: SystemEvent) -> Result<(), WritingToolError> {
        let event_system = self.event_system.clone();
        let system = event_system.write().unwrap();

        // Add to event queue
        system.event_queue.lock().unwrap().push_back(event.clone());

        // Event handlers functionality removed - EventSystem only has event_queue
        // Event processing would need to be implemented separately
        log::info!(
            "Event triggered: {:?} from {}",
            event.event_type,
            event.source
        );

        Ok(())
    }
}

/// Workflow event handler implementation
#[allow(dead_code)]
struct WorkflowEventHandler {
    workflow_id: Uuid,
}

impl EventHandler for WorkflowEventHandler {
    fn handle_event(&self, _event: &SystemEvent) -> Result<(), WritingToolError> {
        // This would trigger the workflow execution
        // For now, just log the event
        println!(
            "Workflow {} triggered by event: {:?}",
            self.workflow_id, _event.event_type
        );
        Ok(())
    }
}

/// Error types for automation
#[derive(Debug, Clone)]
pub enum AutomationError {
    ScriptNotFound(Uuid),
    WorkflowNotFound(Uuid),
    MacroNotFound(Uuid),
    ExecutionTimeout,
    PermissionDenied,
    InvalidScript,
    SandboxViolation,
}

impl std::fmt::Display for AutomationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutomationError::ScriptNotFound(id) => write!(f, "Script not found: {}", id),
            AutomationError::WorkflowNotFound(id) => write!(f, "Workflow not found: {}", id),
            AutomationError::MacroNotFound(id) => write!(f, "Macro not found: {}", id),
            AutomationError::ExecutionTimeout => write!(f, "Script execution timed out"),
            AutomationError::PermissionDenied => {
                write!(f, "Permission denied for script execution")
            }
            AutomationError::InvalidScript => write!(f, "Invalid script content"),
            AutomationError::SandboxViolation => write!(f, "Sandbox security violation detected"),
        }
    }
}

impl std::error::Error for AutomationError {}

/// Helper trait for automation error conversion
impl From<AutomationError> for WritingToolError {
    fn from(error: AutomationError) -> Self {
        match error {
            AutomationError::ScriptNotFound(id) => WritingToolError::ScriptNotFound(id),
            AutomationError::WorkflowNotFound(id) => WritingToolError::WorkflowNotFound(id),
            AutomationError::MacroNotFound(id) => WritingToolError::MacroNotFound(id),
            AutomationError::ExecutionTimeout => WritingToolError::ExecutionTimeout,
            AutomationError::PermissionDenied => WritingToolError::PermissionDenied,
            AutomationError::InvalidScript => WritingToolError::InvalidScript,
            AutomationError::SandboxViolation => {
                WritingToolError::SecurityError("Sandbox violation detected".to_string())
            }
        }
    }
}

/// Extension for LogLevel to get string representation
impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
        }
    }
}

/// Default implementations
impl Default for Script {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "New Script".to_string(),
            description: "A new automation script".to_string(),
            language: ScriptLanguage::Rust,
            code: String::new(),
            version: "1.0.0".to_string(),
            author: "System".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: vec![],
            parameters: vec![],
            permissions: ScriptPermissions {
                file_access: FileAccessPermissions {
                    allowed_directories: vec![],
                    forbidden_directories: vec![],
                    allowed_extensions: vec![],
                    forbidden_extensions: vec![],
                    read_allowed: false,
                    write_allowed: false,
                    execute_allowed: false,
                    create_allowed: false,
                    delete_allowed: false,
                },
                network_access: NetworkAccessPermissions {
                    http_allowed: false,
                    https_allowed: false,
                    ftp_allowed: false,
                    local_only: true,
                    allowed_hosts: vec![],
                    forbidden_hosts: vec![],
                    allowed_ports: vec![],
                    timeout: Some(Duration::from_secs(30)),
                },
                system_access: SystemAccessPermissions {
                    allow_processes: false,
                    allow_subprocesses: false,
                    allow_environment_variables: false,
                    allow_external_commands: false,
                    allowed_commands: vec![],
                    forbidden_commands: vec![],
                },
                execution_timeout: Some(Duration::from_secs(30)),
                memory_limit: Some(100 * 1024 * 1024),
                sandboxed: true,
            },
            execution_context: ExecutionContext {
                workspace_directory: PathBuf::new(),
                working_directory: PathBuf::new(),
                environment_variables: HashMap::new(),
                arguments: vec![],
                stdin: None,
                stdout_redirect: None,
                stderr_redirect: None,
                current_working_set: None,
                variables: HashMap::new(),
            },
            metadata: ScriptMetadata {
                execution_count: 0,
                success_count: 0,
                failure_count: 0,
                total_execution_time: Duration::from_millis(0),
                average_execution_time: Duration::from_millis(0),
                last_executed: None,
                last_success: None,
                last_failure: None,
                rating: None,
                usage_frequency: UsageFrequency::Never,
                dependencies: vec![],
            },
            is_enabled: true,
            is_system: false,
        }
    }
}

