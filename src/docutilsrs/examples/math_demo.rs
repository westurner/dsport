//! Parse a small rST snippet containing both `:math:` role and
//! `.. math::` directive, and write HTML5 + pseudo-XML outputs through
//! the Rust API (so the math arms go through `mathrenderrs`).
//!
//! Usage (sphinx-build style):
//!
//! ```text
//! math_demo -o OUTDIR <input.rst>
//! ```
//!
//! Invoked by `make demo-math-docutils`.

use std::path::PathBuf;

use docutilsrs::{html5, parse_rst, pseudo_xml};

fn usage() -> ! {
    eprintln!("usage: math_demo -o OUTDIR <input.rst>");
    std::process::exit(2);
}

fn main() {
    let mut outdir: Option<PathBuf> = None;
    let mut input: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-o" | "--outdir" => {
                outdir = Some(args.next().unwrap_or_else(|| usage()).into());
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
    let tree = parse_rst(&src);

    let html_path = outdir.join("output.html");
    std::fs::write(&html_path, html5(&tree))
        .unwrap_or_else(|e| panic!("write {}: {e}", html_path.display()));
    println!("wrote {}", html_path.display());

    let px_path = outdir.join("output.pseudoxml");
    std::fs::write(&px_path, pseudo_xml(&tree))
        .unwrap_or_else(|e| panic!("write {}: {e}", px_path.display()));
    println!("wrote {}", px_path.display());
}
