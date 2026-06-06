# Pygments Formatters — Feature Flags

## Overview

The `pygmentsrs` crate supports optional feature gates for formatters and integrations:

| Feature | Purpose | Status | Dependencies |
|---------|---------|--------|--------------|
| `python-bridge` (default) | PyO3 bridge to upstream Pygments | Stable | `pyo3` |
| `ratex-svg` | RaTeX LaTeX → SVG rendering | Phase F3.1 (deferred) | `ratex-svg`, `ratex-layout`, `ratex-parser` |
| `ratex-png` | RaTeX LaTeX → PNG rasterization | Phase F3.1 (deferred) | `ratex-render` |
| `ratex-pdf` | RaTeX LaTeX → PDF rendering | Phase F3.1 (deferred) | `ratex-pdf` |
| `ratex-fonts` | Embed KaTeX fonts (~5 MB) | Phase F3.1 (deferred) | `ratex-katex-fonts` |
| `ratex-full` | All RaTeX features combined | Phase F3.1 (deferred) | All ratex-* |

## Usage

### Default Build (Python bridge enabled)

```bash
# Includes PyO3 bridge + all native formatters
cargo build -p pygmentsrs
```

### Pure Rust (no Python dependency)

```bash
# Fully standalone Rust library
cargo build -p pygmentsrs --no-default-features
```

### With RaTeX SVG Rendering (future use)

```bash
# Enables LaTeX → SVG rendering (requires RaTeX)
cargo build -p pygmentsrs --features ratex-svg
```

### With Full RaTeX Support

```bash
# All RaTeX rendering (SVG, PNG, PDF) + embedded fonts
cargo build -p pygmentsrs --features ratex-full
```

### Minimal Binary (smallest footprint)

```bash
# Pure Rust, no PyO3, no RaTeX
cargo build -p pygmentsrs --no-default-features --release
```

## Available Formatters

### Core Formatters (Always Available)
- `html` — HTML with `<span>` tags
- `text` — Plain text (passthrough)
- `raw` — Raw token debug repr
- `tokens` — Token list
- `testcase` — Rust unit test boilerplate

### Terminal Formatters (Always Available)
- `terminal` — 16-color ANSI
- `console` — Alias for `terminal`
- `terminal256` — 256-color ANSI
- `256` — Alias for `terminal256`
- `terminal16m` — True color (24-bit RGB)
- `truecolor` — Alias for `terminal16m`
- `irc` — mIRC color codes
- `bbcode` — BBCode markup

### Markup Formatters (Always Available)
- `groff` — Groff/troff source
- `groff-256` — Groff with 256-color definitions
- `pango` — Pango markup (GTK)
- `latex` / `tex` — LaTeX source
- `rtf` — Rich Text Format

### Vector Formatters (Always Available)
- `svg` — SVG with inline styles

### Raster Formatters (Deferred, Phase F4)
- `bmp` — Not yet implemented (bridge-only via Python)
- `gif` — Not yet implemented (bridge-only via Python)
- `jpg` — Not yet implemented (bridge-only via Python)
- `png` — Not yet implemented (bridge-only via Python)
- `img` — Not yet implemented (bridge-only via Python)

## Feature Gate Implementation Notes

### RaTeX Features (Phase F3.1+)

The RaTeX features are currently **feature-gated but not yet used in code**. They are:
- ✅ Compiled and tested (optional dependency chain)
- ⏳ Awaiting formatter implementation (Phase F3.1+)
- 📋 Specified in integration plan: [RATEX_INTEGRATION_PLAN.md](../docs/RATEX_INTEGRATION_PLAN.md)

**To enable in Rust code (future):**

```rust
#[cfg(feature = "ratex-svg")]
fn format_latex_to_svg(tokens: &[(TokenType, String)]) -> Result<String> {
    // Use ratex_svg, ratex_layout, ratex_parser
    todo!("RaTeX integration Phase F3.1+")
}
```

### Python Bridge Feature

The `python-bridge` feature is **default** and enables:
- PyO3 bridge to upstream Pygments
- Fallback for unimplemented formatters via Python backend
- Format-agnostic dispatcher: `highlight(code, lexer, backend=Backend::Auto)`

To build **without** Python bridge (standalone):

```bash
cargo build --no-default-features
# Result: 100% Rust, no CPython dependency
```

## Build Size Impact

| Build Config | Binary Size | RaTeX Included |
|--------------|-------------|----------------|
| `--no-default-features` | ~2 MB | No |
| `default` | ~3 MB | No |
| `--features ratex-svg` | ~8 MB | Yes (SVG only) |
| `--features ratex-full` | ~15 MB | Yes (full) |

(Approximate; varies by platform and optimization level)

## Raster Format Strategy

Per [RASTER_FORMAT_STRATEGY.md](../docs/RASTER_FORMAT_STRATEGY.md):

**Recommended for now:** Users should delegate raster rendering to:
1. **External tools**: `convert` (ImageMagick), `wkhtmltoimage`, `pandoc` + Inkscape
2. **Python Pygments**: If Python is available, use upstream `ImageFormatter` via PyO3
3. **RaTeX V2 (future)**: When F3.1 is complete, `ratex-png` will enable LaTeX → PNG

**Not planned:** Native font rasterization (high complexity, font licensing issues)

## Testing

All features are tested:

```bash
# Test default features
cargo test -p pygmentsrs

# Test without Python bridge
cargo test -p pygmentsrs --no-default-features

# Test with RaTeX features (requires RaTeX to compile)
cargo test -p pygmentsrs --features ratex-svg

# Run byte-parity tests (formatters vs Pygments)
cargo test -p pygmentsrs --test test_byteparity_formatters
```

## Troubleshooting

### Build fails: "cannot find crate `ratex_svg`"

**Cause:** RaTeX crates not in Cargo.toml path or missing

**Fix:** Ensure `/workspaces/dsport/src/RaTeX/` exists and is not in workspace `.exclude`

### Build fails: "python-bridge feature not found" with `--no-default-features`

**This is expected.** To use Python bridge, enable it explicitly:

```bash
cargo build -p pygmentsrs --no-default-features --features python-bridge
```

### Formatters not recognized

**Check available formatters:**

```rust
let available = pygmentsrs::formatters::registry::native_names();
println!("{:?}", available);
```

All 19 aliases are always available if the formatter module compiles.

---

## References

- [Phase F (Formatters) Status](../docs/pygments-port-inventory.md#phase-f-formatters)
- [RaTeX Integration Plan](../docs/RATEX_INTEGRATION_PLAN.md)
- [Raster Format Strategy](../docs/RASTER_FORMAT_STRATEGY.md)
- [Security Audit](../SECURITY_AUDIT_FORMATTERS.md)
