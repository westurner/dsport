//! Math rendering backend selector.
//!
//! Three crates in this workspace (`docutilsrs`, `sphinxdocrs`,
//! `myst-md-rs`) need to turn LaTeX math (`$x^2$`, `:math:`x^2``,
//! `.. math::` blocks, `{math}` roles, etc.) into HTML. Historically
//! they each emitted a placeholder `<span class="math">…</span>` and
//! delegated actual rendering to JavaScript at view time
//! (MathJax/KaTeX) or to an out-of-process LaTeX toolchain (Sphinx's
//! `imgmath` extension).
//!
//! This crate is a thin façade that lets all three pick a rendering
//! strategy via a single [`MathBackend`] enum. The **default** backend
//! is [`MathBackend::Ratex`] — pure-Rust SVG rendering powered by the
//! vendored RaTeX engine — so math renders correctly even with
//! JavaScript disabled, without shelling out to a TeX install.
//!
//! Two compatibility backends are offered for parity with Sphinx's
//! built-in extensions:
//!
//! * [`MathBackend::MathJax`] — emit the same `\(…\)` / `\[…\]` markers
//!   `sphinx.ext.mathjax` produces, so callers responsible for the
//!   page template can keep including the MathJax script unchanged.
//! * [`MathBackend::ImgMath`] — emit an `<img>` whose `src` is a
//!   `data:` URL containing the SVG produced by the RaTeX backend,
//!   matching `sphinx.ext.imgmath`'s "drop in a pre-rendered image"
//!   semantics but without needing `latex`/`dvipng`/`dvisvgm`.
//!
//! Inline vs. display selection follows MathJax convention:
//! `$x$` / `\(x\)` are inline; `$$x$$` / `\[x\]` / `align`/`equation`
//! environments are display (block).

/// Backend used to render LaTeX math to HTML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathBackend {
    /// Pure-Rust SVG rendering via RaTeX. Default.
    #[default]
    Ratex,
    /// Emit raw `\(…\)` / `\[…\]` markers for browser-side MathJax.
    MathJax,
    /// Emit `<img src="data:image/svg+xml;base64,…">` using the RaTeX
    /// SVG output. Parity with Sphinx's `sphinx.ext.imgmath`
    /// extension (image-based rendering) without needing a LaTeX
    /// install.
    ImgMath,
}

impl MathBackend {
    /// Parse a `math_renderer` config string. Accepts the upstream
    /// Sphinx names (`"mathjax"`, `"imgmath"`) plus `"ratex"`. Returns
    /// `None` for unknown values; callers decide whether that is an
    /// error or a fall-through to default.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "ratex" => Some(Self::Ratex),
            "mathjax" | "sphinx.ext.mathjax" => Some(Self::MathJax),
            "imgmath" | "sphinx.ext.imgmath" => Some(Self::ImgMath),
            _ => None,
        }
    }

    /// Stable identifier suitable for `math_renderer` config values
    /// and `<div data-renderer="…">` attributes.
    pub fn name(self) -> &'static str {
        match self {
            Self::Ratex => "ratex",
            Self::MathJax => "mathjax",
            Self::ImgMath => "imgmath",
        }
    }
}

/// Inline vs. display selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathDisplay {
    Inline,
    Block,
}

/// Render `latex` to HTML according to `backend` and `display`.
///
/// The output is always a self-contained HTML fragment safe to splice
/// into the document body (no `<script>` tags, no external file
/// references).
pub fn render(backend: MathBackend, display: MathDisplay, latex: &str) -> String {
    match backend {
        MathBackend::MathJax => render_mathjax(display, latex),
        MathBackend::Ratex => render_ratex(display, latex)
            .unwrap_or_else(|| render_fallback(display, latex)),
        MathBackend::ImgMath => render_imgmath(display, latex)
            .unwrap_or_else(|| render_fallback(display, latex)),
    }
}

