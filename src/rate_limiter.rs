use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::error::{AppError, log_error};

/// Rate limiter for API calls and general request throttling
#[derive(Debug)]
pub struct ApiRateLimiter {
    requests: Mutex<HashMap<String, Vec<Instant>>>,
    max_requests_per_minute: u32,
    max_requests_per_hour: u32,
    burst_limit: u32,
    window_minute: Duration,
    window_hour: Duration,
}

impl ApiRateLimiter {
    pub fn new(max_per_minute: u32, max_per_hour: u32, burst_limit: u32) -> Self {
        Self {
            requests: Mutex::new(HashMap::new()),
            max_requests_per_minute: max_per_minute,
            max_requests_per_hour: max_per_hour,
            burst_limit,
            window_minute: Duration::from_secs(60),
            window_hour: Duration::from_secs(3600),
        }
    }

    /// Check if request is allowed under rate limits
    pub fn check_rate_limit(&self, client_id: &str, endpoint: &str) -> Result<(), AppError> {
        let now = Instant::now();
        let key = format!("{}_{}", client_id, endpoint);

        let mut requests = self.requests.lock().expect("Rate limiter requests poisoned");

        // Clean old requests
        if let Some(times) = requests.get_mut(&key) {
            times.retain(|&time| {
                now.duration_since(time) < self.window_hour
            });
        }

        let current_count = requests.get(&key).map(|times| times.len()).unwrap_or(0);

        // Check burst limit (requests in last minute)
        let recent_count = requests.get(&key)
            .map(|times| times.iter().filter(|&&time| now.duration_since(time) < self.window_minute).count())
            .unwrap_or(0);

        // Check limits
        if recent_count >= self.max_requests_per_minute as usize {
            let err = AppError::FileError(format!("Rate limit exceeded: {} requests per minute", self.max_requests_per_minute));
            log_error(&err);
            return Err(err);
        }

        if current_count >= self.max_requests_per_hour as usize {
            let err = AppError::FileError(format!("Rate limit exceeded: {} requests per hour", self.max_requests_per_hour));
            log_error(&err);
            return Err(err);
        }

        if current_count >= self.burst_limit as usize {
            let err = AppError::FileError(format!("Burst limit exceeded: {} requests", self.burst_limit));
            log_error(&err);
            return Err(err);
        }

        // Add new request
        requests.entry(key).or_insert_with(Vec::new).push(now);

        Ok(())
    }

    /// Get current rate limit status for monitoring
    pub fn get_status(&self, client_id: &str, endpoint: &str) -> (usize, usize, usize) {
        let key = format!("{}_{}", client_id, endpoint);
        let requests = self.requests.lock().expect("Rate limiter requests poisoned");

        let current_count = requests.get(&key).map(|times| times.len()).unwrap_or(0);
        let recent_count = requests.get(&key)
            .map(|times| times.iter().filter(|&&time| Instant::now().duration_since(time) < self.window_minute).count())
            .unwrap_or(0);

        (recent_count, current_count, self.max_requests_per_hour as usize)
    }
}

/// DDoS protection mechanisms
#[derive(Debug)]
pub struct DdosProtection {
    suspicious_patterns: Mutex<HashMap<String, u32>>,
    blocklist: Mutex<Vec<String>>,
    max_suspicious_score: u32,
}

impl DdosProtection {
    pub fn new() -> Self {
        Self {
            suspicious_patterns: Mutex::new(HashMap::new()),
            blocklist: Mutex::new(Vec::new()),
            max_suspicious_score: 10,
        }
    }

    /// Analyze request patterns for DDoS indicators
    pub fn analyze_request(&self, client_id: &str, user_agent: &str, request_size: usize) -> Result<(), AppError> {
        let mut suspicious = self.suspicious_patterns.lock().expect("Suspicious patterns poisoned");

        // Check for suspicious patterns
        let mut score = 0u32;

        // Unusual user agent
        if user_agent.is_empty() || user_agent.len() < 10 {
            score += 2;
        }

        // Large request size
        if request_size > 1024 * 1024 { // 1MB
            score += 1;
        }

        // Rapid requests from same client
        let client_key = format!("client_{}", client_id);
        *suspicious.entry(client_key.clone()).or_insert(0) += score;

        let current_score = *suspicious.get(&client_key).unwrap_or(&0);

        if current_score > self.max_suspicious_score {
            let mut blocklist = self.blocklist.lock().expect("Blocklist poisoned");
            if !blocklist.contains(&client_id.to_string()) {
                blocklist.push(client_id.to_string());
                let err = AppError::FileError(format!("Client {} blocked due to suspicious activity", client_id));
                log_error(&err);
                return Err(err);
            }
        }

        Ok(())
    }

    /// Check if client is blocked
    pub fn is_blocked(&self, client_id: &str) -> bool {
        let blocklist = self.blocklist.lock().expect("Blocklist poisoned");
        blocklist.contains(&client_id.to_string())
    }
}

lazy_static::lazy_static! {
    pub static ref API_RATE_LIMITER: Arc<ApiRateLimiter> = Arc::new(ApiRateLimiter::new(60, 1000, 10));
    pub static ref DDOS_PROTECTION: Arc<DdosProtection> = Arc::new(DdosProtection::new());
}

/// Check API rate limits
pub fn check_api_rate_limit(client_id: &str, endpoint: &str) -> Result<(), AppError> {
    API_RATE_LIMITER.check_rate_limit(client_id, endpoint)
}

/// Analyze request for DDoS protection
pub fn analyze_request_for_ddos(client_id: &str, user_agent: &str, request_size: usize) -> Result<(), AppError> {
    DDOS_PROTECTION.analyze_request(client_id, user_agent, request_size)
}

/// Check if client is blocked
pub fn is_client_blocked(client_id: &str) -> bool {
    DDOS_PROTECTION.is_blocked(client_id)
}

/// Get rate limit status for monitoring
pub fn get_rate_limit_status(client_id: &str, endpoint: &str) -> (usize, usize, usize) {
    API_RATE_LIMITER.get_status(client_id, endpoint)
}

// Additional rate limiter for testing
#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub fn check_rate_limit(&self, client_id: &str) -> bool {
        let now = Instant::now();
        let mut requests = self.requests.lock().expect("Rate limiter requests poisoned");

        // Clean old requests
        if let Some(times) = requests.get_mut(client_id) {
            times.retain(|&time| now.duration_since(time) < self.window);
        }

        let current_count = requests.get(client_id).map(|times| times.len()).unwrap_or(0);

        if current_count >= self.max_requests as usize {
            return false;
        }

        // Add new request
        requests.entry(client_id.to_string()).or_insert_with(Vec::new).push(now);
        true
    }
}
