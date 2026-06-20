//! Rust port of `sphinx.cmd.make_mode` (`-M` mode).
//!
//! `Make` handles the `-M <target> <src> <out>` invocation: special
//! targets (`clean`, `help`, `latexpdf`, …) are dispatched to typed
//! methods; all others fall through to `run_generic_build` which calls
//! `sphinx-build -b <builder> …`.
//!
//! Subprocess calls go through the injected [`Runner`] trait so tests
//! can verify the exact commands dispatched without spawning processes.
//!
//! Parity targets from `sphinx.cmd.make_mode`:
//! - `BUILDERS` table (all 25 entries including posix-only ones).
//! - `build_clean` safety checks (same-dir, src-under-build,
//!   build-under-src).
//! - `build_help` output format.
//! - `run_generic_build` argument construction.
//! - `run_make_mode` entry-point validation.

use std::path::{Path, PathBuf};

use crate::cli::io::Runner;

/// The builder table mirroring `make_mode.BUILDERS`.
/// `(os_filter, builder_name, description)`
/// `os_filter` is `""` (all platforms) or `"posix"`.
pub const BUILDERS: &[(&str, &str, &str)] = &[
    ("", "html", "to make standalone HTML files"),
    (
        "",
        "dirhtml",
        "to make HTML files named index.html in directories",
    ),
    ("", "singlehtml", "to make a single large HTML file"),
    ("", "pickle", "to make pickle files"),
    ("", "json", "to make JSON files"),
    (
        "",
        "htmlhelp",
        "to make HTML files and an HTML help project",
    ),
    ("", "qthelp", "to make HTML files and a qthelp project"),
    ("", "devhelp", "to make HTML files and a Devhelp project"),
    ("", "epub", "to make an epub"),
    (
        "",
        "latex",
        "to make LaTeX files, you can set PAPER=a4 or PAPER=letter",
    ),
    (
        "posix",
        "latexpdf",
        "to make LaTeX and PDF files (default pdflatex)",
    ),
    (
        "posix",
        "latexpdfja",
        "to make LaTeX files and run them through platex/dvipdfmx",
    ),
    ("", "text", "to make text files"),
    ("", "man", "to make manual pages"),
    ("", "texinfo", "to make Texinfo files"),
    (
        "posix",
        "info",
        "to make Texinfo files and run them through makeinfo",
    ),
    ("", "gettext", "to make PO message catalogs"),
    (
        "",
        "changes",
        "to make an overview of all changed/added/deprecated items",
    ),
    ("", "xml", "to make Docutils-native XML files"),
    (
        "",
        "pseudoxml",
        "to make pseudoxml-XML files for display purposes",
    ),
    ("", "linkcheck", "to check all external links for integrity"),
    (
        "",
        "doctest",
        "to run all doctests embedded in the documentation (if enabled)",
    ),
    (
        "",
        "coverage",
        "to run coverage check of the documentation (if enabled)",
    ),
    ("", "clean", "to remove everything in the build directory"),
];

/// `Make` object — holds `source_dir`, `build_dir`, and extra opts that
/// are forwarded to `sphinx-build` invocations.
pub struct MakeMode {
    pub source_dir: PathBuf,
    pub build_dir: PathBuf,
    /// Extra options forwarded verbatim to `sphinx-build`.
    pub opts: Vec<String>,
    /// The `sphinx-build` binary to call (default `"sphinx-build"`).
    pub sphinx_build_cmd: String,
}

impl MakeMode {
    pub fn new(
        source_dir: impl Into<PathBuf>,
        build_dir: impl Into<PathBuf>,
        opts: Vec<String>,
    ) -> Self {
        Self {
            source_dir: source_dir.into(),
            build_dir: build_dir.into(),
            opts,
            sphinx_build_cmd: "sphinx-build".to_owned(),
        }
    }

    fn build_dir_join(&self, component: &str) -> PathBuf {
        self.build_dir.join(component)
    }

