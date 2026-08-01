//! `sphinxdocrs::builders::singlehtml` — Rust port of
//! `sphinx.builders.singlehtml.SingleFileHTMLBuilder`.
//!
//! Renders every document into **one** `index.html` page, reusing
//! `HtmlBuilder`'s fragment rendering (`render_fragment_from_tree`) and
//! embedded-theme wrapper (`render_embedded_or_wrap`) per document, then
//! concatenating the fragments (each wrapped in a `<div id="{docname}">`
//! anchor) before wrapping the whole thing once.
//!
//! **Accepted deviation:** this does not go through the *real* resolved
//! Sphinx theme (H6c's `theme_render::ThemeRenderer`) or generate a merged
//! sidebar/TOC reflecting the concatenated structure — each document's
//! fragment is rendered independently (same as `HtmlBuilder`'s embedded
//! placeholder-theme path) and merely concatenated with an anchor `id`, so
//! within-page navigation (`#docname`) works but there is no single merged
//! table of contents. Document order follows [`crate::toctree`]'s resolved
//! toctree rooted at `env.config.root_doc()` (mirroring upstream's
//! `assemble_doctree`/`inline_all_toctrees`, which walks the toctree
//! structure rather than sorting docnames); any document not reachable
//! from the root toctree (orphans, or projects with no `.. toctree::` at
//! all) is appended afterwards in lexicographic order so nothing silently
//! disappears from the merged page.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `SingleFileHTMLBuilder.name` | `"singlehtml"` | constant |
//! | `SingleFileHTMLBuilder.format` | `"html"` | constant |
//! | `SingleFileHTMLBuilder.out_suffix` | `".html"` | constant |
//! | `SingleFileHTMLBuilder.get_target_uri` | [`SinglehtmlBuilder::get_target_uri`] | `""` for `index`, else `"index.html#{docname}"` |
//! | `SingleFileHTMLBuilder.assemble_doctree` + `write` | [`SinglehtmlBuilder::build_all`] | concatenates every document's rendered fragment into one `index.html` |

use std::path::Path;

use docutilsrs::parse_rst_with_source;

use super::html::HtmlBuilder;
use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Single-page HTML builder: every document rendered into one `index.html`.
#[derive(Default)]
pub struct SinglehtmlBuilder {
    inner: HtmlBuilder,
}

impl SinglehtmlBuilder {
    pub fn new() -> Self {
        Self {
            inner: HtmlBuilder::new(),
        }
    }
}

impl Builder for SinglehtmlBuilder {
    fn name(&self) -> &str {
        "singlehtml"
    }
    fn format(&self) -> &str {
        "html"
    }
    fn out_suffix(&self) -> &str {
        ".html"
    }

    /// Mirrors `SingleFileHTMLBuilder.get_target_uri`: `""` for the root
    /// document, else an in-page anchor into the single `index.html`.
    fn get_target_uri(&self, docname: &str) -> String {
        if docname == "index" {
            String::new()
        } else {
            format!("index.html#{docname}")
        }
    }

    /// Single-document convenience path: renders `docname` alone as the
    /// entire page (used only when this builder is invoked outside a full
    /// project build).
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let (title, body) = self.inner.render_fragment_from_tree(docname, &tree);
        let anchored = format!("<div id=\"{}\">\n{body}\n</div>", html_escape_attr(docname));
        let page = HtmlBuilder::render_embedded_or_wrap(
            docname,
            &title,
            &anchored,
            &crate::builders::html::PageMeta::default(),
        );
        write_index(outdir, &page)
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
        let docnames = toctree_order(env, docnames);

        std::fs::create_dir_all(outdir)?;
        super::html::write_static_files(outdir, super::html::PathStyle::Flat, false)?;
        super::html::copy_html_static_path(srcdir, outdir, &env.config)?;
        crate::theme_static::copy_theme_static_files_for_builder(
            &env.config,
            outdir,
            srcdir,
            "singlehtml",
        )
        .map_err(BuildError::Io)?;

