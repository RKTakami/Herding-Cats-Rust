//! Safe Threading Patterns for Global State Management
//!
//! Provides thread-safe patterns for managing global state across UI tools
//! with proper async/await support and deadlock prevention.

use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, Mutex, RwLock};
use tracing::{debug, info, warn};

/// Global state container with thread-safe access
#[derive(Debug)]
pub struct GlobalStateContainer<T> {
    /// The actual state data
    data: Arc<RwLock<T>>,
    /// Broadcast channel for state change notifications
    change_sender: broadcast::Sender<StateChangeEvent>,
    /// Last update timestamp
    last_updated: Arc<Mutex<Instant>>,
    /// Update counter for change tracking
    update_counter: Arc<Mutex<u64>>,
}

/// Types of state change events
#[derive(Debug, Clone)]
pub enum StateChangeEvent {
    /// State was updated
    Updated {
        source: &'static str,
        timestamp: Instant,
    },
    /// State was reset/cleared
    Reset { source: &'static str },
    /// State was synchronized from external source
    Synced {
        source: &'static str,
        items_synced: usize,
    },
}

impl<T: Default> GlobalStateContainer<T> {
    /// Create a new global state container
    pub fn new() -> Self {
        let (change_sender, _) = broadcast::channel(100);
        Self {
            data: Arc::new(RwLock::new(T::default())),
            change_sender,
            last_updated: Arc::new(Mutex::new(Instant::now())),
            update_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Get read access to the state
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, T> {
        self.data.read().await
    }

    /// Get write access to the state
    pub async fn write(&self) -> GlobalStateWriteGuard<'_, T> {
        GlobalStateWriteGuard {
            guard: self.data.write().await,
            container: self,
        }
    }

    /// Update state with a closure (prevents holding write lock too long)
    pub async fn update<F, R>(&self, source: &'static str, updater: F) -> Result<R, String>
    where
        F: FnOnce(&mut T) -> Result<R, String>,
    {
        let mut guard = self.data.write().await;
        let result = updater(&mut guard);

        if result.is_ok() {
            self.record_update(source).await;
        }

        result
    }

    /// Subscribe to state change events
    pub fn subscribe(&self) -> broadcast::Receiver<StateChangeEvent> {
        self.change_sender.subscribe()
    }

    /// Get last update time
    pub async fn last_updated(&self) -> Instant {
        *self.last_updated.lock().await
    }

    /// Get update counter
    pub async fn update_count(&self) -> u64 {
        *self.update_counter.lock().await
    }

    pub(crate) async fn record_update(&self, source: &'static str) {
        let now = Instant::now();
        *self.last_updated.lock().await = now;
        *self.update_counter.lock().await += 1;

        let event = StateChangeEvent::Updated {
            source,
            timestamp: now,
        };
        let _ = self.change_sender.send(event); // Ignore if no subscribers

        debug!("Global state updated by {} at {:?}", source, now);
    }
}

/// Write guard that automatically notifies of changes
pub struct GlobalStateWriteGuard<'a, T> {
    guard: tokio::sync::RwLockWriteGuard<'a, T>,
    container: &'a GlobalStateContainer<T>,
}

impl<'a, T: Default> GlobalStateWriteGuard<'a, T> {
    /// Get mutable access to the state
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.guard
    }

    /// Commit changes and notify subscribers
    pub async fn commit(self, source: &'static str) {
        drop(self.guard);
        self.container.record_update(source).await;
    }
}

impl<'a, T> std::ops::Deref for GlobalStateWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> std::ops::DerefMut for GlobalStateWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

/// Tool registry with thread-safe access
pub struct ThreadSafeToolRegistry {
    /// Registry of tool instances
    tools: Arc<RwLock<HashMap<String, Arc<dyn Send + Sync + 'static>>>>,
    /// Tool lifecycle events
    lifecycle_sender: broadcast::Sender<ToolLifecycleEvent>,
    /// Registry health status
    health_status: Arc<RwLock<RegistryHealth>>,
}

/// Tool lifecycle events
#[derive(Debug, Clone)]
pub enum ToolLifecycleEvent {
    /// Tool was registered
    Registered {
        tool_id: String,
        tool_type: &'static str,
    },
    /// Tool was unregistered
    Unregistered { tool_id: String },
    /// Tool encountered an error
    Error { tool_id: String, error: String },
    /// Tool was reinitialized
    Reinitialized { tool_id: String },
}

/// Registry health information
#[derive(Debug, Clone)]
pub struct RegistryHealth {
    pub total_tools: usize,
    pub healthy_tools: usize,
    pub error_tools: usize,
    pub last_check: Instant,
}

impl Default for RegistryHealth {
    fn default() -> Self {
        Self {
            total_tools: 0,
            healthy_tools: 0,
            error_tools: 0,
            last_check: Instant::now(),
        }
    }
}

impl ThreadSafeToolRegistry {
    /// Create a new thread-safe tool registry
    pub fn new() -> Self {
        let (lifecycle_sender, _) = broadcast::channel(1000);
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            lifecycle_sender,
            health_status: Arc::new(RwLock::new(RegistryHealth::default())),
        }
    }

