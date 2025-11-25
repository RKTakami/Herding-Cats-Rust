# Microsoft Font Licensing Requirements

## Overview
Microsoft's proprietary fonts (Segoe UI, Calibri, Consolas, etc.) are subject to commercial licensing restrictions that prohibit embedding in third-party applications without proper agreements.

## ⚠️ Licensing Restrictions

### What's **NOT** Permitted:
- **Application embedding** without Microsoft partner agreement
- **Commercial redistribution** as bundled fonts
- **Modification or derivatives** without explicit permission
- **Automated font loading** from embedded data
- **Distribution in software packages** without licensing

### What's **Permitted**:
- **System font usage** on Windows/macOS (already installed)
- **Web use** via Microsoft web font licensing (separate from desktop apps)
- **Licensed embedding** through Microsoft Font Partner Program

## Licensing Pathways

### 1. Microsoft Font Partner Program
- **Process**: Direct negotiation with Microsoft
- **Cost**: Varies by font, volume, and application type
- **Rights**: Full embedding and distribution rights
- **Timeline**: Weeks to months for negotiations
- **Contact**: Microsoft font licensing team

### 2. Web Font Licensing
- **Cost**: ~$99-199/year per font family
- **Limitation**: Only for web usage, NOT desktop applications
- **Example**: Segoe UI Web for websites only
- **Restriction**: Cannot embed in desktop software

### 3. Microsoft Open Fonts (Free & Safe)
These fonts are open-source and safe to embed:

#### Microsoft-Owned Open Source Fonts:
- **Cascadia Code** - Modern monospace font (Consolas alternative)
- **Cascadia Mono** - Sans-serif variant of Cascadia Code
- **Cascadia PL** - Programming ligatures version
- **Segoe UI Variable** - Open-source variable version of Segoe UI

#### Microsoft-Sponsored Open Source Fonts:
- **Fira Code** - Monospace font with programming ligatures
- **Fira Sans** - Humanist sans-serif font

#### NOT Available as Open Source:
- **Times New Roman** - Remains proprietary, requires licensing
- **Calibri** - Proprietary, requires licensing
- **Consolas** - Proprietary, use Cascadia Code instead
- **Segoe UI** - Proprietary, use Segoe UI Variable instead
- **Cambria** - Proprietary serif font
- **Corbel** - Proprietary sans-serif font
- **Constantia** - Proprietary serif font
- **Candara** - Proprietary sans-serif font

#### Free Times New Roman Alternatives:
- **Tinos** - Google Fonts, metric-compatible with Times New Roman
- **Crimson Text** - High-quality serif with similar proportions
- **Libre Baskerville** - Classic serif design
- **EB Garamond** - Traditional serif with elegant spacing

## Legal Implications

### Without Proper Licensing:
- **Copyright infringement** risk
- **Commercial distribution** blocked
- **Legal action** potential from Microsoft
- **DMCA takedown** notices for hosted fonts

### Risk Assessment for "Herding Cats":
- **Commercial software**: High risk without licensing
- **Open-source distribution**: Very high risk
- **User font download**: Still requires proper terms
- **System font only**: Lower risk but limited functionality

## Recommended Implementation Strategy

### Option A: Use Microsoft's Open Fonts (Recommended)
```rust
// Safe, legal, and high-quality
pub const SAFE_MICROSOFT_FONTS: &[(&str, &[u8])] = &[
    ("Cascadia Code", include_bytes!("cascadia/CascadiaCode-Regular.ttf")),
    ("Segoe UI Variable", include_bytes!("segue/SegoeUI-Variable.ttf")),
    ("Fira Code", include_bytes!("fira/FiraCode-Regular.ttf")),
];
```

**Benefits:**
- ✅ Legal to embed and distribute
- ✅ Microsoft-maintained quality
- ✅ Cross-platform consistency
- ✅ No licensing costs

### Option B: Partner Program (If Budget Allows)
- Contact: fonts@microsoft.com
- Provide: Application details, expected usage, font list
- Budget: $5,000-$50,000+ annually depending on scope

### Option C: System Font Detection
```rust
// Check for installed Microsoft fonts, use fallback if missing
pub fn get_best_available_font() -> &'static str {
    if system_has_font("Segoe UI") {
        "Segoe UI"  // Use system font
    } else {
        "Inter"     // Fallback to free alternative
    }
}
```

## Cost-Benefit Analysis

| Approach | Cost | Legal Risk | Quality | Control |
|----------|------|------------|---------|---------|
| Partner Program | $$$$ | None | High | Full |
| Web Fonts | $$ | Medium (web only) | High | Limited |
| Open Fonts | Free | None | High | Full |
| System Detection | Free | Low | Variable | Low |

## Recommendations

### For Herding Cats Project:
1. **Start with Microsoft Open Fonts** (Cascadia Code, Segoe UI Variable)
2. **Implement system font detection** for Segoe UI fallback
3. **Add font download** from Microsoft's official repositories
4. **Consider partner program** if commercial success warrants investment

### For Production Applications:
1. **Legal compliance** must be priority #1
2. **Budget for licensing** if Microsoft fonts are critical
3. **Document font sources** and licensing terms
4. **Regular licensing reviews** as project scales

## Official Resources

- **Microsoft Typography**: https://docs.microsoft.com/typography/
- **Font Licensing**: Contact fonts@microsoft.com
- **Open Source Fonts**: https://github.com/microsoft/cascadia-code
- **Legal Framework**: Review Microsoft's font EULA carefully

---

**⚠️ Disclaimer**: This information is for educational purposes. Consult legal counsel for specific licensing decisions and commercial applications.