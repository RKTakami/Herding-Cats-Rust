//! Consistent API Interfaces Between Tools and Services
//!
//! Defines unified contracts for tool interactions, service communication,
//! and cross-tool integration with proper error handling and validation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

// Database types for error handling
use crate as hc_lib;
use hc_lib::error::{DatabaseError, DatabaseResult};

// Use sqlx types directly to avoid import issues

/// Base trait for all tool services
/// Provides common functionality and lifecycle management
#[async_trait::async_trait]
pub trait ToolService: Send + Sync {
    /// Get service name
    fn service_name(&self) -> &'static str;

    /// Get service version
    fn service_version(&self) -> &'static str;

    /// Initialize the service
    async fn initialize(&mut self) -> DatabaseResult<()>;

    /// Check if service is healthy
    async fn health_check(&self) -> DatabaseResult<bool>;

    /// Get service capabilities
    fn capabilities(&self) -> Vec<ToolCapability>;

    /// Graceful shutdown
    async fn shutdown(&mut self) -> DatabaseResult<()>;
}

/// Types of tool capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolCapability {
    /// Can create new items
    Create,
    /// Can read/view items
    Read,
    /// Can update existing items
    Update,
    /// Can delete items
    Delete,
    /// Can search items
    Search,
    /// Can export data
    Export,
    /// Can import data
    Import,
    /// Supports drag and drop
    DragDrop,
    /// Supports real-time collaboration
    Collaboration,
    /// AI-powered features
    AIFeatures,
    /// Custom capability with name
    Custom(String),
}

/// Tool lifecycle events
#[derive(Debug, Clone, Serialize)]
pub enum ToolLifecycleEvent {
    /// Tool was initialized
    Initialized {
        tool_id: String,
        timestamp: i64, // Use Unix timestamp to avoid Instant serialization issues
        version: String,
    },
    /// Tool was registered
    Registered { tool_id: String, tool_type: String },
    /// Tool encountered an error
    Error {
        tool_id: String,
        error: String,
        timestamp: i64, // Use Unix timestamp to avoid Instant serialization issues
        severity: ErrorSeverity,
    },
    /// Tool state changed
    StateChanged {
        tool_id: String,
        old_state: ToolState,
        new_state: ToolState,
        timestamp: i64, // Use Unix timestamp to avoid Instant serialization issues
    },
    /// Tool data was modified
    DataModified {
        tool_id: String,
        operation: DataOperation,
        entity_type: String,
        entity_id: Option<String>,
        timestamp: i64, // Use Unix timestamp to avoid Instant serialization issues
    },
    /// Tool was deactivated
    Deactivated {
        tool_id: String,
        reason: DeactivationReason,
        timestamp: i64, // Use Unix timestamp to avoid Instant serialization issues
    },
    /// Tool was reinitialized
    Reinitialized { tool_id: String },
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ErrorSeverity {
    /// Informational message
    Info,
    /// Warning that doesn't affect functionality
    Warning,
    /// Error that affects some functionality
    Error,
    /// Critical error that affects core functionality
    Critical,
    /// Fatal error requiring tool restart
    Fatal,
}

/// Tool states
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ToolState {
    /// Tool is being initialized
    Initializing,
    /// Tool is ready for use
    Ready,
    /// Tool is currently in use
    Active,
    /// Tool is paused/suspended
    Inactive,
    /// Tool encountered an error
    Error,
    /// Tool is shutting down
    ShuttingDown,
}

/// Data operations that can trigger events
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DataOperation {
    Created,
    Read,
    Updated,
    Deleted,
    Imported,
    Exported,
}

/// Reasons for tool deactivation
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DeactivationReason {
    /// User manually closed the tool
    UserClosed,
    /// Tool encountered a critical error
    CriticalError,
    /// Application is shutting down
    ApplicationShutdown,
    /// Tool was updated/reloaded
    UpdateReload,
    /// Resource constraints
    ResourceConstraints,
}

/// Cross-tool communication message
#[derive(Debug, Clone, Serialize)]
pub struct ToolMessage {
    /// Unique message ID
    pub id: Uuid,
    /// Source tool ID
    pub source_tool: String,
    /// Target tool ID (None for broadcast)
    pub target_tool: Option<String>,
    /// Message type
    pub message_type: MessageType,
    /// Message payload
    pub payload: serde_json::Value,
    /// Timestamp (Unix timestamp to avoid Instant serialization issues)
    pub timestamp: i64,
    /// Priority level
    pub priority: MessagePriority,
}

/// Types of messages between tools
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum MessageType {
    /// Request data from another tool
    DataRequest {
        data_type: String,
        filters: Option<serde_json::Value>,
    },
    /// Send data to another tool
    DataResponse {
        data_type: String,
        data: serde_json::Value,
    },
    /// Notify of data changes
    DataChanged {
        entity_type: String,
        entity_id: String,
        change_type: DataOperation,
    },
    /// Request tool action
    ActionRequest {
        action: String,
        parameters: serde_json::Value,
    },
    /// Response to action request
    ActionResponse {
        action: String,
        success: bool,
        result: Option<serde_json::Value>,
    },
    /// Error notification
    Error {
        error: String,
        context: Option<serde_json::Value>,
    },
    /// Tool availability notification
    AvailabilityChanged { available: bool },
    /// Custom message type
    Custom { type_name: String },
}

/// Message priority levels
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum MessagePriority {
    /// Low priority, can be processed asynchronously
    Low,
    /// Normal priority
    Normal,
    /// High priority, should be processed quickly
    High,
    /// Critical priority, must be processed immediately
    Critical,
}

/// Tool event bus for cross-tool communication
#[derive(Debug)]
pub struct ToolEventBus {
    /// Broadcast channel for tool lifecycle events
    lifecycle_sender: broadcast::Sender<ToolLifecycleEvent>,
    /// Broadcast channel for tool messages
    message_sender: broadcast::Sender<ToolMessage>,
    /// Subscribers to specific tool events
    tool_subscribers: Arc<RwLock<HashMap<String, Vec<broadcast::Sender<ToolMessage>>>>>,
    /// Message processing timeout
    message_timeout: Duration,
}

impl ToolEventBus {
    /// Create a new tool event bus
    pub fn new() -> Self {
        let (lifecycle_sender, _) = broadcast::channel(1000);
        let (message_sender, _) = broadcast::channel(1000);

        Self {
            lifecycle_sender,
            message_sender,
            tool_subscribers: Arc::new(RwLock::new(HashMap::new())),
            message_timeout: Duration::from_secs(30),
        }
    }

