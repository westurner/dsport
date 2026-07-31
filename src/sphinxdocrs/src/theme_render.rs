//! `sphinxdocrs::theme_render` — Tier **H6c**: full per-page context for
//! rendering *real* Sphinx theme templates through `jinja2rs`.
//!
//! [`crate::theme`] ships a small, self-contained placeholder theme
//! (`sphinxdocrs_basic`) for when no real theme can be resolved. This module
//! is the opposite: it loads the *actual* theme's own `.html` templates
//! (found on disk via [`crate::theme_static::resolve_theme_templates`], the
//! same Python-assisted resolution `copy_theme_static_files` already uses
//! for static assets) and renders them through a real `jinja2rs::Environment`
//! — real `layout.html`/`page.html`, real macros (`relbar()`, `sidebar()`,
//! `css()`, `script()`), real `{% trans %}`/`_()` calls, real
//! `{% extends %}`/`{% include %}` inheritance.
//!
//! ## What is ported
//!
//! | upstream context key | source here |
//! | --- | --- |
//! | `pathto` | [`PathtoGlobal`] — `crate::util_osutil::relative_uri` between two `get_target_uri` results |
//! | `hasdoc` | [`HasdocGlobal`] — `env.all_docs` / `use_index` / a fixed `has_search = true` |
//! | `toctree()` | [`ToctreeGlobal`] — the *global* toctree from `crate::toctree::global_toctree_for_doc` (H5d), rendered fresh to HTML on every call with hrefs relative to the current page (see [`render_toc_html`]) |
//! | `toc` (per-page) | rendered from `crate::toctree::get_toc_for(env, docname)` — the subtree *rooted at this document*, distinct from the global `toctree()` above |
//! | `sidebars` | [`resolve_sidebars`] — `html_sidebars` pattern-matched per docname via `crate::util_matching` (the same `fnmatch`-style glob translator `exclude_patterns`/`include_patterns` use), mirroring `StandaloneHTMLBuilder._get_sidebars`'s wildcard-precedence rule |
//! | `parents`/`next`/`prev`/`rellinks` | [`collect_relations`] — a pre-order flatten of the global toctree, mirroring `BuildEnvironment.collect_relations` |
//! | `theme_<option>` | merged `theme.conf`/`theme.toml` options + `html_theme_options` (config wins) |
//! | `html_context` | merged last (highest precedence), matching `self.globalcontext |= self.config.html_context` |
//! | `html-page-context` event | emitted per page via `BuildEnvironment::events_handle` (H4a), best-effort no-op when absent |
//!
//! **Accepted deviations** (see also `crate::toctree`'s own module doc):
//! - `toc` is the subtree of the global toctree *rooted at the current
//!   document* (H5d has no per-section/heading structure to draw a true
//!   in-page heading TOC from, so this is document-granularity, not
//!   section-granularity — still a real local/global distinction, just a
//!   coarser one than upstream's), and `toctree()` ignores its
//!   `maxdepth`/`collapse`/`includehidden` kwargs.
//! - `css_files`/`script_files` are primarily discovered by scanning
//!   `outdir/_static/` for top-level `*.css`/`*.js` files after static
//!   assets are copied (every copied asset gets linked, in the order
//!   `read_dir` returns them, sorted for determinism), with any files a
//!   Python extension registered via `app.add_css_file`/`app.add_js_file`
//!   (**H4c** — `crate::app_facade::PyAppFacade`) appended afterward if not
//!   already present. Registered files are only linked as plain
//!   `<link>`/`<script>` tags (no extra HTML attributes, no SRI, no
//!   inline-script bodies) and are **not** copied into `_static/`
//!   themselves — a registered file renders correctly only when it
//!   already lands there through `html_static_path` or the active theme's
//!   own static files.
//! - `meta`/`metatags` (per-page docinfo/HTML `<meta>` tags) are not
//!   modeled; both are always empty.
//! - `sphinx_version` reports this crate's own version, not upstream
//!   Sphinx's, since there is no bundled Python Sphinx version to report in
//!   a pure-Rust build.
//! - A theme building `titlesuffix`-style values via Jinja string
//!   concatenation (`" — "|safe + docstitle|e`) needs the `+` operator to
//!   propagate markup-safety the way Python MarkupSafe's `Markup.__add__`
//!   does; this is **not** a `theme_render`-level workaround — it's fixed
//!   in the vendored `minijinja` fork itself (`value::ops::add`), so every
//!   theme benefits without any theme-side or call-site change here.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex};

use jinja2rs::Environment;
use markupsafers::Markup;
use minijinja::value::{Object, ObjectRepr, Value};
use minijinja::{Error, ErrorKind, State};

use crate::app_events::EventArg;
use crate::builders::Builder;
use crate::builders::html::HtmlBuilder;
use crate::environment::BuildEnvironment;
use crate::toctree::{self, TocEntry};
use crate::util_osutil::relative_uri;
use crate::util_uri::is_url;

// ── PageState (shared, per-build) ─────────────────────────────────────────────

