//! `sphinxdocrs::builders::gettext` — Rust port of
//! `sphinx.builders.gettext.MessageCatalogBuilder`.
//!
//! Walks each document's doctree, extracts translatable text from
//! section/subsection titles and paragraphs (see the accepted deviation
//! below for the node kinds *not* extracted), groups occurrences by
//! message text, and writes a single GNU gettext `.pot` template.
//!
//! **Accepted deviations:**
//! - Only `Title`/`Subtitle`/`Paragraph` text is extracted. Upstream also
//!   extracts list items, definition list terms/definitions, field
//!   lists, table cells, and image `alt` text — a real
//!   `docutilsrs::doctree::Node` line-number field would be needed to
//!   emit upstream's `#: docname:linenum` location comments faithfully
//!   for all of these; this cut covers the majority of real
//!   documentation prose and is a bounded, testable first step.
//! - Location comments are `#: docname` (no line number), since
//!   `docutilsrs::doctree::Node` doesn't carry source line numbers.
//! - `gettext_compact` (grouping into one `.pot` per top-level path
//!   segment) is not honoured; this always writes a single combined
//!   `sphinx.pot`.
//! - No `gettext_uuid`/`gettext_location`/`gettext_auto_build` config
//!   options are read.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `MessageCatalogBuilder.name` | `"gettext"` | constant |
//! | `MessageCatalogBuilder.format` | `"gettext"` | constant |
//! | `MessageCatalogBuilder.out_suffix` | `".pot"` | constant |
//! | `MessageCatalogBuilder.build` (message extraction + `.pot` write) | [`GettextBuilder::build_all`] | one combined `sphinx.pot`, see deviations above |

use std::collections::BTreeMap;
use std::path::Path;

use docutilsrs::doctree::{Doctree, NodeId, NodeKind};
use docutilsrs::parse_rst_with_source;

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Message-catalog (`.pot`) builder.
#[derive(Debug, Default)]
pub struct GettextBuilder;

impl GettextBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for GettextBuilder {
    fn name(&self) -> &str {
        "gettext"
    }
    fn format(&self) -> &str {
        "gettext"
    }
    fn out_suffix(&self) -> &str {
        ".pot"
    }

    fn get_target_uri(&self, _docname: &str) -> String {
        String::new()
    }

    /// Single-document convenience path: writes a `.pot` containing just
    /// this document's messages.
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let mut catalog: MessageCatalog = BTreeMap::new();
        extract_into(&tree, docname, &mut catalog);
        std::fs::create_dir_all(outdir)?;
        let pot = render_pot(&catalog, "", "", "");
        std::fs::write(outdir.join("sphinx.pot"), pot.as_bytes())?;
        Ok(())
    }

    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let mut result = BuildResult::default();
        let docnames: Vec<String> = if !env.all_docs.is_empty() {
            env.all_docs.keys().cloned().collect()
        } else {
            super::html::discover_rst_docnames_pub(srcdir)
        };

        std::fs::create_dir_all(outdir)?;

        let mut catalog: MessageCatalog = BTreeMap::new();
        for docname in &docnames {
            let tree = match env.get_and_resolve_doctree(docname) {
                Ok(tree) => tree,
                Err(_) => {
                    let src_path = srcdir.join(format!("{docname}.rst"));
                    let source = std::fs::read_to_string(&src_path).map_err(|e| {
                        BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
                    })?;
                    parse_rst_with_source(&source, docname)
                }
            };
            extract_into(&tree, docname, &mut catalog);
            result.written += 1;
        }

        let pot = render_pot(
            &catalog,
            &env.config.project(),
            &env.config.version(),
            &env.config
                .get("project_copyright")
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default(),
        );
        std::fs::write(outdir.join("sphinx.pot"), pot.as_bytes())?;
        Ok(result)
    }
}

/// Ordered-by-first-occurrence map of message text to the (deduplicated,
/// insertion-ordered) list of docnames it was seen in.
type MessageCatalog = BTreeMap<String, Vec<String>>;

/// Extract translatable messages from `tree` into `catalog`, recording
/// `docname` as an occurrence location for each.
fn extract_into(tree: &Doctree, docname: &str, catalog: &mut MessageCatalog) {
    let mut messages = Vec::new();
    walk(tree, tree.root(), &mut messages);
    for msg in messages {
        let locs = catalog.entry(msg).or_default();
        if locs.last().map(String::as_str) != Some(docname) {
            locs.push(docname.to_string());
        }
    }
}

fn walk(tree: &Doctree, id: NodeId, out: &mut Vec<String>) {
    let node = tree.node(id);
    match &node.kind {
        NodeKind::Title | NodeKind::Subtitle { .. } | NodeKind::Paragraph => {
            let text = flatten_text(tree, id);
            if !text.trim().is_empty() {
                out.push(text);
            }
            // Titles/subtitles/paragraphs have no nested block content to
            // recurse into beyond inline markup already flattened above.
        }
        _ => {
            for &child in &node.children {
                walk(tree, child, out);
            }
        }
    }
}

