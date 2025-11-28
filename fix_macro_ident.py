import sys

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    content = f.read()

# Change $theme_type:path to $theme_type:ident
content = content.replace(
    '($window:expr, $colors:expr, $theme_type:path) => {{',
    '($window:expr, $colors:expr, $theme_type:ident) => {{'
)

with open(file_path, "w") as f:
    f.write(content)
