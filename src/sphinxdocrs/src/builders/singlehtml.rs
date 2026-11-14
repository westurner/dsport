//! `sphinxdocrs::builders::singlehtml` — Rust port of
//! `sphinx.builders.singlehtml.SingleFileHTMLBuilder`.
//!
//! Renders a merged doctree into one `index.html`: included documents are
//! inlined at their toctree nodes, with `document-{docname}` targets for
//! cross-document navigation. The assembled tree is rendered through the
//! resolved Sphinx theme when available, with the embedded theme as fallback.
//!
//! Documents not reachable from the root toctree (orphans, or projects with
//! no `.. toctree::` at all) are appended afterwards in lexicographic order
//! so no discovered document silently disappears from the merged page.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `SingleFileHTMLBuilder.name` | `"singlehtml"` | constant |
//! | `SingleFileHTMLBuilder.format` | `"html"` | constant |
//! | `SingleFileHTMLBuilder.out_suffix` | `".html"` | constant |
//! | `SingleFileHTMLBuilder.get_target_uri` | [`SinglehtmlBuilder::get_target_uri`] | `#document-{docname}` for built docs, `""` for `index`, and `{docname}.html` for additional pages |
//! | `SingleFileHTMLBuilder.assemble_doctree` + `write` | [`SinglehtmlBuilder::build_all`] | concatenates every document's rendered fragment into one `index.html` |

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use docutilsrs::doctree::{Doctree, NodeId, NodeKind};
use docutilsrs::parse_rst_with_source;

use super::html::{HtmlBuilder, PathStyle};
use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;
use crate::util_strypes::html_escape_attr;

/// Single-page HTML builder: every document rendered into one `index.html`.
#[derive(Default)]
pub struct SinglehtmlBuilder {
    inner: HtmlBuilder,
    known_docs: RefCell<HashSet<String>>,
}

impl SinglehtmlBuilder {
    pub fn new() -> Self {
        Self {
            inner: HtmlBuilder::new(),
            known_docs: RefCell::new(HashSet::new()),
        }
    }

    pub(crate) fn with_object_signature_highlighting(mut self, enabled: bool) -> Self {
        self.inner = self.inner.with_object_signature_highlighting(enabled);
        self
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

    /// Mirrors `SingleFileHTMLBuilder.get_target_uri` for documents known to
    /// the most recent `build_all` call. The root remains page-relative so
    /// existing links to the merged page continue to resolve correctly.
    fn get_target_uri(&self, docname: &str) -> String {
        if docname == "index" {
            String::new()
        } else if self.known_docs.borrow().contains(docname) {
            format!("#document-{docname}")
        } else {
            format!("{docname}.html")
        }
    }

    /// Single-document convenience path: renders `docname` alone as the
    /// entire page (used only when this builder is invoked outside a full
    /// project build).
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let (title, body) = self.inner.render_fragment_from_tree(docname, &tree);
        let anchored = format!(
            "<div id=\"document-{}\">\n{body}\n</div>",
            html_escape_attr(docname)
        );
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
            super::html::discover_docnames_pub(srcdir, &env.config)
        };
        let docnames = toctree_order(env, docnames);
        self.known_docs
            .replace(docnames.iter().cloned().collect::<HashSet<_>>());

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
        let real_theme =
            crate::theme_render::ThemeRenderer::new(env, outdir, &docnames, PathStyle::Single);
        if real_theme.is_some() {
            let _ = std::fs::remove_file(outdir.join("_static/sphinxdocrs.css"));
        }
        if real_theme.is_some() {
            let _ = std::fs::remove_file(outdir.join("_static/sphinxdocrs.css"));
        }

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

