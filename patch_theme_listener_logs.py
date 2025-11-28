import os

file_path = 'src/ui/tools/individual_tool_windows.rs'

with open(file_path, 'r') as f:
    lines = f.readlines()

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
            log::info!("🎨 [IndividualToolWindowManager] Theme change detected. Scheduling update on main thread.");
            let _ = slint::invoke_from_event_loop(move || {
                ACTIVE_TOOL_WINDOWS.with(|windows| {
                    let windows_map = windows.borrow();
                    log::info!("🎨 [IndividualToolWindowManager] Updating {} active tool windows.", windows_map.len());
                    for (tool_type, window) in windows_map.iter() {
                        log::info!("🎨 [IndividualToolWindowManager] Applying theme to {:?}", tool_type);
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
    print("Successfully patched individual_tool_windows.rs with logging")
else:
    # It might be that the file is already patched (from previous successful run before I tried to add logs via replace_file_content and failed).
    # Or it might be the reverted state.
    # Let's check if the *previous* patch (without logs) is there.
    
    target_block_existing = """    pub fn new(db_state: Arc<RwLock<DatabaseAppState>>) -> Self {
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
    
    if target_block_existing in content:
        print("Found existing patch without logs. Updating to include logs.")
        new_content = content.replace(target_block_existing, replacement_block)
        with open(file_path, 'w') as f:
            f.write(new_content)
        print("Successfully updated patch with logging")
    else:
        print("Target block not found. Could not patch file.")
        # Print a snippet to see what's wrong
        print("Snippet around expected location:")
        for i, line in enumerate(lines):
            if 'pub fn new' in line:
                print("".join(lines[i:i+20]))
                break
