//! Ribbon Group Component
//! 
//! Implements ribbon groups that contain related buttons and controls.

use crate::ui::ribbon::{RibbonGroup, RibbonItem};

/// Create a clipboard group with common editing operations
pub fn create_clipboard_group() -> RibbonGroup {
    let mut group = RibbonGroup::new("clipboard".to_string(), "Clipboard".to_string());
    
    // Add clipboard operations
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("paste".to_string(), "Paste".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("cut".to_string(), "Cut".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("copy".to_string(), "Copy".to_string())
    ));
    
    group
}

/// Create a font formatting group
pub fn create_font_group() -> RibbonGroup {
    let mut group = RibbonGroup::new("font".to_string(), "Font".to_string());
    
    // Add font formatting options
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("bold".to_string(), "Bold".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("italic".to_string(), "Italic".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("underline".to_string(), "Underline".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("font_color".to_string(), "Font Color".to_string())
    ));
    
    group
}

/// Create a paragraph formatting group
pub fn create_paragraph_group() -> RibbonGroup {
    let mut group = RibbonGroup::new("paragraph".to_string(), "Paragraph".to_string());
    
    // Add paragraph formatting options
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("bullets".to_string(), "Bullets".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("numbering".to_string(), "Numbering".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("alignment".to_string(), "Alignment".to_string())
    ));
    
    group
}

/// Create an insert group for adding content
pub fn create_insert_group() -> RibbonGroup {
    let mut group = RibbonGroup::new("insert".to_string(), "Insert".to_string());
    
    // Add insertion options
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("table".to_string(), "Table".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("image".to_string(), "Image".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("chart".to_string(), "Chart".to_string())
    ));
    
    group
}

/// Create a layout group for page formatting
pub fn create_layout_group() -> RibbonGroup {
    let mut group = RibbonGroup::new("layout".to_string(), "Layout".to_string());
    
    // Add layout options
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("margin".to_string(), "Margins".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("orientation".to_string(), "Orientation".to_string())
    ));
    group.add_item(RibbonItem::Button(
        crate::ui::ribbon::RibbonButton::new("size".to_string(), "Size".to_string())
    ));
    
    group
}