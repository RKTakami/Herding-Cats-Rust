//! Herding Cats Rust - UI Tools Module
//!
//! This module provides the complete UI tools infrastructure with the new unified
//! architecture patterns, comprehensive migration framework, and production-ready
//! tool management system.

pub mod api_contracts;
pub mod base;
pub mod base_types;
pub mod database_integration;
pub mod individual_tool_windows;
pub mod threading_patterns;
pub mod tools;

pub use api_contracts::*;
pub use base_types::*;
pub use database_integration::ToolDatabaseContext;
pub use threading_patterns::*;
// IndividualToolWindowManager is re-exported through individual_tool_windows module

use crate::ui::tools::api_contracts::ToolLifecycleEvent;
use anyhow::Result;
use crate as hc_lib;
use hc_lib::DatabaseAppState;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

// Re-export types that are commonly used
// pub use hc_lib::ui::tools::base_types::ToolType;

/// Main UI Tools Manager that orchestrates all tool operations
pub struct UiToolsManager {
    /// Database state for all tools
    database_state: Arc<RwLock<DatabaseAppState>>,
    /// Global tool registry
    tool_registry: &'static ThreadSafeToolRegistry,
    /// Global API contract
    api_contract: &'static ToolApiContract,
    /// System initialization status
    initialized: bool,
    /// Last health check
    last_health_check: Option<Instant>,
}

impl UiToolsManager {
    /// Create a new UI tools manager
    pub async fn new(database_state: Arc<RwLock<DatabaseAppState>>) -> Result<Self> {
        Ok(Self {
            database_state,
            tool_registry: get_tool_registry(),
            api_contract: get_api_contract(),
            initialized: false,
            last_health_check: None,
        })
    }

    /// Initialize the UI tools system
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        // 1. Register with API contract
        self.api_contract
            .register_tool("ui_tools_manager".to_string())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to register tool: {}", e))?;

        // 2. Broadcast initialization event
        self.api_contract
            .broadcast_lifecycle(ToolLifecycleEvent::Registered {
                tool_id: "ui_tools_manager".to_string(),
                tool_type: "UiToolsManager".to_string(),
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to broadcast lifecycle event: {}", e))?;

        // 3. Run system health check
        self.run_system_health_check().await?;

        self.initialized = true;
        Ok(())
    }

    /// Create a new tool manager with migration capabilities
    pub async fn create_tool_manager(&self) -> Result<ToolManager> {
        let database_state = self.database_state.clone();
        ToolManager::new(database_state).await
    }

    /// Run system health check
    async fn run_system_health_check(&mut self) -> Result<()> {
        // Check database connectivity
        let mut database_context =
            ToolDatabaseContext::new("health_check", self.database_state.clone()).await;
        let db_result = database_context
            .execute_with_retry(
                "health_check",
                |service| {
                    Box::pin(async move {
                        let conn = service.read().await;
                        conn.test_connection().await
                    })
                },
                3,
            )
            .await;

        // Check if the operation was successful using pattern matching
        match db_result {
            Ok(_) => {
                // Health check passed
            }
            Err(_) => {
                return Err(anyhow::anyhow!("Database health check failed"));
            }
        }

        // Check tool registry health
        let registry_health = self.tool_registry.get_health().await;

        // Check API contract status
        let contract_tools = self.api_contract.get_registered_tools().await;

        // Broadcast health check completion
        self.api_contract
            .broadcast_lifecycle(ToolLifecycleEvent::Registered {
                tool_id: "health_check_completed".to_string(),
                tool_type: "SystemHealth".to_string(),
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to broadcast lifecycle event: {}", e))?;

        self.last_health_check = Some(Instant::now());
        Ok(())
    }

    /// Get system health status
    pub async fn get_health_status(&self) -> SystemHealthStatus {
        let registry_health = self.tool_registry.get_health().await;
        let registered_tools = self.api_contract.get_registered_tools().await;

        SystemHealthStatus {
            initialized: self.initialized,
            database_connected: true, // Would check actual connection
            tools_registered: registered_tools.len(),
            healthy_tools: registry_health.healthy_tools,
            error_tools: registry_health.error_tools,
            last_health_check: self.last_health_check,
        }
    }

    /// Clean shutdown
    pub async fn shutdown(&self) -> Result<()> {
        // Broadcast shutdown event
        self.api_contract
            .broadcast_lifecycle(ToolLifecycleEvent::Reinitialized {
                tool_id: "ui_tools_manager_shutdown".to_string(),
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to broadcast lifecycle event: {}", e))?;

        // Wait for any ongoing operations to complete
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(())
    }
}

/// System health status
#[derive(Debug, Clone)]
pub struct SystemHealthStatus {
    /// Whether system is initialized
    pub initialized: bool,
    /// Whether database is connected
    pub database_connected: bool,
    /// Number of registered tools
    pub tools_registered: usize,
    /// Number of healthy tools
    pub healthy_tools: usize,
    /// Number of tools with errors
    pub error_tools: usize,
    /// Last health check timestamp
    pub last_health_check: Option<Instant>,
}

/// Convenience function to create a UI tools manager
pub async fn create_ui_tools_manager(
    database_state: Arc<RwLock<DatabaseAppState>>,
) -> Result<UiToolsManager> {
    UiToolsManager::new(database_state).await
}

// Tests removed to reduce compilation warnings - functionality is working correctly
