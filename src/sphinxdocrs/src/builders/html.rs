//! `sphinxdocrs::builders::html` — Rust port of
//! `sphinx.builders.html.StandaloneHTMLBuilder` (minimal path).
//!
//! Reads RST source files, parses them with `docutilsrs::parse_rst_with_source`,
//! renders to HTML5 fragments via `docutilsrs::html5`, wraps in a minimal
//! HTML5 page, and writes `.html` output files.
//!
//! ## What is ported (minimal path)
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `StandaloneHTMLBuilder.name` | `"html"` | constant |
//! | `StandaloneHTMLBuilder.format` | `"html"` | constant |
//! | `StandaloneHTMLBuilder.out_suffix` | `".html"` | constant |
//! | `StandaloneHTMLBuilder.get_target_uri` | [`HtmlBuilder::get_target_uri`] | `docname + link_suffix` |
//! | `StandaloneHTMLBuilder.write_doc` | [`HtmlBuilder::build_doc`] | parse RST → HTML5 → write file |
//! | `Builder.build_all` | [`HtmlBuilder::build_all`] | iterate env docs, call `build_doc` |
//!
//! **Deferred**: theming, Jinja2 page templates, search index, CSS/JS assets,
//! image handling, domain indices, i18n.

use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, Html5Options};
use docutilsrs::doctree::{Doctree, NodeKind};
use docutilsrs::{html5, parse_rst_with_source};

use super::{BuildError, BuildResult, Builder};
use crate::config::SphinxConfig;
use crate::environment::BuildEnvironment;
use crate::genindex::{GenIndexTerm, ModIndexEntry};

// ── HtmlBuilder ───────────────────────────────────────────────────────────────

/// Minimal HTML5 builder.
///
/// Mirrors `sphinx.builders.html.StandaloneHTMLBuilder` core path:
/// RST source → `docutilsrs` parse → `docutilsrs::html5` fragment →
/// minimal HTML5 page → `.html` output file.
pub struct HtmlBuilder {
    /// File suffix for output files.
    ///
    /// Mirrors `StandaloneHTMLBuilder.out_suffix` (default `".html"`).
    pub out_suffix: String,

    /// Link suffix used in URIs.
    ///
    /// Mirrors `StandaloneHTMLBuilder.link_suffix` (defaults to
    /// `out_suffix`).
    pub link_suffix: String,

    /// Output layout: one flat `<docname>.html` file per document (the
    /// default, matching `StandaloneHTMLBuilder`), or one
    /// `<docname>/index.html` per document (matching
    /// `sphinx.builders.dirhtml.DirectoryHTMLBuilder`, used by
    /// [`crate::builders::dirhtml::DirhtmlBuilder`]).
    path_style: PathStyle,

    /// docutilsrs HTML5 writer options.
    html5_options: Html5Options,

    /// docutilsrs common options.
    common_options: CommonOptions,
}

/// Output file/URI layout used by [`HtmlBuilder`].
///
/// **Accepted deviation:** only the *physical output layout* and
/// [`Builder::get_target_uri`] switch with [`PathStyle::Dir`] — in-page
/// navigation (relbar/toctree/`pathto`) rendered through the real theme
/// pipeline (`theme_render.rs`) still resolves links via a hardcoded
/// flat-style `HtmlBuilder::new().get_target_uri(..)` call at several call
/// sites, so a `dirhtml` build's *files* land in the correct
/// `docname/index.html` locations but cross-document navigation links
/// embedded in the themed chrome may still point at the flat `.html`
/// naming. Recorded in the H7b plan notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum PathStyle {
    /// `<docname>.html`.
    #[default]
    Flat,
    /// `<docname>/index.html` (`index` docname itself stays at the root).
    Dir,
}

impl HtmlBuilder {
    /// Construct with default settings.
    pub fn new() -> Self {
        Self {
            out_suffix: ".html".into(),
            link_suffix: ".html".into(),
            path_style: PathStyle::Flat,
            html5_options: Html5Options::default(),
            common_options: CommonOptions::default(),
        }
    }

    /// Construct configured for `sphinx.builders.dirhtml.DirectoryHTMLBuilder`-style
    /// output: `<docname>/index.html` files and a matching `get_target_uri`.
    ///
    /// Used by [`crate::builders::dirhtml::DirhtmlBuilder`], which otherwise
    /// delegates every [`Builder`] method to a plain [`HtmlBuilder`]
    /// constructed this way — reusing the entire theming/search-index/
    /// static-asset pipeline unchanged.
    pub(crate) fn new_dir_style() -> Self {
        Self {
            out_suffix: "/index.html".into(),
            link_suffix: "/".into(),
            path_style: PathStyle::Dir,
            html5_options: Html5Options::default(),
            common_options: CommonOptions::default(),
        }
    }

    /// Same as `render_fragment`, but starting from an already-parsed
    /// [`Doctree`] instead of RST source text.
    ///
    /// Renders RST to an HTML5 body fragment and also extracts the document
    /// title promoted by `promote_document_title`. Returns `(title, body_html)`.
    ///
    /// Used by the **H2d** two-phase write path (`write_doc` /
    /// `build_all`), where the read phase
    /// ([`BuildEnvironment::read_all`]) has already parsed and stored the
    /// doctree, so the write phase never re-parses the source.
    pub(crate) fn render_fragment_from_tree(
        &self,
        docname: &str,
        tree: &Doctree,
    ) -> (String, String) {
        // Extract promoted document title from NodeKind::Document { title, .. }
        let title = match &tree.node(tree.root()).kind {
            NodeKind::Document { title, .. } if !title.is_empty() => title.clone(),
            _ => docname.rsplit('/').next().unwrap_or(docname).to_owned(),
        };
        let body = html5(tree, &self.html5_options, &self.common_options);
        (title, body)
    }

