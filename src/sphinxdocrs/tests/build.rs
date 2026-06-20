//! Build parser and make_mode integration tests.
//! Uses `CapturingRunner` instead of mockall mocks.

use rstest::*;
use tempfile::TempDir;

use sphinxdocrs::build::args::{
    ConfValue, jobs_argument, parse_color, parse_confdir, parse_confoverrides, parse_doctreedir,
    validate_filenames,
};
use sphinxdocrs::build::make_mode::{BUILDERS, MakeMode, run_make_mode};
use sphinxdocrs::cli::io::CapturingRunner;

// ── jobs_argument ─────────────────────────────────────────────────────────────

#[rstest]
#[case("1", 1usize)]
#[case("4", 4)]
#[case("16", 16)]
fn jobs_positive(#[case] v: &str, #[case] want: usize) {
    assert_eq!(jobs_argument(v), Ok(want));
}

#[rstest]
#[case("0")]
#[case("-1")]
#[case("abc")]
#[case("")]
fn jobs_invalid(#[case] v: &str) {
    let e = jobs_argument(v).unwrap_err();
    assert!(e.contains("positive number"), "got: {e}");
}

#[test]
fn jobs_auto_returns_cpu_count() {
    assert!(jobs_argument("auto").unwrap() >= 1);
}

// ── parse_confdir ─────────────────────────────────────────────────────────────

#[rstest]
#[case(true, None, "/src", None)]
#[case(false, Some("/cfg"), "/src", Some("/cfg"))]
#[case(false, None, "/src", Some("/src"))]
#[case(false, Some(""), "/src", Some("/src"))]
fn confdir_cases(
    #[case] noconfig: bool,
    #[case] confdir: Option<&str>,
    #[case] src: &str,
    #[case] expected: Option<&str>,
) {
    use std::path::PathBuf;
    assert_eq!(
        parse_confdir(noconfig, confdir, std::path::Path::new(src)),
        expected.map(PathBuf::from)
    );
}

// ── parse_doctreedir ──────────────────────────────────────────────────────────

#[rstest]
#[case(None, "/out", "/out/.doctrees")]
#[case(Some("/dtree"), "/out", "/dtree")]
fn doctreedir_cases(#[case] d: Option<&str>, #[case] out: &str, #[case] expected: &str) {
    use std::path::PathBuf;
    assert_eq!(
        parse_doctreedir(d, std::path::Path::new(out)),
        PathBuf::from(expected)
    );
}

// ── validate_filenames ────────────────────────────────────────────────────────

#[test]
fn filenames_force_all_with_files_is_error() {
    let e = validate_filenames(true, &["f.rst".to_owned()]).unwrap_err();
    assert!(e.to_string().contains("cannot combine"));
}

#[test]
fn filenames_force_all_empty_ok() {
    assert!(validate_filenames(true, &[]).is_ok());
}

// ── parse_confoverrides ───────────────────────────────────────────────────────

#[test]
fn define_string() {
    let r = parse_confoverrides(&["lang=fr".to_owned()], &[], false).unwrap();
    assert_eq!(r.get("lang"), Some(&ConfValue::Str("fr".into())));
}

#[test]
fn html_define_int() {
    let r = parse_confoverrides(&[], &["x=5".to_owned()], false).unwrap();
    assert_eq!(r.get("html_context.x"), Some(&ConfValue::Int(5)));
}

#[test]
fn html_define_string() {
    let r = parse_confoverrides(&[], &["title=Hello World".to_owned()], false).unwrap();
    assert_eq!(
        r.get("html_context.title"),
        Some(&ConfValue::Str("Hello World".into()))
    );
}

#[test]
fn nitpicky_flag() {
    let r = parse_confoverrides(&[], &[], true).unwrap();
    assert_eq!(r.get("nitpicky"), Some(&ConfValue::Int(1)));
}

#[test]
fn define_missing_equals_error() {
    assert!(parse_confoverrides(&["noeq".to_owned()], &[], false).is_err());
}

// ── parse_color ───────────────────────────────────────────────────────────────

#[rstest]
#[case(false, false, "auto")]
#[case(true, false, "yes")]
#[case(false, true, "no")]
fn color_cases(#[case] color: bool, #[case] no_color: bool, #[case] expected: &str) {
    assert_eq!(parse_color(color, no_color), expected);
}

