//! Render the same expression with each `MathBackend` and write the
//! resulting HTML fragments.
//!
//! Usage (sphinx-build style):
//!
//! ```text
//! backend_matrix [-o OUTDIR]
//! ```
//!
//! With `-o OUTDIR`, writes `OUTDIR/backends.html`. Without `-o`,
//! prints to stdout. Invoked by `make demo-math-mathrenderrs`.

use std::path::PathBuf;

use mathrenderrs::{render, MathBackend, MathDisplay};

fn build_html() -> String {
    use std::fmt::Write;
    let inline = "E = mc^2";
    let block = r"\int_0^1 x^2 \, dx = \frac{1}{3}";
    let mut s = String::new();
    let _ = writeln!(
        s,
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>mathrenderrs backends</title></head><body>"
    );
    for backend in [MathBackend::Ratex, MathBackend::MathJax, MathBackend::ImgMath] {
        let _ = writeln!(s, "<h2>backend = {}</h2>", backend.name());
        let _ = writeln!(
            s,
            "<p>inline: {}</p>",
            render(backend, MathDisplay::Inline, inline)
        );
        let _ = writeln!(
            s,
            "<div>display: {}</div>",
            render(backend, MathDisplay::Block, block)
        );
    }
    let _ = writeln!(s, "</body></html>");
    s
}

fn main() {
    let mut outdir: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-o" | "--outdir" => {
                outdir = Some(args.next().expect("-o requires OUTDIR").into());
            }
            "-h" | "--help" => {
                eprintln!("usage: backend_matrix [-o OUTDIR]");
                std::process::exit(2);
            }
            _ => {
                eprintln!("unexpected argument: {a}");
                std::process::exit(2);
            }
        }
    }

    let html = build_html();
    match outdir {
        Some(dir) => {
            std::fs::create_dir_all(&dir)
                .unwrap_or_else(|e| panic!("mkdir {}: {e}", dir.display()));
            let path = dir.join("backends.html");
            std::fs::write(&path, html)
                .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
            println!("wrote {}", path.display());
        }
        None => print!("{html}"),
    }
}
