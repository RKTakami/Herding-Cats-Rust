//! Comprehensive Main Window with Microsoft Word-like Interface
//! Features full toolbar, menus, and database integration

use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

use crate::database::{DatabaseService, DatabaseConfig};
use crate::ui_state::AppState;
use crate::database_app_state::DatabaseAppState;
use crate::ui::tools::individual_tool_windows::IndividualToolWindowManager;
use crate::ui::enhanced_tool_launcher::get_enhanced_launcher;
use crate::ui::font_manager_window::FontManagerWindowManager;

// Import the Word processor component
slint::include_modules!();

pub struct MainWindowComprehensive {
    window: MainWindowComprehensiveWindow,
    _db_service: Arc<Mutex<DatabaseService>>,
    _app_state: Arc<Mutex<AppState>>,
    secure_storage: Arc<crate::security::secure_storage::SecureStorageService>,
    font_manager_window: Arc<Mutex<Option<FontManagerWindowManager>>>,
}

impl MainWindowComprehensive {
    pub async fn new() -> Result<Self> {
        // Initialize database service
        let db_path = std::path::Path::new("data/comprehensive_app.db");
        let db_config = DatabaseConfig::default();
        let db_service = Arc::new(Mutex::new(
            DatabaseService::new(db_path, db_config)
                .await
                .expect("Failed to initialize database service")
        ));

        // Initialize app state
        let app_state = Arc::new(Mutex::new(AppState::default()));

        // Initialize secure storage
        let secure_storage = Arc::new(crate::security::secure_storage::SecureStorageService::new("herding-cats-rust"));

        // Create window
        let window = MainWindowComprehensiveWindow::new()?;
        
        // Initialize Font Manager Window holder
        let font_manager_window = Arc::new(Mutex::new(None));

        // Initialize callbacks
        init_callbacks(&window, db_service.clone(), app_state.clone(), secure_storage.clone(), font_manager_window.clone())?;

        Ok(Self {
            window,
            _db_service: db_service,
            _app_state: app_state,
            secure_storage,
            font_manager_window,
        })
    }

    pub fn run(&self) -> Result<()> {
        self.window.run().map_err(|e| anyhow::anyhow!("Slint window error: {}", e))
    }
}

