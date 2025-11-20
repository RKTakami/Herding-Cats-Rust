//! Ribbon Dropdown Component
//! 
//! Implements dropdown menus for ribbon interface.

use crate::ui::ribbon::{RibbonDropdown, RibbonDropdownItem};

/// Create a paste dropdown with different paste options
pub fn create_paste_dropdown() -> RibbonDropdown {
    let mut dropdown = RibbonDropdown::new("paste_dropdown".to_string(), "Paste".to_string());
    
    dropdown.add_item(RibbonDropdownItem::new("paste_keep_text".to_string(), "Keep Text Only".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("paste_keep_formatting".to_string(), "Keep Source Formatting".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("paste_merge".to_string(), "Merge Formatting".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("paste_picture".to_string(), "Paste as Picture".to_string()));
    
    dropdown
}

/// Create a font family dropdown
pub fn create_font_family_dropdown() -> RibbonDropdown {
    let mut dropdown = RibbonDropdown::new("font_family".to_string(), "Calibri".to_string());
    
    dropdown.add_item(RibbonDropdownItem::new("font_calibri".to_string(), "Calibri".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("font_times".to_string(), "Times New Roman".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("font_aria".to_string(), "Arial".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("font_consolas".to_string(), "Consolas".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("font_courier".to_string(), "Courier New".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("font_georgia".to_string(), "Georgia".to_string()));
    
    dropdown
}

/// Create a font size dropdown
pub fn create_font_size_dropdown() -> RibbonDropdown {
    let mut dropdown = RibbonDropdown::new("font_size".to_string(), "11".to_string());
    
    dropdown.add_item(RibbonDropdownItem::new("size_8".to_string(), "8".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_9".to_string(), "9".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_10".to_string(), "10".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_11".to_string(), "11".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_12".to_string(), "12".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_14".to_string(), "14".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_16".to_string(), "16".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_18".to_string(), "18".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_20".to_string(), "20".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_24".to_string(), "24".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_28".to_string(), "28".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_36".to_string(), "36".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_48".to_string(), "48".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("size_72".to_string(), "72".to_string()));
    
    dropdown
}

/// Create a text color dropdown
pub fn create_text_color_dropdown() -> RibbonDropdown {
    let mut dropdown = RibbonDropdown::new("text_color".to_string(), "Text Color".to_string());
    
    dropdown.add_item(RibbonDropdownItem::new("color_black".to_string(), "Black".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("color_blue".to_string(), "Blue".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("color_red".to_string(), "Red".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("color_green".to_string(), "Green".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("color_purple".to_string(), "Purple".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("color_orange".to_string(), "Orange".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("color_yellow".to_string(), "Yellow".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("color_white".to_string(), "White".to_string()));
    
    dropdown
}

/// Create an alignment dropdown
pub fn create_alignment_dropdown() -> RibbonDropdown {
    let mut dropdown = RibbonDropdown::new("alignment".to_string(), "Align Left".to_string());
    
    dropdown.add_item(RibbonDropdownItem::new("align_left".to_string(), "Align Left".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("align_center".to_string(), "Align Center".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("align_right".to_string(), "Align Right".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("align_justify".to_string(), "Justify".to_string()));
    
    dropdown
}

/// Create a line spacing dropdown
pub fn create_line_spacing_dropdown() -> RibbonDropdown {
    let mut dropdown = RibbonDropdown::new("line_spacing".to_string(), "Line Spacing".to_string());
    
    dropdown.add_item(RibbonDropdownItem::new("spacing_1".to_string(), "1.0".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("spacing_1_15".to_string(), "1.15".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("spacing_1_5".to_string(), "1.5".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("spacing_2".to_string(), "2.0".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("spacing_2_5".to_string(), "2.5".to_string()));
    dropdown.add_item(RibbonDropdownItem::new("spacing_3".to_string(), "3.0".to_string()));
    
    dropdown
}