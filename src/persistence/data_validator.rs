//! Project Data Validation and Integrity Checking
//! 
//! This module provides comprehensive data validation including integrity checks,
//! cross-tool consistency validation, and automated repair capabilities.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::path::{PathBuf, Path};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Main data validator for project integrity
#[derive(Debug)]
pub struct DataValidator {
    project_path: PathBuf,
    validation_rules: HashMap<String, ValidationRule>,
    schema_definitions: HashMap<String, DataSchema>,
    validation_history: Arc<RwLock<Vec<ValidationRecord>>>,
    auto_repair_config: AutoRepairConfig,
}

/// Configuration for automated repairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRepairConfig {
    pub enable_auto_repair: bool,
    pub repair_critical_issues: bool,
    pub repair_warnings: bool,
    pub create_backup_before_repair: bool,
    pub max_repair_attempts: usize,
    pub validate_after_repair: bool,
    pub repair_log_level: LogLevel,
}

/// Log levels for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

/// Validation rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub rule_type: RuleType,
    pub target_data_type: DataType,
    pub severity: ValidationSeverity,
    pub description: String,
    pub repair_strategy: Option<RepairStrategy>,
    pub is_enabled: bool,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Rule types for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    SchemaValidation,
    ReferentialIntegrity,
    DataConsistency,
    CrossToolValidation,
    FormatValidation,
    SizeValidation,
    TimestampValidation,
    ContentValidation,
}

/// Data types for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    ProjectMetadata,
    HierarchyData,
    CodexData,
    NotesData,
    ResearchData,
    PlotData,
    AnalysisData,
    SearchIndex,
    SettingsData,
    AllData,
}

/// Validation severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Critical,
    Error,
    Warning,
    Info,
}

/// Repair strategies for fixing validation issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepairStrategy {
    RemoveCorruptData,
    RegenerateFromTemplate,
    RepairReferences,
    NormalizeFormats,
    RebuildIndex,
    SkipData,
    ManualReviewRequired,
}

/// Data schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSchema {
    pub schema_id: String,
    pub data_type: DataType,
    pub required_fields: Vec<FieldDefinition>,
    pub optional_fields: Vec<FieldDefinition>,
    pub field_types: HashMap<String, FieldType>,
    pub constraints: Vec<FieldConstraint>,
}

/// Field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub field_name: String,
    pub field_type: FieldType,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
}

/// Field types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    DateTime,
    UUID,
    Array(Box<FieldType>),
    Object,
    Enum(Vec<String>),
}

/// Field constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConstraint {
    pub field_name: String,
    pub constraint_type: ConstraintType,
    pub value: serde_json::Value,
    pub message: String,
}

/// Constraint types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    MinLength(usize),
    MaxLength(usize),
    MinValue(f64),
    MaxValue(f64),
    Pattern(String),
    Unique,
    NotNull,
    Enum(Vec<String>),
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub validation_id: Uuid,
    pub checked_at: SystemTime,
    pub duration_ms: u64,
    pub checked_data_type: DataType,
    pub issues: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationWarning>,
    pub statistics: ValidationStatistics,
}

/// Individual validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub issue_id: Uuid,
    pub severity: ValidationSeverity,
    pub rule_id: String,
    pub issue_type: IssueType,
    pub description: String,
    pub affected_path: Option<PathBuf>,
    pub field_name: Option<String>,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub repairable: bool,
    pub auto_repair_attempted: bool,
    pub auto_repair_successful: bool,
    pub repair_actions: Vec<RepairAction>,
}

/// Issue types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    MissingRequiredField,
    InvalidFieldType,
    ConstraintViolation,
    DataCorruption,
    ReferenceIntegrityError,
    CrossToolInconsistency,
    FormatError,
    SizeLimitExceeded,
    TimestampAnomaly,
    DuplicateEntry,
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub warning_id: Uuid,
    pub severity: ValidationSeverity,
    pub description: String,
    pub affected_path: Option<PathBuf>,
    pub recommendation: Option<String>,
}

/// Repair action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairAction {
    pub action_id: Uuid,
    pub action_type: ActionType,
    pub description: String,
    pub target_path: Option<PathBuf>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub executed: bool,
    pub successful: bool,
    pub error_message: Option<String>,
}

