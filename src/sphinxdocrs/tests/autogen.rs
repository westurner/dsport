//! `sphinx-autogen-rs` integration tests — rstest + cargo-insta.
//!
//! Covers: parser flags, `find_autosummary_in_lines` parametrized
//! parity cases, `find_autosummary_in_files` on real temp files,
//! template rendering snapshot, help-text snapshot.

use std::path::PathBuf;

use rstest::*;
use tempfile::TempDir;

use sphinxdocrs::autogen::generate::{
    ObjType, generate_stub, generate_stubs, infer_obj_type, split_fqn,
};
use sphinxdocrs::autogen::parse_args;
use sphinxdocrs::autogen::scan::{
    AutosummaryEntry, find_autosummary_in_files, find_autosummary_in_lines,
};
use sphinxdocrs::autogen::templates::AutogenTemplates;

// ── helpers ───────────────────────────────────────────────────────────────────

fn args(s: &str) -> Vec<String> {
    s.split_whitespace().map(String::from).collect()
}

fn lines(s: &str) -> Vec<String> {
    s.lines().map(String::from).collect()
}

#[fixture]
#[once]
fn templates() -> AutogenTemplates {
    AutogenTemplates::vendored()
}

// ── parser ────────────────────────────────────────────────────────────────────

#[test]
fn parser_required_source_file() {
    let a = parse_args(&args("source.rst")).unwrap();
    assert_eq!(a.source_files, [PathBuf::from("source.rst")]);
    assert_eq!(a.suffix, "rst");
    assert!(a.output_dir.is_none());
}

#[test]
fn parser_output_dir() {
    let a = parse_args(&args("src.rst -o /out")).unwrap();
    assert_eq!(a.output_dir, Some(PathBuf::from("/out")));
}

#[test]
fn parser_suffix_strips_dot() {
    let a = parse_args(&args("src.rst -s .txt")).unwrap();
    assert_eq!(a.suffix, "txt");
}

#[rstest]
#[case("-i", true, false, false)]
#[case("-a", false, true, false)]
#[case("--remove-old", false, false, true)]
fn parser_bool_flags(
    #[case] flag: &str,
    #[case] imported: bool,
    #[case] all_: bool,
    #[case] remove: bool,
) {
    let a = parse_args(&args(&format!("src.rst {flag}"))).unwrap();
    assert_eq!(a.imported_members, imported);
    assert_eq!(a.respect_module_all, all_);
    assert_eq!(a.remove_old, remove);
}

// ── find_autosummary_in_lines ─────────────────────────────────────────────────

#[rstest]
#[case(
    ".. autosummary::\n\n   Foo\n   Bar\n",
    &["Foo", "Bar"]
)]
#[case(
    ".. autosummary::\n   :toctree: api\n\n   mymod.Thing\n",
    &["mymod.Thing"]
)]
fn scan_basic_entries(#[case] rst: &str, #[case] expected: &[&str]) {
    let entries = find_autosummary_in_lines(&lines(rst), None, None);
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, expected, "rst={rst:?}");
}

#[test]
fn scan_recursive_flag() {
    let rst = ".. autosummary::\n   :toctree: api\n   :recursive:\n\n   mymod\n";
    let entries = find_autosummary_in_lines(&lines(rst), None, None);
    assert_eq!(entries.len(), 1);
    assert!(entries[0].recursive);
}

#[test]
fn scan_template_option() {
    let rst = ".. autosummary::\n   :template: custom.rst\n\n   mymod.Foo\n";
    let entries = find_autosummary_in_lines(&lines(rst), None, None);
    assert_eq!(entries[0].template, "custom.rst");
}

#[test]
fn scan_currentmodule_scopes_names() {
    let rst = ".. currentmodule:: mypackage\n\n.. autosummary::\n\n   Foo\n   Bar\n";
    let entries = find_autosummary_in_lines(&lines(rst), None, None);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "mypackage.Foo");
    assert_eq!(entries[1].name, "mypackage.Bar");
}

