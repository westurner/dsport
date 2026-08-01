//! `sphinxdocrs::builders::changes` — Rust port of
//! `sphinx.builders.changes.ChangesBuilder`.
//!
//! Scans every document's RST source for `.. versionadded::`,
//! `.. versionchanged::`, and `.. deprecated::` directives, groups the
//! resulting change entries by version, and writes a single HTML report
//! (`index.html`) listing them per version, in per-version reverse
//! (newest-first, string-sorted) order.
//!
//! **Accepted deviations:**
//! - Directive bodies are recovered with a text-level indentation scan
//!   (`scan_version_changes`), following the precedent set by
//!   `domains::scan`'s H3a/H3c/H5b scanners, since `docutilsrs`'s parser
//!   has no structural directive registry to hook a real
//!   `versionadded`/`versionchanged`/`deprecated` node onto (**H5a** is
//!   still deferred).
//! - Versions are sorted as plain strings (reverse-lexicographic), not
//!   parsed as PEP 440 / semver — matches upstream's own behaviour for
//!   non-numeric version strings but won't reorder `"2.10"` after
//!   `"2.9"` the way a numeric-aware sort would.
//! - Output is one flat `index.html`, not the module-grouped, module-doc
//!   cross-linked report upstream renders through a Jinja2 template — a
//!   plain `<ul>` per version, in document order within a version.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `ChangesBuilder.name` | `"changes"` | constant |
//! | `ChangesBuilder.format` | `""` | constant, matches upstream (not a document-per-page builder) |
//! | `ChangesBuilder.get_target_uri` | [`ChangesBuilder::get_target_uri`] | always `""`, matches upstream |
//! | `ChangesBuilder.find_versionchanges` + `write` | [`ChangesBuilder::build_all`] | scan + group by version + render `index.html` |

use std::collections::BTreeMap;
use std::path::Path;

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// One recovered `versionadded`/`versionchanged`/`deprecated` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionChange {
    /// `"versionadded"`, `"versionchanged"`, or `"deprecated"`.
    pub kind: &'static str,
    /// The version argument (e.g. `"1.2"`).
    pub version: String,
    /// The directive body text, dedented and joined with spaces.
    pub description: String,
}

/// Changes-report builder.
#[derive(Debug, Default)]
pub struct ChangesBuilder;

impl ChangesBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for ChangesBuilder {
    fn name(&self) -> &str {
        "changes"
    }
    fn format(&self) -> &str {
        ""
    }
    fn out_suffix(&self) -> &str {
        ""
    }
    fn get_target_uri(&self, _docname: &str) -> String {
        String::new()
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let mut by_version: BTreeMap<String, Vec<(String, VersionChange)>> = BTreeMap::new();
        for change in scan_version_changes(source) {
            by_version
                .entry(change.version.clone())
                .or_default()
                .push((docname.to_string(), change));
        }
        std::fs::create_dir_all(outdir)?;
        std::fs::write(
            outdir.join("index.html"),
            render_report(&by_version).as_bytes(),
        )?;
        Ok(())
    }

    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let mut result = BuildResult::default();
        let mut docnames: Vec<String> = if !env.all_docs.is_empty() {
            env.all_docs.keys().cloned().collect()
        } else {
            super::html::discover_rst_docnames_pub(srcdir)
        };
        docnames.sort();

        std::fs::create_dir_all(outdir)?;

        let mut by_version: BTreeMap<String, Vec<(String, VersionChange)>> = BTreeMap::new();
        for docname in &docnames {
            let src_path = srcdir.join(format!("{docname}.rst"));
            let source =
                crate::environment::read_source_file(&src_path, &env.config.source_encoding())
                    .map_err(|e| {
                        BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
                    })?;
            for change in scan_version_changes(&source) {
                by_version
                    .entry(change.version.clone())
                    .or_default()
                    .push((docname.clone(), change));
            }
            result.written += 1;
        }

        std::fs::write(
            outdir.join("index.html"),
            render_report(&by_version).as_bytes(),
        )?;
        Ok(result)
    }
}

