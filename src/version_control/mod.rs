//! Version Control Integration Module
//! 
//! Git integration for Herding Cats Rust enabling document versioning,
//! change tracking, commit history, branching, and collaborative version control.

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppResult, AppError};

/// Git repository configuration and management
#[derive(Debug, Clone)]
pub struct GitRepository {
    pub repository_id: String,
    pub path: PathBuf,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub last_commit: Option<DateTime<Utc>>,
    pub branch_count: usize,
    pub commit_count: usize,
    pub document_count: usize,
    pub remote_url: Option<String>,
    pub auto_commit_enabled: bool,
    pub backup_frequency_hours: u32,
}

/// Git commit information
#[derive(Debug, Clone)]
pub struct GitCommit {
    pub commit_id: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub timestamp: DateTime<Utc>,
    pub files_changed: Vec<String>,
    pub insertions: usize,
    pub deletions: usize,
    pub parent_commits: Vec<String>,
    pub branch_name: String,
    pub tags: Vec<String>,
    pub metadata: CommitMetadata,
}

/// Commit metadata and statistics
#[derive(Debug, Clone)]
pub struct CommitMetadata {
    pub document_type: String,
    pub word_count: usize,
    pub character_count: usize,
    pub paragraph_count: usize,
    pub ai_analysis_version: Option<String>,
    pub collaboration_session_id: Option<String>,
    pub backup_location: Option<String>,
    pub compression_ratio: Option<f32>,
}

/// Branch information
#[derive(Debug, Clone)]
pub struct GitBranch {
    pub name: String,
    pub commit_id: String,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub is_active: bool,
    pub is_protected: bool,
    pub description: String,
    pub remote_tracking: Option<String>,
    pub merge_conflicts: Vec<MergeConflict>,
}

/// Merge conflict information
#[derive(Debug, Clone)]
pub struct MergeConflict {
    pub file_path: String,
    pub conflict_type: ConflictType,
    pub our_changes: String,
    pub their_changes: String,
    pub base_version: Option<String>,
    pub resolution_status: ResolutionStatus,
}

/// Types of merge conflicts
#[derive(Debug, Clone)]
pub enum ConflictType {
    TextContent,      // Content conflicts
    Metadata,         // Document metadata conflicts
    Structure,        // File/folder structure conflicts
    BinaryFile,       // Binary file conflicts
}

/// Conflict resolution status
#[derive(Debug, Clone)]
pub enum ResolutionStatus {
    Unresolved,
    Manual,
    AutoResolved,
    Skipped,
}

/// Document change tracking
#[derive(Debug, Clone)]
pub struct DocumentChange {
    pub change_id: String,
    pub document_path: String,
    pub change_type: ChangeType,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub word_count_change: i32,
    pub character_count_change: i32,
    pub detected_at: DateTime<Utc>,
    pub detection_method: DetectionMethod,
    pub user_id: Option<String>,
    pub ai_analysis: Option<AiChangeAnalysis>,
}

/// Types of document changes
#[derive(Debug, Clone)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
    Renamed,
    ContentUpdate,
    MetadataUpdate,
}

/// How changes were detected
#[derive(Debug, Clone)]
pub enum DetectionMethod {
    FileWatcher,
    ManualSave,
    AutoBackup,
    CollaborationSync,
    ImportExport,
}

/// AI analysis of document changes
#[derive(Debug, Clone)]
pub struct AiChangeAnalysis {
    pub change_significance: f32,
    pub semantic_changes: Vec<String>,
    pub structure_changes: Vec<String>,
    pub quality_metrics: QualityMetrics,
    pub suggestions: Vec<ChangeSuggestion>,
}

/// Quality metrics for changed content
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    pub readability_score: f32,
    pub grammar_score: f32,
    pub style_score: f32,
    pub consistency_score: f32,
    pub engagement_score: f32,
}