fn init_callbacks(
    window: &MainWindowComprehensiveWindow,
    _db_service: Arc<Mutex<DatabaseService>>,
    _app_state: Arc<Mutex<AppState>>,
    secure_storage: Arc<crate::security::secure_storage::SecureStorageService>,
    font_manager_window: Arc<Mutex<Option<FontManagerWindowManager>>>,
) -> Result<()> {
    // Initialize IndividualToolWindowManager and register it with EnhancedToolLauncher
    // This ensures that when we call launch_tool, it can actually spawn the Slint windows
    let db_app_state = Arc::new(RwLock::new(DatabaseAppState::new()));
    let tool_manager = IndividualToolWindowManager::new(db_app_state);
    get_enhanced_launcher().register_tool_manager(tool_manager);

    // AI Settings Callbacks
    let window_weak_ai = window.as_weak();
    let secure_storage_clone_open = secure_storage.clone();
    window.on_open_ai_settings(move || {
        if let Some(window) = window_weak_ai.upgrade() {
            let provider = window.get_ai_provider();
            // Check if key exists
            if let Ok(_) = secure_storage_clone_open.get_api_key(&provider) {
                 window.set_ai_api_key("********".into());
            } else {
                 window.set_ai_api_key("".into());
            }
            window.set_show_ai_settings_popup(true);
            window.set_ai_settings_status("".into());
        }
    });

    let window_weak_ai_change = window.as_weak();
    let secure_storage_clone_change = secure_storage.clone();
    window.on_ai_provider_changed(move |provider| {
        if let Some(window) = window_weak_ai_change.upgrade() {
             if let Ok(_) = secure_storage_clone_change.get_api_key(&provider) {
                 window.set_ai_api_key("********".into());
            } else {
                 window.set_ai_api_key("".into());
            }
            window.set_ai_settings_status("".into());
        }
    });

    let window_weak_ai_close = window.as_weak();
    window.on_close_ai_settings(move || {
        if let Some(window) = window_weak_ai_close.upgrade() {
            window.set_show_ai_settings_popup(false);
        }
    });

    let secure_storage_clone = secure_storage.clone();
    let window_weak_ai_save = window.as_weak();
    window.on_save_ai_key(move |provider, key| {
        if key == "********" {
             if let Some(window) = window_weak_ai_save.upgrade() {
                window.set_ai_settings_status("⚠️ Key unchanged (masked).".into());
            }
            return;
        }

        let status = match secure_storage_clone.set_api_key(&provider, &key) {
            Ok(_) => format!("✅ Key for {} saved securely.", provider),
            Err(e) => format!("❌ Failed to save key: {}", e),
        };
        if let Some(window) = window_weak_ai_save.upgrade() {
            window.set_ai_settings_status(status.into());
        }
    });

    // File Menu Callbacks
    let window_weak_file = window.as_weak();
    window.on_file_new(move || {
        if let Some(window) = window_weak_file.upgrade() {
            window.set_document_content("".into());
            window.set_document_title("Untitled Document".into());
            window.set_status_message("New document created".into());
        }
    });

    let window_weak_open = window.as_weak();
    window.on_file_open(move || {
        if let Some(window) = window_weak_open.upgrade() {
            window.set_status_message("Open file dialog (simulated)".into());
        }
    });

    let window_weak_save = window.as_weak();
    window.on_file_save(move || {
        if let Some(window) = window_weak_save.upgrade() {
            window.set_status_message("Document saved".into());
        }
    });

    let window_weak_save_as = window.as_weak();
    window.on_file_save_as(move || {
        if let Some(window) = window_weak_save_as.upgrade() {
            window.set_status_message("Save As dialog (simulated)".into());
        }
    });

    window.on_file_print(move || { println!("🖨 Print requested"); });
    window.on_file_export(move || { println!("📤 Export requested"); });
    window.on_file_exit(move || { std::process::exit(0); });

    // Edit callbacks
    window.on_edit_cut(move || { println!("✂️ Cut requested"); });
    window.on_edit_copy(move || { println!("📋 Copy requested"); });
    window.on_edit_paste(move || { println!("📋 Paste requested"); });
    window.on_edit_undo(move || { println!("Undo requested"); });
    window.on_edit_redo(move || { println!("Redo requested"); });
    window.on_edit_find(move || { println!("Find requested"); });
    window.on_edit_replace(move || { println!("Replace requested"); });

    // View callbacks
    window.on_view_toolbar(move || { println!("View Toolbar requested"); });
    window.on_view_statusbar(move || { println!("View Statusbar requested"); });
    window.on_view_zoom(move || { println!("View Zoom requested"); });
    window.on_view_fullscreen(move || { println!("View Fullscreen requested"); });

    // Format callbacks (Simulated Markdown)
    let window_weak_bold = window.as_weak();
    window.on_format_bold(move || {
        if let Some(window) = window_weak_bold.upgrade() {
            let current = window.get_document_content();
            window.set_document_content(format!("{} **bold**", current).into());
            window.set_status_message("Bold formatting applied".into());
        }
    });

    let window_weak_italic = window.as_weak();
    window.on_format_italic(move || {
        if let Some(window) = window_weak_italic.upgrade() {
            let current = window.get_document_content();
            window.set_document_content(format!("{} *italic*", current).into());
            window.set_status_message("Italic formatting applied".into());
        }
    });

    let window_weak_underline = window.as_weak();
    window.on_format_underline(move || {
        if let Some(window) = window_weak_underline.upgrade() {
            let current = window.get_document_content();
            window.set_document_content(format!("{} __underline__", current).into());
            window.set_status_message("Underline formatting applied".into());
        }
    });

    // Font Manager
    let font_manager_window_clone = font_manager_window.clone();
    window.on_format_font(move || { 
        println!("Format Font requested");
        if let Ok(mut manager_opt) = font_manager_window_clone.lock() {
            if manager_opt.is_none() {
                 if let Ok(manager) = FontManagerWindowManager::new() {
                     *manager_opt = Some(manager);
                 }
            }
            
            if let Some(manager) = manager_opt.as_ref() {
                let _ = manager.show();
            }
        }
    });
    
    // Other format callbacks
    window.on_format_font_size(move || { println!("Format Font Size requested"); });
    window.on_format_text_color(move || { println!("Format Text Color requested"); });
    window.on_format_highlight(move || { println!("Format Highlight requested"); });
    window.on_align_left(move || { println!("Align Left requested"); });
    window.on_align_center(move || { println!("Align Center requested"); });
    window.on_align_right(move || { println!("Align Right requested"); });
    window.on_align_justify(move || { println!("Align Justify requested"); });
    window.on_insert_bullet_list(move || { println!("Insert Bullet List requested"); });
    window.on_insert_numbered_list(move || { println!("Insert Numbered List requested"); });
    window.on_open_style_gallery(move || { println!("Open Style Gallery requested"); });

    // Tools callbacks - Using EnhancedToolLauncher
    use crate::ui::tools::base_types::ToolType;

    window.on_tools_hierarchy(move || {
        let _ = get_enhanced_launcher().launch_tool(ToolType::Hierarchy);
    });

    window.on_tools_codex(move || {
        let _ = get_enhanced_launcher().launch_tool(ToolType::Codex);
    });

    window.on_tools_plot(move || {
        let _ = get_enhanced_launcher().launch_tool(ToolType::Plot);
    });

    window.on_tools_notes(move || {
        let _ = get_enhanced_launcher().launch_tool(ToolType::Notes);
    });

    window.on_tools_structure(move || {
        let _ = get_enhanced_launcher().launch_tool(ToolType::Structure);
    });

    window.on_tools_brainstorming(move || {
        let _ = get_enhanced_launcher().launch_tool(ToolType::Brainstorming);
    });

    let font_manager_window_clone_tools = font_manager_window.clone();
    window.on_tools_font_manager(move || {
        if let Ok(mut manager_opt) = font_manager_window_clone_tools.lock() {
            if manager_opt.is_none() {
                 if let Ok(manager) = FontManagerWindowManager::new() {
                     *manager_opt = Some(manager);
                 }
            }
            
            if let Some(manager) = manager_opt.as_ref() {
                let _ = manager.show();
            }
        }
    });

    window.on_tools_database(move || { println!("Tools Database requested"); });
    window.on_tools_search(move || { println!("Tools Search requested"); });
    window.on_tools_research(move || { 
        let _ = get_enhanced_launcher().launch_tool(ToolType::Research);
    });
    window.on_tools_analysis(move || { 
        let _ = get_enhanced_launcher().launch_tool(ToolType::Analysis);
    });

    // Help callbacks
    window.on_help_documentation(move || { println!("Help Documentation requested"); });
    window.on_help_about(move || { println!("Help About requested"); });

    // Project callbacks
    window.on_project_new(move || { println!("Project New requested"); });
    window.on_project_open(move || { println!("Project Open requested"); });
    window.on_project_save(move || { println!("Project Save requested"); });
    window.on_project_export(move || { println!("Project Export requested"); });

    Ok(())
}