    /// Subscribe to tool lifecycle events
    pub fn subscribe_lifecycle(&self) -> broadcast::Receiver<ToolLifecycleEvent> {
        self.lifecycle_sender.subscribe()
    }

    /// Subscribe to messages for a specific tool
    pub async fn subscribe_to_tool(&self, tool_id: &str) -> broadcast::Receiver<ToolMessage> {
        let mut subscribers = self.tool_subscribers.write().await;

        let receiver = subscribers
            .entry(tool_id.to_string())
            .or_insert_with(Vec::new)
            .iter()
            .find_map(|s| {
                // broadcast::Receiver doesn't have an ok() method, just use the receiver directly
                Some(s.subscribe())
            })
            .unwrap_or_else(|| {
                // Create new channel if none available
                let (new_sender, new_receiver) = broadcast::channel(100);
                subscribers.get_mut(tool_id).unwrap().push(new_sender);
                new_receiver
            });

        receiver
    }

    /// Broadcast a tool lifecycle event
    pub fn broadcast_lifecycle(&self, event: ToolLifecycleEvent) {
        let _ = self.lifecycle_sender.send(event);
    }

    /// Send a message to a specific tool or broadcast to all
    pub async fn send_message(&self, message: ToolMessage) -> DatabaseResult<()> {
        if let Some(target_tool) = &message.target_tool {
            // Send to specific tool
            self.send_to_tool(target_tool, message.clone()).await
        } else {
            // Broadcast to all tools
            let _ = self.message_sender.send(message);
            Ok(())
        }
    }

    async fn send_to_tool(&self, tool_id: &str, message: ToolMessage) -> DatabaseResult<()> {
        let subscribers = self.tool_subscribers.read().await;
        let tool_channels = subscribers
            .get(tool_id)
            .ok_or_else(|| DatabaseError::Service(format!("Tool '{}' not found", tool_id)))?;

        if tool_channels.is_empty() {
            return Err(DatabaseError::Service(format!(
                "No active channels for tool '{}'",
                tool_id
            )));
        }

        // Send to first available channel
        for channel in tool_channels {
            if channel.send(message.clone()).is_ok() {
                return Ok(());
            }
        }

        Err(DatabaseError::Service(format!(
            "Unable to send message to tool '{}'",
            tool_id
        )))
    }

    /// Get message timeout duration
    pub fn message_timeout(&self) -> Duration {
        self.message_timeout
    }

    /// Set message timeout duration
    pub fn set_message_timeout(&mut self, timeout: Duration) {
        self.message_timeout = timeout;
    }
}

/// Tool configuration with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfiguration {
    /// Tool ID
    pub tool_id: String,
    /// Tool display name
    pub display_name: String,
    /// Tool version
    pub version: String,
    /// Enabled features
    pub features: Vec<String>,
    /// Custom settings
    pub settings: HashMap<String, serde_json::Value>,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Dependencies on other tools
    pub dependencies: Vec<String>,
}

