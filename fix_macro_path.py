import sys

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    content = f.read()

# Change $theme_type:ty to $theme_type:path
content = content.replace(
    '($window:expr, $colors:expr, $theme_type:ty) => {{',
    '($window:expr, $colors:expr, $theme_type:path) => {{'
)

with open(file_path, "w") as f:
    f.write(content)
