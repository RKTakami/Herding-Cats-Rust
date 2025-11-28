//! Centralized Theme Management System
//!
//! Provides unified theming capabilities across all application windows
//! including main app window and individual writing tool windows.

use crate as hc_lib;
use hc_lib::{load_theme_settings, save_theme_settings, ThemeSettings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Theme color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    /// Primary background color
    pub primary_bg: String,
    /// Secondary background color
    pub secondary_bg: String,
    /// Accent color for highlights
    pub accent: String,
    /// Primary text color
    pub text_primary: String,
    /// Secondary text color
    pub text_secondary: String,
    /// Accent text color
    pub text_accent: String,
    /// Border and separator color
    pub border: String,
    /// Button background color
    pub button_bg: String,
    /// Button text color
    pub button_text: String,
    /// Button hover color
    pub button_hover: String,
    /// Menu background color
    pub menu_bg: String,
    /// Toolbar background color
    pub toolbar_bg: String,
    /// Status bar background color
    pub status_bg: String,
    /// Editor background color
    pub editor_bg: String,
    /// Title bar background color
    pub title_bg: String,
    /// Window background color
    pub window_bg: String,
    /// Panel background color
    pub panel_bg: String,
    /// Input field background
    pub input_bg: String,
    /// Input text color
    pub input_text: String,
    /// Input border color
    pub input_border: String,
    /// Input focus border color
    pub input_focus: String,
    /// Border radius for buttons and containers
    pub border_radius: f32,
    /// Menu border radius
    pub menu_border_radius: f32,
    /// Container border radius
    pub container_border_radius: f32,
    /// Input field border radius
    pub input_border_radius: f32,
    /// Ribbon background color
    pub ribbon_bg: String,
    /// Dropdown background color
    pub dropdown_bg: String,
}

impl ThemeColors {
    /// Convert color string to Slint-compatible format
    pub fn to_slint_color(&self, color_field: &str) -> String {
        match color_field {
            "primary_bg" => self.primary_bg.clone(),
            "secondary_bg" => self.secondary_bg.clone(),
            "accent" => self.accent.clone(),
            "text_primary" => self.text_primary.clone(),
            "text_secondary" => self.text_secondary.clone(),
            "text_accent" => self.text_accent.clone(),
            "border" => self.border.clone(),
            "button_bg" => self.button_bg.clone(),
            "button_text" => self.button_text.clone(),
            "button_hover" => self.button_hover.clone(),
            "menu_bg" => self.menu_bg.clone(),
            "toolbar_bg" => self.toolbar_bg.clone(),
            "status_bg" => self.status_bg.clone(),
            "editor_bg" => self.editor_bg.clone(),
            "title_bg" => self.title_bg.clone(),
            "window_bg" => self.window_bg.clone(),
            "panel_bg" => self.panel_bg.clone(),
            "input_bg" => self.input_bg.clone(),
            "input_text" => self.input_text.clone(),
            "input_border" => self.input_border.clone(),
            "input_focus" => self.input_focus.clone(),
            "ribbon_bg" => self.ribbon_bg.clone(),
            "dropdown_bg" => self.dropdown_bg.clone(),
            _ => "#ffffff".to_string(), // Default fallback
        }
    }

    /// Get border radius value for Slint components
    pub fn get_border_radius(&self, radius_type: &str) -> f32 {
        match radius_type {
            "button" => self.border_radius,
            "menu" => self.menu_border_radius,
            "container" => self.container_border_radius,
            "input" => self.input_border_radius,
            _ => self.border_radius, // Default to general border radius
        }
    }

    /// Convert border radius to Slint-compatible string format
    pub fn to_slint_radius(&self, radius_type: &str) -> String {
        format!("{}px", self.get_border_radius(radius_type))
    }
}

/// Predefined theme types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ThemeType {
    Light,
    Dark,
    HighContrast,
    Custom(String), // Custom theme name
}