// ── make_mode: build_clean ────────────────────────────────────────────────────

#[test]
fn clean_nonexistent_build_dir() {
    let tmp = TempDir::new().unwrap();
    let m = MakeMode::new(tmp.path().join("src"), tmp.path().join("build"), vec![]);
    assert_eq!(m.build_clean(), 0);
}

#[test]
fn clean_same_dir_is_error() {
    let tmp = TempDir::new().unwrap();
    let d = tmp.path().to_path_buf();
    assert_eq!(MakeMode::new(&d, &d, vec![]).build_clean(), 1);
}

#[test]
fn clean_src_inside_build_is_error() {
    let tmp = TempDir::new().unwrap();
    let build = tmp.path().to_path_buf();
    let src = build.join("src");
    std::fs::create_dir_all(&src).unwrap();
    assert_eq!(MakeMode::new(&src, &build, vec![]).build_clean(), 1);
}

#[test]
fn clean_removes_contents() {
    let tmp = TempDir::new().unwrap();
    let build = tmp.path().join("build");
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&build).unwrap();
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(build.join("stale.html"), b"old").unwrap();
    let m = MakeMode::new(&src, &build, vec![]);
    assert_eq!(m.build_clean(), 0);
    assert!(std::fs::read_dir(&build).unwrap().next().is_none());
}

// ── make_mode: run_generic_build via CapturingRunner ──────────────────────────

#[test]
fn run_generic_build_correct_args() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    let build = tmp.path().join("build");
    let runner = CapturingRunner::new(0);
    let m = MakeMode::new(&src, &build, vec![]);
    m.run_generic_build("html", None, &runner);
    let calls = runner.calls();
    assert_eq!(calls.len(), 1);
    let (prog, args) = &calls[0];
    assert_eq!(prog, "sphinx-build");
    assert!(args.contains(&"--builder".to_owned()));
    assert!(args.contains(&"html".to_owned()));
    assert!(args.contains(&"--doctree-dir".to_owned()));
}

#[test]
fn run_generic_build_forwards_opts() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    let build = tmp.path().join("build");
    let runner = CapturingRunner::new(0);
    let m = MakeMode::new(&src, &build, vec!["-W".to_owned()]);
    m.run_generic_build("html", None, &runner);
    let (_, args) = &runner.calls()[0];
    assert!(args.contains(&"-W".to_owned()));
}

// ── make_mode: dispatch ───────────────────────────────────────────────────────

#[test]
fn dispatch_help_returns_0() {
    let tmp = TempDir::new().unwrap();
    let runner = CapturingRunner::new(0);
    let m = MakeMode::new(tmp.path().join("s"), tmp.path().join("b"), vec![]);
    assert_eq!(m.dispatch("help", &runner), 0);
    assert!(runner.calls().is_empty()); // help doesn't spawn
}

#[test]
fn dispatch_unknown_target_calls_runner() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    let build = tmp.path().join("build");
    let runner = CapturingRunner::new(0);
    let m = MakeMode::new(&src, &build, vec![]);
    m.dispatch("man", &runner);
    let (_, args) = &runner.calls()[0];
    assert!(args.contains(&"man".to_owned()));
}

// ── run_make_mode: arg validation ─────────────────────────────────────────────

#[test]
fn run_make_mode_too_few_args() {
    let runner = CapturingRunner::new(0);
    assert_eq!(
        run_make_mode(&["html".to_owned(), "src".to_owned()], &runner),
        1
    );
}

// ── BUILDERS table ────────────────────────────────────────────────────────────

#[test]
fn builders_table_complete() {
    let names: Vec<_> = BUILDERS.iter().map(|(_, n, _)| *n).collect();
    for e in &[
        "html",
        "latex",
        "man",
        "texinfo",
        "linkcheck",
        "doctest",
        "coverage",
        "clean",
        "gettext",
    ] {
        assert!(names.contains(e), "missing: {e}");
    }
}

// ── help snapshot ─────────────────────────────────────────────────────────────

#[test]
fn build_help_snapshot() {
    let mut cmd = sphinxdocrs::build::parser::build_parser();
    insta::assert_snapshot!(cmd.render_help().to_string());
}
