//! Hierarchy Tool Demonstration
//!
//! This module provides a complete demonstration of the enhanced hierarchy tool
//! with Unassigned and Manuscript top-level entries, chapter headings, scenes,
//! and enhanced drag and drop functionality.

use super::hierarchy_base::{HierarchyTree, HierarchyItem, HierarchyLevel};
use super::hierarchy::HierarchyTool;
use super::hierarchy_drag::{HierarchyDragHandler, HierarchyDragData};
use slint::{ComponentHandle, VecModel, SharedString};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Demonstration of the complete hierarchy tool functionality
pub struct HierarchyDemo {
    /// Core hierarchy tool
    pub hierarchy_tool: HierarchyTool,
    /// Sample project ID for demo
    pub demo_project_id: String,
    /// Demo chapters for demonstration
    pub demo_chapters: Vec<String>,
    /// Demo scenes for demonstration  
    pub demo_scenes: Vec<String>,
}

impl HierarchyDemo {
    /// Create a new hierarchy demonstration
    pub fn new() -> Self {
        Self {
            hierarchy_tool: HierarchyTool::new(),
            demo_project_id: "demo_project_1".to_string(),
            demo_chapters: vec![
                "Chapter 1: Introduction".to_string(),
                "Chapter 2: Rising Action".to_string(),
                "Chapter 3: Climax".to_string(),
                "Chapter 4: Resolution".to_string(),
            ],
            demo_scenes: vec![
                "Scene 1: Character Establishment".to_string(),
                "Scene 2: World Building".to_string(),
                "Scene 3: Inciting Incident".to_string(),
                "Scene 4: First Conflict".to_string(),
                "Scene 5: Complications".to_string(),
                "Scene 6: Turning Point".to_string(),
                "Scene 7: Final Confrontation".to_string(),
                "Scene 8: Resolution".to_string(),
            ],
        }
    }
    
    /// Initialize the demonstration hierarchy structure
    pub async fn initialize_demo_hierarchy(&mut self) {
        // Create sample chapters under Manuscript
        for (index, chapter_title) in self.demo_chapters.clone().into_iter().enumerate() {
            if let Ok(chapter_id) = self.hierarchy_tool.create_chapter(chapter_title.clone()).await {
                println!("Created chapter: {} with ID: {}", chapter_title, chapter_id);
                
                // Create 1-3 scenes for each chapter
                let scenes_per_chapter = (index % 3) + 1;
                for scene_index in 0..scenes_per_chapter {
                    let scene_title = format!("{}", self.demo_scenes.get(scene_index).unwrap_or(&"Demo Scene".to_string()));
                    if let Ok(scene_id) = self.hierarchy_tool.create_scene(scene_title.clone(), chapter_id.clone()).await {
                        println!("  Created scene: {} with ID: {}", scene_title, scene_id);
                    }
                }
            }
        }
        
        // Create some unassigned scenes
        for i in 0..3 {
            let scene_title = format!("Unassigned Scene {}", i + 1);
            if let Ok(scene_id) = self.hierarchy_tool.create_unassigned_scene(scene_title.clone()).await {
                println!("Created unassigned scene: {} with ID: {}", scene_title, scene_id);
            }
        }
    }
    
    /// Display the current hierarchy structure
    pub fn display_hierarchy(&self) {
        println!("=== CURRENT HIERARCHY STRUCTURE ===");
        let tree = self.hierarchy_tool.get_hierarchy();
        
        for root_item in tree.get_root_items() {
            match root_item.level {
                HierarchyLevel::Unassigned => println!("📝 {}", root_item.title),
                HierarchyLevel::Manuscript => println!("📖 {}", root_item.title),
                _ => {}
            }
            
            self.display_children(root_item, 1);
        }
        println!("==================================");
    }
    
    /// Display children items recursively
    fn display_children(&self, parent: &HierarchyItem, depth: usize) {
        let tree = self.hierarchy_tool.get_hierarchy();
        let children = tree.get_children(&parent.id);
        
        for child in children {
            let indent = "  ".repeat(depth);
            match child.level {
                HierarchyLevel::Chapter => {
                    println!("{}📚 {}", indent, child.title);
                    self.display_children(child, depth + 1);
                },
                HierarchyLevel::Scene => {
                    println!("{}🎬 {} ({} words)", indent, child.title, child.word_count);
                },
                _ => {}
            }
        }
    }
    