impl ThemeType {
    pub fn as_str(&self) -> String {
        match self {
            ThemeType::Light => "Light".to_string(),
            ThemeType::Dark => "Dark".to_string(),
            ThemeType::HighContrast => "High Contrast".to_string(),
            ThemeType::Custom(name) => name.clone(),
        }
    }
}

impl std::fmt::Display for ThemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub theme_type: ThemeType,
    pub colors: ThemeColors,
    pub description: String,
    pub author: String,
    pub version: String,
}

impl Theme {
    /// Create a light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            theme_type: ThemeType::Light,
            description: "Clean and bright light theme".to_string(),
            author: "Herding Cats Team".to_string(),
            version: "1.0".to_string(),
            colors: ThemeColors {
                primary_bg: "#ffffff".to_string(),
                secondary_bg: "#f8f9fa".to_string(),
                accent: "#007bff".to_string(),
                text_primary: "#212529".to_string(),
                text_secondary: "#6c757d".to_string(),
                text_accent: "#007bff".to_string(),
                border: "#dee2e6".to_string(),
                button_bg: "#007bff".to_string(),
                button_text: "#ffffff".to_string(),
                button_hover: "#0056b3".to_string(),
                menu_bg: "#343a40".to_string(),
                toolbar_bg: "#f8f9fa".to_string(),
                status_bg: "#f8f9fa".to_string(),
                editor_bg: "#ffffff".to_string(),
                title_bg: "#e9ecef".to_string(),
                window_bg: "#ffffff".to_string(),
                panel_bg: "#ffffff".to_string(),
                input_bg: "#ffffff".to_string(),
                input_text: "#212529".to_string(),
                input_border: "#ced4da".to_string(),
                input_focus: "#80bdff".to_string(),
                border_radius: 4.0,
                menu_border_radius: 3.0,
                container_border_radius: 6.0,
                input_border_radius: 4.0,
                ribbon_bg: "#f8f9fa".to_string(),
                dropdown_bg: "#ffffff".to_string(),
            },
        }
    }

    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            theme_type: ThemeType::Dark,
            description: "Sleek and modern dark theme".to_string(),
            author: "Herding Cats Team".to_string(),
            version: "1.0".to_string(),
            colors: ThemeColors {
                primary_bg: "#212529".to_string(),
                secondary_bg: "#343a40".to_string(),
                accent: "#6610f2".to_string(),
                text_primary: "#ffffff".to_string(),
                text_secondary: "#adb5bd".to_string(),
                text_accent: "#6610f2".to_string(),
                border: "#495057".to_string(),
                button_bg: "#6610f2".to_string(),
                button_text: "#ffffff".to_string(),
                button_hover: "#5a0ee0".to_string(),
                menu_bg: "#121212".to_string(),
                toolbar_bg: "#343a40".to_string(),
                status_bg: "#343a40".to_string(),
                editor_bg: "#212529".to_string(),
                title_bg: "#495057".to_string(),
                window_bg: "#212529".to_string(),
                panel_bg: "#343a40".to_string(),
                input_bg: "#495057".to_string(),
                input_text: "#ffffff".to_string(),
                input_border: "#6c757d".to_string(),
                input_focus: "#8b949e".to_string(),
                border_radius: 4.0,
                menu_border_radius: 3.0,
                container_border_radius: 6.0,
                input_border_radius: 4.0,
                ribbon_bg: "#343a40".to_string(),
                dropdown_bg: "#212529".to_string(),
            },
        }
    }

    /// Create a high contrast theme
    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            theme_type: ThemeType::HighContrast,
            description: "High contrast theme for accessibility".to_string(),
            author: "Herding Cats Team".to_string(),
            version: "1.0".to_string(),
            colors: ThemeColors {
                primary_bg: "#ffffff".to_string(),
                secondary_bg: "#ffffff".to_string(),
                accent: "#000000".to_string(),
                text_primary: "#000000".to_string(),
                text_secondary: "#666666".to_string(),
                text_accent: "#ff0000".to_string(),
                border: "#000000".to_string(),
                button_bg: "#000000".to_string(),
                button_text: "#ffffff".to_string(),
                button_hover: "#333333".to_string(),
                menu_bg: "#000000".to_string(),
                toolbar_bg: "#ffffff".to_string(),
                status_bg: "#ffffff".to_string(),
                editor_bg: "#ffffff".to_string(),
                title_bg: "#000000".to_string(),
                window_bg: "#ffffff".to_string(),
                panel_bg: "#ffffff".to_string(),
                input_bg: "#ffffff".to_string(),
                input_text: "#000000".to_string(),
                input_border: "#000000".to_string(),
                input_focus: "#ff0000".to_string(),
                border_radius: 2.0, // High contrast: subtle rounding
                menu_border_radius: 2.0,
                container_border_radius: 4.0,
                input_border_radius: 2.0,
                ribbon_bg: "#ffffff".to_string(),
                dropdown_bg: "#ffffff".to_string(),
            },
        }
    }

    /// Create a theme from custom colors
    pub fn from_custom_colors(name: &str, colors: ThemeColors) -> Self {
        Self {
            name: name.to_string(),
            theme_type: ThemeType::Custom(name.to_string()),
            description: format!("Custom theme: {}", name).to_string(),
            author: "User".to_string(),
            version: "1.0".to_string(),
            colors,
        }
    }

    /// Create a warm theme (Sepia)
    pub fn warm() -> Self {
        Self {
            name: "Warm".to_string(),
            theme_type: ThemeType::Custom("Warm".to_string()),
            description: "Warm sepia theme for comfortable reading".to_string(),
            author: "Herding Cats Team".to_string(),
            version: "1.0".to_string(),
            colors: ThemeColors {
                primary_bg: "#fdf6e3".to_string(),
                secondary_bg: "#eee8d5".to_string(),
                accent: "#cb4b16".to_string(),
                text_primary: "#586e75".to_string(),
                text_secondary: "#93a1a1".to_string(),
                text_accent: "#cb4b16".to_string(),
                border: "#d3cbb7".to_string(),
                button_bg: "#cb4b16".to_string(),
                button_text: "#ffffff".to_string(),
                button_hover: "#dc322f".to_string(),
                menu_bg: "#eee8d5".to_string(),
                toolbar_bg: "#fdf6e3".to_string(),
                status_bg: "#eee8d5".to_string(),
                editor_bg: "#fdf6e3".to_string(),
                title_bg: "#eee8d5".to_string(),
                window_bg: "#fdf6e3".to_string(),
                panel_bg: "#eee8d5".to_string(),
                input_bg: "#fdf6e3".to_string(),
                input_text: "#586e75".to_string(),
                input_border: "#d3cbb7".to_string(),
                input_focus: "#cb4b16".to_string(),
                border_radius: 4.0,
                menu_border_radius: 3.0,
                container_border_radius: 6.0,
                input_border_radius: 4.0,
                ribbon_bg: "#fdf6e3".to_string(),
                dropdown_bg: "#fdf6e3".to_string(),
            },
        }
    }

    /// Create a CRT Green theme
    pub fn crt_green() -> Self {
        Self {
            name: "CRT Green".to_string(),
            theme_type: ThemeType::Custom("CRT Green".to_string()),
            description: "Retro CRT monitor green theme".to_string(),
            author: "Herding Cats Team".to_string(),
            version: "1.0".to_string(),
            colors: ThemeColors {
                primary_bg: "#0d1117".to_string(),
                secondary_bg: "#161b22".to_string(),
                accent: "#00ff00".to_string(),
                text_primary: "#00ff00".to_string(),
                text_secondary: "#008f00".to_string(),
                text_accent: "#00ff00".to_string(),
                border: "#30363d".to_string(),
                button_bg: "#00ff00".to_string(),
                button_text: "#0d1117".to_string(),
                button_hover: "#00cc00".to_string(),
                menu_bg: "#161b22".to_string(),
                toolbar_bg: "#0d1117".to_string(),
                status_bg: "#161b22".to_string(),
                editor_bg: "#0d1117".to_string(),
                title_bg: "#161b22".to_string(),
                window_bg: "#0d1117".to_string(),
                panel_bg: "#161b22".to_string(),
                input_bg: "#0d1117".to_string(),
                input_text: "#00ff00".to_string(),
                input_border: "#30363d".to_string(),
                input_focus: "#00ff00".to_string(),
                border_radius: 0.0,
                menu_border_radius: 0.0,
                container_border_radius: 0.0,
                input_border_radius: 0.0,
                ribbon_bg: "#0d1117".to_string(),
                dropdown_bg: "#161b22".to_string(),
            },
        }
    }

    /// Create a CRT Amber theme
    pub fn crt_amber() -> Self {
        Self {
            name: "CRT Amber".to_string(),
            theme_type: ThemeType::Custom("CRT Amber".to_string()),
            description: "Retro CRT monitor amber theme".to_string(),
            author: "Herding Cats Team".to_string(),
            version: "1.0".to_string(),
            colors: ThemeColors {
                primary_bg: "#1a1a1a".to_string(),
                secondary_bg: "#2b2b2b".to_string(),
                accent: "#ffb000".to_string(),
                text_primary: "#ffb000".to_string(),
                text_secondary: "#b37b00".to_string(),
                text_accent: "#ffb000".to_string(),
                border: "#4d4d4d".to_string(),
                button_bg: "#ffb000".to_string(),
                button_text: "#1a1a1a".to_string(),
                button_hover: "#cc8d00".to_string(),
                menu_bg: "#2b2b2b".to_string(),
                toolbar_bg: "#1a1a1a".to_string(),
                status_bg: "#2b2b2b".to_string(),
                editor_bg: "#1a1a1a".to_string(),
                title_bg: "#2b2b2b".to_string(),
                window_bg: "#1a1a1a".to_string(),
                panel_bg: "#2b2b2b".to_string(),
                input_bg: "#1a1a1a".to_string(),
                input_text: "#ffb000".to_string(),
                input_border: "#4d4d4d".to_string(),
                input_focus: "#ffb000".to_string(),
                border_radius: 0.0,
                menu_border_radius: 0.0,
                container_border_radius: 0.0,
                input_border_radius: 0.0,
                ribbon_bg: "#1a1a1a".to_string(),
                dropdown_bg: "#2b2b2b".to_string(),
            },
        }
    }
}

