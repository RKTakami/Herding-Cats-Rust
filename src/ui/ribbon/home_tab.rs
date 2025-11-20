//! Home Tab Implementation
//! 
//! Implements the Home tab for the ribbon interface with common document operations.

use crate::ui::ribbon::{RibbonTab, RibbonGroup, RibbonItem, RibbonButton, RibbonDropdown, RibbonDropdownItem};

/// Create the Home tab with common document operations
pub fn create_home_tab() -> RibbonTab {
    let mut home_tab = RibbonTab::new("home".to_string(), "Home".to_string());

    // Clipboard group
    let mut clipboard_group = RibbonGroup::new("clipboard".to_string(), "Clipboard".to_string());
    
    // Paste dropdown with options
    let mut paste_dropdown = RibbonDropdown::new("paste_dropdown".to_string(), "Paste".to_string());
    paste_dropdown.add_item(RibbonDropdownItem::new("paste_normal".to_string(), "Keep Text Only".to_string()));
    paste_dropdown.add_item(RibbonDropdownItem::new("paste_formatting".to_string(), "Keep Source Formatting".to_string()));
    paste_dropdown.add_item(RibbonDropdownItem::new("paste_merge".to_string(), "Merge Formatting".to_string()));
    
    clipboard_group.add_item(RibbonItem::SplitButton(crate::ui::ribbon::RibbonSplitButton {
        main_button: RibbonButton::new("paste".to_string(), "Paste".to_string()),
        dropdown: paste_dropdown,
    }));
    
    clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("cut".to_string(), "Cut".to_string())));
    clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("copy".to_string(), "Copy".to_string())));
    clipboard_group.add_item(RibbonItem::Button(RibbonButton::new("format_painter".to_string(), "Format Painter".to_string())));
    
    home_tab.add_group(clipboard_group);

    // Font group
    let mut font_group = RibbonGroup::new("font".to_string(), "Font".to_string());
    
    let mut font_dropdown = RibbonDropdown::new("font_family".to_string(), "Calibri".to_string());
    font_dropdown.add_item(RibbonDropdownItem::new("font_calibri".to_string(), "Calibri".to_string()));
    font_dropdown.add_item(RibbonDropdownItem::new("font_times".to_string(), "Times New Roman".to_string()));
    font_dropdown.add_item(RibbonDropdownItem::new("font_aria".to_string(), "Arial".to_string()));
    font_dropdown.add_item(RibbonDropdownItem::new("font_consolas".to_string(), "Consolas".to_string()));
    
    font_group.add_item(RibbonItem::Dropdown(font_dropdown));
    
    let mut size_dropdown = RibbonDropdown::new("font_size".to_string(), "11".to_string());
    size_dropdown.add_item(RibbonDropdownItem::new("size_8".to_string(), "8".to_string()));
    size_dropdown.add_item(RibbonDropdownItem::new("size_9".to_string(), "9".to_string()));
    size_dropdown.add_item(RibbonDropdownItem::new("size_10".to_string(), "10".to_string()));
    size_dropdown.add_item(RibbonDropdownItem::new("size_11".to_string(), "11".to_string()));
    size_dropdown.add_item(RibbonDropdownItem::new("size_12".to_string(), "12".to_string()));
    size_dropdown.add_item(RibbonDropdownItem::new("size_14".to_string(), "14".to_string()));
    
    font_group.add_item(RibbonItem::Dropdown(size_dropdown));
    
    font_group.add_item(RibbonItem::Button(RibbonButton::new("bold".to_string(), "B".to_string())));
    font_group.add_item(RibbonItem::Button(RibbonButton::new("italic".to_string(), "I".to_string())));
    font_group.add_item(RibbonItem::Button(RibbonButton::new("underline".to_string(), "U".to_string())));
    
    let mut text_color_dropdown = RibbonDropdown::new("text_color".to_string(), "Text Color".to_string());
    text_color_dropdown.add_item(RibbonDropdownItem::new("color_black".to_string(), "Black".to_string()));
    text_color_dropdown.add_item(RibbonDropdownItem::new("color_blue".to_string(), "Blue".to_string()));
    text_color_dropdown.add_item(RibbonDropdownItem::new("color_red".to_string(), "Red".to_string()));
    
    font_group.add_item(RibbonItem::Dropdown(text_color_dropdown));
    
    home_tab.add_group(font_group);

    // Paragraph group
    let mut paragraph_group = RibbonGroup::new("paragraph".to_string(), "Paragraph".to_string());
    
    paragraph_group.add_item(RibbonItem::Button(RibbonButton::new("bullets".to_string(), "Bullets".to_string())));
    paragraph_group.add_item(RibbonItem::Button(RibbonButton::new("numbering".to_string(), "Numbering".to_string())));
    
    let mut alignment_dropdown = RibbonDropdown::new("alignment".to_string(), "Align Left".to_string());
    alignment_dropdown.add_item(RibbonDropdownItem::new("align_left".to_string(), "Align Left".to_string()));
    alignment_dropdown.add_item(RibbonDropdownItem::new("align_center".to_string(), "Align Center".to_string()));
    alignment_dropdown.add_item(RibbonDropdownItem::new("align_right".to_string(), "Align Right".to_string()));
    alignment_dropdown.add_item(RibbonDropdownItem::new("align_justify".to_string(), "Justify".to_string()));
    
    paragraph_group.add_item(RibbonItem::Dropdown(alignment_dropdown));
    
    paragraph_group.add_item(RibbonItem::Button(RibbonButton::new("line_spacing".to_string(), "Line Spacing".to_string())));
    
    home_tab.add_group(paragraph_group);

    // Styles group
    let mut styles_group = RibbonGroup::new("styles".to_string(), "Styles".to_string());
    
    let mut quick_styles_dropdown = RibbonDropdown::new("quick_styles".to_string(), "Normal".to_string());
    quick_styles_dropdown.add_item(RibbonDropdownItem::new("style_normal".to_string(), "Normal".to_string()));
    quick_styles_dropdown.add_item(RibbonDropdownItem::new("style_heading1".to_string(), "Heading 1".to_string()));
    quick_styles_dropdown.add_item(RibbonDropdownItem::new("style_heading2".to_string(), "Heading 2".to_string()));
    quick_styles_dropdown.add_item(RibbonDropdownItem::new("style_title".to_string(), "Title".to_string()));
    quick_styles_dropdown.add_item(RibbonDropdownItem::new("style_subtitle".to_string(), "Subtitle".to_string()));
    
    styles_group.add_item(RibbonItem::Dropdown(quick_styles_dropdown));
    
    home_tab.add_group(styles_group);

    // Editing group
    let mut editing_group = RibbonGroup::new("editing".to_string(), "Editing".to_string());
    
    editing_group.add_item(RibbonItem::Button(RibbonButton::new("find".to_string(), "Find".to_string())));
    editing_group.add_item(RibbonItem::Button(RibbonButton::new("replace".to_string(), "Replace".to_string())));
    editing_group.add_item(RibbonItem::Button(RibbonButton::new("select".to_string(), "Select".to_string())));
    
    home_tab.add_group(editing_group);

    home_tab
}

