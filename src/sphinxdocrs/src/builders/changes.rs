//! `sphinxdocrs::builders::changes` — Rust port of
//! `sphinx.builders.changes.ChangesBuilder`.
//!
//! Scans every document's RST source for `.. versionadded::`,
//! `.. versionchanged::`, and `.. deprecated::` directives, groups the
//! resulting change entries by version, and writes the upstream-shaped
//! frameset (`index.html`), report (`changes.html`), and source pages under
//! `rst/`, in per-version reverse (newest-first, string-sorted) order.
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
//! - The report groups entries by source document under "Other changes";
//!   module and C API classification is not recovered from the text scanner.
//! - Theme assets are rendered from vendored static sources with native
//!   default theme values rather than through the full Jinja template bridge.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `ChangesBuilder.name` | `"changes"` | constant |
//! | `ChangesBuilder.format` | `""` | constant, matches upstream (not a document-per-page builder) |
//! | `ChangesBuilder.get_target_uri` | [`ChangesBuilder::get_target_uri`] | always `""`, matches upstream |
//! | `ChangesBuilder.find_versionchanges` + `write` | [`ChangesBuilder::build_all`] | scan + group by version + render report, frameset, source pages, and assets |

use std::collections::BTreeMap;
use std::path::Path;

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;
use crate::util_strypes::{html_escape_attr, html_escape_text};

/// One recovered `versionadded`/`versionchanged`/`deprecated` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionChange {
    /// `"versionadded"`, `"versionchanged"`, or `"deprecated"`.
    pub kind: &'static str,
    /// The version argument (e.g. `"1.2"`).
    pub version: String,
    /// The directive body text, dedented and joined with spaces.
    pub description: String,
    /// One-based source line containing the directive.
    pub line: usize,
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
        write_support_files(outdir)?;
        let version = by_version.keys().next_back().cloned().unwrap_or_default();
        std::fs::write(
            outdir.join("index.html"),
            render_frameset(&version, "", "en").as_bytes(),
        )?;
        std::fs::write(
            outdir.join("changes.html"),
            render_report(&by_version, "", "en").as_bytes(),
        )?;
        write_source_page(outdir, docname, docname, source, "")?;
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
            super::html::discover_docnames_pub(srcdir, &env.config)
        };
        docnames.sort();

        std::fs::create_dir_all(outdir)?;

        let mut by_version: BTreeMap<String, Vec<(String, VersionChange)>> = BTreeMap::new();
        let mut sources = Vec::new();
        for docname in &docnames {
            let src_path =
                super::html::src_path_for_docname_with_suffixes(srcdir, docname, &env.config)?;
            let source =
                crate::environment::read_source_file(&src_path, &env.config.source_encoding())
                    .map_err(|e| {
                        BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
                    })?;
            sources.push((docname.clone(), src_path, source.clone()));
            for change in scan_version_changes(&source) {
                by_version
                    .entry(change.version.clone())
                    .or_default()
                    .push((docname.clone(), change));
            }
            result.written += 1;
        }

        write_support_files(outdir)?;
        let version = env.config.version();
        let version = if version.is_empty() {
            by_version.keys().next_back().cloned().unwrap_or_default()
        } else {
            version
        };
        let docstitle = env.config.html_title();
        let language = env.config.language();
        std::fs::write(
            outdir.join("index.html"),
            render_frameset(&version, &docstitle, &language).as_bytes(),
        )?;
        std::fs::write(
            outdir.join("changes.html"),
            render_report(&by_version, &docstitle, &language).as_bytes(),
        )?;
        for (docname, source_path, source) in sources {
            let filename = source_path
                .strip_prefix(srcdir)
                .unwrap_or(&source_path)
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            write_source_page(outdir, &docname, &filename, &source, &docstitle)?;
        }
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
                line: i + 1,
            });
        }
        i = j.max(i + 1);
    }
    out
}

/// Render the grouped changes as a minimal HTML report, newest version
/// first (reverse string sort, see the module-level accepted deviation).
fn write_support_files(outdir: &Path) -> Result<(), BuildError> {
    std::fs::write(
        outdir.join("default.css"),
        include_bytes!("../../../sphinx/sphinx/themes/default/static/default.css"),
    )?;
    let basic_css = include_str!("../../../sphinx/sphinx/themes/basic/static/basic.css.jinja")
        .replace("{{ theme_sidebarwidth|todim }}", "230px")
        .replace("{{ theme_body_min_width|todim }}", "0px")
        .replace("{{ theme_body_max_width|todim }}", "none");
    std::fs::write(outdir.join("basic.css"), basic_css.as_bytes())?;
    Ok(())
}

