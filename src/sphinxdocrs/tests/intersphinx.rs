//! Integration tests for `sphinxdocrs::intersphinx` inventory parsing.
//!
//! Mirrors the `InventoryFile` cases in
//! `sphinx/tests/test_util/test_util_inventory.py`.

use rstest::*;
use sphinxdocrs::intersphinx::{Inventory, InventoryEntry, InventoryError, dumps};
use tempfile::TempDir;

// ── Fixture helpers ───────────────────────────────────────────────────────────

fn entry(name: &str, item_type: &str, uri: &str, display_name: &str) -> InventoryEntry {
    InventoryEntry {
        name: name.to_owned(),
        item_type: item_type.to_owned(),
        priority: 1,
        uri: uri.to_owned(),
        display_name: display_name.to_owned(),
    }
}

/// A v2 inventory covering the shapes upstream exercises: a module, a class
/// with a `$`-abbreviated anchor, a name containing spaces, a `std:label`,
/// and a de-prioritised entry.
#[fixture]
fn inventory_v2() -> Vec<u8> {
    dumps(
        "foo",
        "2.0",
        &[
            entry(
                "module1",
                "py:module",
                "foo.html#module-module1",
                "Long Module desc",
            ),
            entry("module2", "py:module", "foo.html#module-module2", "module2"),
            entry(
                "module1.func",
                "py:function",
                "sub/foo.html#module1.func",
                "module1.func",
            ),
            entry("CFunc", "c:function", "cfunc.html#CFunc", "CFunc"),
            entry("a term", "std:term", "glossary.html#term-a-term", "a term"),
            entry(
                "The-Julia-Domain",
                "std:label",
                "write/latex.html#the-julia-domain",
                "The Julia Domain",
            ),
            InventoryEntry {
                priority: -1,
                ..entry("docname", "std:doc", "docname.html", "docname")
            },
        ],
    )
    .unwrap()
}

// ── header handling ───────────────────────────────────────────────────────────

#[test]
fn loads_rejects_a_missing_banner() {
    let err = Inventory::loads(b"not an inventory\n", "/util").unwrap_err();
    assert_eq!(
        err,
        InventoryError::InvalidHeader("not an inventory".to_owned())
    );
}

#[test]
fn loads_rejects_an_unsupported_version() {
    let err = Inventory::loads(b"# Sphinx inventory version 3\n", "/util").unwrap_err();
    assert_eq!(err, InventoryError::UnsupportedVersion("3".to_owned()));
    assert_eq!(
        err.to_string(),
        "unknown or unsupported inventory version: \"3\""
    );
}

#[test]
fn loads_rejects_a_truncated_v2_header() {
    let raw = b"# Sphinx inventory version 2\n# Project: foo\n# Version: 1.0\n";
    assert_eq!(
        Inventory::loads(raw, "/util").unwrap_err(),
        InventoryError::MissingProjectInfo
    );
}

#[test]
fn loads_rejects_an_uncompressed_v2_body() {
    let raw = b"# Sphinx inventory version 2\n\
                # Project: foo\n\
                # Version: 1.0\n\
                # The remainder of this file is plain text.\n\
                payload";
    assert_eq!(
        Inventory::loads(raw, "/util").unwrap_err(),
        InventoryError::NotCompressed("# The remainder of this file is plain text.".to_owned())
    );
}

#[test]
fn loads_rejects_a_corrupt_zlib_payload() {
    let mut raw = b"# Sphinx inventory version 2\n\
                    # Project: foo\n\
                    # Version: 1.0\n\
                    # The remainder of this file is compressed using zlib.\n"
        .to_vec();
    raw.extend_from_slice(b"\x78\x9cnot-really-deflate");
    assert!(matches!(
        Inventory::loads(&raw, "/util").unwrap_err(),
        InventoryError::Decompress(_)
    ));
}

#[test]
fn loads_tolerates_a_crlf_banner() {
    let raw = b"# Sphinx inventory version 1\r\nProject: foo\nVersion: 1.0\n";
    assert!(Inventory::loads(raw, "/util").is_ok());
}

// ── version 2 payloads ────────────────────────────────────────────────────────

#[rstest]
fn v2_reads_project_metadata(inventory_v2: Vec<u8>) {
    let inv = Inventory::loads(&inventory_v2, "/util").unwrap();
    let item = inv.get("py:module", "module1").unwrap();
    assert_eq!(item.project_name, "foo");
    assert_eq!(item.project_version, "2.0");
}

