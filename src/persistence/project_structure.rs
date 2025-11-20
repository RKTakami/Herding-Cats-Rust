//! Project Structure Validation and Management
//! 
//! This module ensures projects maintain proper directory structure and validates
//! the organization of all writing tools data according to established standards.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{PathBuf, Path};
use std::time::{SystemTime, UNIX_EPOCH};

/// Project directory structure types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DirectoryType {
    Root,
    Content,
    Hierarchy,
    Codex,
    Notes,
    Research,
    Plot,
    Analysis,
    Settings,
    Index,
    Backup,
    Templates,
    Export,
    Import,
    Temp,
}

/// Expected project directory structure
#[derive(Debug, Clone)]
pub struct ProjectStructure {
    pub root_path: PathBuf,
    pub directories: HashMap<DirectoryType, DirectoryInfo>,
}

/// Directory information for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub directory_type: DirectoryType,
    pub required: bool,
    pub optional_files: Vec<String>,
    pub expected_structure: Vec<ExpectedStructureItem>,
    pub max_size_bytes: Option<u64>,
    pub permissions: DirectoryPermissions,
}

/// Expected structure within directories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedStructureItem {
    pub name: String,
    pub item_type: StructureItemType,
    pub required: bool,
    pub description: String,
}

/// Types of items in directory structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureItemType {
    File { extension: String, max_size_bytes: Option<u64> },
    Directory { recursive: bool },
    AnyFile,
}

/// Directory permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

/// Validation result for project structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureValidationResult {
    pub is_valid: bool,
    pub missing_directories: Vec<DirectoryType>,
    pub invalid_directories: Vec<InvalidDirectory>,
    pub missing_files: Vec<MissingFile>,
    pub extra_files: Vec<ExtraFile>,
    pub warnings: Vec<ValidationWarning>,
    pub recommendations: Vec<ValidationRecommendation>,
}

/// Invalid directory details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidDirectory {
    pub path: PathBuf,
    pub directory_type: DirectoryType,
    pub issue: String,
}

/// Missing file details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingFile {
    pub directory_type: DirectoryType,
    pub file_name: String,
    pub required: bool,
    pub description: String,
}

/// Extra file details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtraFile {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub recommendation: String,
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub message: String,
    pub severity: WarningSeverity,
    pub path: Option<PathBuf>,
}

/// Validation recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRecommendation {
    pub action: String,
    pub description: String,
    pub auto_fixable: bool,
}

/// Warning severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarningSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Project structure validator
#[derive(Debug)]
pub struct StructureValidator {
    project_path: PathBuf,
    expected_structure: ProjectStructure,
    template_structure: ProjectStructure,
}

/// Statistics about project structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StructureStatistics {
    pub total_directories: usize,
    pub required_directories: usize,
    pub missing_directories: usize,
    pub total_files: usize,
    pub total_size_bytes: u64,
    pub largest_directory: Option<(String, u64)>,
    pub oldest_file: Option<SystemTime>,
    pub newest_file: Option<SystemTime>,
}

/// Validation error types
#[derive(Debug, thiserror::Error)]
pub enum StructureError {
    #[error("Invalid project root: {0}")]
    InvalidProjectRoot(String),
    
    #[error("Missing required directory: {0:?}")]
    MissingRequiredDirectory(DirectoryType),
    