fn render_frameset(version: &str, docstitle: &str, language: &str) -> String {
    let lang = if language.is_empty() {
        String::new()
    } else {
        format!(" lang=\"{}\"", html_escape_text(language))
    };
    format!(
        "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Frameset//EN\"\n  \"http://www.w3.org/TR/html4/frameset.dtd\">\n<html{lang}>\n  <head>\n    <title>Changes in Version {} &#8212; {}</title>\n  </head>\n  <frameset cols=\"45%,*\">\n    <frame name=\"main\" src=\"changes.html\">\n    <frame name=\"src\" src=\"about:blank\">\n  </frameset>\n</html>\n",
        html_escape_text(version),
        html_escape_text(docstitle),
    )
}

fn render_report(
    by_version: &BTreeMap<String, Vec<(String, VersionChange)>>,
    docstitle: &str,
    language: &str,
) -> String {
    let mut body = String::new();
    for (version, entries) in by_version.iter().rev() {
        body.push_str(&format!(
            "    <h1>Automatically generated list of changes in version {}</h1>\n    <h2>Other changes</h2>\n",
            html_escape_text(version)
        ));
        let mut by_doc: BTreeMap<&str, Vec<&VersionChange>> = BTreeMap::new();
        for (docname, change) in entries {
            by_doc.entry(docname).or_default().push(change);
        }
        for (docname, changes) in by_doc {
            body.push_str(&format!(
                "    <h4>{}</h4>\n    <ul>\n",
                html_escape_text(docname)
            ));
            for change in changes {
                body.push_str(&format!(
                    "      <li><a href=\"rst/{}.html#L{}\" target=\"src\"><i>{}:</i> {}</a></li>\n",
                    html_escape_attr(docname),
                    change.line,
                    html_escape_text(change.kind),
                    html_escape_text(&change.description)
                ));
            }
            body.push_str("    </ul>\n");
        }
    }
    format!(
        "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\"\n  \"http://www.w3.org/TR/html4/loose.dtd\">\n<html{}>\n  <head>\n    <link rel=\"stylesheet\" href=\"default.css\">\n    <meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">\n    <title>Changes &#8212; {}</title>\n  </head>\n  <body>\n    <div class=\"document\">\n      <div class=\"body\">\n{body}      </div>\n    </div>\n  </body>\n</html>\n",
        if language.is_empty() {
            String::new()
        } else {
            format!(" lang=\"{}\"", html_escape_attr(language))
        },
        html_escape_text(docstitle),
    )
}

fn write_source_page(
    outdir: &Path,
    docname: &str,
    filename: &str,
    source: &str,
    docstitle: &str,
) -> Result<(), BuildError> {
    let mut text = String::new();
    for (index, line) in source.lines().enumerate() {
        let line_number = index + 1;
        let anchored = format!("<a name=\"L{line_number}\"> </a>{}", html_escape_text(line));
        if is_change_directive(line) {
            text.push_str(&format!("<span class=\"hl\">{anchored}</span>\n"));
        } else {
            text.push_str(&format!("{anchored}\n"));
        }
    }
    let page = format!(
        "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\"\n  \"http://www.w3.org/TR/html4/loose.dtd\">\n<html>\n  <head>\n    <title>{} &#8212; {}</title>\n    <style type=\"text/css\">\n      .hl {{ background-color: yellow }}\n    </style>\n  </head>\n  <body style=\"font-size: 90%\">\n    <pre>\n      {text}    </pre>\n  </body>\n</html>\n",
        html_escape_text(filename),
        html_escape_text(docstitle),
    );
    let path = outdir.join("rst").join(format!("{docname}.html"));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, page.as_bytes())?;
    Ok(())
}

fn is_change_directive(line: &str) -> bool {
    let trimmed = line.trim_start();
    [
        ".. version-added::",
        ".. versionadded::",
        ".. version-changed::",
        ".. versionchanged::",
        ".. version-deprecated::",
        ".. deprecated::",
        ".. version-removed::",
        ".. versionremoved::",
    ]
    .iter()
    .any(|directive| trimmed.starts_with(directive))
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
        let html = std::fs::read_to_string(tmp.path().join("changes.html")).unwrap();
        assert!(html.contains("version 1.0"));
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
        let html = std::fs::read_to_string(out.path().join("changes.html")).unwrap();
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
