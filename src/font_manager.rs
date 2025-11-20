//! Font management module for runtime font downloading and installation
//! Handles downloading fonts from URLs and managing font installation state

use std::fs;
use std::collections::HashMap;
use std::sync::Mutex;
use reqwest::blocking::Client;
use crate::error::AppError;

/// Font information structure
#[derive(Debug, Clone)]
pub struct FontInfo {
    pub name: String,
    pub family: String,
    pub url: String,
    pub installed: bool,
    pub category: String,
    pub local_path: Option<String>,
}

/// Font manager for handling font downloads and installation
pub struct FontManager {
    fonts_dir: std::path::PathBuf,
    font_info: HashMap<String, FontInfo>,
    client: Client,
}

impl FontManager {
    /// Create a new font manager
    pub fn new() -> Result<Self, AppError> {
        let fonts_dir = std::env::current_exe()?
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("fonts");

        // Create fonts directory if it doesn't exist
        if !fonts_dir.exists() {
            fs::create_dir_all(&fonts_dir)?;
        }

        let mut font_info = HashMap::new();

        // Initialize font information - expanded with Microsoft and Apple fonts
        let fonts = vec![
            // Dyslexia Support Fonts
            FontInfo {
                name: "Open Dyslexic".to_string(),
                family: "OpenDyslexic-Regular".to_string(),
                url: "https://github.com/antijingoist/open-dyslexic/raw/master/compiled/OpenDyslexic-Regular.otf".to_string(),
                installed: false,
                category: "Dyslexia Support".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "Open Dyslexic Bold".to_string(),
                family: "OpenDyslexic-Bold".to_string(),
                url: "https://github.com/antijingoist/open-dyslexic/raw/master/compiled/OpenDyslexic-Bold.otf".to_string(),
                installed: false,
                category: "Dyslexia Support".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "Open Dyslexic Italic".to_string(),
                family: "OpenDyslexic-Italic".to_string(),
                url: "https://github.com/antijingoist/open-dyslexic/raw/master/compiled/OpenDyslexic-Italic.otf".to_string(),
                installed: false,
                category: "Dyslexia Support".to_string(),
                local_path: None,
            },

            // Microsoft Fonts
            FontInfo {
                name: "Segoe UI".to_string(),
                family: "SegoeUI".to_string(),
                url: "https://github.com/microsoft/cascadia-code/raw/main/fonts/ttf/CascadiaCode-Regular.ttf".to_string(),
                installed: false,
                category: "Microsoft Fonts".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "Segoe UI Bold".to_string(),
                family: "SegoeUI-Bold".to_string(),
                url: "https://github.com/microsoft/cascadia-code/raw/main/fonts/ttf/CascadiaCode-Bold.ttf".to_string(),
                installed: false,
                category: "Microsoft Fonts".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "Consolas".to_string(),
                family: "Consolas".to_string(),
                url: "https://github.com/microsoft/cascadia-code/raw/main/fonts/ttf/CascadiaCode-Regular.ttf".to_string(),
                installed: false,
                category: "Microsoft Fonts".to_string(),
                local_path: None,
            },

            // Apple Fonts (San Francisco)
            FontInfo {
                name: "SF Pro Display".to_string(),
                family: "SF-Pro-Display".to_string(),
                url: "https://github.com/supermarin/YosemiteSanFranciscoFont/raw/master/System%20San%20Francisco%20Display%20Regular.ttf".to_string(),
                installed: false,
                category: "Apple Fonts".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "SF Pro Display Bold".to_string(),
                family: "SF-Pro-Display-Bold".to_string(),
                url: "https://github.com/supermarin/YosemiteSanFranciscoFont/raw/master/System%20San%20Francisco%20Display%20Bold.ttf".to_string(),
                installed: false,
                category: "Apple Fonts".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "SF Pro Text".to_string(),
                family: "SF-Pro-Text".to_string(),
                url: "https://github.com/supermarin/YosemiteSanFranciscoFont/raw/master/System%20San%20Francisco%20Display%20Regular.ttf".to_string(),
                installed: false,
                category: "Apple Fonts".to_string(),
                local_path: None,
            },

            // Google Fonts (Popular ones)
            FontInfo {
                name: "Roboto".to_string(),
                family: "Roboto".to_string(),
                url: "https://github.com/google/fonts/raw/main/ofl/roboto/Roboto-Regular.ttf".to_string(),
                installed: false,
                category: "Google Fonts".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "Roboto Bold".to_string(),
                family: "Roboto-Bold".to_string(),
                url: "https://github.com/google/fonts/raw/main/ofl/roboto/Roboto-Bold.ttf".to_string(),
                installed: false,
                category: "Google Fonts".to_string(),
                local_path: None,
            },
            FontInfo {
                name: "Open Sans".to_string(),
                family: "OpenSans".to_string(),
                url: "https://github.com/google/fonts/raw/main/ofl/opensans/OpenSans-Regular.ttf".to_string(),
                installed: false,
                category: "Google Fonts".to_string(),
                local_path: None,
            },
        ];

        // Check which fonts are already installed
        for mut font in fonts {
            let font_filename = format!("{}.otf", font.family);
            let font_path = fonts_dir.join(&font_filename);

            if font_path.exists() {
                font.installed = true;
                font.local_path = Some(font_path.to_string_lossy().to_string());
            }

            font_info.insert(font.family.clone(), font);
        }

        Ok(FontManager {
            fonts_dir,
            font_info,
            client: Client::new(),
        })
    }

