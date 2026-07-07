//! `sphinx-build-rs` — native Rust entry point.
//!
//! Make mode (`-M target src out [opts]`) is dispatched natively via
//! [`NativeMakeRunner`], which calls the Rust [`SphinxApp`] directly for
//! native builders (html / latex / man) and falls back to Python for others.
//!
//! Direct build mode (`-b builder src out`) uses [`SphinxApp`] directly when
//! `builder` is in [`NATIVE_BUILDERS`].
//!
//! Falls back entirely to Python when `--use-python-impl` /
//! `SPHINXDOCRS_PY_FALLBACK=1` is set.

use std::collections::HashMap;

use sphinxdocrs::application::{SphinxApp, is_native_builder};
use sphinxdocrs::build::NativeMakeRunner;
use sphinxdocrs::build::make_mode::run_make_mode;
use sphinxdocrs::build::parse_args;
use sphinxdocrs::cli::io::{py_fallback_requested, run_python_impl};
use sphinxdocrs::scan::scan_requirements;

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    // Global fallback.
    if py_fallback_requested(&argv) {
        eprintln!("sphinxdocrs: running python: sphinx.cmd.build");
        run_python_impl("sphinx.cmd.build", &argv);
    }

    // Make-mode: `sphinx-build -M target src out [opts...]`
    //
    // Uses NativeMakeRunner so native builders (html/latex/man) run the Rust
    // SphinxApp directly instead of re-spawning the Python sphinx-build.
    if argv.first().map(|s| s.as_str()) == Some("-M") {
        let make_args = argv[1..].to_vec();
        let py_fallback = py_fallback_requested(&argv);
        let runner = NativeMakeRunner::new(py_fallback);
        let code = run_make_mode(&make_args, &runner);
        std::process::exit(code);
    }

    // Direct mode: validate args natively.
    let parsed = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // --scan-requirements: inspect conf.py and exit before doing any build.
    if parsed.scan_requirements {
        let conf_root = parsed.confdir.as_deref().unwrap_or(&parsed.sourcedir);
        // Use sourcedir's parent as the project root so requirement files in
        // the project tree are found.
        let project_root = parsed
            .sourcedir
            .parent()
            .unwrap_or(&parsed.sourcedir)
            .to_path_buf();
        let result = scan_requirements(conf_root, &project_root);
        print!("{}", result.report());
        std::process::exit(if result.all_present() { 0 } else { 1 });
    }

    // If the builder has a native Rust implementation, use SphinxApp.
    if is_native_builder(&parsed.builder) && !py_fallback_requested(&argv) {
        let srcdir = parsed.sourcedir.clone();
        let outdir = parsed.outputdir.clone();
        let doctreedir = parsed.doctreedir.clone();

        // Collect -D overrides (convert ConfValue to String).
        let overrides: HashMap<String, String> = parsed
            .confoverrides
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect();

        eprintln!("sphinxdocrs: running SphinxApp::new");
        match SphinxApp::new(&srcdir, &outdir, &doctreedir, &parsed.builder, overrides) {
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
            Ok(app) => match app.build() {
                Ok(result) => {
                    eprintln!("Build succeeded: {} file(s) written.", result.written);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Build error: {e}");
                    std::process::exit(1);
                }
            },
        }
    }

    // Fall back to Python for all other builders.
    eprintln!("sphinxdocrs: no rust builder found. running python: sphinx.cmd.build");
    run_python_impl("sphinx.cmd.build", &argv);
}
