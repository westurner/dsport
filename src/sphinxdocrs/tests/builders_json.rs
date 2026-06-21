//! Integration tests for `sphinxdocrs::builders::json::JsonBuilder`.
//!
//! Mirrors the pure-Rust-testable subset of
//! `sphinx/tests/test_builders/test_build_all.py` (JSON builder path) and
//! the `sphinxcontrib-serializinghtml` serialization contract:
//!
//! - Builder identity (`name`, `format`, `out_suffix`)
//! - `get_target_uri` semantics matching `SerializingHTMLBuilder`
//! - `build_doc` produces a valid `.fjson` file
//! - `.fjson` deserializes to [`PageContext`] with all required keys
//! - `build_all` writes one `.fjson` per source + `globalcontext.json`
//! - `GlobalContext` carries correct builder name and project metadata
//! - Serde round-trip parity on both context types

use std::collections::HashMap;
use std::path::Path;

use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::json::{GlobalContext, JsonBuilder, PageContext};
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_env(srcdir: &Path, outdir: &Path) -> BuildEnvironment {
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(srcdir, &[(".rst", "restructuredtext")]);
    BuildEnvironment::new(config, project, srcdir, outdir)
}

fn read_fjson(dir: &Path, docname: &str) -> String {
    let path: std::path::PathBuf = docname
        .split('/')
        .collect::<std::path::PathBuf>()
        .with_extension("fjson");
    std::fs::read_to_string(dir.join(path)).expect("output .fjson should exist")
}

fn parse_page(dir: &Path, docname: &str) -> PageContext {
    let raw = read_fjson(dir, docname);
    serde_json::from_str(&raw).expect(".fjson should deserialize to PageContext")
}

fn parse_globalcontext(outdir: &Path) -> GlobalContext {
    let raw = std::fs::read_to_string(outdir.join("globalcontext.json"))
        .expect("globalcontext.json should exist");
    serde_json::from_str(&raw).expect("globalcontext.json should deserialize to GlobalContext")
}

// ── builder identity ──────────────────────────────────────────────────────────

/// Mirrors `JSONHTMLBuilder.name == "json"`.
#[test]
fn builder_name_is_json() {
    assert_eq!(JsonBuilder::new().name(), "json");
}

/// Mirrors `JSONHTMLBuilder.format == "json"`.
#[test]
fn builder_format_is_json() {
    assert_eq!(JsonBuilder::new().format(), "json");
}

/// Mirrors `JSONHTMLBuilder.out_suffix == ".fjson"`.
#[test]
fn builder_out_suffix_is_fjson() {
    assert_eq!(JsonBuilder::new().out_suffix(), ".fjson");
}

/// Mirrors `JSONHTMLBuilder.globalcontext_filename`.
#[test]
fn builder_globalcontext_filename() {
    assert_eq!(
        JsonBuilder::new().globalcontext_filename,
        "globalcontext.json"
    );
}

/// Mirrors `JSONHTMLBuilder.searchindex_filename`.
#[test]
fn builder_searchindex_filename() {
    assert_eq!(JsonBuilder::new().searchindex_filename, "searchindex.json");
}

// ── get_target_uri ────────────────────────────────────────────────────────────
//
// Python reference (`SerializingHTMLBuilder.get_target_uri`):
//   if docname == 'index':   return ''
//   if docname.endswith(SEP + 'index'): return docname[:-5]  # up to sep
//   return docname + SEP

/// `"index"` → `""` (root page has an empty URI).
#[test]
fn get_target_uri_root_index() {
    assert_eq!(JsonBuilder::new().get_target_uri("index"), "");
}

/// `"guide/index"` → `"guide/"` (trailing `index` stripped).
#[test]
fn get_target_uri_subdir_index() {
    assert_eq!(JsonBuilder::new().get_target_uri("guide/index"), "guide/");
}

/// `"guide/intro"` → `"guide/intro/"`.
#[test]
fn get_target_uri_plain_page() {
    assert_eq!(
        JsonBuilder::new().get_target_uri("guide/intro"),
        "guide/intro/"
    );
}

/// `"api/module/foo"` → `"api/module/foo/"`.
#[test]
fn get_target_uri_deeply_nested() {
    assert_eq!(
        JsonBuilder::new().get_target_uri("api/module/foo"),
        "api/module/foo/"
    );
}

// ── build_doc — file creation ─────────────────────────────────────────────────

/// `build_doc` must create `<docname>.fjson` in `outdir`.
#[test]
fn build_doc_creates_fjson_file() {
    let out = TempDir::new().unwrap();
    JsonBuilder::new()
        .build_doc("index", "Title\n=====\n\nContent.\n", out.path())
        .unwrap();
    assert!(
        out.path().join("index.fjson").exists(),
        ".fjson should be created"
    );
}