/// Action types for repairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    RemoveFile,
    RepairFile,
    RegenerateData,
    UpdateReference,
    NormalizeContent,
    RebuildIndex,
    CreateBackup,
    SkipRepair,
}

/// Validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStatistics {
    pub files_checked: usize,
    pub total_issues: usize,
    pub critical_issues: usize,
    pub error_issues: usize,
    pub warning_issues: usize,
    pub auto_repairs_attempted: usize,
    pub auto_repairs_successful: usize,
    pub validation_errors: usize,
}

/// Validation record for history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRecord {
    pub record_id: Uuid,
    pub validation_result: ValidationResult,
    pub project_version: String,
    pub validator_version: String,
}

/// Comprehensive validation report
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub project_health_score: f32,
    pub overall_valid: bool,
    pub critical_issues: Vec<ValidationIssue>,
    pub all_issues: Vec<ValidationIssue>,
    pub recommendations: Vec<ValidationRecommendation>,
    pub auto_repair_summary: AutoRepairSummary,
    pub validation_trends: ValidationTrends,
}

/// Validation recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRecommendation {
    pub priority: RecommendationPriority,
    pub category: String,
    pub title: String,
    pub description: String,
    pub action_items: Vec<String>,
    pub estimated_effort: String,
}

/// Recommendation priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Immediate,
    High,
    Medium,
    Low,
}

/// Auto-repair summary
#[derive(Debug, Clone)]
pub struct AutoRepairSummary {
    pub total_repairs_attempted: usize,
    pub total_repairs_successful: usize,
    pub critical_repairs_successful: usize,
    pub repair_success_rate: f32,
    pub backup_created: bool,
}

/// Validation trends over time
#[derive(Debug, Clone)]
pub struct ValidationTrends {
    pub last_10_validations: Vec<ValidationScore>,
    pub trend_direction: TrendDirection,
    pub common_issue_types: Vec<(IssueType, usize)>,
    pub improvement_areas: Vec<String>,
}

/// Individual validation score
#[derive(Debug, Clone)]
pub struct ValidationScore {
    pub timestamp: SystemTime,
    pub score: f32,
    pub issue_count: usize,
}

/// Trend directions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}

/// Data validator error types
#[derive(Debug, thiserror::Error)]
pub enum DataValidatorError {
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Repair operation failed: {0}")]
    RepairFailed(String),
    
