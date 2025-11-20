use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Simple in-memory cache with TTL support
pub struct Cache<T> {
    data: Arc<Mutex<HashMap<String, (T, Instant)>>>,
    ttl: Duration,
}

impl<T> Cache<T> {
    pub fn new(ttl_seconds: u64) -> Self {
        Cache {
            data: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn get(&self, key: &str) -> Option<T>
    where
        T: Clone,
    {
        let mut data = self.data.lock().unwrap();
        if let Some((value, timestamp)) = data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            } else {
                // Remove expired entry
                data.remove(key);
            }
        }
        None
    }

    pub fn set(&self, key: String, value: T) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, (value, Instant::now()));
    }

    pub fn clear(&self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
    }

    pub fn size(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }
}

/// Global cache for file conversion results (5 minute TTL)
pub static FILE_CONVERSION_CACHE: once_cell::sync::Lazy<Cache<String>> =
    once_cell::sync::Lazy::new(|| Cache::new(300));

/// Global cache for parsed markdown (10 minute TTL)
pub static MARKDOWN_PARSE_CACHE: once_cell::sync::Lazy<Cache<String>> =
    once_cell::sync::Lazy::new(|| Cache::new(600));