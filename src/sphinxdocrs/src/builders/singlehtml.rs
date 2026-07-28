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
//! table of contents. Real per-document ordering also falls back to a
//! lexicographic sort of `env.all_docs`/discovered docnames rather than a
//! toctree-driven document order, since [`crate::toctree`]'s resolved
//! order is keyed by root document and this builder has no "current root"
//! concept the way upstream's `assemble_doctree` does.
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
        let mut docnames: Vec<String> = if !env.all_docs.is_empty() {
            env.all_docs.keys().cloned().collect()
        } else {
            super::html::discover_rst_docnames_pub(srcdir)
        };
        docnames.sort();
        // `index` (the project root) always renders first, matching
        // upstream's `assemble_doctree` starting from `root_doc`.
        if let Some(pos) = docnames.iter().position(|d| d == "index") {
            let root = docnames.remove(pos);
            docnames.insert(0, root);
        }

        std::fs::create_dir_all(outdir)?;

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
                    let source = std::fs::read_to_string(&src_path).map_err(|e| {
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
        Ok(result)
    }
}

fn write_index(outdir: &Path, page: &str) -> Result<(), BuildError> {
    std::fs::create_dir_all(outdir)?;
    std::fs::write(outdir.join("index.html"), page.as_bytes())?;
    Ok(())
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
        std::fs::write(
            src.path().join("about.rst"),
            "About\n=====\n\nSome info.\n",
        )
        .unwrap();
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
}