    /// Register a new tool
    pub async fn register_tool(
        &self,
        tool_id: String,
        tool: Arc<dyn Send + Sync>,
    ) -> Result<(), String> {
        let mut tools = self.tools.write().await;

        if tools.contains_key(&tool_id) {
            return Err(format!("Tool '{}' already registered", tool_id));
        }

        tools.insert(tool_id.clone(), tool);

        // Update health status
        let mut health = self.health_status.write().await;
        health.total_tools = tools.len();
        health.healthy_tools += 1;
        health.last_check = Instant::now();

        // Notify subscribers
        let event = ToolLifecycleEvent::Registered {
            tool_id: tool_id.clone(),
            tool_type: "Tool",
        };
        let _ = self.lifecycle_sender.send(event);

        info!("Tool '{}' registered successfully", tool_id);
        Ok(())
    }

    /// Unregister a tool
    pub async fn unregister_tool(&self, tool_id: &str) -> Result<(), String> {
        let mut tools = self.tools.write().await;
        let tool = tools
            .remove(tool_id)
            .ok_or_else(|| format!("Tool '{}' not found", tool_id))?;

        // Update health status
        let mut health = self.health_status.write().await;
        health.total_tools = tools.len();
        health.healthy_tools = health.healthy_tools.saturating_sub(1);
        health.last_check = Instant::now();

        // Notify subscribers
        let event = ToolLifecycleEvent::Unregistered {
            tool_id: tool_id.to_string(),
        };
        let _ = self.lifecycle_sender.send(event);

        info!("Tool '{}' unregistered", tool_id);
        Ok(())
    }

    /// Get a tool by ID
    pub async fn get_tool(&self, tool_id: &str) -> Option<Arc<dyn Send + Sync>> {
        let tools = self.tools.read().await;
        tools.get(tool_id).cloned()
    }

    /// Get all tool IDs
    pub async fn get_tool_ids(&self) -> Vec<String> {
        let tools = self.tools.read().await;
        tools.keys().cloned().collect()
    }

    /// Subscribe to lifecycle events
    pub fn subscribe_lifecycle(&self) -> broadcast::Receiver<ToolLifecycleEvent> {
        self.lifecycle_sender.subscribe()
    }

    /// Get registry health
    pub async fn get_health(&self) -> RegistryHealth {
        self.health_status.read().await.clone()
    }

    /// Record tool error
    pub async fn record_tool_error(&self, tool_id: &str, error: String) {
        // Update health status
        let mut health = self.health_status.write().await;
        health.error_tools += 1;
        health.healthy_tools = health.healthy_tools.saturating_sub(1);
        health.last_check = Instant::now();

        // Notify subscribers
        let event = ToolLifecycleEvent::Error {
            tool_id: tool_id.to_string(),
            error,
        };
        let _ = self.lifecycle_sender.send(event);
    }

    /// Record tool recovery
    pub async fn record_tool_recovery(&self, tool_id: &str) {
        // Update health status
        let mut health = self.health_status.write().await;
        health.error_tools = health.error_tools.saturating_sub(1);
        health.healthy_tools += 1;
        health.last_check = Instant::now();

        // Notify subscribers
        let event = ToolLifecycleEvent::Reinitialized {
            tool_id: tool_id.to_string(),
        };
        let _ = self.lifecycle_sender.send(event);
    }
}

/// Deadlock-prevention timer for async operations
#[derive(Debug, Clone)]
pub struct OperationTimeout {
    /// Default timeout duration
    default_timeout: Duration,
    /// Maximum retry attempts
    max_retries: usize,
    /// Exponential backoff base
    backoff_base: u64,
}

impl Default for OperationTimeout {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            max_retries: 3,
            backoff_base: 2,
        }
    }
}

impl OperationTimeout {
    /// Create a new timeout configuration
    pub fn new(timeout_secs: u64, max_retries: usize) -> Self {
        Self {
            default_timeout: Duration::from_secs(timeout_secs),
            max_retries,
            backoff_base: 2,
        }
    }

    /// Calculate backoff delay for retry attempt
    pub fn backoff_delay(&self, attempt: usize) -> Duration {
        let delay_ms = self.backoff_base.pow(attempt as u32) * 100;
        Duration::from_millis(delay_ms.min(5000)) // Cap at 5 seconds
    }