/// Suggestions for handling changes
#[derive(Debug, Clone)]
pub struct ChangeSuggestion {
    pub suggestion_type: SuggestionType,
    pub description: String,
    pub priority: Priority,
    pub estimated_effort: EffortLevel,
}

/// Types of change suggestions
#[derive(Debug, Clone)]
pub enum SuggestionType {
    ReviewNeeded,
    BackupCreation,
    BranchCreation,
    MergeRequired,
    ConflictResolution,
    PerformanceOptimization,
}

/// Suggestion priorities
#[derive(Debug, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Effort levels for suggestions
#[derive(Debug, Clone)]
pub enum EffortLevel {
    Minimal,
    Low,
    Medium,
    High,
}

/// Version control configuration
#[derive(Debug, Clone)]
pub struct VersionControlConfig {
    pub enable_git_integration: bool,
    pub auto_commit_enabled: bool,
    pub auto_commit_interval_minutes: u32,
    pub commit_message_template: String,
    pub backup_frequency_hours: u32,
    pub max_branch_count: usize,
    pub enable_ai_analysis: bool,
    pub enable_compression: bool,
    pub exclude_patterns: Vec<String>,
    pub remote_sync_enabled: bool,
    pub conflict_resolution_strategy: ConflictResolutionStrategy,
}

/// Conflict resolution strategies
#[derive(Debug, Clone)]
pub enum ConflictResolutionStrategy {
    Manual,
    LastWriterWins,
    FirstWriterWins,
    MergeByContent,
    AIAssisted,
}

/// Remote repository information
#[derive(Debug, Clone)]
pub struct RemoteRepository {
    pub name: String,
    pub url: String,
    pub remote_type: RemoteType,
    pub authentication: RemoteAuth,
    pub last_sync: Option<DateTime<Utc>>,
    pub sync_status: SyncStatus,
}

/// Types of remote repositories
#[derive(Debug, Clone)]
pub enum RemoteType {
    GitHub,
    GitLab,
    Bitbucket,
    Custom,
}

/// Remote authentication
#[derive(Debug, Clone)]
pub struct RemoteAuth {
    pub auth_type: AuthType,
    pub username: Option<String>,
    pub token: Option<String>,
    pub key_path: Option<String>,
}

/// Authentication types
#[derive(Debug, Clone)]
pub enum AuthType {
    Password,
    Token,
    SSH,
    OAuth,
}

/// Synchronization status
#[derive(Debug, Clone)]
pub enum SyncStatus {
    UpToDate,
    Behind,
    Ahead,
    Diverged,
    Error(String),
}

/// Version control statistics
#[derive(Debug, Clone)]
pub struct VersionControlStats {
    pub total_commits: u64,
    pub commits_this_month: u64,
    pub active_branches: usize,
    pub merge_conflicts_resolved: u64,
    pub backup_count: u64,
    pub storage_used_mb: u64,
    pub average_commit_size_kb: f64,
    pub most_active_contributor: String,
}

/// Git integration engine
pub struct GitIntegration {
    repositories: Arc<tokio::sync::RwLock<HashMap<String, GitRepository>>>,
    change_tracker: Arc<tokio::sync::RwLock<HashMap<String, VecDeque<DocumentChange>>>>,
    config: VersionControlConfig,
    file_watcher: Arc<FileWatcher>,
    ai_analyzer: Arc<AiChangeAnalyzer>,
}

/// File system watcher for change detection
pub struct FileWatcher {
    watched_paths: Arc<tokio::sync::RwLock<HashMap<PathBuf, WatchConfig>>>,
    event_sender: Arc<tokio::sync::mpsc::UnboundedSender<FileEvent>>,
}

/// File system event
#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed(PathBuf, PathBuf),
}

/// File watcher configuration
#[derive(Debug, Clone)]
pub struct WatchConfig {
    pub recursive: bool,
    pub ignore_patterns: Vec<String>,
    pub auto_commit: bool,
}

