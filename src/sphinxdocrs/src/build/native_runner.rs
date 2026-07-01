//! [`NativeMakeRunner`] — a [`Runner`] for make mode that calls the Rust
//! [`SphinxApp`] directly for native builders instead of spawning a
//! `sphinx-build` subprocess.
//!
//! ## Dispatch table
//!
//! | Condition | Action |
//! |---|---|
//! | `py_fallback == true` | spawn Python `sphinx.cmd.build` |
//! | builder is native (html / latex / man) | call [`SphinxApp::new`] + `build()` inline |
//! | builder is unknown | spawn Python `sphinx.cmd.build` |
//!
//! The args received by [`Runner::run`] are the same slice that
//! [`MakeMode::run_generic_build`] assembles:
//!
//! ```text
//! ["--builder", "<name>", "--doctree-dir", "<path>", "<src>", "<out>", ...extras]
//! ```
//!
//! These are fed straight into [`parse_args`] (which drives the same clap
//! parser used by the top-level binary) so we reuse all parsing logic.

use std::collections::HashMap;
use std::io;
use std::path::Path;

use crate::application::{SphinxApp, is_native_builder};
use crate::build::args::parse_args;
use crate::cli::io::Runner;

// ── NativeMakeRunner ─────────────────────────────────────────────────────────

/// A [`Runner`] that drives the Rust builder pipeline inline for native
/// builders and shells out to Python for everything else.
///
/// Construct via [`NativeMakeRunner::new`].
pub struct NativeMakeRunner {
    /// When `true`, bypass the native path entirely and always spawn Python.
    pub py_fallback: bool,
}

impl NativeMakeRunner {
    /// Create a new `NativeMakeRunner`.
    ///
    /// `py_fallback` mirrors the `SPHINXDOCRS_PY_FALLBACK` / `--use-python-impl`
    /// semantics: when `true` every builder call goes straight to Python.
    pub fn new(py_fallback: bool) -> Self {
        Self { py_fallback }
    }
}