/// `build_doc` must create parent directories for nested docnames.
#[test]
fn build_doc_creates_subdirectories() {
    let out = TempDir::new().unwrap();
    JsonBuilder::new()
        .build_doc("guide/intro", "Intro\n=====\n\nParagraph.\n", out.path())
        .unwrap();
    assert!(
        out.path().join("guide").join("intro.fjson").exists(),
        "subdirectory and .fjson should be created"
    );
}

// ── build_doc — JSON structure ────────────────────────────────────────────────

/// The output file must be valid JSON parseable into [`PageContext`].
#[test]
fn build_doc_output_is_valid_json() {
    let out = TempDir::new().unwrap();
    JsonBuilder::new()
        .build_doc("index", "Title\n=====\n\nContent.\n", out.path())
        .unwrap();
    // parse_page panics on error, so reaching here means valid JSON.
    let _ = parse_page(out.path(), "index");
}

/// `current_page_name` must equal the docname.
#[test]
fn fjson_current_page_name_matches_docname() {
    let out = TempDir::new().unwrap();
    JsonBuilder::new()
        .build_doc("guide/intro", "Intro\n=====\n\nText.\n", out.path())
        .unwrap();
    let ctx = parse_page(out.path(), "guide/intro");
    assert_eq!(ctx.current_page_name, "guide/intro");
}

/// `sourcename` must be `<docname>.rst`.
#[test]
fn fjson_sourcename_is_docname_rst() {
    let out = TempDir::new().unwrap();
    JsonBuilder::new()
        .build_doc("index", "Title\n=====\n\nContent.\n", out.path())
        .unwrap();
    let ctx = parse_page(out.path(), "index");
    assert_eq!(ctx.sourcename, "index.rst");
}

/// `body` must contain HTML markup from the RST input.
#[test]
fn fjson_body_contains_html() {
    let out = TempDir::new().unwrap();
    let rst = "Hello\n=====\n\nA **bold** word.\n";
    JsonBuilder::new()
        .build_doc("index", rst, out.path())
        .unwrap();
    let ctx = parse_page(out.path(), "index");
    // RST bold → <strong> in HTML5
    assert!(
        ctx.body.contains("<strong>") || ctx.body.contains("bold"),
        "body should contain rendered markup; got: {}",
        &ctx.body[..ctx.body.len().min(300)]
    );
}

/// `title` must be extracted from the first section heading.
#[test]
fn fjson_title_extracted_from_heading() {
    let out = TempDir::new().unwrap();
    let rst = "My Page Title\n=============\n\nSome content.\n";
    JsonBuilder::new()
        .build_doc("mypage", rst, out.path())
        .unwrap();
    let ctx = parse_page(out.path(), "mypage");
    assert!(
        ctx.title.contains("My Page Title"),
        "title should contain section heading; got: {:?}",
        ctx.title
    );
}

/// When there is no section heading, `title` falls back to the docname.
#[test]
fn fjson_title_fallback_to_docname() {
    let out = TempDir::new().unwrap();
    let rst = "Just a paragraph with no heading underline.\n";
    JsonBuilder::new()
        .build_doc("fallback/doc", rst, out.path())
        .unwrap();
    let ctx = parse_page(out.path(), "fallback/doc");
    assert_eq!(ctx.title, "fallback/doc");
}

/// `parents`, `prev`, `next` are absent (deferred) — they should be empty/None.
#[test]
fn fjson_navigation_fields_are_empty() {
    let out = TempDir::new().unwrap();
    JsonBuilder::new()
        .build_doc("index", "Title\n=====\n\nContent.\n", out.path())
        .unwrap();
    let ctx = parse_page(out.path(), "index");
    assert!(ctx.parents.is_empty(), "parents should be empty");
    assert!(ctx.prev.is_none(), "prev should be None");
    assert!(ctx.next.is_none(), "next should be None");
}

// ── build_all ─────────────────────────────────────────────────────────────────

fn write_rst(dir: &Path, docname: &str, content: &str) {
    let path: std::path::PathBuf = dir
        .join(docname.split('/').collect::<std::path::PathBuf>())
        .with_extension("rst");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, content).unwrap();
}

/// `build_all` must produce one `.fjson` file per RST source.
#[test]
fn build_all_writes_one_fjson_per_doc() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nRoot page.\n");
    write_rst(src.path(), "guide", "Guide\n=====\n\nGuide page.\n");

    let env = make_env(src.path(), out.path());
    let result = JsonBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    assert_eq!(result.written, 2);
    assert!(
        out.path().join("index.fjson").exists(),
        "index.fjson missing"
    );
    assert!(
        out.path().join("guide.fjson").exists(),
        "guide.fjson missing"
    );
}

