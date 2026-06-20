//! Integration tests for `sphinxdocrs::util_rst` and
//! `sphinxdocrs::util_osutil`.
//!
//! Mirrors the upstream test suites:
//! - `sphinx/tests/test_util/test_util_rst.py`
//! - `sphinx/tests/test_util/test_util.py` (osutil subset)

use std::collections::HashSet;
use std::path::PathBuf;

use rstest::*;
use tempfile::TempDir;

// ── util_rst imports ──────────────────────────────────────────────────────────

use sphinxdocrs::util_rst::{
    ContentLine, SECTIONING_CHARS, WIDECHARS_DEFAULT, WIDECHARS_JA, append_epilogue, escape,
    heading, prepend_prologue, textwidth,
};

// ── util_osutil imports ───────────────────────────────────────────────────────

use sphinxdocrs::util_osutil::{
    FileAvoidWrite, SEP, canon_path, copyfile, ensuredir, make_filename,
    make_filename_from_project, os_path, path_stabilize, relative_uri, relpath, rmtree,
};

// ═══════════════════════════════════════════════════════════════════════════════
// util_rst
// ═══════════════════════════════════════════════════════════════════════════════

// ── constants ─────────────────────────────────────────────────────────────────

#[test]
fn sectioning_chars_are_correct() {
    assert_eq!(SECTIONING_CHARS, ['=', '-', '~']);
}

// ── escape ────────────────────────────────────────────────────────────────────

// Mirrors test_escape() from test_util_rst.py

#[test]
fn rst_escape_ref_role() {
    assert_eq!(escape(":ref:`id`"), r"\:ref\:\`id\`");
}

#[test]
fn rst_escape_footnote() {
    assert_eq!(escape("footnote [#]_"), r"footnote \[\#\]\_");
}

#[test]
fn rst_escape_dotted_module() {
    // dots are NOT escaped
    assert_eq!(escape("sphinx.application"), "sphinx.application");
}

#[test]
fn rst_escape_toctree_directive() {
    assert_eq!(escape(".. toctree::"), r"\.. toctree\:\:");
}

#[test]
fn rst_escape_empty() {
    assert_eq!(escape(""), "");
}

#[test]
fn rst_escape_plain_words() {
    assert_eq!(escape("Hello World"), "Hello World");
}

// ── textwidth ─────────────────────────────────────────────────────────────────

// Mirrors test_textwidth() from test_util_rst.py

#[test]
fn textwidth_hello() {
    assert_eq!(textwidth("Hello", WIDECHARS_DEFAULT), 5);
}

#[test]
fn textwidth_cyrillic_wf() {
    // Mirrors: textwidth('русский язык') == 12
    assert_eq!(textwidth("русский язык", WIDECHARS_DEFAULT), 12);
}

#[test]
fn textwidth_cyrillic_wfa() {
    // Mirrors: textwidth('русский язык', 'WFA') == 23
    assert_eq!(textwidth("русский язык", WIDECHARS_JA), 23);
}

#[test]
fn textwidth_empty_string() {
    assert_eq!(textwidth("", WIDECHARS_DEFAULT), 0);
}

#[test]
fn textwidth_ascii_symbols() {
    // All ASCII printable: should be 1 each
    assert_eq!(textwidth("abc!@#", WIDECHARS_DEFAULT), 6);
}

// ── heading ───────────────────────────────────────────────────────────────────

// Mirrors test_heading() from test_util_rst.py

#[test]
fn heading_default_level1() {
    assert_eq!(heading("Hello", 1, None), "Hello\n=====");
}

#[test]
fn heading_level1_explicit() {
    assert_eq!(heading("Hello", 1, None), "Hello\n=====");
}

#[test]
fn heading_level2() {
    assert_eq!(heading("Hello", 2, None), "Hello\n-----");
}

#[test]
fn heading_level3() {
    assert_eq!(heading("Hello", 3, None), "Hello\n~~~~~");
}

