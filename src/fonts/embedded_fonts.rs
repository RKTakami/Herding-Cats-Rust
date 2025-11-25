//! Embedded font module for default open-source fonts
//! Provides built-in fonts that are immediately available without downloads

/// Default embedded fonts with licensing information
#[derive(Debug, Clone)]
pub struct EmbeddedFont {
    pub name: &'static str,
    pub family: &'static str,
    pub category: FontCategory,
    pub license: FontLicense,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub data: &'static [u8],
}

/// Font categories for organization
#[derive(Debug, Clone, PartialEq)]
pub enum FontCategory {
    SansSerif,
    Serif,
    Monospace,
    Display,
    Handwriting,
}

/// Font license types
#[derive(Debug, Clone, PartialEq)]
pub enum FontLicense {
    SILOpen,           // SIL Open Font License (fully free)
    Apache,           // Apache License 2.0
    MIT,              // MIT License
    GPL,              // GPL v3
    PublicDomain,     // Public domain
    MicrosoftOwned,   // Microsoft's open-source fonts
    ThirdParty,       // Third-party open-source
}

/// Font weights
#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub fn get_value(&self) -> u16 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }

    pub fn from_value(value: u16) -> Self {
        match value {
            100 => FontWeight::Thin,
            200 => FontWeight::ExtraLight,
            300 => FontWeight::Light,
            400 => FontWeight::Regular,
            500 => FontWeight::Medium,
            600 => FontWeight::SemiBold,
            700 => FontWeight::Bold,
            800 => FontWeight::ExtraBold,
            900 => FontWeight::Black,
            _ => FontWeight::Regular, // Default to regular
        }
    }
}

/// Font styles
#[derive(Debug, Clone, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Complete embedded font catalog
pub const EMBEDDED_FONTS: &[EmbeddedFont] = &[
    // Crimson Text (Google, SIL Open Font License) - The only font file we have
    EmbeddedFont {
        name: "Crimson Text",
        family: "Crimson Text",
        category: FontCategory::Serif,
        license: FontLicense::SILOpen,
        weight: FontWeight::Regular,
        style: FontStyle::Normal,
        data: include_bytes!("CrimsonText-Regular.ttf"),
    },
];

/// Font categories for UI grouping
pub fn get_fonts_by_category(category: FontCategory) -> Vec<EmbeddedFont> {
    EMBEDDED_FONTS
        .iter()
        .filter(|font| font.category == category)
        .cloned()
        .collect()
}

/// Get default fonts by category
pub fn get_default_font_for_category(category: FontCategory) -> Option<&'static str> {
    match category {
        FontCategory::SansSerif => Some("Inter"),
        FontCategory::Serif => Some("Crimson Text"),
        FontCategory::Monospace => Some("Cascadia Code"),
        FontCategory::Display => Some("Source Sans Pro"),
        FontCategory::Handwriting => None,
    }
}

/// Initialize all embedded fonts (simplified version)
pub fn init_embedded_fonts() -> Result<(), String> {
    println!("Initializing {} embedded fonts...", EMBEDDED_FONTS.len());
    
    for font in EMBEDDED_FONTS {
        println!("✓ Registered embedded font: {} ({:?})", font.name, font.license);
    }
    
    println!("Embedded fonts initialization complete");
    Ok(())
}

/// Get licensing information for a font family
pub fn get_font_license_info(family: &str) -> Option<&FontLicense> {
    EMBEDDED_FONTS
        .iter()
        .find(|font| font.family == family)
        .map(|font| &font.license)
}

/// Check if a font is embedded (built-in)
pub fn is_font_embedded(family: &str) -> bool {
    EMBEDDED_FONTS
        .iter()
        .any(|font| font.family == family)
}

/// Get all unique font families
pub fn get_embedded_families() -> Vec<String> {
    let mut families: Vec<String> = EMBEDDED_FONTS
        .iter()
        .map(|font| font.family.to_string())
        .collect();
    
    families.sort();
    families.dedup();
    families
}