/// State shared by the `pathto`/`hasdoc`/`toctree` globals, built once per
/// [`ThemeRenderer`] (i.e. once per `build_all` call) and mutated only for
/// the "current page" between renders.
struct PageState {
    /// The docname currently being rendered (updated before each
    /// [`ThemeRenderer::render_page`] call).
    current_docname: Mutex<String>,
    /// Every docname discovered/read this build — backs `hasdoc(name)`.
    all_docs: HashSet<String>,
    /// Whether a `genindex` page will be generated — backs
    /// `hasdoc('genindex')`.
    use_index: bool,
    /// The global toctree structure, re-rendered to HTML on every
    /// `toctree()` call (not just once at build start) because each link's
    /// href must be relative to whichever page is currently being
    /// rendered — a page nested under a subdirectory (e.g.
    /// `tutorial/foo.html`) needs `../usage/bar.html`, not the root-relative
    /// `usage/bar.html` a page at the root would need. Mirrors upstream's
    /// `toctree()` Jinja global, which is itself a per-page call computing
    /// fresh `pathto()`-relativized hrefs every time, not a fixed string.
    toc_entries: Vec<TocEntry>,
}

impl PageState {
    /// Return the `get_target_uri`-style URI for the page currently being
    /// rendered.
    fn current_target_uri(&self) -> String {
        let docname = self.current_docname.lock().unwrap().clone();
        HtmlBuilder::new().get_target_uri(&docname)
    }
}

// ── pathto / hasdoc / toctree globals ─────────────────────────────────────────

/// `pathto(otheruri, resource=False)` — mirrors
/// `StandaloneHTMLBuilder.handle_page`'s inner `pathto` closure.
struct PathtoGlobal(Arc<PageState>);

impl std::fmt::Debug for PathtoGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<pathto>")
    }
}

impl Object for PathtoGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        let otheruri = args
            .first()
            .map(|v| v.to_string())
            .ok_or_else(|| Error::new(ErrorKind::MissingArgument, "pathto() requires a name"))?;
        let resource = args.get(1).map(|v| v.is_true()).unwrap_or(false);

        if resource && otheruri.contains("://") {
            return Ok(Value::from_safe_string(otheruri));
        }
        let target_uri = if resource {
            otheruri
        } else {
            HtmlBuilder::new().get_target_uri(&otheruri)
        };
        let base_uri = self.0.current_target_uri();
        let uri = relative_uri(&base_uri, &target_uri);
        let uri = if uri.is_empty() { "#".to_string() } else { uri };
        // Mark the URI as already-safe markup: real templates apply this
        // through `{{ pathto(...)|e }}` (see e.g. `basic/layout.html`), and
        // real MarkupSafe's `escape()` is a no-op on values that are
        // already `Markup` — which real Sphinx relies on since a raw
        // path/URL string is never meant to have its `/` characters
        // HTML-entity-encoded. `Value::from_safe_string` gives the same
        // behavior here (the `|e` filter short-circuits on `is_safe()`).
        Ok(Value::from_safe_string(uri))
    }
}

/// `hasdoc(name)` — mirrors `StandaloneHTMLBuilder.handle_page`'s inner
/// `hasdoc` closure.
struct HasdocGlobal(Arc<PageState>);

impl std::fmt::Debug for HasdocGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<hasdoc>")
    }
}

impl Object for HasdocGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        let name = args.first().map(|v| v.to_string()).unwrap_or_default();
        let found = self.0.all_docs.contains(&name)
            || name == "search"
            || (name == "genindex" && self.0.use_index);
        Ok(Value::from(found))
    }
}

/// `toctree(**kwargs)` — mirrors
/// `StandaloneHTMLBuilder._get_local_toctree` (see the module
/// accepted-deviation note: kwargs are ignored, and this always renders the
/// global toctree).
struct ToctreeGlobal(Arc<PageState>);

impl std::fmt::Debug for ToctreeGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<toctree>")
    }
}

impl Object for ToctreeGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(self: &Arc<Self>, _state: &State<'_, '_>, _args: &[Value]) -> Result<Value, Error> {
        let base_uri = self.0.current_target_uri();
        let html = render_toc_html(&self.0.toc_entries, &base_uri);
        Ok(markupsafers::minijinja_compat::markup_to_value(
            Markup::from_safe(html),
        ))
    }
}

/// `js_tag(js)` / `css_tag(css)` — mirrors `handle_page`'s inner
/// `js_tag`/`css_tag` closures for the case our asset lists always hit
/// (plain filename strings, not `_JavaScript`/`_CascadingStyleSheet`
/// objects with extra attributes/priority): a plain `<script src=... >`
/// or `<link rel="stylesheet" ...>` tag with a `pathto(..., resource=True)`
/// href. **Accepted deviation:** no SRI checksum query string, no extra
/// attributes (`body`, `async`, ...).
struct JsTagGlobal(Arc<PageState>);

impl std::fmt::Debug for JsTagGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<js_tag>")
    }
}

impl Object for JsTagGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        let js = args.first().map(|v| v.to_string()).unwrap_or_default();
        let uri = resource_pathto(&self.0, &js);
        Ok(markupsafers::minijinja_compat::markup_to_value(
            Markup::from_safe(format!(
                "<script src=\"{}\"></script>",
                html_escape_attr(&uri)
            )),
        ))
    }
}

struct CssTagGlobal(Arc<PageState>);

impl std::fmt::Debug for CssTagGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<css_tag>")
    }
}

impl Object for CssTagGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        let css = args.first().map(|v| v.to_string()).unwrap_or_default();
        let uri = resource_pathto(&self.0, &css);
        Ok(markupsafers::minijinja_compat::markup_to_value(
            Markup::from_safe(format!(
                "<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\" />",
                html_escape_attr(&uri)
            )),
        ))
    }
}

/// Shared `pathto(name, resource=True)` computation used by
/// [`JsTagGlobal`]/[`CssTagGlobal`].
fn resource_pathto(state: &PageState, name: &str) -> String {
    if name.contains("://") {
        return name.to_string();
    }
    let base_uri = state.current_target_uri();
    let uri = relative_uri(&base_uri, name);
    if uri.is_empty() { "#".to_string() } else { uri }
}

