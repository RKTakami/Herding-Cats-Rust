import re

file_path = 'src/ui/writing_tools.slint'

with open(file_path, 'r') as f:
    content = f.read()

# Fix HierarchyTool ListView
# Match ListView block with model and delegate
hierarchy_pattern = r'ListView\s*\{\s*height: 400px;\s*model: (\[[\s\S]*?\]);\s*delegate := (VerticalBox\s*\{[\s\S]*?\})\s*\}'
# We need to capture the model array and the delegate body (VerticalBox { ... })
# But regex matching nested braces is hard.

# Alternative: Replace specific lines.
# "model: [" -> "for item in ["
# "];" -> "]:"
# "delegate := VerticalBox {" -> "VerticalBox {"
# But we need to be careful not to break other things.

# Let's try to construct the replacement string for HierarchyTool manually since the data is static.
hierarchy_replacement = """ListView {
            height: 400px;
            for item in [
                { text: "Chapter 1: Introduction" },
                { text: "Chapter 2: Background" },
                { text: "Chapter 3: Main Story" },
                { text: "Chapter 4: Climax" },
                { text: "Chapter 5: Resolution" },
            ] : VerticalBox {
                padding: 5px;
                Rectangle {
                    height: 30px;
                    background: Theme.secondary-bg;
                    border-radius: 4px;
                    Text {
                        text: item.text;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }
        }"""

# Find the original block. It starts with "ListView {" and contains "Chapter 1: Introduction".
# We can use a simpler regex to find the start and end.
hierarchy_search = r'ListView\s*\{\s*height: 400px;\s*model:[\s\S]*?delegate := VerticalBox\s*\{[\s\S]*?\}\s*\}\s*\}'
# This is risky.

# Let's use the exact content from previous view_file if possible.
# Lines 95-117.
original_hierarchy = """        ListView {
            height: 400px;
            model: [
                { text: "Chapter 1: Introduction" },
                { text: "Chapter 2: Background" },
                { text: "Chapter 3: Main Story" },
                { text: "Chapter 4: Climax" },
                { text: "Chapter 5: Resolution" },
            ];
            delegate := VerticalBox {
                padding: 5px;
                Rectangle {
                    height: 30px;
                    background: Theme.secondary-bg;
                    border-radius: 4px;
                    Text {
                        text: root.current_item.text;
                        font-size: 12px;
                        vertical-alignment: center;
                    }
                }
            }
        }"""

# Note: "root.current_item.text" needs to be changed to "item.text"
content = content.replace(original_hierarchy, hierarchy_replacement)


# Fix StructureTool ListView
# This one is larger.
# We will use regex to replace "model: [" with "for item in ["
# And "];" with "]:"
# And "delegate := Rectangle {" with "Rectangle {"
# And "root.current_item" with "item"
# And remove "margin: 4px;"

# We need to target the specific ListView in StructureTool.
# It starts with "ListView {" and contains "1. Ordinary World".

# Let's read the file line by line and process this block.
lines = content.split('\n')
new_lines = []
in_structure_listview = False
for line in lines:
    if 'title: "1. Ordinary World"' in line:
        in_structure_listview = True
    
    if in_structure_listview:
        if 'model: [' in line:
            line = line.replace('model: [', 'for item in [')
        if '];' in line:
            line = line.replace('];', ']:')
        if 'delegate := Rectangle {' in line:
            line = line.replace('delegate := Rectangle {', 'Rectangle {')
        if 'root.current_item' in line:
            line = line.replace('root.current_item', 'item')
        if 'margin: 4px;' in line:
            continue # Skip this line
        
        # Check if we are out of the listview? No easy way to know line by line.
        # But these replacements are safe enough if we restrict to the file.
    
    # Actually, we can just apply these replacements globally or contextually.
    # But "model: [" might appear elsewhere.
    # In HierarchyTool we already replaced it.
    # So we can apply these replacements to the remaining file content.
    pass

# Let's apply replacements on the string content, but only for the StructureTool part.
# We can split the content at "StructureTool".
parts = content.split('export component StructureTool')
if len(parts) > 1:
    structure_part = parts[1]
    
    structure_part = structure_part.replace('model: [', 'for item in [')
    structure_part = structure_part.replace('];', ']:')
    structure_part = structure_part.replace('delegate := Rectangle {', 'Rectangle {')
    structure_part = structure_part.replace('root.current_item', 'item')
    structure_part = structure_part.replace('margin: 4px;', '')
    
    content = parts[0] + 'export component StructureTool' + structure_part

# Write back
with open(file_path, 'w') as f:
    f.write(content)

print("Refactored ListViews in writing_tools.slint")