#[rstest]
fn v2_joins_locations_against_the_base_uri(inventory_v2: Vec<u8>) {
    let inv = Inventory::loads(&inventory_v2, "/util").unwrap();
    assert_eq!(
        inv.get("py:module", "module1").unwrap().uri,
        "/util/foo.html#module-module1"
    );
    assert_eq!(
        inv.get("py:function", "module1.func").unwrap().uri,
        "/util/sub/foo.html#module1.func"
    );
}

#[rstest]
fn v2_expands_dollar_abbreviated_anchors(inventory_v2: Vec<u8>) {
    // `dumps` shortens `cfunc.html#CFunc` to `cfunc.html#$`; `loads` must
    // expand it back using the object name.
    let inv = Inventory::loads(&inventory_v2, "/util").unwrap();
    assert_eq!(
        inv.get("c:function", "CFunc").unwrap().uri,
        "/util/cfunc.html#CFunc"
    );
}

#[rstest]
fn v2_handles_names_containing_spaces(inventory_v2: Vec<u8>) {
    let inv = Inventory::loads(&inventory_v2, "/util").unwrap();
    let item = inv.get("std:term", "a term").unwrap();
    assert_eq!(item.uri, "/util/glossary.html#term-a-term");
    assert_eq!(item.display_name, "-");
}

#[rstest]
fn v2_keeps_display_names_that_differ_from_the_object_name(inventory_v2: Vec<u8>) {
    let inv = Inventory::loads(&inventory_v2, "/util").unwrap();
    assert_eq!(
        inv.get("py:module", "module1").unwrap().display_name,
        "Long Module desc"
    );
    assert_eq!(
        inv.get("std:label", "The-Julia-Domain")
            .unwrap()
            .display_name,
        "The Julia Domain"
    );
}

#[rstest]
fn v2_accepts_negative_priorities(inventory_v2: Vec<u8>) {
    let inv = Inventory::loads(&inventory_v2, "/util").unwrap();
    assert_eq!(
        inv.get("std:doc", "docname").unwrap().uri,
        "/util/docname.html"
    );
}

#[rstest]
fn v2_groups_objects_by_qualified_type(inventory_v2: Vec<u8>) {
    let inv = Inventory::loads(&inventory_v2, "/util").unwrap();
    let types: Vec<&str> = inv.types().collect();
    assert_eq!(
        types,
        [
            "c:function",
            "py:function",
            "py:module",
            "std:doc",
            "std:label",
            "std:term"
        ]
    );
    assert_eq!(inv.objects_of_type("py:module").unwrap().len(), 2);
    assert_eq!(inv.len(), 7);
    assert!(!inv.is_empty());
    assert_eq!(inv.iter().count(), 7);
}

#[test]
fn v2_skips_entries_with_an_unqualified_type() {
    // `notadomain` has no `:`, so upstream drops the row rather than risk a
    // regex backtracking blow-up.
    let raw = dumps(
        "foo",
        "1.0",
        &[
            entry("bad", "notadomain", "bad.html", "-"),
            entry("good", "py:class", "good.html", "-"),
        ],
    )
    .unwrap();
    let inv = Inventory::loads(&raw, "/util").unwrap();
    assert_eq!(inv.len(), 1);
    assert!(inv.get("py:class", "good").is_some());
}

#[test]
fn v2_keeps_the_first_of_duplicated_py_module_entries() {
    // Sphinx ≤ 1.1 emitted two rows per module; the first is the correct one.
    let raw = dumps(
        "foo",
        "1.0",
        &[
            entry("module1", "py:module", "first.html", "-"),
            entry("module1", "py:module", "second.html", "-"),
        ],
    )
    .unwrap();
    let inv = Inventory::loads(&raw, "/util").unwrap();
    assert_eq!(
        inv.get("py:module", "module1").unwrap().uri,
        "/util/first.html"
    );
}

#[test]
fn v2_reports_case_only_label_ambiguities() {
    let raw = dumps(
        "foo",
        "1.0",
        &[
            entry("Label", "std:label", "one.html", "-"),
            entry("label", "std:label", "two.html", "-"),
            // Same target under both spellings — a duplicate, not an ambiguity.
            entry("Term", "std:term", "t.html", "-"),
            entry("term", "std:term", "t.html", "-"),
        ],
    )
    .unwrap();
    let inv = Inventory::loads(&raw, "/util").unwrap();
    assert_eq!(inv.ambiguities(), ["std:label:label"]);
}

// ── version 1 payloads ────────────────────────────────────────────────────────