    /// Execute operation with timeout and retry logic
    pub async fn execute_with_timeout<F, Fut, T>(
        &self,
        operation_name: &str,
        operation: F,
    ) -> Result<T, String>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        for attempt in 1..=self.max_retries {
            let timeout = self.default_timeout;
            let operation_future = operation();

            match tokio::time::timeout(timeout, operation_future).await {
                Ok(Ok(result)) => {
                    if attempt > 1 {
                        debug!(
                            "Operation '{}' succeeded on attempt {}",
                            operation_name, attempt
                        );
                    }
                    return Ok(result);
                }
                Ok(Err(e)) => {
                    if attempt == self.max_retries {
                        return Err(format!(
                            "Operation '{}' failed after {} attempts: {}",
                            operation_name, self.max_retries, e
                        ));
                    }
                    warn!(
                        "Operation '{}' failed on attempt {}, retrying: {}",
                        operation_name, attempt, e
                    );
                }
                Err(_) => {
                    if attempt == self.max_retries {
                        return Err(format!(
                            "Operation '{}' timed out after {} attempts",
                            operation_name, self.max_retries
                        ));
                    }
                    warn!(
                        "Operation '{}' timed out on attempt {}, retrying",
                        operation_name, attempt
                    );
                }
            }

            if attempt < self.max_retries {
                let delay = self.backoff_delay(attempt);
                tokio::time::sleep(delay).await;
            }
        }

        Err(format!(
            "Operation '{}' failed after all retries",
            operation_name
        ))
    }
}

/// Migration helper for tool transitions
#[derive(Debug)]
pub struct MigrationHelper;

impl MigrationHelper {
    pub fn new() -> Self {
        Self
    }

    pub async fn start_assessment(&self, _tool_name: &str) -> Result<(), String> {
        Ok(())
    }
}

/// Global managers using safe threading patterns
lazy_static! {
    /// Thread-safe tool registry
    pub static ref GLOBAL_TOOL_REGISTRY: ThreadSafeToolRegistry = ThreadSafeToolRegistry::new();

    /// Thread-safe window manager
    pub static ref GLOBAL_WINDOW_MANAGER: Arc<RwLock<crate::ui::window_manager::WindowManager>> =
        Arc::new(RwLock::new(
            crate::ui::window_manager::WindowManager::new()
                .expect("Failed to create global window manager")
        ));

    /// Operation timeout configuration
    pub static ref OPERATION_TIMEOUT: OperationTimeout = OperationTimeout::default();
}

/// Get reference to global tool registry
pub fn get_tool_registry() -> &'static ThreadSafeToolRegistry {
    &GLOBAL_TOOL_REGISTRY
}

/// Get reference to global window manager
pub async fn get_window_manager(
) -> tokio::sync::RwLockReadGuard<'static, crate::ui::window_manager::WindowManager> {
    GLOBAL_WINDOW_MANAGER.read().await
}

/// Get reference to global window manager (write access)
pub async fn get_window_manager_mut(
) -> tokio::sync::RwLockWriteGuard<'static, crate::ui::window_manager::WindowManager> {
    GLOBAL_WINDOW_MANAGER.write().await
}

/// Get operation timeout configuration
pub fn get_operation_timeout() -> &'static OperationTimeout {
    &OPERATION_TIMEOUT
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_global_state_container() {
        let container: GlobalStateContainer<String> = GlobalStateContainer::new();

        // Test read/write access
        {
            let mut state = container.write().await;
            *state.get_mut() = "test_value".to_string();
            state.commit("test").await;
        }

        let value = container.read().await;
        assert_eq!(*value, "test_value");
    }

    #[tokio::test]
    async fn test_thread_safe_tool_registry() {
        let registry = ThreadSafeToolRegistry::new();

        // Test tool registration
        let test_tool = Arc::new(()) as Arc<dyn Send + Sync>;
        registry
            .register_tool("test_tool".to_string(), test_tool)
            .await
            .unwrap();

        // Test tool retrieval
        let retrieved = registry.get_tool("test_tool").await;
        assert!(retrieved.is_some());

        // Test tool IDs
        let tool_ids = registry.get_tool_ids().await;
        assert_eq!(tool_ids, vec!["test_tool"]);
    }

    #[tokio::test]
    async fn test_operation_timeout() {
        let timeout = OperationTimeout::new(1, 2);

        // Test successful operation
        let result = timeout
            .execute_with_timeout("test_success", || async { Ok("success") })
            .await;
        assert_eq!(result.unwrap(), "success");

        // Test failing operation with retries
        // Test failing operation with retries
        let attempts = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let attempts_clone = attempts.clone();

        let result = timeout
            .execute_with_timeout("test_failure", move || {
                let attempts = attempts_clone.clone();
                async move {
                    let current_attempts =
                        attempts.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                    if current_attempts < 2 {
                        Err("Simulated failure".to_string())
                    } else {
                        Ok("success")
                    }
                }
            })
            .await;

        assert_eq!(result.unwrap(), "success");
        // Note: We can't easily test the exact number of attempts here
        // since the closure captures the atomic variable by value
    }
}