/// Render a [`TocEntry`] tree as nested `<ul>`/`<li>` HTML, mirroring the
/// shape (not the exact class names/IDs) of
/// `sphinx.environment.adapters.toctree`'s `compact_paragraph`/`bullet_list`
/// output. `base_uri` is the `get_target_uri`-style URI of the page this
/// HTML is being rendered *for*, so every link can be relativized against
/// it (mirrors upstream's `pathto()`-based link construction) — a raw
/// `get_target_uri` result is root-relative and 404s from any page not at
/// the project root.
fn render_toc_html(entries: &[TocEntry], base_uri: &str) -> String {
    if entries.is_empty() {
        return String::new();
    }
    let mut out = String::from("<ul>\n");
    for entry in entries {
        let target_uri = HtmlBuilder::new().get_target_uri(&entry.docname);
        let href = relative_uri(base_uri, &target_uri);
        out.push_str(&format!(
            "<li class=\"toctree-l1\"><a href=\"{}\">{}</a>",
            html_escape_attr(&href),
            html_escape_text(&entry.title)
        ));
        if !entry.children.is_empty() {
            out.push('\n');
            out.push_str(&render_toc_html(&entry.children, base_uri));
        }
        out.push_str("</li>\n");
    }
    out.push_str("</ul>\n");
    out
}

fn html_escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn html_escape_attr(s: &str) -> String {
    html_escape_text(s).replace('"', "&quot;")
}

// ── relations (parents / prev / next) ─────────────────────────────────────────

/// One document's position in the global toctree's pre-order reading
/// sequence. Mirrors the tuple `BuildEnvironment.collect_relations()` stores
/// per docname (`parent`, `prev`, `next`), simplified to plain docnames.
#[derive(Debug, Clone, Default)]
struct Relation {
    parent: Option<String>,
    prev: Option<String>,
    next: Option<String>,
}

/// Flatten the global toctree into pre-order `(docname, parent)` pairs, then
/// derive `prev`/`next` from adjacency in that flattened order — mirrors
/// `BuildEnvironment.collect_relations`'s traversal shape.
fn collect_relations(entries: &[TocEntry]) -> HashMap<String, Relation> {
    let mut flat: Vec<(String, Option<String>)> = Vec::new();
    fn walk(entries: &[TocEntry], parent: Option<&str>, out: &mut Vec<(String, Option<String>)>) {
        for entry in entries {
            out.push((entry.docname.clone(), parent.map(str::to_owned)));
            walk(&entry.children, Some(&entry.docname), out);
        }
    }
    walk(entries, None, &mut flat);

    let mut relations = HashMap::new();
    for (i, (docname, parent)) in flat.iter().enumerate() {
        let prev = if i > 0 {
            Some(flat[i - 1].0.clone())
        } else {
            None
        };
        let next = flat.get(i + 1).map(|(d, _)| d.clone());
        relations.insert(
            docname.clone(),
            Relation {
                parent: parent.clone(),
                prev,
                next,
            },
        );
    }
    relations
}

// ── sidebars (html_sidebars pattern matching) ─────────────────────────────────

/// `sphinx.util.matching.patmatch(name, pattern)`: does `pattern` (a
/// shell-glob using the same syntax as `exclude_patterns`/
/// `include_patterns` — `*`, `**`, `?`, `[...]`) match `name`? Reuses
/// `crate::util_matching::translate_pattern`, the same glob-to-regex
/// translator `Matcher`/`Project::discover` are built on.
fn patmatch(name: &str, pattern: &str) -> bool {
    let Ok(re) = regex::Regex::new(&format!(
        "^{}",
        crate::util_matching::translate_pattern(pattern)
    )) else {
        return false;
    };
    re.is_match(name)
}

/// `sphinx.builders.html._has_wildcard`: does `pattern` contain any glob
/// metacharacter?
fn has_wildcard(pattern: &str) -> bool {
    pattern.contains(['*', '?', '['])
}

/// Resolve the sidebar template list for `docname`, mirroring
/// `StandaloneHTMLBuilder._get_sidebars`'s precedence rule: iterate
/// `html_sidebars` in declaration order; every matching pattern overwrites
/// the previous match *unless* the previous match was non-wildcarded and
/// the new one is wildcarded (so an exact-docname pattern always wins over
/// a glob, regardless of which was declared first or last; among multiple
/// non-wildcarded or multiple wildcarded matches, the last one declared
/// wins). Falls back to `default_sidebars` when nothing matches.
fn resolve_sidebars(
    docname: &str,
    html_sidebars: &[(String, Vec<String>)],
    default_sidebars: &[String],
) -> Vec<String> {
    let mut matched: Option<&str> = None;
    let mut sidebars: Vec<String> = default_sidebars.to_vec();
    for (pattern, templates) in html_sidebars {
        if !patmatch(docname, pattern) {
            continue;
        }
        if matched.is_some() && has_wildcard(pattern) {
            // An exact pattern already matched; a later wildcard pattern
            // does not override it (matches upstream's `continue`).
            continue;
        }
        matched = Some(pattern);
        sidebars = templates.clone();
    }
    sidebars
}

// ── ThemeRenderer ──────────────────────────────────────────────────────────────