#[rstest]
#[case(1, '=')]
#[case(2, '-')]
#[case(3, '~')]
fn heading_uses_correct_char(#[case] level: usize, #[case] expected_char: char) {
    let h = heading("X", level, None);
    let underline = h.lines().nth(1).unwrap();
    assert!(
        underline.chars().all(|c| c == expected_char),
        "level {level}: expected underline of '{expected_char}', got {underline:?}"
    );
}

#[test]
fn heading_cyrillic_no_lang() {
    // Mirrors: heading(env, 'русский язык', 1) == 'русский язык\n============'
    // width=12 under WF
    assert_eq!(
        heading("русский язык", 1, None),
        "русский язык\n============"
    );
}

#[test]
fn heading_cyrillic_ja() {
    // Mirrors: heading(env with language=ja, 'русский язык', 1)
    // width=23 under WFA
    assert_eq!(
        heading("русский язык", 1, Some("ja")),
        "русский язык\n======================="
    );
}

#[test]
fn heading_underline_length_matches_textwidth() {
    let text = "Example";
    let h = heading(text, 1, None);
    let lines: Vec<&str> = h.lines().collect();
    assert_eq!(lines[0], text);
    assert_eq!(lines[1].len(), textwidth(text, WIDECHARS_DEFAULT));
}

// ═══════════════════════════════════════════════════════════════════════════════
// util_osutil
// ═══════════════════════════════════════════════════════════════════════════════

// ── SEP ───────────────────────────────────────────────────────────────────────

#[test]
fn sep_constant() {
    assert_eq!(SEP, '/');
}

// ── os_path ───────────────────────────────────────────────────────────────────

#[test]
fn os_path_posix_unchanged() {
    #[cfg(not(windows))]
    assert_eq!(os_path("a/b/c"), "a/b/c");
}

// ── canon_path ────────────────────────────────────────────────────────────────

#[test]
fn canon_path_str() {
    assert_eq!(canon_path("a/b/c"), "a/b/c");
}

#[test]
fn canon_path_pathbuf() {
    assert_eq!(canon_path(&PathBuf::from("docs/api")), "docs/api");
}

// ── path_stabilize ────────────────────────────────────────────────────────────

#[test]
fn path_stabilize_ascii_unchanged() {
    assert_eq!(path_stabilize("a/b/c"), "a/b/c");
}

#[test]
fn path_stabilize_nfc_normalises_nfd() {
    // 'cafe' + COMBINING ACUTE ACCENT → NFC precomposed 'é'
    let nfd = "cafe\u{0301}";
    assert_eq!(path_stabilize(nfd), "caf\u{00e9}");
}

// ── relative_uri ─────────────────────────────────────────────────────────────

// Mirrors test cases from sphinx/tests/test_util/test_util.py

#[rstest]
#[case("a/b/c.html", "a/b/d.html", "d.html")]
#[case("a/b/index.html", "a/c/d.html", "../c/d.html")]
#[case("a/b/c/d.html", "a/e.html", "../../e.html")]
#[case("f/index.html", "f/index.html", "")]
#[case("f/index.html", "f/", "./")]
#[case("x.html", "/abs.html", "/abs.html")]
#[case("index.html", "api/module.html", "api/module.html")]
#[case("index.html", "index.html", "")]
fn relative_uri_cases(#[case] base: &str, #[case] to: &str, #[case] expected: &str) {
    assert_eq!(
        relative_uri(base, to),
        expected,
        "relative_uri({base:?}, {to:?})"
    );
}

// ── ensuredir ─────────────────────────────────────────────────────────────────

// Mirrors test_ensuredir from test_util.py

#[test]
fn ensuredir_creates_nested_dirs() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("a").join("b").join("c");
    assert!(!path.exists());
    ensuredir(&path).unwrap();
    assert!(path.is_dir());
}

#[test]
fn ensuredir_existing_dir_is_noop() {
    let tmp = TempDir::new().unwrap();
    ensuredir(tmp.path()).unwrap();
    assert!(tmp.path().is_dir());
}

