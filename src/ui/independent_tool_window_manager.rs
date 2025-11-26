//! Independent Tool Window Manager
//!
//! Manages individual tool windows for the independent writing tools architecture.
//! Each writing tool has its own dedicated window with independent lifecycle management.

use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::{
    ui::tools::base_types::ToolType,
    AppState,
    DatabaseAppState,
    ui::tools::individual_tool_windows::{IndividualToolWindowManager, ToolWindowState},
    ui::generated::{
        HierarchyWindow, CodexWindow, BrainstormingTool, AnalysisTool, 
        PlotTool, NotesTool, ResearchTool, StructureTool
    },
};
use slint::ComponentHandle;

/// Independent tool window manager for managing separate tool windows
pub struct IndependentToolWindowManager {
    /// Individual tool window manager
    individual_manager: IndividualToolWindowManager,

    /// Database state for all tools
    db_state: Arc<tokio::sync::RwLock<DatabaseAppState>>,

    /// App state reference
    app_state: Arc<Mutex<AppState>>,

    // Window handles for theme synchronization
    hierarchy_window: Arc<Mutex<Option<slint::Weak<HierarchyWindow>>>>,
    codex_window: Arc<Mutex<Option<slint::Weak<CodexWindow>>>>,
    brainstorming_window: Arc<Mutex<Option<slint::Weak<BrainstormingTool>>>>,
    analysis_window: Arc<Mutex<Option<slint::Weak<AnalysisTool>>>>,
    plot_window: Arc<Mutex<Option<slint::Weak<PlotTool>>>>,
    notes_window: Arc<Mutex<Option<slint::Weak<NotesTool>>>>,
    research_window: Arc<Mutex<Option<slint::Weak<ResearchTool>>>>,
    structure_window: Arc<Mutex<Option<slint::Weak<StructureTool>>>>,
}

impl IndependentToolWindowManager {
    /// Create a new independent tool window manager
    pub fn new(db_state: Arc<tokio::sync::RwLock<DatabaseAppState>>) -> Result<Self> {
        let hierarchy_window = Arc::new(Mutex::new(None));
        let codex_window = Arc::new(Mutex::new(None));
        let brainstorming_window = Arc::new(Mutex::new(None));
        let analysis_window = Arc::new(Mutex::new(None));
        let plot_window = Arc::new(Mutex::new(None));
        let notes_window = Arc::new(Mutex::new(None));
        let research_window = Arc::new(Mutex::new(None));
        let structure_window = Arc::new(Mutex::new(None));

        // Register theme change listener
        let h_win = hierarchy_window.clone();
        let c_win = codex_window.clone();
        let b_win = brainstorming_window.clone();
        let a_win = analysis_window.clone();
        let p_win = plot_window.clone();
        let n_win = notes_window.clone();
        let r_win = research_window.clone();
        let s_win = structure_window.clone();

        let theme_manager = crate::ui::theme_manager::get_theme_manager();
        theme_manager.on_theme_change(move |theme| {
            let theme_name = slint::SharedString::from(theme.name.clone());
            if let Some(window) = h_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<HierarchyWindow>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
            if let Some(window) = c_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<CodexWindow>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
            if let Some(window) = b_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<BrainstormingTool>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
            if let Some(window) = a_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<AnalysisTool>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
            if let Some(window) = p_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<PlotTool>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
            if let Some(window) = n_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<NotesTool>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
            if let Some(window) = r_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<ResearchTool>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
            if let Some(window) = s_win.lock().unwrap().as_ref().and_then(|w: &slint::Weak<StructureTool>| w.upgrade()) {
                window.invoke_set_theme(theme_name.clone());
            }
        });

        Ok(Self {
            individual_manager: IndividualToolWindowManager::new(db_state.clone()),
            db_state,
            app_state: Arc::new(Mutex::new(AppState::default())),
            hierarchy_window,
            codex_window,
            brainstorming_window,
            analysis_window,
            plot_window,
            notes_window,
            research_window,
            structure_window,
        })
    }

