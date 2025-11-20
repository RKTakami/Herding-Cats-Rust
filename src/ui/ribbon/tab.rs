//! Ribbon Tab Component
//! 
//! Implements individual ribbon tabs.

use crate::ui::ribbon::{RibbonTab, RibbonGroup, RibbonItem, RibbonButton};

/// Create a Home tab with common document operations
pub fn create_home_tab() -> RibbonTab {
    let mut home_tab = RibbonTab::new("home".to_string(), "Home".to_string());

    // Clipboard group
    let mut clipboard_group = RibbonGroup::new("clipboard".to_string(), "Clipboard".to_string());
    
    clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("paste".to_string(), "Paste".to_string())));
    clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("cut".to_string(), "Cut".to_string())));
    clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("copy".to_string(), "Copy".to_string())));
    
    home_tab.add_group(clipboard_group);

    // Font group
    let mut font_group = RibbonGroup::new("font".to_string(), "Font".to_string());
    
    font_group.add_item(RibbonItem::Button(RibbonButton::new("bold".to_string(), "Bold".to_string())));
    font_group.add_item(RibbonItem::Button(RibbonButton::new("italic".to_string(), "Italic".to_string())));
    font_group.add_item(RibbonItem::Button(RibbonButton::new("underline".to_string(), "Underline".to_string())));
    
    home_tab.add_group(font_group);

    home_tab
}

/// Create an Insert tab
pub fn create_insert_tab() -> RibbonTab {
    let mut insert_tab = RibbonTab::new("insert".to_string(), "Insert".to_string());

    let mut insert_group = RibbonGroup::new("insert".to_string(), "Insert".to_string());
    
    insert_group.add_item(RibbonItem::Button(RibbonButton::new("table".to_string(), "Table".to_string())));
    insert_group.add_item(RibbonItem::Button(RibbonButton::new("image".to_string(), "Image".to_string())));
    insert_group.add_item(RibbonItem::Button(RibbonButton::new("shape".to_string(), "Shape".to_string())));
    
    insert_tab.add_group(insert_group);

    insert_tab
}

/// Create a Layout tab
pub fn create_layout_tab() -> RibbonTab {
    let mut layout_tab = RibbonTab::new("layout".to_string(), "Layout".to_string());

    let mut layout_group = RibbonGroup::new("layout".to_string(), "Layout".to_string());
    
    layout_group.add_item(RibbonItem::Button(RibbonButton::new("margin".to_string(), "Margins".to_string())));
    layout_group.add_item(RibbonItem::Button(RibbonButton::new("orientation".to_string(), "Orientation".to_string())));
    layout_group.add_item(RibbonItem::Button(RibbonButton::new("break".to_string(), "Breaks".to_string())));
    
    layout_tab.add_group(layout_group);

    layout_tab
}

/// Create a References tab
pub fn create_references_tab() -> RibbonTab {
    let mut references_tab = RibbonTab::new("references".to_string(), "References".to_string());

    let mut references_group = RibbonGroup::new("references".to_string(), "References".to_string());
    
    references_group.add_item(RibbonItem::Button(RibbonButton::new("toc".to_string(), "Table of Contents".to_string())));
    references_group.add_item(RibbonItem::Button(RibbonButton::new("citation".to_string(), "Citation".to_string())));
    references_group.add_item(RibbonItem::Button(RibbonButton::new("bibliography".to_string(), "Bibliography".to_string())));
    
    references_tab.add_group(references_group);

    references_tab
}