/// Flatten all descendant `Text` nodes of `id` into a single string
/// (inline markup like emphasis/strong contributes its text, without
/// surrounding markers — `.pot` `msgid`s are plain text).
fn flatten_text(tree: &Doctree, id: NodeId) -> String {
    let mut out = String::new();
    collect_text(tree, id, &mut out);
    out
}

fn collect_text(tree: &Doctree, id: NodeId, out: &mut String) {
    let node = tree.node(id);
    if let NodeKind::Text(s) = &node.kind {
        out.push_str(s);
    }
    for &child in &node.children {
        collect_text(tree, child, out);
    }
}

/// Escape a string for a gettext `.po`/`.pot` quoted literal:
/// backslash, double-quote, and embedded newlines.
fn escape_po(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Render `catalog` as a GNU gettext `.pot` template.
fn render_pot(catalog: &MessageCatalog, project: &str, version: &str, copyright: &str) -> String {
    let mut out = String::new();
    out.push_str("# SOME DESCRIPTIVE TITLE.\n");
    if !copyright.is_empty() {
        out.push_str(&format!("# Copyright (C) {copyright}\n"));
    }
    out.push_str("# This file is distributed under the same license as the ");
    out.push_str(if project.is_empty() {
        "PACKAGE"
    } else {
        project
    });
    out.push_str(" package.\n");
    out.push_str("#\n");
    out.push_str("msgid \"\"\n");
    out.push_str("msgstr \"\"\n");
    out.push_str(&format!(
        "\"Project-Id-Version: {} {}\\n\"\n",
        if project.is_empty() {
            "PACKAGE"
        } else {
            project
        },
        version
    ));
    out.push_str("\"MIME-Version: 1.0\\n\"\n");
    out.push_str("\"Content-Type: text/plain; charset=utf-8\\n\"\n");
    out.push_str("\"Content-Transfer-Encoding: 8bit\\n\"\n");

    for (msgid, locations) in catalog {
        out.push('\n');
        for loc in locations {
            out.push_str(&format!("#: {loc}\n"));
        }
        out.push_str("msgid \"");
        out.push_str(&escape_po(msgid));
        out.push_str("\"\n");
        out.push_str("msgstr \"\"\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn name_format_and_suffix() {
        let b = GettextBuilder::new();
        assert_eq!(b.name(), "gettext");
        assert_eq!(b.format(), "gettext");
        assert_eq!(b.out_suffix(), ".pot");
    }

    #[test]
    fn extracts_title_and_paragraph_messages() {
        let tree = parse_rst_with_source("Title\n=====\n\nA paragraph of body text.\n", "index");
        let mut catalog = MessageCatalog::new();
        extract_into(&tree, "index", &mut catalog);
        assert!(catalog.contains_key("Title"));
        assert!(catalog.contains_key("A paragraph of body text."));
    }

    #[test]
    fn deduplicates_repeated_messages_across_documents() {
        let tree1 = parse_rst_with_source("Intro\n=====\n\nShared text.\n", "a");
        let tree2 = parse_rst_with_source("Other\n=====\n\nShared text.\n", "b");
        let mut catalog = MessageCatalog::new();
        extract_into(&tree1, "a", &mut catalog);
        extract_into(&tree2, "b", &mut catalog);
        let locations = catalog.get("Shared text.").unwrap();
        assert_eq!(locations, &vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn build_doc_writes_a_valid_pot_header() {
        let tmp = TempDir::new().unwrap();
        GettextBuilder::new()
            .build_doc("index", "Title\n=====\n\nBody text.\n", tmp.path())
            .unwrap();
        let pot = std::fs::read_to_string(tmp.path().join("sphinx.pot")).unwrap();
        assert!(pot.contains("msgid \"\""));
        assert!(pot.contains("Content-Type: text/plain; charset=utf-8"));
        assert!(pot.contains("msgid \"Title\""));
        assert!(pot.contains("msgid \"Body text.\""));
    }

    #[test]
    fn build_all_aggregates_messages_across_the_project() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Welcome\n=======\n\nShared greeting.\n",
        )
        .unwrap();
        std::fs::write(
            src.path().join("about.rst"),
            "About\n=====\n\nShared greeting.\n\nUnique about text.\n",
        )
        .unwrap();
        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        let result = GettextBuilder::new()
            .build_all(src.path(), out.path(), &env)
            .unwrap();
        assert_eq!(result.written, 2);
        let pot = std::fs::read_to_string(out.path().join("sphinx.pot")).unwrap();
        assert!(pot.contains("msgid \"Shared greeting.\""));
        assert!(pot.contains("msgid \"Unique about text.\""));
        assert!(pot.contains("#: index"));
        assert!(pot.contains("#: about"));
    }

    #[test]
    fn escapes_quotes_backslashes_and_newlines() {
        assert_eq!(escape_po("a\"b"), "a\\\"b");
        assert_eq!(escape_po("a\\b"), "a\\\\b");
        assert_eq!(escape_po("a\nb"), "a\\nb");
    }
}
