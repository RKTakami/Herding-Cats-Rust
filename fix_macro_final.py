import sys

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    content = f.read()

# Fix 1: Remove angle brackets around $theme_type
# Current: let slint_colors = <$theme_type> {
# New: let slint_colors = $theme_type {
content = content.replace(
    'let slint_colors = <$theme_type> {',
    'let slint_colors = $theme_type {'
)

# Fix 2: Add semicolon back to invoke_set_theme
# Current: $window.invoke_set_theme(slint_colors)
# New: $window.invoke_set_theme(slint_colors);
# But we need to be careful not to double semicolon if I messed up before.
# In fix_build.py I removed the semicolon.
# In fix_conflicts.py I didn't touch this line.
# So it should be without semicolon.
content = content.replace(
    '$window.invoke_set_theme(slint_colors)',
    '$window.invoke_set_theme(slint_colors);'
)

# Fix 3: Ensure macro ends with }} (no semicolon)
# In fix_conflicts.py I replaced `}};` with `}}`.
# So it should be `}}`.
# But wait, if I add semicolon to the last statement inside the block, the block returns ().
# {{ ...; stmt; }} -> ()
# This is what we want.

with open(file_path, "w") as f:
    f.write(content)