        let meta = crate::builders::html::PageMeta {
            project: env.config.project(),
            docstitle: env.config.project(),
            copyright: env
                .config
                .get("project_copyright")
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default(),
            language: env.config.language(),
        };

        let mut sections: Vec<String> = Vec::with_capacity(docnames.len());
        let mut combined_title = String::new();
        for (i, docname) in docnames.iter().enumerate() {
            let tree = match env.get_and_resolve_doctree(docname) {
                Ok(tree) => tree,
                Err(_) => {
                    let src_path = srcdir.join(format!("{docname}.rst"));
                    let source = crate::environment::read_source_file(
                        &src_path,
                        &env.config.source_encoding(),
                    )
                    .map_err(|e| {
                        BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
                    })?;
                    parse_rst_with_source(&source, docname)
                }
            };
            let (title, body) = self.inner.render_fragment_from_tree(docname, &tree);
            if i == 0 {
                combined_title = title;
            }
            sections.push(format!(
                "<div id=\"{}\">\n{body}\n</div>",
                html_escape_attr(docname)
            ));
            result.written += 1;
        }

        let body = sections.join("\n");
        let page = HtmlBuilder::render_embedded_or_wrap(
            "index",
            if combined_title.is_empty() {
                "Contents"
            } else {
                &combined_title
            },
            &body,
            &meta,
        );
        write_index(outdir, &page)?;
        super::html::write_objects_inventory(&self.inner, env, outdir, &docnames)?;
        Ok(result)
    }
}

fn write_index(outdir: &Path, page: &str) -> Result<(), BuildError> {
    std::fs::create_dir_all(outdir)?;
    std::fs::write(outdir.join("index.html"), page.as_bytes())?;
    Ok(())
}

/// Orders `docnames` following the project's toctree structure rooted at
/// `env.config.root_doc()`, mirroring upstream's `assemble_doctree` /
/// `inline_all_toctrees` document-order walk (see module docs). Documents
/// not reachable from the root toctree (including projects that define no
/// `.. toctree::` at all) are appended afterwards in lexicographic order,
/// so no discovered document is ever dropped from the merged page.
fn toctree_order(env: &BuildEnvironment, docnames: Vec<String>) -> Vec<String> {
    use std::collections::HashSet;

    let all: HashSet<&str> = docnames.iter().map(String::as_str).collect();
    let mut ordered: Vec<String> = Vec::with_capacity(docnames.len());
    let mut seen: HashSet<String> = HashSet::new();

    let root = env.config.root_doc();
    if all.contains(root.as_str()) {
        ordered.push(root.clone());
        seen.insert(root.clone());
    }

    let toc = crate::toctree::global_toctree_for_doc(env, 0);
    flatten_toctree(&toc, &all, &mut ordered, &mut seen);

    let mut leftovers: Vec<String> = docnames.into_iter().filter(|d| !seen.contains(d)).collect();
    leftovers.sort();
    ordered.extend(leftovers);
    ordered
}

/// Depth-first flattening of a resolved toctree into document order,
/// skipping anything not present in `all` (defensive: `toctree_includes`
/// could in principle reference a document this build doesn't know
/// about) and anything already visited (cycle protection).
fn flatten_toctree(
    entries: &[crate::toctree::TocEntry],
    all: &std::collections::HashSet<&str>,
    ordered: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
) {
    for entry in entries {
        if all.contains(entry.docname.as_str()) && seen.insert(entry.docname.clone()) {
            ordered.push(entry.docname.clone());
        }
        flatten_toctree(&entry.children, all, ordered, seen);
    }
}