/// Renders pages through the *real* resolved Sphinx theme's own templates.
///
/// Construct once per `build_all` call via [`ThemeRenderer::new`] (`None`
/// when the theme can't be resolved — e.g. no Python/Sphinx available —
/// callers should fall back to [`crate::theme::ThemeRenderer`] in that
/// case), then call [`render_page`](Self::render_page) once per document.
pub struct ThemeRenderer {
    env: Environment,
    state: Arc<PageState>,
    /// Global (build-wide) context shared by every page: theme options,
    /// `html_context`, project metadata, relations, css/script file lists.
    global_ctx: serde_json::Map<String, serde_json::Value>,
    relations: HashMap<String, Relation>,
    /// Raw `html_sidebars` pattern table (pattern, template list), matched
    /// per docname in [`render_page`](Self::render_page) via
    /// [`resolve_sidebars`].
    html_sidebars: Vec<(String, Vec<String>)>,
    /// Sidebar templates to use when no `html_sidebars` pattern matches a
    /// page — mirrors `self.theme.sidebar_templates` (the classic `basic`
    /// theme's own default list; see the module accepted-deviation note).
    default_sidebars: Vec<String>,
}

impl ThemeRenderer {
    /// Attempt to resolve the active theme's real templates and build a
    /// renderer for `env`. `outdir` must already contain the copied static
    /// assets (`copy_theme_static_files`/`copy_html_static_path` should run
    /// first) so `css_files`/`script_files` can be discovered.
    ///
    /// `all_docs` is the set of docnames actually being built in this run
    /// (backs the `hasdoc()` Jinja2 global). Callers should pass the same
    /// docname list they're about to iterate in their write phase, **not**
    /// necessarily `env.all_docs.keys()` — a caller that skipped the H2
    /// read phase (`BuildEnvironment::find_files`/`read_all`) and instead
    /// discovered docnames by walking the filesystem directly would
    /// otherwise get an empty `hasdoc()` for every name, producing output
    /// that silently differs from the two-phase path (e.g. a themed
    /// `{% if hasdoc('about') %}` link present in one build but not the
    /// other for the exact same project).
    ///
    /// Returns `None` on any resolution failure (no Python, theme not
    /// found, ...) — always a soft failure, never a build error.
    pub fn new<'a>(
        env: &BuildEnvironment,
        outdir: &Path,
        all_docs: impl IntoIterator<Item = &'a String>,
    ) -> Option<Self> {
        let theme_name = env.config.html_theme();
        let (template_dirs, theme_conf_options) = crate::theme_static::resolve_theme_templates(
            &theme_name,
            &env.srcdir,
            &env.config.html_theme_path(),
        )?;

        // Mirrors upstream `BuiltinTemplateLoader.init`'s
        // `loaderchain = pathchain + [p.parent for p in pathchain]`: a
        // theme that inherits another via a sibling-relative reference
        // (e.g. `alabaster`'s `page.html` does `{% extends
        // "basic/layout.html" %}`, naming its base by *directory name*
        // rather than just `layout.html`) can only resolve that reference
        // if each theme dir's *parent* is also in the search chain.
        let mut loader_chain = template_dirs.clone();
        for dir in &template_dirs {
            if let Some(parent) = dir.parent() {
                if !loader_chain.iter().any(|d| d == parent) {
                    loader_chain.push(parent.to_path_buf());
                }
            }
        }

        let jinja_env = Environment::with_loader_paths(loader_chain);

        let all_docs: HashSet<String> = all_docs.into_iter().cloned().collect();
        let toc_entries = toctree::global_toctree_for_doc(env, 0);
        let relations = collect_relations(&toc_entries);

        let state = Arc::new(PageState {
            current_docname: Mutex::new(String::new()),
            all_docs,
            use_index: env.config.html_use_index(),
            toc_entries: toc_entries.clone(),
        });

        let mut jinja_env = jinja_env;
        jinja_env.add_global("pathto", Value::from_object(PathtoGlobal(state.clone())));
        jinja_env.add_global("hasdoc", Value::from_object(HasdocGlobal(state.clone())));
        jinja_env.add_global("toctree", Value::from_object(ToctreeGlobal(state.clone())));
        jinja_env.add_global("js_tag", Value::from_object(JsTagGlobal(state.clone())));
        jinja_env.add_global("css_tag", Value::from_object(CssTagGlobal(state.clone())));

        let global_ctx = build_global_context(env, outdir, &theme_conf_options, &toc_entries);

        Some(Self {
            env: jinja_env,
            state,
            global_ctx,
            relations,
            html_sidebars: env.config.html_sidebars(),
            default_sidebars: vec![
                "localtoc.html".to_string(),
                "relations.html".to_string(),
                "sourcelink.html".to_string(),
                "searchbox.html".to_string(),
            ],
        })
    }

    /// Render `docname` (already-rendered `title`/`body_html` from the
    /// H5/HTML5 write path) through the real theme's `page.html`, emitting
    /// `html-page-context` first if `env` has an event bus installed
    /// (H4a/H6c).
    pub fn render_page(
        &self,
        env: &BuildEnvironment,
        docname: &str,
        title: &str,
        body_html: &str,
        source_suffix: &str,
    ) -> Result<String, String> {
        *self.state.current_docname.lock().unwrap() = docname.to_string();

        let mut ctx = self.global_ctx.clone();
        ctx.insert("pagename".into(), docname.into());
        ctx.insert("current_page_name".into(), docname.into());
        ctx.insert("title".into(), title.into());
        ctx.insert("body".into(), body_html.into());
        ctx.insert("meta".into(), serde_json::Value::Null);
        ctx.insert("metatags".into(), "".into());
        ctx.insert("has_maths_elements".into(), false.into());

        let relation = self.relations.get(docname).cloned().unwrap_or_default();
        let titles = &env.titles;
        let base_uri = self.state.current_target_uri();
        let link_of = |d: &str| relative_uri(&base_uri, &HtmlBuilder::new().get_target_uri(d));
        let title_of = |d: &str| titles.get(d).cloned().unwrap_or_else(|| d.to_string());
        ctx.insert(
            "next".to_string(),
            relation
                .next
                .as_deref()
                .map(|d| serde_json::json!({"link": link_of(d), "title": title_of(d)}))
                .unwrap_or(serde_json::Value::Null),
        );
        ctx.insert(
            "prev".to_string(),
            relation
                .prev
                .as_deref()
                .map(|d| serde_json::json!({"link": link_of(d), "title": title_of(d)}))
                .unwrap_or(serde_json::Value::Null),
        );
        // Parent chain (immediate parent only — upstream walks the full
        // ancestor chain via `self.relations`; H5d's toctree data only
        // tracks the immediate container, so this is a single-entry chain).
        let parents: Vec<serde_json::Value> = relation
            .parent
            .as_deref()
            .map(|d| vec![serde_json::json!({"link": link_of(d), "title": title_of(d)})])
            .unwrap_or_default();
        ctx.insert("parents".into(), parents.into());

        let toc_entries_for_doc = toctree::get_toc_for(env, docname);
        // Local TOC: the subtree rooted at *this* document (distinct from
        // `toctree()`'s global tree — see the module accepted-deviation
        // note on granularity).
        ctx.insert(
            "toc".into(),
            render_toc_html(&toc_entries_for_doc, &base_uri).into(),
        );
        ctx.insert(
            "display_toc".into(),
            (!toc_entries_for_doc.is_empty()).into(),
        );

        // Sidebars are resolved per-page: `html_sidebars` patterns are
        // matched against this specific docname (mirrors
        // `StandaloneHTMLBuilder._get_sidebars`).
        ctx.insert(
            "sidebars".into(),
            resolve_sidebars(docname, &self.html_sidebars, &self.default_sidebars).into(),
        );

        let sourcename = if env.config.html_copy_source() {
            format!("{docname}{source_suffix}")
        } else {
            String::new()
        };
        ctx.insert("sourcename".into(), sourcename.into());

        // `html-page-context` (H4a): let any connected listener mutate the
        // page before render. Best-effort in the sense that it's a no-op if
        // no app owns this env; a *connected* listener that raises now does
        // propagate as a render error (closing the former "listener errors
        // are swallowed" deviation — see `crate::app_events`'s module docs).
        if let Some(events) = env.events_handle() {
            events
                .borrow_mut()
                .emit("html-page-context", &[EventArg::Str(docname.to_string())])
                .map_err(|e| e.0)?;
        }

        self.env
            .get_template("page.html")
            .map_err(|e| e.to_string())?
            .render(to_minijinja_context(ctx))
            .map_err(|e| e.to_string())
    }
}

