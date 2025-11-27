import sys

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    lines = f.readlines()

# Find the start and end of the macro
macro_start = -1
macro_end = -1

for i, line in enumerate(lines):
    if "macro_rules! apply_theme" in line:
        macro_start = i
    if macro_start != -1 and line.strip() == "}" and i > macro_start:
        macro_end = i
        break

if macro_start != -1 and macro_end != -1:
    # Construct the correct macro definition
    new_macro = [
        "macro_rules! apply_theme {\n",
        "    ($window:expr, $colors:expr, $theme_type:path) => {{\n",
        "        let slint_colors = $theme_type {\n",
        "            primary_bg: hex_to_color(&$colors.primary_bg),\n",
        "            secondary_bg: hex_to_color(&$colors.secondary_bg),\n",
        "            accent: hex_to_color(&$colors.accent),\n",
        "            text_primary: hex_to_color(&$colors.text_primary),\n",
        "            text_secondary: hex_to_color(&$colors.text_secondary),\n",
        "            border: hex_to_color(&$colors.border),\n",
        "            menu_bg: hex_to_color(&$colors.menu_bg),\n",
        "            toolbar_bg: hex_to_color(&$colors.toolbar_bg),\n",
        "            status_bg: hex_to_color(&$colors.status_bg),\n",
        "            editor_bg: hex_to_color(&$colors.editor_bg),\n",
        "            title_bg: hex_to_color(&$colors.title_bg),\n",
        "            ribbon_bg: hex_to_color(&$colors.ribbon_bg),\n",
        "            dropdown_bg: hex_to_color(&$colors.dropdown_bg),\n",
        "        };\n",
        "        $window.invoke_set_theme(slint_colors);\n",
        "    }};\n",
        "}\n"
    ]
    
    # Replace the lines
    lines[macro_start:macro_end+1] = new_macro

    with open(file_path, "w") as f:
        f.writelines(lines)