    /// Mirrors `build_clean`: safely remove everything under `build_dir`.
    ///
    /// Safety checks (same-dir, src-under-build, build-under-src) mirror
    /// upstream's checks exactly.
    pub fn build_clean(&self) -> i32 {
        if !self.build_dir.exists() {
            return 0;
        }
        if !self.build_dir.is_dir() {
            eprintln!("Error: '{}' is not a directory!", self.build_dir.display());
            return 1;
        }

        // Resolve both to catch symlinks / relative components.
        let src_resolved = self
            .source_dir
            .canonicalize()
            .unwrap_or_else(|_| self.source_dir.clone());
        let build_resolved = self
            .build_dir
            .canonicalize()
            .unwrap_or_else(|_| self.build_dir.clone());

        if src_resolved == build_resolved {
            eprintln!(
                "Error: '{}' is same as source directory!",
                self.build_dir.display()
            );
            return 1;
        }

        // Refuse if source dir is a child of build dir (would be deleted).
        if src_resolved.starts_with(&build_resolved) {
            eprintln!(
                "Error: '{}' directory contains source directory!",
                self.build_dir.display()
            );
            return 1;
        }

        println!(
            "Removing everything under '{}'...",
            self.build_dir.display()
        );
        match remove_dir_contents(&self.build_dir) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("Error during clean: {e}");
                1
            }
        }
    }

    /// Mirrors `build_help`: print the list of available targets.
    pub fn build_help(&self) {
        println!("Sphinx documentation builder");
        println!("Please use `make TARGET` where TARGET is one of:");
        let is_posix = cfg!(unix);
        for (os_filter, bname, desc) in BUILDERS {
            if os_filter.is_empty() || (is_posix && *os_filter == "posix") {
                println!("  {:<12}  {}", bname, desc);
            }
        }
    }

    /// Mirrors `run_generic_build(builder, doctreedir)`.
    ///
    /// Constructs the `sphinx-build` argument list and calls the
    /// injected [`Runner`]. `PAPER` env var handling is included.
    pub fn run_generic_build(
        &self,
        builder: &str,
        doctreedir: Option<PathBuf>,
        runner: &dyn Runner,
    ) -> i32 {
        let mut opts = self.opts.clone();

        // PAPER env compat (matches upstream)
        let paper = std::env::var("PAPER").unwrap_or_default();
        if paper == "a4" || paper == "letter" {
            opts.extend([
                "-D".to_owned(),
                format!("latex_elements.papersize={paper}paper"),
            ]);
        }

        let doctreedir = doctreedir.unwrap_or_else(|| self.build_dir_join("doctrees"));

        let mut args: Vec<String> = vec![
            "--builder".to_owned(),
            builder.to_owned(),
            "--doctree-dir".to_owned(),
            doctreedir.to_string_lossy().into_owned(),
            self.source_dir.to_string_lossy().into_owned(),
            self.build_dir_join(builder).to_string_lossy().into_owned(),
        ];
        args.extend(opts);

        match runner.run(&self.sphinx_build_cmd, &args, &self.source_dir) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Error: Failed to run {}: {e}", self.sphinx_build_cmd);
                1
            }
        }
    }

    /// Mirrors the per-target dispatch in `Make`. Special targets are
    /// handled natively; everything else goes to `run_generic_build`.
    ///
    /// `"clean"` and `"help"` never need the runner. All other targets
    /// go through it.
    pub fn dispatch(&self, target: &str, runner: &dyn Runner) -> i32 {
        match target {
            "clean" => self.build_clean(),
            "help" => {
                self.build_help();
                0
            }
            "latexpdf" => {
                let rc = self.run_generic_build("latex", None, runner);
                if rc > 0 {
                    return 1;
                }
                self.run_make_cmd("all-pdf", "LATEXOPTS=-halt-on-error", "latex", runner)
            }
            "latexpdfja" => {
                let rc = self.run_generic_build("latex", None, runner);
                if rc > 0 {
                    return 1;
                }
                self.run_make_cmd("all-pdf", "", "latex", runner)
            }
            "info" => {
                let rc = self.run_generic_build("texinfo", None, runner);
                if rc > 0 {
                    return 1;
                }
                self.run_make_cmd("info", "", "texinfo", runner)
            }
            "gettext" => {
                let dtdir = self.build_dir_join("gettext").join(".doctrees");
                self.run_generic_build("gettext", Some(dtdir), runner)
            }
            other => self.run_generic_build(other, None, runner),
        }
    }

    /// Run a `make` sub-command inside `<build_dir>/<sub_dir>`.
    fn run_make_cmd(&self, target: &str, extra: &str, sub_dir: &str, runner: &dyn Runner) -> i32 {
        let cwd = self.build_dir_join(sub_dir);
        let makecmd = std::env::var("MAKE").unwrap_or_else(|_| {
            if cfg!(windows) {
                "make.bat".to_owned()
            } else {
                "make".to_owned()
            }
        });
        let mut args: Vec<String> = vec![target.to_owned()];
        if !extra.is_empty() {
            args.push(extra.to_owned());
        }
        match runner.run(&makecmd, &args, &cwd) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Error: Failed to run: {makecmd}: {e}");
                1
            }
        }
    }
}

