//! Integration tests for `sphinxdocrs::builders::xml::XmlBuilder` (H7a).
//!
//! Complements the inline `#[cfg(test)]` unit tests in
//! `src/builders/xml.rs` with a black-box, multi-document project build
//! and a structural well-formedness check (every opened tag is closed,
//! in stack order) on the aggregate build output.

use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::xml::XmlBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

fn assert_well_formed(xml: &str) {
    let mut stack: Vec<String> = Vec::new();
    for line in xml.lines().skip(2) {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("</") {
            let name = rest.trim_end_matches('>').to_string();
            assert_eq!(stack.pop(), Some(name), "mismatched closing tag in: {xml}");
        } else if let Some(rest) = trimmed.strip_prefix('<') {
            if let Some(name_end) = rest.find([' ', '>']) {
                stack.push(rest[..name_end].to_string());
            }
        }
    }
    assert!(stack.is_empty(), "unclosed tags remain: {stack:?}");
}

#[test]
fn builder_identity() {
    let b = XmlBuilder::new();
    assert_eq!(b.name(), "xml");
    assert_eq!(b.format(), "xml");
    assert_eq!(b.out_suffix(), ".xml");
}

#[test]
fn build_all_over_a_multi_document_project_produces_well_formed_xml() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::create_dir(src.path().join("guide")).unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Welcome\n=======\n\nHomepage & <content>.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("guide/intro.rst"),
        "Intro\n=====\n\n- one\n- two\n\n::\n\n    code block\n",
    )
    .unwrap();

    let config = SphinxConfig::new_defaults();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();

    let result = XmlBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 2);

    let index_xml = std::fs::read_to_string(out.path().join("index.xml")).unwrap();
    assert_well_formed(&index_xml);
    // '&' and '<' in the source text must come through XML-escaped.
    assert!(index_xml.contains("Homepage &amp; &lt;content&gt;."));

    let intro_xml = std::fs::read_to_string(out.path().join("guide/intro.xml")).unwrap();
    assert_well_formed(&intro_xml);
    assert!(intro_xml.contains("code block"));
}

#[test]
fn get_target_uri_matches_upstream_xmlbuilder() {
    let b = XmlBuilder::new();
    assert_eq!(b.get_target_uri("index"), "index.xml");
    assert_eq!(b.get_target_uri("guide/intro"), "guide/intro.xml");
}
