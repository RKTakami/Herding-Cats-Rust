//! Insert Tab Implementation
//! 
//! Implements the Insert tab with tools for adding content to documents.

use crate::ui::ribbon::{RibbonTab, RibbonGroup, RibbonItem, RibbonButton, RibbonDropdown, RibbonDropdownItem};

/// Create the Insert tab with content insertion tools
pub fn create_insert_tab() -> RibbonTab {
    let mut insert_tab = RibbonTab::new("insert".to_string(), "Insert".to_string());

    // Tables group
    let mut tables_group = RibbonGroup::new("tables".to_string(), "Tables".to_string());
    
    let mut table_dropdown = RibbonDropdown::new("table_dropdown".to_string(), "Table".to_string());
    table_dropdown.add_item(RibbonDropdownItem::new("insert_table".to_string(), "Insert Table".to_string()));
    table_dropdown.add_item(RibbonDropdownItem::new("excel_table".to_string(), "Excel Table".to_string()));
    table_dropdown.add_item(RibbonDropdownItem::new("quick_table".to_string(), "Quick Tables".to_string()));
    
    tables_group.add_item(RibbonItem::Dropdown(table_dropdown));
    insert_tab.add_group(tables_group);

    // Illustrations group
    let mut illustrations_group = RibbonGroup::new("illustrations".to_string(), "Illustrations".to_string());
    
    illustrations_group.add_item(RibbonItem::Button(RibbonButton::new("picture".to_string(), "Pictures".to_string())));
    illustrations_group.add_item(RibbonItem::Button(RibbonButton::new("online_pictures".to_string(), "Online Pictures".to_string())));
    illustrations_group.add_item(RibbonItem::Button(RibbonButton::new("shapes".to_string(), "Shapes".to_string())));
    illustrations_group.add_item(RibbonItem::Button(RibbonButton::new("smartart".to_string(), "SmartArt".to_string())));
    illustrations_group.add_item(RibbonItem::Button(RibbonButton::new("chart".to_string(), "Chart".to_string())));
    
    insert_tab.add_group(illustrations_group);

    // Links group
    let mut links_group = RibbonGroup::new("links".to_string(), "Links".to_string());
    
    links_group.add_item(RibbonItem::Button(RibbonButton::new("link".to_string(), "Link".to_string())));
    links_group.add_item(RibbonItem::Button(RibbonButton::new("bookmark".to_string(), "Bookmark".to_string())));
    
    insert_tab.add_group(links_group);

    // Headers & Footer group
    let mut headers_footer_group = RibbonGroup::new("headers_footer".to_string(), "Header & Footer".to_string());
    
    headers_footer_group.add_item(RibbonItem::Button(RibbonButton::new("header".to_string(), "Header".to_string())));
    headers_footer_group.add_item(RibbonItem::Button(RibbonButton::new("footer".to_string(), "Footer".to_string())));
    headers_footer_group.add_item(RibbonItem::Button(RibbonButton::new("page_number".to_string(), "Page Number".to_string())));
    
    insert_tab.add_group(headers_footer_group);

    // Text group
    let mut text_group = RibbonGroup::new("text".to_string(), "Text".to_string());
    
    let mut text_dropdown = RibbonDropdown::new("text_dropdown".to_string(), "Text Box".to_string());
    text_dropdown.add_item(RibbonDropdownItem::new("draw_text_box".to_string(), "Draw Text Box".to_string()));
    text_dropdown.add_item(RibbonDropdownItem::new("horizontal_text".to_string(), "Horizontal".to_string()));
    text_dropdown.add_item(RibbonDropdownItem::new("vertical_text".to_string(), "Vertical".to_string()));
    text_dropdown.add_item(RibbonDropdownItem::new("simple_text".to_string(), "Simple".to_string()));
    text_dropdown.add_item(RibbonDropdownItem::new("scrolling_text".to_string(), "Scrolling".to_string()));
    
    text_group.add_item(RibbonItem::Dropdown(text_dropdown));
    text_group.add_item(RibbonItem::Button(RibbonButton::new("quick_parts".to_string(), "Quick Parts".to_string())));
    text_group.add_item(RibbonItem::Button(RibbonButton::new("word_art".to_string(), "WordArt".to_string())));
    text_group.add_item(RibbonItem::Button(RibbonButton::new("drop_cap".to_string(), "Drop Cap".to_string())));
    text_group.add_item(RibbonItem::Button(RibbonButton::new("signature_line".to_string(), "Signature Line".to_string())));
    text_group.add_item(RibbonItem::Button(RibbonButton::new("date_picker".to_string(), "Date Picker".to_string())));
    
    insert_tab.add_group(text_group);

    // Symbols group
    let mut symbols_group = RibbonGroup::new("symbols".to_string(), "Symbols".to_string());
    
    symbols_group.add_item(RibbonItem::Button(RibbonButton::new("symbol".to_string(), "Symbol".to_string())));
    symbols_group.add_item(RibbonItem::Button(RibbonButton::new("equation".to_string(), "Equation".to_string())));
    
    insert_tab.add_group(symbols_group);

    insert_tab
}