// ── make_filename ─────────────────────────────────────────────────────────────

#[rstest]
#[case("My Project!", "MyProject")]
#[case("hello-world_2", "hello-world_2")]
#[case("", "sphinx")]
#[case("!!!", "sphinx")]
#[case("abc123", "abc123")]
#[case("a b c", "abc")]
fn make_filename_cases(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(make_filename(input), expected, "input={input:?}");
}

// ── make_filename_from_project ────────────────────────────────────────────────

#[rstest]
#[case("Sphinx Documentation", "sphinx")]
#[case("My Project", "myproject")]
#[case("HelloWorld", "helloworld")]
#[case("My Documentation", "my")] // strips " Documentation" suffix
fn make_filename_from_project_cases(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(
        make_filename_from_project(input),
        expected,
        "input={input:?}"
    );
}

// ── FileAvoidWrite ────────────────────────────────────────────────────────────

#[test]
fn file_avoid_write_new_file() {
    use std::io::Write;
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("out.txt");
    let mut w = FileAvoidWrite::new(&path);
    write!(w, "content").unwrap();
    w.close().unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "content");
}

#[test]
fn file_avoid_write_same_content_no_rewrite() {
    use std::io::Write;
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("out.txt");
    std::fs::write(&path, "same").unwrap();
    let mtime_before = std::fs::metadata(&path).unwrap().modified().ok();

    let mut w = FileAvoidWrite::new(&path);
    write!(w, "same").unwrap();
    w.close().unwrap();

    let mtime_after = std::fs::metadata(&path).unwrap().modified().ok();
    assert_eq!(mtime_before, mtime_after);
}

#[test]
fn file_avoid_write_different_content_overwrites() {
    use std::io::Write;
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("out.txt");
    std::fs::write(&path, "old").unwrap();

    let mut w = FileAvoidWrite::new(&path);
    write!(w, "new").unwrap();
    w.close().unwrap();

    assert_eq!(std::fs::read_to_string(&path).unwrap(), "new");
}

#[test]
fn file_avoid_write_get_value() {
    use std::io::Write;
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("out.txt");
    let mut w = FileAvoidWrite::new(&path);
    write!(w, "hello").unwrap();
    assert_eq!(w.get_value(), b"hello");
}

// ── copyfile ──────────────────────────────────────────────────────────────────

// Mirrors test_copyfile (test_util.py)

#[test]
fn copyfile_basic() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dst = tmp.path().join("dst.txt");
    std::fs::write(&src, b"hello").unwrap();
    copyfile(&src, &dst, false).unwrap();
    assert_eq!(std::fs::read(&dst).unwrap(), b"hello");
}

#[test]
fn copyfile_nonexistent_source_errors() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("missing.txt");
    let dst = tmp.path().join("dst.txt");
    let err = copyfile(&src, &dst, false).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn copyfile_identical_content_noop() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dst = tmp.path().join("dst.txt");
    std::fs::write(&src, b"same").unwrap();
    std::fs::write(&dst, b"same").unwrap();
    let mtime_before = std::fs::metadata(&dst).unwrap().modified().ok();
    // Should be a no-op — content is identical.
    copyfile(&src, &dst, false).unwrap();
    // Content unchanged.
    assert_eq!(std::fs::read(&dst).unwrap(), b"same");
    let mtime_after = std::fs::metadata(&dst).unwrap().modified().ok();
    assert_eq!(mtime_before, mtime_after);
}

#[test]
fn copyfile_existing_dest_no_force_aborts() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dst = tmp.path().join("dst.txt");
    std::fs::write(&src, b"new").unwrap();
    std::fs::write(&dst, b"old").unwrap();
    let err = copyfile(&src, &dst, false).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::AlreadyExists);
    // Original not overwritten.
    assert_eq!(std::fs::read(&dst).unwrap(), b"old");
}

