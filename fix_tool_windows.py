import re
import os

file_path = 'src/ui/tools/individual_tool_windows.rs'

with open(file_path, 'r') as f:
    content = f.read()

# 1. Extract and Move Slint Macro
# Find the slint! macro block
# We'll assume it starts with `slint::slint! {` and ends at the end of the file.
start_idx = content.find('slint::slint! {')
if start_idx == -1:
    print("Could not find slint! macro")
    exit(1)

slint_content = content[start_idx:]
content_without_macro = content[:start_idx]

# 2. Add Imports
imports_to_add = """use std::cell::RefCell;
use std::collections::HashMap;
use slint::ComponentHandle;
use crate::ui::theme_manager::{get_current_theme_colors, ThemeColors};"""

if "use slint::ComponentHandle;" not in content_without_macro:
    content_without_macro = content_without_macro.replace("use tokio::sync::RwLock;", "use tokio::sync::RwLock;\n" + imports_to_add)

# 3. Inject Slint Modifications (Struct and Callbacks)
# Inject SlintThemeColors struct
slint_struct = """
    export struct SlintThemeColors {
        primary-bg: color,
        secondary-bg: color,
        accent: color,
        text-primary: color,
        text-secondary: color,
        border: color,
        menu-bg: color,
        toolbar-bg: color,
        status-bg: color,
        editor-bg: color,
        title-bg: color,
        ribbon-bg: color,
        dropdown-bg: color,
    }
"""

slint_content = slint_content.replace(
    'import { Theme } from "../styles.slint";',
    'import { Theme } from "../styles.slint";' + slint_struct
)

# Inject set_theme callback and handler in each component
callback_code = """
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
"""
slint_content = slint_content.replace('callback close_requested();', callback_code.strip())

# Fix colors in Slint content
slint_content = slint_content.replace('background: #2d3748;', 'background: Theme.menu-bg;')
slint_content = slint_content.replace('background: #4a5568;', 'background: Theme.status-bg;')
slint_content = slint_content.replace('background: #ffffff;', 'background: Theme.primary-bg;')
slint_content = slint_content.replace('color: white;', 'color: Theme.text-primary;')


# 4. Add Enum, thread_local, and Theme Helpers (AFTER Slint macro)
# We place the Slint macro AFTER imports, but BEFORE the Rust code that uses it.
# Actually, we can place it right after imports.

helper_code = """
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
    ($window:expr, $colors:expr) => {
        let slint_colors = SlintThemeColors {
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
    };
}

enum ToolWindowHandle {
    Hierarchy(HierarchyToolWindow),
    Codex(CodexToolWindow),
    Brainstorming(BrainstormingToolWindow),
    Analysis(AnalysisToolWindow),
    Plot(PlotToolWindow),
    Notes(NotesToolWindow),
    Research(ResearchToolWindow),
    Structure(StructureToolWindow),
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
            ToolWindowHandle::Hierarchy(w) => apply_theme!(w, colors),
            ToolWindowHandle::Codex(w) => apply_theme!(w, colors),
            ToolWindowHandle::Brainstorming(w) => apply_theme!(w, colors),
            ToolWindowHandle::Analysis(w) => apply_theme!(w, colors),
            ToolWindowHandle::Plot(w) => apply_theme!(w, colors),
            ToolWindowHandle::Notes(w) => apply_theme!(w, colors),
            ToolWindowHandle::Research(w) => apply_theme!(w, colors),
            ToolWindowHandle::Structure(w) => apply_theme!(w, colors),
        }
    }
}

thread_local! {
    static ACTIVE_TOOL_WINDOWS: RefCell<HashMap<ToolType, ToolWindowHandle>> = RefCell::new(HashMap::new());
}
"""

if "static ACTIVE_TOOL_WINDOWS" in content_without_macro:
    pass
else:
    content_without_macro = content_without_macro.replace("/// Individual tool window manager for each writing tool", helper_code + "\n/// Individual tool window manager for each writing tool")