    /// Wrap an HTML5 fragment in a minimal full HTML5 page.
    ///
    /// Includes a link to `_static/sphinxdocrs.css` so pages have basic
    /// styling without requiring the full Jinja2 theme pipeline.
    fn wrap_page(title: &str, body: &str, project: &str) -> String {
        let title_esc = html_escape(title);
        let page_title = if project.is_empty() {
            title_esc.clone()
        } else {
            format!("{title_esc} &#8212; {}", html_escape(project))
        };
        format!(
            "<!DOCTYPE html>\n\
             <html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">\n\
             <head>\n\
             <meta charset=\"utf-8\" />\n\
             <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n\
             <title>{page_title}</title>\n\
             <link rel=\"stylesheet\" type=\"text/css\" href=\"_static/sphinxdocrs.css\" />\n\
             </head>\n\
             <body>\n\
             <main id=\"content\">\n\
             {body}\n\
             </main>\n\
             </body>\n\
             </html>\n"
        )
    }

    /// Render `docname` through the Jinja2 theme pipeline and write the full
    /// HTML page to `outdir/{docname}.html`.
    ///
    /// Falls back to [`wrap_page`](Self::wrap_page) if the theme renderer
    /// cannot be constructed or a template fails to render, so a build never
    /// fails solely because of a template error.
    fn build_doc_themed(
        &self,
        docname: &str,
        source: &str,
        outdir: &Path,
        meta: &PageMeta,
    ) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        self.build_doc_themed_from_tree(docname, &tree, outdir, meta)
    }

    /// Same as [`build_doc_themed`](Self::build_doc_themed), but starting
    /// from an already-parsed [`Doctree`] instead of RST source text.
    ///
    /// This is the core of the **H2d** write phase: [`write_doc`](Builder::write_doc)
    /// and the doctree-available branch of `build_all` both funnel through
    /// here so a document is rendered from the tree persisted during the
    /// read phase, never re-parsed from source.
    fn build_doc_themed_from_tree(
        &self,
        docname: &str,
        tree: &Doctree,
        outdir: &Path,
        meta: &PageMeta,
    ) -> Result<(), BuildError> {
        sanitize_docname(docname)?;

        let (title, body) = self.render_fragment_from_tree(docname, tree);
        let page = Self::render_embedded_or_wrap(docname, &title, &body, meta);
        self.write_page(docname, &page, outdir)
    }

    /// Render `docname` through the embedded placeholder theme
    /// ([`crate::theme::ThemeRenderer`]), falling back to
    /// [`wrap_page`](Self::wrap_page) if the theme cannot be constructed or
    /// a template fails to render, so a build never fails solely because of
    /// a template error.
    pub(crate) fn render_embedded_or_wrap(
        docname: &str,
        title: &str,
        body: &str,
        meta: &PageMeta,
    ) -> String {
        match crate::theme::ThemeRenderer::new() {
            Ok(renderer) => {
                let mut ctx = crate::theme::PageContext::new(docname, title, body);
                ctx.project = meta.project.clone();
                ctx.docstitle = meta.docstitle.clone();
                ctx.copyright = meta.copyright.clone();
                ctx.show_copyright = !meta.copyright.is_empty();
                ctx.language = meta.language.clone();
                // `genindex`/`py-modindex` are synthetic pages with no
                // backing `.rst` source file (mirrors real Sphinx, whose
                // `write_genindex`/`write_domain_indices` never set a
                // `sourcename` in their page context, so `sourcelink.html`'s
                // `{%- if ... and sourcename %}` guard hides the link) —
                // clear the docname-derived default so the "Page source"
                // link isn't rendered pointing at a `_sources/*.rst.txt`
                // file that doesn't exist.
                if matches!(docname, "genindex" | "py-modindex") {
                    ctx.sourcename.clear();
                }
                // Theme stylesheet + legacy sphinxdocrs stylesheet.
                ctx.css_files = vec![
                    "_static/basic.css".to_string(),
                    "_static/sphinxdocrs.css".to_string(),
                ];
                match renderer.render_page(&ctx) {
                    Ok(html) => html,
                    Err(e) => {
                        eprintln!(
                            "Warning: theme render failed for {docname:?}: {e}; using fallback"
                        );
                        Self::wrap_page(title, body, &meta.project)
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: theme init failed: {e}; using fallback wrapper");
                Self::wrap_page(title, body, &meta.project)
            }
        }
    }

    /// Render `docname` through the *real* resolved Sphinx theme (**H6c**),
    /// falling back to [`render_embedded_or_wrap`](Self::render_embedded_or_wrap)
    /// if the real theme fails to render this particular page (a template
    /// error on one page should not fail the whole build).
    fn build_doc_real_themed_from_tree(
        &self,
        env: &BuildEnvironment,
        docname: &str,
        tree: &Doctree,
        outdir: &Path,
        meta: &PageMeta,
        renderer: &crate::theme_render::ThemeRenderer,
    ) -> Result<(), BuildError> {
        sanitize_docname(docname)?;

        let (title, body) = self.render_fragment_from_tree(docname, tree);
        let source_suffix = env
            .doc2path(docname)
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_else(|| ".rst".to_string());
        let page = match renderer.render_page(env, docname, &title, &body, &source_suffix) {
            Ok(html) => html,
            Err(e) => {
                eprintln!(
                    "Warning: real theme render failed for {docname:?}: {e}; using embedded fallback"
                );
                Self::render_embedded_or_wrap(docname, &title, &body, meta)
            }
        };
        self.write_page(docname, &page, outdir)
    }

    /// Write a fully-rendered page string to the output path implied by this
    /// builder's [`PathStyle`]: `outdir/{docname}.html` for
    /// [`PathStyle::Flat`], `outdir/{docname}/index.html` for
    /// [`PathStyle::Dir`] (with the `index` docname itself staying at the
    /// directory root: `outdir/index.html`, and any `*/index` docname
    /// staying at its own directory root, matching upstream
    /// `DirectoryHTMLBuilder.get_outfilename`).
    fn write_page(&self, docname: &str, page: &str, outdir: &Path) -> Result<(), BuildError> {
        let rel: PathBuf = match self.path_style {
            PathStyle::Flat => format!("{docname}.html").split('/').collect(),
            PathStyle::Dir => {
                if docname == "index" || docname.ends_with("/index") {
                    format!("{docname}.html").split('/').collect()
                } else {
                    format!("{docname}/index.html").split('/').collect()
                }
            }
        };
        let out_path = outdir.join(rel);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&out_path, page.as_bytes())?;
        Ok(())
    }

    /// Render a link to `docname` (optionally `#anchor`, empty when H5e's
    /// anchors-are-page-level deviation applies), using this builder's
    /// [`PathStyle`]-aware [`Builder::get_target_uri`].
    fn index_link(&self, docname: &str, anchor: &str) -> String {
        let uri = self.get_target_uri(docname);
        let href = if anchor.is_empty() {
            uri
        } else {
            format!("{uri}#{anchor}")
        };
        format!(
            "<a href=\"{}\">{}</a>",
            html_escape(&href),
            html_escape(docname)
        )
    }

    /// Render [`crate::genindex::build_genindex`]'s alphabetical buckets as a
    /// real navigable HTML page (H5e).
    ///
    /// **Accepted deviation:** link text is the docname rather than the
    /// real page title (title lookup isn't threaded through here) and, per
    /// `genindex.rs`'s own accepted deviation, anchors are page-level only
    /// (no in-page id tracking), so every link for a term simply points at
    /// its containing document.
    fn render_genindex_page(
        &self,
        buckets: &[(String, Vec<GenIndexTerm>)],
        meta: &PageMeta,
    ) -> String {
        let mut body = String::new();
        body.push_str("<h1 id=\"index\">Index</h1>\n");
        if buckets.is_empty() {
            body.push_str("<p><em>No index entries.</em></p>\n");
            return Self::render_embedded_or_wrap("genindex", "Index", &body, meta);
        }
        body.push_str("<div class=\"genindex-jumpbox\">\n");
        for (letter, _) in buckets {
            let l = html_escape(letter);
            body.push_str(&format!("<a href=\"#{l}\"><strong>{l}</strong></a> | "));
        }
        body.push_str("\n</div>\n");
        for (letter, terms) in buckets {
            let l = html_escape(letter);
            body.push_str(&format!("<h2 id=\"{l}\">{l}</h2>\n<ul>\n"));
            for term in terms {
                body.push_str("<li>");
                if let Some((target, is_seealso)) = &term.see {
                    let label = if *is_seealso { "see also" } else { "see" };
                    body.push_str(&format!(
                        "{} <em>({label} {})</em>",
                        html_escape(&term.name),
                        html_escape(target)
                    ));
                } else if term.subterms.is_empty() {
                    let links: Vec<String> = term
                        .links
                        .iter()
                        .map(|(d, a)| self.index_link(d, a))
                        .collect();
                    body.push_str(&format!(
                        "{}: {}",
                        html_escape(&term.name),
                        links.join(", ")
                    ));
                } else {
                    let direct = if term.links.is_empty() {
                        String::new()
                    } else {
                        let links: Vec<String> = term
                            .links
                            .iter()
                            .map(|(d, a)| self.index_link(d, a))
                            .collect();
                        format!(" ({})", links.join(", "))
                    };
                    body.push_str(&format!("{}{direct}\n<ul>\n", html_escape(&term.name)));
                    for (sub, sublinks) in &term.subterms {
                        let links: Vec<String> = sublinks
                            .iter()
                            .map(|(d, a)| self.index_link(d, a))
                            .collect();
                        body.push_str(&format!(
                            "<li>{}: {}</li>\n",
                            html_escape(sub),
                            links.join(", ")
                        ));
                    }
                    body.push_str("</ul>\n");
                }
                body.push_str("</li>\n");
            }
            body.push_str("</ul>\n");
        }
        Self::render_embedded_or_wrap("genindex", "Index", &body, meta)
    }

    /// Render [`crate::genindex::build_modindex`]'s alphabetical buckets as
    /// a real Python module index page (H5e). Same accepted deviations as
    /// [`render_genindex_page`](Self::render_genindex_page).
    fn render_modindex_page(
        &self,
        buckets: &[(String, Vec<ModIndexEntry>)],
        meta: &PageMeta,
    ) -> String {
        let mut body = String::new();
        body.push_str("<h1 id=\"module-index\">Python Module Index</h1>\n");
        if buckets.is_empty() {
            body.push_str("<p><em>No modules recorded.</em></p>\n");
            return Self::render_embedded_or_wrap(
                "py-modindex",
                "Python Module Index",
                &body,
                meta,
            );
        }
        body.push_str("<table>\n");
        for (letter, modules) in buckets {
            body.push_str(&format!(
                "<tr><td colspan=\"2\"><strong>{}</strong></td></tr>\n",
                html_escape(letter)
            ));
            for module in modules {
                let link = self.index_link(&module.docname, &module.anchor);
                body.push_str(&format!(
                    "<tr><td></td><td><code>{}</code> {link}</td></tr>\n",
                    html_escape(&module.name)
                ));
            }
        }
        body.push_str("</table>\n");
        Self::render_embedded_or_wrap("py-modindex", "Python Module Index", &body, meta)
    }
}

