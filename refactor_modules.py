import sys
import re

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    content = f.read()

# 1. Revert macro to use `path`
content = content.replace(
    '($window:expr, $colors:expr, $theme_type:ident) => {{',
    '($window:expr, $colors:expr, $theme_type:path) => {{'
)

# 2. Define tools and their module names
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

# 3. Wrap slint! blocks in modules and revert renaming
# We split by `slint::slint! {` again.
parts = content.split('slint::slint! {')

new_content = parts[0]

# We need to be careful. The first part is the header.
# The subsequent parts are the blocks.
# But wait, `split` removes the separator.
# And I need to reconstruct it.

for i, part in enumerate(parts[1:]):
    if i < len(tools):
        tool_name, mod_name = tools[i]
        
        # Revert import renaming
        # `import { SlintThemeColors as HierarchyThemeColors }` -> `import { SlintThemeColors }`
        part = re.sub(
            r'import \{ SlintThemeColors as \w+ \}',
            'import { SlintThemeColors }',
            part
        )
        
        # Revert callback type
        # `callback set_theme(HierarchyThemeColors);` -> `callback set_theme(SlintThemeColors);`
        part = re.sub(
            r'callback set_theme\(\w+\);',
            'callback set_theme(SlintThemeColors);',
            part
        )
        
        # Wrap in module
        # The part ends with `}` (end of slint! block) and then some newlines/code?
        # Actually `slint!` block is usually followed by `enum ToolWindowHandle` or next block.
        # But `split` splits the whole file.
        # So `part` contains the rest of the file until the next `slint::slint! {`.
        # Wait, `slint!` block ends with `}`.
        # I need to find the closing `}` of the `slint!` block.
        # This is tricky with regex/split.
        
        # Alternative: The `slint!` block is the *start* of `part`.
        # It ends at the last `}` before the next thing?
        # No, `slint!` syntax is `slint::slint! { ... }`.
        # Inside `...` braces are balanced.
        
        # Let's assume the `slint!` block is the first balanced brace group in `part`.
        # But `part` starts with the content *inside* the braces?
        # No, `split` removed `slint::slint! {`.
        # So `part` starts with the content.
        # I need to find the matching closing brace `}`.
        
        # Simple heuristic: The `slint!` block contains `export component ...`.
        # It ends after that component definition.
        # But `export component` has braces too.
        
        # Let's count braces.
        depth = 1 # We start inside the first `{` (removed by split)
        end_index = -1
        for j, char in enumerate(part):
            if char == '{':
                depth += 1
            elif char == '}':
                depth -= 1
                if depth == 0:
                    end_index = j
                    break
        
        if end_index != -1:
            slint_content = part[:end_index] # Content inside `slint! { ... }`
            rest = part[end_index+1:] # Content after `}`
            
            # Construct the module
            # pub mod hierarchy {
            #     slint::slint! {
            #         ...
            #     }
            # }
            
            # Note: `slint::slint!` is needed inside.
            block = f'pub mod {mod_name} {{\n    slint::slint! {{\n{slint_content}\n    }}\n}}\n'
            
            new_content += block + rest
        else:
            # Fallback if brace counting fails (shouldn't happen if valid)
            print(f"Error: Could not find closing brace for {tool_name}")
            new_content += 'slint::slint! {' + part
            
    else:
        # Should not happen
        new_content += 'slint::slint! {' + part

content = new_content

# 4. Update ToolWindowHandle enum
# `Hierarchy(HierarchyToolWindow)` -> `Hierarchy(hierarchy::HierarchyToolWindow)`
for tool_name, mod_name in tools:
    content = content.replace(
        f'{tool_name}({tool_name}ToolWindow)',
        f'{tool_name}({mod_name}::{tool_name}ToolWindow)'
    )

# 5. Update apply_theme! calls
# `apply_theme!(w, colors, HierarchyThemeColors)` -> `apply_theme!(w, colors, hierarchy::SlintThemeColors)`
for tool_name, mod_name in tools:
    # Note: I need to match the previous replacement pattern.
    # In fix_conflicts.py I used `HierarchyThemeColors`.
    # Now I want `hierarchy::SlintThemeColors`.
    
    # Regex to catch potential variations
    # Regex to catch 2-arg calls in match arms with context
    # `ToolWindowHandle::Hierarchy(w) => apply_theme!(w, colors)`
    pattern = f'ToolWindowHandle::{tool_name}\(w\) => apply_theme!\(w, colors\)'
    replacement = f'ToolWindowHandle::{tool_name}(w) => apply_theme!(w, colors, {mod_name}::SlintThemeColors)'
    content = re.sub(pattern, replacement, content)
    
    # Also handle the `&hierarchy_window` calls
    var_name = tool_name.lower() + "_window"
    pattern2 = f'apply_theme!\(&{var_name}, &colors\);'
    replacement2 = f'apply_theme!(&{var_name}, &colors, {mod_name}::SlintThemeColors);'
    content = re.sub(pattern2, replacement2, content)

with open(file_path, "w") as f:
    f.write(content)
