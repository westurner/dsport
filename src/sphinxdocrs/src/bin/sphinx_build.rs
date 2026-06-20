//! `sphinx-build-rs` — native Rust entry point.
//!
//! Make mode (`-M target src out [opts]`) is dispatched natively.
//! Direct build mode (`-b builder src out`) is delegated to the Python
//! `Sphinx` application until a native builder exists.
//!
//! Falls back entirely to Python when `--use-python-impl` /
//! `SPHINXDOCRS_PY_FALLBACK=1` is set.

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

    // Direct mode: validate args natively, then delegate to Python sphinx.
    // (Native builder takeover is gated on `feature = "native-build"`.)
    let parsed = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // Colour disable via util_console (mirrors _validate_colour_support).
    // util_console::wrap() checks the flag internally; expose a thin
    // wrapper here rather than touching the private static directly.
    let _ = parsed.color; // colour handling deferred until native builder

    // Delegate to Python for the actual build.
    run_python_impl("sphinx.cmd.build", &argv);
}