#[test]
fn copyfile_existing_dest_force_overwrites() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dst = tmp.path().join("dst.txt");
    std::fs::write(&src, b"new").unwrap();
    std::fs::write(&dst, b"old").unwrap();
    copyfile(&src, &dst, true).unwrap();
    assert_eq!(std::fs::read(&dst).unwrap(), b"new");
}

// ── relpath ───────────────────────────────────────────────────────────────────

#[test]
fn relpath_sibling_files() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path().join("a").join("b");
    let target = tmp.path().join("a").join("c.txt");
    // base/b relative to base → ".."  then c.txt → "../c.txt"
    let rel = relpath(&target, &base);
    assert!(
        rel.to_str().unwrap().contains("c.txt"),
        "got: {}",
        rel.display()
    );
}

#[test]
fn relpath_same_path() {
    let tmp = TempDir::new().unwrap();
    let p = tmp.path().join("a").join("b.txt");
    let rel = relpath(&p, &p);
    assert_eq!(rel, PathBuf::from(""));
}

// ── rmtree ────────────────────────────────────────────────────────────────────

#[test]
fn rmtree_removes_directory() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("subdir");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("file.txt"), b"data").unwrap();
    rmtree(&dir, false).unwrap();
    assert!(!dir.exists());
}

#[test]
fn rmtree_ignore_errors_on_missing() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("does_not_exist");
    // Should not error when ignore_errors=true.
    rmtree(&missing, true).unwrap();
}

#[test]
fn rmtree_error_on_missing_no_ignore() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("does_not_exist");
    assert!(rmtree(&missing, false).is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// util_rst — prologue / epilogue
// ═══════════════════════════════════════════════════════════════════════════════

// Mirrors test__prepend_prologue / test__append_epilogue from test_util_rst.py.

#[test]
fn prepend_prologue_inserts_before_body() {
    let mut content: Vec<ContentLine> = vec![("Body text.".into(), "<source>", 0)];
    prepend_prologue(&mut content, ".. highlight:: python");
    let texts: Vec<&str> = content.iter().map(|(t, _, _)| t.as_str()).collect();
    assert_eq!(texts[0], ".. highlight:: python");
    assert_eq!(texts[1], ""); // blank separator
    assert_eq!(texts[2], "Body text.");
}

#[test]
fn prepend_prologue_multiline_prologue() {
    let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
    prepend_prologue(&mut content, "line1\nline2");
    let texts: Vec<&str> = content.iter().map(|(t, _, _)| t.as_str()).collect();
    assert_eq!(texts, ["line1", "line2", "", "Body."]);
}

#[test]
fn prepend_prologue_after_docinfo() {
    let mut content: Vec<ContentLine> = vec![
        (":author: Me".into(), "<source>", 0),
        ("".into(), "<source>", 1),
        ("Body.".into(), "<source>", 2),
    ];
    prepend_prologue(&mut content, ".. epilogue-like::");
    // First two lines are docinfo.
    assert_eq!(content[0].0, ":author: Me");
    // blank inserted after docinfo block
    assert_eq!(content[1].0, "");
    // prologue
    assert_eq!(content[2].0, ".. epilogue-like::");
}

#[test]
fn prepend_prologue_empty_noop() {
    let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
    prepend_prologue(&mut content, "");
    assert_eq!(content.len(), 1);
}

#[test]
fn append_epilogue_appends_after_body() {
    let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
    append_epilogue(&mut content, ".. footer::");
    let texts: Vec<&str> = content.iter().map(|(t, _, _)| t.as_str()).collect();
    assert_eq!(texts, ["Body.", "", ".. footer::"]);
}

#[test]
fn append_epilogue_multiline() {
    let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
    append_epilogue(&mut content, "e1\ne2");
    let texts: Vec<&str> = content.iter().map(|(t, _, _)| t.as_str()).collect();
    assert_eq!(texts, ["Body.", "", "e1", "e2"]);
}

#[test]
fn append_epilogue_empty_noop() {
    let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
    append_epilogue(&mut content, "");
    assert_eq!(content.len(), 1);
}