/// Theme manager for handling theme switching and management
pub struct ThemeManager {
    /// Available themes
    themes: Arc<Mutex<HashMap<ThemeType, Theme>>>,
    /// Current active theme
    current_theme: Arc<Mutex<Theme>>,
    /// Theme change callbacks
    theme_change_callbacks: Arc<Mutex<Vec<Box<dyn Fn(&Theme) + Send>>>>,
    /// Settings integration
    settings: Arc<Mutex<ThemeSettings>>,
}

impl ThemeManager {
    /// Create a new theme manager with default themes
    pub fn new() -> Self {
        let mut themes_map = HashMap::new();

        // Add default themes
        themes_map.insert(ThemeType::Light, Theme::light());
        themes_map.insert(ThemeType::Dark, Theme::dark());
        themes_map.insert(ThemeType::HighContrast, Theme::high_contrast());
        
        // Add custom themes that match Slint UI
        let warm = Theme::warm();
        themes_map.insert(warm.theme_type.clone(), warm);
        
        let crt_green = Theme::crt_green();
        themes_map.insert(crt_green.theme_type.clone(), crt_green);
        
        let crt_amber = Theme::crt_amber();
        themes_map.insert(crt_amber.theme_type.clone(), crt_amber);

        // Note: Minimalist themes are defined in minimalist_theme.rs module
        // to avoid circular dependencies

        // Load settings
        let settings = load_theme_settings();

        // Create theme manager instance
        let mut manager = Self {
            themes: Arc::new(Mutex::new(themes_map)),
            current_theme: Arc::new(Mutex::new(Theme::light())),
            theme_change_callbacks: Arc::new(Mutex::new(Vec::new())),
            settings: Arc::new(Mutex::new(settings)),
        };

        // Initialize with saved theme
        manager.initialize_from_settings();

        manager
    }