#[test]
fn v1_synthesises_anchors_and_qualifies_types() {
    let raw = b"# Sphinx inventory version 1\n\
                # Project: foo\n\
                # Version: 1.0\n\
                module mod foo.html\n\
                module.cls class foo.html\n";
    let inv = Inventory::loads(raw, "/util").unwrap();

    let module = inv.get("py:module", "module").unwrap();
    assert_eq!(module.uri, "/util/foo.html#module-module");
    assert_eq!(module.display_name, "-");
    assert_eq!(module.project_name, "foo");
    assert_eq!(module.project_version, "1.0");

    assert_eq!(
        inv.get("py:class", "module.cls").unwrap().uri,
        "/util/foo.html#module.cls"
    );
}

#[test]
fn v1_rejects_a_body_without_project_lines() {
    assert_eq!(
        Inventory::loads(b"# Sphinx inventory version 1\n# Project: foo\n", "/util").unwrap_err(),
        InventoryError::MissingProjectInfo
    );
}

// ── round trip and file loading ───────────────────────────────────────────────

#[rstest]
fn dumps_then_loads_round_trips(inventory_v2: Vec<u8>) {
    let inv = Inventory::loads(&inventory_v2, "").unwrap();
    let re_dumped = dumps(
        "foo",
        "2.0",
        &inv.iter()
            .map(|(item_type, name, item)| InventoryEntry {
                name: name.to_owned(),
                item_type: item_type.to_owned(),
                priority: 1,
                uri: item.uri.clone(),
                display_name: item.display_name.clone(),
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    assert_eq!(Inventory::loads(&re_dumped, "").unwrap(), inv);
}

#[rstest]
fn load_file_reads_a_cached_objects_inv(inventory_v2: Vec<u8>) {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("objects.inv");
    std::fs::write(&path, &inventory_v2).unwrap();

    let inv = Inventory::load_file(&path, "https://example.invalid/docs").unwrap();
    assert_eq!(
        inv.get("py:module", "module2").unwrap().uri,
        "https://example.invalid/docs/foo.html#module-module2"
    );
}

#[test]
fn load_file_surfaces_a_parse_error_as_invalid_data() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("objects.inv");
    std::fs::write(&path, b"garbage\n").unwrap();

    let err = Inventory::load_file(&path, "/util").unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}

// ── parity with upstream ──────────────────────────────────────────────────────

/// `tests/fixtures/objects_v2.inv` was written by `zlib` and parsed with
/// upstream `sphinx.util.inventory.InventoryFile.loads(uri='/util')`; the
/// table below is that parse verbatim. Exact parity.
#[test]
fn v2_matches_upstream_inventoryfile_loads() {
    const EXPECTED: &[(&str, &str, &str, &str)] = &[
        ("c:function", "CFunc", "/util/cfunc.html#CFunc", "-"),
        (
            "cpp:class",
            "a::template<C,B>",
            "/util/index.html#template",
            "-",
        ),
        ("cpp:type", "std", "/util/index.html#std", "-"),
        (
            "cpp:type",
            "std::uint8_t",
            "/util/index.html#std_uint8_t",
            "-",
        ),
        (
            "py:function",
            "module1.func",
            "/util/sub/foo.html#module1.func",
            "-",
        ),
        (
            "py:method",
            "module1.Foo.bar",
            "/util/sub/foo.html#module1.Foo.bar",
            "-",
        ),
        (
            "py:module",
            "module1",
            "/util/foo.html#module-module1",
            "Long Module desc",
        ),
        ("py:module", "module2", "/util/foo.html#module-module2", "-"),
        ("std:doc", "docname", "/util/docname.html", "-"),
        // The object literally named `$`: the trailing-`$` abbreviation
        // expands to the name, reproducing `#$`.
        ("std:label", "$", "/util/index.html#$", "Dollar"),
        (
            "std:label",
            "The-Julia-Domain",
            "/util/write/latex.html#the-julia-domain",
            "The Julia Domain",
        ),
        (
            "std:option",
            "-writer-rst",
            "/util/index.html#cmdoption-writer-rst",
            "-",
        ),
        ("std:term", "a term", "/util/glossary.html#term-a-term", "-"),
    ];

    let raw = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/objects_v2.inv"
    ))
    .unwrap();
    let inv = Inventory::loads(&raw, "/util").unwrap();

    let actual: Vec<(&str, &str, &str, &str)> = inv
        .iter()
        .map(|(item_type, name, item)| {
            (
                item_type,
                name,
                item.uri.as_str(),
                item.display_name.as_str(),
            )
        })
        .collect();
    assert_eq!(actual, EXPECTED);

    for (_, _, item) in inv.iter() {
        assert_eq!(item.project_name, "foo");
        assert_eq!(item.project_version, "2.0");
    }
}
