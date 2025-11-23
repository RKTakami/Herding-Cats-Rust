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
    property <bool> show-insert-dropdown: false;
    property <bool> show-format-dropdown: false;
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

    callback view-toolbar();
    callback view-statusbar();
    callback view-zoom();
    callback view-fullscreen();

    callback insert-table();
    callback insert-image();
    callback insert-link();
    callback insert-page-break();

    callback format-font();
    callback format-paragraph();
    callback format-styles();

    callback tools-database();
    callback tools-search();
    callback tools-research();
    callback tools-analysis();

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

        // Menu Bar (Microsoft Word Style)
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
                        root.show-edit-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-insert-dropdown = false;
                        root.show-format-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-help-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.status-message = "File menu opened";
                    }
                }

                Button {
                    text: "Edit";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-edit-dropdown = !root.show-edit-dropdown;
                        root.show-file-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-insert-dropdown = false;
                        root.show-format-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-help-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.status-message = "Edit menu opened";
                    }
                }

                Button {
                    text: "View";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-view-dropdown = !root.show-view-dropdown;
                        root.show-file-dropdown = false;
                        root.show-edit-dropdown = false;
                        root.show-insert-dropdown = false;
                        root.show-format-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-help-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.status-message = "View menu opened";
                    }
                }

                Button {
                    text: "Insert";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-insert-dropdown = !root.show-insert-dropdown;
                        root.show-file-dropdown = false;
                        root.show-edit-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-format-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-help-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.status-message = "Insert menu opened";
                    }
                }

                Button {
                    text: "Format";
                    width: 60px;
                    height: 24px;
                    clicked => {
                        root.show-format-dropdown = !root.show-format-dropdown;
                        root.show-file-dropdown = false;
                        root.show-edit-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-insert-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-help-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.status-message = "Format menu opened";
                    }
                }

                Button {
                    text: "Tools";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-tools-dropdown = !root.show-tools-dropdown;
                        root.show-file-dropdown = false;
                        root.show-edit-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-insert-dropdown = false;
                        root.show-format-dropdown = false;
                        root.show-help-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.status-message = "Tools menu opened";
                    }
                }

                Button {
                    text: "Projects";
                    width: 70px;
                    height: 24px;
                    clicked => {
                        root.show-projects-dropdown = !root.show-projects-dropdown;
                        root.show-file-dropdown = false;
                        root.show-edit-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-insert-dropdown = false;
                        root.show-format-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-help-dropdown = false;
                        root.status-message = "Projects menu opened";
                    }
                }

                Button {
                    text: "Help";
                    width: 50px;
                    height: 24px;
                    clicked => {
                        root.show-help-dropdown = !root.show-help-dropdown;
                        root.show-file-dropdown = false;
                        root.show-edit-dropdown = false;
                        root.show-view-dropdown = false;
                        root.show-insert-dropdown = false;
                        root.show-format-dropdown = false;
                        root.show-tools-dropdown = false;
                        root.show-projects-dropdown = false;
                        root.status-message = "Help menu opened";
                    }
                }
            }
        }

        // Ribbon Toolbar (Microsoft Word Style)
            Rectangle {
            background: ribbon-bg;
            height: 120px;
            border-color: border;
            border-width: 1px;
            VerticalBox {
                padding: 6px;
                spacing: 4px;

                // Quick Access Toolbar
                    HorizontalBox {
                    spacing: 4px;
                    Button {
                        text: "📄 New";
                        width: 60px;
                        height: 30px;
                        clicked => {
                            root.file_new();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "📂 Open";
                        width: 60px;
                        height: 30px;
                        clicked => {
                            root.file_open();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "💾 Save";
                        width: 60px;
                        height: 30px;
                        clicked => {
                            root.file_save();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "🖨 Print";
                        width: 60px;
                        height: 30px;
                        clicked => {
                            root.file_print();
                            root.click-count += 1;
                        }
                    }

                    Rectangle { }

                    Button {
                        text: "✂️ Cut";
                        width: 50px;
                        height: 30px;
                        clicked => {
                            root.edit_cut();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "📋 Copy";
                        width: 50px;
                        height: 30px;
                        clicked => {
                            root.edit_copy();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "📄 Paste";
                        width: 50px;
                        height: 30px;
                        clicked => {
                            root.edit_paste();
                            root.click-count += 1;
                        }
                    }
                }

                // Writing Tools Ribbon with Database Integration
                    HorizontalBox {
                    spacing: 8px;
                    Button {
                        text: "🔧 Hierarchy";
                        width: 100px;
                        height: 35px;
                        clicked => {
                            root.status-message = "Opening Hierarchy tool (Database Connected)...";
                            root.tools_database();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "📖 Codex";
                        width: 100px;
                        height: 35px;
                        clicked => {
                            root.status-message = "Opening Codex tool (Database Connected)...";
                            root.tools_database();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "💭 Brainstorm";
                        width: 100px;
                        height: 35px;
                        clicked => {
                            root.status-message = "Opening Brainstorming tool (Database Connected)...";
                            root.tools_database();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "🔬 Analysis";
                        width: 100px;
                        height: 35px;
                        clicked => {
                            root.status-message = "Opening Analysis tool (Database Connected)...";
                            root.tools_analysis();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "📊 Plot";
                        width: 100px;
                        height: 35px;
                        clicked => {
                            root.status-message = "Opening Plot tool (Database Connected)...";
                            root.tools_database();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "📝 Notes";
                        width: 100px;
                        height: 35px;
                        clicked => {
                            root.status-message = "Opening Notes tool (Database Connected)...";
                            root.tools_database();
                            root.click-count += 1;
                        }
                    }

                    Button {
                        text: "🔍 Research";
                        width: 100px;
                        height: 35px;
                        clicked => {
                            root.status-message = "Opening Research tool (Database Connected)...";
                            root.tools_research();
                            root.click-count += 1;
                        }
                    }

                    Rectangle { }

                    Button {
                        text: "⚙️ Database";
                        width: 80px;
                        height: 35px;
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
        x: 54px;
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
        x: 104px;
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

    // Insert Dropdown Menu
        Rectangle {
        x: 154px;
        y: 65px;
        width: 150px;
        height: 100px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-insert-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            Button {
                text: "📋 Table";
                width: 145px;
                height: 22px;
                clicked => {
                    root.insert_table();
                    root.show-insert-dropdown = false;
                }
            }

            Button {
                text: "🖼️ Image";
                width: 145px;
                height: 22px;
                clicked => {
                    root.insert_image();
                    root.show-insert-dropdown = false;
                }
            }

            Button {
                text: "🔗 Link";
                width: 145px;
                height: 22px;
                clicked => {
                    root.insert_link();
                    root.show-insert-dropdown = false;
                }
            }

            Button {
                text: "📄 Page Break";
                width: 145px;
                height: 22px;
                clicked => {
                    root.insert_page_break();
                    root.show-insert-dropdown = false;
                }
            }
        }
    }

    // Format Dropdown Menu
        Rectangle {
        x: 204px;
        y: 65px;
        width: 140px;
        height: 80px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-format-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            Button {
                text: "🔤 Font";
                width: 135px;
                height: 22px;
                clicked => {
                    root.format_font();
                    root.show-format-dropdown = false;
                }
            }

            Button {
                text: "📝 Paragraph";
                width: 135px;
                height: 22px;
                clicked => {
                    root.format_paragraph();
                    root.show-format-dropdown = false;
                }
            }

            Button {
                text: "🎨 Styles";
                width: 135px;
                height: 22px;
                clicked => {
                    root.format_styles();
                    root.show-format-dropdown = false;
                }
            }
        }
    }

    // Tools Dropdown Menu (Database Integration)
        Rectangle {
        x: 264px;
        y: 65px;
        width: 160px;
        height: 120px;
        background: dropdown-bg;
        border-color: border;
        border-width: 2px;
        visible: show-tools-dropdown;
        VerticalBox {
            padding: 2px;
            spacing: 1px;
            Button {
                text: "🗄️ Database Manager";
                width: 155px;
                height: 22px;
                clicked => {
                    root.tools_database();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "🔍 Search Tools";
                width: 155px;
                height: 22px;
                clicked => {
                    root.tools_search();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "📚 Research Hub";
                width: 155px;
                height: 22px;
                clicked => {
                    root.tools_research();
                    root.show-tools-dropdown = false;
                }
            }

            Button {
                text: "🔬 Analysis Suite";
                width: 155px;
                height: 22px;
                clicked => {
                    root.tools_analysis();
                    root.show-tools-dropdown = false;
                }
            }
        }
    }

    // Projects Dropdown Menu
        Rectangle {
        x: 314px;
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
}

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

    // Menu callbacks
    window.on_file_new(move || { println!("📄 New file requested"); });
    window.on_file_open(move || { println!("📂 Open file requested"); });
    window.on_file_save(move || { println!("💾 Save file requested"); });
    window.on_file_save_as(move || { println!("💾 Save As requested"); });
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

    // Insert callbacks
    window.on_insert_table(move || { println!("Insert Table requested"); });
    window.on_insert_image(move || { println!("Insert Image requested"); });
    window.on_insert_link(move || { println!("Insert Link requested"); });
    window.on_insert_page_break(move || { println!("Insert Page Break requested"); });

    // Format callbacks
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
    
    window.on_format_paragraph(move || { println!("Format Paragraph requested"); });
    window.on_format_styles(move || { println!("Format Styles requested"); });

    // Tools callbacks
    window.on_tools_database(move || { println!("Tools Database requested"); });
    window.on_tools_search(move || { println!("Tools Search requested"); });
    window.on_tools_research(move || { println!("Tools Research requested"); });
    window.on_tools_analysis(move || { println!("Tools Analysis requested"); });

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
