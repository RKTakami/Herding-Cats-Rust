import re

file_path = 'src/ui/writing_tools.slint'

with open(file_path, 'r') as f:
    content = f.read()

# Fix ResearchTool Controls and closing braces
# Match from "// Research Controls" up to "export component StructureTool"
# This covers the malformed HorizontalBox and the closing braces mess.

research_pattern = r'// Research Controls\s+HorizontalBox\s*\{[\s\S]*?export component StructureTool'
research_replacement = """// Research Controls
                    HorizontalBox {
                        ToolbarButton { text: "➕ Add Source"; }
                        ToolbarButton { text: "📥 Import"; }
                        ToolbarButton { text: "🔍 Find Sources"; }
                    }

                    HorizontalBox {
                        ToolbarButton { text: "📊 Research Plan"; }
                        ToolbarButton { text: "📈 Progress"; }
                        ToolbarButton { text: "📤 Citation"; }
                    }

                    StandardButton {
                        kind: close;
                    }
                }
            }
        }
    }
}

// Structure Tool Window Component
export component StructureTool"""

content = re.sub(research_pattern, research_replacement, content)

# Write back
with open(file_path, 'w') as f:
    f.write(content)

print("Fixed writing_tools.slint")
