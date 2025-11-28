//! Test script for the enhanced hierarchy tool
//!
//! This script demonstrates the complete functionality of the hierarchy tool
//! including the two top-level entries, drag and drop, and selective editing.

use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_complete_hierarchy_functionality() {
        println!("🧪 Testing Complete Hierarchy Tool Functionality");
        
        // Import hierarchy modules
        use herding_cats_rust::ui::tools::hierarchy_demo::HierarchyDemo;
        use herding_cats_rust::ui::tools::hierarchy_base::{HierarchyLevel, HierarchyItem};
        use herding_cats_rust::ui::tools::hierarchy_drag::{HierarchyDragHandler, HierarchyDragData};
        
        println!("✅ Successfully imported all hierarchy modules");
        
        // Create demo instance
        let mut demo = HierarchyDemo::new();
        println!("✅ Created hierarchy demo instance");
        
        // Initialize demo hierarchy
        demo.initialize_demo_hierarchy().await;
        println!("✅ Initialized demo hierarchy with sample data");
        
        // Display current hierarchy
        demo.display_hierarchy();
        println!("✅ Displayed hierarchy structure");
        
        // Test drag and drop functionality
        demo.demonstrate_drag_and_drop();
        println!("✅ Demonstrated drag and drop functionality");
        
        // Test selective drag and drop
        demo.demonstrate_selective_drag_and_drop();
        println!("✅ Demonstrated selective drag and drop");
        
        // Test item creation
        demo.demonstrate_item_creation().await;
        println!("✅ Demonstrated item creation");
        
        // Show hierarchy statistics
        demo.demonstrate_hierarchy_stats();
        println!("✅ Demonstrated hierarchy statistics");
        
        // Verify structure
        let tree = demo.hierarchy_tool.get_hierarchy();
        assert!(tree.len() > 0, "Hierarchy should contain items");
        
        // Verify we have both top-level entries
        let root_items = tree.get_root_items();
        let has_unassigned = root_items.iter().any(|item| item.level == HierarchyLevel::Unassigned);
        let has_manuscript = root_items.iter().any(|item| item.level == HierarchyLevel::Manuscript);
        
        assert!(has_unassigned, "Should have Unassigned top-level item");
        assert!(has_manuscript, "Should have Manuscript top-level item");
        
        println!("✅ Verified hierarchy structure");
        
        println!("🎉 All hierarchy tool tests completed successfully!");
    }
    
    #[test]
    fn test_hierarchy_item_validation() {
        println!("🧪 Testing Hierarchy Item Validation");
        
        use herding_cats_rust::ui::tools::hierarchy_base::{HierarchyItem, HierarchyLevel};
        
        // Test valid item creation
        let unassigned_item = HierarchyItem::new_root(
            "Test Unassigned".to_string(),
            HierarchyLevel::Unassigned,
            "test_project".to_string()
        );
        
        assert!(unassigned_item.validate().is_ok(), "Unassigned item should be valid");
        assert_eq!(unassigned_item.level, HierarchyLevel::Unassigned);
        assert!(unassigned_item.parent_id.is_none(), "Root items should have no parent");
        
        println!("✅ Unassigned item creation and validation passed");
        
        // Test chapter creation
        let chapter_item = HierarchyItem::new_chapter(
            "Test Chapter".to_string(),
            "test_manuscript".to_string(),
            "test_project".to_string()
        );
        
        assert!(chapter_item.validate().is_ok(), "Chapter item should be valid");
        assert_eq!(chapter_item.level, HierarchyLevel::Chapter);
        assert!(chapter_item.parent_id.is_some(), "Chapter should have a parent");
        
        println!("✅ Chapter item creation and validation passed");
        
        // Test scene creation
        let scene_item = HierarchyItem::new_scene(
            "Test Scene".to_string(),
            "test_chapter".to_string(),
            "test_project".to_string()
        );
        
        assert!(scene_item.validate().is_ok(), "Scene item should be valid");
        assert_eq!(scene_item.level, HierarchyLevel::Scene);
        assert!(scene_item.parent_id.is_some(), "Scene should have a parent");
        
        println!("✅ Scene item creation and validation passed");
    }
    
    #[test]
    fn test_drag_validation() {
        println!("🧪 Testing Drag Validation Logic");
        
        use herding_cats_rust::ui::tools::hierarchy_base::{HierarchyItem, HierarchyLevel};
        use herding_cats_rust::ui::tools::hierarchy_drag::{HierarchyDragHandler, HierarchyDragData};
        
        let drag_handler = HierarchyDragHandler::new();
        
        // Create sample items
        let unassigned_item = HierarchyItem::new_root(
            "Test Unassigned".to_string(),
            HierarchyLevel::Unassigned,
            "test_project".to_string()
        );
        
        let manuscript_item = HierarchyItem::new_root(
            "Test Manuscript".to_string(),
            HierarchyLevel::Manuscript,
            "test_project".to_string()
        );
        
        let chapter_item = HierarchyItem::new_chapter(
            "Test Chapter".to_string(),
            manuscript_item.id.clone(),
            "test_project".to_string()
        );
        
        let scene_item = HierarchyItem::new_scene(
            "Test Scene".to_string(),
            chapter_item.id.clone(),
            "test_project".to_string()
        );
        
        // Test valid drop: chapter under manuscript
        let drag_data = HierarchyDragData::new_move(&chapter_item, "test");
        let validation = drag_handler.validate_drop(&drag_data, Some(&manuscript_item));
        assert!(validation.can_drop, "Should be able to drop chapter under manuscript");
        
        println!("✅ Valid drop: chapter under manuscript");
        
        // Test invalid drop: scene under unassigned
        let drag_data = HierarchyDragData::new_move(&scene_item, "test");
        let validation = drag_handler.validate_drop(&drag_data, Some(&unassigned_item));
        assert!(!validation.can_drop, "Should not be able to drop scene under unassigned");
        
        println!("✅ Invalid drop: scene under unassigned correctly rejected");
        
        // Test selective drag
        let text_range = (5, 15);
        let selective_drag_data = HierarchyDragData::new_selective_move(&scene_item, "test", text_range);
        assert!(selective_drag_data.is_selective_drag(), "Should be marked as selective drag");
        assert_eq!(selective_drag_data.get_selected_range(), Some(text_range), "Should preserve text range");
        
        println!("✅ Selective drag functionality works correctly");
    }
}

// Main demonstration function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 HIERARCHY TOOL - COMPLETE DEMONSTRATION 🎬");
    println!("===============================================\n");
    
    // Import the demo
    use herding_cats_rust::ui::tools::hierarchy_demo::HierarchyDemo;
    
    // Create and run complete demo
    let mut demo = HierarchyDemo::new();
    
    // Add demonstration of the enhanced features
    println!("📊 Demonstrating Enhanced Hierarchy Tool Features:");
    println!("   • Unassigned and Manuscript top-level entries");
    println!("   • Chapter headings under Manuscript");
    println!("   • Scenes under chapters");
    println!("   • Enhanced drag and drop functionality");
    println!("   • Selective text drag and drop");
    println!("   • Entire block rearrangement");
    println!("   • Visual feedback for drag operations\n");
    
    // Run the complete demonstration
    demo.run_complete_demo().await;
    
    println!("\n🎉 HIERARCHY TOOL DEMONSTRATION COMPLETE! 🎉");
    println!("=============================================");
    
    Ok(())
}