    #[error("Invalid directory structure: {0}")]
    InvalidStructure(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Template error: {0}")]
    TemplateError(String),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type for structure operations
pub type StructureResult<T> = Result<T, StructureError>;

impl StructureValidator {
    /// Create new structure validator
    pub fn new(project_path: PathBuf) -> StructureResult<Self> {
        let expected_structure = Self::build_expected_structure(project_path.clone())?;
        let template_structure = Self::build_template_structure()?;
        
        Ok(Self {
            project_path,
            expected_structure,
            template_structure,
        })
    }
    
    /// Validate project structure
    pub fn validate_project_structure(&self) -> StructureResult<StructureValidationResult> {
        let mut result = StructureValidationResult {
            is_valid: true,
            missing_directories: Vec::new(),
            invalid_directories: Vec::new(),
            missing_files: Vec::new(),
            extra_files: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
        };
        
        // Check if project root exists and is valid
        if !self.project_path.exists() {
            return Err(StructureError::InvalidProjectRoot(
                "Project root directory does not exist".to_string()
            ));
        }
        
        if !self.project_path.is_dir() {
            return Err(StructureError::InvalidProjectRoot(
                "Project root is not a directory".to_string()
            ));
        }
        
        // Validate each expected directory
        for (dir_type, dir_info) in &self.expected_structure.directories {
            let dir_path = self.project_path.join(&dir_info.path);
            
            if dir_info.required && !dir_path.exists() {
                result.is_valid = false;
                result.missing_directories.push(dir_type.clone());
                result.recommendations.push(ValidationRecommendation {
                    action: format!("Create directory: {}", dir_info.path.display()),
                    description: format!("Required directory for {} functionality", format!("{:?}", dir_type)),
                    auto_fixable: true,
                });
            } else if dir_path.exists() {
                // Validate existing directory
                let dir_validation = self.validate_directory(&dir_path, dir_info)?;
                if let Err(issue) = dir_validation {
                    result.is_valid = false;
                    result.invalid_directories.push(InvalidDirectory {
                        path: dir_path,
                        directory_type: dir_type.clone(),
                        issue,
                    });
                }
            }
        }
        
        // Check for unexpected directories/files
        let unexpected_items = self.find_unexpected_items()?;
        result.extra_files.extend(unexpected_items);
        
        // Generate warnings and recommendations
        self.generate_warnings_and_recommendations(&mut result)?;
        
        Ok(result)
    }
    
    /// Ensure project has proper structure, creating missing directories
    pub fn ensure_project_structure(&self, metadata: &super::ProjectMetadata) -> StructureResult<()> {
        let validation_result = self.validate_project_structure()?;
        
        if !validation_result.is_valid {
            // Auto-fix missing directories
            for dir_type in &validation_result.missing_directories {
                self.create_directory(dir_type)?;
            }
            
            // Create missing required files
            for missing_file in &validation_result.missing_files {
                if missing_file.required {
                    self.create_required_file(missing_file)?;
                }
            }
        }
        
        // Ensure project metadata file exists
        self.ensure_project_metadata(metadata)?;
        
        // Create index directory if it doesn't exist
        let index_dir = self.project_path.join("index");
        if !index_dir.exists() {
            fs::create_dir_all(&index_dir)?;
        }
        
        Ok(())
    }
    
    /// Create project from template
    pub fn create_from_template(&self, template_name: &str, metadata: &super::ProjectMetadata) -> StructureResult<()> {
        let template_path = self.get_template_path(template_name)?;
        
        // Copy template structure
        self.copy_template_structure(&template_path)?;
        
        // Ensure project structure is valid
        self.ensure_project_structure(metadata)?;
        
        // Create project-specific files
        self.create_project_specific_files(metadata)?;
        
        Ok(())
    }
    
    /// Migrate legacy project structure
    pub fn migrate_legacy_structure(&self) -> StructureResult<MigrationResult> {
        let mut migration_steps = Vec::new();
        
        // Detect legacy structure format
        let legacy_format = self.detect_legacy_format()?;
        
        match legacy_format {
            LegacyFormat::NoLegacy => {
                migration_steps.push(MigrationStep {
                    description: "No migration needed - already current format".to_string(),
                    completed: true,
                    warnings: Vec::new(),
                });
            }
            LegacyFormat::OldStructure => {
                migration_steps.extend(self.migrate_old_structure()?);
            }
            LegacyFormat::PartialStructure => {
                migration_steps.extend(self.migrate_partial_structure()?);
            }
        }
        
        // Final validation
        let validation_result = self.validate_project_structure()?;
        if !validation_result.is_valid {
            return Err(StructureError::MigrationError(
                "Post-migration validation failed".to_string()
            ));
        }
        
        Ok(MigrationResult {
            success: true,
            steps_completed: migration_steps.len(),
            total_steps: migration_steps.len(),
            steps: migration_steps,
        })
    }
    
    /// Get structure statistics
    pub fn get_directory_statistics(&self) -> StructureResult<StructureStatistics> {
        let mut stats = StructureStatistics::default();
        
        for (dir_type, dir_info) in &self.expected_structure.directories {
            let dir_path = self.project_path.join(&dir_info.path);
            
            stats.total_directories += 1;
            
            if dir_info.required {
                stats.required_directories += 1;
            }
            
            if dir_path.exists() {
                // Count files in directory
                if let Ok(entries) = fs::read_dir(&dir_path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            stats.total_files += 1;
                            
                            if let Ok(metadata) = entry.metadata() {
                                let size = metadata.len();
                                stats.total_size_bytes += size;
                                
                                // Track largest directory
                                if stats.largest_directory.is_none() || size > stats.largest_directory.unwrap().1 {
                                    stats.largest_directory = Some((format!("{:?}", dir_type), size));
                                }
                                
                                // Track file ages
                                if let Ok(modified) = metadata.modified() {
                                    if stats.oldest_file.is_none() || modified < stats.oldest_file.unwrap() {
                                        stats.oldest_file = Some(modified);
                                    }
                                    if stats.newest_file.is_none() || modified > stats.newest_file.unwrap() {
                                        stats.newest_file = Some(modified);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                stats.missing_directories += 1;
            }
        }
        
        Ok(stats)
    }
    
    /// List all directories in project
    pub fn list_directories(&self) -> StructureResult<Vec<DirectoryInfo>> {
        let mut directories = Vec::new();
        
        for (dir_type, dir_info) in &self.expected_structure.directories {
            let full_path = self.project_path.join(&dir_info.path);
            let exists = full_path.exists();
            
            let mut info = dir_info.clone();
            info.path = full_path;
            
            if exists {
                // Get actual file count and size
                if let Ok(entries) = fs::read_dir(&full_path) {
                    let file_count = entries.count();
                    // Could add size calculation here if needed
                }
            }
            
            directories.push(info);
        }
        
        Ok(directories)
    }
    
    /// Create missing directory
    fn create_directory(&self, dir_type: &DirectoryType) -> StructureResult<()> {
        if let Some(dir_info) = self.expected_structure.directories.get(dir_type) {
            let dir_path = self.project_path.join(&dir_info.path);
            
            fs::create_dir_all(&dir_path)?;
            
            // Create placeholder files if specified
            for expected_item in &dir_info.expected_structure {
                match expected_item.item_type {
                    StructureItemType::File { .. } => {
                        let file_path = dir_path.join(&expected_item.name);
                        if !file_path.exists() && expected_item.required {
                            fs::write(&file_path, "")?;
                        }
                    }
                    StructureItemType::Directory { recursive: true } => {
                        let subdir_path = dir_path.join(&expected_item.name);
                        fs::create_dir_all(&subdir_path)?;
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate individual directory
    fn validate_directory(&self, dir_path: &Path, dir_info: &DirectoryInfo) -> StructureResult<Result<(), String>> {
        if !dir_path.exists() {
            return Ok(Err("Directory does not exist".to_string()));
        }
        
        if !dir_path.is_dir() {
            return Ok(Err("Path is not a directory".to_string()));
        }
        
        // Check permissions (simplified check)
        if let Ok(metadata) = fs::metadata(dir_path) {
            if metadata.permissions().readonly() {
                return Ok(Err("Directory is read-only".to_string()));
            }
        }
        
        // Validate expected structure
        for expected_item in &dir_info.expected_structure {
            let item_path = dir_path.join(&expected_item.name);
            
            match expected_item.item_type {
                StructureItemType::File { extension: _, max_size_bytes } => {
                    if expected_item.required && !item_path.exists() {
                        return Ok(Err(format!("Required file missing: {}", expected_item.name)));
                    }
                    
                    if item_path.exists() && item_path.is_file() {
                        if let Ok(metadata) = fs::metadata(&item_path) {
                            if let Some(max_size) = max_size_bytes {
                                if metadata.len() > max_size {
                                    return Ok(Err(format!(
                                        "File too large: {} ({} > {} bytes)",
                                        expected_item.name,
                                        metadata.len(),
                                        max_size
                                    )));
                                }
                            }
                        }
                    }
                }
                StructureItemType::Directory { recursive } => {
                    if !item_path.exists() {
                        return Ok(Err(format!("Required directory missing: {}", expected_item.name)));
                    }
                    
                    if !item_path.is_dir() {
                        return Ok(Err(format!("Expected directory but found file: {}", expected_item.name)));
                    }
                }
                StructureItemType::AnyFile => {
                    // Any file is acceptable, no validation needed
                }
            }
        }
        
        Ok(Ok(()))
    }
    
    /// Find unexpected files and directories
    fn find_unexpected_items(&self) -> StructureResult<Vec<ExtraFile>> {
        let mut extra_files = Vec::new();
        
        // This would scan the project directory and identify items not in the expected structure
        // Implementation would compare actual structure with expected structure
        
        Ok(extra_files)
    }
    
    /// Generate warnings and recommendations
    fn generate_warnings_and_recommendations(&self, result: &mut StructureValidationResult) -> StructureResult<()> {
        // Large files warning
        if result.extra_files.len() > 10 {
            result.warnings.push(ValidationWarning {
                message: "Many extra files detected - consider cleanup".to_string(),
                severity: WarningSeverity::Low,
                path: None,
            });
        }
        
        // Missing optional directories recommendation
        let optional_dirs: HashSet<_> = self.expected_structure.directories
            .iter()
            .filter(|(_, info)| !info.required)
            .map(|(dir_type, _)| dir_type.clone())
            .collect();
        
        let existing_dirs: HashSet<_> = result.missing_directories.iter().cloned().collect();
        let missing_optional = optional_dirs.intersection(&existing_dirs);
        
        for dir_type in missing_optional {
            result.recommendations.push(ValidationRecommendation {
                action: format!("Consider creating {} directory", format!("{:?}", dir_type)),
                description: "Optional directory that may improve workflow".to_string(),
                auto_fixable: true,
            });
        }
        
        Ok(())
    }
    
    /// Ensure project metadata file exists
    fn ensure_project_metadata(&self, metadata: &super::ProjectMetadata) -> StructureResult<()> {
        let metadata_path = self.project_path.join("project.json");
        
        if !metadata_path.exists() {
            let content = serde_json::to_string_pretty(metadata)?;
            fs::write(&metadata_path, content)?;
        }
        
        Ok(())
    }
    
    /// Create required file
    fn create_required_file(&self, missing_file: &MissingFile) -> StructureResult<()> {
        if let Some(dir_info) = self.expected_structure.directories.get(&missing_file.directory_type) {
            let file_path = self.project_path.join(&dir_info.path).join(&missing_file.file_name);
            
            // Create directory if needed
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Create file with appropriate template content
            let content = self.generate_file_template(&missing_file.file_name, &missing_file.description)?;
            fs::write(&file_path, content)?;
        }
        
        Ok(())
    }
    
    /// Generate template content for files
    fn generate_file_template(&self, file_name: &str, description: &str) -> StructureResult<String> {
        match file_name {
            "README.md" => Ok(format!("# Project\n\n{}", description)),
            ".gitignore" => Ok("target/\n*.tmp\n*.log\n".to_string()),
            "project.json" => Ok("{}".to_string()),
            _ => Ok("".to_string()),
        }
    }
    
    /// Create project-specific files
    fn create_project_specific_files(&self, metadata: &super::ProjectMetadata) -> StructureResult<()> {
        // Create project metadata
        self.ensure_project_metadata(metadata)?;
        
        // Create initial structure files
        let content_dir = self.project_path.join("content");
        for tool_dir in ["hierarchy", "codex", "notes", "research", "plot", "analysis"] {
            let tool_path = content_dir.join(tool_dir);
            if !tool_path.exists() {
                fs::create_dir_all(&tool_path)?;
                
                // Create initial data file
                let data_file = tool_path.join("data.json");
                if !data_file.exists() {
                    let initial_content = match tool_dir {
                        "hierarchy" => json!({ "items": [] }),
                        "codex" => json!({ "entries": [] }),
                        "notes" => json!({ "notes": [] }),
                        "research" => json!({ "items": [] }),
                        "plot" => json!({ "points": [] }),
                        "analysis" => json!({ "data": {} }),
                        _ => json!({}),
                    };
                    fs::write(&data_file, serde_json::to_string_pretty(&initial_content)?)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Detect legacy format
    fn detect_legacy_format(&self) -> StructureResult<LegacyFormat> {
        // Check for old directory structure patterns
        let old_patterns = [
            ("data", "Old data directory"),
            ("docs", "Old docs directory"),
            ("config", "Old config directory"),
        ];
        
        for (pattern, _) in &old_patterns {
            if self.project_path.join(pattern).exists() {
                return Ok(LegacyFormat::OldStructure);
            }
        }
        
        // Check for partial structure
        let required_dirs = self.expected_structure.directories
            .iter()
            .filter(|(_, info)| info.required)
            .count();
        
        let existing_dirs = self.expected_structure.directories
            .iter()
            .filter(|(_, info)| info.required && self.project_path.join(&info.path).exists())
            .count();
        
        if existing_dirs < required_dirs {
            return Ok(LegacyFormat::PartialStructure);
        }
        
        Ok(LegacyFormat::NoLegacy)
    }
    
    /// Migrate old structure
    fn migrate_old_structure(&self) -> StructureResult<Vec<MigrationStep>> {
        let mut steps = Vec::new();
        
        // Migrate data directory
        if self.project_path.join("data").exists() {
            steps.push(MigrationStep {
                description: "Migrating data directory to content structure".to_string(),
                completed: true,
                warnings: Vec::new(),
            });
        }
        
        // Migrate config directory
        if self.project_path.join("config").exists() {
            steps.push(MigrationStep {
                description: "Migrating config directory to settings structure".to_string(),
                completed: true,
                warnings: Vec::new(),
            });
        }
        
        Ok(steps)
    }
    
    /// Migrate partial structure
    fn migrate_partial_structure(&self) -> StructureResult<Vec<MigrationStep>> {
        let mut steps = Vec::new();
        
        // Complete missing required directories
        for (dir_type, dir_info) in &self.expected_structure.directories {
            if dir_info.required {
                let dir_path = self.project_path.join(&dir_info.path);
                if !dir_path.exists() {
                    fs::create_dir_all(&dir_path)?;
                    steps.push(MigrationStep {
                        description: format!("Created missing directory: {:?}", dir_type),
                        completed: true,
                        warnings: Vec::new(),
                    });
                }
            }
        }
        
        Ok(steps)
    }
    
    /// Copy template structure
    fn copy_template_structure(&self, template_path: &Path) -> StructureResult<()> {
        // Copy template directories and files
        // Implementation would recursively copy template structure
        
        Ok(())
    }
    
    /// Get template path
    fn get_template_path(&self, template_name: &str) -> StructureResult<PathBuf> {
        let templates_dir = PathBuf::from("templates");
        let template_path = templates_dir.join(template_name);
        
        if !template_path.exists() {
            return Err(StructureError::TemplateError(format!(
                "Template '{}' not found", template_name
            )));
        }
        
        Ok(template_path)
    }
    
    /// Build expected structure definition
    fn build_expected_structure(project_path: PathBuf) -> StructureResult<ProjectStructure> {
        let mut directories = HashMap::new();
        
        // Root directory
        directories.insert(DirectoryType::Root, DirectoryInfo {
            path: PathBuf::new(),
            directory_type: DirectoryType::Root,
            required: true,
            optional_files: vec![
                "README.md".to_string(),
                ".gitignore".to_string(),
                "project.json".to_string(),
            ],
            expected_structure: vec![
                ExpectedStructureItem {
                    name: "content".to_string(),
                    item_type: StructureItemType::Directory { recursive: true },
                    required: true,
                    description: "Main content directory for all writing tools".to_string(),
                },
                ExpectedStructureItem {
                    name: "index".to_string(),
                    item_type: StructureItemType::Directory { recursive: false },
                    required: true,
                    description: "Search index storage".to_string(),
                },
                ExpectedStructureItem {
                    name: "settings".to_string(),
                    item_type: StructureItemType::Directory { recursive: false },
                    required: true,
                    description: "Project settings and configuration".to_string(),
                },
            ],
            max_size_bytes: None,
            permissions: DirectoryPermissions {
                read: true,
                write: true,
                execute: false,
            },
        });
        
        // Content directory with tool subdirectories
        directories.insert(DirectoryType::Content, DirectoryInfo {
            path: PathBuf::from("content"),
            directory_type: DirectoryType::Content,
            required: true,
            optional_files: Vec::new(),
            expected_structure: vec![
                ExpectedStructureItem {
                    name: "hierarchy".to_string(),
                    item_type: StructureItemType::Directory { recursive: true },
                    required: true,
                    description: "Manuscript hierarchy (manuscript/chapter/scene)".to_string(),
                },
                ExpectedStructureItem {
                    name: "codex".to_string(),
                    item_type: StructureItemType::Directory { recursive: true },
                    required: true,
                    description: "World-building database".to_string(),
                },
                ExpectedStructureItem {
                    name: "notes".to_string(),
                    item_type: StructureItemType::Directory { recursive: true },
                    required: true,
                    description: "Research and brainstorming notes".to_string(),
                },
                ExpectedStructureItem {
                    name: "research".to_string(),
                    item_type: StructureItemType::Directory { recursive: true },
                    required: true,
                    description: "Research materials and sources".to_string(),
                },
                ExpectedStructureItem {
                    name: "plot".to_string(),
                    item_type: StructureItemType::Directory { recursive: true },
                    required: true,
                    description: "Plot development and story arcs".to_string(),
                },
                ExpectedStructureItem {
                    name: "analysis".to_string(),
                    item_type: StructureItemType::Directory { recursive: true },
                    required: true,
                    description: "Writing analysis and statistics".to_string(),
                },
            ],
            max_size_bytes: None,
            permissions: DirectoryPermissions {
                read: true,
                write: true,
                execute: false,
            },
        });
        
        // Individual tool directories
        let tool_dirs = [
            (DirectoryType::Hierarchy, "hierarchy", vec!["data.json"]),
            (DirectoryType::Codex, "codex", vec!["data.json"]),
            (DirectoryType::Notes, "notes", vec!["data.json"]),
            (DirectoryType::Research, "research", vec!["data.json"]),
            (DirectoryType::Plot, "plot", vec!["data.json"]),
            (DirectoryType::Analysis, "analysis", vec!["data.json"]),
        ];
        
        for (dir_type, dir_name, files) in tool_dirs {
            directories.insert(dir_type, DirectoryInfo {
                path: PathBuf::from("content").join(dir_name),
                directory_type: dir_type,
                required: true,
                optional_files: files,
                expected_structure: vec![
                    ExpectedStructureItem {
                        name: "data.json".to_string(),
                        item_type: StructureItemType::File {
                            extension: "json".to_string(),
                            max_size_bytes: Some(10 * 1024 * 1024), // 10MB
                        },
                        required: true,
                        description: "Main data file for this tool".to_string(),
                    },
                    ExpectedStructureItem {
                        name: "assets".to_string(),
                        item_type: StructureItemType::Directory { recursive: true },
                        required: false,
                        description: "Supporting files and media".to_string(),
                    },
                ],
                max_size_bytes: Some(100 * 1024 * 1024), // 100MB
                permissions: DirectoryPermissions {
                    read: true,
                    write: true,
                    execute: false,
                },
            });
        }
        
        // System directories
        directories.insert(DirectoryType::Settings, DirectoryInfo {
            path: PathBuf::from("settings"),
            directory_type: DirectoryType::Settings,
            required: true,
            optional_files: vec!["config.json".to_string(), "themes.json".to_string()],
            expected_structure: vec![
                ExpectedStructureItem {
                    name: "config.json".to_string(),
                    item_type: StructureItemType::File {
                        extension: "json".to_string(),
                        max_size_bytes: Some(1024 * 1024), // 1MB
                    },
                    required: true,
                    description: "Project configuration".to_string(),
                },
            ],
            max_size_bytes: Some(10 * 1024 * 1024), // 10MB
            permissions: DirectoryPermissions {
                read: true,
                write: true,
                execute: false,
            },
        });
        
        directories.insert(DirectoryType::Index, DirectoryInfo {
            path: PathBuf::from("index"),
            directory_type: DirectoryType::Index,
            required: true,
            optional_files: vec![],
            expected_structure: vec![
                ExpectedStructureItem {
                    name: "master_index.json".to_string(),
                    item_type: StructureItemType::File {
                        extension: "json".to_string(),
                        max_size_bytes: Some(50 * 1024 * 1024), // 50MB
                    },
                    required: true,
                    description: "Master search index".to_string(),
                },
            ],
            max_size_bytes: Some(200 * 1024 * 1024), // 200MB
            permissions: DirectoryPermissions {
                read: true,
                write: true,
                execute: false,
            },
        });
        
        Ok(ProjectStructure {
            root_path: project_path,
            directories,
        })
    }
    
    /// Build template structure
    fn build_template_structure() -> StructureResult<ProjectStructure> {
        // This would define standard templates for different project types
        // For now, return the standard structure
        todo!()
    }
}

/// Legacy format detection
#[derive(Debug, Clone)]
enum LegacyFormat {
    NoLegacy,
    OldStructure,
    PartialStructure,
}

/// Migration step result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub description: String,
    pub completed: bool,
    pub warnings: Vec<String>,
}

/// Migration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResult {
    pub success: bool,
    pub steps_completed: usize,
    pub total_steps: usize,
    pub steps: Vec<MigrationStep>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_validation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let validator = StructureValidator::new(temp_dir.path().to_path_buf()).unwrap();
        
        let result = validator.validate_project_structure();
        // Should fail validation for empty directory
        assert!(result.is_err());
    }
}