/// Resource limits for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in MB
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: Option<f64>,
    /// Maximum database connections
    pub max_db_connections: Option<u32>,
    /// Maximum concurrent operations
    pub max_concurrent_operations: Option<usize>,
    /// Timeout for operations in seconds
    pub operation_timeout_seconds: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: Some(512),
            max_cpu_percent: Some(50.0),
            max_db_connections: Some(5),
            max_concurrent_operations: Some(10),
            operation_timeout_seconds: Some(30),
        }
    }
}

impl ToolConfiguration {
    /// Validate tool configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.tool_id.is_empty() {
            return Err("Tool ID cannot be empty".to_string());
        }

        if self.display_name.is_empty() {
            return Err("Display name cannot be empty".to_string());
        }

        if self.version.is_empty() {
            return Err("Version cannot be empty".to_string());
        }

        // Validate resource limits
        if let Some(memory) = self.resource_limits.max_memory_mb {
            if memory == 0 {
                return Err("Max memory must be greater than 0".to_string());
            }
        }

        if let Some(cpu) = self.resource_limits.max_cpu_percent {
            if cpu <= 0.0 || cpu > 100.0 {
                return Err("CPU percentage must be between 0 and 100".to_string());
            }
        }

        Ok(())
    }

    /// Check if tool can be initialized with current resources
    pub fn check_resource_compatibility(
        &self,
        available_resources: &ResourceAvailability,
    ) -> Result<(), String> {
        if let Some(max_memory) = self.resource_limits.max_memory_mb {
            if max_memory > available_resources.available_memory_mb {
                return Err(format!(
                    "Tool requires {}MB memory but only {}MB available",
                    max_memory, available_resources.available_memory_mb
                ));
            }
        }

        if let Some(max_connections) = self.resource_limits.max_db_connections {
            if max_connections > available_resources.available_db_connections {
                return Err(format!(
                    "Tool requires {} database connections but only {} available",
                    max_connections, available_resources.available_db_connections
                ));
            }
        }

        Ok(())
    }
}

/// Available system resources
#[derive(Debug, Clone)]
pub struct ResourceAvailability {
    /// Available memory in MB
    pub available_memory_mb: u64,
    /// Available database connections
    pub available_db_connections: u32,
    /// Available CPU percentage
    pub available_cpu_percent: f64,
    /// Total concurrent operation slots
    pub total_operation_slots: usize,
    /// Used operation slots
    pub used_operation_slots: usize,
}

impl ResourceAvailability {
    /// Get available operation slots
    pub fn available_operation_slots(&self) -> usize {
        self.total_operation_slots
            .saturating_sub(self.used_operation_slots)
    }

    /// Check if enough resources are available
    pub fn has_sufficient_resources(&self, limits: &ResourceLimits) -> bool {
        if let Some(max_memory) = limits.max_memory_mb {
            if max_memory > self.available_memory_mb {
                return false;
            }
        }

        if let Some(max_connections) = limits.max_db_connections {
            if max_connections > self.available_db_connections {
                return false;
            }
        }

        if let Some(max_operations) = limits.max_concurrent_operations {
            if max_operations > self.available_operation_slots() {
                return false;
            }
        }

        true
    }
}

/// API contract for tool validation and verification
#[derive(Debug)]
pub struct ToolApiContract {
    /// Event bus for cross-tool communication
    event_bus: Arc<ToolEventBus>,
    /// Registered tool configurations
    tool_configs: Arc<RwLock<HashMap<String, ToolConfiguration>>>,
    /// Available system resources
    available_resources: Arc<RwLock<ResourceAvailability>>,
}

impl ToolApiContract {
    /// Register a tool with the API contract
    pub async fn register_tool(&self, tool_id: String) -> Result<(), String> {
        let config = ToolConfiguration {
            tool_id: tool_id.clone(),
            display_name: tool_id.clone(),
            version: "1.0.0".to_string(),
            features: vec![],
            settings: HashMap::new(),
            resource_limits: ResourceLimits::default(),
            dependencies: vec![],
        };

        self.register_tool_config(config)
            .await
            .map_err(|e| format!("Failed to register tool: {}", e))
    }

    /// Broadcast a lifecycle event
    pub async fn broadcast_lifecycle(&self, event: ToolLifecycleEvent) -> Result<(), String> {
        self.event_bus.broadcast_lifecycle(event);
        Ok(())
    }

    /// Get all registered tools
    pub async fn get_registered_tools(&self) -> Vec<String> {
        let configs = self.tool_configs.read().await;
        configs.keys().cloned().collect()
    }
}

