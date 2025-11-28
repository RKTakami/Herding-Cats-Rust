//! Individual Writing Tool Windows
//!
//! Creates separate windows for each writing tool instead of the universal window approach.
//! Each tool has its own dedicated window with its own menu system and interface.

use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use std::cell::RefCell;
use std::collections::HashMap;
use slint::ComponentHandle;
use crate::ui::theme_manager::{get_current_theme_colors, ThemeColors};

use crate::ui::tools::base_types::ToolType;
use crate as hc_lib;
use hc_lib::{AppState, DatabaseAppState};

pub mod hierarchy {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Hierarchy Tool Window
    export component HierarchyToolWindow inherits Window {
        width: 800px;
        height: 600px;
        title: "Herding Cats - Hierarchy Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_item();
        callback delete_item();
        callback move_up();
        callback move_down();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // File Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_item(); }
                            }

                            Button {
                                text: "Delete";
                                width: 60px;
                                height: 27px;
                                clicked => { root.delete_item(); }
                            }
                        }
                    }

                    // Edit Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Up";
                                width: 50px;
                                height: 27px;
                                clicked => { root.move_up(); }
                            }

                            Button {
                                text: "Down";
                                width: 60px;
                                height: 27px;
                                clicked => { root.move_down(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "📚 Hierarchy Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "📚 Hierarchy Tool\n\n" +
                              "This tool helps you organize your manuscript structure.\n\n" +
                              "Features:\n" +
                              "• Chapter and scene management\n" +
                              "• Drag-and-drop reordering\n" +
                              "• Word count tracking\n" +
                              "• Structure visualization\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new hierarchy item\n" +
                              "• Delete: Remove selected item\n" +
                              "• Up: Move item up in hierarchy\n" +
                              "• Down: Move item down in hierarchy";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}


pub mod codex {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Codex Tool Window
    export component CodexToolWindow inherits Window {
        width: 800px;
        height: 600px;
        title: "Herding Cats - Codex Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_entry();
        callback search();
        callback export();
        callback import();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // File Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_entry(); }
                            }

                            Button {
                                text: "Search";
                                width: 65px;
                                height: 27px;
                                clicked => { root.search(); }
                            }
                        }
                    }

                    // Tools Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Export";
                                width: 65px;
                                height: 27px;
                                clicked => { root.export(); }
                            }

                            Button {
                                text: "Import";
                                width: 65px;
                                height: 27px;
                                clicked => { root.import(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "📖 Codex Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "📖 Codex Tool\n\n" +
                              "This tool helps you manage world building and reference materials.\n\n" +
                              "Features:\n" +
                              "• Character profiles\n" +
                              "• Location descriptions\n" +
                              "• World building elements\n" +
                              "• Reference material organization\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new codex entry\n" +
                              "• Search: Find codex entries\n" +
                              "• Export: Export codex data\n" +
                              "• Import: Import codex data";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}


pub mod brainstorming {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Brainstorming Tool Window
    export component BrainstormingToolWindow inherits Window {
        width: 900px;
        height: 700px;
        title: "Herding Cats - Brainstorming Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_node();
        callback layout();
        callback zoom_in();
        callback zoom_out();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // Canvas Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_node(); }
                            }

                            Button {
                                text: "Layout";
                                width: 65px;
                                height: 27px;
                                clicked => { root.layout(); }
                            }
                        }
                    }

                    // View Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Zoom+";
                                width: 50px;
                                height: 27px;
                                clicked => { root.zoom_in(); }
                            }

                            Button {
                                text: "Zoom-";
                                width: 50px;
                                height: 27px;
                                clicked => { root.zoom_out(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "💭 Brainstorming Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "💭 Brainstorming Tool\n\n" +
                              "This tool helps you generate and organize creative ideas.\n\n" +
                              "Features:\n" +
                              "• Mindmap visualization\n" +
                              "• Node creation and editing\n" +
                              "• Connection management\n" +
                              "• Layout algorithms\n" +
                              "• Drag-and-drop functionality\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new brainstorming node\n" +
                              "• Layout: Auto-arrange nodes\n" +
                              "• Zoom+: Zoom in on canvas\n" +
                              "• Zoom-: Zoom out on canvas";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}


pub mod analysis {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Analysis Tool Window
    export component AnalysisToolWindow inherits Window {
        width: 800px;
        height: 600px;
        title: "Herding Cats - Analysis Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_analysis();
        callback generate_insights();
        callback export();
        callback import();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // Analysis Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_analysis(); }
                            }

                            Button {
                                text: "Insights";
                                width: 70px;
                                height: 27px;
                                clicked => { root.generate_insights(); }
                            }
                        }
                    }

                    // Data Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Export";
                                width: 65px;
                                height: 27px;
                                clicked => { root.export(); }
                            }

                            Button {
                                text: "Import";
                                width: 65px;
                                height: 27px;
                                clicked => { root.import(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "🔬 Analysis Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "🔬 Analysis Tool\n\n" +
                              "This tool helps you analyze writing structure and patterns.\n\n" +
                              "Features:\n" +
                              "• Writing structure analysis\n" +
                              "• Pattern recognition\n" +
                              "• Insight generation\n" +
                              "• Progress tracking\n" +
                              "• Writing metrics\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new analysis\n" +
                              "• Insights: Generate writing insights\n" +
                              "• Export: Export analysis summary\n" +
                              "• Import: Import analysis data";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}


pub mod plot {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Plot Tool Window
    export component PlotToolWindow inherits Window {
        width: 800px;
        height: 600px;
        title: "Herding Cats - Plot Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_plot_point();
        callback timeline_view();
        callback export();
        callback import();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // Plot Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_plot_point(); }
                            }

                            Button {
                                text: "Timeline";
                                width: 75px;
                                height: 27px;
                                clicked => { root.timeline_view(); }
                            }
                        }
                    }

                    // Data Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Export";
                                width: 65px;
                                height: 27px;
                                clicked => { root.export(); }
                            }

                            Button {
                                text: "Import";
                                width: 65px;
                                height: 27px;
                                clicked => { root.import(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "📊 Plot Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "📊 Plot Tool\n\n" +
                              "This tool helps you develop story plots and narrative arcs.\n\n" +
                              "Features:\n" +
                              "• Plot point management\n" +
                              "• Narrative arc visualization\n" +
                              "• Timeline view\n" +
                              "• Character arc tracking\n" +
                              "• Conflict mapping\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new plot point\n" +
                              "• Timeline: Show timeline view\n" +
                              "• Export: Export plot data\n" +
                              "• Import: Import plot data";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}


pub mod notes {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Notes Tool Window
    export component NotesToolWindow inherits Window {
        width: 700px;
        height: 500px;
        title: "Herding Cats - Notes Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_note();
        callback tag_note();
        callback export();
        callback import();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // Notes Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_note(); }
                            }

                            Button {
                                text: "Tag";
                                width: 50px;
                                height: 27px;
                                clicked => { root.tag_note(); }
                            }
                        }
                    }

                    // Data Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Export";
                                width: 65px;
                                height: 27px;
                                clicked => { root.export(); }
                            }

                            Button {
                                text: "Import";
                                width: 65px;
                                height: 27px;
                                clicked => { root.import(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "📝 Notes Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "📝 Notes Tool\n\n" +
                              "This tool helps you take and organize research notes.\n\n" +
                              "Features:\n" +
                              "• Note creation and editing\n" +
                              "• Tagging and categorization\n" +
                              "• Search and organization\n" +
                              "• Export and import capabilities\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new note\n" +
                              "• Tag: Add tags to note\n" +
                              "• Export: Export notes\n" +
                              "• Import: Import notes";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}


pub mod research {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Research Tool Window
    export component ResearchToolWindow inherits Window {
        width: 800px;
        height: 600px;
        title: "Herding Cats - Research Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_research_item();
        callback cite_source();
        callback export();
        callback import();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // Research Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_research_item(); }
                            }

                            Button {
                                text: "Cite";
                                width: 50px;
                                height: 27px;
                                clicked => { root.cite_source(); }
                            }
                        }
                    }

                    // Sources Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Export";
                                width: 65px;
                                height: 27px;
                                clicked => { root.export(); }
                            }

                            Button {
                                text: "Import";
                                width: 65px;
                                height: 27px;
                                clicked => { root.import(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "📚 Research Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "📚 Research Tool\n\n" +
                              "This tool helps you manage research materials and sources.\n\n" +
                              "Features:\n" +
                              "• Source management\n" +
                              "• Citation tracking\n" +
                              "• Research material organization\n" +
                              "• Bibliography generation\n" +
                              "• Reference linking\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new research item\n" +
                              "• Cite: Add citation\n" +
                              "• Export: Export research data\n" +
                              "• Import: Import research data";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}


pub mod structure {
    slint::slint! {

    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
    import { Theme } from "../styles.slint";
    import { SlintThemeColors } from "../theme_types.slint";

    // Structure Tool Window
    export component StructureToolWindow inherits Window {
        width: 700px;
        height: 500px;
        title: "Herding Cats - Structure Tool";

        // Menu callbacks
        callback close_requested();
        callback set_theme(SlintThemeColors);
        set_theme(c) => {
            Theme.primary-bg = c.primary-bg;
            Theme.secondary-bg = c.secondary-bg;
            Theme.accent = c.accent;
            Theme.text-primary = c.text-primary;
            Theme.text-secondary = c.text-secondary;
            Theme.border = c.border;
            Theme.menu-bg = c.menu-bg;
            Theme.toolbar-bg = c.toolbar-bg;
            Theme.status-bg = c.status-bg;
            Theme.editor-bg = c.editor-bg;
            Theme.title-bg = c.title-bg;
            Theme.ribbon-bg = c.ribbon-bg;
            Theme.dropdown-bg = c.dropdown-bg;
        }
        callback new_structure();
        callback validate();
        callback export();
        callback import();

        VerticalBox {
            spacing: 0;

            // Menu Bar
            Rectangle {
                background: Theme.menu-bg;
                height: 35px;

                HorizontalBox {
                    padding: 6px;
                    spacing: 8px;

                    // Structure Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "New";
                                width: 50px;
                                height: 27px;
                                clicked => { root.new_structure(); }
                            }

                            Button {
                                text: "Validate";
                                width: 75px;
                                height: 27px;
                                clicked => { root.validate(); }
                            }
                        }
                    }

                    // Data Menu
                    Rectangle {
                        background: Theme.status-bg;
                        height: 31px;
                        border-radius: 3px;

                        HorizontalBox {
                            spacing: 4px;

                            Button {
                                text: "Export";
                                width: 65px;
                                height: 27px;
                                clicked => { root.export(); }
                            }

                            Button {
                                text: "Import";
                                width: 65px;
                                height: 27px;
                                clicked => { root.import(); }
                            }
                        }
                    }

                    Rectangle { }

                    Text {
                        text: "🏗️ Structure Tool";
                        color: Theme.text-primary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }

            // Content Area
            Rectangle {
                background: Theme.primary-bg;
                vertical-stretch: 1;
                padding: 20px;

                ScrollView {
                    width: parent.width;
                    height: parent.height;

                    TextEdit {
                        text: "🏗️ Structure Tool\n\n" +
                              "This tool helps you manage document structure and outline.\n\n" +
                              "Features:\n" +
                              "• Document outline management\n" +
                              "• Structure validation\n" +
                              "• Hierarchy checking\n" +
                              "• Consistency analysis\n" +
                              "• Organization tools\n\n" +
                              "Click toolbar buttons above to test functionality:\n" +
                              "• New: Create new structure\n" +
                              "• Validate: Validate structure\n" +
                              "• Export: Export structure data\n" +
                              "• Import: Import structure data";
                        font-size: 14px;
                        wrap: word-wrap;
                        read-only: true;
                    }
                }
            }
        }
    }

    }
}






fn hex_to_color(hex: &str) -> slint::Color {
    let hex = hex.trim_start_matches('#');
    if let Ok(val) = u32::from_str_radix(hex, 16) {
        let r = ((val >> 16) & 0xFF) as u8;
        let g = ((val >> 8) & 0xFF) as u8;
        let b = (val & 0xFF) as u8;
        slint::Color::from_rgb_u8(r, g, b)
    } else {
        slint::Color::from_rgb_u8(255, 255, 255)
    }
}

// Macro to apply theme to any window via invoke_set_theme
macro_rules! apply_theme {
    ($window:expr, $colors:expr, $mod_name:ident) => {{
        let slint_colors = $mod_name::SlintThemeColors {
            primary_bg: hex_to_color(&$colors.primary_bg),
            secondary_bg: hex_to_color(&$colors.secondary_bg),
            accent: hex_to_color(&$colors.accent),
            text_primary: hex_to_color(&$colors.text_primary),
            text_secondary: hex_to_color(&$colors.text_secondary),
            border: hex_to_color(&$colors.border),
            menu_bg: hex_to_color(&$colors.menu_bg),
            toolbar_bg: hex_to_color(&$colors.toolbar_bg),
            status_bg: hex_to_color(&$colors.status_bg),
            editor_bg: hex_to_color(&$colors.editor_bg),
            title_bg: hex_to_color(&$colors.title_bg),
            ribbon_bg: hex_to_color(&$colors.ribbon_bg),
            dropdown_bg: hex_to_color(&$colors.dropdown_bg),
        };
        $window.invoke_set_theme(slint_colors);
    }};
}

enum ToolWindowHandle {
    Hierarchy(hierarchy::HierarchyToolWindow),
    Codex(codex::CodexToolWindow),
    Brainstorming(brainstorming::BrainstormingToolWindow),
    Analysis(analysis::AnalysisToolWindow),
    Plot(plot::PlotToolWindow),
    Notes(notes::NotesToolWindow),
    Research(research::ResearchToolWindow),
    Structure(structure::StructureToolWindow),
}

impl ToolWindowHandle {
    fn hide(&self) -> Result<(), slint::PlatformError> {
        match self {
            ToolWindowHandle::Hierarchy(w) => w.hide(),
            ToolWindowHandle::Codex(w) => w.hide(),
            ToolWindowHandle::Brainstorming(w) => w.hide(),
            ToolWindowHandle::Analysis(w) => w.hide(),
            ToolWindowHandle::Plot(w) => w.hide(),
            ToolWindowHandle::Notes(w) => w.hide(),
            ToolWindowHandle::Research(w) => w.hide(),
            ToolWindowHandle::Structure(w) => w.hide(),
        }
    }

    fn show(&self) -> Result<(), slint::PlatformError> {
        match self {
            ToolWindowHandle::Hierarchy(w) => w.show(),
            ToolWindowHandle::Codex(w) => w.show(),
            ToolWindowHandle::Brainstorming(w) => w.show(),
            ToolWindowHandle::Analysis(w) => w.show(),
            ToolWindowHandle::Plot(w) => w.show(),
            ToolWindowHandle::Notes(w) => w.show(),
            ToolWindowHandle::Research(w) => w.show(),
            ToolWindowHandle::Structure(w) => w.show(),
        }
    }
    
    fn apply_theme(&self, colors: &ThemeColors) {
        match self {
            ToolWindowHandle::Hierarchy(w) => apply_theme!(w, colors, hierarchy),
            ToolWindowHandle::Codex(w) => apply_theme!(w, colors, codex),
            ToolWindowHandle::Brainstorming(w) => apply_theme!(w, colors, brainstorming),
            ToolWindowHandle::Analysis(w) => apply_theme!(w, colors, analysis),
            ToolWindowHandle::Plot(w) => apply_theme!(w, colors, plot),
            ToolWindowHandle::Notes(w) => apply_theme!(w, colors, notes),
            ToolWindowHandle::Research(w) => apply_theme!(w, colors, research),
            ToolWindowHandle::Structure(w) => apply_theme!(w, colors, structure),
        }
    }
}

thread_local! {
    static ACTIVE_TOOL_WINDOWS: RefCell<HashMap<ToolType, ToolWindowHandle>> = RefCell::new(HashMap::new());
}

/// Individual tool window manager for each writing tool
#[derive(Clone)]
pub struct IndividualToolWindowManager {
    /// Database state for all tools
    pub db_state: Arc<RwLock<DatabaseAppState>>,

    /// App state reference
    pub app_state: Arc<Mutex<AppState>>,

    /// Track open tool windows
    pub tool_windows: Arc<Mutex<std::collections::HashMap<ToolType, bool>>>,

    /// Window state tracking for individual tools
    pub window_states: Arc<Mutex<std::collections::HashMap<ToolType, ToolWindowState>>>,
}

/// Window state information for individual tool windows
#[derive(Debug, Clone)]
pub struct ToolWindowState {
    pub is_open: bool,
    pub is_focused: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub z_index: i32,
    pub window_id: u32,
}

impl IndividualToolWindowManager {
    /// Create a new individual tool window manager
    pub fn new(db_state: Arc<RwLock<DatabaseAppState>>) -> Self {
        let manager = Self {
            db_state,
            app_state: Arc::new(Mutex::new(AppState::default())),
            tool_windows: Arc::new(Mutex::new(std::collections::HashMap::new())),
            window_states: Arc::new(Mutex::new(std::collections::HashMap::new())),
        };

        // Register theme change listener
        // This ensures that when the theme changes in the main app,
        // all open tool windows are updated immediately.
        crate::ui::theme_manager::get_theme_manager().on_theme_change(|theme| {
            let colors = theme.colors.clone();
            log::info!("🎨 [IndividualToolWindowManager] Theme change detected. Scheduling update on main thread.");
            let _ = slint::invoke_from_event_loop(move || {
                ACTIVE_TOOL_WINDOWS.with(|windows| {
                    let windows_map = windows.borrow();
                    log::info!("🎨 [IndividualToolWindowManager] Updating {} active tool windows.", windows_map.len());
                    for (tool_type, window) in windows_map.iter() {
                        log::info!("🎨 [IndividualToolWindowManager] Applying theme to {:?}", tool_type);
                        window.apply_theme(&colors);
                    }
                });
            });
        });

        manager
    }

    /// Set app state reference
    pub fn set_app_state(&mut self, app_state: Arc<Mutex<AppState>>) {
        self.app_state = app_state;
    }

    /// Open a specific tool window
    pub fn open_tool_window(&self, tool_type: ToolType) -> Result<()> {
        match tool_type {
            ToolType::Hierarchy => self.open_hierarchy_window()?,
            ToolType::Codex => self.open_codex_window()?,
            ToolType::Brainstorming => self.open_brainstorming_window()?,
            ToolType::Analysis => self.open_analysis_window()?,
            ToolType::Plot => self.open_plot_window()?,
            ToolType::Notes => self.open_notes_window()?,
            ToolType::Research => self.open_research_window()?,
            ToolType::Structure => self.open_structure_window()?,
        }

        log::info!("Opened individual window for tool: {:?}", tool_type);
        Ok(())
    }

    /// Close a specific tool window
    pub fn close_tool_window(&self, tool_type: ToolType) -> Result<()> {
        // Implementation would close the specific window
        log::info!("Closed individual window for tool: {:?}", tool_type);
        Ok(())
    }

    /// Check if a tool window is open
    pub fn is_tool_window_open(&self, tool_type: ToolType) -> bool {
        let windows = self.tool_windows.lock().unwrap();
        *windows.get(&tool_type).unwrap_or(&false)
    }

    /// Get window state for a tool
    pub fn get_window_state(&self, tool_type: ToolType) -> Option<ToolWindowState> {
        let states = self.window_states.lock().unwrap();
        states.get(&tool_type).cloned()
    }

    /// Update window state
    pub fn update_window_state(&self, tool_type: ToolType, state: ToolWindowState) {
        let mut states = self.window_states.lock().unwrap();
        states.insert(tool_type, state);
    }

    /// Close all tool windows
    pub fn close_all_tool_windows(&self) -> Result<()> {
        let mut windows = self.tool_windows.lock().unwrap();
        let mut states = self.window_states.lock().unwrap();

        for tool_type in ToolType::all_types() {
            windows.insert(tool_type, false);
            if let Some(mut state) = states.get_mut(&tool_type) {
                state.is_open = false;
            }
        }

        log::info!("Closed all individual tool windows");
        Ok(())
    }

    /// Get list of open tool windows
    pub fn get_open_tool_windows(&self) -> Vec<ToolType> {
        let windows = self.tool_windows.lock().unwrap();
        ToolType::all_types()
            .into_iter()
            .filter(|&tool| *windows.get(&tool).unwrap_or(&false))
            .collect()
    }

    /// Open Hierarchy tool window
        /// Open Hierarchy tool window
    fn open_hierarchy_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Hierarchy) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Hierarchy tool
        let hierarchy_window = hierarchy::HierarchyToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&hierarchy_window, &colors, hierarchy);

        // Set up callbacks
        let window_weak = hierarchy_window.as_weak();
        hierarchy_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Hierarchy);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        hierarchy_window.on_new_item(move || {
        });

        hierarchy_window.on_delete_item(move || {
        });

        hierarchy_window.on_move_up(move || {
        });

        hierarchy_window.on_move_down(move || {
        });

                // Show the window (non-blocking)
        hierarchy_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Hierarchy, ToolWindowHandle::Hierarchy(hierarchy_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Hierarchy, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Hierarchy, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (100, 100),
            size: (800, 600),
            z_index: 0,
            window_id: 1,
        });
        Ok(())
    }

    /// Open Codex tool window
        /// Open Codex tool window
    fn open_codex_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Codex) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Codex tool
        let codex_window = codex::CodexToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&codex_window, &colors, codex);

        // Set up callbacks
        let window_weak = codex_window.as_weak();
        codex_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Codex);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        codex_window.on_new_entry(move || {
        });

        codex_window.on_search(move || {
        });

        codex_window.on_export(move || {
        });

        codex_window.on_import(move || {
        });

                // Show the window (non-blocking)
        codex_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Codex, ToolWindowHandle::Codex(codex_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Codex, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Codex, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (150, 150),
            size: (800, 600),
            z_index: 1,
            window_id: 2,
        });
        Ok(())
    }

    /// Open Brainstorming tool window
        /// Open Brainstorming tool window
    fn open_brainstorming_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Brainstorming) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Brainstorming tool
        let brainstorming_window = brainstorming::BrainstormingToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&brainstorming_window, &colors, brainstorming);

        // Set up callbacks
        let window_weak = brainstorming_window.as_weak();
        brainstorming_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Brainstorming);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        brainstorming_window.on_new_node(move || {
        });

        brainstorming_window.on_layout(move || {
        });

        brainstorming_window.on_zoom_in(move || {
        });

        brainstorming_window.on_zoom_out(move || {
        });

                // Show the window (non-blocking)
        brainstorming_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Brainstorming, ToolWindowHandle::Brainstorming(brainstorming_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Brainstorming, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Brainstorming, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (200, 200),
            size: (900, 700),
            z_index: 2,
            window_id: 3,
        });
        Ok(())
    }

    /// Open Analysis tool window
        /// Open Analysis tool window
    fn open_analysis_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Analysis) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Analysis tool
        let analysis_window = analysis::AnalysisToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&analysis_window, &colors, analysis);

        // Set up callbacks
        let window_weak = analysis_window.as_weak();
        analysis_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Analysis);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        analysis_window.on_new_analysis(move || {
        });

        analysis_window.on_generate_insights(move || {
        });

        analysis_window.on_export(move || {
        });

        analysis_window.on_import(move || {
        });

                // Show the window (non-blocking)
        analysis_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Analysis, ToolWindowHandle::Analysis(analysis_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Analysis, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Analysis, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (250, 250),
            size: (800, 600),
            z_index: 3,
            window_id: 4,
        });
        Ok(())
    }

    /// Open Plot tool window
        /// Open Plot tool window
    fn open_plot_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Plot) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Plot tool
        let plot_window = plot::PlotToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&plot_window, &colors, plot);

        // Set up callbacks
        let window_weak = plot_window.as_weak();
        plot_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Plot);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        plot_window.on_new_plot_point(move || {
        });

        plot_window.on_timeline_view(move || {
        });

        plot_window.on_export(move || {
        });

        plot_window.on_import(move || {
        });

                // Show the window (non-blocking)
        plot_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Plot, ToolWindowHandle::Plot(plot_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Plot, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Plot, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (300, 300),
            size: (800, 600),
            z_index: 4,
            window_id: 5,
        });
        Ok(())
    }

    /// Open Notes tool window
        /// Open Notes tool window
    fn open_notes_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Notes) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Notes tool
        let notes_window = notes::NotesToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&notes_window, &colors, notes);

        // Set up callbacks
        let window_weak = notes_window.as_weak();
        notes_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Notes);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        notes_window.on_new_note(move || {
        });

        notes_window.on_tag_note(move || {
        });

        notes_window.on_export(move || {
        });

        notes_window.on_import(move || {
        });

                // Show the window (non-blocking)
        notes_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Notes, ToolWindowHandle::Notes(notes_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Notes, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Notes, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (350, 350),
            size: (700, 500),
            z_index: 5,
            window_id: 6,
        });
        Ok(())
    }

    /// Open Research tool window
        /// Open Research tool window
    fn open_research_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Research) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Research tool
        let research_window = research::ResearchToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&research_window, &colors, research);

        // Set up callbacks
        let window_weak = research_window.as_weak();
        research_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Research);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        research_window.on_new_research_item(move || {
        });

        research_window.on_cite_source(move || {
        });

        research_window.on_export(move || {
        });

        research_window.on_import(move || {
        });

                // Show the window (non-blocking)
        research_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Research, ToolWindowHandle::Research(research_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Research, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Research, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (400, 400),
            size: (800, 600),
            z_index: 6,
            window_id: 7,
        });
        Ok(())
    }

    /// Open Structure tool window
        /// Open Structure tool window
    fn open_structure_window(&self) -> Result<()> {
        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow().get(&ToolType::Structure) {
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            } else {
                false
            }
        });

        if is_open {
            return Ok(());
        }

        // Create Slint window for Structure tool
        let structure_window = structure::StructureToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&structure_window, &colors, structure);

        // Set up callbacks
        let window_weak = structure_window.as_weak();
        structure_window.on_close_requested(move || {
            ACTIVE_TOOL_WINDOWS.with(|windows| {
                windows.borrow_mut().remove(&ToolType::Structure);
            });
            if let Some(window) = window_weak.upgrade() {
                window.hide().unwrap();
            }
        });

        structure_window.on_new_structure(move || {
        });

        structure_window.on_validate(move || {
        });

        structure_window.on_export(move || {
        });

        structure_window.on_import(move || {
        });

                // Show the window (non-blocking)
        structure_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {
            windows.borrow_mut().insert(ToolType::Structure, ToolWindowHandle::Structure(structure_window));
        });

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::Structure, true);

        // Update window state
        let mut states = self.window_states.lock().unwrap();
        states.insert(ToolType::Structure, ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (450, 450),
            size: (700, 500),
            z_index: 7,
            window_id: 8,
        });
        Ok(())
    }
}

// Individual tool window implementations using Slint
