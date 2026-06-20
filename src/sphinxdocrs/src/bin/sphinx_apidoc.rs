//! `sphinx-apidoc-rs` — native Rust entry point.
//!
//! Falls back to upstream Python `sphinx-apidoc` when:
//! - `--use-python-impl` flag is present, or
//! - `SPHINXDOCRS_PY_FALLBACK=1` env var is set.

use sphinxdocrs::apidoc::generate::remove_old_files;
use sphinxdocrs::apidoc::{ApidocTemplates, create_modules_toc_file, parse_args, recurse_tree};
use sphinxdocrs::cli::io::{RealFs, SystemClock, py_fallback_requested, run_python_impl};
use sphinxdocrs::quickstart::generate::generate as qs_generate;
use sphinxdocrs::quickstart::settings::QuickstartSettings;
use sphinxdocrs::quickstart::templates::QuickstartTemplates;

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
        run_full_quickstart(&opts, &modules);
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

/// Native implementation of `_full_quickstart`: mirrors
/// `sphinx.ext.apidoc._cli._full_quickstart`.
///
/// Builds a `QuickstartSettings` from the apidoc options and calls
/// `quickstart::generate`. Falls back to Python only if the native
/// quickstart path itself fails.
fn run_full_quickstart(opts: &sphinxdocrs::apidoc::ApidocOptions, modules: &[String]) {
    // Build the sorted mastertoctree text (matches upstream logic).
    let mut sorted_mods = modules.to_vec();
    sorted_mods.sort();
    let mut prev = String::new();
    let mut text = String::new();
    for m in &sorted_mods {
        if m.starts_with(&format!("{prev}.")) {
            continue;
        }
        prev = m.clone();
        text.push_str(&format!("   {m}\n"));
    }

    let release = opts
        .release
        .clone()
        .or_else(|| opts.version.clone())
        .unwrap_or_default();

    let mut extensions = vec![
        "sphinx.ext.autodoc".to_owned(),
        "sphinx.ext.viewcode".to_owned(),
        "sphinx.ext.todo".to_owned(),
    ];
    extensions.extend(opts.extensions.clone());

    let qs = QuickstartSettings {
        path: opts.dest_dir.clone(),
        sep: false,
        dot: "_".to_owned(),
        project: opts.header.clone(),
        author: opts.author.clone().unwrap_or_else(|| "Author".to_owned()),
        version: opts.version.clone().unwrap_or_default(),
        release,
        suffix: format!(".{}", opts.suffix),
        master: "index".to_owned(),
        extensions,
        makefile: true,
        batchfile: true,
        quiet: opts.quiet,
        language: Some("en".to_owned()),
        templatedir: opts.template_dir.clone(),
    };

    if let Err(e) = std::fs::create_dir_all(&qs.path) {
        eprintln!("Error creating {}: {e}", qs.path.display());
        std::process::exit(1);
    }

    let qs_templates = QuickstartTemplates::with_templatedir(opts.template_dir.as_deref());
    let clock = SystemClock;

    if let Err(e) = qs_generate(&qs, &qs_templates, &RealFs, &clock) {
        eprintln!("Error in --full quickstart: {e}");
        std::process::exit(1);
    }

    // Also write the mastertoctree into the index file if non-empty.
    if !text.is_empty() {
        let index = qs.path.join(format!("index.{}", opts.suffix));
        if let Ok(existing) = std::fs::read_to_string(&index) {
            // Append toctree entries to the existing index (best effort).
            let updated = existing.replace("{{ mastertoctree }}", &text);
            let _ = std::fs::write(&index, updated);
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
