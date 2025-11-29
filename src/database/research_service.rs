//! Research Writing Tool Service
//!
//! Comprehensive service for managing research materials including PDF processing,
//! image handling, HTTPS link validation, and citation management for academic writing.

use crate::database::{
    models::research::{CitationReference, ResearchCollection, ResearchMaterial, *},
    DatabaseError, DatabaseResult, EnhancedDatabaseService,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;
use uuid::Uuid;

/// Research service for managing academic materials
#[derive(Debug)]
pub struct ResearchService {
    db_service: Arc<RwLock<EnhancedDatabaseService>>,
}

impl ResearchService {
    /// Create a new research service
    pub fn new(db_service: Arc<RwLock<EnhancedDatabaseService>>) -> Self {
        Self { db_service }
    }

    /// Initialize research tables and indexes
    pub async fn initialize(&self) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;

        db_service
            .execute(CREATE_RESEARCH_TABLES_SQL, &[])
            .await
            .map_err(|e| {
                DatabaseError::Migration(format!("Failed to create research tables: {}", e))
            })?;

        Ok(())
    }

    /// Create a new research material
    pub async fn create_material(
        &self,
        mut material: ResearchMaterial,
    ) -> DatabaseResult<ResearchMaterial> {
        // Validate the material
        self.validate_material(&material)?;

        // Process file if provided
        if let Some(file_path) = material.file_path.clone() {
            self.process_file(&mut material, &file_path).await?;
        }

        // Validate and process URL if provided
        if let Some(url) = material.url.clone() {
            self.validate_and_process_url(&mut material, &url).await?;
        }

        let db_service = self.db_service.read().await;

        let tags_json = serde_json::to_string(&material.tags)
            .map_err(|e| DatabaseError::Service(format!("Failed to serialize tags: {}", e)))?;

        // Insert the material and return the created record
        db_service
            .execute(
                INSERT_RESEARCH_MATERIAL_SQL,
                &[
                    material.id.to_string(),
                    material.project_id.to_string(),
                    format!("{:?}", material.material_type),
                    material.title.clone(),
                    material.description.as_deref().unwrap_or("").to_string(),
                    material
                        .file_path
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default(),
                    material.url.as_deref().unwrap_or("").to_string(),
                    material.author.as_deref().unwrap_or("").to_string(),
                    material
                        .publication_date
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or("".to_string()),
                    material
                        .accessed_date
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or("".to_string()),
                    tags_json,
                    material.metadata.to_string(),
                    material
                        .file_size
                        .map(|s| s as i64)
                        .map(|s| s.to_string())
                        .unwrap_or_default(),
                    material
                        .thumbnail_path
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default(),
                    material.extracted_text.as_deref().unwrap_or("").to_string(),
                ],
            )
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to insert research material: {}", e))
            })?;

        Ok(material)
    }

    /// Get research material by ID
    pub async fn get_material(
        &self,
        material_id: Uuid,
    ) -> DatabaseResult<Option<ResearchMaterial>> {
        let db_service = self.db_service.read().await;

        let rows = db_service
            .query(GET_RESEARCH_MATERIAL_SQL, &[material_id.to_string()])
            .await?;

        if let Some(row) = rows.rows.first() {
            let material = ResearchMaterial {
                id: Uuid::parse_str(
                    row.get(0)
                        .ok_or_else(|| DatabaseError::Service("Failed to get ID".to_string()))?,
                )
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                project_id: Uuid::parse_str(row.get(1).ok_or_else(|| {
                    DatabaseError::Service("Failed to get project ID".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                material_type: self.parse_material_type(row.get(2).ok_or_else(|| {
                    DatabaseError::Service("Failed to get material type".to_string())
                })?)?,
                title: row
                    .get(3)
                    .ok_or_else(|| DatabaseError::Service("Failed to get title".to_string()))?
                    .to_string(),
                description: row.get(4).map(|s: &str| s.to_string()),
                file_path: row.get(5).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                url: row.get(6).map(|s| s.to_string()),
                author: Some(
                    row.get(7)
                        .ok_or_else(|| DatabaseError::Service("Failed to get author".to_string()))?
                        .to_string(),
                ),
                publication_date: row
                    .get(8)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                accessed_date: row
                    .get(9)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                tags: serde_json::from_str(
                    row.get(10)
                        .ok_or_else(|| DatabaseError::Service("Failed to get tags".to_string()))?,
                )
                .unwrap_or_default(),
                metadata: serde_json::from_str(
                    row.get(11).ok_or_else(|| {
                        DatabaseError::Service("Failed to get metadata".to_string())
                    })?,
                )
                .unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(row.get(12).ok_or_else(|| {
                    DatabaseError::Service("Failed to get created_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(row.get(13).ok_or_else(|| {
                    DatabaseError::Service("Failed to get updated_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                file_size: row
                    .get(14)
                    .and_then(|s| s.parse::<i64>().ok())
                    .map(|s| s as u64),
                thumbnail_path: row.get(15).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                extracted_text: row.get(16).map(|s| s.to_string()),
            };
            Ok(Some(material))
        } else {
            Ok(None)
        }
    }

    /// Get all research materials for a project
    pub async fn get_materials_by_project(
        &self,
        project_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> DatabaseResult<Vec<ResearchMaterial>> {
        let db_service = self.db_service.read().await;

        let rows = db_service
            .query(
                GET_RESEARCH_MATERIALS_BY_PROJECT_SQL,
                &[
                    project_id.to_string(),
                    limit.to_string(),
                    offset.to_string(),
                ],
            )
            .await?;

        let mut materials = Vec::new();
        for row in rows {
            let material = ResearchMaterial {
                id: Uuid::parse_str(
                    row.get(0)
                        .ok_or_else(|| DatabaseError::Service("Failed to get ID".to_string()))?,
                )
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                project_id: Uuid::parse_str(row.get(1).ok_or_else(|| {
                    DatabaseError::Service("Failed to get project ID".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                material_type: self.parse_material_type(row.get(2).ok_or_else(|| {
                    DatabaseError::Service("Failed to get material type".to_string())
                })?)?,
                title: row
                    .get(3)
                    .ok_or_else(|| DatabaseError::Service("Failed to get title".to_string()))?
                    .to_string(),
                description: Some(
                    row.get(4)
                        .ok_or_else(|| {
                            DatabaseError::Service("Failed to get description".to_string())
                        })?
                        .to_string(),
                ),
                file_path: row.get(5).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                url: row.get(6).map(|s| s.to_string()),
                author: Some(
                    row.get(7)
                        .ok_or_else(|| DatabaseError::Service("Failed to get author".to_string()))?
                        .to_string(),
                ),
                publication_date: row
                    .get(8)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                accessed_date: row
                    .get(9)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                tags: serde_json::from_str(
                    row.get(10)
                        .ok_or_else(|| DatabaseError::Service("Failed to get tags".to_string()))?,
                )
                .unwrap_or_default(),
                metadata: serde_json::from_str(
                    row.get(11).ok_or_else(|| {
                        DatabaseError::Service("Failed to get metadata".to_string())
                    })?,
                )
                .unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(row.get(12).ok_or_else(|| {
                    DatabaseError::Service("Failed to get created_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(row.get(13).ok_or_else(|| {
                    DatabaseError::Service("Failed to get updated_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                file_size: row
                    .get(14)
                    .and_then(|s| s.parse().ok())
                    .map(|s: i64| s as u64),
                thumbnail_path: row.get(15).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                extracted_text: row.get(16).map(|s| s.to_string()),
            };
            materials.push(material);
        }

        Ok(materials)
    }

    /// Search research materials
    pub async fn search_materials(
        &self,
        search: ResearchSearch,
    ) -> DatabaseResult<Vec<ResearchMaterial>> {
        let db_service = self.db_service.read().await;

        // Convert material types to strings
        let material_types_str: Vec<String> = search
            .material_types
            .iter()
            .map(|mt| format!("{:?}", mt))
            .collect();

        let tags_str: Vec<String> = search.tags;

        let rows = db_service
            .query(
                SEARCH_RESEARCH_MATERIALS_SQL,
                &[
                    search.query,
                    if material_types_str.is_empty() {
                        "".to_string()
                    } else {
                        material_types_str.join(",")
                    },
                    if tags_str.is_empty() {
                        "".to_string()
                    } else {
                        tags_str.join(",")
                    },
                    100.to_string(), // limit
                    0.to_string(),   // offset
                ],
            )
            .await?;

        let mut materials = Vec::new();
        for row in rows {
            let material = ResearchMaterial {
                id: Uuid::parse_str(
                    row.get(0)
                        .ok_or_else(|| DatabaseError::Service("Failed to get ID".to_string()))?,
                )
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                project_id: Uuid::parse_str(row.get(1).ok_or_else(|| {
                    DatabaseError::Service("Failed to get project ID".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                material_type: self.parse_material_type(row.get(2).ok_or_else(|| {
                    DatabaseError::Service("Failed to get material type".to_string())
                })?)?,
                title: row
                    .get(3)
                    .ok_or_else(|| DatabaseError::Service("Failed to get title".to_string()))?
                    .to_string(),
                description: Some(
                    row.get(4)
                        .ok_or_else(|| {
                            DatabaseError::Service("Failed to get description".to_string())
                        })?
                        .to_string(),
                ),
                file_path: row.get(5).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                url: row.get(6).map(|s| s.to_string()),
                author: Some(
                    row.get(7)
                        .ok_or_else(|| DatabaseError::Service("Failed to get author".to_string()))?
                        .to_string(),
                ),
                publication_date: row
                    .get(8)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                accessed_date: row
                    .get(9)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                tags: serde_json::from_str(
                    row.get(10)
                        .ok_or_else(|| DatabaseError::Service("Failed to get tags".to_string()))?,
                )
                .unwrap_or_default(),
                metadata: serde_json::from_str(
                    row.get(11).ok_or_else(|| {
                        DatabaseError::Service("Failed to get metadata".to_string())
                    })?,
                )
                .unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(row.get(12).ok_or_else(|| {
                    DatabaseError::Service("Failed to get created_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(row.get(13).ok_or_else(|| {
                    DatabaseError::Service("Failed to get updated_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                file_size: row
                    .get(14)
                    .and_then(|s| s.parse().ok())
                    .map(|s: i64| s as u64),
                thumbnail_path: row.get(15).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                extracted_text: row.get(16).map(|s| s.to_string()),
            };
            materials.push(material);
        }

        Ok(materials)
    }

    /// Validate research material
    fn validate_material(&self, material: &ResearchMaterial) -> DatabaseResult<()> {
        if material.title.trim().is_empty() {
            return Err(DatabaseError::ValidationError(
                "Material title cannot be empty".to_string(),
            ));
        }

        // Validate URL if present
        if let Some(url) = &material.url {
            if !self.is_valid_url(url) {
                return Err(DatabaseError::ValidationError(format!(
                    "Invalid URL: {}",
                    url
                )));
            }
        }

        // Validate file path if present
        if let Some(path) = &material.file_path {
            if !path.exists() {
                return Err(DatabaseError::ValidationError(format!(
                    "File does not exist: {}",
                    path.display()
                )));
            }
        }

        Ok(())
    }

    /// Validate and process URL
    async fn validate_and_process_url(
        &self,
        material: &mut ResearchMaterial,
        url: &str,
    ) -> DatabaseResult<()> {
        if !self.is_valid_url(url) {
            return Err(DatabaseError::ValidationError(format!(
                "Invalid URL: {}",
                url
            )));
        }

        // Check if HTTPS
        if url.starts_with("https://") {
            material
                .metadata
                .as_object_mut()
                .unwrap()
                .insert("is_secure".to_string(), Value::Bool(true));
        }

        // Try to fetch basic metadata
        if let Ok(parsed_url) = Url::parse(url) {
            material.metadata.as_object_mut().unwrap().insert(
                "domain".to_string(),
                Value::String(parsed_url.domain().unwrap_or("").to_string()),
            );
            material.metadata.as_object_mut().unwrap().insert(
                "tld".to_string(),
                Value::String(
                    parsed_url
                        .domain()
                        .and_then(|d| d.split('.').next_back())
                        .unwrap_or("")
                        .to_string(),
                ),
            );
        }

        Ok(())
    }

    /// Process file and extract metadata
    async fn process_file(
        &self,
        material: &mut ResearchMaterial,
        file_path: &PathBuf,
    ) -> DatabaseResult<()> {
        if !file_path.exists() {
            return Err(DatabaseError::ValidationError(format!(
                "File does not exist: {}",
                file_path.display()
            )));
        }

        // Get file size
        if let Ok(metadata) = std::fs::metadata(file_path) {
            material.file_size = Some(metadata.len());
        }

        // Process different file types
        match material.material_type {
            ResearchMaterialType::Pdf => {
                self.process_pdf(material, file_path).await?;
            }
            ResearchMaterialType::Image => {
                self.process_image(material, file_path).await?;
            }
            _ => {
                // For other file types, just store basic info
                material.metadata.as_object_mut().unwrap().insert(
                    "file_type".to_string(),
                    Value::String("generic".to_string()),
                );
            }
        }

        Ok(())
    }

    /// Process PDF file and extract text/metadata
    async fn process_pdf(
        &self,
        material: &mut ResearchMaterial,
        file_path: &PathBuf,
    ) -> DatabaseResult<()> {
        // Note: In a real implementation, you would use a PDF library like lopdf
        // For now, we'll just store basic metadata
        material
            .metadata
            .as_object_mut()
            .unwrap()
            .insert("pdf_processed".to_string(), Value::Bool(true));

        // You could add PDF text extraction here using a library like:
        // - lopdf for PDF parsing
        // - tesseract for OCR if needed
        material.extracted_text = Some(format!("PDF content from: {}", file_path.display()));

        Ok(())
    }

    /// Process image file and create thumbnail
    async fn process_image(
        &self,
        material: &mut ResearchMaterial,
        _file_path: &PathBuf,
    ) -> DatabaseResult<()> {
        // Note: In a real implementation, you would use an image processing library
        // For now, we'll just store basic metadata
        material
            .metadata
            .as_object_mut()
            .unwrap()
            .insert("image_processed".to_string(), Value::Bool(true));

        // You could add image processing here using libraries like:
        // - image for basic image operations
        // - thumbnail for creating thumbnails
        material.metadata.as_object_mut().unwrap().insert(
            "thumbnail_generated".to_string(),
            Value::Bool(false), // Would be true after thumbnail creation
        );

        Ok(())
    }

    /// Validate URL format
    fn is_valid_url(&self, url: &str) -> bool {
        Url::parse(url).is_ok()
    }

    /// Parse material type from string
    fn parse_material_type(&self, type_str: &str) -> DatabaseResult<ResearchMaterialType> {
        match type_str {
            "Pdf" => Ok(ResearchMaterialType::Pdf),
            "Image" => Ok(ResearchMaterialType::Image),
            "WebLink" => Ok(ResearchMaterialType::WebLink),
            "Citation" => Ok(ResearchMaterialType::Citation),
            "Note" => Ok(ResearchMaterialType::Note),
            "Audio" => Ok(ResearchMaterialType::Audio),
            "Video" => Ok(ResearchMaterialType::Video),
            _ => Err(DatabaseError::ValidationError(format!(
                "Unknown material type: {}",
                type_str
            ))),
        }
    }

    /// Create citation reference
    pub fn create_citation_reference(
        &self,
        material: &ResearchMaterial,
        style: CitationStyle,
    ) -> CitationReference {
        CitationReference::new(material.id, material, style)
    }

    /// Get research analytics for a project
    pub async fn get_analytics(&self, project_id: Uuid) -> DatabaseResult<ResearchAnalytics> {
        let db_service = self.db_service.read().await;

        // Get basic counts using query method
        let total_rows = db_service
            .query(
                "SELECT COUNT(*) FROM research_materials WHERE project_id = ?1",
                &[project_id.to_string()],
            )
            .await?;
        let total_materials: i64 = total_rows
            .rows
            .first()
            .and_then(|r| r.get(0))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let https_rows = db_service.query(
            "SELECT COUNT(*) FROM research_materials WHERE project_id = ?1 AND url LIKE 'https://%'",
            &[project_id.to_string()],
        ).await?;
        let https_count: i64 = https_rows
            .rows
            .first()
            .and_then(|r| r.get(0))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let pdf_rows = db_service.query(
            "SELECT COUNT(*) FROM research_materials WHERE project_id = ?1 AND material_type = 'Pdf'",
            &[project_id.to_string()],
        ).await?;
        let pdf_count: i64 = pdf_rows
            .rows
            .first()
            .and_then(|r| r.get(0))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let image_rows = db_service.query(
            "SELECT COUNT(*) FROM research_materials WHERE project_id = ?1 AND material_type = 'Image'",
            &[project_id.to_string()],
        ).await?;
        let image_count: i64 = image_rows
            .rows
            .first()
            .and_then(|r| r.get(0))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let citation_rows = db_service
            .query(
                "SELECT COUNT(*) FROM citation_references cr
             JOIN research_materials rm ON cr.material_id = rm.id
             WHERE rm.project_id = ?1",
                &[project_id.to_string()],
            )
            .await?;
        let citation_count: i64 = citation_rows
            .rows
            .first()
            .and_then(|r| r.get(0))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let size_rows = db_service
            .query(
                "SELECT COALESCE(SUM(file_size), 0) FROM research_materials WHERE project_id = ?1",
                &[project_id.to_string()],
            )
            .await?;
        let total_file_size: i64 = size_rows
            .rows
            .first()
            .and_then(|r| r.get(0))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        // Get materials by type
        let mut materials_by_type = HashMap::new();
        let type_rows = db_service
            .query(
                "SELECT material_type, COUNT(*) FROM research_materials
             WHERE project_id = ?1 GROUP BY material_type",
                &[project_id.to_string()],
            )
            .await?;

        for row in type_rows {
            let type_str = row.get(0).unwrap_or("");
            let count_str = row.get(1).unwrap_or("0");
            let count: usize = count_str.parse().ok().unwrap_or(0);

            if let Ok(material_type) = self.parse_material_type(type_str) {
                materials_by_type.insert(material_type, count);
            }
        }

        Ok(ResearchAnalytics {
            total_materials: total_materials as usize,
            materials_by_type,
            total_file_size: total_file_size as u64,
            average_access_date: None,  // Would calculate from database
            most_used_tags: Vec::new(), // Would calculate from database
            citation_count: citation_count as usize,
            https_links_count: https_count as usize,
            pdf_count: pdf_count as usize,
            image_count: image_count as usize,
        })
    }

    /// Create a new research collection
    pub async fn create_collection(
        &self,
        collection: ResearchCollection,
    ) -> DatabaseResult<ResearchCollection> {
        let db_service = self.db_service.read().await;

        db_service.execute(
            "INSERT INTO research_collections (id, project_id, name, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            &[
                collection.id.to_string(),
                collection.project_id.to_string(),
                collection.name.clone(),
                collection.description.as_deref().unwrap_or("").to_string(),
                collection.created_at.to_rfc3339(),
                collection.updated_at.to_rfc3339(),
            ],
        ).await
        .map_err(|e| DatabaseError::Service(format!("Failed to insert collection: {}", e)))?;

        Ok(collection)
    }

    /// Add material to collection
    pub async fn add_material_to_collection(
        &self,
        collection_id: Uuid,
        material_id: Uuid,
    ) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;

        db_service
            .execute(
                "INSERT OR IGNORE INTO collection_materials (collection_id, material_id, added_at)
             VALUES (?1, ?2, ?3)",
                &[
                    collection_id.to_string(),
                    material_id.to_string(),
                    Utc::now().to_rfc3339(),
                ],
            )
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to add material to collection: {}", e))
            })?;

        Ok(())
    }

    /// Remove material from collection
    pub async fn remove_material_from_collection(
        &self,
        collection_id: Uuid,
        material_id: Uuid,
    ) -> DatabaseResult<()> {
        let db_service = self.db_service.read().await;

        db_service
            .execute(
                "DELETE FROM collection_materials WHERE collection_id = ?1 AND material_id = ?2",
                &[collection_id.to_string(), material_id.to_string()],
            )
            .await
            .map_err(|e| {
                DatabaseError::Service(format!("Failed to remove material from collection: {}", e))
            })?;

        Ok(())
    }

    /// Get materials in collection
    pub async fn get_collection_materials(
        &self,
        collection_id: Uuid,
    ) -> DatabaseResult<Vec<ResearchMaterial>> {
        let db_service = self.db_service.read().await;

        let rows = db_service
            .query(
                "SELECT rm.* FROM research_materials rm
             JOIN collection_materials cm ON rm.id = cm.material_id
             WHERE cm.collection_id = ?1
             ORDER BY cm.added_at DESC",
                &[collection_id.to_string()],
            )
            .await?;

        let mut materials = Vec::new();
        for row in rows {
            let material = ResearchMaterial {
                id: Uuid::parse_str(
                    row.get(0)
                        .ok_or_else(|| DatabaseError::Service("Failed to get ID".to_string()))?,
                )
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                project_id: Uuid::parse_str(row.get(1).ok_or_else(|| {
                    DatabaseError::Service("Failed to get project ID".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse UUID: {}", e)))?,
                material_type: self.parse_material_type(row.get(2).ok_or_else(|| {
                    DatabaseError::Service("Failed to get material type".to_string())
                })?)?,
                title: row
                    .get(3)
                    .ok_or_else(|| DatabaseError::Service("Failed to get title".to_string()))?
                    .to_string(),
                description: Some(
                    row.get(4)
                        .ok_or_else(|| {
                            DatabaseError::Service("Failed to get description".to_string())
                        })?
                        .to_string(),
                ),
                file_path: row.get(5).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                url: row.get(6).map(|s| s.to_string()),
                author: Some(
                    row.get(7)
                        .ok_or_else(|| DatabaseError::Service("Failed to get author".to_string()))?
                        .to_string(),
                ),
                publication_date: row
                    .get(8)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                accessed_date: row
                    .get(9)
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                tags: serde_json::from_str(
                    row.get(10)
                        .ok_or_else(|| DatabaseError::Service("Failed to get tags".to_string()))?,
                )
                .unwrap_or_default(),
                metadata: serde_json::from_str(
                    row.get(11).ok_or_else(|| {
                        DatabaseError::Service("Failed to get metadata".to_string())
                    })?,
                )
                .unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(row.get(12).ok_or_else(|| {
                    DatabaseError::Service("Failed to get created_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(row.get(13).ok_or_else(|| {
                    DatabaseError::Service("Failed to get updated_at".to_string())
                })?)
                .map_err(|e| DatabaseError::Service(format!("Failed to parse datetime: {}", e)))?
                .with_timezone(&Utc),
                file_size: row.get(14).and_then(|s| s.parse().ok()),
                thumbnail_path: row.get(15).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(s))
                    }
                }),
                extracted_text: row.get(16).map(|s| s.to_string()),
            };
            materials.push(material);
        }

        Ok(materials)
    }
}
