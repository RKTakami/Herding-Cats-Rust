//! Heuristic Project Import Module
//!
//! Handles importing existing projects from the file system by recursively walking directories,
//! reading files, and heuristically classifying their content to populate the project hierarchy.

use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use crate::classify::classify_content;
use crate::ui::tools::hierarchy_base::{HierarchyLevel, HierarchyItem};
use crate::ui::tools::hierarchy_tool_migrated::MigratedHierarchyTool;
use crate::ui::tools::database_integration::DatabaseOperationResult;
use tracing::{info, warn, error};
use crate::ui::tools::base::ToolIntegration;
use crate::database::{ServiceFactory, DatabaseConfig};

/// Statistics for the import process
#[derive(Debug, Default)]
pub struct ImportStats {
    pub files_processed: usize,
    pub items_created: usize,
    pub errors: usize,
}

/// Import a folder into a project
pub async fn import_folder_to_project(
    folder_path: &Path,
    _project_id: &str, // Will be used when creating items
    hierarchy_tool: &mut MigratedHierarchyTool,
) -> Result<ImportStats> {
    let mut stats = ImportStats::default();
    
    info!("Starting heuristic import from: {:?}", folder_path);

    // Create a root "Unassigned" item to hold unclassified or loose items
    let import_root_name = folder_path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Imported Project".to_string());

    let import_root_id = hierarchy_tool.create_hierarchy_item(
        import_root_name,
        HierarchyLevel::Unassigned,
        None,
    ).await.into_result().map_err(|e| anyhow::anyhow!("Failed to create import root: {}", e))?;

    // Recursive helper function
    // We need to use BoxFuture for recursion in async fn, but here we can just collect files first synchronously
    // or use a stack. Since we are in async context, let's just collect paths synchronously using std::fs
    // to avoid complex async recursion if possible, or just iterate.
    
    let mut files_to_process = Vec::new();
    let mut stack = vec![folder_path.to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    files_to_process.push(path);
                }
            }
        }
    }

    for path in files_to_process {
        // Basic check for text files
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !["txt", "md", "markdown", "rst"].contains(&ext_str.as_str()) {
                continue;
            }
        } else {
            continue;
        }

        stats.files_processed += 1;

        match fs::read_to_string(&path) {
            Ok(content) => {
                let filename = path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown File".to_string());

                let classification = classify_content(&content, &filename);
                
                // Map classification to HierarchyLevel
                let (level, parent_id) = match classification.as_str() {
                    "Chapter" => (HierarchyLevel::Chapter, Some(import_root_id.clone())),
                    "Scene" => (HierarchyLevel::Scene, Some(import_root_id.clone())),
                    "Manuscript" => (HierarchyLevel::Manuscript, None),
                    _ => (HierarchyLevel::Unassigned, Some(import_root_id.clone())),
                };
                
                let valid_level = if level == HierarchyLevel::Scene {
                    HierarchyLevel::Unassigned 
                } else if level == HierarchyLevel::Chapter {
                     HierarchyLevel::Chapter
                } else {
                    level
                };

                // Create the item
                let result = hierarchy_tool.create_hierarchy_item(
                    filename,
                    valid_level,
                    parent_id,
                ).await;

                if result.is_success() {
                    stats.items_created += 1;
                } else {
                    warn!("Failed to create item for {}: {:?}", path.display(), result);
                    stats.errors += 1;
                }
            },
            Err(e) => {
                warn!("Failed to read file {}: {}", path.display(), e);
                stats.errors += 1;
            }
        }
    }

    info!("Import completed: {:?}", stats);
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;
    use crate::ui::tools::database_integration::ToolDatabaseContext;
    use crate::database_app_state::DatabaseAppState;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_import_folder() {
        // Setup temp dir with files
        let temp_dir = TempDir::new().unwrap();
        let folder_path = temp_dir.path();

        // Create a chapter file
        let chapter_path = folder_path.join("Chapter 1.txt");
        let mut f = File::create(chapter_path).unwrap();
        writeln!(f, "This is a chapter.").unwrap();

        // Create a note file
        let note_path = folder_path.join("note.md");
        let mut f = File::create(note_path).unwrap();
        writeln!(f, "Just a note.").unwrap();

        // Setup database services
        let db_temp_dir = TempDir::new().unwrap();
        let db_path = db_temp_dir.path().join("test.db");
        let backup_path = db_temp_dir.path().join("backups");
        
        let factory = ServiceFactory::with_paths(&db_path, &backup_path, DatabaseConfig::default())
            .await
            .unwrap();
        let container = factory.initialize().await.unwrap();

        // Setup tool
        let mut db_state_inner = DatabaseAppState::new();
        db_state_inner.set_service_container(container);
        let db_state = Arc::new(RwLock::new(db_state_inner));
        let mut tool = MigratedHierarchyTool::new();
        let mut context = ToolDatabaseContext::new("test_import_project", db_state).await;
        tool.initialize(&mut context).await.unwrap();

        // Run import
        let stats = import_folder_to_project(folder_path, "test_import_project", &mut tool).await.unwrap();

        // Verify stats
        assert_eq!(stats.files_processed, 2);
        assert_eq!(stats.items_created, 2);
        assert_eq!(stats.errors, 0);

        // Verify hierarchy
        let tree = tool.get_hierarchy_tree();
        assert!(tree.len() > 0);
        
        // Should have an import root
        let roots = tree.get_root_items();
        assert_eq!(roots.len(), 1);
        let root_id = &roots[0].id;

        // Should have 2 children under root
        let children = tree.get_children(root_id);
        assert_eq!(children.len(), 2);
    }
}
