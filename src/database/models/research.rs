//! Research Writing Tool Data Models
//!
//! Provides comprehensive data structures for managing research materials including
//! PDF documents, images, web links, and citations for academic writing.

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Research material types that can be attached to writing projects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResearchMaterialType {
    /// PDF document
    Pdf,
    /// Image file (JPG, PNG, GIF, etc.)
    Image,
    /// Web link/URL
    WebLink,
    /// Citation reference
    Citation,
    /// Note or annotation
    Note,
    /// Audio file
    Audio,
    /// Video file
    Video,
}

impl ResearchMaterialType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ResearchMaterialType::Pdf => "PDF Document",
            ResearchMaterialType::Image => "Image",
            ResearchMaterialType::WebLink => "Web Link",
            ResearchMaterialType::Citation => "Citation",
            ResearchMaterialType::Note => "Note",
            ResearchMaterialType::Audio => "Audio",
            ResearchMaterialType::Video => "Video",
        }
    }

    pub fn file_extensions(&self) -> &'static [&'static str] {
        match self {
            ResearchMaterialType::Pdf => &["pdf"],
            ResearchMaterialType::Image => &["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp"],
            ResearchMaterialType::WebLink => &[],
            ResearchMaterialType::Citation => &["bib", "ris"],
            ResearchMaterialType::Note => &["txt", "md"],
            ResearchMaterialType::Audio => &["mp3", "wav", "ogg", "m4a"],
            ResearchMaterialType::Video => &["mp4", "avi", "mov", "wmv", "flv"],
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            ResearchMaterialType::Pdf => "application/pdf",
            ResearchMaterialType::Image => "image/*",
            ResearchMaterialType::WebLink => "text/uri-list",
            ResearchMaterialType::Citation => "application/x-bibtex",
            ResearchMaterialType::Note => "text/plain",
            ResearchMaterialType::Audio => "audio/*",
            ResearchMaterialType::Video => "video/*",
        }
    }
}

/// Citation styles for academic references
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitationStyle {
    Apa,
    Mla,
    Chicago,
    Harvard,
    Ieee,
    Vancouver,
    Custom,
}

impl CitationStyle {
    pub fn display_name(&self) -> &'static str {
        match self {
            CitationStyle::Apa => "APA",
            CitationStyle::Mla => "MLA",
            CitationStyle::Chicago => "Chicago",
            CitationStyle::Harvard => "Harvard",
            CitationStyle::Ieee => "IEEE",
            CitationStyle::Vancouver => "Vancouver",
            CitationStyle::Custom => "Custom",
        }
    }
}

/// Research material metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMaterial {
    pub id: Uuid,
    pub project_id: Uuid,
    pub material_type: ResearchMaterialType,
    pub title: String,
    pub description: Option<String>,
    pub file_path: Option<PathBuf>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
    pub accessed_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub file_size: Option<u64>,
    pub thumbnail_path: Option<PathBuf>,
    pub extracted_text: Option<String>,
}

impl Default for ResearchMaterial {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id: Uuid::nil(),
            material_type: ResearchMaterialType::Note,
            title: "New Research Material".to_string(),
            description: None,
            file_path: None,
            url: None,
            author: None,
            publication_date: None,
            accessed_date: None,
            tags: Vec::new(),
            metadata: serde_json::Value::Null,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            file_size: None,
            thumbnail_path: None,
            extracted_text: None,
        }
    }
}

impl ResearchMaterial {
    pub fn new(project_id: Uuid, material_type: ResearchMaterialType, title: String) -> Self {
        let mut material = Self::default();
        material.project_id = project_id;
        material.material_type = material_type;
        material.title = title;
        material
    }

    /// Check if material has a valid file
    pub fn has_file(&self) -> bool {
        self.file_path.is_some() && self.file_path.as_ref().unwrap().exists()
    }

    /// Check if material has a valid URL
    pub fn has_url(&self) -> bool {
        self.url.as_ref().is_some_and(|u| !u.is_empty())
    }