/// Convert a `serde_json`-built context into a
/// `HashMap<String, minijinja::Value>`, marking already-rendered-HTML
/// fields (`body`, `toc`) as native minijinja "safe strings"
/// ([`Value::from_safe_string`]) so the auto-escaper doesn't re-escape
/// them, and marking the `link` sub-field of `next`/`prev`/`parents` (each
/// a `pathto()`-style href — see [`relation_link_value`]) safe the same
/// way, since real templates render these with `{{ prev.link|e }}` /
/// `{{ next.link|e }}` (e.g. `haiku/layout.html`) and a raw path/URL is
/// never meant to have its `/` characters HTML-entity-encoded. Plain
/// values go through `Value::from_serialize`; since `minijinja::Value`
/// special-cases serializing *itself*, each field's safety marker
/// round-trips correctly through this per-field `HashMap<String, Value>`
/// context — passing the whole context as a single `serde_json::Value`
/// instead would have no way to mark any individual field safe at all.
fn to_minijinja_context(ctx: serde_json::Map<String, serde_json::Value>) -> HashMap<String, Value> {
    let mut out = HashMap::with_capacity(ctx.len());
    for (key, value) in ctx {
        let converted = match key.as_str() {
            "body" | "toc" => {
                Value::from_safe_string(value.as_str().unwrap_or_default().to_string())
            }
            "next" | "prev" => relation_link_value(&value),
            "parents" => Value::from_serialize(
                value
                    .as_array()
                    .map(|items| items.iter().map(relation_link_value).collect::<Vec<_>>())
                    .unwrap_or_default(),
            ),
            _ => Value::from_serialize(&value),
        };
        out.insert(key, converted);
    }
    out
}

/// Convert one `{"link": ..., "title": ...}` relation object (built by
/// [`ThemeRenderer::render_page`]'s `link_of`/`title_of` closures) into a
/// minijinja `Value`, marking `link` as already-safe markup — mirrors
/// [`PathtoGlobal::call`]'s same reasoning, since these hrefs are computed
/// with the exact same `relative_uri`/`get_target_uri` pair. `title` stays
/// a plain (auto-escaped) string. `serde_json::Value::Null` (no
/// next/prev/parent) maps to minijinja's undefined-like `()`.
fn relation_link_value(value: &serde_json::Value) -> Value {
    let Some(obj) = value.as_object() else {
        return Value::from_serialize(value);
    };
    let mut out: std::collections::BTreeMap<String, Value> = std::collections::BTreeMap::new();
    for (k, v) in obj {
        let converted = if k == "link" {
            Value::from_safe_string(v.as_str().unwrap_or_default().to_string())
        } else {
            Value::from_serialize(v)
        };
        out.insert(k.clone(), converted);
    }
    Value::from_serialize(&out)
}

