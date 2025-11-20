//! Configuration Security Module
//! 
//! Handles encryption keys and security configuration for the application.

use crate::services::SecurityService;

/// Generate an encryption key for configuration security
pub fn generate_encryption_key() -> String {
    // Use the existing security service to generate a secure key
    let security_service = SecurityService::new();
    security_service.secure_random_string(32)
}

/// Security configuration structure
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub encryption_key: String,
    pub api_key: Option<String>,
    pub enable_encryption: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            encryption_key: generate_encryption_key(),
            api_key: None,
            enable_encryption: true,
        }
    }
}

impl SecurityConfig {
    /// Create a new security configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the API key
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Enable or disable encryption
    pub fn with_encryption(mut self, enabled: bool) -> Self {
        self.enable_encryption = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_encryption_key() {
        let key1 = generate_encryption_key();
        let key2 = generate_encryption_key();
        
        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
        assert_ne!(key1, key2); // Should be unique
    }

    #[test]
    fn test_security_config_creation() {
        let config = SecurityConfig::new();
        
        assert_eq!(config.encryption_key.len(), 32);
        assert!(config.enable_encryption);
        assert!(config.api_key.is_none());
    }
}