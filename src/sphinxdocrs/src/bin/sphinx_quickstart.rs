//! `sphinx-quickstart-rs` — native Rust entry point.
//!
//! Falls back to the upstream Python `sphinx-quickstart` when:
//! - `--use-python-impl` flag is present, or
//! - `SPHINXDOCRS_PY_FALLBACK=1` env var is set.
//!
//! Otherwise uses the fully native Rust port in `sphinxdocrs::quickstart`.

use sphinxdocrs::cli::io::{
    RealFs, RealTerminal, SystemClock, py_fallback_requested, run_python_impl,
};
use sphinxdocrs::quickstart::{
    QuickstartTemplates, ask_user, generate, is_fully_specified, parse_args,
};

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    // Fallback ladder.
    if py_fallback_requested(&argv) {
        run_python_impl("sphinx.cmd.quickstart", &argv);
    }

    let settings = match parse_args(&{
        let mut full = vec!["sphinx-quickstart".to_owned()];
        full.extend(argv.iter().cloned());
        full
    }) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    let term = RealTerminal;
    let fs = RealFs;
    let clock = SystemClock;

    let mut settings = settings;

    // Interactive prompts for missing fields (unless -q + fully specified).
    if !settings.quiet || !is_fully_specified(&settings) {
        ask_user(&mut settings, &term, &fs);
    }

    let templates = QuickstartTemplates::with_templatedir(settings.templatedir.as_deref());

    // Ensure path exists before generate.
    if let Err(e) = std::fs::create_dir_all(&settings.path) {
        eprintln!("Error: cannot create {}: {e}", settings.path.display());
        std::process::exit(1);
    }

    match generate(&settings, &templates, &fs, &clock) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
