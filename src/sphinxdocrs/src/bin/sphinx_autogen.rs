//! `sphinx-autogen-rs` — native Rust entry point.
//!
//! **C4.2**: Full native implementation:
//! 1. Parse arguments (clap, mirroring upstream).
//! 2. Scan RST source files for `.. autosummary::` directives.
//! 3. Generate `.rst` stub files natively from the scanned entries —
//!    using heuristic type detection (CamelCase → class; otherwise → module).
//!    Member lists are left empty; `autodoc` populates them at build time.
//!
//! Delegates to the upstream Python `sphinx-autogen` **only** when:
//! - `--use-python-impl` flag is present, or
//! - `SPHINXDOCRS_PY_FALLBACK=1` env var is set.

use std::path::Path;

use sphinxdocrs::autogen::{
    AutogenTemplates, find_autosummary_in_files, generate_stubs, parse_args,
};
use sphinxdocrs::cli::io::{py_fallback_requested, run_python_impl};

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    if py_fallback_requested(&argv) {
        run_python_impl("sphinx.ext.autosummary.generate", &argv);
    }

    // Parse args natively to validate early and extract options.
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // Native scan: collect autosummary entries from source files.
    let entries = find_autosummary_in_files(&args.source_files);

    if entries.is_empty() {
        // Nothing to do.
        return;
    }

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
    eprintln!(
        "[autosummary] generating autosummary for: {}",
        showed.join(", ")
    );

    // Determine output directory: prefer -o/--output-dir; fall back to the
    // toctree path in the first entry that has one.
    let output_dir = args.output_dir.clone().or_else(|| {
        entries
            .iter()
            .find_map(|e| e.toctree.as_deref().map(std::path::PathBuf::from))
    });

    let output_dir = match output_dir {
        Some(d) => d,
        None => {
            eprintln!(
                "[autosummary] no output directory specified and no :toctree: found; \
                 use -o OUTPUT_DIR to specify one"
            );
            std::process::exit(1);
        }
    };

    if output_dir != Path::new("") {
        eprintln!("[autosummary] writing to {}", output_dir.display());
    }

    let templates = AutogenTemplates::with_templatedir(args.templates.as_deref());

    let suffix = &args.suffix;
    let generated = generate_stubs(
        &entries,
        &output_dir,
        suffix,
        /*overwrite=*/ true,
        args.remove_old,
        &templates,
    );

    if !generated.is_empty() {
        eprintln!(
            "[autosummary] wrote {} stub file(s) to {}",
            generated.len(),
            output_dir.display()
        );
    }
}