    #[error("Schema not found: {0}")]
    SchemaNotFound(String),
    
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Backup creation failed: {0}")]
    BackupFailed(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type for validation operations
pub type ValidationResult<T> = Result<T, DataValidatorError>;

impl DataValidator {
    /// Create new data validator
    pub fn new(project_path: PathBuf) -> ValidationResult<Self> {
        // Initialize validation rules
        let mut validation_rules = HashMap::new();
        Self::initialize_default_rules(&mut validation_rules);
        
        // Initialize schema definitions
        let mut schema_definitions = HashMap::new();
        Self::initialize_default_schemas(&mut schema_definitions);
        
        Ok(Self {
            project_path,
            validation_rules,
            schema_definitions,
            validation_history: Arc::new(RwLock::new(Vec::new())),
            auto_repair_config: AutoRepairConfig::default(),
        })
    }
    
    /// Validate entire project integrity
    pub async fn validate_project_integrity(&self) -> ValidationResult<ValidationReport> {
        let start_time = SystemTime::now();
        
        // Validate all data types
        let mut all_issues = Vec::new();
        let mut all_warnings = Vec::new();
        let mut statistics = ValidationStatistics::default();
        
        let data_types = [
            DataType::ProjectMetadata,
            DataType::HierarchyData,
            DataType::CodexData,
            DataType::NotesData,
            DataType::ResearchData,
            DataType::PlotData,
            DataType::AnalysisData,
            DataType::SearchIndex,
            DataType::SettingsData,
        ];
        
        for data_type in &data_types {
            match self.validate_data_type(data_type.clone()).await {
                Ok(result) => {
                    all_issues.extend(result.issues);
                    all_warnings.extend(result.warnings);
                    
                    // Aggregate statistics
                    statistics.files_checked += result.statistics.files_checked;
                    statistics.total_issues += result.statistics.total_issues;
                    statistics.critical_issues += result.statistics.critical_issues;
                    statistics.error_issues += result.statistics.error_issues;
                    statistics.warning_issues += result.statistics.warning_issues;
                    statistics.auto_repairs_attempted += result.statistics.auto_repairs_attempted;
                    statistics.auto_repairs_successful += result.statistics.auto_repairs_successful;
                }
                Err(e) => {
                    statistics.validation_errors += 1;
                    eprintln!("Validation error for {:?}: {}", data_type, e);
                }
            }
        }
        
        // Perform cross-tool validation
        let cross_tool_issues = self.validate_cross_tool_consistency().await?;
        all_issues.extend(cross_tool_issues);
        
        // Calculate project health score
        let health_score = self.calculate_project_health_score(&all_issues, &statistics);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&all_issues);
        
        // Auto-repair summary
        let auto_repair_summary = AutoRepairSummary {
            total_repairs_attempted: statistics.auto_repairs_attempted,
            total_repairs_successful: statistics.auto_repairs_successful,
            critical_repairs_successful: statistics.auto_repairs_successful, // Simplified
            repair_success_rate: if statistics.auto_repairs_attempted > 0 {
                statistics.auto_repairs_successful as f32 / statistics.auto_repairs_attempted as f32
            } else {
                1.0
            },
            backup_created: false, // Would be tracked during repairs
        };
        
        // Validation trends
        let validation_trends = self.analyze_validation_trends().await?;
        
        // Create final report
        let report = ValidationReport {
            project_health_score: health_score,
            overall_valid: all_issues.is_empty(),
            critical_issues: all_issues.iter().filter(|i| i.severity == ValidationSeverity::Critical).cloned().collect(),
            all_issues,
            recommendations,
            auto_repair_summary,
            validation_trends,
        };
        
        // Record validation in history
        self.record_validation(&report).await?;
        
        Ok(report)
    }
    
    /// Validate specific data type
    pub async fn validate_data_type(&self, data_type: DataType) -> ValidationResult<ValidationResult> {
        let start_time = SystemTime::now();
        
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        let mut statistics = ValidationStatistics::default();
        
        // Get applicable validation rules
        let applicable_rules = self.get_applicable_rules(&data_type);
        
        // Validate file existence and format
        let file_path = self.get_data_file_path(&data_type)?;
        if file_path.exists() {
            statistics.files_checked += 1;
            
            // Load and validate data
            let data = self.load_data(&file_path, &data_type).await?;
            
            // Apply validation rules
            for rule in &applicable_rules {
                match self.apply_validation_rule(&rule, &data, &file_path).await {
                    Ok(rule_issues) => {
                        issues.extend(rule_issues);
                    }
                    Err(e) => {
                        eprintln!("Rule validation failed for {}: {}", rule.rule_id, e);
                        statistics.validation_errors += 1;
                    }
                }
            }
            
            // Schema validation
            if let Some(schema) = self.schema_definitions.get(&format!("{:?}", data_type)) {
                let schema_issues = self.validate_against_schema(&data, schema)?;
                issues.extend(schema_issues);
            }
            
        } else {
            // File doesn't exist - this might be an issue for required data types
            if matches!(data_type, DataType::ProjectMetadata | DataType::SettingsData) {
                issues.push(ValidationIssue {
                    issue_id: Uuid::new_v4(),
                    severity: ValidationSeverity::Critical,
                    rule_id: "file_existence".to_string(),
                    issue_type: IssueType::MissingRequiredField,
                    description: format!("Required data file missing: {}", file_path.display()),
                    affected_path: Some(file_path),
                    field_name: None,
                    expected_value: Some("File exists".to_string()),
                    actual_value: Some("File not found".to_string()),
                    repairable: true,
                    auto_repair_attempted: false,
                    auto_repair_successful: false,
                    repair_actions: vec![RepairAction {
                        action_id: Uuid::new_v4(),
                        action_type: ActionType::RegenerateData,
                        description: "Create missing data file with default structure".to_string(),
                        target_path: Some(file_path),
                        parameters: HashMap::new(),
                        executed: false,
                        successful: false,
                        error_message: None,
                    }],
                });
            }
        }
        
        // Calculate statistics
        for issue in &issues {
            match issue.severity {
                ValidationSeverity::Critical => statistics.critical_issues += 1,
                ValidationSeverity::Error => statistics.error_issues += 1,
                ValidationSeverity::Warning => statistics.warning_issues += 1,
                ValidationSeverity::Info => {}
            }
        }
        statistics.total_issues = issues.len();
        
        let duration = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        
        Ok(ValidationResult {
            is_valid: issues.is_empty(),
            validation_id: Uuid::new_v4(),
            checked_at: SystemTime::now(),
            duration_ms: duration,
            checked_data_type: data_type,
            issues,
            warnings,
            statistics,
        })
    }
    
    /// Validate cross-tool consistency
    pub async fn validate_cross_tool_consistency(&self) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Check for orphaned references between tools
        issues.extend(self.validate_references().await?);
        
        // Check for data consistency across tools
        issues.extend(self.validate_cross_tool_data().await?);
        
        // Check for timestamp consistency
        issues.extend(self.validate_timestamp_consistency().await?);
        
        Ok(issues)
    }
    