impl ToolApiContract {
    /// Create a new tool API contract
    pub fn new() -> Self {
        Self {
            event_bus: Arc::new(ToolEventBus::new()),
            tool_configs: Arc::new(RwLock::new(HashMap::new())),
            available_resources: Arc::new(RwLock::new(ResourceAvailability {
                available_memory_mb: 2048,
                available_db_connections: 20,
                available_cpu_percent: 80.0,
                total_operation_slots: 50,
                used_operation_slots: 0,
            })),
        }
    }

    /// Register a tool configuration
    pub async fn register_tool_config(&self, config: ToolConfiguration) -> DatabaseResult<()> {
        // Validate configuration
        config
            .validate()
            .map_err(|e| DatabaseError::Service(format!("Invalid tool configuration: {}", e)))?;

        // Check resource compatibility
        {
            let resources = self.available_resources.read().await;
            config
                .check_resource_compatibility(&resources)
                .map_err(|e| {
                    DatabaseError::Service(format!("Resource validation failed: {}", e))
                })?;
        }

        // Register the configuration
        let mut configs = self.tool_configs.write().await;
        configs.insert(config.tool_id.clone(), config);

        // Notify about new tool registration
        self.event_bus
            .broadcast_lifecycle(ToolLifecycleEvent::Initialized {
                tool_id: "api_contract".to_string(),
                timestamp: Instant::now().elapsed().as_secs() as i64,
                version: "1.0.0".to_string(),
            });

        Ok(())
    }

    /// Get tool configuration by ID
    pub async fn get_tool_config(&self, tool_id: &str) -> Option<ToolConfiguration> {
        let configs = self.tool_configs.read().await;
        configs.get(tool_id).cloned()
    }

    /// Get all registered tool IDs
    pub async fn get_all_tool_ids(&self) -> Vec<String> {
        let configs = self.tool_configs.read().await;
        configs.keys().cloned().collect()
    }

    /// Update available resources
    pub async fn update_resources(&self, resources: ResourceAvailability) {
        *self.available_resources.write().await = resources;
    }

    /// Get event bus reference
    pub fn event_bus(&self) -> Arc<ToolEventBus> {
        self.event_bus.clone()
    }

    /// Check if tool can be activated
    pub async fn can_activate_tool(&self, tool_id: &str) -> Result<bool, String> {
        let config = self
            .get_tool_config(tool_id)
            .await
            .ok_or_else(|| format!("Tool '{}' not registered", tool_id))?;

        let resources = self.available_resources.read().await;
        config
            .check_resource_compatibility(&resources)
            .map_err(|e| format!("Resource check failed: {}", e))?;

        Ok(true)
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_API_CONTRACT: ToolApiContract = ToolApiContract::new();
}

/// Get reference to global API contract
pub fn get_api_contract() -> &'static ToolApiContract {
    &GLOBAL_API_CONTRACT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_event_bus() {
        let bus = ToolEventBus::new();

        // Subscribe to lifecycle events
        let mut lifecycle_subscriber = bus.subscribe_lifecycle();

        // Broadcast event
        let event = ToolLifecycleEvent::Initialized {
            tool_id: "test_tool".to_string(),
            timestamp: Instant::now().elapsed().as_secs() as i64,
            version: "1.0.0".to_string(),
        };

        bus.broadcast_lifecycle(event.clone());

        // Check if event was received (with timeout)
        let received_event =
            tokio::time::timeout(Duration::from_millis(100), lifecycle_subscriber.recv()).await;

        assert!(received_event.is_ok());
    }

    #[tokio::test]
    async fn test_tool_configuration_validation() {
        let config = ToolConfiguration {
            tool_id: "test_tool".to_string(),
            display_name: "Test Tool".to_string(),
            version: "1.0.0".to_string(),
            features: vec!["test".to_string()],
            settings: HashMap::new(),
            resource_limits: ResourceLimits::default(),
            dependencies: vec![],
        };

        assert!(config.validate().is_ok());
    }

    #[tokio::test]
    async fn test_invalid_tool_configuration() {
        let config = ToolConfiguration {
            tool_id: "".to_string(),
            display_name: "".to_string(),
            version: "".to_string(),
            features: vec![],
            settings: HashMap::new(),
            resource_limits: ResourceLimits::default(),
            dependencies: vec![],
        };

        assert!(config.validate().is_err());
    }

    #[tokio::test]
    async fn test_api_contract_registration() {
        let contract = ToolApiContract::new();

        let config = ToolConfiguration {
            tool_id: "test_tool".to_string(),
            display_name: "Test Tool".to_string(),
            version: "1.0.0".to_string(),
            features: vec!["test".to_string()],
            settings: HashMap::new(),
            resource_limits: ResourceLimits::default(),
            dependencies: vec![],
        };

        let result = contract.register_tool_config(config).await;
        assert!(result.is_ok());

        let tool_ids = contract.get_all_tool_ids().await;
        assert!(tool_ids.contains(&"test_tool".to_string()));
    }
}
