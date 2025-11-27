import sys
import re

file_path = "src/ui/tools/individual_tool_windows.rs"

with open(file_path, "r") as f:
    content = f.read()

# Remove inline struct SlintThemeColors
# It looks like:
#     struct SlintThemeColors {
#         ...
#     }
# We'll use regex to remove it.
# Be careful not to remove other structs if any (but there shouldn't be any in slint! blocks).
content = re.sub(r'    struct SlintThemeColors \{[^}]+\}\n', '', content, flags=re.DOTALL)

# Now add imports to all blocks.
# We look for 'from "std-widgets.slint";'
# And we want to ensure we have:
#     import { Theme } from "../styles.slint";
#     import { SlintThemeColors } from "../theme_types.slint";

# First, remove existing imports of Theme and SlintThemeColors to avoid duplicates
content = re.sub(r'    import \{ Theme \} from "../styles.slint";\n', '', content)
content = re.sub(r'    import \{ SlintThemeColors \} from "../styles.slint";\n', '', content)
# Also handle the combined import if it exists (though I removed it)
content = re.sub(r'    import \{ Theme, SlintThemeColors \} from "../styles.slint";\n', '', content)

# Now insert the new imports
replacement = '    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";\n    import { Theme } from "../styles.slint";\n    import { SlintThemeColors } from "../theme_types.slint";'

# Replace the std-widgets import line with the block of imports
# Note: The std-widgets import might be multi-line or single-line.
# In my previous edits, I made it single line in the first block, but others might be multi-line?
# No, I restored the file, so they are all single line (except the first one which I modified with sed? No, I restored).
# Wait, I restored the file in Step 345.
# Then I ran `fix_build.py` in Step 354.
# `fix_build.py` modified the file.
# So the first block has inline struct.
# The others have `import { Theme }`.

# My regex removal of inline struct should work.
# My regex removal of `import { Theme }` should work.

# Now I need to find the std-widgets import.
# In `fix_build.py`, I didn't change the std-widgets import line itself.
# So it should be:
#     import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
# (Single line).

# However, in the original file (restored), was it single line?
# Step 217 view showed:
# 19:     import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";
# Yes, single line.

# So I can replace it.
content = content.replace(
    '    import { Button, TextEdit, ScrollView, HorizontalBox, VerticalBox } from "std-widgets.slint";',
    replacement
)

with open(file_path, "w") as f:
    f.write(content)