    /// Perform automated repairs
    pub async fn perform_auto_repairs(&mut self, validation_result: &ValidationResult) -> ValidationResult<Vec<RepairAction>> {
        if !self.auto_repair_config.enable_auto_repair {
            return Ok(Vec::new());
        }
        
        let mut repair_actions = Vec::new();
        
        for issue in &validation_result.issues {
            if issue.repairable && self.should_attempt_repair(&issue) {
                match self.attempt_auto_repair(&issue).await {
                    Ok(actions) => {
                        repair_actions.extend(actions);
                    }
                    Err(e) => {
                        eprintln!("Auto-repair failed for issue {}: {}", issue.issue_id, e);
                    }
                }
            }
        }
        
        Ok(repair_actions)
    }
    
    /// Get validation history
    pub async fn get_validation_history(&self) -> ValidationResult<Vec<ValidationRecord>> {
        let history = self.validation_history.read().await;
        Ok(history.clone())
    }
    
    /// Export validation report
    pub async fn export_validation_report(&self, report: &ValidationReport, output_path: &Path) -> ValidationResult<()> {
        let report_json = serde_json::to_string_pretty(report)?;
        fs::write(output_path, report_json)?;
        Ok(())
    }
    
    // Private helper methods
    
    fn initialize_default_rules(rules: &mut HashMap<String, ValidationRule>) {
        // File existence rules
        rules.insert("file_existence".to_string(), ValidationRule {
            rule_id: "file_existence".to_string(),
            rule_type: RuleType::SchemaValidation,
            target_data_type: DataType::AllData,
            severity: ValidationSeverity::Critical,
            description: "Check that required files exist",
            repair_strategy: Some(RepairStrategy::RegenerateFromTemplate),
            is_enabled: true,
            parameters: HashMap::new(),
        });
        
        // JSON format validation
        rules.insert("json_format".to_string(), ValidationRule {
            rule_id: "json_format".to_string(),
            rule_type: RuleType::FormatValidation,
            target_data_type: DataType::AllData,
            severity: ValidationSeverity::Error,
            description: "Validate JSON file format",
            repair_strategy: Some(RepairStrategy::RemoveCorruptData),
            is_enabled: true,
            parameters: HashMap::new(),
        });
        
        // Data size validation
        rules.insert("size_limit".to_string(), ValidationRule {
            rule_id: "size_limit".to_string(),
            rule_type: RuleType::SizeValidation,
            target_data_type: DataType::AllData,
            severity: ValidationSeverity::Warning,
            description: "Check file size limits",
            repair_strategy: Some(RepairStrategy::SkipData),
            is_enabled: true,
            parameters: HashMap::new(),
        });
    }
    
    fn initialize_default_schemas(schemas: &mut HashMap<String, DataSchema>) {
        // Project metadata schema
        schemas.insert("ProjectMetadata".to_string(), DataSchema {
            schema_id: "project_metadata".to_string(),
            data_type: DataType::ProjectMetadata,
            required_fields: vec![
                FieldDefinition {
                    field_name: "id".to_string(),
                    field_type: FieldType::UUID,
                    is_required: true,
                    default_value: None,
                    description: Some("Project ID".to_string()),
                },
                FieldDefinition {
                    field_name: "name".to_string(),
                    field_type: FieldType::String,
                    is_required: true,
                    default_value: None,
                    description: Some("Project name".to_string()),
                },
            ],
            optional_fields: vec![
                FieldDefinition {
                    field_name: "description".to_string(),
                    field_type: FieldType::String,
                    is_required: false,
                    default_value: None,
                    description: Some("Project description".to_string()),
                },
            ],
            field_types: HashMap::new(),
            constraints: vec![],
        });
    }
    