    /// Initialize theme manager from settings
    fn initialize_from_settings(&self) {
        let theme_name = {
            let settings = self.settings.lock().unwrap();
            settings.current_theme.clone()
        };

        // Try to load the saved theme
        if let Ok(()) = self.set_theme_by_name(&theme_name) {
            log::info!(
                "Initialized theme manager with theme: {}",
                theme_name
            );
        } else {
            log::warn!(
                "Failed to load saved theme: {}, using default",
                theme_name
            );
        }
    }

    /// Set theme by name (string lookup)
    pub fn set_theme_by_name(&self, theme_name: &str) -> Result<(), String> {
        // First try built-in themes
        match theme_name {
            "Light" => self.set_theme(ThemeType::Light),
            "Dark" => self.set_theme(ThemeType::Dark),
            "High Contrast" => self.set_theme(ThemeType::HighContrast),
            "Minimalist Light" => self.set_theme(ThemeType::Custom("Minimalist Light".to_string())),
            "Minimalist Dark" => self.set_theme(ThemeType::Custom("Minimalist Dark".to_string())),
            _ => {
                // Try to find custom theme
                let themes = self.themes.lock().unwrap();
                if let Some(theme) = themes.values().find(|t| t.name == theme_name) {
                    let mut current = self.current_theme.lock().unwrap();
                    *current = theme.clone();
                    // Drop locks before notifying to avoid deadlocks
                    drop(current);
                    drop(themes);
                    
                    // We need to get the theme again to pass reference, or clone it before dropping lock
                    // Let's clone it inside the lock
                    let theme_clone = {
                        let themes = self.themes.lock().unwrap();
                        themes.values().find(|t| t.name == theme_name).unwrap().clone()
                    };
                    
                    self.notify_theme_change(&theme_clone);
                    Ok(())
                } else {
                    Err(format!("Theme '{}' not found", theme_name))
                }
            }
        }
    }

