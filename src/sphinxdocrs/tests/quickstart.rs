//! Quickstart integration tests — rstest parametrization + cargo-insta snapshots.
//!
//! Uses only concrete (non-mockall) helpers so they compile as an independent
//! test crate. Mock-based unit tests live inline in the source modules.

use rstest::*;
use tempfile::TempDir;

use sphinxdocrs::cli::io::{FixedClock, RealFs, ScriptedTerminal};
use sphinxdocrs::quickstart::generate::{ask_user, generate, valid_dir};
use sphinxdocrs::quickstart::parser::parse_args;
use sphinxdocrs::quickstart::settings::QuickstartSettings;
use sphinxdocrs::quickstart::templates::QuickstartTemplates;
use sphinxdocrs::quickstart::validate;

// ── helpers ──────────────────────────────────────────────────────────────────

fn args(s: &str) -> Vec<String> {
    std::iter::once("sphinx-quickstart")
        .chain(s.split_whitespace())
        .map(String::from)
        .collect()
}

#[fixture]
fn fixed_clock() -> FixedClock {
    FixedClock::snapshot()
}

#[fixture]
#[once]
fn templates() -> QuickstartTemplates {
    QuickstartTemplates::vendored()
}

// ── validators ────────────────────────────────────────────────────────────────

#[rstest]
#[case("y", Some(true))]
#[case("Y", Some(true))]
#[case("yes", Some(true))]
#[case("YES", Some(true))]
#[case("n", Some(false))]
#[case("N", Some(false))]
#[case("no", Some(false))]
#[case("NO", Some(false))]
#[case("maybe", None)]
#[case("", None)]
#[case("1", None)]
fn boolean_parity(#[case] input: &str, #[case] want: Option<bool>) {
    assert_eq!(validate::boolean(input).ok(), want);
}

#[rstest]
#[case(".rst", true)]
#[case(".txt", true)]
#[case(".md", true)]
#[case("rst", false)]
#[case(".", false)]
#[case("", false)]
fn suffix_parity(#[case] input: &str, #[case] ok: bool) {
    assert_eq!(validate::suffix(input).is_ok(), ok, "input={input}");
}

#[rstest]
#[case("hello", true)]
#[case("x", true)]
#[case("", false)]
fn nonempty_parity(#[case] input: &str, #[case] ok: bool) {
    assert_eq!(validate::nonempty(input).is_ok(), ok);
}

#[rstest]
#[case("anything", true)]
#[case("", true)]
fn allow_empty_always_passes(#[case] input: &str, #[case] ok: bool) {
    assert_eq!(validate::allow_empty(input).is_ok(), ok);
}

#[rstest]
#[case("a", true)]
#[case("b", true)]
#[case("z", false)]
fn choice_validation(#[case] input: &str, #[case] ok: bool) {
    let c = validate::choice(&["a", "b"]);
    assert_eq!(c(input).is_ok(), ok, "input={input}");
}

// ── parser ────────────────────────────────────────────────────────────────────

#[test]
fn defaults_from_flags() {
    let s = parse_args(&args("-q -p P -a A")).unwrap();
    assert_eq!(s.suffix, ".rst");
    assert_eq!(s.master, "index");
    assert_eq!(s.dot, "_");
    assert!(!s.sep);
    assert!(s.makefile);
    assert!(s.batchfile);
    assert!(s.quiet);
}

#[rstest]
#[case("--sep", true)]
#[case("--no-sep", false)]
fn sep_flag_parity(#[case] flag: &str, #[case] expected: bool) {
    assert_eq!(
        parse_args(&args(&format!("-q -p P -a A {flag}")))
            .unwrap()
            .sep,
        expected
    );
}

#[rstest]
#[case("--no-makefile --no-batchfile", false, false)]
#[case("--no-makefile", false, true)]
#[case("--no-batchfile", true, false)]
#[case("", true, true)]
fn makefile_batchfile(#[case] flags: &str, #[case] mk: bool, #[case] bat: bool) {
    let s = parse_args(&args(&format!("-q -p P -a A {flags}"))).unwrap();
    assert_eq!(s.makefile, mk);
    assert_eq!(s.batchfile, bat);
}

#[rstest]
#[case("--ext-autodoc", "sphinx.ext.autodoc")]
#[case("--ext-mathjax", "sphinx.ext.mathjax")]
#[case("--ext-viewcode", "sphinx.ext.viewcode")]
fn ext_flags(#[case] flag: &str, #[case] expected_ext: &str) {
    let s = parse_args(&args(&format!("-q -p P -a A {flag}"))).unwrap();
    assert!(
        s.extensions.contains(&expected_ext.to_owned()),
        "{:?}",
        s.extensions
    );
}

#[test]
fn release_defaults_to_version() {
    assert_eq!(
        parse_args(&args("-q -p P -a A -v 2.0")).unwrap().release,
        "2.0"
    );
}

#[test]
fn explicit_release() {
    assert_eq!(
        parse_args(&args("-q -p P -a A -v 2.0 -r 2.0.1"))
            .unwrap()
            .release,
        "2.0.1"
    );
}

// ── valid_dir (real FS) ───────────────────────────────────────────────────────

#[test]
fn valid_dir_nonexistent_is_ok() {
    let tmp = TempDir::new().unwrap();
    assert!(valid_dir(
        &tmp.path().join("nope"),
        false,
        "_",
        "index",
        ".rst",
        &RealFs
    ));
}