/// Home tab functionality and command handlers
pub struct HomeTabManager;

impl HomeTabManager {
    /// Register command handlers for the Home tab
    pub fn register_commands(ribbon_manager: &mut crate::ui::ribbon::RibbonManager) {
        // Clipboard commands
        ribbon_manager.register_command("paste".to_string(), || {
            println!("📋 Paste command executed");
        });
        
        ribbon_manager.register_command("cut".to_string(), || {
            println!("✂️ Cut command executed");
        });
        
        ribbon_manager.register_command("copy".to_string(), || {
            println!("📋 Copy command executed");
        });
        
        ribbon_manager.register_command("format_painter".to_string(), || {
            println!("🎨 Format Painter command executed");
        });

        // Font commands
        ribbon_manager.register_command("bold".to_string(), || {
            println!("🅱️ Bold formatting applied");
        });
        
        ribbon_manager.register_command("italic".to_string(), || {
            println!("🅘️ Italic formatting applied");
        });
        
        ribbon_manager.register_command("underline".to_string(), || {
            println!("🅤️ Underline formatting applied");
        });

        // Paragraph commands
        ribbon_manager.register_command("bullets".to_string(), || {
            println!("• Bullets applied");
        });
        
        ribbon_manager.register_command("numbering".to_string(), || {
            println!("1. Numbering applied");
        });
        
        ribbon_manager.register_command("line_spacing".to_string(), || {
            println!("📏 Line spacing options opened");
        });

        // Editing commands
        ribbon_manager.register_command("find".to_string(), || {
            println!("🔍 Find dialog opened");
        });
        
        ribbon_manager.register_command("replace".to_string(), || {
            println!("🔍 Replace dialog opened");
        });
        
        ribbon_manager.register_command("select".to_string(), || {
            println!("🎯 Select options opened");
        });
    }

    /// Update font family in the dropdown
    pub fn update_font_family(ribbon_manager: &mut crate::ui::ribbon::RibbonManager, font_family: &str) {
        if let Some(active_tab) = ribbon_manager.get_active_tab() {
            if active_tab.id == "home" {
                // Find the font family dropdown and update it
                // This would require more complex logic to actually update the UI
                println!("🔤 Font family changed to: {}", font_family);
            }
        }
    }

    /// Update font size in the dropdown
    pub fn update_font_size(ribbon_manager: &mut crate::ui::ribbon::RibbonManager, font_size: &str) {
        if let Some(active_tab) = ribbon_manager.get_active_tab() {
            if active_tab.id == "home" {
                println!("📏 Font size changed to: {}", font_size);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_tab_creation() {
        let home_tab = create_home_tab();
        
        assert_eq!(home_tab.id, "home");
        assert_eq!(home_tab.title, "Home");
        assert!(home_tab.groups.len() >= 5); // Should have Clipboard, Font, Paragraph, Styles, Editing
        
        // Check that Clipboard group exists
        let clipboard_group = home_tab.groups.iter()
            .find(|group| group.id == "clipboard");
        assert!(clipboard_group.is_some());
        
        // Check that Font group exists
        let font_group = home_tab.groups.iter()
            .find(|group| group.id == "font");
        assert!(font_group.is_some());
    }

    #[test]
    fn test_home_tab_commands() {
        let mut ribbon_manager = crate::ui::ribbon::RibbonManager::new();
        ribbon_manager.add_tab(create_home_tab());
        
        // Register commands
        HomeTabManager::register_commands(&mut ribbon_manager);
        
        // Test that commands can be executed
        assert!(ribbon_manager.execute_command("bold"));
        assert!(ribbon_manager.execute_command("italic"));
        assert!(ribbon_manager.execute_command("underline"));
        assert!(ribbon_manager.execute_command("bullets"));
        assert!(ribbon_manager.execute_command("find"));
    }
}