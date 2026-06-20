//! `sphinx-autogen-rs` — native Rust entry point.
//!
//! **Native layer**: RST file scanning (`find_autosummary_in_lines`) and
//! argument parsing run in Rust. The stub-file *generation* step (which
//! needs Python object import/introspection via `autodoc`) is currently
//! delegated to the Python `sphinx-autogen`.
//!
//! Falls back entirely to Python when:
//! - `--use-python-impl` flag is present, or
//! - `SPHINXDOCRS_PY_FALLBACK=1` env var is set.

use sphinxdocrs::autogen::{find_autosummary_in_files, parse_args};
use sphinxdocrs::cli::io::{py_fallback_requested, run_python_impl};

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    if py_fallback_requested(&argv) {
        run_python_impl("sphinx.ext.autosummary.generate", &argv);
    }

    // Parse args natively to validate them early.
    let _args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // Native scan: collect autosummary entries from source files.
    // This lets us log what would be generated without Python.
    let entries = find_autosummary_in_files(&_args.source_files);
    if !entries.is_empty() {
        // Log discovered entries (mirrors upstream logger.info call).
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let showed: Vec<&str> = if names.len() > 20 {
            let mut s: Vec<&str> = names[..10].to_vec();
            s.push("...");
            s.extend_from_slice(&names[names.len() - 10..]);
            s
        } else {
            names.clone()
        };
        if _args.output_dir.is_some() {
            eprintln!(
                "[autosummary] generating autosummary for: {}",
                showed.join(", ")
            );
        }
    }

    // Delegate stub generation to Python (requires autodoc import machinery).
    run_python_impl("sphinx.ext.autosummary.generate", &argv);
}
