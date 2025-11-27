//! Hierarchy Database Module
//! 
//! Database operations for hierarchy tool including CRUD operations,
//! schema management, and transaction handling for hierarchy items.
//! 
//! Note: This module has been updated to use SQLx instead of rusqlite.
//! All rusqlite references have been removed.

use crate::DatabaseError;
use crate::DatabaseResult;
use sqlx::Row;

use super::hierarchy_base::{HierarchyItem, HierarchyLevel};

/// Hierarchy database service for managing hierarchy items in the database
/// 
/// Note: This service should be updated to use SQLx patterns instead of rusqlite.
/// The current implementation is incomplete due to the migration from rusqlite to SQLx.
pub struct HierarchyDatabaseService {
    pool: sqlx::SqlitePool,
}

impl HierarchyDatabaseService {
    /// Create a new hierarchy database service
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Initialize the hierarchy table schema
    pub async fn initialize_schema(&self) -> DatabaseResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS hierarchy_items (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT NOT NULL,
                level TEXT NOT NULL,
                parent_id TEXT,
                position INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME,
                updated_at DATETIME,
                metadata TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
                FOREIGN KEY (parent_id) REFERENCES hierarchy_items(id) ON DELETE SET NULL
            )"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to create hierarchy_items table: {}", e)))?;

        Ok(())
    }
    
    /// Create a new hierarchy item
    pub async fn create_item(&self, item: &HierarchyItem) -> DatabaseResult<String> {
        let level_str = format!("{:?}", item.level); // Serialize enum to string
        let metadata_json = serde_json::to_string(&item.metadata).unwrap_or_default();
        
        sqlx::query(
            "INSERT INTO hierarchy_items (id, project_id, title, level, parent_id, position, created_at, updated_at, metadata)
             VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'), ?)"
        )
        .bind(&item.id)
        .bind(&item.project_id)
        .bind(&item.title)
        .bind(&level_str)
        .bind(&item.parent_id)
        .bind(item.position as i64)
        .bind(metadata_json)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to create hierarchy item: {}", e)))?;
        
        Ok(item.id.clone())
    }
    
    /// Get all hierarchy items for a project
    pub async fn get_items_by_project(&self, project_id: &str) -> DatabaseResult<Vec<HierarchyItem>> {
        let rows = sqlx::query(
            "SELECT id, project_id, title, level, parent_id, position, created_at, updated_at, metadata 
             FROM hierarchy_items WHERE project_id = ? ORDER BY position ASC"
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DatabaseError::Service(format!("Failed to get hierarchy items: {}", e)))?;
        
        let mut items = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let title: String = row.get("title");
            let level_str: String = row.get("level");
            let parent_id: Option<String> = row.get("parent_id");
            let position: i64 = row.get("position");
            let project_id: String = row.get("project_id");
            // Dates and metadata handling simplified for now
            
            let level = match level_str.as_str() {
                "Manuscript" => HierarchyLevel::Manuscript,
                "Chapter" => HierarchyLevel::Chapter,
                "Scene" => HierarchyLevel::Scene,
                _ => HierarchyLevel::Unassigned,
            };
            
            let mut item = HierarchyItem::new(id, title, level, parent_id, project_id);
            item.position = position as u32;
            items.push(item);
        }
        
        // Reconstruct children relationships
        // This is O(N^2) but N is small for now. Better to use a map.
        // For now, we just return the flat list and let the tool rebuild the tree if needed.
        // But HierarchyItem has `children` field.
        // The `HierarchyTree` in `MigratedHierarchyTool` rebuilds the tree from items.
        // So returning flat items is fine, as long as `HierarchyTree::add_item` handles it.
        // Wait, `HierarchyTree::add_item` expects children to be populated?
        // No, `HierarchyTree::add_item` populates children in the tree structure based on `parent_id`.
        // But `HierarchyItem` struct has `children` field.
        // `HierarchyTree` uses `items: HashMap<String, HierarchyItem>`.
        // When adding, it updates parent's children list.
        // So the input items don't need `children` populated.
        
        Ok(items)
    }
    
    /// Delete a hierarchy item
    pub async fn delete_item(&self, item_id: &str) -> DatabaseResult<()> {
        sqlx::query("DELETE FROM hierarchy_items WHERE id = ?")
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to delete hierarchy item: {}", e)))?;
        Ok(())
    }
    
    /// Update item position
    pub async fn update_item_position(&self, item_id: &str, position: u32) -> DatabaseResult<()> {
        sqlx::query("UPDATE hierarchy_items SET position = ? WHERE id = ?")
            .bind(position as i64)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::Service(format!("Failed to update item position: {}", e)))?;
        Ok(())
    }
    
    // Other methods can remain unimplemented or be implemented as needed
    
    pub async fn get_item(&self, _item_id: &str) -> DatabaseResult<Option<HierarchyItem>> {
        Err(DatabaseError::NotImplemented("get_item".to_string()))
    }
    
    pub async fn update_item(&self, _item: &HierarchyItem) -> DatabaseResult<()> {
        Err(DatabaseError::NotImplemented("update_item".to_string()))
    }
    
    pub async fn move_item(&self, _item_id: &str, _new_parent_id: Option<String>, _position: u32) -> DatabaseResult<()> {
        Err(DatabaseError::NotImplemented("move_item".to_string()))
    }
    
    pub async fn get_root_items(&self, _project_id: &str) -> DatabaseResult<Vec<HierarchyItem>> {
        Err(DatabaseError::NotImplemented("get_root_items".to_string()))
    }
    
    pub async fn get_children(&self, _parent_id: &str) -> DatabaseResult<Vec<HierarchyItem>> {
        Err(DatabaseError::NotImplemented("get_children".to_string()))
    }
    
    pub async fn reorder_items(&self, _parent_id: Option<String>, _item_ids: &[String]) -> DatabaseResult<()> {
        Err(DatabaseError::NotImplemented("reorder_items".to_string()))
    }
    
    pub async fn count_items(&self, _project_id: &str) -> DatabaseResult<usize> {
        Err(DatabaseError::NotImplemented("count_items".to_string()))
    }
    
    pub async fn item_exists(&self, _item_id: &str) -> DatabaseResult<bool> {
        Err(DatabaseError::NotImplemented("item_exists".to_string()))
    }
}