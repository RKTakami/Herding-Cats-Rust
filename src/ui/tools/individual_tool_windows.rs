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

    import {
    Button,
    TextEdit,
    ScrollView,
    HorizontalBox,
    VerticalBox,
} from "std-widgets.slint";
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
                            clicked => {
                                root.new_item();
                            }
                        }

                        Button {
                            text: "Delete";
                            width: 60px;
                            height: 27px;
                            clicked => {
                                root.delete_item();
                            }
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
                            clicked => {
                                root.move_up();
                            }
                        }

                        Button {
                            text: "Down";
                            width: 60px;
                            height: 27px;
                            clicked => {
                                root.move_down();
                            }
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
                    text: "📚 Hierarchy Tool\n\n" + "This tool helps you organize your manuscript structure.\n\n" + "Features:\n" + "• Chapter and scene management\n" + "• Drag-and-drop reordering\n" + "• Word count tracking\n" + "• Structure visualization\n\n" + "Click toolbar buttons above to test functionality:\n" + "• New: Create new hierarchy item\n" + "• Delete: Remove selected item\n" + "• Up: Move item up in hierarchy\n" + "• Down: Move item down in hierarchy";
                    font-size: 14px;
                    wrap: word-wrap;
                    read-only: true;
                }
            }
        }
    }
}