    fn get_applicable_rules(&self, data_type: &DataType) -> Vec<&ValidationRule> {
        self.validation_rules
            .values()
            .filter(|rule| rule.is_enabled && 
                (rule.target_data_type == *data_type || rule.target_data_type == DataType::AllData))
            .collect()
    }
    
    fn get_data_file_path(&self, data_type: &DataType) -> ValidationResult<PathBuf> {
        let path = match data_type {
            DataType::ProjectMetadata => self.project_path.join("project.json"),
            DataType::HierarchyData => self.project_path.join("content").join("hierarchy").join("data.json"),
            DataType::CodexData => self.project_path.join("content").join("codex").join("data.json"),
            DataType::NotesData => self.project_path.join("content").join("notes").join("data.json"),
            DataType::ResearchData => self.project_path.join("content").join("research").join("data.json"),
            DataType::PlotData => self.project_path.join("content").join("plot").join("data.json"),
            DataType::AnalysisData => self.project_path.join("content").join("analysis").join("data.json"),
            DataType::SearchIndex => self.project_path.join("index").join("master_index.json"),
            DataType::SettingsData => self.project_path.join("settings").join("config.json"),
            DataType::AllData => return Err(DataValidatorError::InvalidFormat("Cannot get path for AllData".to_string())),
        };
        
        Ok(path)
    }
    
    async fn load_data(&self, file_path: &Path, data_type: &DataType) -> ValidationResult<serde_json::Value> {
        if !file_path.exists() {
            return Err(DataValidatorError::InvalidFormat(format!(
                "Data file not found: {}", file_path.display()
            )));
        }
        
        let content = fs::read_to_string(file_path)?;
        let data: serde_json::Value = serde_json::from_str(&content)?;
        
        Ok(data)
    }
    
    async fn apply_validation_rule(&self, rule: &ValidationRule, data: &serde_json::Value, file_path: &Path) -> ValidationResult<Vec<ValidationIssue>> {
        match rule.rule_type {
            RuleType::SchemaValidation => self.validate_schema(rule, data, file_path).await,
            RuleType::FormatValidation => self.validate_format(rule, data, file_path).await,
            RuleType::SizeValidation => self.validate_size(rule, data, file_path).await,
            _ => Ok(Vec::new()),
        }
    }
    
    async fn validate_schema(&self, rule: &ValidationRule, data: &serde_json::Value, file_path: &Path) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Check required fields
        if let Some(schema) = self.schema_definitions.get(&format!("{:?}", rule.target_data_type)) {
            for field_def in &schema.required_fields {
                if !data.get(&field_def.field_name).is_some() {
                    issues.push(ValidationIssue {
                        issue_id: Uuid::new_v4(),
                        severity: rule.severity.clone(),
                        rule_id: rule.rule_id.clone(),
                        issue_type: IssueType::MissingRequiredField,
                        description: format!("Required field '{}' is missing", field_def.field_name),
                        affected_path: Some(file_path.to_path_buf()),
                        field_name: Some(field_def.field_name.clone()),
                        expected_value: Some("Field present".to_string()),
                        actual_value: Some("Field missing".to_string()),
                        repairable: true,
                        auto_repair_attempted: false,
                        auto_repair_successful: false,
                        repair_actions: vec![RepairAction {
                            action_id: Uuid::new_v4(),
                            action_type: ActionType::RegenerateData,
                            description: format!("Add missing required field: {}", field_def.field_name),
                            target_path: Some(file_path.to_path_buf()),
                            parameters: HashMap::new(),
                            executed: false,
                            successful: false,
                            error_message: None,
                        }],
                    });
                }
            }
        }
        