    // ... (skipping create_custom_theme for brevity as it's less critical)

    /// Get theme colors for current theme
    pub fn get_colors(&self) -> ThemeColors {
        self.current_theme.lock().unwrap().colors.clone()
    }

    /// Create and add a custom theme from colors
    pub fn create_custom_theme(&self, name: &str, colors: ThemeColors) -> Result<(), String> {
        let theme = Theme::from_custom_colors(name, colors);
        self.add_theme(theme.clone());

        // Save to settings
        let mut settings = self.settings.lock().unwrap();
        settings.current_theme = name.to_string();
        if let Err(e) = save_theme_settings(&settings) {
            log::warn!("Failed to save theme settings: {}", e);
        }

        // Switch to the new theme
        self.set_theme(ThemeType::Custom(name.to_string()))
    }

    /// Get all available theme types
    pub fn get_available_themes(&self) -> Vec<ThemeType> {
        self.themes.lock().unwrap().keys().cloned().collect()
    }

    /// Get all available theme names
    pub fn get_available_theme_names(&self) -> Vec<String> {
        self.themes.lock().unwrap().values().map(|t| t.name.clone()).collect()
    }

    /// Get available theme names (static method for external use)
    pub fn get_available_themes_list() -> Vec<String> {
        let manager = get_theme_manager();
        manager.get_available_theme_names()
    }

    /// Set theme by name (static method for external use)
    pub fn set_theme_by_name_static(theme_name: &str) -> Result<(), String> {
        let manager = get_theme_manager();
        manager.set_theme_by_name(theme_name)
    }

    /// Get theme by type
    pub fn get_theme(&self, theme_type: &ThemeType) -> Option<Theme> {
        self.themes.lock().unwrap().get(theme_type).cloned()
    }

