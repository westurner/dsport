# RaTeX Integration Plan for Pygments Formatters (Phase F, V2)

## Executive Summary

RaTeX is a mature Rust LaTeX math typesetter that can render:
- **SVG** (vector, infinite zoom, text/math mixed)
- **PNG/WebP** (raster, fast rendering, device-pixel-aware)
- **PDF** (production quality output)

RaTeX is **OPTIONAL** and should be feature-gated. The current LaTeX formatter (F3) produces **LaTeX source**, not rendered output. RaTeX V2 would add **optional rendering** to produce image formats.

## Architecture Overview

```
LatexFormatter (current)
    ↓ (produces LaTeX source: \documentclass{...}\begin{lstlisting}...\end{lstlisting})
    ↓
RaTeX V2 (optional, feature-gated)
    ↓ (parses LaTeX, lays out math)
    ├→ render_to_png() → bytes  (PNG image)
    ├→ render_to_svg() → String (SVG markup)
    └→ render_to_pdf() → bytes  (PDF document)
```

## RaTeX Crate Structure

Located in `/workspaces/dsport/src/RaTeX/`:

| Crate | Purpose | Public API |
|-------|---------|-----------|
| `ratex-types` | Data types (DisplayList, DisplayItem, Color) | Core types |
| `ratex-parser` | LaTeX parser | parse() → AST |
| `ratex-layout` | Layout engine | to_display_list() → DisplayList |
| `ratex-render` | PNG/raster | render_to_png(DisplayList, opts) → Vec<u8> |
| `ratex-svg` | SVG vector | display_list_to_svg() → String |
| `ratex-pdf` | PDF output | ??? (TBD from source) |
| `ratex-font-loader` | Font management | loads `.ttf` from directory or embedded |
| `ratex-katex-fonts` | KaTeX fonts | embedded TTF + PNG strikes |

## Proposed Implementation

### Phase F3.1 — Prototype (Non-blocking)

**Goal:** Prove RaTeX can render Pygments LaTeX output.

**Steps:**
1. Create `latex_to_svg()` wrapper:
   ```rust
   fn latex_to_svg(latex_src: &str) -> Result<String, String> {
       let ast = ratex_parser::parse(latex_src)?;
       let display_list = ratex_layout::to_display_list(&ast)?;
       let options = SvgOptions { font_size: 40.0, .. };
       ratex_svg::display_list_to_svg(&display_list, &options)
   }
   ```

2. Test with sample LaTeX from LatexFormatter output (e.g., `\documentclass{article}\begin{lstlisting}x = 42\end{lstlisting}`)
   - ⚠️ **Risk**: `lstlisting` environment may not be in RaTeX's scope (math-only)
   - **Mitigation**: Use `\text{}` or extract code tokens separately

3. Create optional Cargo feature:
   ```toml
   [features]
   ratex-svg = ["dep:ratex-svg", "dep:ratex-layout", "dep:ratex-parser"]
   ```

### Phase F3.2 — Integration (Deferred to V2)

**Goal:** Add native `svg`, `pdf`, `png` formatters using RaTeX.

**New formatters:**
- `SvgMathFormatter` — tokenizes Python → LaTeX math → RaTeX SVG
  - Input: Python tokens (mostly for math/REPL)
  - Output: `<svg>...</svg>` with rendered math
  - Example: `42 ** 2` → $42^2$ → SVG math

- `PdfMathFormatter` — Python tokens → LaTeX → PDF
  - Output: binary PDF file
  - Use case: formal documentation

- `PngMathFormatter` — Python tokens → LaTeX → PNG
  - Output: binary PNG (lossy)
  - Use case: web thumbnails, compatibility

**Open Questions:**
1. Should formatters handle **full code** or **math only**?
   - Current: Full code (with `\texttt{}` for literals)
   - RaTeX: Optimized for math ($E = mc^2$)
   - Decision: Preserve code tokens as-is, use RaTeX only for math regions

2. Font selection: Embed or external?
   - `ratex-katex-fonts` feature: embed KaTeX fonts (~5 MB)
   - Or: system fonts + fallback
   - Recommendation: Embed for standalone, gate behind `ratex-full-fonts`

3. PDF rendering: library API?
   - TBD: check `ratex-pdf` crate public API
   - May require development if no public API exists

### Phase F3.3 — Bridge Fallback (Optional)

If RaTeX rendering fails (e.g., malformed LaTeX), fall back to:
1. Python Pygments via PyO3 bridge (if available)
2. Raw LaTeX source (if no bridge)

```rust
pub fn format_latex_to_svg(tokens: &[(TokenType, String)]) -> Result<String, String> {
    let latex = LatexFormatter::format(tokens);  // source code
    
    #[cfg(feature = "ratex-svg")]
    {
        match latex_to_svg(&latex) {
            Ok(svg) => Ok(svg),
            Err(_) => Ok(latex),  // Fallback to source
        }
    }
    
    #[cfg(not(feature = "ratex-svg"))]
    Ok(latex)  // No RaTeX: return source only
}
```

## Feature Gates Proposed