        let mut document_trees = HashMap::with_capacity(docnames.len());
        for docname in &docnames {
            let mut tree = match env.get_and_resolve_doctree(docname) {
                Ok(tree) => tree,
                Err(_) => {
                    let src_path = super::html::src_path_for_docname_with_suffixes(
                        srcdir,
                        docname,
                        &env.config,
                    )?;
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
            env.resolve_xref_nodes_with_builder(&mut tree, docname, self);
            let image_prefix =
                crate::util_osutil::relative_uri(&self.get_target_uri(docname), "_images");
            super::html::copy_image_assets(&mut tree, srcdir, outdir, docname, &image_prefix)?;
            document_trees.insert(docname.clone(), tree);
            result.written += 1;
        }

        let root_doc = env.config.root_doc();
        let merged_tree = assemble_singlehtml_doctree(env, &root_doc, &docnames, &document_trees);
        let (_, body) = self
            .inner
            .render_fragment_from_tree(&root_doc, &merged_tree);
        let combined_title = HtmlBuilder::document_title(&merged_tree).unwrap_or_default();
        let title = if combined_title.is_empty() {
            "Contents"
        } else {
            &combined_title
        };
        let page = if let Some(renderer) = &real_theme {
            let theme_title = env.config.html_title();
            let source_suffix =
                super::html::src_path_for_docname_with_suffixes(srcdir, &root_doc, &env.config)?
                    .extension()
                    .map(|extension| format!(".{}", extension.to_string_lossy()))
                    .unwrap_or_else(|| ".rst".into());
            match renderer.render_page(
                env,
                &root_doc,
                &theme_title,
                &body,
                &source_suffix,
                Some(&merged_tree),
            ) {
                Ok(page) => page,
                Err(error) => {
                    eprintln!(
                        "Warning: real singlehtml theme render failed: {error}; using embedded fallback"
                    );
                    HtmlBuilder::render_embedded_or_wrap("index", title, &body, &meta)
                }
            }
        } else {
            HtmlBuilder::render_embedded_or_wrap("index", title, &body, &meta)
        };
        write_index(outdir, &page)?;
        super::html::write_objects_inventory(self, env, outdir, &docnames)?;
        Ok(result)
    }
}

fn assemble_singlehtml_doctree(
    env: &BuildEnvironment,
    root_doc: &str,
    docnames: &[String],
    documents: &HashMap<String, Doctree>,
) -> Doctree {
    let mut merged = Doctree::new_document(root_doc);
    let merged_root = merged.root();
    let mut included = HashSet::new();
    if let Some(root) = documents.get(root_doc) {
        merged.node_mut(merged_root).kind = root.node(root.root()).kind.clone();
        included.insert(root_doc.to_string());
        let source = InlineSource {
            docname: root_doc,
            tree: root,
        };
        for &child in &root.node(root.root()).children.clone() {
            inline_node_into(
                env,
                &mut merged,
                merged_root,
                &source,
                child,
                documents,
                &mut included,
            );
        }
    }
    for docname in docnames {
        inline_document(
            env,
            &mut merged,
            merged_root,
            docname,
            documents,
            &mut included,
        );
    }
    merged
}

struct InlineSource<'a> {
    docname: &'a str,
    tree: &'a Doctree,
}

fn inline_document(
    env: &BuildEnvironment,
    merged: &mut Doctree,
    parent: NodeId,
    docname: &str,
    documents: &HashMap<String, Doctree>,
    included: &mut HashSet<String>,
) {
    if !included.insert(docname.to_string()) {
        return;
    }
    let Some(source) = documents.get(docname) else {
        return;
    };
    merged.append(
        parent,
        NodeKind::Target {
            ids: format!("document-{docname}"),
            names: String::new(),
            refuri: String::new(),
            anonymous: false,
        },
    );
    let source = InlineSource {
        docname,
        tree: source,
    };
    for child in source.tree.node(source.tree.root()).children.clone() {
        inline_node_into(env, merged, parent, &source, child, documents, included);
    }
}