/// Insert tab functionality and command handlers
pub struct InsertTabManager;

impl InsertTabManager {
    /// Register command handlers for the Insert tab
    pub fn register_commands(ribbon_manager: &mut crate::ui::ribbon::RibbonManager) {
        // Tables commands
        ribbon_manager.register_command("insert_table".to_string(), || {
            println!("📋 Table insertion requested");
        });
        
        ribbon_manager.register_command("excel_table".to_string(), || {
            println!("📊 Excel table requested");
        });
        
        ribbon_manager.register_command("quick_table".to_string(), || {
            println!("⚡ Quick table requested");
        });

        // Illustrations commands
        ribbon_manager.register_command("picture".to_string(), || {
            println!("🖼️ Picture insertion requested");
        });
        
        ribbon_manager.register_command("online_pictures".to_string(), || {
            println!("🌐 Online pictures requested");
        });
        
        ribbon_manager.register_command("shapes".to_string(), || {
            println!("🔶 Shape insertion requested");
        });
        
        ribbon_manager.register_command("smartart".to_string(), || {
            println!("🧠 SmartArt requested");
        });
        
        ribbon_manager.register_command("chart".to_string(), || {
            println!("📈 Chart requested");
        });

        // Links commands
        ribbon_manager.register_command("link".to_string(), || {
            println!("🔗 Link insertion requested");
        });
        
        ribbon_manager.register_command("bookmark".to_string(), || {
            println!("🔖 Bookmark requested");
        });

        // Header & Footer commands
        ribbon_manager.register_command("header".to_string(), || {
            println!("📄 Header requested");
        });
        
        ribbon_manager.register_command("footer".to_string(), || {
            println!("📄 Footer requested");
        });
        
        ribbon_manager.register_command("page_number".to_string(), || {
            println!("🔢 Page number requested");
        });

        // Text commands
        ribbon_manager.register_command("draw_text_box".to_string(), || {
            println!("📝 Draw text box requested");
        });
        
        ribbon_manager.register_command("horizontal_text".to_string(), || {
            println!("↔️ Horizontal text requested");
        });
        
        ribbon_manager.register_command("vertical_text".to_string(), || {
            println!("↕️ Vertical text requested");
        });
        
        ribbon_manager.register_command("quick_parts".to_string(), || {
            println!("⚡ Quick parts requested");
        });
        
        ribbon_manager.register_command("word_art".to_string(), || {
            println!("🎨 WordArt requested");
        });
        
        ribbon_manager.register_command("drop_cap".to_string(), || {
            println!("🔤 Drop cap requested");
        });
        
        ribbon_manager.register_command("signature_line".to_string(), || {
            println!("✍️ Signature line requested");
        });
        
        ribbon_manager.register_command("date_picker".to_string(), || {
            println!("📅 Date picker requested");
        });

        // Symbols commands
        ribbon_manager.register_command("symbol".to_string(), || {
            println!("🔧 Symbol requested");
        });
        
        ribbon_manager.register_command("equation".to_string(), || {
            println!("📐 Equation requested");
        });
    }
}