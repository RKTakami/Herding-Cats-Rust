# Times New Roman Alternatives for Herding Cats

## Current Project Status
Your project already includes **Crimson Text** (`src/fonts/CrimsonText-Regular.ttf`), which is an excellent serif font for document editing and formal text.

## Times New Roman Free Alternatives

### 1. Tinos (Metric-Compatible) ⭐
```rust
// Best drop-in replacement for Times New Roman
pub const SERIF_FONTS: &[(&str, &[u8], &str)] = &[
    ("Times New Roman", include_bytes!("tinos/Tinos-Regular.ttf"), "Tinos-Regular"),
    ("Tinos", include_bytes!("tinos/Tinos-Regular.ttf"), "Tinos-Regular"),
    ("Crimson Text", include_bytes!("CrimsonText-Regular.ttf"), "Crimson Text"),  // Already in project
];
```

**Benefits:**
- ✅ Same text metrics as Times New Roman
- ✅ Perfect for replacing TNR in existing documents
- ✅ High readability and professional appearance
- ✅ Free to embed and distribute

### 2. Crimson Text (Already Available)
Your project includes this font - no additional licensing needed!

**Current font path**: `src/fonts/CrimsonText-Regular.ttf`

### 3. Libre Baskerville
```rust
pub const CLASSIC_SERIF: &[(&str, &[u8])] = &[
    ("Libre Baskerville", include_bytes!("libre-baskerville/LibreBaskerville-Regular.ttf")),
    ("Libre Baskerville Bold", include_bytes!("libre-baskerville/LibreBaskerville-Bold.ttf")),
];
```

## Font Stack Recommendation

```rust
pub fn get_serif_font_stack() -> Vec<&'static str> {
    vec![
        "Times New Roman",    // Use Tinos as fallback
        "Tinos",             // Metric-compatible
        "Crimson Text",      // Already in project
        "Libre Baskerville", // Classic alternative
        "serif",             // System fallback
    ]
}
```

## Usage in UI Components

```slint
// Document editing components
component DocumentEditor inherits Window {
    default-font-family: "Times New Roman";  // Will use Tinos as fallback
    default-font-size: 12pt;
}

// Formal text components  
component FormalText inherits Text {
    default-font-family: "Crimson Text";  // Use existing project font
}
```

## Licensing Status
- **Tinos**: SIL Open Font License (free)
- **Crimson Text**: SIL Open Font License (free, already included)
- **Libre Baskerville**: SIL Open Font License (free)
- **Times New Roman**: Microsoft proprietary (requires licensing)

## Implementation Priority
1. **Use Crimson Text** (already available in your project)
2. **Add Tinos** for Times New Roman compatibility
3. **Add Libre Baskerville** for classic serif option
4. **Avoid embedding actual Times New Roman** without Microsoft licensing