/// Scan `source` for `.. versionadded::`/`.. versionchanged::`/
/// `.. deprecated::` directives, returning one [`VersionChange`] per
/// occurrence in source order.
pub fn scan_version_changes(source: &str) -> Vec<VersionChange> {
    let mut out = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        let indent = lines[i].len() - trimmed.len();
        let kind = ["versionadded", "versionchanged", "deprecated"]
            .into_iter()
            .find(|k| trimmed.starts_with(&format!(".. {k}::")));
        let Some(kind) = kind else {
            i += 1;
            continue;
        };
        let rest = trimmed[format!(".. {kind}::").len()..].trim();
        let version = rest.split_whitespace().next().unwrap_or("").to_string();

        // Collect the indented body block that follows.
        let mut j = i + 1;
        let mut body_lines: Vec<String> = Vec::new();
        while j < lines.len() {
            if lines[j].trim().is_empty() {
                // A single blank line may separate the directive from its
                // body; stop only on a *second* consecutive blank line or
                // a dedented non-blank line.
                if j + 1 >= lines.len() || lines[j + 1].trim().is_empty() {
                    break;
                }
                let next_indent = lines[j + 1].len() - lines[j + 1].trim_start().len();
                if next_indent <= indent {
                    break;
                }
                j += 1;
                continue;
            }
            let this_indent = lines[j].len() - lines[j].trim_start().len();
            if this_indent <= indent {
                break;
            }
            body_lines.push(lines[j].trim().to_string());
            j += 1;
        }
        let description = body_lines.join(" ");
        if !version.is_empty() {
            out.push(VersionChange {
                kind,
                version,
                description,
            });
        }
        i = j.max(i + 1);
    }
    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Render the grouped changes as a minimal HTML report, newest version
/// first (reverse string sort, see the module-level accepted deviation).
fn render_report(by_version: &BTreeMap<String, Vec<(String, VersionChange)>>) -> String {
    let mut body = String::new();
    for (version, entries) in by_version.iter().rev() {
        body.push_str(&format!(
            "<h2>Changes in version {}</h2>\n<ul>\n",
            html_escape(version)
        ));
        for (docname, change) in entries {
            body.push_str(&format!(
                "<li><strong>{}</strong> ({}): {}</li>\n",
                html_escape(change.kind),
                html_escape(docname),
                html_escape(&change.description)
            ));
        }
        body.push_str("</ul>\n");
    }
    format!(
        "<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"/><title>Changes</title></head>\n<body>\n<h1>Changes</h1>\n{body}</body></html>\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn name_format_and_suffix() {
        let b = ChangesBuilder::new();
        assert_eq!(b.name(), "changes");
        assert_eq!(b.format(), "");
        assert_eq!(b.out_suffix(), "");
    }

    #[test]
    fn scans_versionadded_versionchanged_and_deprecated() {
        let source = "\
Title
=====

.. versionadded:: 1.0
   Initial feature.

.. versionchanged:: 1.2
   Changed behavior
   across two lines.

.. deprecated:: 2.0
   Use something else instead.
";
        let changes = scan_version_changes(source);
        assert_eq!(changes.len(), 3);
        assert_eq!(changes[0].kind, "versionadded");
        assert_eq!(changes[0].version, "1.0");
        assert_eq!(changes[0].description, "Initial feature.");
        assert_eq!(changes[1].kind, "versionchanged");
        assert_eq!(changes[1].version, "1.2");
        assert_eq!(changes[1].description, "Changed behavior across two lines.");
        assert_eq!(changes[2].kind, "deprecated");
        assert_eq!(changes[2].version, "2.0");
    }

    #[test]
    fn no_changes_returns_empty() {
        assert!(scan_version_changes("Title\n=====\n\nBody.\n").is_empty());
    }

    #[test]
    fn build_doc_writes_index_html_with_entries() {
        let tmp = TempDir::new().unwrap();
        ChangesBuilder::new()
            .build_doc(
                "index",
                ".. versionadded:: 1.0\n   A new thing.\n",
                tmp.path(),
            )
            .unwrap();
        let html = std::fs::read_to_string(tmp.path().join("index.html")).unwrap();
        assert!(html.contains("Changes in version 1.0"));
        assert!(html.contains("A new thing."));
        assert!(html.contains("versionadded"));
    }

    #[test]
    fn build_all_groups_entries_by_version_across_documents() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Welcome\n=======\n\n.. versionadded:: 1.0\n   Feature A.\n",
        )
        .unwrap();
        std::fs::write(
            src.path().join("about.rst"),
            "About\n=====\n\n.. versionadded:: 1.0\n   Feature B.\n\n.. deprecated:: 2.0\n   Old API.\n",
        )
        .unwrap();
        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        let result = ChangesBuilder::new()
            .build_all(src.path(), out.path(), &env)
            .unwrap();
        assert_eq!(result.written, 2);
        let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(html.contains("Feature A."));
        assert!(html.contains("Feature B."));
        assert!(html.contains("Old API."));
        // Newest version (2.0) rendered before older version (1.0).
        assert!(html.find("2.0").unwrap() < html.find("1.0").unwrap());
    }

    #[test]
    fn get_target_uri_is_always_empty() {
        let b = ChangesBuilder::new();
        assert_eq!(b.get_target_uri("index"), "");
        assert_eq!(b.get_target_uri("guide/intro"), "");
    }
}
