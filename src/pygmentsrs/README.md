
# pygmentsrs

A Rust port of [Pygments](https://pygments.org/) for syntax highlighting, designed for use with docutilsrs, docutils, sphinx, and sphinxdocrs.

## Features

- **451+ native lexers** — Python, JavaScript, LaTeX, Rust, and 448+ more languages (transpiled from Pygments)
- **15 native formatters** — HTML, ANSI terminal (16/256/24-bit), LaTeX, RTF, Groff, Pango, SVG, BBCode, IRC, and more
- **Optional RaTeX rendering** — LaTeX → SVG/PNG/PDF (feature-gated)
- **Pure Rust** — No Python dependency required (optional PyO3 bridge available)
- **100% test coverage** — Byte-parity tests against upstream Pygments

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pygmentsrs = "0.1"
```

Or with optional RaTeX rendering:

```toml
[dependencies]
pygmentsrs = { version = "0.1", features = ["ratex-full"] }
```

### Basic Usage

#### Lexical Analysis

```rust
use pygmentsrs::lexers::get_lexer_by_name;

let code = "def hello():\n    print('world')";
let lexer = get_lexer_by_name("python").unwrap();

for (token_type, value) in lexer.get_tokens(code) {
    println!("{}: {}", token_type.short_name(), value);
}
// Output:
// kn: def
// n: hello
// p: (
// p: )
// p: :
// ...
```

#### Syntax Highlighting to HTML

```rust
use pygmentsrs::lexers::get_lexer_by_name;
use pygmentsrs::formatters::registry::format_native;

let code = "let x = 42;";
let lexer = get_lexer_by_name("rust").unwrap();
let tokens: Vec<_> = lexer.get_tokens(code).collect();

let html = format_native("html", &tokens).expect("HTML formatter");
println!("{}", html);
// Output: <div class="highlight"><pre><span></span>...
```

## Formatter Examples

### Terminal Output (ANSI Colors)

```rust
use pygmentsrs::formatters::registry::format_native;
use pygmentsrs::lexers::get_lexer_by_name;

let code = "x = 42  # answer";
let lexer = get_lexer_by_name("python").unwrap();
let tokens: Vec<_> = lexer.get_tokens(code).collect();

// 16-color ANSI (classic)
let ansi = format_native("terminal", &tokens).unwrap();

// 256-color ANSI (more colors)
let ansi256 = format_native("terminal256", &tokens).unwrap();

// 24-bit true color (16 million colors)
let truecolor = format_native("terminal16m", &tokens).unwrap();
```

### Markup Formats

```rust
// LaTeX source code (for PDF compilation)
let latex = format_native("latex", &tokens).unwrap();

// Rich Text Format
let rtf = format_native("rtf", &tokens).unwrap();

// Groff/troff (Unix documentation)
let groff = format_native("groff", &tokens).unwrap();

// Pango markup (GTK applications)
let pango = format_native("pango", &tokens).unwrap();
```

### Vector Graphics

```rust
// SVG (scalable vector format)
let svg = format_native("svg", &tokens).unwrap();
// Output: <svg xmlns="..."><text>...
```

### Alternative Markup

```rust
// BBCode (forums, social media)
let bbcode = format_native("bbcode", &tokens).unwrap();

// mIRC color codes (IRC chat)
let irc = format_native("irc", &tokens).unwrap();
```

## Optional RaTeX Rendering

With the `ratex-*` features enabled, you can render LaTeX source to vector/raster formats:

### LaTeX to SVG (Vector)

```rust
#[cfg(feature = "ratex-svg")]
{
    use ratex_parser;
    use ratex_layout;
    use ratex_svg::{SvgOptions, display_list_to_svg};
    
    let latex = r#"\frac{x^2 + 1}{2}"#;
    
    // Parse LaTeX
    let ast = ratex_parser::parse(latex).unwrap();
    
    // Layout to display list
    let display_list = ratex_layout::to_display_list(&ast).unwrap();
    
    // Render to SVG
    let options = SvgOptions {
        font_size: 40.0,
        padding: 10.0,
        ..Default::default()
    };
    let svg = display_list_to_svg(&display_list, &options);
    println!("{}", svg);  // <svg xmlns="...">...</svg>
}
```

### LaTeX to PNG (Raster)

```rust
#[cfg(feature = "ratex-png")]
{
    use ratex_parser;
    use ratex_layout;
    use ratex_render::{render_to_png, RenderOptions};
    use std::fs;
    
    let latex = r#"\int_0^{\infty} e^{-x} dx"#;
    
    // Parse and layout
    let ast = ratex_parser::parse(latex).unwrap();
    let display_list = ratex_layout::to_display_list(&ast).unwrap();
    
    // Render to PNG
    let options = RenderOptions {
        font_size: 40.0,
        padding: 10.0,
        ..Default::default()
    };
    let png_bytes = render_to_png(&display_list, &options).unwrap();
    
    // Save to file
    fs::write("formula.png", png_bytes).unwrap();
}
```

### LaTeX to PDF (Document)

```rust
#[cfg(feature = "ratex-pdf")]
{
    use ratex_parser;
    use ratex_layout;
    use ratex_pdf;  // TBD: API to be confirmed
    
    let latex = r#"\documentclass{article}\begin{document}Hello\end{document}"#;
    
    // Parse, layout, render to PDF
    let ast = ratex_parser::parse(latex).unwrap();
    let display_list = ratex_layout::to_display_list(&ast).unwrap();
    
    // PDF rendering (Phase F3.1+)
    // let pdf_bytes = ratex_pdf::render(&display_list).unwrap();
}
```

## Build Options

### Default Build (with PyO3 bridge)

```bash
cargo build -p pygmentsrs
```

Includes Python interoperability via PyO3. Formatters can fall back to upstream Pygments if native implementation not available.

### Pure Rust (no Python)

```bash
cargo build -p pygmentsrs --no-default-features
```

Standalone Rust library with zero CPython dependency.

### With RaTeX Support

```bash
# SVG rendering only
cargo build -p pygmentsrs --features ratex-svg

# SVG + PNG rasterization
cargo build -p pygmentsrs --features ratex-svg,ratex-png

# All RaTeX features (SVG, PNG, PDF + embedded fonts)
cargo build -p pygmentsrs --features ratex-full
```

## Feature Flags

| Feature | Purpose | Default | Size Impact |
|---------|---------|---------|-------------|
| `python-bridge` | PyO3 bridge to Pygments | ✅ Yes | +1 MB |
| `ratex-svg` | LaTeX → SVG rendering | ❌ No | +5 MB |
| `ratex-png` | LaTeX → PNG rasterization | ❌ No | +3 MB |
| `ratex-pdf` | LaTeX → PDF rendering | ❌ No | +2 MB |
| `ratex-fonts` | Embed KaTeX fonts | ❌ No | +5 MB |
| `ratex-full` | All RaTeX features | ❌ No | +15 MB |

See [PYGMENTS_FEATURE_FLAGS.md](../../docs/PYGMENTS_FEATURE_FLAGS.md) for detailed feature documentation.

## Objectives

- Port Pygments to Rust as pygmentsrs
- Provide native lexers and formatters for use with docutilsrs, docutils, sphinx, sphinxdocrs
- Support optional RaTeX for advanced LaTeX rendering
- Maintain byte-parity with upstream Pygments
- Enable pure-Rust or Python-interoperable builds

## Status

**Phase F (Formatters) Complete** ✅

- 15 native formatters implemented (83% coverage)
- 451+ native lexers (436 generated + 15 built-in)
- 77 tests passing (lexers + formatters + byte-parity)
- Optional RaTeX feature gates (Phase F3.1+)
- Raster formats bridge-only (Phase F4, deferred)

See [PHASE_F_COMPLETION_SUMMARY.md](../../docs/PHASE_F_COMPLETION_SUMMARY.md) for details.

## Documentation

- [Feature Flags Guide](../../docs/PYGMENTS_FEATURE_FLAGS.md) — Build options and features
- [RaTeX Integration Plan](../../docs/RATEX_INTEGRATION_PLAN.md) — Optional rendering engine
- [Raster Format Strategy](../../docs/RASTER_FORMAT_STRATEGY.md) — PNG/BMP/JPG/GIF decisions
- [Formatter Audit](../../SECURITY_AUDIT_FORMATTERS.md) — Security review
- [Port Inventory](../../docs/pygments-port-inventory.md) — Lexer and formatter tracking

## Examples

See the `/examples` directory for complete working examples:
- `highlight.rs` — HTML output from multiple lexers
- `formatters.rs` — All formatter types
- `ratex_svg.rs` — LaTeX to SVG rendering (requires `ratex-svg` feature)

## Dependencies

**Core:**
- `fancy-regex` — Regex engine with advanced features
- `phf` — Perfect hashing for lexer registry

**Optional:**
- `pyo3` — Python bridge (feature: `python-bridge`)
- `ratex-*` — LaTeX rendering (features: `ratex-svg`, `ratex-png`, `ratex-pdf`, `ratex-fonts`)

## License

BSD-2-Clause (same as upstream Pygments)

See [LICENSE.pygments](../../LICENSE.pygments) for details.