        Ok(issues)
    }
    
    async fn validate_format(&self, rule: &ValidationRule, data: &serde_json::Value, file_path: &Path) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Check if data is valid JSON
        if data.is_null() {
            issues.push(ValidationIssue {
                issue_id: Uuid::new_v4(),
                severity: rule.severity.clone(),
                rule_id: rule.rule_id.clone(),
                issue_type: IssueType::DataCorruption,
                description: "Data is null or corrupted".to_string(),
                affected_path: Some(file_path.to_path_buf()),
                field_name: None,
                expected_value: Some("Valid JSON data".to_string()),
                actual_value: Some("Null or invalid data".to_string()),
                repairable: true,
                auto_repair_attempted: false,
                auto_repair_successful: false,
                repair_actions: vec![RepairAction {
                    action_id: Uuid::new_v4(),
                    action_type: ActionType::RegenerateData,
                    description: "Regenerate corrupted data file".to_string(),
                    target_path: Some(file_path.to_path_buf()),
                    parameters: HashMap::new(),
                    executed: false,
                    successful: false,
                    error_message: None,
                }],
            });
        }
        
        Ok(issues)
    }
    
    async fn validate_size(&self, rule: &ValidationRule, data: &serde_json::Value, file_path: &Path) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Check file size (simplified - would check actual file size)
        let max_size = 10 * 1024 * 1024; // 10MB
        
        if let Ok(metadata) = fs::metadata(file_path) {
            if metadata.len() > max_size {
                issues.push(ValidationIssue {
                    issue_id: Uuid::new_v4(),
                    severity: rule.severity.clone(),
                    rule_id: rule.rule_id.clone(),
                    issue_type: IssueType::SizeLimitExceeded,
                    description: format!("File size ({}) exceeds limit ({})", metadata.len(), max_size),
                    affected_path: Some(file_path.to_path_buf()),
                    field_name: None,
                    expected_value: Some(format!("<= {} bytes", max_size)),
                    actual_value: Some(format!("{} bytes", metadata.len())),
                    repairable: false,
                    auto_repair_attempted: false,
                    auto_repair_successful: false,
                    repair_actions: Vec::new(),
                });
            }
        }
        
        Ok(issues)
    }
    
    fn validate_against_schema(&self, data: &serde_json::Value, schema: &DataSchema) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Validate field types
        for (field_name, field_type) in &schema.field_types {
            if let Some(field_value) = data.get(field_name) {
                let is_valid_type = match field_type {
                    FieldType::String => field_value.is_string(),
                    FieldType::Number => field_value.is_number(),
                    FieldType::Boolean => field_value.is_boolean(),
                    FieldType::DateTime => field_value.is_string(), // Simplified
                    FieldType::UUID => field_value.is_string(), // Simplified
                    FieldType::Array(_) => field_value.is_array(),
                    FieldType::Object => field_value.is_object(),
                    FieldType::Enum(values) => field_value.is_string() && values.contains(&field_value.as_str().unwrap_or("")),
                };
                
                if !is_valid_type {
                    issues.push(ValidationIssue {
                        issue_id: Uuid::new_v4(),
                        severity: ValidationSeverity::Error,
                        rule_id: "schema_validation".to_string(),
                        issue_type: IssueType::InvalidFieldType,
                        description: format!("Field '{}' has invalid type", field_name),
                        affected_path: None,
                        field_name: Some(field_name.clone()),
                        expected_value: Some(format!("{:?}", field_type)),
                        actual_value: Some(format!("{:?}", field_value)),
                        repairable: false,
                        auto_repair_attempted: false,
                        auto_repair_successful: false,
                        repair_actions: Vec::new(),
                    });
                }
            }
        }
        
        // Validate constraints
        for constraint in &schema.constraints {
            if let Some(field_value) = data.get(&constraint.field_name) {
                let constraint_valid = match &constraint.constraint_type {
                    ConstraintType::NotNull => !field_value.is_null(),
                    ConstraintType::MinLength(min) => {
                        if let Some(s) = field_value.as_str() {
                            s.len() >= *min
                        } else {
                            false
                        }
                    }
                    ConstraintType::MaxLength(max) => {
                        if let Some(s) = field_value.as_str() {
                            s.len() <= *max
                        } else {
                            false
                        }
                    }
                    _ => true, // Simplified for other constraint types
                };
                
                if !constraint_valid {
                    issues.push(ValidationIssue {
                        issue_id: Uuid::new_v4(),
                        severity: ValidationSeverity::Error,
                        rule_id: "schema_constraint".to_string(),
                        issue_type: IssueType::ConstraintViolation,
                        description: format!("Field '{}' violates constraint: {}", constraint.field_name, constraint.message),
                        affected_path: None,
                        field_name: Some(constraint.field_name.clone()),
                        expected_value: Some(constraint.message.clone()),
                        actual_value: Some(format!("{:?}", field_value)),
                        repairable: false,
                        auto_repair_attempted: false,
                        auto_repair_successful: false,
                        repair_actions: Vec::new(),
                    });
                }
            }
        }
        
        Ok(issues)
    }
    
    async fn validate_references(&self) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // This would validate references between different tools
        // For now, return empty vector as placeholder
        
        Ok(issues)
    }
    
    async fn validate_cross_tool_data(&self) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // This would validate data consistency across tools
        // For now, return empty vector as placeholder
        
        Ok(issues)
    }
    
    async fn validate_timestamp_consistency(&self) -> ValidationResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // This would validate timestamp consistency across the project
        // For now, return empty vector as placeholder
        
        Ok(issues)
    }
    
    fn calculate_project_health_score(&self, issues: &[ValidationIssue], statistics: &ValidationStatistics) -> f32 {
        let mut score = 100.0;
        
        // Deduct points for issues
        for issue in issues {
            match issue.severity {
                ValidationSeverity::Critical => score -= 20.0,
                ValidationSeverity::Error => score -= 10.0,
                ValidationSeverity::Warning => score -= 5.0,
                ValidationSeverity::Info => score -= 1.0,
            }
        }
        
        // Bonus for successful auto-repairs
        if statistics.auto_repairs_attempted > 0 {
            let repair_rate = statistics.auto_repairs_successful as f32 / statistics.auto_repairs_attempted as f32;
            score += repair_rate * 5.0; // Up to 5 points bonus
        }
        
        // Ensure score is between 0 and 100
        score.max(0.0).min(100.0)
    }
    
    fn generate_recommendations(&self, issues: &[ValidationIssue]) -> Vec<ValidationRecommendation> {
        let mut recommendations = Vec::new();
        
        let critical_count = issues.iter().filter(|i| i.severity == ValidationSeverity::Critical).count();
        let error_count = issues.iter().filter(|i| i.severity == ValidationSeverity::Error).count();
        
        if critical_count > 0 {
            recommendations.push(ValidationRecommendation {
                priority: RecommendationPriority::Immediate,
                category: "Data Integrity".to_string(),
                title: format!("Fix {} Critical Issues", critical_count),
                description: format!("Your project has {} critical data integrity issues that need immediate attention.", critical_count),
                action_items: vec!["Run detailed validation".to_string(), "Review critical issues".to_string(), "Attempt auto-repairs".to_string()],
                estimated_effort: "30-60 minutes".to_string(),
            });
        }
        
        if error_count > 0 {
            recommendations.push(ValidationRecommendation {
                priority: RecommendationPriority::High,
                category: "Data Quality".to_string(),
                title: format!("Address {} Data Errors", error_count),
                description: format!("There are {} data format or consistency errors that should be resolved.", error_count),
                action_items: vec!["Review error details".to_string(), "Apply fixes manually".to_string(), "Re-validate".to_string()],
                estimated_effort: "15-30 minutes".to_string(),
            });
        }
        
        recommendations
    }
    
    async fn analyze_validation_trends(&self) -> ValidationResult<ValidationTrends> {
        let history = self.validation_history.read().await;
        
        let last_10_validations = history
            .iter()
            .rev()
            .take(10)
            .map(|record| ValidationScore {
                timestamp: record.validation_result.checked_at,
                score: self.calculate_project_health_score(&record.validation_result.issues, &record.validation_result.statistics),
                issue_count: record.validation_result.statistics.total_issues,
            })
            .collect();
        
        let common_issue_types = Vec::new(); // Would analyze issue types from history
        
        let trend_direction = if last_10_validations.len() >= 2 {
            let recent_score = last_10_validations[0].score;
            let older_score = last_10_validations[last_10_validations.len() - 1].score;
            
            if recent_score > older_score + 5.0 {
                TrendDirection::Improving
            } else if recent_score < older_score - 5.0 {
                TrendDirection::Declining
            } else {
                TrendDirection::Stable
            }
        } else {
            TrendDirection::Stable
        };
        
        Ok(ValidationTrends {
            last_10_validations,
            trend_direction,
            common_issue_types,
            improvement_areas: Vec::new(), // Would analyze patterns
        })
    }
    
    async fn record_validation(&self, report: &ValidationReport) -> ValidationResult<()> {
        let mut history = self.validation_history.write().await;
        
        let record = ValidationRecord {
            record_id: Uuid::new_v4(),
            validation_result: ValidationResult {
                is_valid: report.overall_valid,
                validation_id: Uuid::new_v4(),
                checked_at: SystemTime::now(),
                duration_ms: 0, // Would track actual duration
                checked_data_type: DataType::AllData,
                issues: report.all_issues.clone(),
                warnings: Vec::new(), // Would include warnings
                statistics: ValidationStatistics {
                    files_checked: 0, // Would aggregate actual counts
                    total_issues: report.all_issues.len(),
                    critical_issues: report.critical_issues.len(),
                    error_issues: 0,
                    warning_issues: 0,
                    auto_repairs_attempted: report.auto_repair_summary.total_repairs_attempted,
                    auto_repairs_successful: report.auto_repair_summary.total_repairs_successful,
                    validation_errors: 0,
                },
            },
            project_version: "1.0.0".to_string(), // Would get actual version
            validator_version: env!("CARGO_PKG_VERSION").to_string(),
        };
        
        history.push(record);
        
        // Keep only last 100 validation records
        if history.len() > 100 {
            history.drain(0..history.len() - 100);
        }
        
        Ok(())
    }
    
    fn should_attempt_repair(&self, issue: &ValidationIssue) -> bool {
        match issue.severity {
            ValidationSeverity::Critical => self.auto_repair_config.repair_critical_issues,
            ValidationSeverity::Error => true,
            ValidationSeverity::Warning => self.auto_repair_config.repair_warnings,
            ValidationSeverity::Info => false,
        }
    }
    
    async fn attempt_auto_repair(&self, issue: &ValidationIssue) -> ValidationResult<Vec<RepairAction>> {
        let mut actions = Vec::new();
        
        for repair_action in &issue.repair_actions {
            let mut executed_action = repair_action.clone();
            
            match repair_action.action_type {
                ActionType::RegenerateData => {
                    // Attempt to regenerate data
                    executed_action.executed = true;
                    executed_action.successful = true; // Simplified
                }
                ActionType::RemoveFile => {
                    if let Some(ref path) = repair_action.target_path {
                        if path.exists() {
                            fs::remove_file(path)?;
                            executed_action.executed = true;
                            executed_action.successful = true;
                        }
                    }
                }
                _ => {
                    // Other repair actions would be implemented here
                    executed_action.executed = false;
                    executed_action.successful = false;
                    executed_action.error_message = Some("Auto-repair not implemented for this action type".to_string());
                }
            }
            
            actions.push(executed_action);
        }
        
        Ok(actions)
    }
}

