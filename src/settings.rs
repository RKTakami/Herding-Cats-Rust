use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub paper_size: String,
    pub margins: f32, // in inches
    pub line_spacing: f32,
    pub theme: Option<String>,
    pub font_size: Option<i32>,
    pub auto_save: Option<bool>,
    pub ai_model: Option<String>,
    pub api_key: Option<String>,
    pub enable_ai_suggestions: Option<bool>,
    pub enable_ai_analysis: Option<bool>,
    // Theme-specific settings
    pub theme_settings: Option<ThemeSettings>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeSettings {
    pub current_theme: String,
    pub theme_variant: Option<String>, // For custom themes
    pub use_system_theme: bool,
    pub theme_transition_enabled: bool,
    pub high_contrast_mode: bool,
    pub reduce_animations: bool,
    pub custom_colors: Option<CustomThemeColors>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomThemeColors {
    pub primary_bg: String,
    pub secondary_bg: String,
    pub accent: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub border: String,
    pub button_bg: String,
    pub button_text: String,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            current_theme: "Light".to_string(),
            theme_variant: None,
            use_system_theme: false,
            theme_transition_enabled: true,
            high_contrast_mode: false,
            reduce_animations: false,
            custom_colors: None,
        }
    }
}

impl Default for CustomThemeColors {
    fn default() -> Self {
        Self {
            primary_bg: "#ffffff".to_string(),
            secondary_bg: "#f8f9fa".to_string(),
            accent: "#007bff".to_string(),
            text_primary: "#212529".to_string(),
            text_secondary: "#6c757d".to_string(),
            border: "#dee2e6".to_string(),
            button_bg: "#007bff".to_string(),
            button_text: "#ffffff".to_string(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            paper_size: "Letter".to_string(),
            margins: 1.0,
            line_spacing: 1.0,
            theme: Some("Light".to_string()),
            font_size: Some(14),
            auto_save: Some(true),
            ai_model: Some("gpt-3".to_string()),
            api_key: None,
            enable_ai_suggestions: Some(false),
            enable_ai_analysis: Some(true),
            theme_settings: Some(ThemeSettings::default()),
        }
    }
}

/// Get the settings file path
fn get_settings_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("settings.json")
}

/// Get the theme settings file path
fn get_theme_settings_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("theme_settings.json")
}

/// Load theme settings from file
pub fn load_theme_settings() -> ThemeSettings {
    let theme_path = get_theme_settings_path();

    match std::fs::read_to_string(&theme_path) {
        Ok(content) => match serde_json::from_str::<ThemeSettings>(&content) {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Failed to parse theme settings file: {}, using defaults", e);
                ThemeSettings::default()
            }
        },
        Err(_) => {
            // File doesn't exist or can't be read, return default settings
            ThemeSettings::default()
        }
    }
}

/// Save theme settings to file
pub fn save_theme_settings(theme_settings: &ThemeSettings) -> Result<(), String> {
    let theme_path = get_theme_settings_path();

    let json = serde_json::to_string_pretty(theme_settings)
        .map_err(|e| format!("Failed to serialize theme settings: {}", e))?;

    std::fs::write(&theme_path, json)
        .map_err(|e| format!("Failed to write theme settings file: {}", e))
}

/// Apply theme settings to the application
pub fn apply_theme_settings(settings: &ThemeSettings) -> Result<(), String> {
    // This would integrate with the theme manager to apply the current theme
    // For now, we'll just validate the theme name
    match settings.current_theme.as_str() {
        "Light" | "Dark" | "High Contrast" => {
            // Built-in theme, should be available
            Ok(())
        }
        _ => {
            // Custom theme, check if it has custom colors defined
            if settings.custom_colors.is_some() {
                Ok(())
            } else {
                Err(format!(
                    "Unknown theme '{}' and no custom colors defined",
                    settings.current_theme
                ))
            }
        }
    }
}

/// Update current theme in settings
pub fn update_current_theme(theme_name: &str) -> Result<(), String> {
    let mut settings = load_theme_settings();
    settings.current_theme = theme_name.to_string();

    // Apply the theme settings
    apply_theme_settings(&settings)?;

    // Save the updated settings
    save_theme_settings(&settings)
}

/// Load settings from file system
pub fn load_settings() -> Settings {
    let settings_path = get_settings_path();

    match std::fs::read_to_string(&settings_path) {
        Ok(content) => match serde_json::from_str::<Settings>(&content) {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Failed to parse settings file: {}, using defaults", e);
                Settings::default()
            }
        },
        Err(_) => {
            // File doesn't exist or can't be read, return default settings
            Settings::default()
        }
    }
}

/// Save settings to file system
pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let settings_path = get_settings_path();

    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    std::fs::write(&settings_path, json)
        .map_err(|e| format!("Failed to write settings file: {}", e))
}
