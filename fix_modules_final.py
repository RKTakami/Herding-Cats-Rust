import sys
import re

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    content = f.read()

# 1. Update macro definition
# From: ($window:expr, $colors:expr, $theme_type:path) => {{
# To:   ($window:expr, $colors:expr, $mod_name:ident) => {{
content = content.replace(
    '($window:expr, $colors:expr, $theme_type:path) => {{',
    '($window:expr, $colors:expr, $mod_name:ident) => {{'
)

# Update struct instantiation in macro
# From: let slint_colors = $theme_type {
# To:   let slint_colors = $mod_name::SlintThemeColors {
content = content.replace(
    'let slint_colors = $theme_type {',
    'let slint_colors = $mod_name::SlintThemeColors {'
)

# 2. Update apply_theme! calls
# From: apply_theme!(w, colors, hierarchy::SlintThemeColors)
# To:   apply_theme!(w, colors, hierarchy)
tools = [
    ("Hierarchy", "hierarchy"),
    ("Codex", "codex"),
    ("Brainstorming", "brainstorming"),
    ("Analysis", "analysis"),
    ("Plot", "plot"),
    ("Notes", "notes"),
    ("Research", "research"),
    ("Structure", "structure"),
]

for tool_name, mod_name in tools:
    # Regex for the call in match arm
    # `apply_theme!(w, colors, hierarchy::SlintThemeColors)`
    pattern = f'apply_theme!\(w, colors, {mod_name}::SlintThemeColors\)'
    replacement = f'apply_theme!(w, colors, {mod_name})'
    content = re.sub(pattern, replacement, content)
    
    # Regex for the call in open_X_tool (if any)
    # `apply_theme!(&hierarchy_window, &colors, hierarchy::SlintThemeColors);`
    var_name = tool_name.lower() + "_window"
    pattern2 = f'apply_theme!\(&{var_name}, &colors, {mod_name}::SlintThemeColors\);'
    replacement2 = f'apply_theme!(&{var_name}, &colors, {mod_name});'
    content = re.sub(pattern2, replacement2, content)

# 3. Update window instantiation
# From: HierarchyToolWindow::new()
# To:   hierarchy::HierarchyToolWindow::new()
for tool_name, mod_name in tools:
    content = content.replace(
        f'{tool_name}ToolWindow::new()',
        f'{mod_name}::{tool_name}ToolWindow::new()'
    )

with open(file_path, "w") as f:
    f.write(content)