/// AI change analyzer
pub struct AiChangeAnalyzer {
    change_queue: Arc<tokio::sync::mpsc::UnboundedReceiver<DocumentChange>>,
    analysis_cache: Arc<tokio::sync::RwLock<HashMap<String, AiChangeAnalysis>>>,
}

/// Implementation of Git integration engine
impl GitIntegration {
    /// Create new Git integration instance
    pub fn new(config: VersionControlConfig) -> Self {
        let file_watcher = Arc::new(FileWatcher::new());
        let ai_analyzer = Arc::new(AiChangeAnalyzer::new());

        Self {
            repositories: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            change_tracker: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            config,
            file_watcher,
            ai_analyzer,
        }
    }

    /// Initialize a new Git repository for a project
    pub async fn initialize_repository(
        &self,
        project_path: &Path,
        repository_name: String,
        description: String,
    ) -> AppResult<String> {
        let repository_id = Uuid::new_v4().to_string();
        
        // Initialize Git repository
        self.run_git_command(project_path, &["init"]).await?;
        
        // Create .gitignore
        self.create_gitignore(project_path).await?;
        
        // Configure Git user (placeholder - would use actual user info)
        self.run_git_command(project_path, &["config", "user.name", "HerdingCats"]).await?;
        self.run_git_command(project_path, &["config", "user.email", "herdingcats@example.com"]).await?;
        
        // Create initial commit
        self.run_git_command(project_path, &["add", "."]).await?;
        self.run_git_command(project_path, &["commit", "-m", "Initial commit - Herding Cats repository"]).await?;
        
        // Store repository information
        let repository = GitRepository {
            repository_id: repository_id.clone(),
            path: project_path.to_path_buf(),
            name: repository_name,
            description,
            created_at: Utc::now(),
            last_commit: Some(Utc::now()),
            branch_count: 1,
            commit_count: 1,
            document_count: 0,
            remote_url: None,
            auto_commit_enabled: self.config.auto_commit_enabled,
            backup_frequency_hours: self.config.backup_frequency_hours,
        };
        
        let mut repositories = self.repositories.write().await;
        repositories.insert(repository_id.clone(), repository);
        
        // Start watching for changes
        self.start_watching(project_path, &repository_id).await?;
        
        Ok(repository_id)
    }

    /// Create a new branch
    pub async fn create_branch(
        &self,
        repository_id: String,
        branch_name: String,
        from_branch: Option<String>,
    ) -> AppResult<()> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            let mut git_command = vec!["checkout", "-b", &branch_name];
            
            if let Some(from) = from_branch {
                git_command = vec!["checkout", "-b", &branch_name, &from];
            }
            
            self.run_git_command(&repository.path, &git_command).await?;
            