/// Build the context shared by every page of this build: theme options,
/// `html_context`, project/version metadata, static asset lists, and the
/// global rellinks list (genindex).
fn build_global_context(
    env: &BuildEnvironment,
    outdir: &Path,
    theme_conf_options: &std::collections::BTreeMap<String, String>,
    toc_entries: &[TocEntry],
) -> serde_json::Map<String, serde_json::Value> {
    let config = &env.config;
    let mut ctx = serde_json::Map::new();

    ctx.insert("embedded".into(), false.into());
    ctx.insert("project".into(), config.project().into());
    ctx.insert("release".into(), config.release().into());
    ctx.insert("version".into(), config.version().into());
    ctx.insert(
        "copyright".into(),
        config
            .get("project_copyright")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default()
            .into(),
    );
    ctx.insert("master_doc".into(), config.root_doc().into());
    ctx.insert("root_doc".into(), config.root_doc().into());
    ctx.insert(
        "use_opensearch".into(),
        (!config.html_use_opensearch().is_empty()).into(),
    );
    ctx.insert("docstitle".into(), config.html_title().into());
    ctx.insert("shorttitle".into(), config.html_short_title().into());
    ctx.insert("show_copyright".into(), config.html_show_copyright().into());
    ctx.insert(
        "show_search_summary".into(),
        config.html_show_search_summary().into(),
    );
    ctx.insert("show_sphinx".into(), config.html_show_sphinx().into());
    ctx.insert("has_source".into(), config.html_copy_source().into());
    ctx.insert("show_source".into(), config.html_show_sourcelink().into());
    ctx.insert(
        "sourcelink_suffix".into(),
        config.html_sourcelink_suffix().into(),
    );
    ctx.insert("file_suffix".into(), ".html".into());
    ctx.insert("link_suffix".into(), ".html".into());
    ctx.insert("language".into(), config.language().into());
    ctx.insert("sphinx_version".into(), env!("CARGO_PKG_VERSION").into());
    ctx.insert("builder".into(), "html".into());
    ctx.insert("html5_doctype".into(), true.into());
    ctx.insert("html_tag".into(), serde_json::Value::Null);
    ctx.insert("pageurl".into(), serde_json::Value::Null);
    ctx.insert("content_root".into(), "".into());
    ctx.insert("encoding".into(), "utf-8".into());

    let last_updated = config.html_last_updated_fmt();
    ctx.insert(
        "last_updated".into(),
        last_updated
            .map(serde_json::Value::from)
            .unwrap_or(serde_json::Value::Null),
    );

    // Logo / favicon (basename when a local file, verbatim when a URL).
    let logo = config
        .html_logo()
        .map(|p| basename_or_url(&p))
        .unwrap_or_default();
    let favicon = config
        .html_favicon()
        .map(|p| basename_or_url(&p))
        .unwrap_or_default();
    ctx.insert("logo_url".into(), logo.into());
    ctx.insert(
        "logo_alt".into(),
        format!("Logo of {}", config.project()).into(),
    );
    ctx.insert("favicon_url".into(), favicon.into());

    // rellinks: genindex only (domain indices aren't modeled here).
    let mut rellinks: Vec<serde_json::Value> = Vec::new();
    if config.html_use_index() {
        rellinks.push(serde_json::json!([
            "genindex",
            "General Index",
            "I",
            "index"
        ]));
    }
    ctx.insert("rellinks".into(), rellinks.into());

    // Static assets actually present in outdir/_static (see module doc),
    // plus any files a Python extension registered via
    // `app.add_css_file`/`app.add_js_file` (H4c) that aren't already in
    // that discovered list (e.g. a name the extension expects
    // `html_static_path`/the active theme to have already copied in).
    let (mut css_files, mut script_files) = discover_static_assets(outdir);
    for css in &env.added_css_files {
        if !css_files.contains(&css.filename) {
            css_files.push(css.filename.clone());
        }
    }
    for js in &env.added_js_files {
        if let Some(name) = &js.filename {
            if !script_files.contains(name) {
                script_files.push(name.clone());
            }
        }
    }
    ctx.insert("css_files".into(), css_files.into());
    ctx.insert("script_files".into(), script_files.into());
    ctx.insert("theme_nosidebar".into(), false.into());

    // `sidebars` itself is per-page (see `resolve_sidebars`, called from
    // `render_page`) since `html_sidebars` patterns are matched against
    // each docname individually — not inserted here.

    // Theme options: theme.conf/theme.toml defaults first, html_theme_options
    // wins (matches `theme.get_options(self.theme_options)`).
    for (k, v) in theme_conf_options {
        ctx.insert(k.clone(), v.clone().into());
    }
    for (k, v) in config.html_theme_options() {
        ctx.insert(format!("theme_{k}"), config_val_to_json(&v));
    }

    // `html_context` merges last (highest precedence), matching
    // `self.globalcontext |= self.config.html_context`.
    for (k, v) in config.html_context() {
        ctx.insert(k, config_val_to_json(&v));
    }

    let _ = toc_entries; // reserved for a future numfig-aware rellinks pass
    ctx
}

fn basename_or_url(path: &str) -> String {
    if is_url(path) {
        path.to_string()
    } else {
        Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path)
            .to_string()
    }
}