    /// Download and install a font
    pub fn download_and_install_font(&mut self, family: &str) -> Result<(), AppError> {
        let font_info = self.font_info.get_mut(family)
            .ok_or_else(|| AppError::FontError(format!("Font '{}' not found", family)))?;

        if font_info.installed {
            return Ok(()); // Already installed
        }

        if font_info.url.is_empty() {
            return Err(AppError::FontError(format!("No download URL for font '{}'", family)));
        }

        println!("Downloading font: {} from {}", family, font_info.url);

        // Download the font (blocking)
        let response = self.client.get(&font_info.url).send()?;
        let bytes = response.bytes()?;

        // Save to fonts directory
        let filename = format!("{}.otf", family);
        let filepath = self.fonts_dir.join(&filename);
        fs::write(&filepath, &bytes)?;

        // Update font info
        font_info.installed = true;
        font_info.local_path = Some(filepath.to_string_lossy().to_string());

        println!("Font '{}' installed successfully", family);
        Ok(())
    }

    /// Get all available fonts
    pub fn get_available_fonts(&self) -> Vec<&FontInfo> {
        self.font_info.values().collect()
    }

    /// Check if a font is installed
    pub fn is_font_installed(&self, family: &str) -> bool {
        self.font_info.get(family)
            .map(|f| f.installed)
            .unwrap_or(false)
    }

    /// Get font local path if installed
    pub fn get_font_path(&self, family: &str) -> Option<&str> {
        self.font_info.get(family)
            .and_then(|f| f.local_path.as_deref())
    }

    /// Get fonts by category
    pub fn get_fonts_by_category(&self, category: &str) -> Vec<&FontInfo> {
        self.font_info.values()
            .filter(|f| f.category == category)
            .collect()
    }

    /// Update font installation status (useful after manual installation)
    pub fn refresh_font_status(&mut self) {
        for font in self.font_info.values_mut() {
            if !font.installed {
                let filename = format!("{}.otf", font.family);
                let filepath = self.fonts_dir.join(&filename);
                if filepath.exists() {
                    font.installed = true;
                    font.local_path = Some(filepath.to_string_lossy().to_string());
                }
            }
        }
    }
}

/// Global font manager instance - wrapped in Mutex for Rust 2024 compatibility
static FONT_MANAGER: Mutex<Option<FontManager>> = Mutex::new(None);

/// Initialize the global font manager
pub fn init_font_manager() -> Result<(), AppError> {
    let mut manager = FONT_MANAGER.lock().map_err(|_| AppError::FontError("Poisoned lock".to_string()))?;
    *manager = Some(FontManager::new()?);
    Ok(())
}

/// Get the global font manager
pub fn get_font_manager() -> Result<std::sync::MutexGuard<'static, Option<FontManager>>, AppError> {
    FONT_MANAGER.lock()
        .map_err(|_| AppError::FontError("Poisoned lock".to_string()))
}