    /// Get file extension if available
    pub fn file_extension(&self) -> Option<String> {
        self.file_path
            .as_ref()
            .and_then(|path| path.extension())
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    /// Check if URL is HTTPS
    pub fn is_secure_url(&self) -> bool {
        self.url
            .as_ref()
            .map(|url| url.starts_with("https://"))
            .unwrap_or(false)
    }

    /// Validate URL format
    pub fn is_valid_url(&self) -> bool {
        if let Some(url) = &self.url {
            url::Url::parse(url).is_ok()
        } else {
            false
        }
    }

    /// Get citation in APA format (simplified)
    pub fn format_citation(&self, style: CitationStyle) -> String {
        let author = self.author.as_deref().unwrap_or("Unknown");
        let title = &self.title;
        let year = self
            .publication_date
            .as_ref()
            .map(|date| date.year().to_string())
            .unwrap_or_else(|| "n.d.".to_string());

        match style {
            CitationStyle::Apa => {
                if let Some(url) = &self.url {
                    format!("{} ({}) {}. Retrieved from {}", author, year, title, url)
                } else {
                    format!("{} ({}). {}.", author, year, title)
                }
            }
            CitationStyle::Mla => {
                if let Some(url) = &self.url {
                    format!("{}. {}. {}.", author, title, url)
                } else {
                    format!("{}. {}.", author, title)
                }
            }
            _ => format!("{} - {} ({})", author, title, year),
        }
    }
}

/// Research collection for organizing materials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchCollection {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub materials: Vec<Uuid>, // References to ResearchMaterial IDs
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResearchCollection {
    pub fn new(project_id: Uuid, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id,
            name,
            description: None,
            materials: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn add_material(&mut self, material_id: Uuid) {
        if !self.materials.contains(&material_id) {
            self.materials.push(material_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_material(&mut self, material_id: &Uuid) {
        if let Some(index) = self.materials.iter().position(|id| id == material_id) {
            self.materials.remove(index);
            self.updated_at = Utc::now();
        }
    }
}

/// Citation reference for bibliography
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationReference {
    pub id: Uuid,
    pub material_id: Uuid,
    pub citation_style: CitationStyle,
    pub formatted_citation: String,
    pub bibliography_entry: String,
    pub in_text_citation: String,
    pub created_at: DateTime<Utc>,
}

impl CitationReference {
    pub fn new(material_id: Uuid, material: &ResearchMaterial, style: CitationStyle) -> Self {
        let formatted = material.format_citation(style);
        let bibliography = format!(
            "{}. {}",
            formatted,
            material.description.as_deref().unwrap_or("")
        );
        let in_text = match style {
            CitationStyle::Apa => {
                let author = material.author.as_deref().unwrap_or("Unknown");
                let year = material
                    .publication_date
                    .as_ref()
                    .map(|date| date.year().to_string())
                    .unwrap_or_else(|| "n.d.".to_string());
                format!("({} {})", author, year)
            }
            CitationStyle::Mla => {
                let author = material.author.as_deref().unwrap_or("Unknown");
                format!("({})", author)
            }
            _ => "(citation)".to_string(),
        };

        Self {
            id: Uuid::new_v4(),
            material_id,
            citation_style: style,
            formatted_citation: formatted,
            bibliography_entry: bibliography,
            in_text_citation: in_text,
            created_at: Utc::now(),
        }
    }
}

/// Research analytics and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchAnalytics {
    pub total_materials: usize,
    pub materials_by_type: std::collections::HashMap<ResearchMaterialType, usize>,
    pub total_file_size: u64,
    pub average_access_date: Option<DateTime<Utc>>,
    pub most_used_tags: Vec<(String, usize)>,
    pub citation_count: usize,
    pub https_links_count: usize,
    pub pdf_count: usize,
    pub image_count: usize,
}

/// Research search parameters
#[derive(Debug, Clone)]
pub struct ResearchSearch {
    pub query: String,
    pub material_types: Vec<ResearchMaterialType>,
    pub tags: Vec<String>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub author: Option<String>,
    pub has_file: Option<bool>,
    pub has_url: Option<bool>,
    pub is_https: Option<bool>,
}

/// Database query constants for research materials
pub const CREATE_RESEARCH_TABLES_SQL: &str = r#"
-- Research Materials Table
CREATE TABLE IF NOT EXISTS research_materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    material_type TEXT NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_path TEXT,
    url TEXT,
    author VARCHAR(255),
    publication_date TIMESTAMPTZ,
    accessed_date TIMESTAMPTZ,
    tags TEXT[], -- Array of tags
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    file_size BIGINT,
    thumbnail_path TEXT,
    extracted_text TEXT,
    
    -- Indexes
    INDEX idx_research_materials_project_id (project_id),
    INDEX idx_research_materials_type (material_type),
    INDEX idx_research_materials_author (author),
    INDEX idx_research_materials_created_at (created_at),
    INDEX idx_research_materials_tags USING GIN (tags)
);

-- Research Collections Table
CREATE TABLE IF NOT EXISTS research_collections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Indexes
    INDEX idx_research_collections_project_id (project_id)
);

-- Collection Materials Junction Table
CREATE TABLE IF NOT EXISTS collection_materials (
    collection_id UUID NOT NULL REFERENCES research_collections(id) ON DELETE CASCADE,
    material_id UUID NOT NULL REFERENCES research_materials(id) ON DELETE CASCADE,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (collection_id, material_id)
);

-- Citation References Table
CREATE TABLE IF NOT EXISTS citation_references (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_id UUID NOT NULL REFERENCES research_materials(id) ON DELETE CASCADE,
    citation_style TEXT NOT NULL,
    formatted_citation TEXT NOT NULL,
    bibliography_entry TEXT NOT NULL,
    in_text_citation TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    INDEX idx_citation_references_material_id (material_id)
);

-- Research Analytics View
CREATE OR REPLACE VIEW research_analytics AS
SELECT 
    rm.project_id,
    COUNT(*) as total_materials,
    COUNT(CASE WHEN rm.material_type = 'Pdf' THEN 1 END) as pdf_count,
    COUNT(CASE WHEN rm.material_type = 'Image' THEN 1 END) as image_count,
    COUNT(CASE WHEN rm.material_type = 'WebLink' THEN 1 END) as weblink_count,
    COUNT(CASE WHEN rm.material_type = 'Citation' THEN 1 END) as citation_count,
    COUNT(CASE WHEN rm.url LIKE 'https://%') as https_count,
    COALESCE(SUM(rm.file_size), 0) as total_file_size,
    AVG(rm.accessed_date) as avg_access_date
FROM research_materials rm
GROUP BY rm.project_id;
"#;

/// Insert research material SQL
pub const INSERT_RESEARCH_MATERIAL_SQL: &str = r#"
INSERT INTO research_materials (
    id, project_id, material_type, title, description, file_path, url, 
    author, publication_date, accessed_date, tags, metadata, file_size, thumbnail_path, extracted_text
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
RETURNING *;
"#;

/// Update research material SQL
pub const UPDATE_RESEARCH_MATERIAL_SQL: &str = r#"
UPDATE research_materials 
SET title = $2, description = $3, file_path = $4, url = $5, 
    author = $6, publication_date = $7, accessed_date = $8, 
    tags = $9, metadata = $10, file_size = $11, thumbnail_path = $12, 
    extracted_text = $13, updated_at = NOW()
WHERE id = $1
RETURNING *;
"#;

/// Get research material by ID SQL
pub const GET_RESEARCH_MATERIAL_SQL: &str = r#"
SELECT * FROM research_materials WHERE id = $1;
"#;

/// Get all research materials for project SQL
pub const GET_RESEARCH_MATERIALS_BY_PROJECT_SQL: &str = r#"
SELECT * FROM research_materials 
WHERE project_id = $1 
ORDER BY created_at DESC
LIMIT $2 OFFSET $3;
"#;

/// Search research materials SQL
pub const SEARCH_RESEARCH_MATERIALS_SQL: &str = r#"
SELECT * FROM research_materials 
WHERE project_id = $1 
AND (
    title ILIKE $4 
    OR description ILIKE $4 
    OR author ILIKE $4
    OR $4 = ''
)
AND (material_type = ANY($2) OR $2 IS NULL)
AND (tags && $3 OR $3 IS NULL)
ORDER BY created_at DESC
LIMIT $5 OFFSET $6;
"#;