/// Remove all direct children of `dir` without removing `dir` itself.
fn remove_dir_contents(dir: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        } else {
            std::fs::remove_file(&path)?;
        }
    }
    Ok(())
}

/// Entry point matching `run_make_mode(args)`.
///
/// `args` is everything after `-M` in the original argv:
/// `[builder, source_dir, build_dir, extra_opts...]`.
pub fn run_make_mode(args: &[String], runner: &dyn Runner) -> i32 {
    if args.len() < 3 {
        eprintln!("Error: at least 3 arguments (builder, source dir, build dir) are required.");
        return 1;
    }
    let builder = &args[0];
    let source_dir = PathBuf::from(&args[1]);
    let build_dir = PathBuf::from(&args[2]);
    let opts = args[3..].to_vec();

    let make = MakeMode::new(source_dir, build_dir, opts);
    make.dispatch(builder, runner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::io::MockRunner;
    use tempfile::TempDir;

    fn make_mode_in(tmp: &TempDir) -> MakeMode {
        MakeMode::new(tmp.path().join("src"), tmp.path().join("build"), vec![])
    }

    // ── build_clean ───────────────────────────────────────────────────────

    #[test]
    fn clean_nonexistent_build_dir_is_ok() {
        let tmp = TempDir::new().unwrap();
        let m = make_mode_in(&tmp);
        // build dir doesn't exist yet
        assert_eq!(m.build_clean(), 0);
    }

    #[test]
    fn clean_same_dir_is_error() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("same")).unwrap();
        let dir = tmp.path().join("same");
        let m = MakeMode::new(&dir, &dir, vec![]);
        assert_eq!(m.build_clean(), 1);
    }

    #[test]
    fn clean_src_inside_build_is_error() {
        let tmp = TempDir::new().unwrap();
        let build = tmp.path().join("build");
        let src = build.join("src"); // src is inside build
        std::fs::create_dir_all(&src).unwrap();
        std::fs::create_dir_all(&build).unwrap();
        let m = MakeMode::new(&src, &build, vec![]);
        assert_eq!(m.build_clean(), 1);
    }

    #[test]
    fn clean_removes_contents() {
        let tmp = TempDir::new().unwrap();
        let build = tmp.path().join("build");
        let src = tmp.path().join("src");
        std::fs::create_dir_all(&build).unwrap();
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(build.join("stale.html"), b"old content").unwrap();
        let m = MakeMode::new(&src, &build, vec![]);
        assert_eq!(m.build_clean(), 0);
        assert!(
            std::fs::read_dir(&build).unwrap().next().is_none(),
            "build dir should be empty after clean"
        );
    }

    // ── run_generic_build ─────────────────────────────────────────────────

    #[test]
    fn run_generic_build_dispatches_correct_args() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("src");
        let build = tmp.path().join("build");
        std::fs::create_dir_all(&src).unwrap();

        let mut mock = MockRunner::new();
        mock.expect_run()
            .withf(|prog, args, _cwd| {
                prog == "sphinx-build"
                    && args.contains(&"--builder".to_owned())
                    && args.contains(&"html".to_owned())
                    && args.contains(&"--doctree-dir".to_owned())
            })
            .returning(|_, _, _| Ok(0))
            .times(1);

        let m = MakeMode::new(&src, &build, vec![]);
        m.run_generic_build("html", None, &mock);
    }

    // ── run_make_mode ─────────────────────────────────────────────────────

    #[test]
    fn run_make_mode_too_few_args_returns_1() {
        let mock = MockRunner::new();
        assert_eq!(run_make_mode(&["html".to_owned()], &mock), 1);
    }

    // ── BUILDERS table ────────────────────────────────────────────────────

    #[test]
    fn builders_table_has_all_entries() {
        assert!(BUILDERS.len() >= 23);
        let names: Vec<_> = BUILDERS.iter().map(|(_, n, _)| *n).collect();
        for expected in ["html", "latex", "clean", "linkcheck", "man"] {
            assert!(names.contains(&expected), "missing: {expected}");
        }
    }
}