            // Update repository information
            drop(repositories);
            let mut repositories = self.repositories.write().await;
            if let Some(repo) = repositories.get_mut(&repository_id) {
                repo.branch_count += 1;
            }
        } else {
            return Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ));
        }
        
        Ok(())
    }

    /// Commit changes to the repository
    pub async fn commit_changes(
        &self,
        repository_id: String,
        message: String,
        author: Option<String>,
        files: Option<Vec<String>>,
    ) -> AppResult<String> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            // Stage files
            let mut add_command = vec!["add"];
            if let Some(files_to_add) = files {
                add_command.extend(files_to_add);
            } else {
                add_command.push(".");
            }
            
            self.run_git_command(&repository.path, &add_command).await?;
            
            // Create commit
            let mut commit_args = vec!["commit", "-m", &message];
            if let Some(author_name) = author {
                commit_args.extend(&["--author", &author_name]);
            }
            
            let commit_output = self.run_git_command(&repository.path, &commit_args).await?;
            let commit_id = self.extract_commit_id(&commit_output);
            
            // Update repository information
            drop(repositories);
            let mut repositories = self.repositories.write().await;
            if let Some(repo) = repositories.get_mut(&repository_id) {
                repo.last_commit = Some(Utc::now());
                repo.commit_count += 1;
            }
            
            // Analyze changes if AI is enabled
            if self.config.enable_ai_analysis {
                self.analyze_recent_changes(&repository_id).await?;
            }
            
            Ok(commit_id)
        } else {
            Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ))
        }
    }

    /// Get commit history for a repository
    pub async fn get_commit_history(
        &self,
        repository_id: String,
        branch: Option<String>,
        limit: Option<usize>,
    ) -> AppResult<Vec<GitCommit>> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            let mut git_args = vec!["log", "--oneline", "--pretty=format:%H|%s|%an|%ae|%ad"];
            
            if let Some(br) = branch {
                git_args.push(&br);
            }
            
            if let Some(lim) = limit {
                git_args.extend(&["-n", &lim.to_string()]);
            }
            
            let output = self.run_git_command(&repository.path, &git_args).await?;
            self.parse_commit_history(&output)
        } else {
            Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ))
        }
    }

    /// Get branch information
    pub async fn get_branches(&self, repository_id: String) -> AppResult<Vec<GitBranch>> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            let output = self.run_git_command(&repository.path, &["branch", "-a"]).await?;
            self.parse_branch_list(&output, &repository.path)
        } else {
            Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ))
        }
    }

    /// Merge branches
    pub async fn merge_branches(
        &self,
        repository_id: String,
        source_branch: String,
        target_branch: String,
        strategy: Option<ConflictResolutionStrategy>,
    ) -> AppResult<MergeResult> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            // Switch to target branch
            self.run_git_command(&repository.path, &["checkout", &target_branch]).await?;
            
            // Attempt merge
            let merge_output = self.run_git_command(
                &repository.path, 
                &["merge", "--no-ff", &source_branch]
            ).await;
            
            match merge_output {
                Ok(_) => {
                    // Successful merge
                    Ok(MergeResult::Success {
                        merge_commit_id: "generated".to_string(),
                        conflicts: Vec::new(),
                    })
                },
                Err(_) => {
                    // Merge conflict occurred
                    let conflicts = self.detect_merge_conflicts(&repository.path).await?;
                    
                    if let Some(strategy) = strategy {
                        let resolution = self.resolve_merge_conflicts(
                            &repository.path, 
                            &conflicts, 
                            strategy
                        ).await?;
                        
                        Ok(MergeResult::Conflicts {
                            conflicts,
                            resolution,
                        })
                    } else {
                        Ok(MergeResult::Conflicts {
                            conflicts,
                            resolution: None,
                        })
                    }
                }
            }
        } else {
            Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ))
        }
    }

    /// Create an automatic backup
    pub async fn create_backup(
        &self,
        repository_id: String,
        backup_name: Option<String>,
    ) -> AppResult<String> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            let backup_id = Uuid::new_v4().to_string();
            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
            let backup_name = backup_name.unwrap_or_else(|| 
                format!("backup_{}", timestamp)
            );
            
            // Create backup directory
            let backup_path = repository.path.join("backups").join(&backup_name);
            fs::create_dir_all(&backup_path)?;
            
            // Copy repository contents
            self.copy_directory(&repository.path, &backup_path).await?;
            
            // Create backup metadata
            let metadata = BackupMetadata {
                backup_id: backup_id.clone(),
                repository_id,
                backup_name,
                created_at: Utc::now(),
                size_mb: self.calculate_directory_size(&backup_path).await?,
                compression_ratio: if self.config.enable_compression {
                    Some(self.compress_backup(&backup_path).await?)
                } else {
                    None
                },
                commit_id: self.get_current_commit_id(&repository.path).await?,
                ai_analysis: if self.config.enable_ai_analysis {
                    Some(self.analyze_backup(&repository.path).await?)
                } else {
                    None
                },
            };
            
            // Save metadata
            let metadata_path = backup_path.join("backup_metadata.json");
            let metadata_json = serde_json::to_string_pretty(&metadata)?;
            fs::write(&metadata_path, metadata_json)?;
            
            Ok(backup_id)
        } else {
            Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ))
        }
    }

    /// Get version control statistics
    pub async fn get_statistics(&self, repository_id: String) -> AppResult<VersionControlStats> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            // Get total commits
            let commit_count_output = self.run_git_command(
                &repository.path, 
                &["rev-list", "--count", "HEAD"]
            ).await?;
            let total_commits: u64 = commit_count_output.trim().parse().unwrap_or(0);
            
            // Get branch count
            let branch_output = self.run_git_command(
                &repository.path, 
                &["branch", "--list"]
            ).await?;
            let active_branches = branch_output.lines().count();
            
            // Calculate storage usage
            let storage_used = self.calculate_repository_size(&repository.path).await?;
            
            // Get average commit size
            let avg_commit_size = self.calculate_average_commit_size(&repository.path).await?;
            
            Ok(VersionControlStats {
                total_commits,
                commits_this_month: 0, // Would calculate based on date filtering
                active_branches,
                merge_conflicts_resolved: 0, // Would track from history
                backup_count: self.count_backups(&repository.path).await?,
                storage_used_mb: storage_used,
                average_commit_size_kb: avg_commit_size,
                most_active_contributor: "System".to_string(), // Would analyze commit history
            })
        } else {
            Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ))
        }
    }

    /// Set up remote repository synchronization
    pub async fn add_remote(
        &self,
        repository_id: String,
        remote_name: String,
        remote_url: String,
        remote_type: RemoteType,
    ) -> AppResult<()> {
        let repositories = self.repositories.read().await;
        
        if let Some(repository) = repositories.get(&repository_id) {
            // Add remote
            self.run_git_command(
                &repository.path, 
                &["remote", "add", &remote_name, &remote_url]
            ).await?;
            
            // Update repository information
            drop(repositories);
            let mut repositories = self.repositories.write().await;
            if let Some(repo) = repositories.get_mut(&repository_id) {
                repo.remote_url = Some(remote_url);
            }
            
            Ok(())
        } else {
            Err(AppError::VersionControlError(
                "Repository not found".to_string()
            ))
        }
    }

    // Private helper methods
    async fn run_git_command(&self, path: &Path, args: &[&str]) -> AppResult<String> {
        use tokio::process::Command;
        
        let output = Command::new("git")
            .args(args)
            .current_dir(path)
            .output()
            .await?;
            
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(AppError::VersionControlError(
                format!("Git command failed: {}", error)
            ))
        }
    }

    async fn create_gitignore(&self, path: &Path) -> AppResult<()> {
        let gitignore_content = vec![
            "# Herding Cats specific",
            "*.backup",
            "*.bak", 
            "backups/",
            ".cache/",
            "temp/",
            "",
            "# IDE files",
            ".vscode/settings.json",
            ".idea/",
            "*.swp",
            "*.swo",
            "",
            "# OS files",
            ".DS_Store",
            "Thumbs.db",
            "",
            "# Logs",
            "*.log",
        ].join("\n");
        
        fs::write(path.join(".gitignore"), gitignore_content)?;
        Ok(())
    }

    fn extract_commit_id(&self, output: &str) -> String {
        // Extract commit hash from git output
        let lines: Vec<&str> = output.lines().collect();
        if let Some(line) = lines.first() {
            line.split(' ').next().unwrap_or("unknown").to_string()
        } else {
            "unknown".to_string()
        }
    }

    fn parse_commit_history(&self, output: &str) -> AppResult<Vec<GitCommit>> {
        let mut commits = Vec::new();
        
        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                commits.push(GitCommit {
                    commit_id: parts[0].to_string(),
                    message: parts[1].to_string(),
                    author: parts[2].to_string(),
                    email: parts[3].to_string(),
                    timestamp: Utc::now(), // Would parse actual timestamp
                    files_changed: Vec::new(),
                    insertions: 0,
                    deletions: 0,
                    parent_commits: Vec::new(),
                    branch_name: "main".to_string(),
                    tags: Vec::new(),
                    metadata: CommitMetadata::default(),
                });
            }
        }
        
        Ok(commits)
    }

    fn parse_branch_list(&self, output: &str, _path: &Path) -> AppResult<Vec<GitBranch>> {
        let mut branches = Vec::new();
        
        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            let is_active = line.trim_start().starts_with('*');
            let branch_name = line.trim().trim_start_matches('*').trim();
            
            branches.push(GitBranch {
                name: branch_name.to_string(),
                commit_id: "current".to_string(),
                created_at: Utc::now(),
                last_modified: Utc::now(),
                is_active,
                is_protected: false,
                description: format!("Branch: {}", branch_name),
                remote_tracking: None,
                merge_conflicts: Vec::new(),
            });
        }
        
        Ok(branches)
    }

    async fn start_watching(&self, path: &Path, repository_id: &str) -> AppResult<()> {
        // Start file system watching for the repository
        self.file_watcher.add_watch(
            path.to_path_buf(),
            WatchConfig {
                recursive: true,
                ignore_patterns: vec![
                    ".git".to_string(),
                    "target".to_string(),
                    "backups".to_string(),
                ],
                auto_commit: self.config.auto_commit_enabled,
            }
        ).await?;
        
        Ok(())
    }

    async fn detect_merge_conflicts(&self, path: &Path) -> AppResult<Vec<MergeConflict>> {
        let conflict_files_output = self.run_git_command(path, &["diff", "--name-only", "--diff-filter=U"]).await?;
        
        let mut conflicts = Vec::new();
        for file_path in conflict_files_output.lines() {
            if !file_path.trim().is_empty() {
                conflicts.push(MergeConflict {
                    file_path: file_path.to_string(),
                    conflict_type: ConflictType::TextContent,
                    our_changes: "current".to_string(),
                    their_changes: "incoming".to_string(),
                    base_version: None,
                    resolution_status: ResolutionStatus::Unresolved,
                });
            }
        }
        
        Ok(conflicts)
    }

    async fn resolve_merge_conflicts(
        &self,
        path: &Path,
        conflicts: &[MergeConflict],
        strategy: ConflictResolutionStrategy,
    ) -> AppResult<Option<ConflictResolution>> {
        match strategy {
            ConflictResolutionStrategy::LastWriterWins => {
                // Use current branch version
                for conflict in conflicts {
                    self.run_git_command(path, &["checkout", "--ours", &conflict.file_path]).await?;
                }
                self.run_git_command(path, &["add", "."]).await?;
                Ok(Some(ConflictResolution::AutoResolved))
            },
            ConflictResolutionStrategy::AIAssisted => {
                // AI-assisted resolution would be implemented here
                // For now, fall back to manual
                Ok(None)
            },
            _ => Ok(None),
        }
    }

    async fn analyze_recent_changes(&self, _repository_id: &str) -> AppResult<()> {
        // AI analysis of recent changes would be implemented here
        Ok(())
    }

    async fn analyze_backup(&self, path: &Path) -> AppResult<AiBackupAnalysis> {
        // Analyze backup for quality and changes
        Ok(AiBackupAnalysis {
            quality_score: 85.0,
            change_summary: "Stable backup".to_string(),
            recommendations: Vec::new(),
        })
    }

    async fn calculate_directory_size(&self, path: &Path) -> AppResult<u64> {
        let mut total_size = 0u64;
        
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    total_size += self.calculate_directory_size(&path).await?;
                } else if path.is_file() {
                    total_size += entry.metadata()?.len();
                }
            }
        }
        
        Ok(total_size)
    }

    async fn calculate_repository_size(&self, path: &Path) -> AppResult<u64> {
        self.calculate_directory_size(path).await
    }

    async fn calculate_average_commit_size(&self, _path: &Path) -> AppResult<f64> {
        // Would analyze git history for commit sizes
        Ok(15.5) // Placeholder
    }

    async fn count_backups(&self, path: &Path) -> AppResult<u64> {
        let backup_path = path.join("backups");
        if backup_path.exists() && backup_path.is_dir() {
            let count = fs::read_dir(&backup_path)?.count();
            Ok(count as u64)
        } else {
            Ok(0)
        }
    }

    async fn get_current_commit_id(&self, path: &Path) -> AppResult<String> {
        let output = self.run_git_command(path, &["rev-parse", "HEAD"]).await?;
        Ok(output.trim().to_string())
    }

    async fn copy_directory(&self, src: &Path, dst: &Path) -> AppResult<()> {
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if src_path.is_dir() {
                // Skip .git directory
                if src_path.file_name().unwrap() != ".git" {
                    self.copy_directory(&src_path, &dst_path).await?;
                }
            } else if src_path.is_file() {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }

    async fn compress_backup(&self, _path: &Path) -> AppResult<f32> {
        // Would implement compression logic
        Ok(0.75) // Placeholder compression ratio
    }
}