fn inline_node_into(
    env: &BuildEnvironment,
    merged: &mut Doctree,
    parent: NodeId,
    source: &InlineSource<'_>,
    source_id: NodeId,
    documents: &HashMap<String, Doctree>,
    included: &mut HashSet<String>,
) {
    let node = source.tree.node(source_id);
    if let NodeKind::Toctree {
        caption,
        entries,
        hidden,
        ..
    } = &node.kind
    {
        if !hidden && !entries.is_empty() {
            let wrapper = merged.append(
                parent,
                NodeKind::Container {
                    classes: "toctree-wrapper compound".into(),
                },
            );
            if let Some(caption) = caption {
                let caption_node = merged.append(wrapper, NodeKind::Caption);
                let caption_text = merged.append(
                    caption_node,
                    NodeKind::Inline {
                        classes: "caption-text".into(),
                    },
                );
                merged.append(caption_text, NodeKind::Text(caption.clone()));
            }
            for entry in entries {
                let included_doc = crate::environment::docname_join(source.docname, entry);
                inline_document(env, merged, wrapper, &included_doc, documents, included);
            }
        }
        return;
    }

    let cloned = merged.append(parent, node.kind.clone());
    if let Some(line) = node.line {
        merged.set_line(cloned, line);
    }
    for child in node.children.clone() {
        inline_node_into(env, merged, cloned, source, child, documents, included);
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
    fn get_target_uri_defaults_to_root_or_additional_page() {
        let b = SinglehtmlBuilder::new();
        assert_eq!(b.get_target_uri("index"), "");
        assert_eq!(b.get_target_uri("about"), "about.html");
        assert_eq!(b.get_target_uri("guide/intro"), "guide/intro.html");
    }

    #[test]
    fn get_target_uri_uses_anchors_for_known_docs() {
        let b = SinglehtmlBuilder::new();
        b.known_docs.borrow_mut().extend([
            "index".to_string(),
            "about".to_string(),
            "guide/intro".to_string(),
        ]);

        assert_eq!(b.get_target_uri("about"), "#document-about");
        assert_eq!(b.get_target_uri("guide/intro"), "#document-guide/intro");
        assert_eq!(b.get_target_uri("missing"), "missing.html");
    }

    #[test]
    fn get_target_uri_keeps_root_page_behavior_for_known_index() {
        let b = SinglehtmlBuilder::new();
        b.known_docs.borrow_mut().insert("index".to_string());

        assert_eq!(b.get_target_uri("index"), "");
    }

    #[test]
    fn build_doc_writes_single_index_html() {
        let tmp = TempDir::new().unwrap();
        SinglehtmlBuilder::new()
            .build_doc("about", "About\n=====\n\nSome info.\n", tmp.path())
            .unwrap();
        let out = std::fs::read_to_string(tmp.path().join("index.html")).unwrap();
        assert!(out.contains("id=\"document-about\""));
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
        let builder = SinglehtmlBuilder::new();
        let result = builder.build_all(src.path(), out.path(), &env).unwrap();
        assert_eq!(result.written, 2);
        assert_eq!(builder.get_target_uri("about"), "#document-about");
        assert_eq!(builder.get_target_uri("additional"), "additional.html");
        assert_eq!(builder.get_target_uri("index"), "");
        // Exactly one HTML file is produced.
        assert!(out.path().join("index.html").exists());
        assert!(!out.path().join("about.html").exists());
        let combined = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(!combined.contains("id=\"document-index\""));
        assert!(combined.contains("id=\"document-about\""));
        assert!(combined.contains("Homepage."));
        assert!(combined.contains("Some info."));
        // `index` renders before `about` in document order.
        assert!(
            combined.find("Homepage.").unwrap() < combined.find("id=\"document-about\"").unwrap()
        );
    }

    #[test]
    fn build_all_follows_toctree_order_not_alphabetical() {
        // `zzz` sorts after `aaa` alphabetically, but the toctree lists
        // `zzz` before `aaa` — the merged page must follow the toctree.
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Welcome\n=======\n\nRoot content.\n\n.. toctree::\n\n   zzz\n   aaa\n",
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
        let idx_root_content = combined.find("Root content.").unwrap();
        let idx_zzz = combined.find("id=\"document-zzz\"").unwrap();
        let idx_aaa = combined.find("id=\"document-aaa\"").unwrap();
        assert!(idx_root_content < idx_zzz, "root doc must render first");
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
        assert!(combined.contains("id=\"document-orphan\""));
        let idx_aaa = combined.find("id=\"document-aaa\"").unwrap();
        let idx_orphan = combined.find("id=\"document-orphan\"").unwrap();
        assert!(
            idx_aaa < idx_orphan,
            "orphan doc must be appended after toctree-ordered docs"
        );
    }
}
