import re

file_path = 'src/ui/tools/individual_tool_windows.rs'

with open(file_path, 'r') as f:
    content = f.read()

# Remove println! statements
# We'll use regex to match println! macro calls
# This regex matches println!("..."); including potential newlines inside the string (though unlikely for these logs)
# and handles the semicolon.
# Be careful not to remove other printlns if any (though we only added them for debugging)
# The logs we added start with specific emojis or tags, but let's just remove all println! in this file 
# assuming there were none before (which is likely for a UI file, usually they use log::info/warn).
# Wait, the grep output showed `log::info!`.
# So we should ONLY remove `println!`.

new_content = re.sub(r'^\s*println!.*?;[\r\n]*', '', content, flags=re.MULTILINE)

with open(file_path, 'w') as f:
    f.write(new_content)

print(f"Removed println! statements from {file_path}")
