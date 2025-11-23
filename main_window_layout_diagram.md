# Herding Cats Main Window Layout

## Application Window Structure
```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ 📝 Herding Cats - Professional Word Processor                        [DB: Connected]   [✕] │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│ File │ Projects │ Edit │ Tools │ View │ Help                                              │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│ 𝐁 Bold │ 𝘐 Italic │ U Underline │ 📝 Font │ 📏 Size │ 🔤 Color │ 🎨 Highlight │ ⬅ Left │ ↔ Center │ ➡ Right │ ↔ Justify │ 📋 Bullet │ 🔢 Number │ 📄 Styles │ ⚙️ Settings │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│ ┌─────────────────────────────── Document Editor ─────────────────────────────────────┐ │
│ │ 📄 Document Title: [Untitled Document                                                  ] │ │
│ ├─────────────────────────────────────────────────────────────────────────────────────┤ │
│ │                                                                                     │ │
│ │ ┌─ Text Editor Area (Scrollable) ─────────────────────────────────────────────────┐ │ │
│ │ │                                                                             │ │ │
│ │ │ 📝 Start writing your document...                                          │ │ │
│ │ │                                                                     │ │ │
│ │ │ This is a professional word processor with full database integration.        │ │ │
│ │ │ Use the menus above to access advanced features.                            │ │ │
│ │ │                                                                     │ │ │
│ │ │ 🔧 All writing tools are database-connected and can open as standalone     │ │ │
│ │ │ windows.                                                              │ │ │
│ │ │                                                                     │ │ │
│ │ │                                                                             │ │ │
│ │ │                                                                             │ │ │
│ │ │                                                                             │ │ │
│ │ │                                                                             │ │ │
│ │ └─────────────────────────────────────────────────────────────────────────────┘ │ │
│ │                                                                                     │ │
│ └─────────────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│ Status: Ready - Database Connected    Clicks: 0    Database: Active    Multi-Window Ready │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Individual Tool Windows

### Writing Tools Suite
Each tool opens as a standalone resizable window:

```
┌─ Hierarchy Tool Window ──────────────────────────────────────────────┐
│ 📊 Document Hierarchy - Enhanced                      [●] [○] [✕]    │
├──────────────────────────────────────────────────────────────────────┤
│                                                                     │
│ Chapter 1: Introduction                        ✓ Complete          │
│ Chapter 2: Background                         🔄 In Progress       │
│ Chapter 3: Main Story                         📝 Planned           │
│ Chapter 4: Climax                             📝 Planned           │
│ Chapter 5: Resolution                         📝 Planned           │
│                                                                     │
│ [➕ Add Chapter] [✏️ Edit] [🗑️ Delete]                            │
│ [⬆️ Move Up] [⬇️ Move Down] [🔄 Reorganize]                       │
└──────────────────────────────────────────────────────────────────────┘

┌─ World Building Codex Window ─────────────────────────────────────────┐
│ 📖 World Building Codex - Enhanced                [●] [○] [✕]        │
├──────────────────────────────────────────────────────────────────────┤
│ [👥 Characters] [📍 Locations] [⚡ Magic/Tech] [📜 Events]          │
│                                                                     │
│ Codex entries will appear here...                                  │
│                                                                     │
│ 👥 Character Name: Alice                                           │
│ Description: A brave adventurer with a mysterious past            │
│ Skills: Sword fighting, magic                                     │
│ Relationships: Friend of Bob                                      │
│                                                                     │
│ [➕ Add Entry] [✏️ Edit] [🔍 Search]                              │
│ [📤 Export] [📥 Import] [🗂️ Categories]                           │
└──────────────────────────────────────────────────────────────────────┘