# 5. Update open_*_window methods (in content_without_macro)
tools = [
    ("Hierarchy", "hierarchy", "Hierarchy"),
    ("Codex", "codex", "Codex"),
    ("Brainstorming", "brainstorming", "Brainstorming"),
    ("Analysis", "analysis", "Analysis"),
    ("Plot", "plot", "Plot"),
    ("Notes", "notes", "Notes"),
    ("Research", "research", "Research"),
    ("Structure", "structure", "Structure")
]

for tool_name, tool_var, tool_type in tools:
    regex_pattern1 = (
        fr'fn open_{tool_var}_window\(&self\) -> Result<\(\)> {{\s+'
        fr'println!\("🚀 Opening {tool_name} Tool Window"\);\s+'
        fr'// Create Slint window for {tool_name} tool\s+'
        fr'let {tool_var}_window = {tool_name}ToolWindow::new\(\)\?;\s+'
        fr'// Set up callbacks\s+'
        fr'{tool_var}_window\.on_close_requested\(move \|\| {{\s+'
        fr'// Simple close action for now\s+'
        fr'}}\);'
    )
    
    replacement1 = f"""    /// Open {tool_name} tool window
    fn open_{tool_var}_window(&self) -> Result<()> {{
        println!("🚀 Opening {tool_name} Tool Window");

        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {{
            if let Some(window) = windows.borrow().get(&ToolType::{tool_type}) {{
                println!("⚠️ {tool_name} tool window already open - showing it");
                let _ = window.show();
                // Also update theme on existing window
                window.apply_theme(&colors);
                true
            }} else {{
                false
            }}
        }});

        if is_open {{
            return Ok(());
        }}

        // Create Slint window for {tool_name} tool
        let {tool_var}_window = {tool_name}ToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&{tool_var}_window, &colors);

        // Set up callbacks
        let window_weak = {tool_var}_window.as_weak();
        {tool_var}_window.on_close_requested(move || {{
            ACTIVE_TOOL_WINDOWS.with(|windows| {{
                windows.borrow_mut().remove(&ToolType::{tool_type});
            }});
            if let Some(window) = window_weak.upgrade() {{
                window.hide().unwrap();
            }}
        }});"""
    
    if re.search(regex_pattern1, content_without_macro):
        content_without_macro = re.sub(regex_pattern1, replacement1, content_without_macro, count=1)

    regex_pattern2 = (
        fr'// Show the window\s+'
        fr'{tool_var}_window\.run\(\)\?;\s+'
        fr'// Store weak reference\s+'
        fr'let mut windows = self\.tool_windows\.lock\(\)\.unwrap\(\);\s+'
        fr'windows\.insert\(ToolType::{tool_type}, true\);'
    )
    
    replacement2 = f"""        // Show the window (non-blocking)
        {tool_var}_window.show()?;

        // Store handle to keep it alive
        ACTIVE_TOOL_WINDOWS.with(|windows| {{
            windows.borrow_mut().insert(ToolType::{tool_type}, ToolWindowHandle::{tool_name}({tool_var}_window));
        }});

        // Store weak reference
        let mut windows = self.tool_windows.lock().unwrap();
        windows.insert(ToolType::{tool_type}, true);"""

    if re.search(regex_pattern2, content_without_macro):
        content_without_macro = re.sub(regex_pattern2, replacement2, content_without_macro, count=1)


# 6. Assemble Final Content
# Insert Slint macro after imports
# We look for the last import line
last_import_idx = content_without_macro.rfind('use ')
# Find the end of that line
end_of_import_line = content_without_macro.find(';', last_import_idx) + 1
# Insert slint macro there
final_content = content_without_macro[:end_of_import_line] + "\n\n" + slint_content + "\n\n" + content_without_macro[end_of_import_line:]

with open(file_path, 'w') as f:
    f.write(final_content)

print("Done!")
