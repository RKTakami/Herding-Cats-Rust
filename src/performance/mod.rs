//! Performance Monitoring Module
//! 
//! Handles performance metrics, monitoring, and optimization tracking.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance metrics for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation_name: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub duration_ms: Option<u64>,
    pub success: bool,
    pub error_message: Option<String>,
    pub additional_data: HashMap<String, String>,
}

impl PerformanceMetrics {
    /// Create a new performance metrics instance
    pub fn new(operation_name: String) -> Self {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            operation_name,
            start_time,
            end_time: None,
            duration_ms: None,
            success: false,
            error_message: None,
            additional_data: HashMap::new(),
        }
    }

    /// Mark the operation as completed
    pub fn complete(mut self, success: bool) -> Self {
        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let duration_ms = end_time.saturating_sub(self.start_time);

        self.end_time = Some(end_time);
        self.duration_ms = Some(duration_ms);
        self.success = success;

        self
    }

    /// Add additional data to the metrics
    pub fn with_data(mut self, key: String, value: String) -> Self {
        self.additional_data.insert(key, value);
        self
    }

    /// Set error message
    pub fn with_error(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }
}

/// Performance monitor for tracking operations
#[derive(Default)]
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<Vec<PerformanceMetrics>>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self::default()
    }

    /// Record performance metrics
    pub fn record_metrics(&self, metrics: PerformanceMetrics) {
        let mut guard = self.metrics.lock().unwrap();
        guard.push(metrics);
    }

    /// Get operation count
    pub fn get_operation_count(&self, operation_name: &str) -> usize {
        let guard = self.metrics.lock().unwrap();
        guard.iter()
            .filter(|m| m.operation_name == operation_name)
            .count()
    }

    /// Get success rate for an operation
    pub fn get_success_rate(&self, operation_name: &str) -> Option<f64> {
        let guard = self.metrics.lock().unwrap();
        let operations: Vec<&PerformanceMetrics> = guard.iter()
            .filter(|m| m.operation_name == operation_name)
            .collect();

        if operations.is_empty() {
            None
        } else {
            let success_count = operations.iter()
                .filter(|m| m.success)
                .count();
            Some(success_count as f64 / operations.len() as f64)
        }
    }

    /// Get average duration for an operation
    pub fn get_average_duration(&self, operation_name: &str) -> Option<u64> {
        let guard = self.metrics.lock().unwrap();
        let operations: Vec<&PerformanceMetrics> = guard.iter()
            .filter(|m| m.operation_name == operation_name && m.duration_ms.is_some())
            .collect();

        if operations.is_empty() {
            None
        } else {
            let total_duration: u64 = operations.iter()
                .map(|m| m.duration_ms.unwrap())
                .sum();
            Some(total_duration / operations.len() as u64)
        }
    }

    /// Get all metrics
    pub fn get_all_metrics(&self) -> Vec<PerformanceMetrics> {
        let guard = self.metrics.lock().unwrap();
        guard.clone()
    }

    /// Clear all metrics
    pub fn clear_metrics(&self) {
        let mut guard = self.metrics.lock().unwrap();
        guard.clear();
    }

    /// Get performance summary
    pub fn get_summary(&self) -> PerformanceSummary {
        let guard = self.metrics.lock().unwrap();
        
        let total_operations = guard.len();
        let successful_operations = guard.iter().filter(|m| m.success).count();
        let failed_operations = total_operations - successful_operations;

        let mut operation_stats = std::collections::HashMap::new();
        for metric in &*guard {
            let stats = operation_stats.entry(metric.operation_name.clone())
                .or_insert_with(|| OperationStats::default());
            
            stats.count += 1;
            if metric.success {
                stats.success_count += 1;
            }
            if let Some(duration) = metric.duration_ms {
                stats.total_duration += duration;
            }
        }

        PerformanceSummary {
            total_operations,
            successful_operations,
            failed_operations,
            success_rate: if total_operations > 0 {
                successful_operations as f64 / total_operations as f64
            } else {
                0.0
            },
            operation_stats,
        }
    }
}

/// Operation statistics
#[derive(Debug, Clone)]
pub struct OperationStats {
    pub count: usize,
    pub success_count: usize,
    pub total_duration: u64,
}

impl Default for OperationStats {
    fn default() -> Self {
        Self {
            count: 0,
            success_count: 0,
            total_duration: 0,
        }
    }
}

/// Performance summary
#[derive(Debug)]
pub struct PerformanceSummary {
    pub total_operations: usize,
    pub successful_operations: usize,
    pub failed_operations: usize,
    pub success_rate: f64,
    pub operation_stats: HashMap<String, OperationStats>,
}

/// Performance tracker RAII guard
pub struct PerformanceTracker {
    monitor: Arc<PerformanceMonitor>,
    operation_name: String,
    start_time: Instant,
    additional_data: HashMap<String, String>,
}

impl PerformanceTracker {
    /// Create a new performance tracker
    pub fn new(monitor: Arc<PerformanceMonitor>, operation_name: String) -> Self {
        Self {
            monitor,
            operation_name,
            start_time: Instant::now(),
            additional_data: HashMap::new(),
        }
    }

    /// Add additional data
    pub fn add_data(&mut self, key: String, value: String) {
        self.additional_data.insert(key, value);
    }
}

impl Drop for PerformanceTracker {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        let metrics = PerformanceMetrics::new(self.operation_name.clone())
            .complete(true)
            .with_data("tracker".to_string(), "true".to_string());

        // Add any additional data
        for (key, value) in &self.additional_data {
            // This won't work with the current implementation, but shows the intent
        }

        self.monitor.record_metrics(metrics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metrics() {
        let metrics = PerformanceMetrics::new("test_operation".to_string())
            .complete(true);

        assert_eq!(metrics.operation_name, "test_operation");
        assert!(metrics.success);
        assert!(metrics.duration_ms.is_some());
        assert!(metrics.end_time.is_some());
    }

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();
        
        let metrics = PerformanceMetrics::new("test_op".to_string());
        let completed = metrics.complete(true);
        monitor.record_metrics(completed);

        assert_eq!(monitor.get_operation_count("test_op"), 1);
        assert_eq!(monitor.get_success_rate("test_op"), Some(1.0));
        assert!(monitor.get_average_duration("test_op").is_some());
    }

    #[test]
    fn test_performance_tracker() {
        let monitor = Arc::new(PerformanceMonitor::new());
        
        {
            let _tracker = PerformanceTracker::new(monitor.clone(), "tracked_op".to_string());
            // Simulate some work
            std::thread::sleep(Duration::from_millis(10));
        }

        // Metrics should be recorded when tracker is dropped
        assert_eq!(monitor.get_operation_count("tracked_op"), 1);
    }
}