#[test]
fn scan_tilde_stripped() {
    let rst = ".. autosummary::\n\n   ~mymod.LongClass\n";
    let entries = find_autosummary_in_lines(&lines(rst), None, None);
    assert_eq!(entries[0].name, "mymod.LongClass");
}

#[test]
fn scan_no_entries_empty() {
    let rst = "Plain text.\n\n.. note::\n   Not autosummary.\n";
    assert!(find_autosummary_in_lines(&lines(rst), None, None).is_empty());
}

#[test]
fn scan_multiple_blocks() {
    let rst = ".. autosummary::\n\n   Foo\n\nSome text.\n\n.. autosummary::\n   :toctree: api\n\n   Bar\n";
    let entries = find_autosummary_in_lines(&lines(rst), None, None);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "Foo");
    assert_eq!(entries[1].name, "Bar");
}

// ── find_autosummary_in_files ─────────────────────────────────────────────────

#[test]
fn scan_files_reads_entries() {
    let tmp = TempDir::new().unwrap();
    let f1 = tmp.path().join("doc1.rst");
    let f2 = tmp.path().join("doc2.rst");
    std::fs::write(&f1, ".. autosummary::\n\n   mymod.Foo\n").unwrap();
    std::fs::write(&f2, ".. autosummary::\n\n   other.Bar\n").unwrap();
    let entries = find_autosummary_in_files(&[&f1, &f2]);
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&"mymod.Foo"), "{names:?}");
    assert!(names.contains(&"other.Bar"), "{names:?}");
}

// ── template snapshots ────────────────────────────────────────────────────────

#[rstest]
fn base_template_snapshot(templates: &AutogenTemplates) {
    use serde_json::json;
    let ctx = json!({
        "fullname": "mymod.MyClass",
        "module": "mymod",
        "objname": "MyClass",
        "objtype": "class",
    });
    let rendered = templates.render("base.rst", &ctx).unwrap();
    insta::assert_snapshot!(rendered);
}

// ── help snapshot ─────────────────────────────────────────────────────────────

#[test]
fn autogen_help_snapshot() {
    let mut cmd = sphinxdocrs::autogen::parser::build_parser();
    insta::assert_snapshot!(cmd.render_help().to_string());
}

// ── generate: infer_obj_type + split_fqn ─────────────────────────────────────

#[rstest]
#[case("sphinx.cmd.build", ObjType::Module)]
#[case("mypkg", ObjType::Module)]
#[case("MyClass", ObjType::Class)]
#[case("sphinx.cmd.build.Sphinx", ObjType::Class)]
#[case("sphinx.cmd.build.BuildArgs", ObjType::Class)]
fn integration_infer_obj_type(#[case] name: &str, #[case] want: ObjType) {
    assert_eq!(infer_obj_type(name), want, "name={name}");
}

#[test]
fn integration_split_fqn_dotted() {
    let (m, q) = split_fqn("sphinx.cmd.build");
    assert_eq!(m, "sphinx.cmd");
    assert_eq!(q, "build");
}

#[test]
fn integration_split_fqn_simple() {
    let (m, q) = split_fqn("mypkg");
    assert_eq!(m, "");
    assert_eq!(q, "mypkg");
}

// ── generate_stub integration ─────────────────────────────────────────────────

#[rstest]
fn generate_stub_module_rst(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let entry = AutosummaryEntry {
        name: "sphinx.cmd.build".to_string(),
        toctree: None,
        template: String::new(),
        recursive: false,
    };
    let path = generate_stub(&entry, tmp.path(), "rst", true, templates)
        .unwrap()
        .unwrap();
    assert_eq!(path.file_name().unwrap(), "sphinx.cmd.build.rst");
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(
        content.contains(".. automodule:: sphinx.cmd.build"),
        "got:\n{content}"
    );
    assert!(content.contains("sphinx.cmd.build"), "heading missing");
}