#[test]
fn valid_dir_with_conf_py_false() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("conf.py"), b"").unwrap();
    assert!(!valid_dir(tmp.path(), false, "_", "index", ".rst", &RealFs));
}

#[test]
fn valid_dir_with_makefile_false() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Makefile"), b"").unwrap();
    assert!(!valid_dir(tmp.path(), false, "_", "index", ".rst", &RealFs));
}

#[test]
fn valid_dir_empty_is_ok() {
    let tmp = TempDir::new().unwrap();
    assert!(valid_dir(tmp.path(), false, "_", "index", ".rst", &RealFs));
}

// ── generate — tree snapshots ─────────────────────────────────────────────────

fn list_tree(root: &std::path::Path) -> Vec<String> {
    let mut paths = Vec::new();
    fn walk(dir: &std::path::Path, root: &std::path::Path, acc: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut v: Vec<_> = entries.flatten().collect();
            v.sort_by_key(|e| e.path());
            for e in v {
                let rel = e
                    .path()
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                acc.push(rel.clone());
                if e.path().is_dir() {
                    walk(&e.path(), root, acc);
                }
            }
        }
    }
    walk(root, root, &mut paths);
    paths
}

#[rstest]
#[case::flat_make("-q -p TestProject -a TestAuthor")]
#[case::sep("-q --sep -p TestProject -a TestAuthor")]
#[case::no_make("-q --no-makefile --no-batchfile -p TestProject -a TestAuthor")]
#[case::with_ext("-q -p TestProject -a TestAuthor --ext-autodoc --ext-mathjax")]
fn quickstart_tree_snapshot(
    fixed_clock: FixedClock,
    templates: &QuickstartTemplates,
    #[case] flags: &str,
) {
    let tmp = TempDir::new().unwrap();
    let s = parse_args(&args(flags)).unwrap().with_path(tmp.path());
    generate(&s, templates, &RealFs, &fixed_clock).unwrap();
    // Use the rstest case name as the snapshot suffix so each case gets its own
    // snapshot file and insta doesn't number them arbitrarily.
    let case_name = flags
        .split_whitespace()
        .filter(|w| w.starts_with("--"))
        .map(|w| w.trim_start_matches("--"))
        .collect::<Vec<_>>()
        .join("_");
    let snap_name = if case_name.is_empty() {
        "flat_make".to_owned()
    } else {
        case_name
    };
    insta::with_settings!({snapshot_suffix => snap_name}, {
        insta::assert_yaml_snapshot!(list_tree(tmp.path()));
    });
}

#[rstest]
fn conf_py_snapshot(fixed_clock: FixedClock, templates: &QuickstartTemplates) {
    let tmp = TempDir::new().unwrap();
    let s = parse_args(&args("-q -p SnapshotProj -a AuthorName -v 0.1"))
        .unwrap()
        .with_path(tmp.path());
    generate(&s, templates, &RealFs, &fixed_clock).unwrap();
    insta::assert_snapshot!(std::fs::read_to_string(tmp.path().join("conf.py")).unwrap());
}

#[rstest]
fn makefile_uses_lf(fixed_clock: FixedClock, templates: &QuickstartTemplates) {
    let tmp = TempDir::new().unwrap();
    let s = parse_args(&args("-q -p P -a A"))
        .unwrap()
        .with_path(tmp.path());
    generate(&s, templates, &RealFs, &fixed_clock).unwrap();
    let bytes = std::fs::read(tmp.path().join("Makefile")).unwrap();
    assert!(
        !bytes.windows(2).any(|w| w == b"\r\n"),
        "Makefile must use LF only"
    );
}

#[rstest]
fn make_bat_uses_crlf(fixed_clock: FixedClock, templates: &QuickstartTemplates) {
    let tmp = TempDir::new().unwrap();
    let s = parse_args(&args("-q -p P -a A"))
        .unwrap()
        .with_path(tmp.path());
    generate(&s, templates, &RealFs, &fixed_clock).unwrap();
    let bytes = std::fs::read(tmp.path().join("make.bat")).unwrap();
    for i in 0..bytes.len() {
        if bytes[i] == b'\n' {
            assert!(i > 0 && bytes[i - 1] == b'\r', "make.bat bare LF at {i}");
        }
    }
}

// ── ask_user via ScriptedTerminal ─────────────────────────────────────────────

#[test]
fn ask_user_fills_settings() {
    let answers = [
        ".", "n", "_", "MyProj", "Me", "1.0", "1.0", "en", ".rst", "index", "n", "n", "n", "n",
        "n", "n", "n", "n", "n", "n", // 10 extensions
        "y", "y", // makefile, batchfile
    ];
    let term = ScriptedTerminal::new(answers);
    let mut settings = QuickstartSettings {
        path: std::env::temp_dir(),
        ..Default::default()
    };
    ask_user(&mut settings, &term, &RealFs);
    assert_eq!(settings.project, "MyProj");
    assert_eq!(settings.author, "Me");
    assert!(settings.language.is_none());
    assert!(settings.makefile);
    assert!(settings.batchfile);
}

// ── help snapshot ─────────────────────────────────────────────────────────────

#[test]
fn quickstart_help_snapshot() {
    let mut cmd = sphinxdocrs::quickstart::parser::build_parser();
    insta::assert_snapshot!(cmd.render_help().to_string());
}