/// Minimal attribute-safe escaping for the `id="..."` anchors built from
/// docnames (which may contain `/`, harmless in an HTML `id` attribute but
/// still passed through a real escaper for `"`/`&`/`<` safety).
fn html_escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn name_format_and_suffix() {
        let b = SinglehtmlBuilder::new();
        assert_eq!(b.name(), "singlehtml");
        assert_eq!(b.format(), "html");
        assert_eq!(b.out_suffix(), ".html");
    }

    #[test]
    fn get_target_uri_root_is_empty_others_are_anchors() {
        let b = SinglehtmlBuilder::new();
        assert_eq!(b.get_target_uri("index"), "");
        assert_eq!(b.get_target_uri("about"), "index.html#about");
        assert_eq!(b.get_target_uri("guide/intro"), "index.html#guide/intro");
    }

    #[test]
    fn build_doc_writes_single_index_html() {
        let tmp = TempDir::new().unwrap();
        SinglehtmlBuilder::new()
            .build_doc("about", "About\n=====\n\nSome info.\n", tmp.path())
            .unwrap();
        let out = std::fs::read_to_string(tmp.path().join("index.html")).unwrap();
        assert!(out.contains("id=\"about\""));
        assert!(out.contains("Some info."));
    }

    #[test]
    fn build_all_merges_every_document_into_one_page() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Welcome\n=======\n\nHomepage.\n",
        )
        .unwrap();
        std::fs::write(src.path().join("about.rst"), "About\n=====\n\nSome info.\n").unwrap();
        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        let result = SinglehtmlBuilder::new()
            .build_all(src.path(), out.path(), &env)
            .unwrap();
        assert_eq!(result.written, 2);
        // Exactly one HTML file is produced.
        assert!(out.path().join("index.html").exists());
        assert!(!out.path().join("about.html").exists());
        let combined = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(combined.contains("id=\"index\""));
        assert!(combined.contains("id=\"about\""));
        assert!(combined.contains("Homepage."));
        assert!(combined.contains("Some info."));
        // `index` renders before `about` in document order.
        assert!(combined.find("id=\"index\"").unwrap() < combined.find("id=\"about\"").unwrap());
    }

    #[test]
    fn build_all_follows_toctree_order_not_alphabetical() {
        // `zzz` sorts after `aaa` alphabetically, but the toctree lists
        // `zzz` before `aaa` — the merged page must follow the toctree.
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Welcome\n=======\n\n.. toctree::\n\n   zzz\n   aaa\n",
        )
        .unwrap();
        std::fs::write(src.path().join("aaa.rst"), "Aaa\n===\n\nAaa content.\n").unwrap();
        std::fs::write(src.path().join("zzz.rst"), "Zzz\n===\n\nZzz content.\n").unwrap();

        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let mut env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        env.find_files().unwrap();
        env.read_all().unwrap();

        let result = SinglehtmlBuilder::new()
            .build_all(src.path(), out.path(), &env)
            .unwrap();
        assert_eq!(result.written, 3);
        let combined = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        let idx_index = combined.find("id=\"index\"").unwrap();
        let idx_zzz = combined.find("id=\"zzz\"").unwrap();
        let idx_aaa = combined.find("id=\"aaa\"").unwrap();
        assert!(idx_index < idx_zzz, "root doc must render first");
        assert!(
            idx_zzz < idx_aaa,
            "toctree order (zzz before aaa) must win over alphabetical order"
        );
    }

    #[test]
    fn build_all_appends_orphan_docs_not_in_any_toctree() {
        // `orphan` has no toctree entry anywhere — it must still appear
        // (nothing is silently dropped), just after the toctree-ordered docs.
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Welcome\n=======\n\n.. toctree::\n\n   aaa\n",
        )
        .unwrap();
        std::fs::write(src.path().join("aaa.rst"), "Aaa\n===\n\nAaa content.\n").unwrap();
        std::fs::write(
            src.path().join("orphan.rst"),
            "Orphan\n======\n\nOrphan content.\n",
        )
        .unwrap();

        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let mut env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        env.find_files().unwrap();
        env.read_all().unwrap();

        let result = SinglehtmlBuilder::new()
            .build_all(src.path(), out.path(), &env)
            .unwrap();
        assert_eq!(result.written, 3);
        let combined = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(combined.contains("id=\"orphan\""));
        let idx_aaa = combined.find("id=\"aaa\"").unwrap();
        let idx_orphan = combined.find("id=\"orphan\"").unwrap();
        assert!(
            idx_aaa < idx_orphan,
            "orphan doc must be appended after toctree-ordered docs"
        );
    }
}