#[rstest]
fn generate_stub_class_rst(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let entry = AutosummaryEntry {
        name: "mypkg.MyHelper".to_string(),
        toctree: None,
        template: String::new(),
        recursive: false,
    };
    let path = generate_stub(&entry, tmp.path(), "rst", true, templates)
        .unwrap()
        .unwrap();
    assert_eq!(path.file_name().unwrap(), "mypkg.MyHelper.rst");
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(
        content.contains(".. autoclass:: MyHelper"),
        "got:\n{content}"
    );
    assert!(content.contains(".. currentmodule:: mypkg"));
}

#[rstest]
fn generate_stub_suffix_leading_dot(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let entry = AutosummaryEntry {
        name: "mymod".to_string(),
        toctree: None,
        template: String::new(),
        recursive: false,
    };
    // suffix with leading dot should be normalised
    let path = generate_stub(&entry, tmp.path(), ".txt", true, templates)
        .unwrap()
        .unwrap();
    assert_eq!(path.file_name().unwrap(), "mymod.txt");
}

#[rstest]
fn generate_stub_no_overwrite(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("pkg.mod.rst");
    std::fs::write(&file, "original\n").unwrap();
    let entry = AutosummaryEntry {
        name: "pkg.mod".to_string(),
        toctree: None,
        template: String::new(),
        recursive: false,
    };
    // Different content + overwrite=false → should skip (return None)
    let result = generate_stub(&entry, tmp.path(), "rst", false, templates).unwrap();
    assert!(result.is_none(), "expected skip, got {result:?}");
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "original\n");
}

// ── generate_stubs batch integration ─────────────────────────────────────────

#[rstest]
fn generate_stubs_writes_all(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let entries = vec![
        AutosummaryEntry {
            name: "pkg.mod_a".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        },
        AutosummaryEntry {
            name: "pkg.ClassB".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        },
    ];
    let generated = generate_stubs(&entries, tmp.path(), "rst", true, false, templates);
    assert_eq!(generated.len(), 2);
    assert!(tmp.path().join("pkg.mod_a.rst").exists());
    assert!(tmp.path().join("pkg.ClassB.rst").exists());
}

#[rstest]
fn generate_stubs_remove_old_deletes_stale(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let stale = tmp.path().join("stale.rst");
    std::fs::write(&stale, "old\n").unwrap();
    let entries = vec![AutosummaryEntry {
        name: "pkg.fresh".to_string(),
        toctree: None,
        template: String::new(),
        recursive: false,
    }];
    generate_stubs(&entries, tmp.path(), "rst", true, true, templates);
    assert!(!stale.exists(), "stale.rst should have been removed");
    assert!(tmp.path().join("pkg.fresh.rst").exists());
}

// ── stub content snapshot ─────────────────────────────────────────────────────

#[rstest]
fn generate_stub_module_snapshot(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let entry = AutosummaryEntry {
        name: "mypkg.helpers".to_string(),
        toctree: None,
        template: String::new(),
        recursive: false,
    };
    generate_stub(&entry, tmp.path(), "rst", true, templates)
        .unwrap()
        .unwrap();
    let content = std::fs::read_to_string(tmp.path().join("mypkg.helpers.rst")).unwrap();
    insta::assert_snapshot!(content);
}

#[rstest]
fn generate_stub_class_snapshot(templates: &AutogenTemplates) {
    let tmp = TempDir::new().unwrap();
    let entry = AutosummaryEntry {
        name: "mypkg.MyClass".to_string(),
        toctree: None,
        template: String::new(),
        recursive: false,
    };
    generate_stub(&entry, tmp.path(), "rst", true, templates)
        .unwrap()
        .unwrap();
    let content = std::fs::read_to_string(tmp.path().join("mypkg.MyClass.rst")).unwrap();
    insta::assert_snapshot!(content);
}
