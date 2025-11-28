import sys
import re

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    content = f.read()

# 1. Fix apply_theme macro definition
# Remove the extra semicolon I added: `}};` -> `}}`
# And update it to accept $theme_type
# Old: ($window:expr, $colors:expr) => {{
# New: ($window:expr, $colors:expr, $theme_type:ty) => {{
# Old: let slint_colors = SlintThemeColors {
# New: let slint_colors = <$theme_type> {

content = content.replace(
    '($window:expr, $colors:expr) => {{',
    '($window:expr, $colors:expr, $theme_type:ty) => {{'
)

content = content.replace(
    'let slint_colors = SlintThemeColors {',
    'let slint_colors = <$theme_type> {'
)

# Fix the trailing semicolon issue (replace `}};` with `}}`)
# I added `}};` in fix_build.py.
content = content.replace('}};', '}}')

# 2. Update slint! blocks
# We need to find each block and update the import and callback.
# The blocks are identified by the component name.
# HierarchyToolWindow, CodexToolWindow, etc.

tools = [
    ("Hierarchy", "HierarchyThemeColors"),
    ("Codex", "CodexThemeColors"),
    ("Brainstorming", "BrainstormingThemeColors"),
    ("Analysis", "AnalysisThemeColors"),
    ("Plot", "PlotThemeColors"),
    ("Notes", "NotesThemeColors"),
    ("Research", "ResearchThemeColors"),
    ("Structure", "StructureThemeColors"),
]

# We'll iterate through the file and replace imports/callbacks based on context?
# Or just replace all occurrences if they are unique enough?
# The import line is generic: `import { SlintThemeColors } from "../theme_types.slint";`
# We need to change it to `import { SlintThemeColors as XThemeColors } ...`
# But we need to know WHICH tool we are in.

# We can split the content by `slint::slint! {`
parts = content.split('slint::slint! {')

# parts[0] is the header (imports, macro def).
# parts[1] is the first tool (Hierarchy).
# ...
# parts[8] is the last tool.
# Wait, are they in order?
# Let's assume they are in the order of the `tools` list.
# I should verify the order.
# Hierarchy, Codex, Brainstorming, Analysis, Plot, Notes, Research, Structure.
# Let's check the file content or assume the order based on previous logs.
# Log line 1063: open_hierarchy_tool
# Log line 1079: open_codex_tool
# ...
# This matches the order.

# However, splitting by `slint::slint! {` might be risky if I have other things.
# But it's a good heuristic.

new_content = parts[0]

for i, part in enumerate(parts[1:]):
    if i < len(tools):
        tool_name, theme_type = tools[i]
        
        # Replace import
        part = part.replace(
            'import { SlintThemeColors } from "../theme_types.slint";',
            f'import {{ SlintThemeColors as {theme_type} }} from "../theme_types.slint";'
        )
        
        # Replace callback
        part = part.replace(
            'callback set_theme(SlintThemeColors);',
            f'callback set_theme({theme_type});'
        )
        
        new_content += 'slint::slint! {' + part
    else:
        # Should not happen if count matches
        new_content += 'slint::slint! {' + part

content = new_content

# 3. Update apply_theme! invocations
# They look like: `apply_theme!(w, colors)`
# We need to add the theme type.
# `ToolWindowHandle::Hierarchy(w) => apply_theme!(w, colors),`
# -> `ToolWindowHandle::Hierarchy(w) => apply_theme!(w, colors, HierarchyThemeColors),`

for tool_name, theme_type in tools:
    pattern = f'ToolWindowHandle::{tool_name}(w) => apply_theme!(w, colors)'
    replacement = f'ToolWindowHandle::{tool_name}(w) => apply_theme!(w, colors, {theme_type})'
    # Note: I might have added a semicolon in fix_build.py?
    # `apply_theme!(w, colors);`
    # Let's handle both cases.
    
    content = content.replace(pattern + ';', replacement) # If semicolon exists
    content = content.replace(pattern, replacement) # If no semicolon (or if replace above failed)

# Also update the `open_X_tool` functions?
# No, they call `apply_theme!`?
# Wait, `apply_theme!` is called in `set_theme` function which switches on `handle`.
# Ah, `apply_theme!` is ONLY called in `set_theme` function?
# Let's check the file content.
# `fn set_theme(...)` matches on `self.handle`.
# Yes.

# But wait, `open_hierarchy_tool` calls `apply_theme!` too?
# Log line 1375: `apply_theme!(&hierarchy_window, &colors);`
# Yes!
# So I need to update those calls too.
# `apply_theme!(&hierarchy_window, &colors);`
# -> `apply_theme!(&hierarchy_window, &colors, HierarchyThemeColors);`

for tool_name, theme_type in tools:
    # The variable name is usually `lowercase_tool_name_window`.
    var_name = tool_name.lower() + "_window"
    # Except `hierarchy_window` matches.
    
    pattern = f'apply_theme!(&{var_name}, &colors);'
    replacement = f'apply_theme!(&{var_name}, &colors, {theme_type});'
    content = content.replace(pattern, replacement)

with open(file_path, "w") as f:
    f.write(content)