/// Merge result types
#[derive(Debug, Clone)]
pub enum MergeResult {
    Success {
        merge_commit_id: String,
        conflicts: Vec<MergeConflict>,
    },
    Conflicts {
        conflicts: Vec<MergeConflict>,
        resolution: Option<ConflictResolution>,
    },
}

#[derive(Debug, Clone)]
pub enum ConflictResolution {
    AutoResolved,
    Manual,
}

/// Backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub backup_id: String,
    pub repository_id: String,
    pub backup_name: String,
    pub created_at: DateTime<Utc>,
    pub size_mb: u64,
    pub compression_ratio: Option<f32>,
    pub commit_id: String,
    pub ai_analysis: Option<AiBackupAnalysis>,
}

/// AI backup analysis
#[derive(Debug, Clone)]
pub struct AiBackupAnalysis {
    pub quality_score: f32,
    pub change_summary: String,
    pub recommendations: Vec<String>,
}

// Default implementations
impl Default for CommitMetadata {
    fn default() -> Self {
        Self {
            document_type: "general".to_string(),
            word_count: 0,
            character_count: 0,
            paragraph_count: 0,
            ai_analysis_version: None,
            collaboration_session_id: None,
            backup_location: None,
            compression_ratio: None,
        }
    }
}

impl Default for VersionControlConfig {
    fn default() -> Self {
        Self {
            enable_git_integration: true,
            auto_commit_enabled: true,
            auto_commit_interval_minutes: 30,
            commit_message_template: "Auto-commit: {timestamp}".to_string(),
            backup_frequency_hours: 24,
            max_branch_count: 10,
            enable_ai_analysis: true,
            enable_compression: false,
            exclude_patterns: vec![
                "*.backup".to_string(),
                "*.bak".to_string(),
                ".cache".to_string(),
            ],
            remote_sync_enabled: false,
            conflict_resolution_strategy: ConflictResolutionStrategy::Manual,
        }
    }
}

// File watcher implementation
impl FileWatcher {
    pub fn new() -> Self {
        Self {
            watched_paths: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            event_sender: Arc::new(tokio::sync::mpsc::unbounded_channel().0),
        }
    }

    pub async fn add_watch(&self, path: PathBuf, config: WatchConfig) -> AppResult<()> {
        let mut watched_paths = self.watched_paths.write().await;
        watched_paths.insert(path, config);
        Ok(())
    }
}

// AI change analyzer implementation  
impl AiChangeAnalyzer {
    pub fn new() -> Self {
        let (_sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self {
            change_queue: Arc::new(receiver),
            analysis_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}