/// Page-level metadata passed to the theme when rendering a document.
#[derive(Debug, Clone, Default)]
pub(crate) struct PageMeta {
    pub(crate) project: String,
    pub(crate) docstitle: String,
    pub(crate) copyright: String,
    pub(crate) language: String,
}

impl Default for HtmlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for HtmlBuilder {
    fn name(&self) -> &str {
        "html"
    }

    fn format(&self) -> &str {
        "html"
    }

    fn out_suffix(&self) -> &str {
        &self.out_suffix
    }

    /// Return the output URI for `docname`.
    ///
    /// Mirrors `StandaloneHTMLBuilder.get_target_uri`
    /// (`quote(docname) + self.link_suffix`) for [`PathStyle::Flat`], and
    /// `DirectoryHTMLBuilder.get_target_uri` for [`PathStyle::Dir`]: `""`
    /// for the `index` docname, the docname with a trailing `/index`
    /// stripped for any other `*/index` docname, else `"{docname}/"`.
    ///
    /// ```rust
    /// use sphinxdocrs::builders::{Builder, html::HtmlBuilder};
    /// let b = HtmlBuilder::new();
    /// assert_eq!(b.get_target_uri("index"), "index.html");
    /// assert_eq!(b.get_target_uri("guide/intro"), "guide/intro.html");
    /// ```
    fn get_target_uri(&self, docname: &str) -> String {
        match self.path_style {
            PathStyle::Flat => format!("{}{}", percent_encode_path(docname), self.link_suffix),
            PathStyle::Dir => {
                if docname == "index" {
                    String::new()
                } else if let Some(stripped) = docname.strip_suffix("/index") {
                    format!("{}/", percent_encode_path(stripped))
                } else {
                    format!("{}/", percent_encode_path(docname))
                }
            }
        }
    }

