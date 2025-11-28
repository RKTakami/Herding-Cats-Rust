import os

file_path = 'src/ui/tools/individual_tool_windows.rs'

with open(file_path, 'r') as f:
    content = f.read()

# Replacement 1: Start of function and color getting
target1 = '''    fn open_hierarchy_window(&self) -> Result<()> {
        println!("🚀 Opening Hierarchy Tool Window");

        // Get current theme colors
        let colors = get_current_theme_colors();

        // Check if already open in this thread
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {'''

replacement1 = '''    fn open_hierarchy_window(&self) -> Result<()> {
        println!("🚀 [open_hierarchy_window] Opening Hierarchy Tool Window");

        // Get current theme colors
        println!("🎨 [open_hierarchy_window] Getting theme colors...");
        let colors = get_current_theme_colors();
        println!("🎨 [open_hierarchy_window] Theme colors acquired.");

        // Check if already open in this thread
        println!("🔍 [open_hierarchy_window] Checking ACTIVE_TOOL_WINDOWS...");
        let is_open = ACTIVE_TOOL_WINDOWS.with(|windows| {'''

if target1 in content:
    content = content.replace(target1, replacement1)
    print("Applied replacement 1")
else:
    print("Target 1 not found!")

# Replacement 2: Window creation and theme application
target2 = '''        // Create Slint window for Hierarchy tool
        let hierarchy_window = hierarchy::HierarchyToolWindow::new()?;
        
        // Apply theme
        apply_theme!(&hierarchy_window, &colors, hierarchy);'''

replacement2 = '''        // Create Slint window for Hierarchy tool
        println!("🏗️ [open_hierarchy_window] Creating Slint window...");
        let hierarchy_window = hierarchy::HierarchyToolWindow::new()?;
        println!("🏗️ [open_hierarchy_window] Slint window created.");
        
        // Apply theme
        println!("🎨 [open_hierarchy_window] Applying theme...");
        apply_theme!(&hierarchy_window, &colors, hierarchy);
        println!("🎨 [open_hierarchy_window] Theme applied.");'''

if target2 in content:
    content = content.replace(target2, replacement2)
    print("Applied replacement 2")
else:
    print("Target 2 not found!")

with open(file_path, 'w') as f:
    f.write(content)