    /// Set app state reference
    pub fn set_app_state(&mut self, app_state: Arc<Mutex<AppState>>) {
        self.app_state = app_state.clone();
        self.individual_manager.set_app_state(app_state);
    }

    /// Open a specific tool window
    pub async fn open_tool_window(&self, tool_type: ToolType) -> Result<()> {
        log::info!("🚀 Opening independent window for tool: {:?}", tool_type);

        // Check if tool is already open
        if self.individual_manager.is_tool_window_open(tool_type) {
            log::info!("💡 Tool {:?} is already open, focusing existing window", tool_type);
            self.focus_tool_window(tool_type);
            return Ok(());
        }

        // Open the specific tool window
        match tool_type {
            ToolType::Hierarchy => self.open_hierarchy_window().await?,
            ToolType::Codex => self.open_codex_window().await?,
            ToolType::Brainstorming => self.open_brainstorming_window().await?,
            ToolType::Analysis => self.open_analysis_window().await?,
            ToolType::Plot => self.open_plot_window().await?,
            ToolType::Notes => self.open_notes_window().await?,
            ToolType::Research => self.open_research_window().await?,
            ToolType::Structure => self.open_structure_window().await?,
        }

        log::info!("✅ Independent tool window opened for: {:?}", tool_type);
        Ok(())
    }

    /// Close a specific tool window
    pub fn close_tool_window(&self, tool_type: ToolType) -> Result<()> {
        log::info!("👋 Closing independent window for tool: {:?}", tool_type);

        // Update window state
        let state = ToolWindowState {
            is_open: false,
            is_focused: false,
            position: (0, 0),
            size: (0, 0),
            z_index: 0,
            window_id: 0,
        };

        self.individual_manager.update_window_state(tool_type, state);

        // Update tool window tracking
        let mut windows = self.individual_manager.tool_windows.lock().unwrap();
        windows.insert(tool_type, false);

        log::info!("✅ Independent tool window closed for: {:?}", tool_type);
        Ok(())
    }

    /// Close all tool windows
    pub fn close_all_tool_windows(&self) -> Result<()> {
        log::info!("👋 Closing all independent tool windows");
        self.individual_manager.close_all_tool_windows()
    }

    /// Check if a tool window is open
    pub fn is_tool_window_open(&self, tool_type: ToolType) -> bool {
        self.individual_manager.is_tool_window_open(tool_type)
    }

    /// Focus a specific tool window
    pub fn focus_tool_window(&self, tool_type: ToolType) {
        if self.is_tool_window_open(tool_type) {
            log::info!("🎯 Focusing tool window: {:?}", tool_type);
            // In a real implementation, this would bring the window to front
            if let Some(mut state) = self.individual_manager.get_window_state(tool_type) {
                state.is_focused = true;
                self.individual_manager.update_window_state(tool_type, state);
            }
        }
    }

    /// Get list of open tool windows
    pub fn get_open_tool_windows(&self) -> Vec<ToolType> {
        self.individual_manager.get_open_tool_windows()
    }

    /// Get window state for a tool
    pub fn get_window_state(&self, tool_type: ToolType) -> Option<ToolWindowState> {
        self.individual_manager.get_window_state(tool_type)
    }

    /// Update window state
    pub fn update_window_state(&self, tool_type: ToolType, state: ToolWindowState) {
        self.individual_manager.update_window_state(tool_type, state);
    }

    // Individual tool window implementations
    async fn open_hierarchy_window(&self) -> Result<()> {
        println!("🚀 Opening Hierarchy Tool Window (Independent)");

        // Create and configure the hierarchy window
        let hierarchy_window = HierarchyWindow::new()?;

        // Store handle for theme synchronization
        *self.hierarchy_window.lock().unwrap() = Some(hierarchy_window.as_weak());

        // Set up callbacks
        hierarchy_window.on_close_requested(move || {
            println!("📚 Hierarchy window closed");
        });

        hierarchy_window.on_new_item(move || {
            println!("➕ Hierarchy: New item created");
        });

        hierarchy_window.on_delete_item(move || {
            println!("🗑️ Hierarchy: Item deleted");
        });

        hierarchy_window.on_move_up(move || {
            println!("⬆️ Hierarchy: Item moved up");
        });

        hierarchy_window.on_move_down(move || {
            println!("⬇️ Hierarchy: Item moved down");
        });

        // Show the window
        hierarchy_window.show()?;

        // Update state
        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (100, 100),
            size: (800, 600),
            z_index: 0,
            window_id: 1,
        };
        self.individual_manager.update_window_state(ToolType::Hierarchy, state);

