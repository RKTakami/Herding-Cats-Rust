use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::Rng;
use std::sync::{Arc, Mutex};

/// Core service trait for dependency injection
pub trait Service: Send + Sync {}

/// File service for handling file operations
#[derive(Clone, Debug)]
pub struct FileService;

impl Default for FileService {
    fn default() -> Self {
        Self::new()
    }
}

impl FileService {
    pub fn new() -> Self {
        FileService
    }

    pub async fn save_document(&self, content: String, path: String) -> Result<String, String> {
        crate::file_ops::save_document_impl(content, path).await?;
        Ok("Saved successfully".to_string())
    }

    pub async fn load_document(&self, path: String) -> Result<String, String> {
        crate::file_ops::load_document_impl(path).await
    }

    pub async fn import_project(
        &self,
        source_path: String,
        dest_path: String,
    ) -> Result<String, String> {
        crate::file_ops::import_project_impl(source_path, dest_path).await?;
        Ok("Import completed".to_string())
    }
}

impl Service for FileService {}

/// Settings service for configuration management
#[derive(Clone, Debug)]
pub struct SettingsService {
    // For standalone operation, we'll use a default settings path
    settings_path: std::path::PathBuf,
}

impl Default for SettingsService {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsService {
    pub fn new() -> Self {
        // Use a default settings path for standalone operation
        let settings_path = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("settings.json");
        SettingsService { settings_path }
    }

    pub fn get_settings(&self) -> crate::settings::Settings {
        // For standalone operation, we'll load from the default path
        match std::fs::read_to_string(&self.settings_path) {
            Ok(content) => {
                if let Ok(settings) = serde_json::from_str::<crate::settings::Settings>(&content) {
                    // API key decryption removed for standalone operation
                    // settings.api_key remains as loaded from JSON
                    settings
                } else {
                    crate::settings::Settings::default()
                }
            }
            Err(_) => crate::settings::Settings::default(),
        }
    }

    pub fn save_settings(&self, settings: &crate::settings::Settings) -> Result<(), String> {
        // Ensure directory exists
        if let Some(parent) = self.settings_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create settings directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(settings)
            .map_err(|e| format!("Serialization error: {}", e))?;

        std::fs::write(&self.settings_path, json).map_err(|e| format!("Write error: {}", e))
    }
}

impl Service for SettingsService {}

/// Writing tools service for UI tool management
#[derive(Clone, Debug)]
pub struct WritingToolsService {
    // For standalone operation, we'll implement these as no-op or stub functions
}

impl Default for WritingToolsService {
    fn default() -> Self {
        Self::new()
    }
}

impl WritingToolsService {
    pub fn new() -> Self {
        WritingToolsService {}
    }

    pub async fn open_writing_tool(&self, tool: String) -> Result<(), String> {
        // For standalone operation, we'll just log the tool request
        println!(
            "Writing tool '{}' requested (standalone mode - no UI action taken)",
            tool
        );
        Ok(())
    }

    pub async fn cycle_writing_tool(&self) -> Result<(), String> {
        // For standalone operation, this is a no-op
        println!("Writing tool cycling requested (standalone mode - no UI action taken)");
        Ok(())
    }

    pub async fn brainstorm_notes(&self, prompt: String) -> Result<String, String> {
        // For standalone operation, return a stub response
        println!("Brainstorm requested with prompt: {}", prompt);
        Ok(format!("Brainstorm result for: {}\n\n- Idea 1: Implement this feature\n- Idea 2: Add more functionality\n- Idea 3: Consider edge cases", prompt))
    }
}

impl Service for WritingToolsService {}

/// Security service for cryptographic operations and secure utilities
#[derive(Clone, Debug)]
pub struct SecurityService;

impl Default for SecurityService {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityService {
    pub fn new() -> Self {
        SecurityService
    }

    /// Generate a cryptographically secure random string of specified length
    pub fn secure_random_string(&self, length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Generate a secure hash using SHA-256
    pub fn secure_hash(&self, data: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Encrypt data using AES-GCM
    pub fn encrypt_data(&self, data: &str, key: &[u8; 32]) -> Result<Vec<u8>, String> {
        use aes_gcm::aead::{Aead, KeyInit};

        let encryption_key = Key::<Aes256Gcm>::from_slice(key);
        let nonce = Nonce::from_slice(b"unique nonce");
        let cipher = Aes256Gcm::new(encryption_key);

        cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))
    }

    /// Decrypt data using AES-GCM
    pub fn decrypt_data(&self, encrypted_data: &[u8], key: &[u8; 32]) -> Result<String, String> {
        use aes_gcm::aead::{Aead, KeyInit};

        let encryption_key = Key::<Aes256Gcm>::from_slice(key);
        let nonce = Nonce::from_slice(b"unique nonce");
        let cipher = Aes256Gcm::new(encryption_key);

        let decrypted = cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| format!("Decryption failed: {}", e))?;

        String::from_utf8(decrypted).map_err(|e| format!("Invalid UTF-8 in decrypted data: {}", e))
    }

    /// Validate API key format (basic security check)
    pub fn validate_api_key_format(&self, api_key: &str) -> bool {
        if api_key.is_empty() || api_key.trim().is_empty() {
            return false;
        }

        // Check for basic injection patterns
        let dangerous_patterns = [
            "<script",
            "javascript:",
            "vbscript:",
            "data:",
            "onerror=",
            "onclick=",
        ];
        if dangerous_patterns
            .iter()
            .any(|&pattern| api_key.to_lowercase().contains(pattern))
        {
            return false;
        }

        // API keys should be reasonably long and contain alphanumeric characters
        api_key.len() >= 10
            && api_key
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    }
}

impl Service for SecurityService {}

/// Service registry for dependency injection
#[derive(Clone, Debug)]
pub struct ServiceRegistry {
    file_service: Arc<FileService>,
    settings_service: Arc<Mutex<Option<SettingsService>>>,
    writing_tools_service: Arc<Mutex<Option<WritingToolsService>>>,
    security_service: Arc<SecurityService>,
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistry {
    pub fn new() -> Self {
        ServiceRegistry {
            file_service: Arc::new(FileService::new()),
            settings_service: Arc::new(Mutex::new(None)),
            writing_tools_service: Arc::new(Mutex::new(None)),
            security_service: Arc::new(SecurityService::new()),
        }
    }

    pub fn initialize_with_app(&self) {
        let mut guard = self
            .settings_service
            .lock()
            .expect("Settings service poisoned");
        *guard = Some(SettingsService::new());
        let mut guard = self
            .writing_tools_service
            .lock()
            .expect("Writing tools service poisoned");
        *guard = Some(WritingToolsService::new());
    }

    pub fn file_service(&self) -> Arc<FileService> {
        Arc::clone(&self.file_service)
    }

    pub fn settings_service(&self) -> SettingsService {
        self.settings_service
            .lock()
            .expect("Settings service poisoned")
            .as_ref()
            .cloned()
            .expect("No settings service")
    }

    pub fn writing_tools_service(&self) -> WritingToolsService {
        self.writing_tools_service
            .lock()
            .expect("Writing tools service poisoned")
            .as_ref()
            .cloned()
            .expect("No writing tools service")
    }

    pub fn security_service(&self) -> Arc<SecurityService> {
        Arc::clone(&self.security_service)
    }
}
