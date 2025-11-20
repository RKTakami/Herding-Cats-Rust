use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Memory usage tracker
pub struct MemoryTracker {
    allocations: Arc<Mutex<HashMap<String, usize>>>,
    total_allocated: Arc<Mutex<usize>>,
}

impl MemoryTracker {
    pub fn new() -> Self {
        MemoryTracker {
            allocations: Arc::new(Mutex::new(HashMap::new())),
            total_allocated: Arc::new(Mutex::new(0)),
        }
    }

    pub fn track_allocation(&self, key: String, size: usize) {
        let mut allocations = self.allocations.lock().expect("Memory tracker allocations poisoned");
        let mut total = self.total_allocated.lock().unwrap();

        // Remove previous allocation if it exists
        if let Some(prev_size) = allocations.get(&key) {
            *total = total.saturating_sub(*prev_size);
        }

        allocations.insert(key, size);
        *total = total.saturating_add(size);
    }

    pub fn release_allocation(&self, key: &str) {
        let mut allocations = self.allocations.lock().expect("Memory tracker allocations poisoned");
        let mut total = self.total_allocated.lock().expect("Memory tracker total poisoned");

        if let Some(size) = allocations.remove(key) {
            *total = total.saturating_sub(size);
        }
    }

    pub fn get_total_memory(&self) -> usize {
        *self.total_allocated.lock().expect("Memory tracker total poisoned")
    }

    pub fn get_allocation_count(&self) -> usize {
        self.allocations.lock().expect("Memory tracker allocations poisoned").len()
    }

    pub fn get_allocation(&self, key: &str) -> Option<usize> {
        self.allocations.lock().expect("Memory tracker allocations poisoned").get(key).copied()
    }

    pub fn clear(&self) {
        let mut allocations = self.allocations.lock().expect("Memory tracker allocations poisoned");
        let mut total = self.total_allocated.lock().expect("Memory tracker total poisoned");
        allocations.clear();
        *total = 0;
    }
}

/// Global memory tracker
pub static MEMORY_TRACKER: once_cell::sync::Lazy<MemoryTracker> =
    once_cell::sync::Lazy::new(|| MemoryTracker::new());

/// Memory-aware resource cleanup
pub struct MemoryGuard {
    key: String,
}

impl MemoryGuard {
    pub fn new(key: String, size: usize) -> Self {
        MEMORY_TRACKER.track_allocation(key.clone(), size);
        MemoryGuard { key }
    }
}

impl Drop for MemoryGuard {
    fn drop(&mut self) {
        MEMORY_TRACKER.release_allocation(&self.key);
    }
}

/// Helper macro for tracking memory usage
#[macro_export]
macro_rules! track_memory {
    ($key:expr, $size:expr, $operation:block) => {{
        let _guard = crate::memory::MemoryGuard::new($key.to_string(), $size);
        $operation
    }};
}

/// Memory-efficient file processing with size limits
pub const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024; // 50MB
pub const MAX_MEMORY_USAGE: usize = 500 * 1024 * 1024; // 500MB

/// Check if we should process a file based on memory constraints
pub fn can_process_file(file_size: u64, estimated_processing_memory: usize) -> bool {
    let current_memory = MEMORY_TRACKER.get_total_memory();

    // Check file size limit
    if file_size > MAX_FILE_SIZE {
        return false;
    }

    // Check memory usage limit
    if current_memory + estimated_processing_memory > MAX_MEMORY_USAGE {
        return false;
    }

    true
}

/// Get current memory usage percentage
pub fn get_memory_usage_percentage() -> f64 {
    let current = MEMORY_TRACKER.get_total_memory() as f64;
    let max = MAX_MEMORY_USAGE as f64;
    (current / max) * 100.0
}