fn render_mathjax(display: MathDisplay, latex: &str) -> String {
    let escaped = html_escape_text(latex);
    match display {
        MathDisplay::Inline => format!(
            r#"<span class="math notranslate nohighlight">\({escaped}\)</span>"#
        ),
        MathDisplay::Block => format!(
            r#"<div class="math notranslate nohighlight">\[{escaped}\]</div>"#
        ),
    }
}

/// Fallback used when the RaTeX backend is disabled at compile time
/// or fails to parse the input. Mirrors the historical
/// `<span class="math">` placeholder so downstream rendering can fall
/// back to whatever client-side JS the page already provides.
fn render_fallback(display: MathDisplay, latex: &str) -> String {
    let escaped = html_escape_text(latex);
    match display {
        MathDisplay::Inline => format!(r#"<span class="math">{escaped}</span>"#),
        MathDisplay::Block => format!(r#"<div class="math">{escaped}</div>"#),
    }
}

#[cfg(not(feature = "ratex"))]
fn render_ratex(_: MathDisplay, _: &str) -> Option<String> {
    None
}

#[cfg(feature = "ratex")]
fn render_ratex(display: MathDisplay, latex: &str) -> Option<String> {
    let svg = ratex_svg_for(display, latex)?;
    let class = if display == MathDisplay::Block {
        "math math-block"
    } else {
        "math math-inline"
    };
    Some(format!(
        r#"<span class="{class}" data-renderer="ratex">{svg}</span>"#
    ))
}

#[cfg(not(feature = "ratex"))]
fn render_imgmath(_: MathDisplay, _: &str) -> Option<String> {
    None
}

#[cfg(feature = "ratex")]
fn render_imgmath(display: MathDisplay, latex: &str) -> Option<String> {
    use base64_alt::encode as b64;
    let svg = ratex_svg_for(display, latex)?;
    let encoded = b64(svg.as_bytes());
    let alt = html_escape_attr(latex);
    let class = if display == MathDisplay::Block {
        "math math-block"
    } else {
        "math math-inline"
    };
    let img = format!(
        r#"<img class="{class}" alt="{alt}" src="data:image/svg+xml;base64,{encoded}" data-renderer="imgmath" />"#
    );
    Some(match display {
        MathDisplay::Inline => img,
        MathDisplay::Block => format!(r#"<div class="math math-block">{img}</div>"#),
    })
}

#[cfg(feature = "ratex")]
fn ratex_svg_for(display: MathDisplay, latex: &str) -> Option<String> {
    use ratex_layout::{LayoutOptions, layout, to_display_list};
    use ratex_parser::parse;
    use ratex_svg::{SvgOptions, render_to_svg};
    use ratex_types::MathStyle;

    let nodes = parse(latex).ok()?;
    let mut opts = LayoutOptions::default();
    opts.style = match display {
        MathDisplay::Inline => MathStyle::Text,
        MathDisplay::Block => MathStyle::Display,
    };
    let layout = layout(&nodes, &opts);
    let list = to_display_list(&layout);
    let svg_opts = SvgOptions::default();
    Some(render_to_svg(&list, &svg_opts))
}

fn html_escape_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

fn html_escape_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

/// Tiny stand-in for `base64` — we avoid pulling in a 4th-party crate
/// since the standard library has the bytes we need (encoding 6-bit
/// groups). Used only by the imgmath backend.
#[cfg(feature = "ratex")]
mod base64_alt {
    const TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode(input: &[u8]) -> String {
        let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
        let mut i = 0;
        while i + 3 <= input.len() {
            let b0 = input[i];
            let b1 = input[i + 1];
            let b2 = input[i + 2];
            out.push(TABLE[(b0 >> 2) as usize] as char);
            out.push(TABLE[(((b0 & 0b11) << 4) | (b1 >> 4)) as usize] as char);
            out.push(TABLE[(((b1 & 0b1111) << 2) | (b2 >> 6)) as usize] as char);
            out.push(TABLE[(b2 & 0b0011_1111) as usize] as char);
            i += 3;
        }
        match input.len() - i {
            1 => {
                let b0 = input[i];
                out.push(TABLE[(b0 >> 2) as usize] as char);
                out.push(TABLE[((b0 & 0b11) << 4) as usize] as char);
                out.push('=');
                out.push('=');
            }
            2 => {
                let b0 = input[i];
                let b1 = input[i + 1];
                out.push(TABLE[(b0 >> 2) as usize] as char);
                out.push(TABLE[(((b0 & 0b11) << 4) | (b1 >> 4)) as usize] as char);
                out.push(TABLE[((b1 & 0b1111) << 2) as usize] as char);
                out.push('=');
            }
            _ => {}
        }
        out
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn rfc4648_vectors() {
            assert_eq!(encode(b""), "");
            assert_eq!(encode(b"f"), "Zg==");
            assert_eq!(encode(b"fo"), "Zm8=");
            assert_eq!(encode(b"foo"), "Zm9v");
            assert_eq!(encode(b"foob"), "Zm9vYg==");
            assert_eq!(encode(b"fooba"), "Zm9vYmE=");
            assert_eq!(encode(b"foobar"), "Zm9vYmFy");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_default_is_ratex() {
        assert_eq!(MathBackend::default(), MathBackend::Ratex);
    }

    #[test]
    fn backend_from_name_round_trip() {
        for &b in &[MathBackend::Ratex, MathBackend::MathJax, MathBackend::ImgMath] {
            assert_eq!(MathBackend::from_name(b.name()), Some(b));
        }
        // Accept the upstream sphinx ext.* aliases.
        assert_eq!(
            MathBackend::from_name("sphinx.ext.mathjax"),
            Some(MathBackend::MathJax)
        );
        assert_eq!(
            MathBackend::from_name("sphinx.ext.imgmath"),
            Some(MathBackend::ImgMath)
        );
        assert_eq!(MathBackend::from_name("not-a-renderer"), None);
    }

    #[test]
    fn mathjax_inline_wraps_in_paren_delimiters() {
        let out = render(MathBackend::MathJax, MathDisplay::Inline, "x^2");
        assert!(out.contains("\\(x^2\\)"), "got {out}");
        assert!(out.contains("class=\"math notranslate nohighlight\""));
    }

    #[test]
    fn mathjax_block_wraps_in_bracket_delimiters() {
        let out = render(MathBackend::MathJax, MathDisplay::Block, "x^2");
        assert!(out.contains("\\[x^2\\]"), "got {out}");
    }

    #[test]
    fn mathjax_escapes_html_metachars() {
        let out = render(MathBackend::MathJax, MathDisplay::Inline, "a<b & c>d");
        assert!(out.contains("a&lt;b &amp; c&gt;d"), "got {out}");
    }

    #[cfg(feature = "ratex")]
    #[test]
    fn ratex_inline_emits_svg() {
        let out = render(MathBackend::Ratex, MathDisplay::Inline, "x^2");
        assert!(
            out.contains("<svg") && out.contains("data-renderer=\"ratex\""),
            "got {out}"
        );
    }

    #[cfg(feature = "ratex")]
    #[test]
    fn imgmath_emits_data_url() {
        let out = render(MathBackend::ImgMath, MathDisplay::Inline, "x^2");
        assert!(
            out.contains("data:image/svg+xml;base64,") && out.contains("data-renderer=\"imgmath\""),
            "got {out}"
        );
    }

    #[cfg(feature = "ratex")]
    #[test]
    fn ratex_unparseable_falls_back() {
        // Unbalanced braces — RaTeX parser should reject and we fall
        // back to the placeholder span rather than panicking.
        let out = render(MathBackend::Ratex, MathDisplay::Inline, "\\frac{1}{");
        assert!(out.contains("class=\"math\""), "got {out}");
    }
}