    /// Demonstrate drag and drop functionality
    pub fn demonstrate_drag_and_drop(&mut self) {
        println!("\n=== DRAG AND DROP DEMONSTRATION ===");
        
        let tree = self.hierarchy_tool.get_hierarchy();
        
        // Find a scene to move
        let scene_to_move = tree.get_leaf_items()
            .into_iter()
            .find(|item| item.level == HierarchyLevel::Scene)
            .expect("No scene found to move");
            
        println!("Moving scene: {}", scene_to_move.title);
        
        // Find a target chapter
        let target_chapter = tree.get_items_by_level(HierarchyLevel::Chapter)
            .into_iter()
            .find(|chapter| chapter.id != scene_to_move.parent_id.as_deref().unwrap_or("none"))
            .expect("No target chapter found");
            
        println!("Target chapter: {}", target_chapter.title);
        
        // Create drag data
        let drag_data = HierarchyDragData::new_move(scene_to_move, "hierarchy");
        
        // Validate the drop
        let drag_handler = HierarchyDragHandler::new();
        let validation = drag_handler.validate_drop(&drag_data, Some(target_chapter));
        
        println!("Drop validation: {:?}", validation);
        
        if validation.can_drop {
            println!("✅ Drag and drop operation is valid");
            println!("Visual feedback: {:?}", validation.visual_feedback);
        } else {
            println!("❌ Drag and drop operation is invalid: {:?}", validation.reason);
        }
    }
    
    /// Demonstrate selective text drag and drop
    pub fn demonstrate_selective_drag_and_drop(&mut self) {
        println!("\n=== SELECTIVE DRAG AND DROP DEMONSTRATION ===");
        
        let tree = self.hierarchy_tool.get_hierarchy();
        
        // Find a scene with content
        let scene = tree.get_leaf_items()
            .into_iter()
            .find(|item| item.level == HierarchyLevel::Scene)
            .expect("No scene found for selective drag");
            
        // Simulate selecting text in the scene (e.g., characters 10-30)
        let text_range = (10, 30);
        
        println!("Selective drag of text range {:?} from scene: {}", text_range, scene.title);
        
        // Create selective drag data
        let drag_data = HierarchyDragData::new_selective_move(scene, "hierarchy", text_range);
        
        println!("Is selective drag: {}", drag_data.is_selective_drag());
        println!("Selected range: {:?}", drag_data.get_selected_range());
        
        // Find target for selective drop
        let target_scene = tree.get_leaf_items()
            .into_iter()
            .find(|item| item.level == HierarchyLevel::Scene && item.id != scene.id)
            .expect("No target scene found");
            
        let drag_handler = HierarchyDragHandler::new();
        let validation = drag_handler.validate_drop(&drag_data, Some(target_scene));
        
        println!("Selective drop validation: {:?}", validation);
        
        if validation.can_drop {
            println!("✅ Selective drag and drop operation is valid");
        } else {
            println!("❌ Selective drag and drop operation is invalid");
        }
    }
    
    /// Demonstrate item creation
    pub async fn demonstrate_item_creation(&mut self) {
        println!("\n=== ITEM CREATION DEMONSTRATION ===");
        
        // Create a new chapter
        if let Ok(chapter_id) = self.hierarchy_tool.create_chapter("Chapter 5: New Beginning".to_string()).await {
            println!("Created new chapter with ID: {}", chapter_id);
            
            // Create a new scene under this chapter
            if let Ok(scene_id) = self.hierarchy_tool.create_scene("New Scene: Awakening".to_string(), chapter_id).await {
                println!("Created new scene with ID: {}", scene_id);
            }
        }
        
        // Create an unassigned scene
        if let Ok(scene_id) = self.hierarchy_tool.create_unassigned_scene("Loose Idea Scene".to_string()).await {
            println!("Created unassigned scene with ID: {}", scene_id);
        }
    }
    