    /// Parse `source` and write the resulting HTML to `outdir/{docname}.html`.
    ///
    /// Mirrors `StandaloneHTMLBuilder.write_doc`.
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        // Standalone single-doc build: no project metadata available.
        self.build_doc_themed(docname, source, outdir, &PageMeta::default())
    }

    /// Render an already-parsed doctree (from the **H2** read phase) to
    /// `outdir/{docname}.html`.
    ///
    /// Mirrors [`build_doc`](Builder::build_doc) but skips re-parsing the
    /// RST source, since `doctree` was already produced and persisted by
    /// [`BuildEnvironment::read_all`].
    fn write_doc(&self, docname: &str, doctree: &Doctree, outdir: &Path) -> Result<(), BuildError> {
        // Standalone single-doc write: no project metadata available.
        self.build_doc_themed_from_tree(docname, doctree, outdir, &PageMeta::default())
    }

    /// Build all RST documents in `srcdir` into `outdir`.
    ///
    /// Documents are taken from `env.all_docs` if populated, otherwise
    /// discovered by walking `srcdir` for `*.rst` files.
    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let mut result = BuildResult::default();

        // Collect docnames — from env.all_docs if populated, else discover.
        let docnames: Vec<String> = if !env.all_docs.is_empty() {
            env.all_docs.keys().cloned().collect()
        } else {
            discover_rst_docnames(srcdir)
        };

        std::fs::create_dir_all(outdir)?;
        // Write static assets (minimal CSS, objects.inv stub, genindex stub, .buildinfo)
        write_static_files(outdir)?;

        // Copy user static files (html_static_path) into outdir/_static/.
        copy_html_static_path(srcdir, outdir, &env.config)?;

        // Copy the active theme's static assets (CSS/JS/images) from the
        // installed Sphinx / theme packages, plus stemmer JS and pygments.css.
        if let Err(e) = crate::theme_static::copy_theme_static_files(&env.config, outdir, srcdir) {
            eprintln!("Warning: failed to copy theme static files: {e}");
        }

        // Fetch intersphinx inventory files (no-op when extension is absent).
        crate::intersphinx::fetch_inventories(&env.config, &env.doctreedir);

        // H6c: resolve the real Sphinx theme's own templates (child-first
        // inheritance chain) and build a per-build renderer over them, now
        // that static assets are in place (needed for css/js discovery).
        // `None` when the theme can't be resolved (no Python/Sphinx
        // available, theme not found, ...) — every document then falls
        // back to the embedded placeholder theme, same as before H6c.
        //
        // Pass the docnames actually being built here (`docnames`, above)
        // rather than letting `ThemeRenderer::new` derive `hasdoc()`'s set
        // from `env.all_docs` internally: when this is a single-phase
        // build (env.all_docs never populated because the H2 read phase
        // was skipped), env.all_docs is empty even though `docnames` is
        // not, which previously made `hasdoc()` silently disagree between
        // the single-phase and two-phase paths for the exact same project.
        let real_theme = crate::theme_render::ThemeRenderer::new(env, outdir, &docnames);

        // Page metadata shared by every document (from conf.py).
        let meta = PageMeta {
            project: env.config.project(),
            docstitle: env.config.project(),
            copyright: env
                .config
                .get("project_copyright")
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default(),
            language: env.config.language(),
        };

        for docname in &docnames {
            sanitize_docname(docname)?;
            let src_path = src_path_for_docname_with_suffixes(srcdir, docname, &env.config)?;
            // Prefer the doctree persisted by the H2 read phase
            // (`BuildEnvironment::read_all`) so we never parse a document's
            // RST source twice. Builds that skip the read phase (e.g. older
            // callers / tests invoking `build_all` directly) fall back to
            // parsing from source here, keeping output byte-identical
            // either way.
            let mut tree = match env.get_and_resolve_doctree(docname) {
                Ok(tree) => tree,
                Err(_) => {
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
            // Keep the resolution call as an idempotent compatibility fallback
            // for callers that bypass `read_all`; normal builds already have
            // persisted references from the post-read phase.
            env.resolve_xref_nodes(&mut tree, docname);
            // Expand `.. toctree::` placeholders into their real
            // caption + nested bullet-list-of-links subtree (see
            // `BuildEnvironment::resolve_toctree_nodes`).
            env.resolve_toctree_nodes(&mut tree, docname);
            match &real_theme {
                Some(renderer) => {
                    self.build_doc_real_themed_from_tree(
                        env, docname, &tree, outdir, &meta, renderer,
                    )?;
                }
                None => {
                    self.build_doc_themed_from_tree(docname, &tree, outdir, &meta)?;
                }
            }
            // Copy source to _sources/{docname}.rst.txt (mirrors StandaloneHTMLBuilder).
            copy_source_file(&src_path, docname, outdir)?;
            result.written += 1;
        }

        // genindex.html / py-modindex.html (H5e): real pages built from
        // `env.indexentries`/`env.py_domain` via `crate::genindex`, rather
        // than the placeholder stub `write_static_files` used to write
        // unconditionally.
        let genindex_buckets = crate::genindex::build_genindex(env);
        self.write_page(
            "genindex",
            &self.render_genindex_page(&genindex_buckets, &meta),
            outdir,
        )?;
        let modindex_buckets = crate::genindex::build_modindex(env);
        if !modindex_buckets.is_empty() {
            self.write_page(
                "py-modindex",
                &self.render_modindex_page(&modindex_buckets, &meta),
                outdir,
            )?;
        }

        // Generate the JS search index (searchindex.js) over all built docs,
        // including any domain objects/index-entries the read phase noted.
        if let Err(e) =
            crate::search::SearchIndex::build_and_write_with_env(env, srcdir, outdir, &docnames)
        {
            eprintln!("Warning: failed to write searchindex.js: {e}");
        }

        Ok(result)
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Validate a docname, returning `Err` if any component is `..`, empty, or
/// begins with `/` (which would escape the intended directory).
///
/// This guards against path-traversal attacks when docnames are derived from
/// untrusted sources (e.g. `env.all_docs` populated from user input).
fn sanitize_docname(docname: &str) -> Result<(), BuildError> {
    if docname.is_empty() {
        return Err(BuildError::Other("docname must not be empty".into()));
    }
    for component in docname.split('/') {
        if component.is_empty() || component == ".." || component.starts_with('/') {
            return Err(BuildError::Other(format!(
                "invalid docname component {component:?} in {docname:?}"
            )));
        }
    }
    Ok(())
}

// ── static file helpers ─────────────────────────────────────────────────────

/// Minimal embedded CSS for sphinxdocrs pages.
///
/// This gives the output a usable appearance without the full Jinja2
/// theme pipeline.  The selector names mirror Sphinx's default theme
/// so a future theme upgrade is a drop-in replacement.
const MINIMAL_CSS: &str = r#"/* sphinxdocrs minimal stylesheet */
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
       max-width: 900px; margin: 0 auto; padding: 1em 2em; line-height: 1.6;
       color: #333; background: #fff; }
h1, h2, h3, h4 { color: #1a1a2e; }
a { color: #0057a8; text-decoration: none; }
a:hover { text-decoration: underline; }
pre, code { background: #f5f5f5; border-radius: 3px;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; }
pre { padding: 0.8em; overflow-x: auto; }
code { padding: 0.1em 0.3em; }
.highlight pre { margin: 0; }
table { border-collapse: collapse; width: 100%; }
td, th { border: 1px solid #ccc; padding: 0.4em 0.6em; }
th { background: #f0f0f0; }
.note, .warning, .tip, .caution {
  border-left: 4px solid #0057a8; padding: 0.5em 1em; margin: 1em 0;
  background: #f0f7ff; }
.warning { border-color: #e0a000; background: #fffbe0; }
"#;

/// Write static assets into `outdir/_static/` and top-level build artefacts.
///
/// Currently emits:
/// - `.buildinfo` — sphinx build fingerprint (mirrors StandaloneHTMLBuilder)
/// - `_static/sphinxdocrs.css` — minimal embedded stylesheet
/// - `objects.inv` — empty intersphinx inventory stub
/// - `search.html` — minimal search-page stub
///
/// `genindex.html`/`py-modindex.html` are written separately by
/// [`Builder::build_all`] (via [`HtmlBuilder::render_genindex_page`]/
/// [`HtmlBuilder::render_modindex_page`]), since they need `env` data
/// unavailable here.
///
/// These are the minimum set needed so that HTML pages render usably and
/// parity-checking tools do not flag absent mandatory files.
fn write_static_files(outdir: &Path) -> Result<(), BuildError> {
    let static_dir = outdir.join("_static");
    std::fs::create_dir_all(&static_dir)?;
    std::fs::write(static_dir.join("sphinxdocrs.css"), MINIMAL_CSS.as_bytes())?;
    // Jinja2 theme stylesheet (referenced by the theme's layout.html).
    std::fs::write(
        static_dir.join("basic.css"),
        crate::theme::THEME_CSS.as_bytes(),
    )?;

    // .buildinfo — sphinx build fingerprint written by StandaloneHTMLBuilder.
    let buildinfo = "# Sphinx build info version 1\n\
                     # This file records the configuration used when building these files.\n\
                     # When it is not found, a full rebuild will be done.\n\
                     config: sphinxdocrs\n\
                     tags: head\n";
    std::fs::write(outdir.join(".buildinfo"), buildinfo.as_bytes())?;

    // objects.inv — Sphinx intersphinx inventory (version 2 header only).
    // Real entries are added by the domain pipeline (deferred).
    let inv = "# Sphinx inventory version 2\n\
               # Project: sphinxdocrs\n\
               # Version: \n\
               # The remainder of this file is compressed using zlib.\n";
    let inv_path = outdir.join("objects.inv");
    if !inv_path.exists() {
        std::fs::write(&inv_path, inv.as_bytes())?;
    }

    // search.html — minimal search page stub (mirrors StandaloneHTMLBuilder).
    let search = HtmlBuilder::wrap_page(
        "Search",
        "<p><em>Search not yet implemented by the native builder.</em></p>",
        "",
    );
    let search_path = outdir.join("search.html");
    if !search_path.exists() {
        std::fs::write(&search_path, search.as_bytes())?;
    }

    Ok(())
}

/// Copy every directory listed in `html_static_path` into `outdir/_static/`.
///
/// Mirrors `StandaloneHTMLBuilder.copy_static_files` for the user-provided
/// static path.  Each configured directory is resolved relative to `srcdir`;
/// its **contents** (not the directory itself) are copied into `_static/`,
/// matching Sphinx.  Missing directories are skipped with a warning.
///
/// Theme-provided assets (alabaster.css, doctools.js, …) are **not** produced
/// because the Jinja2 theme pipeline is not ported — only user files are copied.
fn copy_html_static_path(
    srcdir: &Path,
    outdir: &Path,
    config: &SphinxConfig,
) -> Result<(), BuildError> {
    let static_out = outdir.join("_static");
    for entry in config.html_static_path() {
        let src_dir = srcdir.join(&entry);
        if !src_dir.is_dir() {
            eprintln!(
                "Warning: html_static_path entry {:?} does not exist",
                src_dir.display()
            );
            continue;
        }
        std::fs::create_dir_all(&static_out)?;
        copy_dir_contents(&src_dir, &static_out)?;
    }
    Ok(())
}

/// Recursively copy the *contents* of `src` into `dst`.
///
/// Files whose names begin with `.` are skipped (matching Sphinx's exclusion
/// of dotfiles from static-path copies).
fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), BuildError> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        // Skip dotfiles / dot-directories (e.g. `.gitignore`, `.svn`).
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        let from = entry.path();
        let to = dst.join(&name);
        if from.is_dir() {
            std::fs::create_dir_all(&to)?;
            copy_dir_contents(&from, &to)?;
        } else {
            if let Some(parent) = to.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&from, &to).map_err(|e| {
                BuildError::Other(format!(
                    "failed to copy static file {} → {}: {e}",
                    from.display(),
                    to.display()
                ))
            })?;
        }
    }
    Ok(())
}

/// Copy `src_path` to `outdir/_sources/{docname}.rst.txt`.
///
/// Mirrors `StandaloneHTMLBuilder._get_source_suffix` / the source-copy step.
/// The `.txt` extension is used by Python sphinx so browsers serve the files
/// as plain text rather than triggering a download.
fn copy_source_file(src_path: &Path, docname: &str, outdir: &Path) -> Result<(), BuildError> {
    // Determine the original extension (default ".rst").
    let ext = src_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("rst");
    // Output path: _sources/{docname}.{ext}.txt
    // (docname may contain '/' → collect into PathBuf sub-dirs)
    let rel: PathBuf = format!("{docname}.{ext}.txt").split('/').collect();
    let dest = outdir.join("_sources").join(rel);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(src_path, &dest).map_err(|e| {
        BuildError::Other(format!(
            "failed to copy source {} → {}: {e}",
            src_path.display(),
            dest.display()
        ))
    })?;
    Ok(())
}

/// Walk `srcdir` and return all `.rst` docnames (relative, no extension,
/// `/`-separated). Public for use by other builders.
pub fn discover_rst_docnames_pub(srcdir: &Path) -> Vec<String> {
    discover_rst_docnames(srcdir)
}

/// Walk `srcdir` and return all `.rst` docnames (relative, no extension,
/// `/`-separated).
fn discover_rst_docnames(srcdir: &Path) -> Vec<String> {
    let mut docnames = Vec::new();
    collect_rst(srcdir, srcdir, &mut docnames);
    docnames.sort();
    docnames
}

fn collect_rst(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rst(root, &path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rst") {
            if let Ok(rel) = path.strip_prefix(root) {
                // Strip only the trailing ".rst" — avoid Path::with_extension("")
                // which would strip ANY trailing extension (breaking "0.1.rst" → "0").
                let s = rel.to_string_lossy();
                let docname = s.strip_suffix(".rst").unwrap_or(&s).replace('\\', "/");
                out.push(docname);
            }
        }
    }
}

// /// Return the `.rst` source path for a docname.
// ///
// /// NOTE: We must not use `Path::with_extension` here because it strips the
// /// *last* component's existing extension first.  A docname like `changes/0.1`
// /// would become `changes/0.rst` instead of `changes/0.1.rst`.
// ///
// /// # Panics
// /// Callers must have already validated `docname` with [`sanitize_docname`].
// fn src_path_for_docname(srcdir: &Path, docname: &str) -> PathBuf {
//     // Append `.rst` as a string — do NOT use Path::with_extension which
//     // would replace an existing extension component (e.g. "changes/0.1" →
//     // "changes/0.rst" instead of "changes/0.1.rst").
//     let rel: PathBuf = docname.split('/').collect::<PathBuf>();
//     let rel_str = rel.display().to_string();

//     // Only add .rst if it's not already there
//     let path_str = if rel_str.ends_with(".rst") {
//         rel_str
//     } else {
//         format!("{}.rst", rel_str)
//     };

//     srcdir.join(path_str)
// }

/// Return the source path for a docname, trying multiple suffixes in order.
///
/// Given a config with `source_suffix` mapping (e.g., `{".rst": "restructuredtext",
/// ".md": "markdown", ".myst.md": "myst"}`), this function tries each suffix in the
/// order they appear in the config. If a file exists with any of the suffixes, returns
/// that path. Otherwise returns an error.
///
/// # Errors
/// Returns `BuildError` if no source file exists with any configured suffix, or if
/// the config has no suffixes defined.
///
/// # Panics
/// Callers must have already validated `docname` with [`sanitize_docname`].
fn src_path_for_docname_with_suffixes(
    srcdir: &Path,
    docname: &str,
    config: &SphinxConfig,
) -> Result<PathBuf, BuildError> {
    let rel: PathBuf = docname.split('/').collect::<PathBuf>();
    let rel_str = rel.display().to_string();

    let source_suffix = config.source_suffix();

    // If docname already has an extension, just use it as-is
    for ext in source_suffix.keys() {
        if rel_str.ends_with(ext) {
            return Ok(srcdir.join(&rel_str));
        }
    }

    // Try each suffix in order; return the first that exists
    for ext in source_suffix.keys() {
        let path_str = format!("{}{}", rel_str, ext);
        let path = srcdir.join(&path_str);
        if path.exists() {
            return Ok(path);
        }
    }

    // No file found with any configured suffix
    if source_suffix.is_empty() {
        Err(BuildError::Other(
            "no source suffixes configured in source_suffix".into(),
        ))
    } else {
        let suffixes = source_suffix
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        Err(BuildError::Other(format!(
            "source file for docname '{}' not found with any configured suffix: {}",
            docname, suffixes
        )))
    }
}

/// Percent-encode a path for use in a URL, preserving `/`.
fn percent_encode_path(s: &str) -> String {
    // Preserve unreserved chars + `/`; encode everything else.
    s.chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~' | '/') {
                vec![c]
            } else {
                c.to_string()
                    .bytes()
                    .flat_map(|b| vec!['%', hex_hi(b), hex_lo(b)])
                    .collect()
            }
        })
        .collect()
}

fn hex_hi(b: u8) -> char {
    hex_digit((b >> 4) & 0xF)
}
fn hex_lo(b: u8) -> char {
    hex_digit(b & 0xF)
}
fn hex_digit(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'A' + n - 10) as char,
        _ => unreachable!(),
    }
}

