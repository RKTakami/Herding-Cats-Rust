//! Downloadable font collections including Microsoft fonts and other premium sets
//! Provides legal font downloads with proper licensing information

use std::collections::HashMap;
use crate::error::AppError;

/// Font collection information
#[derive(Debug, Clone)]
pub struct FontCollection {
    pub name: &'static str,
    pub description: &'static str,
    pub publisher: &'static str,
    pub license: FontDistributionLicense,
    pub category: FontCollectionCategory,
    pub fonts: &'static [DownloadableFont],
    pub download_url: Option<&'static str>,
    pub size_mb: Option<f32>,
    pub verified_safe: bool,
}

/// Distribution license types
#[derive(Debug, Clone, PartialEq)]
pub enum FontDistributionLicense {
    Freeware,           // Free for personal/commercial use
    OpenSource,         // Open source license
    Commercial,         // Requires purchase
    Educational,        // Free for educational use
    Trial,             // Time-limited trial
    MicrosoftOwned,    // Microsoft's proprietary fonts
    SystemFonts,       // System-installed fonts only
}

/// Font collection categories
#[derive(Debug, Clone, PartialEq)]
pub enum FontCollectionCategory {
    MicrosoftEssentials,
    MicrosoftComplete,
    GoogleFonts,
    AdobeFonts,
    DeveloperFonts,
    TypographyPacks,
    SystemFonts,
}

/// Individual downloadable font
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadableFont {
    pub name: &'static str,
    pub family: &'static str,
    pub weights: &'static [FontWeight],
    pub styles: &'static [FontStyle],
    pub license: FontDistributionLicense,
    pub download_url: Option<&'static str>,
    pub file_size: Option<&'static str>,
    pub file_format: FontFileFormat,
}

/// Font file formats
#[derive(Debug, Clone, PartialEq)]
pub enum FontFileFormat {
    TTF,    // TrueType
    OTF,    // OpenType
    WOFF,   // Web Open Font Format
    WOFF2,  // Web Open Font Format 2.0
    EOT,    // Embedded OpenType
}

/// Font weight representation
#[derive(Debug, Clone, PartialEq)]
pub struct FontWeight {
    pub value: u16,
    pub name: &'static str,  // e.g., "Regular", "Bold", "Light"
}

/// Font style representation
#[derive(Debug, Clone, PartialEq)]
pub struct FontStyle {
    pub name: &'static str,  // e.g., "Normal", "Italic", "Oblique"
}