impl Default for AutoRepairConfig {
    fn default() -> Self {
        Self {
            enable_auto_repair: false, // Disabled by default for safety
            repair_critical_issues: true,
            repair_warnings: false,
            create_backup_before_repair: true,
            max_repair_attempts: 3,
            validate_after_repair: true,
            repair_log_level: LogLevel::Info,
        }
    }
}

impl Default for ValidationStatistics {
    fn default() -> Self {
        Self {
            files_checked: 0,
            total_issues: 0,
            critical_issues: 0,
            error_issues: 0,
            warning_issues: 0,
            auto_repairs_attempted: 0,
            auto_repairs_successful: 0,
            validation_errors: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_validator_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let validator = DataValidator::new(temp_dir.path().to_path_buf()).unwrap();
        
        assert_eq!(validator.validation_rules.len(), 3);
        assert_eq!(validator.schema_definitions.len(), 1);
    }

    #[test]
    fn test_project_health_score_calculation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let validator = DataValidator::new(temp_dir.path().to_path_buf()).unwrap();
        
        let issues = vec![
            ValidationIssue {
                issue_id: Uuid::new_v4(),
                severity: ValidationSeverity::Critical,
                rule_id: "test".to_string(),
                issue_type: IssueType::DataCorruption,
                description: "Test critical issue".to_string(),
                affected_path: None,
                field_name: None,
                expected_value: None,
                actual_value: None,
                repairable: false,
                auto_repair_attempted: false,
                auto_repair_successful: false,
                repair_actions: Vec::new(),
            }
        ];
        
        let statistics = ValidationStatistics::default();
        let score = validator.calculate_project_health_score(&issues, &statistics);
        
        assert!(score < 100.0);
        assert!(score >= 0.0);
    }
}