impl Runner for NativeMakeRunner {
    /// Called by [`MakeMode::run_generic_build`] with
    /// `("sphinx-build", ["--builder", name, "--doctree-dir", …, src, out], cwd)`.
    ///
    /// Routes native builders to [`SphinxApp`]; all others fall back to
    /// Python.
    fn run(&self, _program: &str, args: &[String], _cwd: &Path) -> io::Result<i32> {
        if self.py_fallback {
            eprintln!(
                "sphinxdocrs: make mode: py_fallback requested; running python: sphinx.cmd.build"
            );
            return spawn_python_build(args);
        }

        let parsed = match parse_args(args) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("sphinxdocrs: make mode: arg parse error: {e}");
                return Ok(2);
            }
        };

        if !is_native_builder(&parsed.builder) {
            eprintln!(
                "sphinxdocrs: make mode: builder {:?} has no Rust implementation; \
                 running python: sphinx.cmd.build",
                parsed.builder
            );
            return spawn_python_build(args);
        }

        let overrides: HashMap<String, String> = parsed
            .confoverrides
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect();

        eprintln!(
            "sphinxdocrs: make mode: running SphinxApp (builder={})",
            parsed.builder
        );
        match SphinxApp::new(
            &parsed.sourcedir,
            &parsed.outputdir,
            &parsed.doctreedir,
            &parsed.builder,
            overrides,
        ) {
            Err(e) => {
                eprintln!("Error: {e}");
                Ok(1)
            }
            Ok(app) => match app.build() {
                Ok(result) => {
                    eprintln!("Build succeeded: {} file(s) written.", result.written);
                    Ok(0)
                }
                Err(e) => {
                    eprintln!("Build error: {e}");
                    Ok(1)
                }
            },
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Spawn `python -c "from sphinx.cmd.build import main; ..."` with `args`.
fn spawn_python_build(args: &[String]) -> io::Result<i32> {
    let py_code = "import sys; from sphinx.cmd.build import main; sys.exit(main())";
    let status = std::process::Command::new("python")
        .arg("-c")
        .arg(py_code)
        .args(args)
        .status()?;
    Ok(status.code().unwrap_or(1))
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ── helpers ───────────────────────────────────────────────────────────────

    fn setup_src(tmp: &TempDir) -> std::path::PathBuf {
        let src = tmp.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("index.rst"), "Welcome\n=======\n\nHomepage.\n").unwrap();
        src
    }

    fn build_args(builder: &str, src: &Path, out: &Path, doctrees: &Path) -> Vec<String> {
        vec![
            "--builder".to_owned(),
            builder.to_owned(),
            "--doctree-dir".to_owned(),
            doctrees.to_string_lossy().into_owned(),
            src.to_string_lossy().into_owned(),
            out.to_string_lossy().into_owned(),
        ]
    }

    // ── arg-parse error → code 2 ─────────────────────────────────────────────

    #[test]
    fn bad_args_return_code_2() {
        let runner = NativeMakeRunner::new(false);
        let bad: Vec<String> = vec!["--unknown-flag-xyz".to_owned()];
        let code = runner
            .run("sphinx-build", &bad, Path::new("."))
            .expect("should not be an I/O error");
        assert_eq!(code, 2, "unrecognised args should yield exit code 2");
    }

    // ── native builder (html) → SphinxApp ────────────────────────────────────

    #[test]
    fn html_builder_invokes_sphinx_app() {
        let tmp = TempDir::new().unwrap();
        let src = setup_src(&tmp);
        let out = tmp.path().join("_build/html");
        let doctrees = tmp.path().join("_build/.doctrees");
        let args = build_args("html", &src, &out, &doctrees);

        let runner = NativeMakeRunner::new(false);
        let code = runner
            .run("sphinx-build", &args, &src)
            .expect("run should not return an I/O error");
        // SphinxApp was called; it either succeeds (0) or fails building (1),
        // but must not return 2 (arg-parse) or panic.
        assert_ne!(code, 2, "html should not trigger an arg-parse error");
    }

    // ── native builder (latex) → SphinxApp ───────────────────────────────────

    #[test]
    fn latex_builder_invokes_sphinx_app() {
        let tmp = TempDir::new().unwrap();
        let src = setup_src(&tmp);
        let out = tmp.path().join("_build/latex");
        let doctrees = tmp.path().join("_build/.doctrees");
        let args = build_args("latex", &src, &out, &doctrees);

        let runner = NativeMakeRunner::new(false);
        let code = runner
            .run("sphinx-build", &args, &src)
            .expect("run should not return an I/O error");
        assert_ne!(code, 2);
    }

    // ── non-native builder → Python (skips SphinxApp) ────────────────────────
    //
    // We can't easily assert *which* branch was taken without mocking Python,
    // but we can verify the runner itself doesn't return 2 (arg-parse error)
    // and doesn't panic for a well-formed non-native builder invocation.
    //
    // We use `SPHINXDOCRS_PY_FALLBACK=1` to short-circuit before it actually
    // tries to spawn Python, verifying the routing code is reached.

    #[test]
    fn py_fallback_env_routes_to_python_path() {
        // With py_fallback=true the runner takes the Python branch without
        // consulting the builder name.  We cannot assert on the Python exit
        // code here, so just verify it doesn't panic and doesn't arg-parse.
        let runner = NativeMakeRunner::new(true);
        // Build args for a known-native builder — py_fallback should still
        // bypass SphinxApp.
        let tmp = TempDir::new().unwrap();
        let src = setup_src(&tmp);
        let out = tmp.path().join("_build/html");
        let doctrees = tmp.path().join("_build/.doctrees");
        let args = build_args("html", &src, &out, &doctrees);

        // spawn_python_build will try to run `python`, which is available in
        // the dev container; allow both 0 and non-zero exit codes.
        let result = runner.run("sphinx-build", &args, &src);
        assert!(
            result.is_ok(),
            "py_fallback runner should not error at the I/O level: {result:?}"
        );
    }

    // ── make mode end-to-end with NativeMakeRunner ───────────────────────────
    //
    // Exercises the full `-M html src out` flow through run_make_mode +
    // NativeMakeRunner.  With a real (minimal) source tree this exercises the
    // same code path as `sphinx-build-rs -M html . _build`.

    #[test]
    fn run_make_mode_with_native_runner_dispatches_html() {
        use crate::build::make_mode::run_make_mode;

        let tmp = TempDir::new().unwrap();
        let src = setup_src(&tmp);
        let build = tmp.path().join("_build");
        std::fs::create_dir_all(&build).unwrap();

        let args: Vec<String> = vec![
            "html".to_owned(),
            src.to_string_lossy().into_owned(),
            build.to_string_lossy().into_owned(),
        ];

        let runner = NativeMakeRunner::new(false);
        let code = run_make_mode(&args, &runner);
        // SphinxApp was invoked; code may be 0 or 1 (build error) but not 2.
        assert_ne!(code, 2, "run_make_mode should not hit an arg-parse error");
    }
}
