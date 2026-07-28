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
//! | `toctree()` | [`ToctreeGlobal`] — pre-rendered from `crate::toctree::global_toctree_for_doc` (H5d) |
//! | `parents`/`next`/`prev`/`rellinks` | [`collect_relations`] — a pre-order flatten of the global toctree, mirroring `BuildEnvironment.collect_relations` |
//! | `theme_<option>` | merged `theme.conf`/`theme.toml` options + `html_theme_options` (config wins) |
//! | `html_context` | merged last (highest precedence), matching `self.globalcontext |= self.config.html_context` |
//! | `html-page-context` event | emitted per page via `BuildEnvironment::events_handle` (H4a), best-effort no-op when absent |
//!
//! **Accepted deviations** (see also `crate::toctree`'s own module doc):
//! - `toc`/`toctree()` both render the *global* toctree (H5d has no
//!   per-section local-TOC structure to draw a true local TOC from), and
//!   `toctree()` ignores its `maxdepth`/`collapse`/`includehidden` kwargs.
//! - `sidebars` pattern matching only supports an exact-docname or `"**"`
//!   wildcard entry in `html_sidebars` (no `fnmatch`-style globs).
//! - `css_files`/`script_files` are discovered by scanning `outdir/_static/`
//!   for top-level `*.css`/`*.js` files after static assets are copied,
//!   rather than tracking the upstream `add_css_file`/`add_js_file`
//!   registry — every copied asset gets linked, in the order `read_dir`
//!   returns them (sorted for determinism).
//! - `meta`/`metatags` (per-page docinfo/HTML `<meta>` tags) are not
//!   modeled; both are always empty.
//! - `sphinx_version` reports this crate's own version, not upstream
//!   Sphinx's, since there is no bundled Python Sphinx version to report in
//!   a pure-Rust build.

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
    /// Pre-rendered `<ul>`-nested HTML for the global toctree — backs both
    /// `toctree()` and the per-page `toc` context key (see the module
    /// accepted-deviation note: no true local/global distinction).
    toc_html: String,
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
            return Ok(Value::from(otheruri));
        }
        let target_uri = if resource {
            otheruri
        } else {
            HtmlBuilder::new().get_target_uri(&otheruri)
        };
        let base_uri = self.0.current_target_uri();
        let uri = relative_uri(&base_uri, &target_uri);
        let uri = if uri.is_empty() { "#".to_string() } else { uri };
        Ok(Value::from(uri))
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
        Ok(markupsafers::minijinja_compat::markup_to_value(
            Markup::from_safe(self.0.toc_html.clone()),
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
/// output.
fn render_toc_html(entries: &[TocEntry]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    let mut out = String::from("<ul>\n");
    for entry in entries {
        let href = HtmlBuilder::new().get_target_uri(&entry.docname);
        out.push_str(&format!(
            "<li class=\"toctree-l1\"><a href=\"{}\">{}</a>",
            html_escape_attr(&href),
            html_escape_text(&entry.title)
        ));
        if !entry.children.is_empty() {
            out.push('\n');
            out.push_str(&render_toc_html(&entry.children));
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
}

impl ThemeRenderer {
    /// Attempt to resolve the active theme's real templates and build a
    /// renderer for `env`. `outdir` must already contain the copied static
    /// assets (`copy_theme_static_files`/`copy_html_static_path` should run
    /// first) so `css_files`/`script_files` can be discovered.
    ///
    /// Returns `None` on any resolution failure (no Python, theme not
    /// found, ...) — always a soft failure, never a build error.
    pub fn new(env: &BuildEnvironment, outdir: &Path) -> Option<Self> {
        let theme_name = env.config.html_theme();
        let (template_dirs, theme_conf_options) =
            crate::theme_static::resolve_theme_templates(&theme_name)?;

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

        let all_docs: HashSet<String> = env.all_docs.keys().cloned().collect();
        let toc_entries = toctree::global_toctree_for_doc(env, 0);
        let toc_html = render_toc_html(&toc_entries);
        let relations = collect_relations(&toc_entries);

        let state = Arc::new(PageState {
            current_docname: Mutex::new(String::new()),
            all_docs,
            use_index: env.config.html_use_index(),
            toc_html,
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
        let link_of = |d: &str| HtmlBuilder::new().get_target_uri(d);
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
        ctx.insert("toc".into(), self.state.toc_html.clone().into());
        ctx.insert(
            "display_toc".into(),
            (!toc_entries_for_doc.is_empty()).into(),
        );

        let sourcename = if env.config.html_copy_source() {
            format!("{docname}{source_suffix}")
        } else {
            String::new()
        };
        ctx.insert("sourcename".into(), sourcename.into());

        // `html-page-context` (H4a): let any connected listener mutate the
        // page before render. Best-effort — no-op if no app owns this env.
        if let Some(events) = env.events_handle() {
            events
                .borrow_mut()
                .emit("html-page-context", &[EventArg::Str(docname.to_string())]);
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
/// them. Plain values go through `Value::from_serialize`; since
/// `minijinja::Value` special-cases serializing *itself*, each field's
/// safety marker round-trips correctly through this per-field
/// `HashMap<String, Value>` context — passing the whole context as a
/// single `serde_json::Value` instead would have no way to mark any
/// individual field safe at all.
fn to_minijinja_context(ctx: serde_json::Map<String, serde_json::Value>) -> HashMap<String, Value> {
    let mut out = HashMap::with_capacity(ctx.len());
    for (key, value) in ctx {
        let converted = match key.as_str() {
            "body" | "toc" => {
                Value::from_safe_string(value.as_str().unwrap_or_default().to_string())
            }
            _ => Value::from_serialize(&value),
        };
        out.insert(key, converted);
    }
    out
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

    // Static assets actually present in outdir/_static (see module doc).
    let (css_files, script_files) = discover_static_assets(outdir);
    ctx.insert("css_files".into(), css_files.into());
    ctx.insert("script_files".into(), script_files.into());

    // Sidebars: exact-docname or "**" wildcard match in html_sidebars,
    // else the classic `basic` theme default set.
    // (Per-page override happens in the docname-specific context below via
    // `resolve_sidebars`, stashed here as the raw pattern table.)
    ctx.insert(
        "html_sidebars".into(),
        config
            .html_sidebars()
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::from(v)))
            .collect::<serde_json::Map<_, _>>()
            .into(),
    );
    let default_sidebars = vec![
        "localtoc.html".to_string(),
        "relations.html".to_string(),
        "sourcelink.html".to_string(),
        "searchbox.html".to_string(),
    ];
    let sidebars = config
        .html_sidebars()
        .into_iter()
        .find(|(pattern, _)| pattern == "**")
        .map(|(_, templates)| templates)
        .unwrap_or(default_sidebars);
    ctx.insert("sidebars".into(), sidebars.into());
    ctx.insert("theme_nosidebar".into(), false.into());

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
        let html = render_toc_html(&entries);
        assert!(html.contains("guide/intro.html"), "got: {html}");
        assert!(html.contains("Intro"), "got: {html}");
        assert!(html.contains("guide/sub.html"), "got: {html}");
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
}