    /// Get theme by name
    pub fn get_theme_by_name(&self, theme_name: &str) -> Option<Theme> {
        self.themes.lock().unwrap().values().find(|t| t.name == theme_name).cloned()
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> Theme {
        self.current_theme.lock().unwrap().clone()
    }

    /// Get current theme name
    pub fn get_current_theme_name(&self) -> String {
        self.current_theme.lock().unwrap().name.clone()
    }

    /// Set current theme
    pub fn set_theme(&self, theme_type: ThemeType) -> Result<(), String> {
        let themes = self.themes.lock().unwrap();
        if let Some(theme) = themes.get(&theme_type) {
            let mut current = self.current_theme.lock().unwrap();
            *current = theme.clone();

            // Update settings
            {
                let mut settings = self.settings.lock().unwrap();
                settings.current_theme = theme.name.clone();
                if let Err(e) = save_theme_settings(&settings) {
                    log::warn!("Failed to save theme settings: {}", e);
                }
            }
            
            let theme_clone = theme.clone();
            drop(current);
            drop(themes);

            // Notify callbacks
            self.notify_theme_change(&theme_clone);
            Ok(())
        } else {
            Err(format!("Theme {:?} not found", theme_type))
        }
    }

    /// Add or update a custom theme
    pub fn add_theme(&self, theme: Theme) {
        self.themes.lock().unwrap().insert(theme.theme_type.clone(), theme);
    }

    /// Remove a custom theme
    pub fn remove_theme(&self, theme_type: &ThemeType) -> bool {
        if matches!(theme_type, ThemeType::Custom(_)) {
            self.themes.lock().unwrap().remove(theme_type).is_some()
        } else {
            false // Don't allow removing built-in themes
        }
    }

    /// Get theme settings
    pub fn get_theme_settings(&self) -> ThemeSettings {
        self.settings.lock().unwrap().clone()
    }

    /// Update theme settings
    pub fn update_theme_settings(&self, settings: ThemeSettings) -> Result<(), String> {
        let mut current_settings = self.settings.lock().unwrap();
        *current_settings = settings.clone();
        save_theme_settings(&current_settings)
    }

    /// Register a callback for theme changes
    pub fn on_theme_change<F>(&self, callback: F)
    where
        F: Fn(&Theme) + Send + 'static,
    {
        self.theme_change_callbacks
            .lock()
            .unwrap()
            .push(Box::new(callback));
    }

    /// Notify all callbacks about theme change
    fn notify_theme_change(&self, theme: &Theme) {
        let callbacks = self.theme_change_callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback(theme);
        }
    }

    /// Get theme colors for current theme


    /// Get a specific color from current theme
    pub fn get_color(&self, color_name: &str) -> String {
        let colors = self.get_colors();
        colors.to_slint_color(color_name)
    }

    /// Get border radius value from current theme
    pub fn get_radius(&self, radius_type: &str) -> f32 {
        let colors = self.get_colors();
        colors.get_border_radius(radius_type)
    }

    /// Get border radius as Slint-compatible string from current theme
    pub fn get_radius_string(&self, radius_type: &str) -> String {
        let colors = self.get_colors();
        colors.to_slint_radius(radius_type)
    }
}

/// Global theme manager instance
lazy_static::lazy_static! {
    pub static ref THEME_MANAGER: Arc<ThemeManager> = {
        Arc::new(ThemeManager::new())
    };
}

/// Get a reference to the global theme manager
pub fn get_theme_manager() -> Arc<ThemeManager> {
    // println!("🔒 [ThemeManager] Accessing global instance...");
    let manager = THEME_MANAGER.clone();
    // println!("🔓 [ThemeManager] Global instance accessed.");
    manager
}

/// Get current theme colors
pub fn get_current_theme_colors() -> ThemeColors {
    println!("🎨 [get_current_theme_colors] Calling get_theme_manager()...");
    let manager = get_theme_manager();
    println!("🎨 [get_current_theme_colors] Manager acquired. Calling get_colors()...");
    let colors = manager.get_colors();
    println!("🎨 [get_current_theme_colors] Colors acquired.");
    colors
}

