import os

file_path = 'src/ui/tools/individual_tool_windows.rs'

with open(file_path, 'r') as f:
    lines = f.readlines()

# Find the line where `IndividualToolWindowManager::new` is defined
# and where the struct initialization ends.

# We look for:
#     pub fn new(db_state: Arc<RwLock<DatabaseAppState>>) -> Self {
#         Self {
#             ...
#         }
#     }

start_index = -1
end_brace_index = -1

for i, line in enumerate(lines):
    if 'pub fn new(db_state: Arc<RwLock<DatabaseAppState>>) -> Self {' in line:
        start_index = i
    if start_index != -1 and i > start_index:
        if '}' in line and lines[i-1].strip().endswith('}'): # This is heuristic, looking for the closing brace of Self { ... }
             # Actually, let's look for the closing brace of the function if we can't find the struct one easily.
             # But we want to change `Self { ... }` to `let manager = Self { ... }; ... manager`
             pass

# Let's try a more robust replacement using the exact content we know is there.
content = "".join(lines)

target_block = """    pub fn new(db_state: Arc<RwLock<DatabaseAppState>>) -> Self {
        Self {
            db_state,
            app_state: Arc::new(Mutex::new(AppState::default())),
            tool_windows: Arc::new(Mutex::new(std::collections::HashMap::new())),
            window_states: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }"""

replacement_block = """    pub fn new(db_state: Arc<RwLock<DatabaseAppState>>) -> Self {
        let manager = Self {
            db_state,
            app_state: Arc::new(Mutex::new(AppState::default())),
            tool_windows: Arc::new(Mutex::new(std::collections::HashMap::new())),
            window_states: Arc::new(Mutex::new(std::collections::HashMap::new())),
        };

        // Register theme change listener
        // This ensures that when the theme changes in the main app,
        // all open tool windows are updated immediately.
        crate::ui::theme_manager::get_theme_manager().on_theme_change(|theme| {
            let colors = theme.colors.clone();
            let _ = slint::invoke_from_event_loop(move || {
                ACTIVE_TOOL_WINDOWS.with(|windows| {
                    for window in windows.borrow().values() {
                        window.apply_theme(&colors);
                    }
                });
            });
        });

        manager
    }"""

if target_block in content:
    new_content = content.replace(target_block, replacement_block)
    with open(file_path, 'w') as f:
        f.write(new_content)
    print("Successfully patched individual_tool_windows.rs")
else:
    print("Target block not found. Could not patch file.")
    # Print a snippet to see what's wrong
    print("Snippet around expected location:")
    for i, line in enumerate(lines):
        if 'pub fn new' in line:
            print("".join(lines[i:i+10]))
            break