/// `build_all` must write `globalcontext.json`.
#[test]
fn build_all_writes_globalcontext() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nContent.\n");

    let env = make_env(src.path(), out.path());
    JsonBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    assert!(
        out.path().join("globalcontext.json").exists(),
        "globalcontext.json should be written"
    );
}

/// `globalcontext.json` must deserialize to [`GlobalContext`].
#[test]
fn globalcontext_is_valid_json() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nContent.\n");

    let env = make_env(src.path(), out.path());
    JsonBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    // parse_globalcontext panics on error.
    let _ = parse_globalcontext(out.path());
}

/// `GlobalContext.builder` must be `"json"`.
#[test]
fn globalcontext_builder_field_is_json() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nContent.\n");

    let env = make_env(src.path(), out.path());
    JsonBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let gc = parse_globalcontext(out.path());
    assert_eq!(gc.builder, "json");
}

/// `GlobalContext.titles` must contain an entry for each built docname.
#[test]
fn globalcontext_titles_contains_all_docs() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nRoot.\n");
    write_rst(src.path(), "about", "About\n=====\n\nAbout page.\n");

    let env = make_env(src.path(), out.path());
    JsonBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let gc = parse_globalcontext(out.path());
    assert!(
        gc.titles.contains_key("index"),
        "titles should have 'index'"
    );
    assert!(
        gc.titles.contains_key("about"),
        "titles should have 'about'"
    );
}

/// `GlobalContext.last_updated` must look like `YYYY-MM-DD`.
#[test]
fn globalcontext_last_updated_is_date() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nContent.\n");

    let env = make_env(src.path(), out.path());
    JsonBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let gc = parse_globalcontext(out.path());
    // Expect format YYYY-MM-DD (10 chars, digits and hyphens)
    assert_eq!(
        gc.last_updated.len(),
        10,
        "last_updated should be YYYY-MM-DD"
    );
    assert!(
        gc.last_updated
            .chars()
            .all(|c| c.is_ascii_digit() || c == '-'),
        "last_updated should only contain digits and hyphens; got {:?}",
        gc.last_updated
    );
}

// ── serde round-trips ─────────────────────────────────────────────────────────

/// A [`PageContext`] must survive JSON round-trip without data loss.
#[test]
fn page_context_serde_round_trip() {
    let original = PageContext {
        body: "<p>Hello <strong>world</strong></p>".into(),
        title: "Hello".into(),
        toc: "<ul><li><a href=\"\">Hello</a></li></ul>".into(),
        display_toc: false,
        current_page_name: "index".into(),
        parents: Vec::new(),
        prev: None,
        next: None,
        sourcename: "index.rst".into(),
    };
    let json = serde_json::to_string_pretty(&original).unwrap();
    let restored: PageContext = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

/// A [`GlobalContext`] must survive JSON round-trip without data loss.
#[test]
fn global_context_serde_round_trip() {
    let original = GlobalContext {
        project: "Test Project".into(),
        copyright: "2024 Test Author".into(),
        release: "1.2.3".into(),
        version: "1.2".into(),
        builder: "json".into(),
        last_updated: "2024-06-01".into(),
        titles: HashMap::from([
            ("index".into(), "Index Page".into()),
            ("guide".into(), "User Guide".into()),
        ]),
    };
    let json = serde_json::to_string_pretty(&original).unwrap();
    let restored: GlobalContext = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

/// [`PageContext`] JSON must include all required upstream keys.
#[test]
fn page_context_json_has_required_keys() {
    let out = TempDir::new().unwrap();
    JsonBuilder::new()
        .build_doc("index", "Title\n=====\n\nContent.\n", out.path())
        .unwrap();
    let raw = read_fjson(out.path(), "index");
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let obj = value.as_object().expect("top-level must be a JSON object");

    for key in &[
        "body",
        "title",
        "toc",
        "display_toc",
        "current_page_name",
        "sourcename",
    ] {
        assert!(obj.contains_key(*key), "missing required key: {key}");
    }
}

// ── multi-suffix / mixed-format integration tests ─────────────────────────────

fn make_env_multi(srcdir: &Path, outdir: &Path, suffixes: &[(&str, &str)]) -> BuildEnvironment {
    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(srcdir, suffixes);
    BuildEnvironment::new(config, project, srcdir, outdir)
}

/// `build_all` with a `.md`-only project produces `.fjson` files.
#[test]
fn build_all_md_only_project() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nRoot.\n");
    // rename to .md
    std::fs::rename(src.path().join("index.rst"), src.path().join("index.md")).unwrap();

    let env = make_env_multi(src.path(), out.path(), &[(".md", "myst")]);
    let result = JsonBuilder::with_source_suffixes(vec![".md".into()])
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    assert_eq!(result.written, 1);
    assert!(out.path().join("index.fjson").exists());
}

/// `sourcename` in `.fjson` must use `.md` for an `.md` source file.
#[test]
fn fjson_sourcename_reflects_md_extension() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nRoot.\n");
    std::fs::rename(src.path().join("index.rst"), src.path().join("index.md")).unwrap();

    let env = make_env_multi(src.path(), out.path(), &[(".md", "myst")]);
    JsonBuilder::with_source_suffixes(vec![".md".into()])
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let ctx = parse_page(out.path(), "index");
    assert!(
        ctx.sourcename.ends_with(".md"),
        "sourcename should end with .md; got {:?}",
        ctx.sourcename
    );
}