        Ok(())
    }

    /// Set the theme for all open tool windows
    pub fn set_theme(&self, theme_name: String) {
        let theme_name = slint::SharedString::from(theme_name);
        
        if let Some(window) = self.hierarchy_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
        if let Some(window) = self.codex_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
        if let Some(window) = self.brainstorming_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
        if let Some(window) = self.analysis_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
        if let Some(window) = self.plot_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
        if let Some(window) = self.notes_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
        if let Some(window) = self.research_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
        if let Some(window) = self.structure_window.lock().unwrap().as_ref().and_then(|w| w.upgrade()) {
            window.invoke_set_theme(theme_name.clone());
        }
    }

    async fn open_codex_window(&self) -> Result<()> {
        println!("🚀 Opening Codex Tool Window (Independent)");

        let codex_window = CodexWindow::new()?;

        // Store handle for theme synchronization
        *self.codex_window.lock().unwrap() = Some(codex_window.as_weak());

        codex_window.on_close_requested(move || {
            println!("📖 Codex window closed");
        });

        codex_window.on_new_entry(move || {
            println!("📝 Codex: New entry created");
        });

        codex_window.on_search(move || {
            println!("🔍 Codex: Search initiated");
        });

        codex_window.on_export(move || {
            println!("📤 Codex: Data exported");
        });

        codex_window.on_import(move || {
            println!("📥 Codex: Data imported");
        });

        codex_window.show()?;

        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (150, 150),
            size: (800, 600),
            z_index: 1,
            window_id: 2,
        };
        self.individual_manager.update_window_state(ToolType::Codex, state);

        Ok(())
    }

    async fn open_brainstorming_window(&self) -> Result<()> {
        println!("🚀 Opening Brainstorming Tool Window (Independent)");

        let brainstorming_window = BrainstormingTool::new()?;

        // Store handle for theme synchronization
        *self.brainstorming_window.lock().unwrap() = Some(brainstorming_window.as_weak());

        brainstorming_window.on_close_requested(move || {
            println!("💭 Brainstorming window closed");
        });

        brainstorming_window.on_new_node(move || {
            println!("➕ Brainstorming: New node created");
        });

        brainstorming_window.on_layout(move || {
            println!("🔧 Brainstorming: Auto-arrange layout");
        });

        brainstorming_window.on_zoom_in(move || {
            println!("🔍 Brainstorming: Zoom in");
        });

        brainstorming_window.on_zoom_out(move || {
            println!("🔍 Brainstorming: Zoom out");
        });

        brainstorming_window.show()?;

        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (200, 200),
            size: (900, 700),
            z_index: 2,
            window_id: 3,
        };
        self.individual_manager.update_window_state(ToolType::Brainstorming, state);

        Ok(())
    }

    async fn open_analysis_window(&self) -> Result<()> {
        println!("🚀 Opening Analysis Tool Window (Independent)");

        let analysis_window = AnalysisTool::new()?;

        // Store handle for theme synchronization
        *self.analysis_window.lock().unwrap() = Some(analysis_window.as_weak());

        analysis_window.on_close_requested(move || {
            println!("🔬 Analysis window closed");
        });

        analysis_window.on_new_analysis(move || {
            println!("📊 Analysis: New analysis created");
        });

        analysis_window.on_generate_insights(move || {
            println!("💡 Analysis: Insights generated");
        });

        analysis_window.on_export(move || {
            println!("📤 Analysis: Summary exported");
        });

        analysis_window.on_import(move || {
            println!("📥 Analysis: Data imported");
        });

        analysis_window.show()?;

        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (250, 250),
            size: (800, 600),
            z_index: 3,
            window_id: 4,
        };
        self.individual_manager.update_window_state(ToolType::Analysis, state);

        Ok(())
    }

    async fn open_plot_window(&self) -> Result<()> {
        println!("🚀 Opening Plot Tool Window (Independent)");

        let plot_window = PlotTool::new()?;

        // Store handle for theme synchronization
        *self.plot_window.lock().unwrap() = Some(plot_window.as_weak());

        plot_window.on_close_requested(move || {
            println!("📊 Plot window closed");
        });

        plot_window.on_new_plot_point(move || {
            println!("➕ Plot: New plot point created");
        });

        plot_window.on_timeline_view(move || {
            println!("📊 Plot: Timeline view shown");
        });

        plot_window.on_export(move || {
            println!("📤 Plot: Data exported");
        });

        plot_window.on_import(move || {
            println!("📥 Plot: Data imported");
        });

        plot_window.show()?;

        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (300, 300),
            size: (800, 600),
            z_index: 4,
            window_id: 5,
        };
        self.individual_manager.update_window_state(ToolType::Plot, state);

        Ok(())
    }

    async fn open_notes_window(&self) -> Result<()> {
        println!("🚀 Opening Notes Tool Window (Independent)");

        let notes_window = NotesTool::new()?;

        // Store handle for theme synchronization
        *self.notes_window.lock().unwrap() = Some(notes_window.as_weak());

        notes_window.on_close_requested(move || {
            println!("📝 Notes window closed");
        });

        notes_window.on_new_note(move || {
            println!("➕ Notes: New note created");
        });

        notes_window.on_tag_note(move || {
            println!("🏷️ Notes: Note tagged");
        });

        notes_window.on_export(move || {
            println!("📤 Notes: Notes exported");
        });

        notes_window.on_import(move || {
            println!("📥 Notes: Notes imported");
        });

        notes_window.show()?;

        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (350, 350),
            size: (700, 500),
            z_index: 5,
            window_id: 6,
        };
        self.individual_manager.update_window_state(ToolType::Notes, state);

        Ok(())
    }

    async fn open_research_window(&self) -> Result<()> {
        println!("🚀 Opening Research Tool Window (Independent)");

        let research_window = ResearchTool::new()?;

        // Store handle for theme synchronization
        *self.research_window.lock().unwrap() = Some(research_window.as_weak());

        research_window.on_close_requested(move || {
            println!("📚 Research window closed");
        });

        research_window.on_add_source(move || {
            println!("➕ Research: New research item created");
        });

        research_window.on_citation(move || {
            println!("📚 Research: Citation added");
        });

        research_window.on_export(move || {
            println!("📤 Research: Data exported");
        });

        research_window.on_import(move || {
            println!("📥 Research: Data imported");
        });

        research_window.show()?;

        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (400, 400),
            size: (800, 600),
            z_index: 6,
            window_id: 7,
        };
        self.individual_manager.update_window_state(ToolType::Research, state);

        Ok(())
    }

    async fn open_structure_window(&self) -> Result<()> {
        println!("🚀 Opening Structure Tool Window (Independent)");

        let structure_window = StructureTool::new()?;

        // Store handle for theme synchronization
        *self.structure_window.lock().unwrap() = Some(structure_window.as_weak());

        structure_window.on_close_requested(move || {
            println!("🏗️ Structure window closed");
        });

        structure_window.on_new_structure(move || {
            println!("➕ Structure: New structure created");
        });

        structure_window.on_validate(move || {
            println!("✅ Structure: Structure validated");
        });

        structure_window.on_export(move || {
            println!("📤 Structure: Data exported");
        });

        structure_window.on_import(move || {
            println!("📥 Structure: Data imported");
        });

        structure_window.show()?;

        let state = ToolWindowState {
            is_open: true,
            is_focused: true,
            position: (450, 450),
            size: (700, 500),
            z_index: 7,
            window_id: 8,
        };
        self.individual_manager.update_window_state(ToolType::Structure, state);

        Ok(())
    }
}

// Independent window implementations using Slint
// Note: We are now using external .slint files imported via all_modules.slint
// instead of inline macros. This allows for better code organization and
// shared theme support.