/// Complete downloadable font catalog
pub const FONT_COLLECTIONS: &[FontCollection] = &[
    // Microsoft Essential Fonts Collection
    FontCollection {
        name: "Microsoft Essential Fonts",
        description: "Core Microsoft fonts for professional documents and UI",
        publisher: "Microsoft Corporation",
        license: FontDistributionLicense::MicrosoftOwned,
        category: FontCollectionCategory::MicrosoftEssentials,
        fonts: &[
            DownloadableFont {
                name: "Segoe UI",
                family: "Segoe UI",
                weights: &[
                    FontWeight { value: 300, name: "Light" },
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 600, name: "Semibold" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::MicrosoftOwned,
                download_url: Some("https://github.com/microsoft/cascadia-code/releases"),
                file_size: Some("2.1 MB"),
                file_format: FontFileFormat::TTF,
            },
            DownloadableFont {
                name: "Consolas",
                family: "Consolas",
                weights: &[
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::MicrosoftOwned,
                download_url: Some("https://github.com/microsoft/cascadia-code"),
                file_size: Some("1.8 MB"),
                file_format: FontFileFormat::TTF,
            },
            DownloadableFont {
                name: "Calibri",
                family: "Calibri",
                weights: &[
                    FontWeight { value: 300, name: "Light" },
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 600, name: "Semibold" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::MicrosoftOwned,
                download_url: Some("https://docs.microsoft.com/typography/font-list/calibri"),
                file_size: Some("2.3 MB"),
                file_format: FontFileFormat::TTF,
            },
        ],
        download_url: Some("https://www.microsoft.com/en-us/typography"),
        size_mb: Some(6.2),
        verified_safe: true,
    },

    // Google Fonts Collection
    FontCollection {
        name: "Google Fonts Complete",
        description: "Popular open-source fonts from Google's font library",
        publisher: "Google LLC",
        license: FontDistributionLicense::OpenSource,
        category: FontCollectionCategory::GoogleFonts,
        fonts: &[
            DownloadableFont {
                name: "Roboto",
                family: "Roboto",
                weights: &[
                    FontWeight { value: 100, name: "Thin" },
                    FontWeight { value: 300, name: "Light" },
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 500, name: "Medium" },
                    FontWeight { value: 700, name: "Bold" },
                    FontWeight { value: 900, name: "Black" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::OpenSource,
                download_url: Some("https://fonts.google.com/specimen/Roboto"),
                file_size: Some("1.2 MB"),
                file_format: FontFileFormat::TTF,
            },
            DownloadableFont {
                name: "Open Sans",
                family: "Open Sans",
                weights: &[
                    FontWeight { value: 300, name: "Light" },
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 600, name: "Semibold" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::OpenSource,
                download_url: Some("https://fonts.google.com/specimen/Open+Sans"),
                file_size: Some("1.4 MB"),
                file_format: FontFileFormat::TTF,
            },
            DownloadableFont {
                name: "Lato",
                family: "Lato",
                weights: &[
                    FontWeight { value: 100, name: "Thin" },
                    FontWeight { value: 300, name: "Light" },
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 700, name: "Bold" },
                    FontWeight { value: 900, name: "Black" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::OpenSource,
                download_url: Some("https://fonts.google.com/specimen/Lato"),
                file_size: Some("1.1 MB"),
                file_format: FontFileFormat::TTF,
            },
        ],
        download_url: Some("https://fonts.google.com"),
        size_mb: Some(15.8),
        verified_safe: true,
    },

    // Developer Font Pack
    FontCollection {
        name: "Developer Font Pack",
        description: "High-quality monospace fonts for coding and technical work",
        publisher: "Various Open Source",
        license: FontDistributionLicense::OpenSource,
        category: FontCollectionCategory::DeveloperFonts,
        fonts: &[
            DownloadableFont {
                name: "Fira Code",
                family: "Fira Code",
                weights: &[
                    FontWeight { value: 300, name: "Light" },
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 500, name: "Medium" },
                    FontWeight { value: 600, name: "Semibold" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::OpenSource,
                download_url: Some("https://github.com/tonsky/FiraCode"),
                file_size: Some("3.2 MB"),
                file_format: FontFileFormat::TTF,
            },
            DownloadableFont {
                name: "Source Code Pro",
                family: "Source Code Pro",
                weights: &[
                    FontWeight { value: 200, name: "ExtraLight" },
                    FontWeight { value: 300, name: "Light" },
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 500, name: "Medium" },
                    FontWeight { value: 600, name: "Semibold" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                ],
                license: FontDistributionLicense::OpenSource,
                download_url: Some("https://fonts.google.com/specimen/Source+Code+Pro"),
                file_size: Some("2.8 MB"),
                file_format: FontFileFormat::TTF,
            },
        ],
        download_url: Some("https://github.com/tonsky/FiraCode"),
        size_mb: Some(6.0),
        verified_safe: true,
    },

    // System Fonts Collection
    FontCollection {
        name: "System Fonts",
        description: "Fonts available on your operating system - no download needed",
        publisher: "Your Operating System",
        license: FontDistributionLicense::SystemFonts,
        category: FontCollectionCategory::SystemFonts,
        fonts: &[
            DownloadableFont {
                name: "Arial",
                family: "Arial",
                weights: &[
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::SystemFonts,
                download_url: None,
                file_size: None,
                file_format: FontFileFormat::TTF,
            },
            DownloadableFont {
                name: "Helvetica",
                family: "Helvetica",
                weights: &[
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::SystemFonts,
                download_url: None,
                file_size: None,
                file_format: FontFileFormat::TTF,
            },
            DownloadableFont {
                name: "Times New Roman",
                family: "Times New Roman",
                weights: &[
                    FontWeight { value: 400, name: "Regular" },
                    FontWeight { value: 700, name: "Bold" },
                ],
                styles: &[
                    FontStyle { name: "Normal" },
                    FontStyle { name: "Italic" },
                ],
                license: FontDistributionLicense::SystemFonts,
                download_url: None,
                file_size: None,
                file_format: FontFileFormat::TTF,
            },
        ],
        download_url: None,
        size_mb: None,
        verified_safe: true,
    },
];

/// Get all font collections
pub fn get_font_collections() -> &'static [FontCollection] {
    FONT_COLLECTIONS
}

/// Get collections by category
pub fn get_collections_by_category(category: FontCollectionCategory) -> Vec<&'static FontCollection> {
    FONT_COLLECTIONS
        .iter()
        .filter(|collection| collection.category == category)
        .collect()
}

/// Get a specific collection by name
pub fn get_collection_by_name(name: &str) -> Option<&FontCollection> {
    FONT_COLLECTIONS
        .iter()
        .find(|collection| collection.name == name)
}

/// Check if collection requires licensing acceptance
pub fn requires_license_acceptance(collection: &FontCollection) -> bool {
    matches!(collection.license, 
        FontDistributionLicense::MicrosoftOwned | 
        FontDistributionLicense::Commercial |
        FontDistributionLicense::Trial
    )
}

/// Get license text for a collection
pub fn get_license_text(collection: &FontCollection) -> String {
    match collection.license {
        FontDistributionLicense::MicrosoftOwned => {
            format!("{} Font License Agreement\n\nBy downloading these fonts, you agree to Microsoft's Font License Agreement. These fonts are proprietary to Microsoft Corporation and are subject to Microsoft's terms of use.\n\nDistribution of these fonts in your software applications may require additional licensing from Microsoft.", 
                collection.publisher)
        },
        FontDistributionLicense::OpenSource => {
            format!("Open Source License Agreement\n\n{} fonts are distributed under open source licenses (SIL OFL, Apache, MIT, etc.). You are free to use, modify, and distribute these fonts according to the specific license terms.\n\nSee the individual font pages for specific license details.", 
                collection.publisher)
        },
        FontDistributionLicense::SystemFonts => {
            "System Font Agreement\n\nThese fonts are already installed on your operating system. No download or additional licensing is required. Usage is governed by your operating system's font license terms.".to_string()
        },
        _ => "License information not available.".to_string(),
    }
}

/// Get recommended actions for licensing compliance
pub fn get_licensing_recommendations(collection: &FontCollection) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    match collection.license {
        FontDistributionLicense::MicrosoftOwned => {
            recommendations.push("Review Microsoft's Font License Agreement".to_string());
            recommendations.push("Ensure compliance with Microsoft's terms".to_string());
            recommendations.push("Consider purchasing commercial licenses if redistributing".to_string());
            recommendations.push("Contact Microsoft for enterprise licensing if needed".to_string());
        },
        FontDistributionLicense::OpenSource => {
            recommendations.push("Review specific open source license terms".to_string());
            recommendations.push("Include license attribution where required".to_string());
            recommendations.push("Maintain license files in your distribution".to_string());
        },
        FontDistributionLicense::Commercial => {
            recommendations.push("Purchase appropriate commercial licenses".to_string());
            recommendations.push("Keep purchase receipts for compliance".to_string());
            recommendations.push("Review vendor-specific licensing terms".to_string());
        },
        FontDistributionLicense::SystemFonts => {
            recommendations.push("No action required - fonts already licensed with OS".to_string());
        },
        _ => {
            recommendations.push("Review licensing terms before use".to_string());
        }
    }
    
    recommendations
}