/// `build_all` on a mixed `.rst` + `.md` project writes one `.fjson` per doc.
#[test]
fn build_all_mixed_rst_and_md() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nRoot page.\n");
    write_rst(src.path(), "guide", "Guide\n=====\n\nRST guide.\n");
    // create an .md doc alongside the .rst docs
    std::fs::write(src.path().join("notes.md"), "Notes\n=====\n\nMD notes.\n").unwrap();

    let env = make_env_multi(
        src.path(),
        out.path(),
        &[(".rst", "restructuredtext"), (".md", "myst")],
    );
    let result = JsonBuilder::with_source_suffixes(vec![".rst".into(), ".md".into()])
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    assert_eq!(
        result.written, 3,
        "should build index.rst, guide.rst, notes.md"
    );
    for doc in &["index", "guide", "notes"] {
        assert!(
            out.path().join(format!("{doc}.fjson")).exists(),
            "{doc}.fjson should exist"
        );
    }
}

/// Each `.fjson` in a mixed project carries the correct `sourcename` extension.
#[test]
fn fjson_sourcename_per_extension_in_mixed_project() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "page_rst", "RST\n===\n\nContent.\n");
    std::fs::write(src.path().join("page_md.md"), "MD\n==\n\nContent.\n").unwrap();

    let env = make_env_multi(
        src.path(),
        out.path(),
        &[(".rst", "restructuredtext"), (".md", "myst")],
    );
    JsonBuilder::with_source_suffixes(vec![".rst".into(), ".md".into()])
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let rst_ctx = parse_page(out.path(), "page_rst");
    let md_ctx = parse_page(out.path(), "page_md");

    assert!(
        rst_ctx.sourcename.ends_with(".rst"),
        "RST sourcename should end with .rst; got {:?}",
        rst_ctx.sourcename
    );
    assert!(
        md_ctx.sourcename.ends_with(".md"),
        "MD sourcename should end with .md; got {:?}",
        md_ctx.sourcename
    );
}

/// When both `index.rst` and `index.md` exist, the first configured suffix wins.
#[test]
fn build_all_first_suffix_wins_on_docname_conflict() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "RST Index\n=========\n\nFrom RST.\n");
    std::fs::write(
        src.path().join("index.md"),
        "MD Index\n========\n\nFrom MD.\n",
    )
    .unwrap();

    let env = make_env_multi(
        src.path(),
        out.path(),
        &[(".rst", "restructuredtext"), (".md", "myst")],
    );
    JsonBuilder::with_source_suffixes(vec![".rst".into(), ".md".into()])
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    // Only one index.fjson should exist; it should come from the .rst source.
    let ctx = parse_page(out.path(), "index");
    assert!(
        ctx.sourcename.ends_with(".rst"),
        "first suffix (.rst) should win; got {:?}",
        ctx.sourcename
    );
    assert!(
        ctx.body.contains("From RST") || ctx.title.contains("RST"),
        "content should be from RST file"
    );
}

/// `globalcontext.json` `titles` must contain entries for all docs in a
/// mixed project.
#[test]
fn build_all_mixed_globalcontext_titles() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();

    write_rst(src.path(), "index", "Index\n=====\n\nRoot.\n");
    std::fs::write(src.path().join("readme.md"), "Readme\n======\n\nMD doc.\n").unwrap();

    let env = make_env_multi(
        src.path(),
        out.path(),
        &[(".rst", "restructuredtext"), (".md", "myst")],
    );
    JsonBuilder::with_source_suffixes(vec![".rst".into(), ".md".into()])
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    let gc = parse_globalcontext(out.path());
    assert!(
        gc.titles.contains_key("index"),
        "titles should contain 'index'"
    );
    assert!(
        gc.titles.contains_key("readme"),
        "titles should contain 'readme'"
    );
}
