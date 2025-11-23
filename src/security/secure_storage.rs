use anyhow::{Result, Context};
use keyring::Entry;

pub struct SecureStorageService {
    service_name: String,
}

impl SecureStorageService {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }

    pub fn set_api_key(&self, provider: &str, key: &str) -> Result<()> {
        let entry = Entry::new(&self.service_name, provider)?;
        entry.set_password(key)?;
        Ok(())
    }

    pub fn get_api_key(&self, provider: &str) -> Result<String> {
        let entry = Entry::new(&self.service_name, provider)?;
        let password = entry.get_password()?;
        Ok(password)
    }

    pub fn delete_api_key(&self, provider: &str) -> Result<()> {
        let entry = Entry::new(&self.service_name, provider)?;
        entry.delete_password()?;
        Ok(())
    }
}
