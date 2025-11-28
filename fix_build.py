import sys

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    lines = f.readlines()

# Fix apply_theme macro
# Look for "macro_rules! apply_theme"
macro_start = -1
for i, line in enumerate(lines):
    if "macro_rules! apply_theme" in line:
        macro_start = i
        break

if macro_start != -1:
    # Find the start of the body
    body_start = -1
    for i in range(macro_start, len(lines)):
        if "=> {" in lines[i]:
            body_start = i
            break
    
    if body_start != -1:
        lines[body_start] = lines[body_start].replace("=> {", "=> {{")
        
        # Find the end of the body
        for i in range(body_start, len(lines)):
            if "$window.invoke_set_theme(slint_colors);" in lines[i]:
                lines[i] = lines[i].replace(";", "") # Remove semicolon
            if lines[i].strip() == "};":
                lines[i] = lines[i].replace("};", "}};")
                break

# Insert SlintThemeColors in the first slint! block
slint_start = -1
for i, line in enumerate(lines):
    if "slint::slint! {" in line:
        slint_start = i
        break

if slint_start != -1:
    # Look for the import line
    import_line = -1
    for i in range(slint_start, len(lines)):
        if 'from "std-widgets.slint";' in lines[i]:
            import_line = i
            break
    
    if import_line != -1:
        # Insert after import_line
        insertion = [
            '    import { Theme } from "../styles.slint";\n',
            '    struct SlintThemeColors {\n',
            '        primary-bg: color,\n',
            '        secondary-bg: color,\n',
            '        accent: color,\n',
            '        text-primary: color,\n',
            '        text-secondary: color,\n',
            '        border: color,\n',
            '        menu-bg: color,\n',
            '        toolbar-bg: color,\n',
            '        status-bg: color,\n',
            '        editor-bg: color,\n',
            '        title-bg: color,\n',
            '        ribbon-bg: color,\n',
            '        dropdown-bg: color,\n',
            '    }\n'
        ]
        lines[import_line+1:import_line+1] = insertion

# Add import { Theme } to OTHER slint! blocks
slint_blocks = []
for i, line in enumerate(lines):
    if "slint::slint! {" in line:
        slint_blocks.append(i)

# Skip the first one (already handled)
for block_start in slint_blocks[1:]:
    # Find import line
    import_line = -1
    for i in range(block_start, len(lines)):
        if 'from "std-widgets.slint";' in lines[i]:
            import_line = i
            break
    
    if import_line != -1:
        lines[import_line+1:import_line+1] = ['    import { Theme } from "../styles.slint";\n']

with open(file_path, "w") as f:
    f.writelines(lines)
