use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::path::Path;
use lazy_static::lazy_static;
use crate::error::{AppError, log_error};

/// Rate limiter for file operations
#[derive(Debug)]
struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            requests: HashMap::new(),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    fn check_rate_limit(&mut self, key: &str) -> bool {
        let now = Instant::now();

        // Clean old requests
        if let Some(times) = self.requests.get_mut(key) {
            times.retain(|&time| now.duration_since(time) < self.window);
        }

        // Check if under limit
        let current_count = self.requests.get(key).map(|times| times.len()).unwrap_or(0);
        if current_count >= self.max_requests as usize {
            return false;
        }

        // Add new request
        self.requests.entry(key.to_string()).or_insert_with(Vec::new).push(now);
        true
    }
}

/// File access control and rate limiting
#[derive(Debug)]
pub struct FileAccessControl {
    rate_limiter: Mutex<RateLimiter>,
    allowed_operations: Vec<String>,
    max_file_size: u64,
    max_total_operations: u32,
}

impl FileAccessControl {
    pub fn new() -> Self {
        Self {
            rate_limiter: Mutex::new(RateLimiter::new(100, 60)), // 100 requests per minute
            allowed_operations: vec![
                "read".to_string(),
                "write".to_string(),
                "save".to_string(),
                "load".to_string(),
                "import".to_string(),
                "export".to_string(),
            ],
            max_file_size: 50 * 1024 * 1024, // 50MB
            max_total_operations: 1000,
        }
    }

    /// Check if operation is allowed and within rate limits
    pub fn check_access(&self, operation: &str, user_id: &str, file_path: &Path) -> Result<(), AppError> {
        // Check if operation is allowed
        if !self.allowed_operations.contains(&operation.to_string()) {
            let err = AppError::FileError(format!("Operation '{}' is not allowed", operation));
            log_error(&err);
            return Err(err);
        }

        // Rate limiting check
        let rate_key = format!("{}_{}", user_id, operation);
        if !self.rate_limiter.lock().unwrap().check_rate_limit(&rate_key) {
            let err = AppError::FileError(format!("Rate limit exceeded for operation '{}'", operation));
            log_error(&err);
            return Err(err);
        }

        // File size validation
        if let Ok(metadata) = file_path.metadata() {
            let file_size = metadata.len();
            if file_size > self.max_file_size {
                let err = AppError::FileError(format!("File size {} exceeds maximum allowed size", file_size));
                log_error(&err);
                return Err(err);
            }
        }

        // Log successful access check
        eprintln!("File access allowed: operation={}, user={}, path={}",
                 operation, user_id, file_path.display());

        Ok(())
    }

    /// Record audit log for sensitive operations
    pub fn audit_log(&self, operation: &str, user_id: &str, file_path: &Path, success: bool, details: Option<&str>) {
        let status = if success { "SUCCESS" } else { "FAILED" };
        let detail_msg = details.unwrap_or("");

        eprintln!("AUDIT: [{}] operation={}, user={}, path={}, details={}",
                 status, operation, user_id, file_path.display(), detail_msg);

        // TODO: Implement persistent audit logging to file/database
        // For now, just log to stderr for security monitoring
    }
}

lazy_static! {
    pub static ref FILE_ACCESS_CONTROL: Arc<FileAccessControl> = Arc::new(FileAccessControl::new());
}

/// Check file access permissions (to be integrated into file operations)
pub fn check_file_access(operation: &str, user_id: &str, file_path: &Path) -> Result<(), AppError> {
    FILE_ACCESS_CONTROL.check_access(operation, user_id, file_path)
}

/// Log audit event for file operations
pub fn log_file_operation(operation: &str, user_id: &str, file_path: &Path, success: bool, details: Option<&str>) {
    FILE_ACCESS_CONTROL.audit_log(operation, user_id, file_path, success, details);
}