/// Scan `outdir/_static` for top-level `*.css`/`*.js` files (sorted for
/// determinism). See the module accepted-deviation note.
fn discover_static_assets(outdir: &Path) -> (Vec<String>, Vec<String>) {
    let static_dir = outdir.join("_static");
    let mut css = Vec::new();
    let mut js = Vec::new();
    let Ok(entries) = std::fs::read_dir(&static_dir) else {
        return (css, js);
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.ends_with(".css") {
            css.push(format!("_static/{name}"));
        } else if name.ends_with(".js") {
            js.push(format!("_static/{name}"));
        }
    }
    css.sort();
    js.sort();
    (css, js)
}

fn config_val_to_json(v: &crate::config::ConfigVal) -> serde_json::Value {
    use crate::config::ConfigVal;
    match v {
        ConfigVal::Null => serde_json::Value::Null,
        ConfigVal::Bool(b) => (*b).into(),
        ConfigVal::Int(i) => (*i).into(),
        ConfigVal::Float(f) => (*f).into(),
        ConfigVal::Str(s) => s.clone().into(),
        ConfigVal::List(items) => items
            .iter()
            .map(config_val_to_json)
            .collect::<Vec<_>>()
            .into(),
        ConfigVal::Map(pairs) => pairs
            .iter()
            .map(|(k, v)| (k.clone(), config_val_to_json(v)))
            .collect::<serde_json::Map<_, _>>()
            .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_toc_html_nested() {
        let entries = vec![TocEntry {
            docname: "guide/intro".into(),
            title: "Intro".into(),
            children: vec![TocEntry {
                docname: "guide/sub".into(),
                title: "Sub".into(),
                children: vec![],
            }],
        }];
        let html = render_toc_html(&entries, "index.html");
        assert!(html.contains("guide/intro.html"), "got: {html}");
        assert!(html.contains("Intro"), "got: {html}");
        assert!(html.contains("guide/sub.html"), "got: {html}");
    }

    #[test]
    fn render_toc_html_relativizes_links_from_a_nested_page() {
        // The bug this guards against: links rendered for a page nested in
        // a subdirectory (e.g. `tutorial/foo.html`) must climb back out
        // (`../usage/bar.html`), not reuse the root-relative
        // `get_target_uri` result (`usage/bar.html`) verbatim — the latter
        // 404s because it resolves relative to `tutorial/` instead of the
        // site root.
        let entries = vec![
            TocEntry {
                docname: "tutorial/index".into(),
                title: "Tutorial".into(),
                children: vec![],
            },
            TocEntry {
                docname: "usage/installation".into(),
                title: "Installing".into(),
                children: vec![],
            },
        ];
        let html = render_toc_html(&entries, "tutorial/foo.html");
        assert!(
            html.contains("href=\"index.html\""),
            "same-directory link should have no ../ prefix, got: {html}"
        );
        assert!(
            html.contains("href=\"../usage/installation.html\""),
            "cross-directory link should climb back out, got: {html}"
        );
    }

    #[test]
    fn collect_relations_prev_next_parent() {
        let entries = vec![
            TocEntry {
                docname: "a".into(),
                title: "A".into(),
                children: vec![TocEntry {
                    docname: "a1".into(),
                    title: "A1".into(),
                    children: vec![],
                }],
            },
            TocEntry {
                docname: "b".into(),
                title: "B".into(),
                children: vec![],
            },
        ];
        let rel = collect_relations(&entries);
        assert_eq!(rel["a"].prev, None);
        assert_eq!(rel["a"].next.as_deref(), Some("a1"));
        assert_eq!(rel["a1"].parent.as_deref(), Some("a"));
        assert_eq!(rel["a1"].prev.as_deref(), Some("a"));
        assert_eq!(rel["a1"].next.as_deref(), Some("b"));
        assert_eq!(rel["b"].prev.as_deref(), Some("a1"));
        assert_eq!(rel["b"].next, None);
    }

    #[test]
    fn basename_or_url_variants() {
        assert_eq!(basename_or_url("img/logo.png"), "logo.png");
        assert_eq!(
            basename_or_url("https://example.com/logo.png"),
            "https://example.com/logo.png"
        );
    }

    #[test]
    fn patmatch_supports_globs_like_sphinx() {
        assert!(patmatch("index", "index"));
        assert!(!patmatch("guide/index", "index"));
        assert!(patmatch("guide/index", "guide/*"));
        assert!(!patmatch("guide/sub/index", "guide/*"));
        assert!(patmatch("guide/sub/index", "guide/**"));
        assert!(patmatch("anything", "**"));
        assert!(patmatch("a", "?"));
        assert!(!patmatch("ab", "?"));
    }

    #[test]
    fn has_wildcard_detects_glob_metacharacters() {
        assert!(!has_wildcard("index"));
        assert!(!has_wildcard("guide/intro"));
        assert!(has_wildcard("**"));
        assert!(has_wildcard("guide/*"));
        assert!(has_wildcard("a?c"));
        assert!(has_wildcard("[abc]"));
    }

    #[test]
    fn resolve_sidebars_falls_back_to_default() {
        let sidebars = resolve_sidebars("index", &[], &["localtoc.html".into()]);
        assert_eq!(sidebars, vec!["localtoc.html".to_string()]);
    }

    #[test]
    fn resolve_sidebars_wildcard_match() {
        let html_sidebars = vec![("**".to_string(), vec!["custom.html".to_string()])];
        let sidebars = resolve_sidebars("anything", &html_sidebars, &["default.html".into()]);
        assert_eq!(sidebars, vec!["custom.html".to_string()]);
    }

    #[test]
    fn resolve_sidebars_glob_pattern_like_sphinx() {
        // Mirrors a real Sphinx `html_sidebars` config using a directory
        // glob, e.g. `html_sidebars = {'guide/*': ['localtoc.html']}`.
        let html_sidebars = vec![("guide/*".to_string(), vec!["localtoc.html".to_string()])];
        assert_eq!(
            resolve_sidebars("guide/intro", &html_sidebars, &["default.html".into()]),
            vec!["localtoc.html".to_string()]
        );
        assert_eq!(
            resolve_sidebars("other/intro", &html_sidebars, &["default.html".into()]),
            vec!["default.html".to_string()]
        );
    }

    #[test]
    fn resolve_sidebars_exact_pattern_wins_over_wildcard() {
        // Mirrors `StandaloneHTMLBuilder._get_sidebars`: an exact-docname
        // pattern always wins over a wildcard, regardless of declaration
        // order.
        let html_sidebars = vec![
            ("**".to_string(), vec!["wildcard.html".to_string()]),
            ("index".to_string(), vec!["exact.html".to_string()]),
        ];
        assert_eq!(
            resolve_sidebars("index", &html_sidebars, &["default.html".into()]),
            vec!["exact.html".to_string()]
        );

        // Order reversed: exact declared first, wildcard second — exact
        // still wins.
        let html_sidebars_reversed = vec![
            ("index".to_string(), vec!["exact.html".to_string()]),
            ("**".to_string(), vec!["wildcard.html".to_string()]),
        ];
        assert_eq!(
            resolve_sidebars("index", &html_sidebars_reversed, &["default.html".into()]),
            vec!["exact.html".to_string()]
        );
    }

    #[test]
    fn resolve_sidebars_last_non_wildcard_match_wins() {
        let html_sidebars = vec![
            ("index".to_string(), vec!["first.html".to_string()]),
            ("index".to_string(), vec!["second.html".to_string()]),
        ];
        assert_eq!(
            resolve_sidebars("index", &html_sidebars, &["default.html".into()]),
            vec!["second.html".to_string()]
        );
    }

    #[test]
    fn local_toc_differs_from_global_toctree() {
        // The `toc` context key (local, per-document) must be built from
        // `toctree::get_toc_for`, not the global `toctree()` tree — a
        // subtree rooted elsewhere in a multi-document project should not
        // leak into a page whose own toctree doesn't reach it.
        let root_entries = vec![TocEntry {
            docname: "guide".into(),
            title: "Guide".into(),
            children: vec![TocEntry {
                docname: "guide/intro".into(),
                title: "Intro".into(),
                children: vec![],
            }],
        }];
        let global_html = render_toc_html(&root_entries, "index.html");
        // The document "guide/intro" has no nested toctree of its own.
        let local_html = render_toc_html(&[], "index.html");
        assert!(global_html.contains("guide/intro.html"));
        assert!(local_html.is_empty());
        assert_ne!(global_html, local_html);
    }

    fn make_test_env(srcdir: &str, doctreedir: &str) -> crate::environment::BuildEnvironment {
        let config = crate::config::SphinxConfig::new_defaults();
        let project = crate::environment::EnvProject::new(srcdir, &[(".rst", "restructuredtext")]);
        crate::environment::BuildEnvironment::new(config, project, srcdir, doctreedir)
    }

    /// `app.add_css_file`/`app.add_js_file` registrations (H4c, synced onto
    /// `BuildEnvironment::added_css_files`/`added_js_files` by
    /// `SphinxApp::build`) must appear in the theme's `css_files`/
    /// `script_files` template lists even when `_static/` doesn't contain
    /// them (e.g. the outdir doesn't exist at all in this test).
    #[test]
    fn build_global_context_includes_extension_registered_assets() {
        let mut env = make_test_env(
            "/tmp/theme-render-test-assets-src",
            "/tmp/theme-render-test-assets-doctrees",
        );
        env.added_css_files.push(crate::registry::CssFile {
            filename: "extra.css".into(),
            attributes: HashMap::new(),
        });
        env.added_js_files.push(crate::registry::JsFile {
            filename: Some("extra.js".into()),
            attributes: HashMap::new(),
        });

        let outdir = Path::new("/tmp/theme-render-test-assets-outdir-does-not-exist");
        let ctx = build_global_context(&env, outdir, &Default::default(), &[]);

        let css_files: Vec<String> = ctx["css_files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert!(
            css_files.contains(&"extra.css".to_string()),
            "{css_files:?}"
        );

        let script_files: Vec<String> = ctx["script_files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert!(
            script_files.contains(&"extra.js".to_string()),
            "{script_files:?}"
        );
    }

    /// A registered asset whose filename is already present in the
    /// discovered `_static/` listing must not be duplicated.
    #[test]
    fn build_global_context_dedupes_registered_asset_already_on_disk() {
        let outdir = tempfile::TempDir::new().unwrap();
        let static_dir = outdir.path().join("_static");
        std::fs::create_dir_all(&static_dir).unwrap();
        std::fs::write(static_dir.join("shared.css"), "/* css */").unwrap();

        let mut env = make_test_env(
            "/tmp/theme-render-test-dedupe-src",
            "/tmp/theme-render-test-dedupe-doctrees",
        );
        env.added_css_files.push(crate::registry::CssFile {
            filename: "_static/shared.css".into(),
            attributes: HashMap::new(),
        });

        let ctx = build_global_context(&env, outdir.path(), &Default::default(), &[]);
        let css_files: Vec<String> = ctx["css_files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert_eq!(
            css_files
                .iter()
                .filter(|f| f.as_str() == "_static/shared.css")
                .count(),
            1,
            "{css_files:?}"
        );
    }
}