/// Get a specific color from current theme
pub fn get_theme_color(color_name: &str) -> String {
    get_theme_manager().get_color(color_name)
}

/// Get border radius value from current theme
pub fn get_theme_radius(radius_type: &str) -> f32 {
    get_theme_manager().get_radius(radius_type)
}

/// Get border radius as Slint-compatible string from current theme
pub fn get_theme_radius_string(radius_type: &str) -> String {
    get_theme_manager().get_radius_string(radius_type)
}

/// Get available theme names (module-level function)
pub fn get_available_theme_names() -> Vec<String> {
    get_theme_manager().get_available_theme_names()
}

/// Set theme by name (module-level function)
pub fn set_theme_by_name(theme_name: &str) -> Result<(), String> {
    get_theme_manager().set_theme_by_name(theme_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemeManager::new();
        assert_eq!(manager.get_available_themes().len(), 3);
        assert!(manager.get_theme(&ThemeType::Light).is_some());
        assert!(manager.get_theme(&ThemeType::Dark).is_some());
        assert!(manager.get_theme(&ThemeType::HighContrast).is_some());
    }

    #[test]
    fn test_theme_switching() {
        let manager = ThemeManager::new();

        // Switch to dark theme
        assert!(manager.set_theme(ThemeType::Dark).is_ok());
        let current = manager.get_current_theme();
        assert_eq!(current.theme_type, ThemeType::Dark);

        // Switch to light theme
        assert!(manager.set_theme(ThemeType::Light).is_ok());
        let current = manager.get_current_theme();
        assert_eq!(current.theme_type, ThemeType::Light);
    }

    #[test]
    fn test_invalid_theme_switch() {
        let manager = ThemeManager::new();
        let result = manager.set_theme(ThemeType::Custom("nonexistent".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_theme() {
        let manager = ThemeManager::new();

        let custom_theme = Theme {
            name: "Custom".to_string(),
            theme_type: ThemeType::Custom("custom".to_string()),
            description: "Test custom theme".to_string(),
            author: "Test".to_string(),
            version: "1.0".to_string(),
            colors: ThemeColors {
                primary_bg: "#ff0000".to_string(),
                secondary_bg: "#00ff00".to_string(),
                accent: "#0000ff".to_string(),
                text_primary: "#ffffff".to_string(),
                text_secondary: "#cccccc".to_string(),
                text_accent: "#ffff00".to_string(),
                border: "#000000".to_string(),
                button_bg: "#888888".to_string(),
                button_text: "#000000".to_string(),
                button_hover: "#aaaaaa".to_string(),
                menu_bg: "#111111".to_string(),
                toolbar_bg: "#222222".to_string(),
                status_bg: "#333333".to_string(),
                editor_bg: "#444444".to_string(),
                title_bg: "#555555".to_string(),
                window_bg: "#666666".to_string(),
                panel_bg: "#777777".to_string(),
                input_bg: "#888888".to_string(),
                input_text: "#999999".to_string(),
                input_border: "#aaaaaa".to_string(),
                input_focus: "#bbbbbb".to_string(),
                border_radius: 4.0,
                menu_border_radius: 3.0,
                container_border_radius: 6.0,
                input_border_radius: 4.0,
                ribbon_bg: "#cccccc".to_string(),
                dropdown_bg: "#dddddd".to_string(),
            },
        };

        manager.add_theme(custom_theme.clone());
        assert!(manager
            .set_theme(ThemeType::Custom("custom".to_string()))
            .is_ok());

        let current = manager.get_current_theme();
        assert_eq!(current.name, "Custom");

        // Test removing custom theme
        assert!(manager.remove_theme(&ThemeType::Custom("custom".to_string())));
        assert!(!manager.remove_theme(&ThemeType::Light)); // Should fail for built-in themes
    }
}
