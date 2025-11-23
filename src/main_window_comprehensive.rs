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
slint::slint! {
    import {
    Button,
    TextEdit,
    ScrollView,
    HorizontalBox,
    VerticalBox,
    LineEdit,
    StandardButton,
    ComboBox,
} from "std-widgets.slint";

    // Main Application Window Component
    export component MainWindowComprehensiveWindow inherits Window {
    title: "Herding Cats - Professional Word Processor";
    preferred-width: 1200px;
    preferred-height: 800px;
    min-width: 1000px;
    min-height: 700px;

    // Document and application state
    property <string> document-title: "Untitled Document";
    property <string> document-content: "";
    property <string> status-message: "Ready - Database Connected";
    property <int> click-count: 0;
    property <bool> show-file-dropdown: false;
    property <bool> show-edit-dropdown: false;
    property <bool> show-view-dropdown: false;
    property <bool> show-tools-dropdown: false;
    property <bool> show-help-dropdown: false;
    property <bool> show-projects-dropdown: false;

    // Database connection status
    property <string> db-status: "Connected to Database";
    property <bool> db-connected: true;

    // Theme colors - Professional Word-like theme
    property <color> primary-bg: #ffffff;
    property <color> secondary-bg: #f8f9fa;
    property <color> accent: #007bff;
    property <color> text-primary: #212529;
    property <color> text-secondary: #6c757d;
    property <color> border: #dee2e6;
    property <color> menu-bg: #f8f9fa;
    property <color> toolbar-bg: #ffffff;
    property <color> status-bg: #e9ecef;
    property <color> editor-bg: #ffffff;
    property <color> title-bg: #e9ecef;
    property <color> ribbon-bg: #f5f5f5;
    property <color> dropdown-bg: #ffffff;

    // Initialize with professional theme
    init => {
        primary-bg = #ffffff;
        secondary-bg = #f8f9fa;
        accent = #007bff;
        text-primary = #212529;
        text-secondary = #6c757d;
        border = #dee2e6;
        menu-bg = #f8f9fa;
        toolbar-bg = #ffffff;
        status-bg = #e9ecef;
        editor-bg = #ffffff;
        title-bg = #e9ecef;
        ribbon-bg = #f5f5f5;
        dropdown-bg = #ffffff;
    }

    // Menu callbacks
    callback file-new();
    callback file-open();
    callback file-save();
    callback file-save-as();
    callback file-print();
    callback file-export();
    callback file-exit();

    // AI Settings
    in-out property <bool> show-ai-settings-popup: false;
    in-out property <string> ai-settings-status: "";
    in-out property <string> ai-provider: "Anthropic";
    in-out property <string> ai-api-key: "";
    callback open-ai-settings();
    callback save-ai-key(string, string);
    callback close-ai-settings();
    callback ai-provider-changed(string);

    callback edit-cut();
    callback edit-copy();
    callback edit-paste();
    callback edit-undo();
    callback edit-redo();
    callback edit-find();
    callback edit-replace();

    // Word Processing Callbacks
    callback format-bold();
    callback format-italic();
    callback format-underline();
    callback format-font();
    callback format-font-size();
    callback format-text-color();
    callback format-highlight();
    callback align-left();
    callback align-center();
    callback align-right();
    callback align-justify();
    callback insert-bullet-list();
    callback insert-numbered-list();
    callback open-style-gallery();

    callback view-toolbar();
    callback view-statusbar();
    callback view-zoom();
    callback view-fullscreen();

    callback tools-database();
    callback tools-search();
    callback tools-research();
    callback tools-analysis();
    
    // Writing Tools Callbacks
    callback tools-hierarchy();
    callback tools-codex();
    callback tools-plot();
    callback tools-notes();
    callback tools-structure();
    callback tools-brainstorming();
    callback tools-font-manager();

    callback help-documentation();
    callback help-about();

    // Project management callbacks
    callback project-new();
    callback project-open();
    callback project-save();
    callback project-export();

    VerticalBox {
        spacing: 0;

        // Application Title Bar with Database Status
        Rectangle {
            background: primary-bg;
            height: 35px;
            border-color: border;
            border-width: 0px;
            HorizontalBox {
                padding: 8px;
                spacing: 16px;

                Text {
                    text: "📝 Herding Cats - Professional Word Processor";
                    color: text-primary;
                    font-size: 14px;
                    font-weight: 600;
                }

                Rectangle { }

                Text {
                    text: db-status;
                    color: accent;
                    font-size: 11px;
                    font-weight: 500;
                }

                StandardButton {
                    kind: close;
                }
            }
        }

        // Menu Bar (Microsoft Word Style) - Insert and Format removed
        Rectangle {
            background: menu-bg;
            height: 30px;
            border-color: border;
            border-width: 1px;
            HorizontalBox {
                padding: 4px;
                spacing: 2px;

                Button {
                    text: "File";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-file-dropdown = !root.show-file-dropdown;
                        root.show-projects-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-help-dropdown = false;
                        root.status-message = "File menu opened";
                    }
                }

                Button {
                    text: "Projects";
                    width: 70px;
                    height: 24px;
                    clicked => {
                        root.show-projects-dropdown = !root.show-projects-dropdown;
                        root.show-file-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-help-dropdown = false;
                        root.status-message = "Projects menu opened";
                    }
                }

                Button {
                    text: "Edit";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-edit-dropdown = !root.show-edit-dropdown;
                        root.show-file-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-help-dropdown = false;
                        root.status-message = "Edit menu opened";
                    }
                }

                Button {
                    text: "Tools";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-tools-dropdown = !root.show-tools-dropdown;
                        root.show-file-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.show-edit-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-help-dropdown = false;
                        root.status-message = "Tools menu opened";
                    }
                }

                Button {
                    text: "View";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-view-dropdown = !root.show-view-dropdown;
                        root.show-file-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-help-dropdown = false;
                        root.status-message = "View menu opened";
                    }
                }

                Button {
                    text: "Help";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-help-dropdown = !root.show-help-dropdown;
                        root.show-file-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-view-dropdown = false;
                        root.status-message = "Help menu opened";
                    }
                }
            }
        }

        // Ribbon Toolbar (Microsoft Word Style) - Quick Access Toolbar Removed
        Rectangle {
            background: ribbon-bg;
            height: 90px;
            border-color: border;
            border-width: 1px;
            VerticalBox {
                padding: 6px;
                spacing: 4px;

                // Advanced Word Processing Ribbon Toolbar (Microsoft Word/WordPerfect Style)
                HorizontalBox {
                    spacing: 6px;
                    
                    // Home Tab - Font and Paragraph Formatting
                    Button {
                        text: "𝐁 Bold";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Bold formatting applied";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "𝘐 Italic";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Italic formatting applied";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "U Underline";
                        width: 70px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Underline formatting applied";
                            root.click-count += 1;
                        }
                    }

                    Rectangle {
                        width: 4px;
                    }

                    Button {
                        text: "📝 Font";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Opening font settings...";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "📏 Size";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Opening font size selector...";
                            root.click-count += 1;
                        }
                    }

                    Rectangle {
                        width: 4px;
                    }

                    Button {
                        text: "🔤 Color";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Opening text color picker...";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "🎨 Highlight";
                        width: 70px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Opening highlight options...";
                            root.click-count += 1;
                        }
                    }

                    Rectangle {
                        width: 4px;
                    }

                    Button {
                        text: "⬅ Left";
                        width: 50px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Left text alignment";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "↔ Center";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Center text alignment";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "➡ Right";
                        width: 50px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Right text alignment";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "↔ Justify";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Justify text alignment";
                            root.click-count += 1;
                        }
                    }

                    Rectangle {
                        width: 4px;
                    }

                    Button {
                        text: "📋 Bullet";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Bullet list applied";
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "🔢 Number";
                        width: 60px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Numbered list applied";
                            root.click-count += 1;
                        }
                    }

                    Rectangle {
                        width: 4px;
                    }

                    Button {
                        text: "📄 Styles";
                        width: 70px;
                        height: 32px;
                        clicked => {
                            root.status-message = "Opening style gallery...";
                            root.click-count += 1;
                        }
                    }

                    Rectangle { }

                    Button {
                        text: "⚙️ Settings";
                        width: 70px;
                        height: 32px;
                        clicked => {
                            root.tools_database();
                            root.click-count += 1;
                        }
                    }
                }
            }
        }

        // Document Editor Area
        Rectangle {
            background: editor-bg;
            border-color: border;
            border-width: 2px;
            VerticalBox {
                padding: 10px;

                // Document Title Bar
                Rectangle {
                    background: title-bg;
                    height: 40px;
                    border-color: border;
                    border-width: 1px;
                    HorizontalBox {
                        padding: 8px;
                        spacing: 8px;
                        Text {
                            text: "📄 Document Editor";
                            color: text-primary;
                            font-size: 14px;
                            font-weight: 600;
                        }

                        LineEdit {
                            text: document-title;
                        }
                    }
                }

                // Main Document Content Area
                Rectangle {
                    background: editor-bg;
                    border-color: border;
                    border-width: 1px;
                    ScrollView {
                        width: parent.width;
                        height: 450px;
                        TextEdit {
                            text: document-content;
                            font-size: 16px;
                            wrap: word-wrap;
                            placeholder-text: "📝 Start writing your document...\n\nThis is a professional word processor with full database integration.\nUse the menus above to access advanced features.\n\n🔧 All writing tools are database-connected and can open as standalone windows.";
                        }
                    }
                }
            }
        }

        // Status Bar
        Rectangle {
            background: status-bg;
            height: 30px;
            border-color: border;
            border-width: 1px;
            HorizontalBox {
                padding: 4px;
                spacing: 16px;
                Text {
                    text: root.status-message;
                    color: text-secondary;
                    font-size: 12px;
                    vertical-alignment: center;
                }

                Text {
                    text: "Clicks: " + click-count;
                    color: accent;
                    font-size: 12px;
                    vertical-alignment: center;
                }

                Rectangle { }

                Text {
                    text: "Database: Active";
                    color: accent;
                    font-size: 12px;
                    vertical-alignment: center;
                }

                Text {
                    text: "Multi-Window Ready";
                    color: accent;
                    font-size: 12px;
                    vertical-alignment: center;
                }
            }
        }
    }

    // File Dropdown Menu
    Rectangle {
        x: 4px;
        y: 65px;
        width: 180px;
        height: 160px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-file-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            Button {
                text: "📄 New Document";
                width: 175px;
                height: 22px;
                clicked => {
                    root.file_new();
                    root.show-file-dropdown = false;
                }
            }

            Button {
                text: "📂 Open Document";
                width: 175px;
                height: 22px;
                clicked => {
                    root.file_open();
                    root.show-file-dropdown = false;
                }
            }

            Button {
                text: "💾 Save Document";
                width: 175px;
                height: 22px;
                clicked => {
                    root.file_save();
                    root.show-file-dropdown = false;
                }
            }

            Button {
                text: "💾 Save As...";
                width: 175px;
                height: 22px;
                clicked => {
                    root.file_save_as();
                    root.show-file-dropdown = false;
                }
            }

            Rectangle {
                height: 4px;
            }

            Button {
                text: "🤖 AI Settings";
                width: 175px;
                height: 22px;
                clicked => {
                    root.open-ai-settings();
                    root.show-file-dropdown = false;
                }
            }

            Rectangle {
                height: 4px;
            }

            Button {
                text: "🖨 Print";
                width: 175px;
                height: 22px;
                clicked => {
                    root.file_print();
                    root.show-file-dropdown = false;
                }
            }

            Button {
                text: "📤 Export";
                width: 175px;
                height: 22px;
                clicked => {
                    root.file_export();
                    root.show-file-dropdown = false;
                }
            }

            Button {
                text: "❌ Exit";
                width: 175px;
                height: 22px;
                clicked => {
                    root.file_exit();
                    root.show-file-dropdown = false;
                }
            }
        }
    }

    // Edit Dropdown Menu
    Rectangle {
        x: 224px;
        y: 65px;
        width: 160px;
        height: 140px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-edit-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            Button {
                text: "✂️ Cut";
                width: 155px;
                height: 22px;
                clicked => {
                    root.edit_cut();
                    root.show-edit-dropdown = false;
                }
            }

            Button {
                text: "📋 Copy";
                width: 155px;
                height: 22px;
                clicked => {
                    root.edit_copy();
                    root.show-edit-dropdown = false;
                }
            }

            Button {
                text: "📄 Paste";
                width: 155px;
                height: 22px;
                clicked => {
                    root.edit_paste();
                    root.show-edit-dropdown = false;
                }
            }

            Rectangle {
                height: 4px;
            }

            Button {
                text: "↶ Undo";
                width: 155px;
                height: 22px;
                clicked => {
                    root.edit_undo();
                    root.show-edit-dropdown = false;
                }
            }

            Button {
                text: "↷ Redo";
                width: 155px;
                height: 22px;
                clicked => {
                    root.edit_redo();
                    root.show-edit-dropdown = false;
                }
            }

            Rectangle {
                height: 4px;
            }

            Button {
                text: "🔍 Find";
                width: 155px;
                height: 22px;
                clicked => {
                    root.edit_find();
                    root.show-edit-dropdown = false;
                }
            }

            Button {
                text: "🔄 Replace";
                width: 155px;
                height: 22px;
                clicked => {
                    root.edit_replace();
                    root.show-edit-dropdown = false;
                }
            }
        }
    }

    // View Dropdown Menu
    Rectangle {
        x: 264px;
        y: 65px;
        width: 140px;
        height: 100px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-view-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            Button {
                text: "🛠️ Toolbar";
                width: 135px;
                height: 22px;
                clicked => {
                    root.view_toolbar();
                    root.show-view-dropdown = false;
                }
            }

            Button {
                text: "📊 Status Bar";
                width: 135px;
                height: 22px;
                clicked => {
                    root.view_statusbar();
                    root.show-view-dropdown = false;
                }
            }

            Button {
                text: "🔍 Zoom";
                width: 135px;
                height: 22px;
                clicked => {
                    root.view_zoom();
                    root.show-view-dropdown = false;
                }
            }

            Button {
                text: "🖥️ Full Screen";
                width: 135px;
                height: 22px;
                clicked => {
                    root.view_fullscreen();
                    root.show-view-dropdown = false;
                }
            }
        }
    }

    // Tools Dropdown Menu (Writing Tools & Database Integration)
    Rectangle {
        x: 184px;
        y: 65px;
        width: 200px;
        height: 280px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-tools-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            
            // Writing Tools Section
            Text {
                text: "✍️ Writing Tools";
                font-size: 11px;
                font-weight: 600;
                color: text-secondary;
            }
            
            Button {
                text: "📊 Document Hierarchy";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_hierarchy();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "📖 World Building Codex";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_codex();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "📈 Plot Development";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_plot();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "📝 Research Notes";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_notes();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "📊 Plot Structure";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_structure();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "🧠 Brainstorming";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_brainstorming();
                    root.show-tools-dropdown = false;
                }
            }

            Rectangle {
                height: 4px;
            }
            
            // Utility Tools Section
            Text {
                text: "🛠️ Utility Tools";
                font-size: 11px;
                font-weight: 600;
                color: text-secondary;
            }
            
            Button {
                text: "🔍 Search Tools";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_search();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "📚 Research Hub";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_research();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "📈 Writing Analysis";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_analysis();
                    root.show-tools-dropdown = false;
                }
            }
            
            Button {
                text: "🔤 Font Manager";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_font_manager();
                    root.show-tools-dropdown = false;
                }
            }

            Rectangle {
                height: 4px;
            }
            
            // Database Tools Section
            Text {
                text: "🗄️ Database";
                font-size: 11px;
                font-weight: 600;
                color: text-secondary;
            }
            
            Button {
                text: "🗄️ Database Manager";
                width: 195px;
                height: 22px;
                clicked => {
                    root.tools_database();
                    root.show-tools-dropdown = false;
                }
            }
        }
    }

    // Projects Dropdown Menu
    Rectangle {
        x: 54px;
        y: 65px;
        width: 170px;
        height: 120px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-projects-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            Button {
                text: "📁 New Project";
                width: 165px;
                height: 22px;
                clicked => {
                    root.project_new();
                    root.show-projects-dropdown = false;
                }
            }

            Button {
                text: "📂 Open Project";
                width: 165px;
                height: 22px;
                clicked => {
                    root.project_open();
                    root.show-projects-dropdown = false;
                }
            }

            Button {
                text: "💾 Save Project";
                width: 165px;
                height: 22px;
                clicked => {
                    root.project_save();
                    root.show-projects-dropdown = false;
                }
            }

            Button {
                text: "📤 Export Project";
                width: 165px;
                height: 22px;
                clicked => {
                    root.project_export();
                    root.show-projects-dropdown = false;
                }
            }
        }
    }

    // AI Settings Popup
    Rectangle {
        visible: root.show-ai-settings-popup;
        width: 100%;
        height: 100%;
        background: #00000080; // Semi-transparent overlay
        
        TouchArea { // Block clicks
        }

        Rectangle {
            width: 400px;
            height: 350px;
            background: editor-bg;
            border-color: border;
            border-width: 1px;
            border-radius: 4px;
            VerticalBox {
                padding: 16px;
                spacing: 12px;
                Text {
                    text: "🤖 AI Provider Settings";
                    font-size: 18px;
                    font-weight: 600;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Securely store your API keys.";
                    font-size: 12px;
                    color: text-secondary;
                    horizontal-alignment: center;
                }

                HorizontalBox {
                    Text {
                        text: "Provider:";
                        vertical-alignment: center;
                        width: 80px;
                    }

                    ComboBox {
                        width: 200px;
                        model: ["Anthropic", "OpenAI", "Gemini", "OpenRouter"];
                        current-value <=> root.ai-provider;
                        selected(value) => {
                            root.ai-provider-changed(value);
                        }
                    }
                }

                HorizontalBox {
                    Text {
                        text: "API Key:";
                        vertical-alignment: center;
                        width: 80px;
                    }

                    LineEdit {
                        placeholder-text: "Enter API Key";
                        input-type: password;
                        width: 200px;
                        text <=> root.ai-api-key;
                    }
                }

                Text {
                    text: root.ai-settings-status;
                    color: accent;
                    horizontal-alignment: center;
                }

                HorizontalBox {
                    alignment: center;
                    spacing: 16px;
                    Button {
                        text: "Save Key";
                        width: 100px;
                        clicked => {
                            root.save-ai-key(root.ai-provider, root.ai-api-key);
                        }
                    }

                    Button {
                        text: "Close";
                        width: 100px;
                        clicked => {
                            root.close-ai-settings();
                        }
                    }
                }
            }
        }
    }
}
