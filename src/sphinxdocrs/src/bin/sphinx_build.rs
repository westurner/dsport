//! `sphinx-build-rs` — native Rust entry point.
//!
//! Make mode (`-M target src out [opts]`) is dispatched natively.
//! Direct build mode (`-b builder src out`) uses the native [`SphinxApp`]
//! when `builder` is in [`NATIVE_BUILDERS`] (currently `"html"`).
//!
//! Falls back entirely to Python when `--use-python-impl` /
//! `SPHINXDOCRS_PY_FALLBACK=1` is set.

use std::collections::HashMap;

use sphinxdocrs::application::{SphinxApp, is_native_builder};
use sphinxdocrs::build::make_mode::run_make_mode;
use sphinxdocrs::build::parse_args;
use sphinxdocrs::cli::io::{ProcessRunner, py_fallback_requested, run_python_impl};

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    // Global fallback.
    if py_fallback_requested(&argv) {
        run_python_impl("sphinx.cmd.build", &argv);
    }

    // Make-mode: `sphinx-build -M target src out [opts...]`
    if argv.first().map(|s| s.as_str()) == Some("-M") {
        let make_args = argv[1..].to_vec();
        let runner = ProcessRunner;
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
    run_python_impl("sphinx.cmd.build", &argv);
}