    /// Demonstrate hierarchy statistics
    pub fn demonstrate_hierarchy_stats(&self) {
        println!("\n=== HIERARCHY STATISTICS ===");
        let tree = self.hierarchy_tool.get_hierarchy();
        
        let unassigned_items = tree.get_items_by_level(HierarchyLevel::Unassigned);
        let manuscript_items = tree.get_items_by_level(HierarchyLevel::Manuscript);
        let chapter_items = tree.get_items_by_level(HierarchyLevel::Chapter);
        let scene_items = tree.get_items_by_level(HierarchyLevel::Scene);
        
        println!("Top-level Unassigned items: {}", unassigned_items.len());
        println!("Top-level Manuscript items: {}", manuscript_items.len());
        println!("Chapter items: {}", chapter_items.len());
        println!("Scene items: {}", scene_items.len());
        println!("Total items: {}", tree.len());
        println!("Branch items (with children): {}", tree.get_branch_items().len());
        println!("Leaf items (no children): {}", tree.get_leaf_items().len());
        
        // Calculate word count
        let total_words: u32 = scene_items.iter().map(|item| item.word_count).sum();
        println!("Total word count across all scenes: {}", total_words);
    }
    
    /// Run complete demonstration
    pub async fn run_complete_demo(&mut self) {
        println!("🎬 STARTING COMPLETE HIERARCHY TOOL DEMONSTRATION 🎬\n");
        
        self.initialize_demo_hierarchy().await;
        self.display_hierarchy();
        self.demonstrate_item_creation().await;
        self.display_hierarchy();
        self.demonstrate_drag_and_drop();
        self.demonstrate_selective_drag_and_drop();
        self.demonstrate_hierarchy_stats();
        
        println!("\n✅ HIERARCHY TOOL DEMONSTRATION COMPLETE ✅");
    }
}

/// Convert hierarchy tree to Slint-compatible data
pub fn tree_to_slint_data(tree: &HierarchyTree) -> Vec<super::hierarchy_ui::SlintHierarchyItem> {
    let mut items = Vec::new();
    
    // Build flat list for Slint ListView
    let root_items = tree.get_root_items();
    
    for root_item in root_items {
        add_item_recursive_slint(root_item, 0, &mut items);
    }
    
    items
}

/// Add item and children recursively for Slint data
fn add_item_recursive_slint(item: &HierarchyItem, depth: usize, items: &mut Vec<super::hierarchy_ui::SlintHierarchyItem>) {
    // Convert to Slint format
    let slint_item = super::hierarchy_ui::SlintHierarchyItem {
        id: SharedString::from(item.id.clone()),
        title: SharedString::from(item.title.clone()),
        level: item.level as u32,
        level_name: SharedString::from(item.level.display_name()),
        has_children: !item.children.is_empty(),
        is_expanded: false, // This would be managed by UI state
        depth: depth as u32,
        word_count: item.word_count,
        position: item.position,
    };
    
    items.push(slint_item);
    
    // Add children if they exist
    // Note: This would need access to the tree, so we can't implement it here
    // In the actual implementation, this would be handled by the SlintHierarchyModel
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hierarchy_demo() {
        let mut demo = HierarchyDemo::new();
        
        // Initialize demo hierarchy
        demo.initialize_demo_hierarchy().await;
        
        // Verify structure
        let tree = demo.hierarchy_tool.get_hierarchy();
        assert!(tree.len() > 0, "Hierarchy should contain items");
        
        // Verify we have both top-level entries
        let root_items = tree.get_root_items();
        let has_unassigned = root_items.iter().any(|item| item.level == HierarchyLevel::Unassigned);
        let has_manuscript = root_items.iter().any(|item| item.level == HierarchyLevel::Manuscript);
        
        assert!(has_unassigned, "Should have Unassigned top-level item");
        assert!(has_manuscript, "Should have Manuscript top-level item");
    }
    
    #[test]
    fn test_drag_validation() {
        let drag_handler = HierarchyDragHandler::new();
        
        // Create sample items
        let unassigned_item = HierarchyItem::new_root(
            "Test Unassigned".to_string(),
            HierarchyLevel::Unassigned,
            "test_project".to_string()
        );
        
        let scene_item = HierarchyItem::new_scene(
            "Test Scene".to_string(),
            "test_parent".to_string(),
            "test_project".to_string()
        );
        
        // Test dragging scene onto unassigned (should be invalid)
        let drag_data = HierarchyDragData::new_move(&scene_item, "test");
        let validation = drag_handler.validate_drop(&drag_data, Some(&unassigned_item));
        
        assert!(!validation.can_drop, "Cannot drag scene into unassigned");
    }
}