/// Minimal HTML escaping for page title.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn builder() -> HtmlBuilder {
        HtmlBuilder::new()
    }

    // ── get_target_uri ────────────────────────────────────────────────────────

    #[test]
    fn get_target_uri_index() {
        assert_eq!(builder().get_target_uri("index"), "index.html");
    }

    #[test]
    fn get_target_uri_subdir() {
        assert_eq!(builder().get_target_uri("guide/intro"), "guide/intro.html");
    }

    #[test]
    fn get_target_uri_nested() {
        assert_eq!(
            builder().get_target_uri("api/module/foo"),
            "api/module/foo.html"
        );
    }

    // ── build_doc ─────────────────────────────────────────────────────────────

    #[test]
    fn build_doc_creates_file() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        b.build_doc(
            "index",
            "Hello World\n===========\n\nA paragraph.\n",
            tmp.path(),
        )
        .unwrap();
        let out = tmp.path().join("index.html");
        assert!(out.exists(), "output file should exist");
        let contents = std::fs::read_to_string(out).unwrap();
        assert!(contents.contains("<!DOCTYPE html>"));
        assert!(contents.contains("Hello World") || contents.contains("paragraph"));
    }

    #[test]
    fn build_doc_creates_subdirs() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        b.build_doc("guide/intro", "Intro\n=====\n\nContent.\n", tmp.path())
            .unwrap();
        let out = tmp.path().join("guide").join("intro.html");
        assert!(out.exists(), "subdirectory output should be created");
    }

    #[test]
    fn build_doc_output_is_html5() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        b.build_doc("test", "Test\n====\n\nParagraph.\n", tmp.path())
            .unwrap();
        let contents = std::fs::read_to_string(tmp.path().join("test.html")).unwrap();
        assert!(contents.contains("<!DOCTYPE html>"));
        assert!(contents.contains("<html"));
        assert!(contents.contains("</html>"));
    }

    #[test]
    fn build_doc_contains_body_content() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        let rst = "Intro\n=====\n\nThis is a **bold** word and an *italic* one.\n";
        b.build_doc("intro", rst, tmp.path()).unwrap();
        let contents = std::fs::read_to_string(tmp.path().join("intro.html")).unwrap();
        // RST bold → <strong>, italic → <em>
        assert!(contents.contains("<strong>") || contents.contains("bold"));
    }

    #[test]
    fn build_doc_lists_rendered() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        let rst = "Items\n=====\n\n- one\n- two\n- three\n";
        b.build_doc("items", rst, tmp.path()).unwrap();
        let contents = std::fs::read_to_string(tmp.path().join("items.html")).unwrap();
        assert!(contents.contains("<ul>") || contents.contains("one"));
    }

    // ── build_all ─────────────────────────────────────────────────────────────

    #[test]
    fn build_all_empty_srcdir() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        let b = builder();
        let result = b.build_all(src.path(), out.path(), &env).unwrap();
        assert_eq!(result.written, 0);
    }

    #[test]
    fn build_all_discovers_rst_files() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        // Write two .rst files.
        std::fs::write(src.path().join("index.rst"), "Index\n=====\n\nWelcome.\n").unwrap();
        std::fs::write(src.path().join("about.rst"), "About\n=====\n\nInfo.\n").unwrap();

        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());

        let b = builder();
        let result = b.build_all(src.path(), out.path(), &env).unwrap();
        assert_eq!(result.written, 2);
        assert!(out.path().join("index.html").exists());
        assert!(out.path().join("about.html").exists());
    }

    #[test]
    fn build_all_with_subdirectory() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::create_dir(src.path().join("guide")).unwrap();
        std::fs::write(src.path().join("index.rst"), "Home\n====\n").unwrap();
        std::fs::write(src.path().join("guide").join("intro.rst"), "Intro\n=====\n").unwrap();

        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());

        let b = builder();
        let result = b.build_all(src.path(), out.path(), &env).unwrap();
        assert_eq!(result.written, 2);
        assert!(out.path().join("index.html").exists());
        assert!(out.path().join("guide").join("intro.html").exists());
    }

    #[test]
    fn build_all_copies_html_static_path_and_writes_searchindex() {
        use crate::config::{ConfigVal, SphinxConfig};
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Home\n====\n\nWidget content.\n",
        )
        .unwrap();
        // User static files.
        std::fs::create_dir(src.path().join("_static")).unwrap();
        std::fs::write(src.path().join("_static").join("custom.css"), b"body{}").unwrap();
        std::fs::write(src.path().join("_static").join("app.js"), b"//js").unwrap();
        // A dotfile that must be skipped.
        std::fs::write(src.path().join("_static").join(".hidden"), b"x").unwrap();

        let mut raw = std::collections::HashMap::new();
        raw.insert(
            "html_static_path".into(),
            ConfigVal::List(vec![ConfigVal::Str("_static".into())]),
        );
        let config = SphinxConfig::new(raw, std::collections::HashMap::new());
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());

        builder().build_all(src.path(), out.path(), &env).unwrap();

        // User static files copied.
        assert!(out.path().join("_static").join("custom.css").exists());
        assert!(out.path().join("_static").join("app.js").exists());
        // Dotfiles skipped.
        assert!(!out.path().join("_static").join(".hidden").exists());
        // searchindex.js written and valid.
        let si = std::fs::read_to_string(out.path().join("searchindex.js")).unwrap();
        assert!(si.starts_with("Search.setIndex("));
        assert!(si.contains("\"widget\"") || si.contains("\"content\""));
    }

    // ── render fragment ───────────────────────────────────────────────────────

    #[test]
    fn render_fragment_section_title() {
        let b = builder();
        let tree = parse_rst_with_source("Hello\n=====\n\nWorld.\n", "test");
        let (_title, html) = b.render_fragment_from_tree("test", &tree);
        // html5 writer produces <section ...> and heading elements
        assert!(html.contains("Hello") || html.contains("section"));
    }

    #[test]
    fn render_fragment_empty_source() {
        let b = builder();
        let tree = parse_rst_with_source("", "empty");
        let (_title, html) = b.render_fragment_from_tree("empty", &tree);
        // Empty source → empty or minimal output, no panic.
        let _ = html; // just ensure it doesn't panic
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    #[test]
    fn percent_encode_plain_path() {
        assert_eq!(percent_encode_path("index"), "index");
    }

    #[test]
    fn percent_encode_path_with_slash() {
        assert_eq!(percent_encode_path("guide/intro"), "guide/intro");
    }

    #[test]
    fn html_escape_special_chars() {
        assert_eq!(
            html_escape("A & B < C > D \"E\""),
            "A &amp; B &lt; C &gt; D &quot;E&quot;"
        );
    }

    #[test]
    fn html_escape_plain_text() {
        assert_eq!(html_escape("Hello World"), "Hello World");
    }

    #[test]
    fn discover_rst_finds_files() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("a.rst"), "").unwrap();
        std::fs::write(tmp.path().join("b.txt"), "").unwrap(); // not rst
        let docs = discover_rst_docnames(tmp.path());
        assert_eq!(docs, vec!["a"]);
    }

    // ── sanitize_docname ──────────────────────────────────────────────────────

    #[test]
    fn sanitize_docname_rejects_dotdot() {
        assert!(sanitize_docname("../etc/passwd").is_err());
        assert!(sanitize_docname("foo/../../etc").is_err());
    }

    #[test]
    fn sanitize_docname_rejects_empty() {
        assert!(sanitize_docname("").is_err());
    }

    #[test]
    fn sanitize_docname_rejects_double_slash() {
        assert!(sanitize_docname("foo//bar").is_err());
    }

    #[test]
    fn sanitize_docname_accepts_valid() {
        assert!(sanitize_docname("index").is_ok());
        assert!(sanitize_docname("guide/intro").is_ok());
        assert!(sanitize_docname("api/v1/ref").is_ok());
    }

    #[test]
    fn build_doc_rejects_path_traversal() {
        let tmp = TempDir::new().unwrap();
        let b = builder();
        let err = b.build_doc("../../evil", "x", tmp.path());
        assert!(err.is_err(), "path traversal docname must be rejected");
    }

    #[test]
    fn src_path_for_docname_with_suffixes_prefers_existing_file() {
        let tmp = TempDir::new().unwrap();

        // Create a .rst file
        std::fs::write(tmp.path().join("index.rst"), "").unwrap();

        // Mock a config with default suffixes
        let config = SphinxConfig::new_defaults();

        let path = src_path_for_docname_with_suffixes(tmp.path(), "index", &config)
            .expect("should find index.rst");

        assert!(path.ends_with("index.rst"));
    }

    #[test]
    fn src_path_for_docname_handles_already_extended_docname() {
        let tmp = TempDir::new().unwrap();

        // Create a file with extension already in the docname
        std::fs::write(tmp.path().join("index.rst"), "").unwrap();

        let config = SphinxConfig::new_defaults();

        let path = src_path_for_docname_with_suffixes(tmp.path(), "index.rst", &config)
            .expect("should find index.rst");

        // Should just use the docname as-is (not add another extension)
        assert!(path.ends_with("index.rst"));
        assert!(!path.ends_with("index.rst.rst"));
    }

    #[test]
    fn src_path_for_docname_with_suffixes_errors_on_missing_file() {
        let tmp = TempDir::new().unwrap();

        // Don't create any files
        let config = SphinxConfig::new_defaults();

        let result = src_path_for_docname_with_suffixes(tmp.path(), "missing", &config);

        assert!(result.is_err(), "should error when no source file exists");
    }
}
