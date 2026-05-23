//! Render a MyST Markdown input file with each available math backend
//! and write `<backend>.html` files into the given output directory.
//!
//! Usage (sphinx-build style):
//!
//! ```text
//! math_demo -o OUTDIR <input.md>
//! ```
//!
//! Invoked by `make demo-math-myst`.

use std::path::PathBuf;

use myst_md_rs::{render_html_with, MathBackend};

/// Wrap a body fragment in a minimal standalone HTML page. For the
/// MathJax backend we emit a `<script defer>` tag in the `<head>` so
/// the page actually loads MathJax at view time (otherwise the inline
/// `\(…\)` / `\[…\]` markers stay literal).
///
/// `mathjax_src` should be the URL configured in `conf.py`'s
/// `mathjax_path` (Sphinx default:
/// `https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js`).
/// `integrity_attr`, when `Some`, is rendered verbatim as the
/// `integrity="…"` SRI attribute on the `<script>` tag — pair this
/// with `sphinxdocrs::assets::sri_hash_file` to mirror the
/// `app.add_js_file(..., integrity=...)` Sphinx API.
fn wrap_page(
    title: &str,
    backend: MathBackend,
    body: &str,
    mathjax_src: &str,
    integrity_attr: Option<&str>,
) -> String {
    let head_extra = match backend {
        MathBackend::MathJax => {
            let integrity = integrity_attr
                .map(|s| format!(r#" integrity="{s}" crossorigin="anonymous""#))
                .unwrap_or_default();
            format!(
                r#"<script>window.MathJax = {{ tex: {{ inlineMath: [['\\(','\\)']], displayMath: [['\\[','\\]']] }} }};</script>
<script defer src="{mathjax_src}"{integrity}></script>
"#
            )
        }
        _ => String::new(),
    };
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
{head_extra}</head>
<body>
{body}
</body>
</html>
"#
    )
}

fn usage() -> ! {
    eprintln!(
        "usage: math_demo -o OUTDIR [--mathjax-src URL] [--mathjax-integrity SRI] <input.md>"
    );
    std::process::exit(2);
}

fn main() {
    let mut outdir: Option<PathBuf> = None;
    let mut input: Option<PathBuf> = None;
    let mut mathjax_src =
        "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js".to_string();
    let mut integrity: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-o" | "--outdir" => {
                outdir = Some(args.next().unwrap_or_else(|| usage()).into());
            }
            "--mathjax-src" => {
                mathjax_src = args.next().unwrap_or_else(|| usage());
            }
            "--mathjax-integrity" => {
                integrity = Some(args.next().unwrap_or_else(|| usage()));
            }
            "-h" | "--help" => usage(),
            _ if input.is_none() => input = Some(a.into()),
            _ => usage(),
        }
    }
    let input = input.unwrap_or_else(|| usage());
    let outdir = outdir.unwrap_or_else(|| usage());
    std::fs::create_dir_all(&outdir)
        .unwrap_or_else(|e| panic!("mkdir {}: {e}", outdir.display()));

    let src = std::fs::read_to_string(&input)
        .unwrap_or_else(|e| panic!("read {}: {e}", input.display()));

    for backend in [MathBackend::Ratex, MathBackend::MathJax, MathBackend::ImgMath] {
        let body = render_html_with(&src, backend);
        let title = format!("myst-md-rs math demo ({})", backend.name());
        let html = wrap_page(&title, backend, &body, &mathjax_src, integrity.as_deref());
        let path = outdir.join(format!("{}.html", backend.name()));
        std::fs::write(&path, html)
            .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        println!("wrote {}", path.display());
    }
}