```toml
[features]
# Core (always on)
default = ["python-bridge"]

# Optional RaTeX rendering (V2)
ratex-svg      = ["dep:ratex-svg"]
ratex-pdf      = ["dep:ratex-pdf"]
ratex-png      = ["dep:ratex-render"]
ratex-fonts    = ["dep:ratex-katex-fonts"]  # embed KaTeX fonts
ratex-full     = ["ratex-svg", "ratex-pdf", "ratex-png", "ratex-fonts"]

# Python bridge (existing)
python-bridge  = ["dep:pyo3"]
```

## Output Format Examples

### LaTeX Source (Current)
```latex
\documentclass{article}
\usepackage{xcolor}
\begin{lstlisting}
\PY{n}{x} \PY{o}{=} \PY{l}{42}
\end{lstlisting}
\end{document}
```

### LaTeX → SVG (V2, RaTeX)
```xml
<svg viewBox="0 0 200 50">
  <g font-family="KaTeX_Main">
    <text x="10" y="40">x = 42</text>
  </g>
</svg>
```

### LaTeX → PDF (V2, RaTeX)
Binary PDF with KaTeX font metrics + glyph outlines.

## Known Limitations

1. **RaTeX is math-focused**, not full LaTeX
   - Supported: `$...$`, `\frac{}{}`, `\sqrt{}`, environments
   - Unsupported: `\usepackage{}`, `\documentclass{}`, custom macros
   - Impact: Phase F3.2 may need preprocessing to strip non-math commands

2. **Font availability**
   - KaTeX fonts are designed for web/math
   - For code highlighting, may need fallback to monospace fonts
   - Recommendation: Use `\text{\ttfamily ...}` for code tokens

3. **Coloring in RaTeX**
   - RaTeX supports `\color{rgb(r, g, b)}` or named colors
   - Current LatexFormatter uses `\textcolor{#RRGGBB}{...}`
   - Need to verify compatibility or convert format

## Delivery Timeline

**Phase F3 (Current - Formatters):**
- ✅ Phase F0, F1, F2, F3 complete (15 formatters, 22 tests)

**Phase F3.1 (V2 Prototype - Optional):**
- Estimated: 2-3 days
- Create `latex_to_svg()` wrapper
- Prototype tests
- NOT blocking Phase F release

**Phase F3.2 (V2 Full Integration - Deferred):**
- Estimated: 1-2 weeks (if pursued)
- Implement `SvgMathFormatter`, `PdfMathFormatter`, `PngMathFormatter`
- Resolve color/font compatibility
- Target: Post-Phase F release (backlog)

**Phase F4 (Raster Formatters - Deferred):**
- `BmpFormatter`, `GifFormatter`, `JpgFormatter`, `WebpFormatter`
- Will likely use RaTeX V2 or bridge to Python via image crate
- Target: Phase F5 (low priority)

## Success Criteria

- [ ] RaTeX V2 prototype compiles with `--features ratex-svg`
- [ ] LaTeX source → SVG renders without panicking
- [ ] Byte-parity tests pass for SVG output vs Pygments
- [ ] Feature gate is optional: `cargo build --no-default-features` still works
- [ ] Fallback to LaTeX source when RaTeX unavailable

## Risks & Mitigations

| Risk | Severity | Mitigation |
|------|----------|-----------|
| RaTeX API unstable | Medium | Maintain fork or pin version |
| Math-only scope | Medium | Use `\text{}` for non-math, preprocess |
| Font licensing | Low | KaTeX fonts are Apache 2.0, compatible |
| Build time | Low | Make feature optional, gate behind `ratex-full` |
| Circular dep (RaTeX → Pygments) | Low | RaTeX has no dep on Pygments; inverse only |

## Next Steps (Recommendation)

1. **Immediate**: Merge Phase F (15 formatters, byte-parity tests, security audit) ✅
2. **Short term**: Document this plan, create issue for F3.1 prototype
3. **Medium term**: Evaluate F3.1 prototype against use cases
4. **Long term**: Plan F3.2 and F4 based on demand

---

## Appendix: RaTeX Pub API Summary

### ratex-parser
```rust
pub fn parse(src: &str) -> Result<MathList, ParseError>
```

### ratex-layout
```rust
pub fn to_display_list(ast: &MathList) -> Result<DisplayList, LayoutError>
// DisplayList { width: f32, height: f32, depth: f32, items: Vec<DisplayItem> }
```

### ratex-svg
```rust
pub struct SvgOptions {
    pub font_size: f64,
    pub padding: f64,
    pub stroke_width: f64,
    pub font_dir: Option<PathBuf>,
    pub embed_glyphs: bool,  // requires `standalone` feature
}

pub fn display_list_to_svg(list: &DisplayList, options: &SvgOptions) -> String
```

### ratex-render
```rust
pub fn render_to_png(display_list: &DisplayList, options: &RenderOptions) -> Result<Vec<u8>, String>
// RenderOptions { font_size: f32, padding: f32, background_color: Color, ... }
```

### ratex-pdf
- API TBD (requires source inspection)