┌─ Plot Development Window ────────────────────────────────────────────┐
│ 📈 Plot Development - Enhanced                    [●] [○] [✕]       │
├──────────────────────────────────────────────────────────────────────┤
│ Complete: 2/6 │ In Progress: 1/6 │ Planned: 3/6                     │
│                                                                     │
│ Act I: Setup - Character Introduction         ✓ Complete          │
│ Act I: Inciting Incident                       ✓ Complete          │
│ Act II: Rising Action                          🔄 In Progress      │
│ Act II: Midpoint Crisis                        📝 Planned          │
│ Act III: Climax                                📝 Planned          │
│ Act III: Resolution                            📝 Planned          │
│                                                                     │
│ [➕ Add Plot Point] [✏️ Edit] [🎯 Track Progress]                  │
│ [🌉 Connect Scenes] [📊 Analytics] [🔄 Timeline View]             │
└──────────────────────────────────────────────────────────────────────┘
```

## Menu System

### File Menu
```
┌─ File ─────────────────────────┐
│ 📄 New Document                │
│ 📂 Open Document               │
│ 💾 Save Document               │
│ 💾 Save As...                  │
│                               │
│ 🤖 AI Settings                │
│                               │
│ 🖨 Print                      │
│ 📤 Export                     │
│ ❌ Exit                       │
└───────────────────────────────┘
```

### Tools Menu (Writing Tools & Database)
```
┌─ Tools ─────────────────────────────────────────────────────────────────────────────┐
│ ✍️ Writing Tools                                                                     │
│ 📊 Document Hierarchy                                                                │
│ 📖 World Building Codex                                                              │
│ 📈 Plot Development                                                                  │
│ 📝 Research Notes                                                                    │
│ 📊 Plot Structure                                                                    │
│ 🧠 Brainstorming                                                                     │
│                                                                                       │
│ 🛠️ Utility Tools                                                                    │
│ 🔍 Search Tools                                                                      │
│ 📚 Research Hub                                                                      │
│ 📈 Writing Analysis                                                                  │
│ 🔤 Font Manager                                                                      │
│                                                                                       │
│ 🗄️ Database                                                                         │
│ 🗄️ Database Manager                                                                 │
└───────────────────────────────────────────────────────────────────────────────────────┘
```

## Window Dimensions & Layout

### Main Application Window
- **Preferred Size**: 1200px × 800px
- **Minimum Size**: 1000px × 700px
- **Fixed Components**:
  - Title Bar: 35px height
  - Menu Bar: 30px height
  - Ribbon Toolbar: 90px height
  - Status Bar: 30px height
- **Editable Area**: Variable (fits remaining space)

### Individual Tool Windows
- **Font Manager**: 600px × 500px
- **Writing Tools**: Resizable with drag handles
- **Enhanced Windows**: Support minimize/maximize/close controls

### Color Scheme (Professional Theme)
- **Primary Background**: #ffffff (White)
- **Secondary Background**: #f8f9fa (Light Gray)
- **Accent Color**: #007bff (Blue)
- **Text Primary**: #212529 (Dark Gray)
- **Text Secondary**: #6c757d (Medium Gray)
- **Border Color**: #dee2e6 (Light Border Gray)
- **Menu Background**: #f8f9fa (Light Gray)
- **Toolbar Background**: #ffffff (White)
- **Status Background**: #e9ecef (Medium Light Gray)

## Database Integration Indicators
- **Main Window**: Shows "Database: Connected" in title bar
- **Status Bar**: Displays "Database: Active" and "Multi-Window Ready"
- **AI Integration**: Dedicated AI settings popup with provider selection

## Window Management Features
- **Multi-Window Support**: Each writing tool can open as standalone window
- **Drag & Drop**: Enhanced tool windows support dragging
- **Resize Functionality**: Tool windows have resize handles
- **Theme Consistency**: Unified toolbar system across all windows
- **State Persistence**: Window states and positions maintained

## Ribbon Toolbar Details
- **Font Formatting**: Bold, Italic, Underline, Font, Size
- **Text Styling**: Color, Highlight
- **Alignment**: Left, Center, Right, Justify
- **Lists**: Bullet, Number
- **Styles**: Style Gallery
- **Settings**: Database and tool access

This layout provides a professional Microsoft Word-like interface with advanced database integration and a comprehensive suite of writing tools that can operate independently or within the main application window.