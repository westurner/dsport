//! `sphinx-apidoc-rs` — native Rust entry point.
//!
//! Falls back to upstream Python `sphinx-apidoc` when:
//! - `--use-python-impl` flag is present, or
//! - `SPHINXDOCRS_PY_FALLBACK=1` env var is set.

use sphinxdocrs::apidoc::generate::remove_old_files;
use sphinxdocrs::apidoc::{ApidocTemplates, create_modules_toc_file, parse_args, recurse_tree};
use sphinxdocrs::cli::io::{py_fallback_requested, run_python_impl};

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    if py_fallback_requested(&argv) {
        run_python_impl("sphinx.ext.apidoc", &argv);
    }

    let opts = match parse_args(&argv) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // Validate module_path
    if !opts.module_path.is_dir() {
        eprintln!("Error: {} is not a directory.", opts.module_path.display());
        std::process::exit(1);
    }

    // Ensure output dir exists
    if !opts.dry_run {
        if let Err(e) = std::fs::create_dir_all(&opts.dest_dir) {
            eprintln!("Error: cannot create {}: {e}", opts.dest_dir.display());
            std::process::exit(1);
        }
    }

    // Compile exclude patterns
    let excludes: Vec<regex::Regex> = opts
        .exclude_pattern
        .iter()
        .filter_map(|pat| {
            let resolved = std::path::Path::new(pat)
                .canonicalize()
                .unwrap_or_else(|_| std::path::PathBuf::from(pat));
            let fnmatch_re = fnmatch_to_regex(&resolved.to_string_lossy());
            regex::Regex::new(&fnmatch_re).ok()
        })
        .collect();

    let templates = ApidocTemplates::with_templatedir(opts.template_dir.as_deref());

    let (mut written_files, modules) =
        match recurse_tree(&opts.module_path, &excludes, &opts, &templates) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        };

    if opts.full {
        // Delegate --full to the Python quickstart bridge for now.
        run_python_impl("sphinx.ext.apidoc", &argv);
    } else if !opts.toc_file.is_empty() {
        match create_modules_toc_file(&modules, &opts, &opts.toc_file, &templates) {
            Ok(path) => written_files.push(path),
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }

    if opts.remove_old && !opts.dry_run {
        if let Err(e) = remove_old_files(&written_files, &opts.dest_dir, &opts.suffix) {
            eprintln!("Warning during remove-old: {e}");
        }
    }
}

/// Translate an fnmatch pattern to a regex string (simplified).
fn fnmatch_to_regex(pat: &str) -> String {
    let mut re = String::from("(?s)");
    for ch in pat.chars() {
        match ch {
            '*' => re.push_str(".*"),
            '?' => re.push('.'),
            '.' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '+' | '\\' => {
                re.push('\\');
                re.push(ch);
            }
            c => re.push(c),
        }
    }
    re
}
